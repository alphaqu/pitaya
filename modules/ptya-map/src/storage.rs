use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::write;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::sleep;
use std::time::Duration;

use anyways::audit::Audit;
use anyways::ext::AuditExt;
use anyways::Result;
use bincode::config::standard;
use bincode::error::DecodeError;
use euclid::default::Box2D;
use rayon::{ThreadPool, ThreadPoolBuilder};
use reqwest::blocking::{Client, Response};
use reqwest::Method;
use thiserror::Error;

use crate::tile::Tile;
use crate::pos::TilePosition;
use crate::TOKEN;

pub struct MapStorage {
    cache_dir: PathBuf,
    storage_cached: HashSet<TilePosition>,
    visible: HashMap<TilePosition, Tile>,
    client: Client,

    requested: HashSet<TilePosition>,
    complete: (Sender<(TilePosition, Result<Tile, TileRequestError>)>, Receiver<(TilePosition, Result<Tile, TileRequestError>)>),
    thread_pool: Arc<ThreadPool>,
    redraw_request: bool,
}

impl MapStorage {
    pub fn new<P: AsRef<Path>>(cache_dir: P, thread_pool: &Arc<ThreadPool>) -> Result<MapStorage> {
        let dir = PathBuf::from_iter(cache_dir.as_ref());
        fs::create_dir(&dir);

        let mut cached = HashSet::new();
        for dir in fs::read_dir(&dir)?.flatten() {
            println!("{:?}", dir.path());
            if let Some(value) = dir
                .path()
                .file_name()
                .and_then(|v| v.to_str())
                .and_then(|v| TilePosition::parse(v.strip_suffix(".mvt")?))
            {
                println!("Found cached tile {value:?}");
                cached.insert(value);
            }
        }


        Ok(MapStorage {
            cache_dir: dir,
            storage_cached: cached,
            visible: HashMap::new(),
            client: Default::default(),
            requested: Default::default(),
            complete: channel(),
            thread_pool: thread_pool.clone(),
            redraw_request: false
        })
    }

    pub fn prepare_tile(&mut self, pos: TilePosition) -> Result<bool> {
        // Get all completed
        for (pos, result) in self.complete.1.try_iter() {
            let tile = result.wrap_err_with(|| format!("Failed to acquire tile {pos:?}."))?;
            self.requested.remove(&pos);
            self.visible.insert(pos, tile);
            if !self.storage_cached.contains(&pos) {
                self.storage_cached.insert(pos);
            }
        }

        if !pos.is_valid() {
            return Ok(false);
        }
        // Get or request.
        if self.visible.contains_key(&pos) {
            Ok(true)
        } else {
            self.redraw_request = true;
            if !self.requested.contains(&pos) {
                self.requested.insert(pos);

                let task = TileRequestTask {
                    client: self.client.clone(),
                    cache_dir: self.cache_dir.clone(),
                    output: self.complete.0.clone()
                };

                if self.storage_cached.contains(&pos) {
                    self.thread_pool.spawn(move || {
                        task.output.send((pos, task.get_cached(pos))).unwrap();
                    });
                } else {
                    self.thread_pool.spawn(move || {
                        task.output.send((pos, task.get_mapbox(pos))).unwrap();
                    });
                }
            }

            Ok(false)
        }
    }

    pub fn get_tile(&self, pos: TilePosition) -> Option<&Tile> {
        self.visible.get(&pos)
    }

    pub fn end_frame(&mut self, viewport: Box2D<f32>) -> bool {
        let mut remove = Vec::new();
        for (pos, _) in &mut self.visible {
            if !viewport.intersects(&pos.get_map_position().to_box2d()) {
                remove.push(*pos);
            }
        }
        for pos in remove {
            self.visible.remove(&pos);
        }

        let redraw = self.redraw_request;
        self.redraw_request = false;
        redraw
    }
}

#[derive(Error, Debug)]
pub enum TileRequestError {
    // Read cache
    #[error("Failed to read cache file")]
    CacheRead(std::io::Error),
    #[error("Failed to decompress cache file")]
    CacheDecompress(std::io::Error),
    #[error("Failed to decode cache file")]
    DecodeDecode(DecodeError),
    // Request tile
    #[error("Failed to request tile data")]
    RequestData(reqwest::Error),
    #[error("Failed to decompress requested tile data")]
    RequestDecompress(std::io::Error),
    #[error("Failed to parse requested tile data")]
    RequestParse(protobuf::Error),
    // Cache
    #[error("Failed to cache tile data")]
    CacheData(std::io::Error),
}

pub struct TileRequestTask {
    client: Client,
    cache_dir: PathBuf,
    output: Sender<(TilePosition, Result<Tile, TileRequestError>)>,
}

impl TileRequestTask {
    fn get_cached(&self, pos: TilePosition) -> Result<Tile, TileRequestError> {
        let path = pos.get_cached_path(self.cache_dir.clone());
        let data = fs::read(path).map_err(TileRequestError::CacheRead)?;
        Self::read_mvt(&data)
    }

    fn get_mapbox(&self, pos: TilePosition) -> Result<Tile, TileRequestError> {
        let request = self
            .client
            .request(
                Method::GET,
                &format!(
                    "https://api.mapbox.com/v4/mapbox.mapbox-streets-v8/{}/{}/{}.mvt",
                    pos.zoom, pos.x, pos.y
                ),
            )
            .query(&[("access_token", TOKEN)]);
        let response = request
            .send().map_err(TileRequestError::RequestData)?;
        let bytes = response.bytes().map_err(TileRequestError::RequestData)?;
        let buf = pos.get_cached_path(self.cache_dir.clone());
        write(buf, &bytes).map_err(TileRequestError::CacheData)?;
        Self::read_mvt(bytes.as_ref())
    }

    fn read_mvt(data: &[u8]) -> Result<Tile, TileRequestError> {
        let mut decoder = flate2::read::GzDecoder::new(data);
        let mut out = Vec::new();
        decoder.read_to_end(&mut out).map_err(TileRequestError::RequestDecompress)?;
        let tile = Tile::from_bytes(&out).map_err(TileRequestError::RequestParse)?;
        Ok(tile)
    }
}
