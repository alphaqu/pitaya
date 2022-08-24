mod entry;

use crate::dropper::AppDropper;
use crate::sidebar::entry::SidebarEntry;
use egui::panel::Side;
use egui::style::Margin;
use egui::{Context, Frame, Vec2};
use ptya_core::ui::{Pui, INTERACTIVE_SIZE, SPACING_SIZE};
use ptya_core::System;

pub struct Sidebar {
    side: Side,
    entries: Vec<SidebarEntry>,
}

impl Sidebar {
    pub fn new() -> Sidebar {
        Sidebar {
            side: Side::Left,
            entries: vec![]
        }
    }
    
    pub fn update(&mut self, system: &System) {
        self.entries.clear();
        for (id, _)  in system.app.apps().iter() {
            self.entries.push(SidebarEntry {
                id: id.clone()
            });
        }
    }
    
    pub fn tick(&mut self, system: &System, dropper: &mut Option<AppDropper>) {
        let color = system.color.new_state().ascend(1.0);
        egui::SidePanel::new(self.side, "sidebar")
            .frame(Frame {
                inner_margin: Margin::same(SPACING_SIZE),
                fill: color.bg(),
                ..Frame::default()
            })
            .max_width(SPACING_SIZE + INTERACTIVE_SIZE + SPACING_SIZE)
            .resizable(false)
            .show(&system.egui_ctx, |ui| {
                let mut ui = Pui::new(ui, system, color);
                for entry in &mut self.entries {
                    let response = entry.draw(&mut ui);
                    if response.drag_started() {
                        *dropper = Some(AppDropper::new(entry.id.clone()));
                    }
                }
            });
    }
}
