use crate::asset::Location;
use crate::color::color::{ColorSettings, ColorState, ColorType};
use crate::color::theme::{Theme, ThemeColors};
use crate::ui::animation::lerp::Lerp;
use crate::{AssetComp, Settings};
use anyways::ext::AuditExt;
use epaint::{Color32, Shadow};
use log::{debug, error};

pub mod color;
pub mod theme;

pub struct ColorComp {
    pub themes: Vec<Theme>,
    pub color: ColorSettings,
}

impl ColorComp {
    pub fn init() -> ColorComp {
        ColorComp {
            themes: vec![],
            color: ColorSettings::new(
                &Theme::fallback(),
            )
            .unwrap(),
        }
    }

    pub async fn new(asset: &AssetComp, settings: &Settings) -> anyways::Result<ColorComp> {
        let paths = asset
            .read_dir(Location::Assets, "./theme")
            .await
            .wrap_err("Failed to get theme directory")?;

        let mut themes = Vec::new();
        for path in paths {
            let data = match asset.read_file(Location::Assets, path).await {
                Ok(data) => data,
                Err(err) => {
                    error!("Failed to read theme file: {err}");
                    continue;
                }
            };

            let theme: Theme = match serde_json::from_slice(&data) {
                Ok(data) => data,
                Err(err) => {
                    error!("Failed to parse theme file: {err}");
                    continue;
                }
            };

            debug!("Loaded {}", theme.id);
            themes.push(theme);
        }

        let fallback = Theme::fallback();
        let theme = themes.iter().find(|v| v.id == settings.current_theme).unwrap_or(&fallback);

        const PITAYA_RED: [u8; 4] = [0xff, 0xe5, 0x4c, 0x64];

        Ok(ColorComp {
            color: ColorSettings::material(PITAYA_RED, true),
            themes,
        })
    }

    pub fn inactive_bg(&self) -> Color32 {
        Color32::lerp_static(&Color32::TRANSPARENT, &self.color.neutral.on_color_container, 0.12)
    }

    pub fn inactive_fg(&self) -> Color32 {
        Color32::lerp_static(&Color32::TRANSPARENT, &self.color.neutral.on_color_container, 0.38)
    }

    pub fn shadow(&self) -> Shadow {
        Shadow {
            extrusion: 10.0,
            color: self.color.shadow
        }
    }

    pub fn compose_bg(&self, value: f32, accent: Color32, state: ColorState) -> Color32  {
        self.compose(value, self.color.neutral.color, accent, state)
    }
    pub fn compose(&self, value: f32, bg: Color32, accent: Color32, state: ColorState) -> Color32 {
        Color32::lerp_static(&bg, &accent, (value * 0.08) + state.get_opacity_boost())
    }


    pub fn bg(&self, value: f32, ty: ColorType, state: ColorState) -> Color32 {
        if state.is_active() {
            let style = self.color.get_style(ty);
            let bg = self.color.neutral.color;
            self.compose(value, bg, style.color, state)
        } else {
            self.inactive_bg()
        }
    }

    pub fn c_bg(&self, value: f32, ty: ColorType, state: ColorState) -> Color32 {
        if state.is_active() {
            let style = self.color.get_style(ty);
            let bg = style.color_container;
            self.compose(value, bg, style.on_color_container, state)
        } else {
            self.inactive_bg()
        }
    }

    pub fn fg(&self, ty: ColorType, state: ColorState) -> Color32 {
        if state.is_active() {
            self.color.get_style(ty).on_color
        } else {
            self.inactive_fg()
        }
    }

    pub fn c_fg(&self, ty: ColorType, state: ColorState) -> Color32 {
        if state.is_active() {
            self.color.get_style(ty).on_color_container
        } else {
            self.inactive_fg()
        }
    }
}
