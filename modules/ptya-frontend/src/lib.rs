#![feature(hash_drain_filter)]
#![feature(drain_filter)]

use crate::content::Content;
use crate::dropper::AppDropper;
use crate::sidebar::Sidebar;
use anyways::ext::AuditExt;
use anyways::Result;
use glium::backend::Context;
use log::info;
use ptya_core::System;
use std::rc::Rc;

mod content;
mod dropper;
mod sidebar;

const DEBUG_MODE: bool = false;

pub struct Frontend {
	pub system: System,
	sidebar: Sidebar,
	content: Content,
	dropper: Option<AppDropper>,
}

impl Frontend {
	pub fn new(ctx: egui::Context, gl_ctx: Rc<Context>) -> Result<Frontend> {
		Ok(Frontend {
			system: System::new(ctx, gl_ctx).wrap_err("Failed to init early system")?,
			sidebar: Sidebar::new(),
			content: Content::new(),
			dropper: None,
		})
	}

	pub fn tick(&mut self) -> Result<()> {
		if self.system.is_loaded() {
		//	self.system.egui_ctx.set_debug_on_hover(true);
			if let Some(dropper) = &mut self.dropper {
				dropper.tick(&self.system);
				self.system.egui_ctx.request_repaint();
			}

			self.sidebar.tick(&self.system, &mut self.dropper);
			self.content.tick(&self.system, &mut self.dropper);

			let mut finished = false;
			if let Some(dropper) = &mut self.dropper {
				finished = dropper.finish(&self.system);
			}

			if finished {
				info!("App dropper sequence finished");
				self.dropper = None;
			}
		}

		// Updated
		if self.system.tick()? {
			self.system.app.load_app(
				&self.system.gl_ctx,
				ptya_playground::manifest(),
				ptya_playground::load(),
			);
			//self.system.app.load_app(
			//	&self.system.gl_ctx,
			//	ptya_map::manifest(),
			//	ptya_map::load(&self.system).wrap_err("Failed to initialize map application")?,
			//);
			self.system.app.update(&self.system);
			self.sidebar.update(&self.system);
		}

		Ok(())
	}
}
