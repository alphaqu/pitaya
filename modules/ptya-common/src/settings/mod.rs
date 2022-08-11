use egui::Vec2;
use crate::settings::color::ColorSettings;
use crate::StyleSettings;

pub mod style;
pub mod color;

pub struct Settings {
	pub max_widgets: usize,

	#[deprecated]
	pub style: StyleSettings,
	pub color: ColorSettings,
	pub layout: LayoutSettings,
}

pub struct LayoutSettings {
	pub spacing_size: f32,
	pub rounding_size: f32,
	pub interactive_size: f32,

	pub widget_width: f32,
	pub widget_add_size: f32,
}

impl LayoutSettings {
	pub fn new() -> LayoutSettings {
		LayoutSettings {
			spacing_size: 20.0,
			rounding_size: 25.0,
			interactive_size: 110.0,
			widget_width: 440.0,
			widget_add_size: 150.0,
		}
	}
}
