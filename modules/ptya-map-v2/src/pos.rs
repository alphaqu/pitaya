use mathie::{Rect, Vec2D};
use crate::unit::MapUnit;

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct TilePosition {
	pub zoom: Zoom,
	pub x: u32,
	pub y: u32,
}

impl TilePosition {
	/// Gets the parent of this tile position. The parent is the tile on the above zoom level holding this tile.
	pub fn get_parent(&self) -> Option<TilePosition> {
		if let Some(zoom) = self.zoom.get_parent() {
			Some(TilePosition {
				zoom,
				x: self.x / 2,
				y: self.y / 2
			})
		} else {
			None
		}
	}

	/// Gets the position of the tile in Map Space
	#[inline(always)]
	pub fn get_pos(&self) -> Vec2D<f32, MapUnit> {
		let num_tiles = self.zoom.get_num_tiles();
		Vec2D::new(
			self.x as f32 / num_tiles as f32,
			self.y as f32 / num_tiles as f32,
		)
	}

	/// Gets the size of the tile in Map Space
	#[inline(always)]
	pub fn get_size(&self) -> Vec2D<f32, MapUnit> {
		let size = 1.0 / self.zoom.get_num_tiles() as f32;
		Vec2D::new(
			size,
			size
		)
	}
	
	/// Gets the rectangle that this tile covers in map space.
	#[inline(always)]
	pub fn get_rect(&self) -> Rect<f32, MapUnit> {
		Rect::new_u(
			self.get_pos(),
			self.get_size(),
			MapUnit::default()
		)
	}

	pub fn is_valid(&self) -> bool {
		let scale = self.zoom.get_num_tiles();
		self.x < scale && self.y < scale
	}

	
	pub fn get_file_name(&self) -> String {
		format!("tile-{}-{}x{}.mvt", self.zoom.zoom, self.x, self.y)
	}
	
	pub fn parse_file_name(text: &str) -> Option<Self> {
		let (_, value) = text.split_once("tile-")?;
		let (zoom, value) = value.split_once("-")?;
		let (x, y) = value.split_once("x")?;
		Some(TilePosition {
			zoom: Zoom::new(zoom.parse().ok()?)?,
			x: x.parse().ok()?,
			y: y.parse().ok()?
		})
	}

}


#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Zoom {
	pub zoom: u8
}

impl Zoom {
	pub fn new(value: u8) -> Option<Zoom> {
		if value <= 23 {
			Some(Zoom {
				zoom: value
			})
		} else {
			None
		}
	}
	
	pub fn get_parent(&self) -> Option<Zoom> {
		if self.zoom != 0 {
			Zoom::new(self.zoom - 1)
		} else {
			None
		}
	}
	
	pub fn get_child(&self) -> Option<Zoom> {
		if self.zoom != 255 {
			Zoom::new(self.zoom + 1)
		} else {
			None
		}
	}

	#[inline(always)]
	pub const fn get_num_tiles(&self) -> u32 {
		2u32.pow(self.zoom as u32)
	}
}

