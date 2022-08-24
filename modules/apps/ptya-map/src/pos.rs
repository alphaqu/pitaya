use std::f32::consts::PI;
use std::ops::{Div, Mul};
use std::path::PathBuf;
use egui::Vec2;
use euclid::Angle;
use euclid::default::{Box2D, Point2D, Rect, Size2D};

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct TilePosition {
	pub zoom: u8,
	pub x: u32,
	pub y: u32,
}

impl TilePosition {
	pub fn get_outer(&self) -> Option<TilePosition>{
		if self.zoom != 0 {
			Some(TilePosition {
				zoom: self.zoom - 1,
				x: self.x / 2,
				y: self.y / 2
			})
		} else {
			None
		}
	}

	pub fn get_viewport_magic(&self, viewport: Box2D<f32>, area: egui::Rect) -> egui::Rect {
		let tile_scale = 2u32.pow(self.zoom as u32) as f32;
		let tile_size = Vec2::new(1.0 / tile_scale, 1.0 / tile_scale);

		let viewport_size = Vec2::new(viewport.width(), viewport.height());
		let viewport_min = Vec2::new(viewport.min.x, viewport.min.y);
		let pos = (Vec2::new(
			self.x as f32 / tile_scale,
			self.y as f32 / tile_scale,
		) - viewport_min) * Vec2::new(1.0 / viewport_size.x, 1.0 / viewport_size.y);

		egui::Rect::from_min_size(
			area.min + pos.mul(area.size()),
			tile_size.div(viewport_size).mul(area.size()),
		)
	}

	pub fn is_valid(&self) -> bool {
		let scale = 2u32.pow(self.zoom as u32);
		self.x < scale && self.y < scale
	}

	pub fn get_map_position(&self) -> Rect<f32> {
		let scale = 2u32.pow(self.zoom as u32);
		let size = Size2D::new(1.0 / scale as f32, 1.0 / scale as f32);
		let origin = Point2D::new(self.x as f32 / scale as f32, self.y as f32 / scale as f32);
		Rect::new(origin, size)
	}

	pub fn get_cached_path(&self, mut dir: PathBuf) -> PathBuf {
		dir.push(format!("tile-{}-{}x{}.mvt", self.zoom, self.x, self.y));
		dir
	}

	pub fn parse(text: &str) -> Option<Self> {
		let (_, value) = text.split_once("tile-")?;
		let (zoom, value) = value.split_once("-")?;
		let (x, y) = value.split_once("x")?;
		Some(TilePosition {
			zoom: zoom.parse().ok()?,
			x: x.parse().ok()?,
			y: y.parse().ok()?
		})
	}
}

pub fn lat_to_y(lat: Angle<f32>, zoom: u8) -> f32 {
	let n = (zoom as f32).powf(2.0);
	(1.0 - lat.radians.tan().asinh() / PI) / 2.0 * n
}

pub fn lon_to_x(lon: Angle<f32>, zoom: u8) -> f32 {
	let n = (zoom as f32).powf(2.0);
	lon.to_degrees() / 360.0 * n
}

pub fn y_to_lat(y: f32, zoom: u8) -> Angle<f32> {
	let n = (zoom as f32).powf(2.0);
	Angle::radians((PI * (1.0 - 2.0 * y / n)).sinh().atan())
}

pub fn x_to_lon(x: f32, zoom: u8) -> Angle<f32> {
	let n = (zoom as f32).powf(2.0);
	Angle::degrees(x / n * 360.0 - 180.0)
}

pub fn get_tile_pos(v: f32, zoom: u8) -> u32 {
	let max_tile = zoom.pow(2);
	v.clamp(0.0, max_tile as f32) as u32
}