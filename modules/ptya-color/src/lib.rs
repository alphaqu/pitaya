//! # Pitaya Color
//! The core of pitayas flavorful theme system.
//! This crate contains everything involving color like blending background colors and constructing a user friendly color scheme.

mod color;
pub mod config;
mod theme;

use egui::Color32;
use log::info;
use ptya_animation::Lerp;
use std::ops::Deref;

pub use crate::color::{ColorGroup, ColorTag};
use crate::config::{ColorConfig, ThemeTag};
pub use crate::theme::Theme;

pub struct ColorManager {
	config: ColorConfig,
	theme: Theme,
}

impl ColorManager {
	pub async fn new(config: ColorConfig) -> ColorManager {
		info!("Created color manager");

		ColorManager {
			theme: Theme::new(
				match config.theme {
					ThemeTag::Pitaya => [0xe5, 0x4c, 0x64],
					ThemeTag::Custom(rgb) => rgb,
				},
				config.dark_mode,
			)
			.await,
			config,
		}
	}

	pub fn theme(&self) -> &Theme {
		&self.theme
	}

	pub fn new_state(&self) -> ColorState {
		ColorState {
			level: 0.0,
			theme: self.theme(),
		}
	}
}

#[derive(Copy, Clone)]
pub struct ColorState<'a> {
	pub level: f32,
	pub theme: &'a Theme,
}

impl<'a> ColorState<'a> {
	pub fn ascend(self, amount: f32) -> ColorState<'a> {
		ColorState {
			level: self.level + amount,
			theme: self.theme,
		}
	}

	#[inline(always)]
	pub fn bg(&self) -> Color32 {
		self.group_bg(&self.theme.primary)
	}

	#[inline(always)]
	pub fn tag_bg(&self, color: ColorTag) -> Color32 {
		self.group_bg(self.theme.get(color))
	}

	#[inline(always)]
	pub fn group_bg(&self, group: &ColorGroup) -> Color32 {
		self.color_bg(group.color)
	}

	#[inline(always)]
	pub fn color_bg(&self, accent: Color32) -> Color32 {
		self.theme.bg.lerp(&accent, self.level / 14.0)
	}
}

impl<'a> Deref for ColorState<'a> {
	type Target = Theme;

	fn deref(&self) -> &Self::Target {
		self.theme
	}
}
