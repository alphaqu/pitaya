mod mvt;

use std::collections::HashSet;
use crate::pos::TilePosition;
use anyways::ext::AuditExt;
use anyways::Result;
use atlas::data::TileData;
use fxhash::FxHashSet;
use reqwest::Method;
use std::fs::write;
use std::io::Read;
use std::path::PathBuf;
use log::trace;
use tokio::sync::RwLock;
use crate::query::mvt::parse_mvt;

pub(crate) const API_TOKEN: &str = env!("MAPBOX_TOKEN");

/// Handles requesting map tiles.
pub struct MapQuery {
    // Storage cache
    cache_path: PathBuf,
    cache_tiles: RwLock<FxHashSet<TilePosition>>,

    // Mapbox
    client: reqwest::Client,
}

impl MapQuery {
	pub fn new() -> Result<MapQuery>  {
		let path = PathBuf::from("../../../../../cache");
		std::fs::create_dir(&path);

		let mut cached = FxHashSet::default();
		for dir in std::fs::read_dir(&path)?.flatten() {
			//println!("{:?}", dir.path());
			if let Some(value) = dir
				.path()
				.file_name()
				.and_then(|v| v.to_str())
				.and_then(|v| TilePosition::parse_file_name(v.strip_suffix(".mvt")?))
			{
				//println!("Found cached tile {value:?}");
				cached.insert(value);
			}
		}

		Ok(MapQuery {
			cache_path: path,
			cache_tiles: RwLock::new(cached),
			client: reqwest::Client::new()
		})
	}

	pub async fn get(&self, pos: TilePosition) -> Result<TileData> {
		if self.cache_tiles.read().await.contains(&pos)  {
			self.get_cached(pos).await.wrap_err_with(|| format!("Failed to get cached tile {:?}", pos))
		} else {
			self.cache_tiles.write().await.insert(pos);
			self.get_mapbox(pos).await.wrap_err_with(|| format!("Failed to get mapbox tile {:?}", pos))
		}
	}

	async fn get_cached(&self, pos: TilePosition) -> Result<TileData> {
		let path = pos.get_file_name();
		let path = self.cache_path.join(path);
		let data = tokio::fs::read(path).await.wrap_err("Failed to read file")?;
		Self::read_mvt(&data).await.wrap_err("Failed to read MVT")
	}

	async fn get_mapbox(&self, pos: TilePosition) -> Result<TileData> {
        let request = self
            .client
            .request(
                Method::GET,
                &format!(
                    "https://api.mapbox.com/v4/mapbox.mapbox-streets-v8/{}/{}/{}.mvt",
                    pos.zoom.zoom, pos.x, pos.y
                ),
            )
            .query(&[("access_token", API_TOKEN)]);

        let response = request.send().await.wrap_err("Failed to request tile")?;
        let bytes = response
            .bytes()
            .await
            .wrap_err("Failed to acquire response")?;

		let path = pos.get_file_name();
		let path = self.cache_path.join(path);
		tokio::fs::write(path, &bytes)
            .await
            .wrap_err("Failed to save response as cache.")?;

        Self::read_mvt(bytes.as_ref()).await.wrap_err("Failed to read MVT")
    }

	async fn read_mvt(data: &[u8]) -> Result<TileData> {
		let mut decoder = flate2::read::GzDecoder::new(data);
		let mut out = Vec::new();
		decoder.read_to_end(&mut out).wrap_err("Failed to decompress data")?;
		let data = parse_mvt(&out)?;
		Ok(data)
	}
}
