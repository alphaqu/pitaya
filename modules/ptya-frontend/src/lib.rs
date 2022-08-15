#![feature(stmt_expr_attributes)]
#![feature(drain_filter)]

use crate::content::ContentPanel;
use crate::sidebar::SidebarPanel;
use egui::{Context, Grid, LayerId, Pos2, Rect, Stroke, Vec2};
use ptya_common::color::color::{ColorState, ColorType};
use ptya_common::System;
use std::rc::Rc;

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
        let painter = system.ctx.layer_painter(LayerId::background());
        painter.rect(
            Rect::from_min_size(Pos2::new(0.0, 0.0), Vec2::INFINITY),
            0.0,
            system.color.bg(0.0, ColorType::Primary, ColorState::Idle),
            Stroke::none(),
        );
        self.sidebar
            .update(&system.ctx.clone(), system, &mut self.content);
        self.content.update(&system.ctx.clone(), system);
    }
}
