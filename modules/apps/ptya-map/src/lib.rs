#![feature(drain_filter)]

use std::f32::consts::PI;
use std::ops::{Div, Mul};
use std::sync::Arc;
use std::time::Instant;

use crate::graphics::{LayerDrawer, MapDrawer};
use anyways::ext::AuditExt;
use anyways::Result;
use egui::epaint::{PathShape, Tessellator};
use egui::{
    Align2, Color32, FontFamily, FontId, Id, LayerId, Mesh, Order, Painter, Pos2, Rounding, Sense,
    Shape, Stroke, Ui, Vec2,
};
use euclid::default::{Box2D, Point2D, Rect, Vector2D};
use euclid::{Angle, Size2D, UnknownUnit};
use image::imageops::FilterType;
use image::ImageFormat;
use lyon_tessellation::{FillOptions, StrokeOptions};
use rayon::{ThreadPool, ThreadPoolBuilder};
use ptya_common::app_icon;
use ptya_common::apps::app::{EGuiApplication, AppInfo};
use ptya_common::settings::style::{StyleColor, StyleSettings};
use ptya_common::settings::Settings;

use crate::tile::{Command, Tile};
use crate::pos::{get_tile_pos, lat_to_y, lon_to_x, x_to_lon, y_to_lat, TilePosition};
use crate::storage::MapStorage;
use crate::viewer::MapViewer;

mod tile;
mod graphics;
pub mod pos;
mod storage;
mod viewer;
mod style;

pub(crate) const TOKEN: &str = env!("MAPBOX_TOKEN");

pub struct Map {
    storage: MapStorage,
    drawer: MapDrawer,
    old_area: egui::Rect,
    frame_time: f32,
    
    thread_pool: Arc<ThreadPool>,
    pub viewer: MapViewer,
    pub dirty: bool,
}

impl EGuiApplication for Map {
    fn tick(&mut self, ui: &mut Ui, settings: &Settings) {

        let rect = ui.max_rect();
        let response = ui.interact(rect, ui.id(), Sense::click_and_drag());
        let drag_delta = response.drag_delta();
        let scale = self.viewer.get_scale();

        if drag_delta.x != 0.0 || drag_delta.y != 0.0 || response.ctx.input().scroll_delta.y != 0.0
        {
            self.dirty = true;
        }

        self.viewer.x -= (drag_delta.x / rect.height()) / scale;
        self.viewer.y -= (drag_delta.y / rect.height()) / scale;

        if let Some(hover) = response.hover_pos() {
            self.viewer.zoom += response.ctx.input().scroll_delta.y / 100.0;
        }

        ui.painter().rect(rect, 0.0, settings.style.bg_2, Stroke::none());
        self.draw(ui.painter(), rect, settings).unwrap();
    }
    
}

impl Map {
    pub fn new() -> Result<Map> {
        let thread_pool = Arc::new(ThreadPoolBuilder::new().build()?);
        Ok(Map {
            storage: MapStorage::new("./cache", &thread_pool).wrap_err("Failed to create Storage")?,
            thread_pool,
            drawer: MapDrawer::new(vec![
                (
                    "water".to_string(),
                    LayerDrawer {
                        stroke_width: 0.0,
                        stroke: StyleColor::Custom(Color32::TRANSPARENT),
                        fill: StyleColor::Bg0,
                    },
                ),
                (
                    "admin".to_string(),
                    LayerDrawer {
                        stroke_width: 2.0,
                        stroke: StyleColor::Bg4,
                        fill: StyleColor::Custom(Color32::TRANSPARENT),
                    },
                ),
                (
                    "road".to_string(),
                    LayerDrawer {
                        stroke_width: 2.0,
                        stroke: StyleColor::Fg0,
                        fill: StyleColor::Custom(Color32::TRANSPARENT),
                    },
                ),
            ]),
            old_area: egui::Rect::NOTHING,
            frame_time: 0.0,
            viewer: MapViewer {
                zoom: 12.5,
                x: 0.53574723,
                y: 0.30801734,
            },
            dirty: false,
        })
    }

    pub fn app_info() -> AppInfo {
        AppInfo {
            id: "map".to_string(),
            name: "Map".to_string(),
            icon: app_icon!("../icon.png"),
        }
    }

