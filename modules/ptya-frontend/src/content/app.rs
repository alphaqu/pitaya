use crate::content::opening_app::DraggingApp;
use egui::epaint::Shadow;
use egui::{Color32, Id, LayerId, Order, PaintCallback, Rect, Rgba, Rounding, Sense, Stroke, Ui, Vec2};
use glium::framebuffer::{RenderBuffer, SimpleFrameBuffer, ToColorAttachment};
use glium::texture::UncompressedFloatFormat;
use glium::Surface;
use log::{debug, info, trace};
use ptya_common::ui::animation::state::State;
use ptya_common::ui::animation::transition::Transition;
use ptya_common::System;
use std::rc::Rc;
use ptya_common::app::AppId;
use ptya_common::color::color::{ColorState, ColorType};
use ptya_common::settings::SPACING_SIZE;

#[derive(Debug, Copy, Clone, Hash)]
pub enum AppLocation {
    Primary,
    Widget(usize),
}

pub struct AppPanel {
    rect: Transition<Rect>,
    app_id: AppId,
    dragging: bool,
    for_removal: bool,
}

impl AppPanel {
    pub fn new(ui: &Ui, app_id: AppId, rect: Rect) -> AppPanel {
        AppPanel {
            rect: Transition {
                state: State::basic(app_id.egui_id().with("panel"), ui),
                from: rect,
                to: rect,
            },
            app_id,
            dragging: false,
            for_removal: false,
        }
    }

    pub fn set_rect(&mut self, rect: Rect, ui: &Ui) {
        self.rect.set(ui, rect);
    }

    pub fn draw(
        &mut self,
        ui: &mut Ui,
        system: &mut System,
        dragging_app: &mut Option<DraggingApp>,
    ) {
        let rect = self.rect.get(ui);
        self.draw_indicator(ui, rect, system, dragging_app);
        ui.allocate_ui_at_rect(rect, |ui| {
            let arc = system.app.clone();
            let mut guard = arc.write().unwrap();
            let app = guard.get_mut_app(&self.app_id);
            ui.set_clip_rect(rect);
            app.tick(ui, rect, system);
            
            //  match &mut app.app {
            //                 AppInstance::EGui(egui) => {
            //                     egui.tick(ui, &system.settings);
            //                 }
            //                 AppInstance::OpenGL { ctx, buffer, app } => {
            //                     let (width, height) = buffer.get_dimensions();
            //                     if rect.width() as u32 != width || rect.height() as u32 != height {
            //                         *buffer = Rc::new(
            //                             RenderBuffer::new(
            //                                 ctx,
            //                                 UncompressedFloatFormat::U8U8U8U8,
            //                                 rect.width() as u32,
            //                                 rect.height() as u32,
            //                             )
            //                                 .unwrap(),
            //                         );
            //                     }
            //
            //                     {
            //                         let rc = (*buffer).as_ref();
            //                         let mut fb = SimpleFrameBuffer::new(ctx, rc).unwrap();
            //                         let color32: Rgba = system.color.bg(2.0, ColorType::Primary).into();
            //                         let srgba = color32.to_rgba_unmultiplied();
            //                         fb.clear_color_srgb(
            //                             srgba[0],
            //                             srgba[1],
            //                             srgba[2],
            //                             srgba[3],
            //                         );
            //                         app.tick(ui, ctx, &mut fb, rect, &system.settings);
            //                     }
            //
            //                     let framebuffer = buffer.clone();
            //                     ui.painter().add(PaintCallback {
            //                         rect,
            //                         callback: framebuffer,
            //                     });
            //                 }
            //             }
        });
    }

    fn draw_indicator(
        &mut self,
        ui: &mut Ui,
        rect: Rect,
        system: &System,
        dragging_app: &mut Option<DraggingApp>,
    ) {
        let right_top = rect.right_top();
        let size = 50.0;
        let spacing_s = SPACING_SIZE / 2.0;
        let actual_right_top = right_top + Vec2::new(spacing_s, -spacing_s);
        let rect = Rect::from_min_size(
            actual_right_top - Vec2::new(size, 0.0),
            Vec2::new(size, size),
        );
        let rounding = Rounding {
            nw: 0.0,
            ne: size / 2.0,
            sw: size / 2.0,
            se: 0.0
        };

        let response = ui.interact(
            rect,
            ui.id().with("app_indicator").with(self.app_id()),
            Sense::click_and_drag(),
        );


        let state = ColorState::new(&response);
        ui.painter().rect(
            rect,
            rounding,
            system.color.bg(4.0, ColorType::Primary, state),
            Stroke::none(),
        );
        ui.painter().add(
            system.color.shadow()
            .tessellate(rect, rounding),
        );


        if response.clicked() {
            self.mark_removal();
            *dragging_app = None;
        } else if !self.for_removal {
            if let Some(dragged) = dragging_app {
                if self.dragging {
                    dragged.tick_response(ui, &response);
                }
            } else if response.drag_started() && !response.drag_released(){
                info!("briuh");
                debug!(target: "drag-app", "APP: {:?} SOURCE: App Thingie", self.app_id);
                *dragging_app = Some(DraggingApp::new(ui, rect, self.app_id.clone()));
                self.dragging = true;
            } else {
                self.dragging = false;
            }
        }

    }

    pub fn mark_removal(&mut self) {
        self.for_removal = true;
    }

    pub fn for_removal(&self) -> bool {
        self.for_removal
    }

    pub fn rect(&mut self, ui: &Ui) -> Rect {
        self.rect.get(ui)
    }

    pub fn app_id(&self) -> &AppId {
        &self.app_id
    }
}
