use egui::Color32;
use material_color_utilities_rs::palettes::core::CorePalette;
use crate::color::{ColorGroup, ColorTag};

#[derive(Default, Clone)]
pub struct Theme {
    // Accents
    pub primary: ColorGroup,
    pub secondary: ColorGroup,
    pub tertiary: ColorGroup,
    // Colors
    pub red: ColorGroup,
    pub orange: ColorGroup,
    pub yellow: ColorGroup,
    pub green: ColorGroup,
    pub blue: ColorGroup,
    // Neutral
    pub bg: Color32,
    pub fg: Color32,
    pub outline: Color32,
    pub outline_weak: Color32,
    pub shadow: Color32,
}

impl Theme {
    pub async fn new(rgb: [u8; 3], dark_mode: bool) -> Theme {
        let source_argb = [0xff, rgb[0], rgb[1], rgb[2]];
        let mut palette = CorePalette::new(source_argb, false);

        let bg = if dark_mode { 10 } else { 99 };
        let fg = if dark_mode { 90 } else { 10 };
        let outline = if dark_mode { 60 } else { 50 };
        let outline_weak = if dark_mode { 30 } else { 80 };

        Theme {
            primary: ColorGroup::new_tonal(&mut palette.a1, dark_mode).await,
            secondary: ColorGroup::new_tonal(&mut palette.a2, dark_mode).await,
            tertiary: ColorGroup::new_tonal(&mut palette.a3, dark_mode).await,
            red: ColorGroup::new_custom(source_argb, [0xff, 0xff, 0x00, 0x00], dark_mode).await,
            orange: ColorGroup::new_custom(source_argb, [0xff, 0xff, 0x80, 0x00], dark_mode).await,
            yellow: ColorGroup::new_custom(source_argb, [0xff, 0xff, 0xff, 0x00], dark_mode).await,
            green: ColorGroup::new_custom(source_argb, [0xff, 0x00, 0xff, 0x00], dark_mode).await,
            blue: ColorGroup::new_custom(source_argb, [0xff, 0x00, 0xff, 0xff], dark_mode).await,
            bg: color32_from_argb(palette.n1.tone(bg)),
            fg: color32_from_argb(palette.n1.tone(fg)),
            outline: color32_from_argb(palette.n2.tone(outline)),
            outline_weak: color32_from_argb(palette.n2.tone(outline_weak)),
            shadow: Color32::BLACK.linear_multiply(0.1),
        }
    }

    pub fn get(&self, color: ColorTag) -> &ColorGroup {
        match color {
            ColorTag::Primary => &self.primary,
            ColorTag::Secondary => &self.secondary,
            ColorTag::Tertiary => &self.tertiary,
            ColorTag::Red => &self.red,
            ColorTag::Orange => &self.orange,
            ColorTag::Yellow => &self.yellow,
            ColorTag::Green => &self.green,
            ColorTag::Blue => &self.blue,
        }
    }
}

#[inline(always)]
pub(crate) fn color32_from_argb(argb: [u8; 4]) -> Color32 {
    Color32::from_rgba_premultiplied(argb[1], argb[2], argb[3], argb[0])
}
