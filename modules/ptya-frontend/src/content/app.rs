use egui::{Color32, Id, PaintCallback, Rect, Rounding, Sense, Stroke, Ui, Vec2};
use glium::framebuffer::{RenderBuffer, SimpleFrameBuffer};
use glium::texture::UncompressedFloatFormat;
use glium::Surface;
use log::trace;
use ptya_common::apps::app::{AppId, AppInstance};
use ptya_common::ui::animation::state::State;
use ptya_common::ui::animation::transition::Transition;
use ptya_common::System;
use std::rc::Rc;

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

    pub fn draw(&mut self, ui: &mut Ui, system: &mut System) {
        let rect = self.rect.get(ui);
        ui.allocate_ui_at_rect(rect, |ui| {
            let app = system.apps.get_mut_app(&self.app_id);
            ui.set_clip_rect(rect);
            match &mut app.app {
                AppInstance::EGui(egui) => {
                    egui.tick(ui, &system.settings);
                }
                AppInstance::OpenGL { ctx, buffer, app } => {
                    let (width, height) = buffer.get_dimensions();
                    if rect.width() as u32 != width || rect.height() as u32 != height {
                        *buffer = Rc::new(
                            RenderBuffer::new(
                                ctx,
                                UncompressedFloatFormat::U8U8U8U8,
                                rect.width() as u32,
                                rect.height() as u32,
                            )
                            .unwrap(),
                        );
                    }

                    {
                        let rc = (*buffer).as_ref();
                        let mut fb = SimpleFrameBuffer::new(ctx, rc).unwrap();
                        let color32 = system.settings.style.bg_2;
                        fb.clear_color(
                            color32.r() as f32 / 255.0,
                            color32.g() as f32 / 255.0,
                            color32.b() as f32 / 255.0,
                            color32.a() as f32 / 255.0,
                        );
                        app.tick(ui, ctx, &mut fb, rect, &system.settings);
                    }

                    let framebuffer = buffer.clone();
                    ui.painter().add(PaintCallback {
                        rect,
                        callback: framebuffer,
                    });
                }
            }
        });

        //  if let Some(app) = &self.app {
        //             let opacity = if let Some(dragging_app) = dragging_app {
        //                 1.0 - dragging_app.state.get(ui)
        //             } else {
        //                 1.0
        //             };
        //
        //             let size = system.settings.layout.window_control_size;
        //             let window_corner = (rect.right_top()) - Vec2::new(size.x, 0.0);
        //             let control = Rect::from_min_size(window_corner, size);
        //
        //             ui.painter().rect(
        //                 control,
        //                 Rounding {
        //                     nw: 0.0,
        //                     ne: system.settings.rounding,
        //                     sw: system.settings.rounding,
        //                     se: 0.0,
        //                 },
        //                 Color32::lerp(Color32::TRANSPARENT, system.settings.style.bg_2, opacity),
        //                 Stroke::none(),
        //             );
        //
        //             let response = ui.interact(
        //                 control,
        //                 id.with("window_decorator").with(&self.app),
        //                 Sense::click_and_drag(),
        //             );
        //
        //             if self.dragging {
        //                 println!("drag");
        //                 if let Some(drag) = dragging_app {
        //                     drag.tick_response(ui, &response);
        //                 }
        //             }
        //
        //             if response.clicked() {
        //                 println!("click");
        //                 *dragging_app = None;
        //                 self.dragging = false;
        //             } else if response.drag_started() && dragging_app.is_none() && opacity == 1.0 {
        //                 println!("start");
        //                 *dragging_app = Some(DraggingApp::new(ui, id, control, app.clone()));
        //                 self.dragging = true;
        //             } else if response.drag_released() {
        //                 println!("release");
        //                 self.dragging = false;
        //             }
        //         }
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
