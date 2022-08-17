use crate::{ColorState, ColorType, System};
use egui::{Area, Id, LayerId, Layout, Order, Pos2, Sense, Ui, Vec2};
use epaint::{ClippedShape, PaintCallback, Rect, Rounding, Stroke};
use fxhash::FxHashMap;
use glium::backend::Context;
use glium::framebuffer::{RenderBuffer, SimpleFrameBuffer};
use glium::texture::{SrgbTexture2d, UncompressedFloatFormat};
use glium::Surface;
use log::info;
use std::rc::Rc;
use std::sync::RwLock;
use egui::style::Margin;
use crate::settings::{ROUNDING_SIZE, SPACING_SIZE};
use crate::ui::animation::state::State;

pub struct AppComp {
    pub apps: FxHashMap<AppId, AppContainer>,
}

impl AppComp {
    pub fn init() -> AppComp {
        AppComp {
            apps: Default::default(),
        }
    }

    pub fn load_app(&mut self, app: AppContainer) {
        info!("Loading application: {}", &app.manifest.id);

        self.apps.insert(
            AppId {
                id: app.manifest.id.clone(),
            },
            app,
        );
    }

    pub fn get_mut_app(&mut self, id: &AppId) -> &mut AppContainer {
        self.apps.get_mut(id).unwrap()
    }

    pub fn get_app(&self, id: &AppId) -> &AppContainer {
        self.apps.get(id).unwrap()
    }
}

pub struct AppContainer {
    pub app: Box<dyn AppImpl>,
    pub buffer: Rc<SrgbTexture2d>,
    pub manifest: Manifest,
}

impl AppContainer {
    pub fn new(ctx: &Rc<Context>, manifest: Manifest, app: Box<dyn AppImpl>) -> AppContainer {
        AppContainer {
            app,
            buffer: Rc::new(
                SrgbTexture2d::empty(ctx, 0, 0).unwrap(),
            ),
            manifest,
        }
    }

    pub fn tick(&mut self, ui: &mut Ui, rect: egui::Rect, system: &mut System) {
        let ppp = ui.ctx().pixels_per_point();

        let width = (rect.width() * ppp) as u32;
        let height = (rect.height() * ppp) as u32;

        let mut redraw = true;
        if self.buffer.width() != width || self.buffer.height() != height {
            self.buffer = Rc::new(
                SrgbTexture2d::empty(&system.gl_ctx, width, height).unwrap(),
            );
            redraw = true;
        }



        if redraw {
            let shape = {
                let mut fb = SimpleFrameBuffer::new(&system.gl_ctx, &*self.buffer).unwrap();
                fb.clear_color(0.0, 0.0, 0.0, 0.0);
                ui.painter().rect(rect, ROUNDING_SIZE, system.color.bg(1.0, ColorType::Primary, ColorState::Idle), Stroke::none());

                let id = ui.id().with("app_window");
                let area = Area::new(id).movable(false).fixed_pos(rect.min + Vec2::splat(SPACING_SIZE)).drag_bounds(rect.shrink(SPACING_SIZE));
                let layer_id = area.layer();
                //let state = ui.ctx().memory().areas.get(id).cloned();
                //if state.is_none() {
                //    ui.ctx().request_repaint();
                //}
                //let state = state.unwrap_or_else(|| egui::area::State {
                //    pos: rect.min,
                //    size: rect.size(),
                //    interactable: true,
                //});
                area.show(ui.ctx(), |ui| {
                    ui.set_clip_rect(ui.clip_rect().expand(SPACING_SIZE));
                    self.app.tick(ui, &mut fb, system);
               });

              // let mut app_ui = Ui::new(ui.ctx().clone(), LayerId::new(Order::Middle, id), id, rect, rect);
              // app_ui.set_enabled(ui.is_enabled());
              // app_ui.set_visible(true);
              // self.app.tick(&mut app_ui, &mut fb, system);
              // ui.ctx().memory().areas.set_state(layer_id, state);

                let shape: Vec<ClippedShape> = ui.ctx().layer_painter(layer_id).paint_list().0.drain(..).collect();
                shape
            };

            ui.painter().add(PaintCallback {
                rect: rect,
                callback: Rc::new((self.buffer.clone(), shape)),
            });
        }

        ui.painter().add(PaintCallback {
            rect,
            callback: Rc::new((self.buffer.clone(), Rounding::same(ROUNDING_SIZE))),
        });
    }
}

pub trait AppImpl: Send {
    /// Runs after initialization and every time a setting (like a theme) changes.
    fn update(&mut self, system: &mut System);

    /// Runs every frame.
    fn tick(&mut self, ui: &mut Ui, fb: &mut SimpleFrameBuffer, system: &mut System);
}

pub struct Manifest {
    pub id: String,
    //pub icon: Icon,
}

//pub enum Icon {
//	MaterialSymbols(String),
//	IconAsset(String),
//}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct AppId {
    pub id: String,
}

impl AppId {
    pub fn egui_id(&self) -> egui::Id {
        Id::new("pitaya@app_id").with(&self.id)
    }
}
