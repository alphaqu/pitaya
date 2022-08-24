use egui::epaint::ClippedShape;
use egui::{Area, Color32, Id, PaintCallback, Rect, Vec2};
use glium::framebuffer::SimpleFrameBuffer;
use glium::texture::{MipmapsOption, SrgbFormat, SrgbTexture2d};
use glium::Surface;
use ptya_core::animation::{Animation, AnimationImpl, Easing};
use ptya_core::app::{App, AppId};
use ptya_core::ui::components::ProgressSpinner;
use ptya_core::ui::{Pui, ROUNDING, SPACING_SIZE};
use ptya_core::System;
use std::rc::Rc;

pub struct AppPanel {
    id: AppId,
    rect: Rect,
    animation: Option<Id>,

    // App Drawing
    buffer: Rc<SrgbTexture2d>,
}

impl AppPanel {
    pub fn new(sys: &System, id: AppId, from: Rect) -> AppPanel {
        AppPanel {
            id,
            rect: from,
            animation: None,
            buffer: Rc::new(
                SrgbTexture2d::empty_with_format(
                    &sys.gl_ctx,
                    SrgbFormat::U8U8U8U8,
                    MipmapsOption::NoMipmap,
                    0,
                    0,
                )
                .unwrap(),
            ),
        }
    }

    fn get_ani(ui: &mut Pui, rect: Rect, id: Id) -> Animation<Rect> {
        ui.sys
            .animation
            .get_or::<Rect>(id, || AnimationImpl::simple(rect))
    }

    pub fn set_rect(&mut self, ui: &mut Pui, rect: Rect) {
        if let Some(id) = self.animation {
            if Self::get_ani(ui, rect, id).is_finished() {
                self.animation = None;
            }
        }

        if self.rect != rect && self.animation.is_none() {
            let id = self.id.egui_id().with("rect_animation");
            Self::get_ani(ui, rect, id)
                .set_from(self.rect)
                .set_to(rect)
                .set_easing(Easing::EaseInOut)
                .begin();

            self.animation = Some(id);
            self.rect = rect;
        }
    }

    pub fn get_rect(&mut self, ui: &mut Pui) -> Rect {
        if let Some(id) = self.animation {
            Self::get_ani(ui, self.rect, id).get_value()
        } else {
            self.rect
        }
    }

    pub fn draw(&mut self, ui: &mut Pui) {
        let mut ui = ui.ascend(1.0);
        let rect = self.get_rect(&mut ui);
        if let Some(container) = ui.sys.app.apps().get_mut(&self.id) {
            if let Some(app) = container.app() {
                self.draw_app(&mut ui, app);
            } else {
                ui.centered_and_justified(|ui| ProgressSpinner::new(None).ui(ui));
            }
        }
    }

    fn draw_app(&mut self, ui: &mut Pui, app: &mut Box<dyn App>) {
        let rect = self.get_rect(ui);

        let ppp = ui.ctx().pixels_per_point();

        let width = (rect.width() * ppp) as u32;
        let height = (rect.height() * ppp) as u32;

        // TODO redraw on actual change
        let mut redraw = true;
        if self.buffer.width() != width || self.buffer.height() != height {
            self.buffer = Rc::new(SrgbTexture2d::empty(&ui.sys().gl_ctx, width, height).unwrap());
            redraw = true;
        }

        if redraw {
            let shape = {
                let mut fb = SimpleFrameBuffer::new(&ui.sys().gl_ctx, &*self.buffer).unwrap();
                fb.clear_color(0.0, 0.0, 0.0, 1.0);
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
                    app.tick(&mut pui, &mut fb);
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
                callback: Rc::new((self.buffer.clone(), shape)),
            });
        }

        ui.painter().add(PaintCallback {
            rect,
            callback: Rc::new((self.buffer.clone(), ROUNDING)),
        });
    }
    pub fn id(&self) -> &AppId {
        &self.id
    }
}
