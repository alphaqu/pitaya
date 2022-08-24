#![feature(hash_drain_filter)]

use std::rc::Rc;
use anyways::ext::AuditExt;
use ptya_core::System;
use crate::content::Content;
use crate::dropper::AppDropper;
use crate::sidebar::Sidebar;
use anyways::Result;
use glium::backend::Context;
use log::info;

mod sidebar;
mod content;
mod dropper;

const DEBUG_MODE: bool = false;

pub struct Frontend {
    system: System,
    sidebar: Sidebar,
    content: Content,
    dropper: Option<AppDropper>
}

impl Frontend {
    pub fn new(ctx: egui::Context, gl_ctx: Rc<Context>) -> Result<Frontend> {
       Ok( Frontend {
           system: System::new(ctx, gl_ctx).wrap_err("Failed to init early system")?,
           sidebar: Sidebar::new(),
           content: Content::new(),
           dropper: None
       })
    }

    pub fn tick(&mut self) -> Result<()> {
        if self.system.is_loaded() {
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
                self.dropper = None;
            }
        }

        // Updated
        if self.system.tick()? {
            self.system.app.load_app(&self.system, ptya_playground::manifest(), ptya_playground::load());
            self.sidebar.update(&self.system);
        }

        Ok(())
    }
}