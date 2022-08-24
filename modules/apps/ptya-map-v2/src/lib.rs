#![feature(hash_drain_filter)]

use crate::graphics::MapGraphics;
use crate::pos::TilePosition;
use crate::query::MapQuery;
use crate::styler::MapStyler;
use crate::unit::MapUnit;
use crate::viewport::Viewport;
use anyways::ext::AuditExt;
use atlas::mesh::MeshBuilder;
use atlas::types::Color;
use egui::{Color32, PaintCallback, Painter, Pos2, Rounding, Sense, Stroke, Ui, Vec2};
use fxhash::{FxHashMap, FxHashSet};
use glium::backend::Context;
use glium::framebuffer::SimpleFrameBuffer;
use glium::Surface;
use log::{info, trace};
use mathie::{Rect, Vec2D};
use ptya_common::app_icon;
use ptya_common::app::app::{Manifest, EGuiApplication, OpenGLApplication};
use ptya_common::settings::Settings;
use std::fs::metadata;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::thread::sleep;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use viewer::MapViewer;

mod graphics;
mod pos;
mod query;
mod styler;
mod unit;
mod viewer;
mod viewport;

pub struct Map {
    query: Arc<MapQuery>,
    viewer: MapViewer,
    graphics: MapGraphics,
    styler: Arc<RwLock<MapStyler>>,

    tokio: Runtime,
    requested: FxHashSet<TilePosition>,
    new_tiles: (
        Sender<(TilePosition, MeshBuilder)>,
        Receiver<(TilePosition, MeshBuilder)>,
    ),
}

impl OpenGLApplication for Map {
    fn tick(
        &mut self,
        ui: &mut Ui,
        ctx: &Rc<Context>,
        fb: &mut SimpleFrameBuffer,
        rect: egui::Rect,
        settings: &Settings,
    ) {
        let response = ui.interact(rect, ui.id(), Sense::click_and_drag());
        let drag_delta = response.drag_delta();
        let scale = 2f64.powf(self.viewer.zoom as f64);
        //info!("{scale}");
        self.viewer.x -= ((drag_delta.x as f64 / rect.height() as f64) / scale) * 2.0;
        self.viewer.y -= ((drag_delta.y as f64 / rect.height() as f64) / scale) * 2.0;

        if let Some(hover) = response.hover_pos() {
            self.viewer.zoom += response.ctx.input().scroll_delta.y as f64 / 250.0;
        }

        let (width, height) = fb.get_dimensions();
        self.tick(ui, ctx, fb, Vec2D::new(width, height), rect, settings);

        if !self.requested.is_empty() {
            ui.ctx().request_repaint();
        }
    }
}

impl Map {
    pub fn app_info() -> Manifest {
        Manifest {
            id: "mapv2".to_string(),
            name: "MapTwo".to_string(),
            icon: app_icon!("../icon.png"),
        }
    }

    pub fn new(ctx: &Rc<Context>) -> anyways::Result<Map> {
        Ok(Map {
            query: Arc::new(MapQuery::new()?),
            viewer: MapViewer {
                zoom: 12.5,
                x: 0.53574723,
                y: 0.30801734,
            },
            graphics: MapGraphics::new(ctx)?,
            styler: Arc::new(RwLock::new(MapStyler {
                settings: Default::default()
            })),
            tokio: Runtime::new().wrap_err("Failed to create runtime")?,
            requested: Default::default(),
            new_tiles: channel(16),
        })
    }

    pub fn get_viewport(&mut self) -> &mut MapViewer {
        &mut self.viewer
    }

