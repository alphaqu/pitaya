use std::fs::{OpenOptions};
use egui::{Color32, Rounding, Stroke, Vec2};

pub const ANIMATION_TIME: f32 = 0.25;



#[derive(Copy, Clone)]
pub struct StyleSettings {
	pub bg_body: Color32,
	pub bg_0: Color32,
	pub bg_1: Color32,
	pub bg_2: Color32,
	pub bg_3: Color32,
	pub bg_4: Color32,

	pub fg_0: Color32,
	pub fg_1: Color32,
	pub fg_2: Color32,
	pub fg_3: Color32,
	pub fg_4: Color32,
	pub fg_5: Color32,


	pub red: Color32,
	pub orange: Color32,
	pub yellow: Color32,
	pub green: Color32,
	pub cyan: Color32,
	pub blue: Color32,
	pub purple: Color32,
}

#[derive(Copy, Clone)]
pub enum StyleColor {
	Custom(Color32),
	Bg0,
	Bg1,
	Bg2,
	Bg3,
	Bg4,
	Fg0,
	Fg1,
	Fg2,
	Fg3,
	Fg4,
	Red,
	Orange,
	Yellow,
	Green,
	Cyan,
	Blue,
	Purple,
}


impl StyleSettings {
	pub fn get_color(&self, color: StyleColor) -> Color32 {
		match color {
			StyleColor::Bg0 => self.bg_0,
			StyleColor::Bg1 => self.bg_1,
			StyleColor::Bg2 => self.bg_2,
			StyleColor::Bg3 => self.bg_3,
			StyleColor::Bg4 => self.bg_4,
			StyleColor::Fg0 => self.fg_0,
			StyleColor::Fg1 => self.fg_1,
			StyleColor::Fg2 => self.fg_2,
			StyleColor::Fg3 => self.fg_3,
			StyleColor::Fg4 => self.fg_4,
			StyleColor::Red => self.red,
			StyleColor::Orange => self.orange,
			StyleColor::Yellow => self.yellow,
			StyleColor::Green => self.green,
			StyleColor::Cyan => self.cyan,
			StyleColor::Blue => self.blue,
			StyleColor::Purple => self.purple,
			StyleColor::Custom(color) => color,
		}
	}

	pub fn pitaya_dark() -> StyleSettings {
		StyleSettings {
			bg_body: color("#131415"),
			bg_0: color("#1c1d1f"),
			bg_1: color("#252629"),
			bg_2: color("#2e2f32"),
			bg_3: color("#37383c"),
			bg_4: color("#404146"),
			fg_0: color("#5b5c61"),
			fg_1: color("#7f7f83"),
			fg_2: color("#a5a5a8"),
			fg_3: color("#ccccce"),
			fg_4: color("#f4f4f5"),
			fg_5: color("#ffffff"),
			red: color("#ff6188"),
			orange: color("#fc9867"),
			yellow: color("#ffd866"),
			green: color("#9ee05c"),
			cyan: color("#78dce8"),
			blue: color("#85aaf2"),
			purple: color("#ab9df2")
		}
	}
}


fn color(mut string: &str) -> Color32 {
	if string.starts_with('#') {
		string = &string[1..];
	}


	let r = u8::from_str_radix(&string[0..2], 16).unwrap();
	let g = u8::from_str_radix(&string[2..4], 16).unwrap();
	let b = u8::from_str_radix(&string[4..6], 16).unwrap();
	Color32::from_rgb(r, g, b)
}