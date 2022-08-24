use material_color_utilities_rs::blend::harmonize;
use material_color_utilities_rs::palettes::core::CorePalette;
use material_color_utilities_rs::palettes::tonal::TonalPalette;
use egui::Color32;
use ptya_animation::Lerp;
use crate::theme::color32_from_argb;

#[derive(Clone, PartialEq, Eq, Default)]
pub struct ColorGroup {
	pub color: Color32,
	pub on_color: Color32,
	pub color_container: Color32,
	pub on_color_container: Color32,
}

impl ColorGroup {
	pub async fn new_custom(source: [u8; 4], color: [u8; 4], dark_mode: bool) -> ColorGroup {
		let value = harmonize(color, source);
		let mut palette = CorePalette::new(value, false);
		Self::new_tonal(&mut palette.a1, dark_mode).await
	}
	
	pub async fn new_tonal(tones: &mut TonalPalette, dark_mode: bool) -> ColorGroup {
		if dark_mode {
			ColorGroup {
				color: color32_from_argb(tones.tone(80)),
				on_color: color32_from_argb(tones.tone(20)),
				color_container: color32_from_argb(tones.tone(30)),
				on_color_container: color32_from_argb(tones.tone(90)),
			}
		} else {
			ColorGroup {
				color: color32_from_argb(tones.tone(40)),
				on_color: color32_from_argb(tones.tone(100)),
				color_container: color32_from_argb(tones.tone(90)),
				on_color_container: color32_from_argb(tones.tone(10)),
			}
		}
	}
}

impl Lerp for ColorGroup {
	fn lerp_static(v0: &Self, v1: &Self, t: f32) -> Self {
		ColorGroup {
			color: v0.color.lerp(&v1.color, t),
			on_color: v0.on_color.lerp(&v1.on_color, t),
			color_container: v0.color_container.lerp(&v1.color_container, t),
			on_color_container: v0.on_color_container.lerp(&v1.on_color_container, t)
		}
	}
}

pub enum ColorTag {
	Primary,
	Secondary,
	Tertiary,
	Red,
	Orange,
	Yellow,
	Green,
	Blue
}