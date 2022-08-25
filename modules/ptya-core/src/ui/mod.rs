use crate::color::{ColorState, Theme};
use crate::ui::font::load_fonts;
use crate::{AssetManager, System};
use anyways::ext::AuditExt;
use egui::style::Spacing;
use egui::FontFamily::Proportional;
use egui::{Context, FontDefinitions, FontId, InnerResponse, LayerId, Rect, Rounding, Style, TextStyle, Ui, Vec2, Visuals};
use std::ops::{Deref, DerefMut};

pub const SPACING_SIZE: f32 = 25.0;
pub const ROUNDING: Rounding = Rounding {
    nw: 25.0,
    ne: 25.0,
    sw: 25.0,
    se: 25.0,
};

pub const VISUAL_SIZE: f32 = 40.0;
pub const INTERACTIVE_SIZE: f32 = 90.0;

pub mod components;
mod font;
pub mod util;

pub struct Pui<'t, 'mgr, 'egui> {
    color: ColorState<'t>,
    pub sys: &'mgr System,
    ui: &'egui mut Ui,
}

impl<'t, 'mgr, 'egui> Pui<'t, 'mgr, 'egui> {
    pub fn new(
        ui: &'egui mut Ui,
        sys: &'mgr System,
        color: ColorState<'t>,
    ) -> Pui<'t, 'mgr, 'egui> {
        Pui { color, sys, ui }
    }

    pub fn sys(&self) -> &System {
        &self.sys
    }

    pub fn color(&self) -> ColorState<'t> {
        self.color
    }

    pub fn ascend(&mut self, amount: f32) -> Pui<'t, 'mgr, '_> {
        Pui {
            color: self.color.ascend(amount),
            sys: self.sys,
            ui: self.ui,
        }
    }

    pub fn ui<'e>(&mut self, ui: &'e mut Ui) -> Pui<'t, 'mgr, 'e> {
        Pui {
            color: self.color,
            sys: self.sys,
            ui,
        }
    }

    pub fn centered_and_justified<R>(
        &mut self,
        add_contents: impl FnOnce(&mut Pui) -> R,
    ) -> InnerResponse<R> {
        self.ui.centered_and_justified(|ui| {
            let mut pui = Pui {
                color: self.color,
                sys: self.sys,
                ui,
            };
            add_contents(&mut pui)
        })
    }


	pub fn allocate_ui_at_rect<R>(
		&mut self,
		max_rect: Rect,
		add_contents: impl FnOnce(&mut Pui) -> R,
	) -> InnerResponse<R> {
		self.ui.allocate_ui_at_rect(max_rect, |ui| {
			let mut pui = Pui {
				color: self.color,
				sys: self.sys,
				ui,
			};
			add_contents(&mut pui)
		})
	}
	pub fn with_layer_id<R>(
		&mut self,
		id: LayerId,
		add_contents: impl FnOnce(&mut Pui) -> R,
	) -> InnerResponse<R> {
		self.ui.with_layer_id(id, |ui| {
			let mut pui = Pui {
				color: self.color,
				sys: self.sys,
				ui,
			};
			add_contents(&mut pui)
		})
	}

    pub fn child<'nt, 'negui>(
        &self,
        new_ui: &'negui mut Ui,
        ascend: f32,
        new_theme: Option<&'nt Theme>,
    ) -> Pui<'nt, 'mgr, 'negui>
    where
        't: 'nt,
    {
        Pui {
            color: ColorState {
                level: self.color.level + ascend,
                theme: new_theme.unwrap_or(self.color.theme),
            },
            sys: self.sys,
            ui: new_ui,
        }
    }
}

impl<'t, 'mgr, 'egui> Deref for Pui<'t, 'mgr, 'egui> {
    type Target = Ui;

    fn deref(&self) -> &Self::Target {
        self.ui
    }
}

impl<'t, 'mgr, 'egui> DerefMut for Pui<'t, 'mgr, 'egui> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ui
    }
}

pub struct UiAssets {
	fonts: FontDefinitions
}

impl UiAssets {
	pub async fn new(asset: &AssetManager) -> anyways::Result<UiAssets> {
		Ok(UiAssets {
			fonts: load_fonts(asset).await.wrap_err("Failed to load fonts")?
		})
	}
	
	pub fn apply(self, ctx: Context) {
		ctx.set_fonts(self.fonts);
		ctx.set_style(Style {
			text_styles: [
				(TextStyle::Heading, FontId::new(90.0, Proportional)),
				(
					TextStyle::Name("Heading2".into()),
					FontId::new(75.0, Proportional),
				),
				(
					TextStyle::Name("Context".into()),
					FontId::new(69.0, Proportional),
				),
				(TextStyle::Body, FontId::new(35.0, Proportional)),
				(TextStyle::Monospace, FontId::new(42.0, Proportional)),
				(TextStyle::Button, FontId::new(35.0, Proportional)),
				(TextStyle::Small, FontId::new(30.0, Proportional)),
			]
				.into(),
			spacing: Spacing {
				item_spacing: Vec2::new(25.0, 25.0),
				button_padding: Vec2::new(24.0, 12.0),
				interact_size: Vec2::new(INTERACTIVE_SIZE, INTERACTIVE_SIZE),
				..Spacing::default()
			},
			visuals: Visuals {
				clip_rect_margin: SPACING_SIZE,
				..Visuals::default()
			},
			..Style::default()
		});
	}
}