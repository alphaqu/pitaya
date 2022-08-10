use crate::feature::FeatureData;
use crate::geometry::{GeometryData};

use crate::style::{FeatureStyleType, LayerStyle};
use crate::tile::TileMeshBuilder;

pub struct LayerData {
	pub name: String,
	pub features: Vec<FeatureData>
}

pub struct LayerManager {
	pub style: LayerStyle
}

impl LayerManager {
	pub fn compile(&self, layer: LayerData, scale: f32, zoom: u8, builder: &mut TileMeshBuilder) {
		for data in layer.features {
			if let Some(styler) = self.compile_styler(&data) {
				match (data.geometry, styler) {
					(GeometryData::Path(geometry), FeatureStyleType::Path(styler)) => {
						builder.push_feature(geometry, styler, data.fields, scale, zoom);
					}
				}
			}
		}
	}

	fn compile_styler(&self, data: &FeatureData) -> Option<FeatureStyleType> {
		'main: for styler in &self.style.features {
			if styler.style.get_kind() == data.geometry.get_kind() {
				for predicate in &styler.predicates {
					let value = data.fields.fields.get(&predicate.field);
					if !predicate.condition.check(value) {
						continue 'main;
					}
				}

				return Some(styler.style.clone());
			}
		}

		None
	}
}