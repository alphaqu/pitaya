pub mod painter;
pub mod tessellator;
mod render;
mod state;

use anyways::ext::AuditExt;
use egui::epaint::{CircleShape, Tessellator, Vertex, WHITE_UV};
use egui::{Align2, Color32, Mesh, PaintCallback, Painter, Pos2, Rect, Shape, Stroke, Vec2};
use lyon_tessellation::math::{Point, Transform};
use lyon_tessellation::{
    BuffersBuilder, FillOptions, FillRule, FillTessellator, FillVertex, FillVertexConstructor,
    Orientation, StrokeOptions, StrokeTessellator, StrokeVertex, StrokeVertexConstructor,
    VertexBuffers,
};
use std::collections::HashMap;
use std::mem::swap;

use crate::tile::{Geometry, Layer, Value};
use crate::graphics::painter::{GeometryStyle, TilePainter};
use crate::{Tile, TilePosition};
use anyways::Result;
use euclid::default::{Box2D, Vector2D};
use euclid::Point2D;
use fxhash::{FxHashMap, FxHashSet};
use glium::Surface;
use lyon_tessellation::path::{Path, Polygon};
use ptya_common::settings::style::StyleColor;
use ptya_common::settings::Settings;
use rayon::{ThreadPool, ThreadPoolBuilder};

pub struct MapDrawer {
    layers: FxHashMap<String, LayerDrawer>,

    tessellated_tiles_lookup: FxHashSet<TilePosition>,
    tessellated_tiles: Vec<(TilePosition, Mesh, bool)>,
    drawn_tiles: FxHashSet<TilePosition>,
    tile_tessellator: TilePainter,
    dirty: bool,
}

impl MapDrawer {
    pub fn new(layers: Vec<(String, LayerDrawer)>) -> MapDrawer {
        let painter = TilePainter::new(|tessellator, settings, zoom, tile, rect| {
            
            let mut roads = 0;
            for layer in &tile.layers {
                match layer.name.as_str() {
                    "water" => {
                        for feature in &layer.features {
                            tessellator.geometry(
                                &feature.geo,
                                GeometryStyle::new().polygon(settings.style.bg_0, Stroke::none()),
                            )?;
                        }
                    }
                    "admin" => {
                        tessellator.set_scale(rect.height() * 2.0);
                        for feature in &layer.features {
                            tessellator.geometry(
                                &feature.geo,
                                GeometryStyle::new().line(Stroke::new(2.0, settings.style.fg_0)),
                            )?;
                        }
                    }
                    "road" => {
                        tessellator.set_scale(rect.height() * 2.0);
                        roads += 1;
                        
                        for feature in &layer.features {
                            let color = settings.style.bg_4;
                            
                            
                            let mut width = 1.0;
                            if let Some(Value::String(string)) = feature.fields.get("class") {
                                match string.as_str() {
                                    "motorway" => {
                                        width = 2.5;
                                    }
                                    "motorway_link" => {
                                        width = 2.25;
                                    }
                                    "primary" => {
                                        width = 2.0;
                                    }
                                    "primary_link" => {
                                        width = 1.75;
                                    }
                                    "secondary" => {
                                        width = 1.75;
                                    }
                                    "secondary_link" => {
                                        width = 1.50;
                                    }
                                    _ => {}
                                }
                            }
                            let stroke = Stroke::new(width, color);
                            tessellator.geometry(
                                &feature.geo,
                                GeometryStyle::new()
                                    .polygon(settings.style.bg_3, stroke)
                                    .line(stroke),
                            )?;
                        }
                    }
                    _ => {}
                }
            }

            if roads > 1 {
                println!("{roads}");
            }

            Ok(())
        });
        MapDrawer {
            layers: layers.into_iter().collect(),
            tessellated_tiles_lookup: Default::default(),
            tessellated_tiles: Default::default(),
            drawn_tiles: Default::default(),
            tile_tessellator: painter,
            dirty: true,
        }
    }

    pub fn push(
        &mut self,
        zoom: f32,
        position: &TilePosition,
        tile: &Tile,
        rect: Rect,
        settings: &Settings,
    ) -> Result<()> {
        if !self.tessellated_tiles_lookup.contains(position) {
            let mesh = self.tile_tessellator.tessellate(zoom, tile, settings, rect)?;
            self.tessellated_tiles.push((*position, mesh, false));
            self.tessellated_tiles_lookup.insert(*position);
        }
        self.drawn_tiles.insert(*position);
        Ok(())
    }

