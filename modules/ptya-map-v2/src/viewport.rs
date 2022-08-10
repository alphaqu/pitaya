use mathie::unit::Unit;
use mathie::{Rect, Vec2D};
use crate::pos::Zoom;
use crate::TilePosition;
use crate::unit::MapUnit;

pub struct Viewport {
	pub resolution: Vec2D<u32>,

	pub pos: Vec2D<f32, MapUnit>,
	pub zoom: f32,

	pub view: Rect<f32, MapUnit>
}

impl Viewport {
	pub fn get_tiles(&self) -> Vec<TilePosition> {
		let num_tiles = self.get_num_tiles() as f32;
		let tile_viewport = self.view * num_tiles;
		let mut out = Vec::new();
		let min = tile_viewport.min();
		let max = tile_viewport.max();
		for y in (*min.y() as i32)..=(*max.y() as i32) {
			for x in (*min.x() as i32)..=(*max.x() as i32) {
				let x = ((x as f32 / num_tiles).rem_euclid(1.0) * num_tiles) as u32;
				let y = ((y as f32 / num_tiles).rem_euclid(1.0) * num_tiles) as u32;

				if let Some(zoom) =Zoom::new(self.zoom as u8) {
					let pos = TilePosition {
						zoom,
						x,
						y,
					};

					out.push(pos);	
				}
			}
		}
		out
	}
	
	pub fn get_num_tiles(&self) -> f32  {
		2f32.powf(self.zoom)
	}
}

// 0 to 1 float value. Starts on the top left
#[derive(Copy, Clone, Default, Debug)]
pub struct ViewportSpace;

impl Unit for ViewportSpace {}