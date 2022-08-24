mod mvt;

use crate::pos::TilePosition;
use crate::query::mvt::parse_mvt;
use ahash::AHashSet;
use anyways::ext::AuditExt;
use anyways::Result;
use map_renderer::data::TileData;
use ptya_core::asset::{AssetManager, Location};
use ptya_core::System;
use reqwest::Method;
use std::io::Read;
use std::path::{Path, PathBuf};
use tokio::sync::RwLock;

pub(crate) const API_TOKEN: &str = env!("MAPBOX_TOKEN");

/// Handles requesting map tiles.
pub struct MapQuery {
	// Storage cache
	asset_manager: AssetManager,
	cache_tiles: RwLock<AHashSet<TilePosition>>,

	// Mapbox
	client: reqwest::Client,
}

impl MapQuery {
	pub async fn new(asset: AssetManager) -> Result<MapQuery> {
		let mut cached = AHashSet::new();
		for dir in asset.read_dir(Location::Cache, "map").await? {
			//println!("{:?}", dir.path());
			if let Some(value) = dir
				.file_name()
				.and_then(|v| v.to_str())
				.and_then(|v| TilePosition::parse_file_name(v.strip_suffix(".mvt")?))
			{
				//println!("Found cached tile {value:?}");
				cached.insert(value);
			}
		}

		Ok(MapQuery {
			asset_manager: asset,
			cache_tiles: RwLock::new(cached),
			client: reqwest::Client::new(),
		})
	}

	pub async fn get(&self, pos: TilePosition) -> Result<TileData> {
		if self.cache_tiles.read().await.contains(&pos) {
			self.get_cached(pos)
				.await
				.wrap_err_with(|| format!("Failed to get cached tile {:?}", pos))
		} else {
			self.cache_tiles.write().await.insert(pos);
			self.get_mapbox(pos)
				.await
				.wrap_err_with(|| format!("Failed to get mapbox tile {:?}", pos))
		}
	}

	async fn get_cached(&self, pos: TilePosition) -> Result<TileData> {
		let path = pos.get_file_name();
		let data = self
			.asset_manager
			.read_file(Location::Cache, Path::new("map/").join(path))
			.await?;
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
		self
			.asset_manager
			.write_file(Location::Cache, Path::new("map/").join(path), bytes.as_ref())
			.await?;

		Self::read_mvt(bytes.as_ref())
			.await
			.wrap_err("Failed to read MVT")
	}

	async fn read_mvt(data: &[u8]) -> Result<TileData> {
		let mut decoder = flate2::read::GzDecoder::new(data);
		let mut out = Vec::new();
		decoder
			.read_to_end(&mut out)
			.wrap_err("Failed to decompress data")?;
		let data = parse_mvt(&out)?;
		Ok(data)
	}
}
