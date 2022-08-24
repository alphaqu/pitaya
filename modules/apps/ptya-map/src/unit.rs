use crate::pos::Zoom;
use mathie::unit::{Unit, UnitCompatibility};

#[derive(Copy, Clone, Debug, Default)]
pub struct MapUnit;

impl Unit for MapUnit {}

#[derive(Copy, Clone, Debug)]
pub struct TileUnit(pub Zoom);

impl Unit for TileUnit {}

impl UnitCompatibility<f64, MapUnit> for TileUnit {
	fn convert_value(&self, value: f64, unit: MapUnit) -> Option<f64> {
		let tile_num = self.0.get_num_tiles() as f64;
		Some(value * tile_num)
	}
}

impl UnitCompatibility<f64, TileUnit> for MapUnit {
	fn convert_value(&self, value: f64, unit: TileUnit) -> Option<f64> {
		let tile_num = unit.0.get_num_tiles() as f64;
		Some(value / tile_num)
	}
}
