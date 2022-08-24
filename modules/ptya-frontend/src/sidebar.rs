use crate::content::opening_app::DraggingApp;
use crate::content::ContentPanel;
use egui::epaint::Shadow;
use egui::panel::Side;
use egui::style::Margin;
use egui::{
    pos2, Color32, ColorImage, Context, Frame, ImageData, Mesh, Rect, TextureHandle, Ui, Vec2,
};
use log::{debug, info};
use ptya_core::System;
use ptya_core::ui::{INTERACTIVE_SIZE, Pui, SPACING_SIZE};

pub struct SidebarPanel {
    apps: Vec<SidebarAppEntry>,
}

impl SidebarPanel {
    pub fn new(system: &System) -> SidebarPanel {
        let mut apps = Vec::new();
        for (id, app) in system.app.apps().iter() {
            apps.push(SidebarAppEntry::new(id.clone()));
        }

        SidebarPanel { apps }
    }

    pub fn update(&mut self, system: &mut System, content: &mut ContentPanel) {
        egui::SidePanel::new(Side::Left, "side_bar")
            .frame(Frame {
                inner_margin: Margin::same(SPACING_SIZE),
                outer_margin: Default::default(),
                rounding: Default::default(),
                shadow: Default::default(),
                fill: system.color.new_state().ascend(1.0).bg(),
                stroke: Default::default(),
            })
            .max_width(SPACING_SIZE + INTERACTIVE_SIZE + SPACING_SIZE)
            .resizable(false)
            .show(&system.egui_ctx, |ui| {
                let mut pui = Pui::new(ui, system, system.color.new_state());


                let entry_size = Vec2::new(
                    INTERACTIVE_SIZE,
                    INTERACTIVE_SIZE,
                );

                for entry in &mut self.apps {
                    entry.update(&mut pui, content,  entry_size);
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
        SidebarAppEntry {
            app: id,
            dragged: false,
        }
    }

    pub fn update(&mut self, ui: &mut Pui, content: &mut ContentPanel, size: Vec2) {
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
        let state = ColorState::new(&response);

        // Rect
        let painter = ui.painter();
        let rounding = 25.0;
        painter.rect_filled(
            rect,
            rounding,
            system.color.bg(3.0, ColorType::Primary, state),
        );

        // Draw icon
        // TODO icon
        //{
        //    let texture_id = system.app.get_app(&self.app).icon_handle.id();
        //    let icon = Rect::from_center_size(rect.center(), Vec2::new(60.0, 60.0));
        //    let mut mesh = Mesh::with_texture(texture_id);
        //    mesh.add_rect_with_uv(
        //        icon,
        //        Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
        //        system.color.fg(ColorType::Neutral, state),
        //    );
        //    painter.add(mesh);
        //}
    }
}
