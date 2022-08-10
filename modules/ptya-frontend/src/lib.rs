#![feature(stmt_expr_attributes)]
#![feature(drain_filter)]

use egui::{Context, Grid};
use ptya_common::System;
use crate::content::ContentPanel;
use crate::sidebar::SidebarPanel;

pub mod sidebar;
pub mod content;


pub struct Frontend {
	sidebar: SidebarPanel,
	content: ContentPanel,
}

impl Frontend {
	pub fn tick(&mut self, ctx: &Context, system: &mut System) {
		self.sidebar.update(ctx, system, &mut self.content);
		self.content.update(ctx, system);
	}
}