use ahash::AHashMap;
use crate::geometry::GeometryData;

pub struct TileData {
	pub layers: Vec<LayerData>,
}

pub struct LayerData {
	pub name: String,
	pub features: Vec<FeatureData>,
}

pub struct FeatureData {
	pub fields: AHashMap<String, Value>,
	pub geometry: GeometryData,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
	String(String),
	F32(f32),
	F64(f64),
	I64(i64),
	U64(u64),
	Bool(bool),
}

impl Value {
	pub fn to_str(&self) -> Option<&str> {
		match self {
			Value::String(string) => Some(&**string),
			Value::F32(_) => None,
			Value::F64(_) => None,
			Value::I64(_) => None,
			Value::U64(_) => None,
			Value::Bool(_) => None,
		}
	}

	pub fn to_f64(&self) -> Option<f64> {
		match self {
			Value::String(_) => None,
			Value::F32(v) => Some(*v as f64),
			Value::F64(v) => Some(*v as f64),
			Value::I64(v) => Some(*v as f64),
			Value::U64(v) => Some(*v as f64),
			Value::Bool(_) => None,
		}
	}
}
