use crate::unit::MapUnit;
use crate::viewport::Viewport;
use crate::TilePosition;
use mathie::{Rect, Vec2D};

pub struct MapViewer {
	pub zoom: f64,
	pub x: f64,
	pub y: f64,
}

impl MapViewer {
	pub fn get_viewport(&self, resolution: Vec2D<u32>, aspect_ratio: f32) -> Viewport {
		let num_tiles = self.get_scale();

		let width = (1.0 / num_tiles) * aspect_ratio as f64;
		let min_x = self.x - width;
		let max_x = self.x + width;

		let height = 1.0 / num_tiles;
		let min_y = self.y - height;
		let max_y = self.y + height;

		Viewport {
			resolution,
			pos: Vec2D::new(self.x, self.y),
			zoom: self.zoom,
			view: Rect::new_min_max([min_x, min_y], [max_x, max_y]),
		}
	}

	pub fn get_scale(&self) -> f64 {
		2f64.powf(self.zoom)
	}

	pub fn get_level_scale(&self) -> u32 {
		2u32.pow(self.zoom as u32)
	}
}
