use crate::content::opening_app::DraggingApp;
use crate::content::ContentPanel;
use egui::panel::Side;
use egui::style::Margin;
use egui::{
    pos2, Color32, ColorImage, Context, Frame, ImageData, Mesh, Rect, TextureHandle, Ui, Vec2,
};
use log::{debug, info};
use ptya_common::apps::app::{EGuiApplication, AppId};
use ptya_common::System;

pub struct SidebarPanel {
    apps: Vec<SidebarAppEntry>,
}

impl SidebarPanel {
    pub fn new(ctx: &Context, system: &System) -> SidebarPanel {
        let mut apps = Vec::new();
        for (id, app) in &system.apps.apps {
            apps.push(SidebarAppEntry::new(id.clone()));
        }

        SidebarPanel { apps }
    }

    pub fn update(&mut self, ctx: &Context, system: &System, content: &mut ContentPanel) {
        egui::SidePanel::new(Side::Left, "side_bar")
            .frame(Frame {
                inner_margin: Margin::same(25.0),
                outer_margin: Default::default(),
                rounding: Default::default(),
                shadow: Default::default(),
                fill: system.settings.style.bg_0,
                stroke: Default::default(),
            })
            .max_width(100.0)
            .resizable(false)
            .show(ctx, |ui| {
                let width = ui.available_width();
                let entry_size = Vec2::new(width, width);

                for entry in &mut self.apps {
                    entry.update(ui, content, system, entry_size);
                    // Padding
                    //ui.allocate_space(Vec2::new(width, 25.0));
                }
            });
    }
}

pub struct SidebarAppEntry {
    app: AppId,
    dragged: bool,
}

impl SidebarAppEntry {
    pub fn new(id: AppId) -> SidebarAppEntry {
        SidebarAppEntry { app: id, dragged: false }
    }

    pub fn update(&mut self, ui: &mut Ui, content: &mut ContentPanel, system: &System, size: Vec2) {
        let (id, rect) = ui.allocate_space(size);
        let response = ui.interact(rect, id, egui::Sense::click_and_drag());

        if let Some(dragged) = &mut content.dragging_app {
            if self.dragged {
                dragged.tick_response(ui, &response);
            }
        } else if response.drag_started() {
            debug!(target: "drag-app", "APP: {:?} SOURCE: Sidebar Entry", self.app);
            content.dragging_app = Some(DraggingApp::new(ui, rect, self.app.clone()));
            self.dragged = true;
        } else {
            self.dragged = false;
        }

        // Rect
        let painter = ui.painter();
        painter.rect_filled(rect, 25.0, system.settings.style.bg_2);

        // Draw icon
        {
            let texture_id = system.apps.get_app(&self.app).icon_handle.id();
            let icon = Rect::from_center_size(rect.center(), Vec2::new(70.0, 70.0));
            let mut mesh = Mesh::with_texture(texture_id);
            mesh.add_rect_with_uv(
                icon,
                Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                Color32::WHITE,
            );
            painter.add(mesh);
        }
    }
}
