#![feature(hash_drain_filter)]

use crate::graphics::MapGraphics;
use crate::pos::TilePosition;
use crate::query::MapQuery;
use anyways::ext::AuditExt;
use atlas::predicate::ValueSelector;
use atlas::style::{FeatureStyleType, FeatureStyler, LayerStyle, MapStyle, PathFeatureStyle};
use atlas::tile::{TileData, TileMesh, TileMeshBuilder};
use atlas::types::Color;
use atlas::Atlas;
use egui::{PaintCallback, Sense, Ui};
use fxhash::{FxHashMap, FxHashSet};
use glium::backend::Context;
use glium::framebuffer::SimpleFrameBuffer;
use glium::Surface;
use log::{info, trace};
use ptya_common::app_icon;
use ptya_common::apps::app::{AppInfo, EGuiApplication, OpenGLApplication};
use ptya_common::settings::Settings;
use std::fs::metadata;
use std::rc::Rc;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use mathie::Vec2D;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use viewer::MapViewer;
use crate::viewport::Viewport;

mod graphics;
mod pos;
mod query;
mod unit;
mod viewer;
mod viewport;

pub struct Map {
    query: Arc<MapQuery>,
    viewer: MapViewer,
    graphics: MapGraphics,
    atlas: Arc<Atlas>,

    tokio: Runtime,
    requested: FxHashSet<TilePosition>,
    new_tiles: (
        Sender<(TilePosition, TileMeshBuilder)>,
        Receiver<(TilePosition, TileMeshBuilder)>,
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
        let scale = self.viewer.get_scale();

        self.viewer.x -= (drag_delta.x / rect.width()) / scale;
        self.viewer.y -= (drag_delta.y / rect.height()) / scale;

        if let Some(hover) = response.hover_pos() {
            self.viewer.zoom += response.ctx.input().scroll_delta.y / 1000.0;
        }

        let (width, height) = fb.get_dimensions();
        self.tick(ctx, fb, Vec2D::new(width, height), rect.aspect_ratio());

        if !self.requested.is_empty() {
            ui.ctx().request_repaint();
        }
    }
}

impl Map {
    pub fn app_info() -> AppInfo {
        AppInfo {
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
            atlas: Arc::new(Atlas::new(MapStyle {
                layers: FxHashMap::from_iter([(
                    "road".to_string(),
                    LayerStyle {
                        features: vec![FeatureStyler {
                            predicates: vec![],
                            style: FeatureStyleType::Path(PathFeatureStyle {
                                color: ValueSelector::Constant(Color([255, 0, 0, 255])),
                                width: ValueSelector::Constant(1.0),
                            }),
                        }],
                    },
                )]),
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
        ctx: &Rc<Context>,
        framebuffer: &mut SimpleFrameBuffer,
        resolution: Vec2D<u32>,
        aspect_ratio: f32,
    ) {
        let viewport = self.viewer.get_viewport(resolution, aspect_ratio);
        for pos in viewport.get_tiles() {
            let mut renderer_pos = pos;
            loop {
                if self.graphics.contains_tile(renderer_pos) {
                    self.graphics
                        .draw_tile(framebuffer, &viewport, renderer_pos)
                        .unwrap();
                    // Breaks this while and requests the needed pos
                    break;
                } else {
                    if renderer_pos == pos {
                        self.request_tile(&viewport, resolution, renderer_pos);
                    }
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
            trace!("Map tile {pos:?} is built and ready.");
        }

        self.graphics.clear(&viewport);
    }

    fn request_tile(&mut self, viewport: &Viewport, resolution: Vec2D<u32>, pos: TilePosition) {
        if !self.requested.contains(&pos) {
            self.requested.insert(pos);
            trace!("Starting request for {pos:?}");

            let query = self.query.clone();
            let atlas = self.atlas.clone();
            let sender = self.new_tiles.0.clone();
            let view = viewport.view.any_unit();
            self.tokio.spawn(async move {
                trace!("Querying map tile {pos:?}");
                // TODO error handling
                let tile = query.get(pos).await.unwrap();
                trace!("Compiling map tile {pos:?}");
                let data = atlas.compile(tile, view, resolution, pos.zoom.zoom);
                trace!("Sending map tile {pos:?}");
                sender.send((pos, data)).await.ok().unwrap();
            });
        }
    }
}
