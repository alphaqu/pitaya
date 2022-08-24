use log::info;
use mathie::unit::Unit;
use mathie::{Rect, Vec2D};
use crate::pos::Zoom;
use crate::TilePosition;
use crate::unit::MapUnit;

pub struct Viewport {
	pub resolution: Vec2D<u32>,

	pub pos: Vec2D<f64, MapUnit>,
	pub zoom: f64,

	pub view: Rect<f64, MapUnit>
}

impl Viewport {
	pub fn get_tiles(&self) -> Vec<TilePosition> {

		//return vec![TilePosition {
		//	zoom: Zoom { zoom: 0 },
		//	x: 0,
		//	y: 0
		//}];
		//      let level_scale = self.get_level_scale();
		//         fn calc_pos(pos: f32, scale: u32) -> f32 {
		//             let mut value = pos * scale as f32;
		//             if value < 0.0 {
		//                 value -= 1.0;
		//             }
		//             value
		//         }
		//
		//         Box2D::new(
		//             Point2D::new(calc_pos(viewport.min.x, level_scale), calc_pos(viewport.min.y, level_scale)),
		//             Point2D::new(calc_pos(viewport.max.x, level_scale), calc_pos(viewport.max.y, level_scale)),
		//         )

		fn calc_pos(pos: f64, scale: u64) -> f64 {
			let mut value = pos * scale as f64;
			if value < 0.0 {
				value -= 1.0;
			}
			value
		}

		let level_num_tiles = self.get_level_num_tiles();
		let tile_viewport: Rect<f64, MapUnit> = Rect::new_min_max(
			[calc_pos(self.view.min().x(), level_num_tiles), calc_pos(self.view.min().y(), level_num_tiles)],
			[calc_pos(self.view.max().x(), level_num_tiles), calc_pos(self.view.max().y(), level_num_tiles)],
		);

		let mut out = Vec::new();
		let min = tile_viewport.min();
		let max = tile_viewport.max();
		for y in (min.y() as i64)..=(max.y() as i64) {
			for x in (min.x() as i64)..=(max.x() as i64) {
				if x < 0 || y < 0 || x >= level_num_tiles as i64 || y >= level_num_tiles as i64{
					continue;
				}
				let x = ((x as f64 / level_num_tiles as f64).rem_euclid(1.0) * level_num_tiles as f64) as u64;
				let y = ((y as f64 / level_num_tiles as f64).rem_euclid(1.0) * level_num_tiles as f64) as u64;

				if let Some(zoom) = Zoom::new(self.zoom as u8) {
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
	
	pub fn get_num_tiles(&self) -> f64  {
		2f64.powf(self.zoom)
	}

	pub fn get_level_num_tiles(&self) -> u64  {
		2u64.pow(self.zoom as u32)
	}
}

// 0 to 1 float value. Starts on the top left
#[derive(Copy, Clone, Default, Debug)]
pub struct ViewportSpace;

impl Unit for ViewportSpace {}