    pub fn draw(&mut self, painter: &Painter, area: egui::Rect, settings: &Settings) -> Result<()> {
        if self.old_area != area {
            self.old_area = area;
            self.dirty = true;
        }

        let viewport = self.viewer.get_viewport(area.aspect_ratio());

        if self.dirty {
            painter.ctx().request_repaint();

            let mut rect1 = area;
            rect1.set_height(10.0);
            self.dirty = false;
            self.drawer.clear();
            let start = Instant::now();
            let tile_viewport = self.viewer.get_tile_viewport(viewport);
            let tiles = self.viewer.get_tiles(&mut self.storage, &tile_viewport)?;
            for request in &tiles {
                let rect = request.render_tile_pos.get_viewport_magic(viewport, area);
                let cull_rect = request.cull_tile_pos.get_viewport_magic(viewport, area);
                self.drawer
                    .push(self.viewer.zoom, &request.render_tile_pos, request.render_tile, rect, settings)
                    .wrap_err("Failed to render layer")?;
            }
            if self.storage.end_frame(viewport) {
                self.dirty = true;
            }
            self.frame_time = start.elapsed().as_secs_f32();
        }

        let start = Instant::now();
        self.drawer.draw(painter, viewport, area);

        painter.text(
            area.min,
            Align2::LEFT_TOP,
            format!("x: {}, y: {}, zoom: {}, ms: {:.5}+{:.5}", self.viewer.x, self.viewer.y,  self.viewer.zoom, self.frame_time * 1000.0, start.elapsed().as_secs_f32() * 1000.0),
            FontId::new(40.0, FontFamily::Proportional),
            Color32::WHITE,
        );
        return Ok(());
        //for (rect, pos) in tile_drawn {
        //    painter.text(rect.center(), Align2::CENTER_CENTER, format!("{}x{} ({})", pos.x, pos.y, pos.zoom), FontId::new(20.0, FontFamily::Proportional), Color32::WHITE);
        //    painter.rect(
        //        rect,
        //        0.0,
        //        Color32::TRANSPARENT,
        //        Stroke::new(1.0, Color32::WHITE),
        //    );
        //}
        // let tile_viewport = self.viewer.get_tile_viewport(viewport);
        //         let tiles = self.viewer.get_tiles(&mut self.storage, &tile_viewport)?;
        //
        //         for (pos, tile) in &tiles {
        //             let rect = pos.get_map_position();
        //             let rect = rect.translate(-viewport.min.to_vector());
        //             let rect = rect.scale(1.0 / viewport.width(), 1.0 / viewport.height());
        //             let rect = rect.scale(area.width(), area.height());
        //             let rect = egui::Rect::from_min_size(
        //                 Pos2::new(rect.origin.x, rect.origin.y),
        //                 Vec2::new(rect.size.width, rect.size.height),
        //             );
        //
        //             self.drawer
        //                 .push(tile, &rect, settings)
        //                 .wrap_err("Failed to render layer")?;
        //         }
        //
        //         self.drawer.draw(painter);
        //         for (pos, tile) in tiles {
        //             let rect = pos.get_map_position();
        //             let rect = rect.translate(-viewport.min.to_vector());
        //             let rect = rect.scale(1.0 / viewport.width(), 1.0 / viewport.height());
        //             let rect = rect.scale(area.width(), area.height());
        //             let rect = egui::Rect::from_min_size(
        //                 Pos2::new(rect.origin.x, rect.origin.y),
        //                 Vec2::new(rect.size.width, rect.size.height),
        //             );
        //
        //             painter.rect(
        //                 rect,
        //                 Rounding::none(),
        //                 Color32::TRANSPARENT,
        //                 Stroke::new(1.0, Color32::WHITE),
        //             );
        //             painter.text(
        //                 rect.center(),
        //                 Align2::CENTER_CENTER,
        //                 format!("{}x{} ({})", pos.x, pos.y, pos.zoom),
        //                 FontId::proportional(40.0),
        //                 Color32::WHITE,
        //             );
        //         }
        //
        //         //painter.rect(
        //         //    area,
        //         //    Rounding::none(),
        //         //    Color32::TRANSPARENT,
        //         //    Stroke::new(10.0, Color32::RED),
        //         //);
        //
        //         self.storage.end_frame(viewport);
        //         Ok(())
    }
}