    pub fn tick(
        &mut self,
        ui: &mut Ui,
        ctx: &Rc<Context>,
        framebuffer: &mut SimpleFrameBuffer,
        resolution: Vec2D<u32>,
        rect: egui::Rect,
        settings: &Settings,
    ) {
        if settings.color.theme_id != self.styler.read().unwrap().settings.theme_id {
            self.styler.write().unwrap().settings = settings.color.clone();
        }

        let painter = ui.ctx().debug_painter();

        // minimap
        let minimap = egui::Rect::from_min_size(rect.min, Vec2::splat(rect.height() / 2.0));
       // painter.rect(
       //     minimap,
       //     Rounding::none(),
       //     Color32::TRANSPARENT,
       //     Stroke::new(5.0, Color32::WHITE),
       // );

        let viewport = self.viewer.get_viewport(resolution, rect.aspect_ratio());

        //  let view_rect = Rect::new_any([0.0f32, 0.0], [1.0f32, 1.0]);
        //  let view_rect = viewport.view;

        draw_debug(&painter, minimap, viewport.view, Color32::RED);
        // draw viewport position
        //6painter.rect(egui::Rect::from_min_max(
        //6    minimap_rect.min + Vec2::new(view_rect.min().x() * minimap_rect.size().x, view_rect.min().y() * minimap_rect.size().y),
        //6    minimap_rect.min + Vec2::new(view_rect.max().x() * minimap_rect.size().x, view_rect.max().y() * minimap_rect.size().y),
        //6), Rounding::none(), Color32::RED.linear_multiply(0.1), Stroke::new(1.0, Color32::RED));

        for pos in viewport.get_tiles() {
            let mut renderer_pos = pos;
            loop {
                if self.graphics.contains_tile(renderer_pos) {
                    self.graphics
                        .draw_tile(
                            &self.styler.read().unwrap(),
                            &painter,
                            minimap,
                            rect,
                            framebuffer,
                            &viewport,
                            renderer_pos,
                        )
                        .unwrap();
                    // Breaks this while and requests the needed pos
                    break;
                } else {
                    self.request_tile(&viewport, resolution, renderer_pos);
                    if let Some(pos) = renderer_pos.get_parent() {
                        renderer_pos = pos;
                    } else {
                        break;
                    }
                }
            }
            // if self.graphics.contains_tile(pos) {
            //                 self.graphics
            //                     .draw_tile(framebuffer, viewport, zoom, height_res, pos)
            //                     .unwrap();
            //
            //                 // draws and continues
            //                 continue;
            //             } else {
            //                 let mut current_pos = pos;
            //                 while let Some(pos) = current_pos.get_outer() {
            //                     current_pos = pos;
            //
            //                     if self.graphics.contains_tile(current_pos) {
            //                         self.graphics
            //                             .draw_tile(framebuffer, viewport, zoom, height_res,current_pos)
            //                             .unwrap();
            //                         // Breaks this while and requests the needed pos
            //                         break;
            //                     }
            //                 }
            //             }
        }

        while let Ok((pos, mesh)) = self.new_tiles.1.try_recv() {
            trace!("Building map tile {pos:?}");
            self.requested.remove(&pos);
            self.graphics.add_tile(pos, mesh.build(ctx));
            ui.ctx().request_repaint();
            trace!("Map tile {pos:?} is built and ready.");
        }

        self.graphics.clear(&painter, minimap, &viewport);
    }

    fn request_tile(&mut self, viewport: &Viewport, resolution: Vec2D<u32>, pos: TilePosition) {
        if !self.requested.contains(&pos) {
            self.requested.insert(pos);
            trace!("Starting request for {pos:?}");

            let query = self.query.clone();
            let styler = self.styler.clone();
            let sender = self.new_tiles.0.clone();
            let view = viewport.view.any_unit();

            let tile_rect = pos.get_rect();
            let view_rect = viewport.view;
            let scale = (tile_rect.size() / view_rect.size()).any_unit();
            let scale = (1.0 / viewport.resolution.y() as f64) / scale.y();

            let handle = self.tokio.spawn(async move {
                trace!("Querying map tile {pos:?}");
                // TODO error handling
                let tile = query.get(pos).await.unwrap();
                trace!("Compiling map tile {pos:?}");
                let builder = MeshBuilder::compile(&*styler.read().unwrap(), tile, pos.zoom.zoom, scale as f32);
                trace!("Sending map tile {pos:?}");
                sender.send((pos, builder)).await.ok().unwrap();
            });
        }
    }
}

pub(crate) fn draw_debug(
    painter: &Painter,
    minimap: egui::Rect,
    rect: Rect<f64, MapUnit>,
    color: Color32,
) {
    return;
    // let minimap_size = minimap.size();
    //     let min = rect.min();
    //     let max = rect.max();
    //     painter.rect(
    //         egui::Rect::from_min_max(
    //             minimap.min + Vec2::new(min.x() * minimap_size.x, min.y() * minimap_size.y),
    //             minimap.min + Vec2::new(max.x() * minimap_size.x, max.y() * minimap_size.y),
    //         ),
    //         Rounding::none(),
    //         color.linear_multiply(0.1),
    //         Stroke::new(1.0, color),
    //     );
}
pub(crate) fn test_draw_tile(
    painter: &Painter,
    screen: egui::Rect,
    pos: Vec2D<f64>,
    scale: Vec2D<f64>,
) {
    return;

   // let tile = egui::Rect::from_min_size(Pos2::new(0.0, 0.0), Vec2::new(1.0, 1.0));
   // let add = tile.translate(Vec2::new(pos.x(), pos.y()));
   // let scale = Vec2::new(scale.x(), scale.y());

   // // 0 - 1
   // let scaled = egui::Rect::from_min_max(
   //     (add.min.to_vec2() * scale).to_pos2(),
   //     (add.max.to_vec2() * scale).to_pos2(),
   // );

   // // screen correction
   // let screen = egui::Rect::from_min_max(
   //     screen.min + (scaled.min.to_vec2() * screen.size()),
   //     screen.min + (scaled.max.to_vec2() * screen.size()),
   // );

   // let color = Color32::from_rgb(255, 0, 255);
   // painter.rect(
   //     screen,
   //     Rounding::none(),
   //     color.linear_multiply(0.2),
   //     Stroke::new(1.0, color),
   // );
}
