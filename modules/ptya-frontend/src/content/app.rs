use std::rc::Rc;

use glium::framebuffer::SimpleFrameBuffer;
use glium::Surface;
use glium::texture::SrgbTexture2d;
use log::{debug, info};

use egui::{Area, PaintCallback, Rect, Rounding, Sense, Ui, Vec2};
use egui::epaint::{ClippedShape, Shadow};
use ptya_core::app::AppId;
use ptya_core::ui::{Pui, ROUNDING, SPACING_SIZE};

use crate::content::opening_app::DraggingApp;

#[derive(Debug, Copy, Clone, Hash)]
pub enum AppLocation {
    Primary,
    Widget(usize),
}

pub struct AppPanel {
    rect: Rect,
    app_id: AppId,
    dragging: bool,
    for_removal: bool,
}

impl AppPanel {
    pub fn new(ui: &Ui, app_id: AppId, rect: Rect) -> AppPanel {
        AppPanel {
            rect,
            app_id,
            dragging: false,
            for_removal: false,
        }
    }

    pub fn set_rect(&mut self, rect: Rect, ui: &Ui) {
        self.rect = rect;
    }

    pub fn draw(
        &mut self,
        ui: &mut Pui,
        dragging_app: &mut Option<DraggingApp>,
    ) {
        let rect = self.rect;
        self.draw_indicator(ui, rect, dragging_app);
        ui.allocate_ui_at_rect(rect, |egui_ui| {
            let mut ui = ui.child(egui_ui, 0.0, None);

            // Get app
            let mut apps = ui.sys.app.apps();
            let app = if let Some(app) = apps.get_mut(&self.app_id) {
                app
            } else {
                return;
            };

            let ppp = ui.ctx().pixels_per_point();

            let width = (rect.width() * ppp) as u32;
            let height = (rect.height() * ppp) as u32;

            // TODO redraw on actual change
            let mut redraw = true;
            if app.buffer.width() != width || app.buffer.height() != height {
                app.buffer = Rc::new(SrgbTexture2d::empty(&ui.sys().gl_ctx, width, height).unwrap());
                redraw = true;
            }

            if redraw {
                let shape = {
                    let mut fb = SimpleFrameBuffer::new(&ui.sys().gl_ctx, &*app.buffer).unwrap();
                    fb.clear_color(0.0, 0.0, 0.0, 0.0);
                    ui.painter().rect_filled(rect, ROUNDING, ui.color().bg());

                    let id = ui.id().with("app_window");
                    let area = Area::new(id)
                        .movable(false)
                        .fixed_pos(rect.min + Vec2::splat(SPACING_SIZE))
                        .drag_bounds(rect.shrink(SPACING_SIZE));
                    let layer_id = area.layer();

                    area.show(ui.ctx(), |app_ui| {
                        app_ui.set_clip_rect(app_ui.clip_rect().expand(SPACING_SIZE));
                        let mut pui = ui.child(app_ui, 0.0, None);
                        app.app.tick(&mut pui, &mut fb);
                    });

                    let shape: Vec<ClippedShape> = ui
                        .ctx()
                        .layer_painter(layer_id)
                        .paint_list()
                        .0
                        .drain(..)
                        .collect();
                    shape
                };

                ui.painter().add(PaintCallback {
                    rect,
                    callback: Rc::new((app.buffer.clone(), shape)),
                });
            }

            ui.painter().add(PaintCallback {
                rect,
                callback: Rc::new((app.buffer.clone(), ROUNDING)),
            });


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

    fn draw_indicator(&mut self, ui: &mut Pui, rect: Rect, dragging_app: &mut Option<DraggingApp>) {
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
            se: 0.0,
        };

        let response = ui.interact(
            rect,
            ui.id().with("app_indicator").with(self.app_id()),
            Sense::click_and_drag(),
        );

        ui.painter().rect_filled(rect, rounding, ui.color().bg());
        ui.painter().add(
            Shadow {
                extrusion: 5.0,
                color: ui.color().shadow,
            }
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
            } else if response.drag_started() && !response.drag_released() {
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
        self.rect
    }

    pub fn app_id(&self) -> &AppId {
        &self.app_id
    }
}
