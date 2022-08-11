use crate::pos::Zoom;
use mathie::unit::{Unit, UnitCompatibility};

#[derive(Copy, Clone, Debug, Default)]
pub struct MapUnit;

impl Unit for MapUnit {}

#[derive(Copy, Clone, Debug)]
pub struct TileUnit(pub Zoom);

impl Unit for TileUnit {}

impl UnitCompatibility<f32, MapUnit> for TileUnit {
    fn convert_value(&self, value: f32, unit: MapUnit) -> Option<f32> {
        let tile_num = self.0.get_num_tiles() as f32;
        Some(value * tile_num)
    }
}

impl UnitCompatibility<f32, TileUnit> for MapUnit {
	fn convert_value(&self, value: f32, unit: TileUnit) -> Option<f32> {
		let tile_num = unit.0.get_num_tiles() as f32;
		Some(value / tile_num)
	}
}