    pub fn draw(&mut self, painter: &Painter, viewport: Box2D<f32>, area: Rect) {
        // painter.add(CallbackFn::new(
        //             area,
        //             |info, painter| {
        //                 let mut frame_buffer = painter.get_framebuffer();
        //                 frame_buffer.clear_color(0.0, 1.0, 0.0, 1.0);
        //             }
        //         ));
        for (pos, mesh, remove) in &mut self.tessellated_tiles {
            let rect = pos.get_viewport_magic(viewport, area);

            if !self.drawn_tiles.contains(pos) {
                *remove = true;
                continue;
            }

            if area.intersects(rect) {
                let mut mesh = mesh.clone();
                for vertex in &mut mesh.vertices {
                    vertex.pos.x = (vertex.pos.x * rect.width()) + rect.min.x;
                    vertex.pos.y = (vertex.pos.y * rect.height()) + rect.min.y;
                }
                painter.add(mesh);
                painter.rect(rect, 0.0, Color32::TRANSPARENT, Stroke::new(1.0, Color32::WHITE));
            }
        }

        for (pos, rect, _) in self
            .tessellated_tiles
            .drain_filter(|(_, _, remove)| *remove)
        {
            self.tessellated_tiles_lookup.remove(&pos);
        }
    }

    pub fn clear(&mut self) {
        self.drawn_tiles.clear();
    }
}

pub struct LayerDrawer {
    pub stroke_width: f32,
    pub stroke: StyleColor,
    pub fill: StyleColor,
}

pub struct WithColor {
    pub stroke: Color32,
    pub fill: Color32,
}

impl<'a> FillVertexConstructor<Vertex> for WithColor {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        Vertex {
            pos: Pos2::new(vertex.position().x, vertex.position().y),
            uv: WHITE_UV,
            color: self.fill,
        }
    }
}

impl<'a> StrokeVertexConstructor<Vertex> for WithColor {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> Vertex {
        Vertex {
            pos: Pos2::new(vertex.position().x, vertex.position().y),
            uv: WHITE_UV,
            color: self.stroke,
        }
    }
}

// pub struct MapDrawer {
// 	pub mesh: Mesh,
// 	pub rect: Rect,
// 	pub stroke: Stroke,
// 	pub fill: Color32,
// }
//
// impl MapDrawer {
// 	pub fn push(&mut self, tessellator: &mut Tessellator, geometry: &Geometry) {
// 		let mut builder: VertexBuffers<Vertex, u32> = VertexBuffers::new();
//
//
// 		match geometry {
// 			Geometry::Point { x, y } => {
//
// 				tessellator.tessellate_circle(Point::new(*x, *y), 1.0, &FillOptions::even_odd(), )
// 				tessellator.tessellate_circle(CircleShape::filled(Pos2::new(*x, *y), 1.0, self.fill), &mut self.mesh);
// 			}
// 			Geometry::Lines { lines } => {
// 				let mut x = self.rect.min.x;
// 				let mut y = self.rect.min.y;
// 				let mut path = PathShape::line(Vec::new(), self.stroke);
// 				for line in lines {
// 					x += line.x  * self.rect.width();
// 					y += line.y  * self.rect.height();
// 					path.points.push(Pos2::new(x, y));
// 					for (dx, dy) in &line.values {
// 						x += dx * self.rect.width() ;
// 						y += dy * self.rect.height() ;
// 						path.points.push(Pos2::new(x, y))
// 					}
//
// 					tessellator.tessellate_path(&path, &mut self.mesh);
// 					path.points.clear();
// 				}
// 			}
// 			Geometry::Polygon { polygons} => {
// 				let mut x = self.rect.min.x;
// 				let mut y = self.rect.min.y;
// 				let mut path = PathShape::convex_polygon(Vec::new(),  self.fill, self.stroke);
// 				for line in polygons {
// 					match line {
// 						GeometryPolygon::Basic(line) => {
// 							x += line.x  * self.rect.width();
// 							y += line.y  * self.rect.height();
// 							path.points.push(Pos2::new(x, y));
// 							for (dx, dy) in &line.values {
// 								x += dx * self.rect.width() ;
// 								y += dy * self.rect.height() ;
// 								path.points.push(Pos2::new(x, y))
// 							}
//
// 							let exterior = Self::shoelace(&path.points) > 0.0;
// 							if exterior {
// 								tessellator.tessellate_path(&path, &mut self.mesh);
// 								path.points.clear();
// 							}
// 						}
// 						GeometryPolygon::Advanced { .. } => {}
// 					}
// 				}
// 			}
// 		}
// 	}
//
// 	pub fn draw(self, painter: &Painter) {
//
// 		painter.add(Shape::Mesh(self.mesh));
// 	}
//
// 	fn shoelace(v: &Vec<Pos2>) -> f32 {
// 		let n = v.len();
// 		let mut a = 0.0;
// 		for i in 0..(n - 1) {
// 			let next = v[i + 1];
// 			let current = v[i];
// 			a += current.x * next.y - next.x * current.y;
// 		}
//
// 		let last = v[n - 1];
// 		let first = v[0];
// 		(a + last.x * first.y - first.x * last.y).abs() / 2.0
// 	}
// }
