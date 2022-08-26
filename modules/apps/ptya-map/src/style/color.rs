use egui::{Color32, Rgba};
use material_color_utilities_rs::blend::harmonize;

use ptya_core::animation::Lerp;

use crate::{ColorTag, Theme};

#[derive(Default)]
pub struct MapColorManager {
	pub border_0: [f32; 4],
	pub border_1: [f32; 4],
	pub border_2: [f32; 4],
	// Area
	pub water: [f32; 4],
	// Roads
	pub major_road: [f32; 4],
	pub big_road: [f32; 4],
	pub medium_road: [f32; 4],
	pub small_road: [f32; 4],
	pub walkway: [f32; 4],
	// Buildings
	pub hospital: [f32; 4],
	pub airport: [f32; 4],
	pub school: [f32; 4],
	pub park: [f32; 4],
	pub beach: [f32; 4],
}

impl MapColorManager {
	pub fn mapbox() -> MapColorManager {
		MapColorManager {
			border_0: Self::hex(0xe2e3e4),
			border_1: Self::hex(0xe2e3e4),
			border_2: Self::hex(0xe2e3e4),
			water: Self::hex(0x06151e),
			major_road: Self::hex(0x556577),
			big_road: Self::hex(0x556577),
			medium_road: Self::hex(0x556577),
			small_road: Self::hex(0x556577),
			walkway: Self::hex(0x273954),
			hospital: Self::hex(0x3d2929),
			airport: Self::hex(0x18304e),
			school: Self::hex(0x141e29),
			park: Self::hex(0x122623),
			beach: Self::hex(0x030507)
		}
	}

	pub fn quantize(self, theme: &Theme) -> MapColorManager {
		let design_color = Self::color32_argb(theme.primary.color);
		MapColorManager {
			border_0: Self::fix(design_color, self.border_0),
			border_1: Self::fix(design_color, self.border_1),
			border_2: Self::fix(design_color, self.border_2),
			water: Self::fix(design_color, self.water),
			major_road: Self::fix(design_color, self.major_road),
			big_road: Self::fix(design_color, self.big_road),
			medium_road: Self::fix(design_color, self.medium_road),
			small_road: Self::fix(design_color, self.small_road),
			walkway: Self::fix(design_color, self.walkway),
			hospital: Self::fix(design_color, self.hospital),
			airport: Self::fix(design_color, self.airport),
			school: Self::fix(design_color, self.school),
			park: Self::fix(design_color, self.park),
			beach:Self::fix(design_color, self.beach),
		}
	}

	fn fix(color: [u8; 4], accent: [f32; 4]) -> [f32; 4] {
		Self::f2(harmonize(Self::f(accent), color))
	}

	fn f([r,g,b,a]: [f32; 4]) -> [u8; 4] {
		let color: Color32 = Rgba::from_rgba_premultiplied(
			r,
			g,
			b,
			a,
		).into();
		[
			color.a(),
			color.r(),
			color.g(),
			color.b(),
		]
	}

	fn f2(argb: [u8; 4]) -> [f32; 4] {
		let color: Rgba = Color32::from_rgba_premultiplied(
			argb[1],
			argb[2],
			argb[3],
			argb[0],
		).into();
		color.to_array()
	}

	fn color32_argb(v: Color32) ->  [u8; 4] {
		[
			v.a(),
			v.r(),
			v.g(),
			v.b(),
		]
	}

	pub fn new(theme: &Theme) -> MapColorManager {
		MapColorManager {
			border_0: Self::neutral(theme, 0.4),
			border_1: Self::neutral(theme, 0.3),
			border_2: Self::neutral(theme, 0.1),
			water: Self::colored(theme, ColorTag::Primary, 0.025),
			// Roads
			major_road: Self::colored(theme, ColorTag::Primary, 0.8),
			big_road: Self::colored(theme, ColorTag::Primary, 0.6),
			medium_road: Self::colored(theme, ColorTag::Primary, 0.5),
			small_road: Self::colored(theme, ColorTag::Primary, 0.4),
			walkway: Self::colored(theme, ColorTag::Primary, 0.1),
			// Buildings
			hospital: Self::colored(theme, ColorTag::Red, 0.15),
			airport: Self::colored(theme, ColorTag::Blue, 0.1),
			school: Self::colored(theme, ColorTag::Orange, 0.15),
			park: Self::colored(theme, ColorTag::Green, 0.15),
			beach: Self::colored(theme, ColorTag::Yellow, 0.15),
		}
	}

	fn hex(rgb: u32) -> [f32; 4] {
		let color = Color32::from_rgb(
			((rgb >> 16) & 0xFF) as u8,
			((rgb >> 8) & 0xFF) as u8,
			((rgb >> 0) & 0xFF) as u8,
		);

		let rgba: Rgba = color.into();
		rgba.to_array()
	}

	fn colored(theme: &Theme, tag: ColorTag, opacity: f32) -> [f32; 4] {
		let value = theme.get(tag);
		let x: Rgba = theme.bg.lerp(&value.color, opacity).into();
		x.to_array()
	}

	fn neutral(theme: &Theme,  opacity: f32) -> [f32; 4] {
		let x: Rgba = theme.bg.lerp(&theme.fg, opacity).into();
		x.to_array()
	}
}
