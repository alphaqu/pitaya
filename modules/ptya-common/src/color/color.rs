use crate::color::theme::{Theme, ThemeColors};
use crate::ui::animation::lerp::Lerp;
use egui::{Color32, Response};
use material_color_utilities_rs::blend::harmonize;
use material_color_utilities_rs::palettes::core::CorePalette;
use material_color_utilities_rs::score::score;
use palette::convert::FromColorUnclamped;
use palette::rgb::{FromHexError, Rgb, Rgba};
use palette::{Hue, Lab, Lch, Srgb};
use std::collections::HashMap;
use std::str::FromStr;
use material_color_utilities_rs::palettes::tonal::TonalPalette;
use material_color_utilities_rs::scheme::Scheme;

#[derive(Default, Clone)]
pub struct ColorSettings {
    pub theme_id: String,
    // Primaries
    pub primary: ColorGroup,
    pub secondary: ColorGroup,
    pub tertiary: ColorGroup,
    // Colors
    pub red: ColorGroup,
    pub orange: ColorGroup,
    pub yellow: ColorGroup,
    pub green: ColorGroup,
    pub blue: ColorGroup,
    //
    pub neutral: ColorGroup,
    pub shadow: Color32,
}

impl ColorSettings {
    pub fn new(theme: &Theme) -> Result<ColorSettings, FromHexError> {

        let red = ColorGroup::new(&theme.base)?;
        Ok(ColorSettings {
            theme_id: theme.id.to_string(),
            primary: ColorGroup::new(&theme.primary)?,
            secondary: ColorGroup::new(&theme.secondary)?,
            tertiary: ColorGroup::new(&theme.tertiary)?,
            red: red.hue(0.0),
            orange: red.hue(30.0),
            yellow: red.hue(60.0),
            green: red.hue(120.0),
            blue: red.hue(240.0),
            neutral: ColorGroup::new(&theme.neutral)?,
            shadow: parse_color(0x000000).linear_multiply(0.1),
        })
    }

    pub fn material(source_argb: [u8; 4], dark_mode: bool) -> ColorSettings {
        let mut palette = CorePalette::new(source_argb, false);
        ColorSettings {
            theme_id: "".to_string(),
            primary: ColorGroup::from_tonal(&mut palette.a1, dark_mode),
            secondary: ColorGroup::from_tonal(&mut palette.a2, dark_mode),
            tertiary: ColorGroup::from_tonal(&mut palette.a3, dark_mode),
            red: ColorGroup::custom(source_argb, [0xff, 0xff, 0x00, 0x00], dark_mode),
            orange: ColorGroup::custom(source_argb, [0xff, 0xff, 0x80, 0x00], dark_mode),
            yellow: ColorGroup::custom(source_argb, [0xff, 0xf5, 0xd7, 0x41], dark_mode),
            green: ColorGroup::custom(source_argb, [0xff, 0x90, 0xa7, 0x49], dark_mode),
            blue: ColorGroup::custom(source_argb, [0xff, 0x41, 0xd5, 0xd7], dark_mode),
            neutral:  {
                let tones = &mut palette.n1;
                let other = &mut palette.n2;
                if dark_mode {
                    ColorGroup {
                        color: parse_argb(tones.tone(10)),
                        on_color: parse_argb(tones.tone(90)),
                        color_container: parse_argb(other.tone(30)),
                        on_color_container: parse_argb(other.tone(80)),
                    }
                } else {
                    ColorGroup {
                        color: parse_argb(tones.tone(99)),
                        on_color: parse_argb(tones.tone(10)),
                        color_container: parse_argb(other.tone(90)),
                        on_color_container: parse_argb(other.tone(30)),
                    }
                }
            },
            shadow: Color32::BLACK.linear_multiply(0.1)
        }
    }

    pub fn get_style(&self, ty: ColorType) -> &ColorGroup {
        match ty {
            // Primary
            ColorType::Primary => &self.primary,
            ColorType::Secondary => &self.secondary,
            ColorType::Tertiary => &self.tertiary,
            // Colors
            ColorType::Red => &self.red,
            ColorType::Orange => &self.orange,
            ColorType::Yellow => &self.yellow,
            ColorType::Green => &self.green,
            ColorType::Blue => &self.blue,
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
            }
            //else if response.clicked() {
            //    ColorState::Press
            //} else if response.has_focus() {
            //    ColorState::Focus
            //} else if response.hovered() {
            //    ColorState::Hover
            //}
            else {
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
    Green,
    Blue,
    // Neutral
    Neutral,
}

#[derive(Default, Clone, PartialEq)]
pub struct ColorGroup {
    // primary
    pub color: Color32,
    pub on_color: Color32,
    // container
    pub color_container: Color32,
    pub on_color_container: Color32,
}

impl ColorGroup {
    pub fn new(theme: &ThemeColors) -> Result<ColorGroup, FromHexError> {
        let bg = Rgb::<palette::encoding::Srgb, u8>::from_str(&theme.bg)?;
        let fg = Rgb::<palette::encoding::Srgb, u8>::from_str(&theme.fg)?;
        let c_bg = Rgb::<palette::encoding::Srgb, u8>::from_str(&theme.c_bg)?;
        let c_fg = Rgb::<palette::encoding::Srgb, u8>::from_str(&theme.c_fg)?;
        Ok(ColorGroup {
            color: Color32::from_rgb(bg.red, bg.green, bg.blue),
            on_color: Color32::from_rgb(fg.red, fg.green, fg.blue),
            color_container: Color32::from_rgb(c_bg.red, c_bg.green, c_bg.blue),
            on_color_container: Color32::from_rgb(c_fg.red, c_fg.green, c_fg.blue),
        })
    }

    fn custom(source: [u8; 4], color: [u8; 4], dark_mode: bool) -> ColorGroup {
        let value = harmonize(color, source);
        let mut palette = CorePalette::new(value, false);
        Self::from_tonal(&mut palette.a1, dark_mode)
    }

    fn from_tonal(tones: &mut TonalPalette, dark_mode: bool) -> ColorGroup {
        if dark_mode {
            ColorGroup {
                color: parse_argb(tones.tone(80)),
                on_color: parse_argb(tones.tone(20)),
                color_container: parse_argb(tones.tone(30)),
                on_color_container: parse_argb(tones.tone(90)),
            }
        } else {
            ColorGroup {
                color: parse_argb(tones.tone(40)),
                on_color: parse_argb(tones.tone(100)),
                color_container: parse_argb(tones.tone(90)),
                on_color_container: parse_argb(tones.tone(10)),
            }
        }
    }

    pub fn hue(&self, hue: f32) -> ColorGroup {
        let bg = labify(self.color, hue);
        let fg = labify(self.on_color, hue);
        let c_bg = labify(self.color_container, hue);
        let c_fg = labify(self.on_color_container, hue);
        ColorGroup {
            color: bg,
            on_color: fg,
            color_container: c_bg,
            on_color_container: c_fg,
        }
    }
}

fn parse_argb(argb: [u8; 4]) -> Color32 {
    Color32::from_rgba_premultiplied(argb[1], argb[2], argb[3], argb[0])
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
