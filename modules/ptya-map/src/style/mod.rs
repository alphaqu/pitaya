use egui::{Color32, Stroke, Vec2};
use fxhash::FxHashMap;
use lyon_tessellation::{LineCap, TessellationResult};
use ptya_common::ui::animation::spectrum::{LerpSpectrum, Spectrum};
use crate::graphics::painter::Tessellator;
use crate::style::component::{Component, ComponentStyle};
use crate::Tile;
use crate::tile::{Geometry, GeomType};

pub mod component;
pub mod filter;
pub mod style;


type LayerNameLookup = FxHashMap<String, Vec<usize>>;

pub struct MapStyle {
	components: Vec<Component>,

	// heavy caching shit
	lookup: LayerNameLookup
}

impl MapStyle {
	pub fn render(&self, tile: &Tile, zoom: f32, tess: &mut Tessellator) -> TessellationResult {
		for layer in &tile.layers {
			if let Some(components) = self.lookup.get(&layer.name)  {
				for component in components {
					let component = &self.components[*component];
					for feature in &layer.features {
						component.draw(feature, zoom, tess)?;
					}
				}
			}
		}

		Ok(())
	}
}


