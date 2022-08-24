#![feature(stmt_expr_attributes)]
#![feature(drain_filter)]

use crate::content::ContentPanel;
use crate::sidebar::SidebarPanel;
use egui::{LayerId, Pos2, Rect, Stroke, Vec2};

use ptya_core::System;
pub const WIDGET_WIDTH: f32 = 440.0;
pub const WIDGET_ADD_SIZE: f32 = 150.0;

pub mod content;
pub mod sidebar;

pub struct Frontend {
    sidebar: SidebarPanel,
    content: ContentPanel,
}

impl Frontend {
    pub fn new(system: &mut System) -> Frontend {
        Frontend {
            sidebar: SidebarPanel::new(system),
            content: ContentPanel::new(),
        }
    }

    pub fn tick(&mut self, system: &mut System) {
        let painter = system.egui_ctx.layer_painter(LayerId::background());
        painter.rect(
            Rect::from_min_size(Pos2::new(0.0, 0.0), Vec2::INFINITY),
            0.0,
            system.color.new_state().bg,
            Stroke::none(),
        );
        self.sidebar
            .update(system, &mut self.content);
        self.content.update(system);
    }
}
