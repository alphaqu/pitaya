use crate::data::{FeatureData, Value};
use crate::geometry::GeometryData;
use crate::style::fill::FillStyle;
use crate::style::stroke::StrokeStyle;
use crate::types::MapVertex;
use std::any::Any;
use std::ops::Range;

pub mod fill;
pub mod stroke;

pub(crate) struct StyleAllocation {
	pub old_style: Box<dyn Any + Send>,
	pub vertices_range: Range<usize>,
}

/// A style declares a style of a geometry values.
pub trait Style: Copy + Clone + Any + Send {
	type Input<'a>: Copy + Clone;

	fn get_len(input: Self::Input<'_>) -> usize;
	fn prepare(&mut self, scale: f32);
	fn needs_update(&self, old_styler: Self) -> bool;
	fn compile(&self, input: Self::Input<'_>, v: &mut Vec<MapVertex>, i: &mut Vec<u32>);
	fn update(&self, input: Self::Input<'_>, v: &mut [MapVertex], old_styler: Option<Self>);
}

/// A styler submits all of the styles it wants to use on a feature.
pub trait Styler {
	/// Visits all of the features and submits all of their styles.
	///
	/// **FEATURES SLICE DOES NOT CHANGE AND THIS METHOD SHOULD ALWAYS GIVE SAME RESULT ON THE SAME INPUTS**
	///
	/// # Arguments
	///
	/// * `handler`: The handler which should consume all of the feature styles.
	/// * `layer`: The current layer name.
	/// * `features`: The features on this layer.
	fn visit_features<S: StyleHandler>(
		&self,
		handler: &mut S,
		layer: &str,
		zoom: f32,
		features: &[FeatureData],
	);

	fn get_z_index(&self, layer: &str) -> f32;
}

pub type ZIndex = f32;

/// A StyleHandler is a consumer of all of the styles that a Styler might submit.
pub trait StyleHandler {
	fn submit<'a, S: Style>(&'a mut self, input: impl Into<S::Input<'a>>, style: S);
}

pub struct StyleBuilder<'a, S: StyleHandler> {
	pub zoom: f32,
	pub feature: &'a FeatureData,
	pub handler: &'a mut S,
}

impl<'a, S: StyleHandler> StyleBuilder<'a, S> {
	pub fn field(&self, name: &str) -> Option<&Value> {
		self.feature.fields.get(name)
	}

	pub fn fill(&mut self, color: [f32; 4]) {
		match &self.feature.geometry {
			GeometryData::Path(_) => {}
			GeometryData::Fill(fill) => {
				let style = FillStyle { color };
				for polygon in &fill.polygons {
					self.handler.submit(polygon, style);
				}
			}
		}
	}

	pub fn stroke(&mut self, color: [f32; 4], width: impl NumberProvider) {
		match &self.feature.geometry {
			GeometryData::Path(path) => {
				let style = StrokeStyle {
					color,
					width: width.get(self.zoom),
				};
				for path in &path.paths {
					self.handler.submit(path, style);
				}
			}
			GeometryData::Fill(_) => {}
		}
	}
}

pub trait NumberProvider {
	fn get(self, zoom: f32) -> f32;
}

pub struct Linear<const V: usize> {
	pub stops: [(f32, f32); V],
}

impl<const V: usize> NumberProvider for Linear<V> {
	fn get(self, zoom: f32) -> f32 {
		let pos = (zoom - self.stops[0].0) / (self.stops[1].0 - self.stops[0].0);
		let color_pos = (pos * (V - 1) as f32).floor() as usize;

		let (low_zoom, low_value) = self.stops[color_pos.clamp(0, V - 1)];
		let (high_zoom, high_value) = self.stops[(color_pos + 1).clamp(0, V - 1)];
		let t = {
			let difference = high_zoom - low_zoom;
			let progress = zoom - low_zoom;

			if difference == 0.0 {
				0.0
			} else {
				progress / difference
			}
		};

		((high_value - low_value) * t.clamp(0.0, 1.0)) + low_value
	}
}


pub struct Exponential<const V: usize> {
	pub base: f32,
	pub stops: [(f32, f32); V],
}

impl<const V: usize> NumberProvider for Exponential<V> {
	fn get(self, zoom: f32) -> f32 {
		let pos = (zoom - self.stops[0].0) / (self.stops[V - 1].0 - self.stops[0].0);
		let color_pos = (pos * (V - 1) as f32).floor() as usize;

		let (low_zoom, low_value) = self.stops[color_pos.clamp(0, V - 1)];
		let (high_zoom, high_value) = self.stops[(color_pos + 1).clamp(0, V - 1)];
		let base = self.base;
		let t = {
			let difference = high_zoom - low_zoom;
			let progress = zoom - low_zoom;

			if difference == 0.0 {
				0.0
			} else if base == 1.0 {
				progress / difference
			} else {
				(base.powf(progress) - 1.0) / (base.powf(difference) - 1.0)
			}
		};

		((high_value - low_value) * t.clamp(0.0, 1.0)) + low_value
	}
}
