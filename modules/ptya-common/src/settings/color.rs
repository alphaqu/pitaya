use egui::Color32;
use crate::ui::animation::lerp::Lerp;

pub struct ColorSettings {
    // flavors
    pub primary: ColorStyle,
    pub secondary: ColorStyle,
    pub tertiary: ColorStyle,
    // notification
    pub info: ColorStyle,
    pub warning: ColorStyle,
    pub error: ColorStyle,
    //
    pub neutral: ColorStyle,
	pub shadow: Color32,
}

impl ColorSettings {
	pub fn get_style(&self, ty: ColorType) -> &ColorStyle {
		match ty {
			ColorType::Primary => &self.primary,
			ColorType::Secondary => &self.secondary,
			ColorType::Tertiary => &self.tertiary,
			ColorType::Info => &self.info,
			ColorType::Warning => &self.warning,
			ColorType::Error => &self.error,
			ColorType::Neutral => &self.neutral,
		}
	}

	pub fn bg(&self, value: f32, ty: ColorType) -> Color32 {
		let style = self.get_style(ty);
		let bg = self.neutral.bg;
		Color32::lerp(&bg, &style.bg, value * 0.045)
	}

	pub fn fg(&self, ty: ColorType) -> Color32 {
		let style = self.get_style(ty);
		style.fg
		//let bg = style.fg;
		//Color32::lerp(&bg, &style.fg, value * 0.045)
	}
}

pub enum ColorType {
	Primary,
	Secondary,
	Tertiary,
	Info,
	Warning,
	Error,
	Neutral
}

impl Default for ColorSettings {
    fn default() -> Self {
        ColorSettings {
            primary: ColorStyle {
                bg: parse_color(0xD0BCFF),
                fg: parse_color(0x371E73),
                c_bg: parse_color(0x4F378B),
                c_fg: parse_color(0xEADDFF),
            },
            secondary: ColorStyle {
	            bg: parse_color(0xCCC2DC),
	            fg: parse_color(0x332D41),
	            c_bg: parse_color(0x4A4458),
	            c_fg: parse_color(0xE8DEF8),
            },
            tertiary: ColorStyle {
	            bg: parse_color(0xEFB8C8),
	            fg: parse_color(0x492532),
	            c_bg: parse_color(0x633B48),
	            c_fg: parse_color(0xFFD8E4),
            },
            info: ColorStyle {
	            bg: parse_color(0x000000),
	            fg: parse_color(0x000000),
	            c_bg: parse_color(0x000000),
	            c_fg: parse_color(0x000000),
            },
            warning: ColorStyle {
	            bg: parse_color(0x000000),
	            fg: parse_color(0x000000),
	            c_bg: parse_color(0x000000),
	            c_fg: parse_color(0x000000),
            },
            error: ColorStyle {
	            bg: parse_color(0xF2B8B5),
	            fg: parse_color(0x601410),
	            c_bg: parse_color(0x8C1D18),
	            c_fg: parse_color(0xF9DEDC),
            },
            neutral: ColorStyle {
	            bg: parse_color(0x1C1B1F),
	            fg: parse_color(0xE6E1E5),
	            c_bg: parse_color(0x1c1b1f),
	            c_fg: parse_color(0xE6E1E5),
            },
	        shadow: parse_color(0x000000).linear_multiply(0.1)
        }
    }
}

pub struct ColorStyle {
    // primary
    pub bg: Color32,
    pub fg: Color32,
    // container
    pub c_bg: Color32,
    pub c_fg: Color32,
}

fn parse_color(val: u32) -> Color32 {
    Color32::from_rgb(
        ((val >> 16) & 0xFF) as u8,
        ((val >> 8) & 0xFF) as u8,
        (val & 0xFF) as u8,
    )
}
