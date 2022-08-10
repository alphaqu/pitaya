use crate::pos::Zoom;
use mathie::unit::{Unit, UnitCompatibility};
use mathie::Value;

#[derive(Copy, Clone, Debug, Default)]
pub struct MapUnit;

impl Unit for MapUnit {}

#[derive(Copy, Clone, Debug)]
pub struct TileUnit(pub Zoom);

impl Unit for TileUnit {}

impl UnitCompatibility<f32, MapUnit> for TileUnit {
    fn convert_value(&self, value: Value<f32, MapUnit>) -> Option<Value<f32, Self>> {
        let tile_num = self.0.get_num_tiles() as f32;
        Some(Value::new_u(value.val() * tile_num, *self))
    }
}

impl UnitCompatibility<f32, TileUnit> for MapUnit {
	fn convert_value(&self, value: Value<f32, TileUnit>) -> Option<Value<f32, Self>> {
		let tile_num = value.unit().0.get_num_tiles() as f32;
		Some(Value::new_u(value.val() / tile_num, *self))
	}
}
