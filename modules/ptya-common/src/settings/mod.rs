use egui::Vec2;
use crate::StyleSettings;

pub mod style;

pub struct Settings {
	pub max_widgets: usize,
	pub rounding: f32,
	pub margin: Vec2,

	pub style: StyleSettings,
	pub layout: LayoutSettings,
}

pub struct LayoutSettings {
	pub keyboard_size: f32,
	
	pub button_rounding: f32,
	pub button_padding: Vec2,
	
	pub window_control_size: Vec2,
	
	pub content_margin: f32,
	pub widget_width: f32,
	pub widget_add_size: f32,
	#[deprecated]
	pub widget_padding: f32
}
