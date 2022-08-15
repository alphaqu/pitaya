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
        Ok(ColorComp {
            color: ColorSettings::new(theme)?,
            themes,
        })
    }

    pub fn inactive_bg(&self) -> Color32 {
        Color32::lerp(&Color32::TRANSPARENT, &self.color.neutral.c_fg, 0.12)
    }

    pub fn inactive_fg(&self) -> Color32 {
        Color32::lerp(&Color32::TRANSPARENT, &self.color.neutral.c_fg, 0.38)
    }

    pub fn shadow(&self) -> Shadow {
        Shadow {
            extrusion: 10.0,
            color: self.color.shadow
        }
    }

    pub fn bg(&self, value: f32, ty: ColorType, state: ColorState) -> Color32 {
        if state.is_active() {
            let style = self.color.get_style(ty);
            let bg = self.color.neutral.bg;
            Color32::lerp(&bg, &style.bg, (value * 0.045) + state.get_opacity_boost())
        } else {
            self.inactive_bg()
        }
    }

    pub fn c_bg(&self, value: f32, ty: ColorType, state: ColorState) -> Color32 {
        if state.is_active() {
            let style = self.color.get_style(ty);
            let bg = style.c_bg;
            Color32::lerp(
                &bg,
                &style.c_fg,
                (value * 0.045) + state.get_opacity_boost(),
            )
        } else {
            self.inactive_bg()
        }
    }

    pub fn fg(&self, ty: ColorType, state: ColorState) -> Color32 {
        if state.is_active() {
            self.color.get_style(ty).fg
        } else {
            self.inactive_fg()
        }
    }

    pub fn c_fg(&self, ty: ColorType, state: ColorState) -> Color32 {
        if state.is_active() {
            self.color.get_style(ty).c_fg
        } else {
            self.inactive_fg()
        }
    }
}
