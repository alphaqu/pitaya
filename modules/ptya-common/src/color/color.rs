use crate::color::theme::{Theme, ThemeColors};
use crate::ui::animation::lerp::Lerp;
use egui::{Color32, Response};
use palette::convert::FromColorUnclamped;
use palette::rgb::{FromHexError, Rgb, Rgba};
use palette::{Hue, Lab, Lch, Srgb};
use std::str::FromStr;

#[derive(Default, Clone)]
pub struct ColorSettings {
    pub theme_id: String,
    // flavors
    pub primary: ColorStyle,
    pub secondary: ColorStyle,
    pub tertiary: ColorStyle,
    // notification
    pub red: ColorStyle,
    pub orange: ColorStyle,
    pub yellow: ColorStyle,
    pub spring_green: ColorStyle,
    pub green: ColorStyle,
    pub turquoise: ColorStyle,
    pub cyan: ColorStyle,
    pub ocean: ColorStyle,
    pub blue: ColorStyle,
    pub violet: ColorStyle,
    pub magenta: ColorStyle,
    pub raspberry: ColorStyle,
    //
    pub neutral: ColorStyle,
    pub shadow: Color32,
}

impl ColorSettings {
    pub fn new(theme: &Theme) -> Result<ColorSettings, FromHexError> {
        let red = ColorStyle::new(&theme.base)?;
        Ok(ColorSettings {
            theme_id: theme.id.to_string(),
            primary: ColorStyle::new(&theme.primary)?,
            secondary: ColorStyle::new(&theme.secondary)?,
            tertiary: ColorStyle::new(&theme.tertiary)?,
            red: red.hue(0.0),
            orange: red.hue(30.0),
            yellow: red.hue(60.0),
            spring_green: red.hue(90.0),
            green: red.hue(120.0),
            turquoise: red.hue(150.0),
            cyan: red.hue(180.0),
            ocean: red.hue(210.0),
            blue: red.hue(240.0),
            violet: red.hue(270.0),
            magenta: red.hue(300.0),
            raspberry: red.hue(330.0),
            neutral: ColorStyle::new(&theme.neutral)?,
            shadow: parse_color(0x000000).linear_multiply(0.1),
        })
    }

    pub fn get_style(&self, ty: ColorType) -> &ColorStyle {
        match ty {
            // Primary
            ColorType::Primary => &self.primary,
            ColorType::Secondary => &self.secondary,
            ColorType::Tertiary => &self.tertiary,
            // Colors
            ColorType::Red => &self.red,
            ColorType::Orange => &self.orange,
            ColorType::Yellow => &self.yellow,
            ColorType::SpringGreen => &self.spring_green,
            ColorType::Green => &self.green,
            ColorType::Turquoise => &self.turquoise,
            ColorType::Cyan => &self.cyan,
            ColorType::Ocean => &self.ocean,
            ColorType::Blue => &self.blue,
            ColorType::Violet => &self.violet,
            ColorType::Magenta => &self.magenta,
            ColorType::Raspberry => &self.raspberry,
            // Neutral
            ColorType::Neutral => &self.neutral,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ColorState {
    Idle,
    Inactive,
    Hover,
    Focus,
    Press,
    Drag,
}

impl ColorState {
    pub fn new(response: &Response) -> ColorState {
        if response.enabled() {
            if response.dragged() {
                ColorState::Drag
            } else if response.clicked() {
                println!("press");

                ColorState::Press
            } else if response.has_focus() {
                ColorState::Focus
            } else if response.hovered() {
                ColorState::Hover
            } else {
                ColorState::Idle
            }
        } else {
            ColorState::Inactive
        }
    }

    pub fn get_opacity_boost(&self) -> f32 {
        match self {
            ColorState::Idle => 0.0,
            ColorState::Inactive => 0.0,
            ColorState::Hover => 0.08,
            ColorState::Focus => 0.12,
            ColorState::Press => 0.12,
            ColorState::Drag => 0.16,
        }
    }

    pub fn is_active(&self) -> bool {
        *self != ColorState::Inactive
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ColorType {
    Primary,
    Secondary,
    Tertiary,
    // Colors
    Red,
    Orange,
    Yellow,
    SpringGreen,
    Green,
    Turquoise,
    Cyan,
    Ocean,
    Blue,
    Violet,
    Magenta,
    Raspberry,
    // Neutral
    Neutral,
}

#[derive(Default, Clone)]
pub struct ColorStyle {
    // primary
    pub bg: Color32,
    pub fg: Color32,
    // container
    pub c_bg: Color32,
    pub c_fg: Color32,
}

impl ColorStyle {
    pub fn new(theme: &ThemeColors) -> Result<ColorStyle, FromHexError> {
        let bg = Rgb::<palette::encoding::Srgb, u8>::from_str(&theme.bg)?;
        let fg = Rgb::<palette::encoding::Srgb, u8>::from_str(&theme.fg)?;
        let c_bg = Rgb::<palette::encoding::Srgb, u8>::from_str(&theme.c_bg)?;
        let c_fg = Rgb::<palette::encoding::Srgb, u8>::from_str(&theme.c_fg)?;
        Ok(ColorStyle {
            bg: Color32::from_rgb(bg.red, bg.green, bg.blue),
            fg: Color32::from_rgb(fg.red, fg.green, fg.blue),
            c_bg: Color32::from_rgb(c_bg.red, c_bg.green, c_bg.blue),
            c_fg: Color32::from_rgb(c_fg.red, c_fg.green, c_fg.blue),
        })
    }

    pub fn hue(&self, hue: f32) -> ColorStyle {
        let bg = labify(self.bg, hue);
        let fg = labify(self.fg, hue);
        let c_bg = labify(self.c_bg, hue);
        let c_fg = labify(self.c_fg, hue);
        ColorStyle { bg, fg, c_bg, c_fg }
    }
}

fn lab(color: Color32) -> Lab {
    let rgb = Srgb::new(color.r(), color.g(), color.b());
    let rgb: Srgb<f32> = rgb.into_format();
    Lab::from_color_unclamped(rgb)
}

fn from_lab(lab: Lab) -> Color32 {
    let rgb2 = Srgb::from_color_unclamped(lab);
    let rgb1: Srgb<u8> = rgb2.into_format();
    Color32::from_rgb(rgb1.red, rgb1.green, rgb1.blue)
}

fn labify(color: Color32, hue: f32) -> Color32 {
    let rgb = Srgb::new(color.r(), color.g(), color.b());
    let rgb: Srgb<f32> = rgb.into_format();
    let lab = Lch::from_color_unclamped(rgb).shift_hue(hue);
    let rgb2 = Srgb::from_color_unclamped(lab);
    let rgb1: Srgb<u8> = rgb2.into_format();
    Color32::from_rgb(rgb1.red, rgb1.green, rgb1.blue)
}

fn parse_color(val: u32) -> Color32 {
    Color32::from_rgb(
        ((val >> 16) & 0xFF) as u8,
        ((val >> 8) & 0xFF) as u8,
        (val & 0xFF) as u8,
    )
}
