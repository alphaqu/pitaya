use mathie::{Rect, Vec2D};
use crate::TilePosition;
use crate::unit::MapUnit;
use crate::viewport::Viewport;

pub struct MapViewer {
	pub zoom: f32,
	pub x: f32,
	pub y: f32,
}

impl MapViewer {

	pub fn get_viewport(&self, resolution: Vec2D<u32>, aspect_ratio: f32) -> Viewport {
		let num_tiles = self.get_scale();


		let width = (0.5 / num_tiles) * aspect_ratio;
		let min_x = self.x - width;
		let max_x = self.x + width;

		let height = 0.5 / num_tiles;
		let min_y = self.y - height;
		let max_y = self.y + height;
		
		Viewport {
			resolution,
			pos: Vec2D::new(self.x, self.y),
			zoom: self.zoom,
			view: Rect::new([min_x, min_y], [width, height])
		}
	}

	// 	pub fn get_viewport(&self, aspect_ratio: f32) -> Box2D<f32> {
	// 		let scale = self.get_scale();
	// 
	// 		let width = (0.5  / scale) * aspect_ratio;
	// 		let min_x = self.x - width;
	// 		let max_x = self.x + width;
	// 
	// 		let height = 0.5 / scale;
	// 		let min_y = self.y - height;
	// 		let max_y = self.y + height;
	// 
	// 		Box2D::new(
	// 			Point2D::new(min_x, min_y),
	// 			Point2D::new(max_x, max_y),
	// 		)
	// 	}
	// 
	// 	pub fn get_tile_viewport(&self, viewport: Box2D<f32>) -> Box2D<f32> {
	// 		let level_scale = self.get_level_scale();
	// 		fn calc_pos(pos: f32, scale: u32) -> f32 {
	// 			let mut value = pos * scale as f32;
	// 			if value < 0.0 {
	// 				value -= 1.0;
	// 			}
	// 			value
	// 		}
	// 
	// 		Box2D::new(
	// 			Point2D::new(calc_pos(viewport.min.x, level_scale), calc_pos(viewport.min.y, level_scale)),
	// 			Point2D::new(calc_pos(viewport.max.x, level_scale), calc_pos(viewport.max.y, level_scale)),
	// 		)
	// 	}

	// 	pub fn get_tiles(&mut self,  tile_viewport: &Box2D<f32>) -> Vec<TilePosition> {
	// 		let level_scale = self.get_level_scale() as f32;
	// 		let mut out = Vec::new();
	// 		for y in (tile_viewport.min.y as i32)..=(tile_viewport.max.y as i32) {
	// 			for x in (tile_viewport.min.x as i32)..=(tile_viewport.max.x as i32) {
	// 				let x = ((x as f32 / level_scale).rem_euclid(1.0) * level_scale) as u32;
	// 				let y = ((y as f32 / level_scale).rem_euclid(1.0) * level_scale) as u32;
	// 
	// 				let pos = TilePosition {
	// 					zoom: self.zoom as u8,
	// 					x,
	// 					y,
	// 				};
	// 
	// 				out.push(pos);
	// 			}
	// 		}
	// 		out
	// 	}

	pub fn get_scale(&self) -> f32 {
		2f32.powf(self.zoom)
	}

	pub fn get_level_scale(&self) -> u32 {
		2u32.pow(self.zoom as u32)
	}
}
