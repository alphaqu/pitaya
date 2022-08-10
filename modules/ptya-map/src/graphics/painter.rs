use crate::tile::Geometry;
use crate::graphics::WithColor;
use crate::{LayerDrawer, Tile, TilePosition};
use egui::epaint::Vertex;
use egui::{Color32, Mesh, Pos2, Rect, Stroke};
use euclid::default::Point2D;
use fxhash::{FxHashMap, FxHashSet};
use lyon_tessellation::math::Point;
use lyon_tessellation::path::Path;
use lyon_tessellation::{
    BuffersBuilder, FillOptions, FillTessellator, StrokeOptions, StrokeTessellator,
    TessellationError, TessellationResult, VertexBuffers,
};
use ptya_common::settings::Settings;
use std::mem::swap;

pub struct TilePainter {
    tessellator: Tessellator,
    drawer: fn(&mut Tessellator, &Settings, f32, &Tile, Rect) -> TessellationResult
}

impl TilePainter {
    pub fn new(drawer: fn(&mut Tessellator, &Settings, f32, &Tile, Rect) -> TessellationResult) -> TilePainter {
        TilePainter {
            tessellator: Tessellator::new(),
            drawer
        }
    }

    pub fn tessellate(
        &mut self,
        zoom: f32,
        tile: &Tile,
        settings: &Settings,
        rect: Rect,
    ) -> Result<Mesh, TessellationError> {
        self.tessellator.reset();
        self.tessellator.set_scale(rect.height());
        (self.drawer)(&mut self.tessellator, settings, zoom, tile, rect)?;

        let mut mesh = Mesh {
            indices: Vec::new(),
            vertices: Vec::new(),
            texture_id: Default::default(),
        };
        swap(&mut mesh.indices, &mut self.tessellator.buffer.indices);
        swap(&mut mesh.vertices, &mut self.tessellator.buffer.vertices);
        Ok(mesh)

        //for layer in &tile.layers {

            // if let Some(drawer) = layers.get(&layer.name) {
            //                 self.stroke_options.line_width = drawer.stroke_width / rect.height();
            //                 let stroke =
            //                     Stroke::new(drawer.stroke_width, settings.style.get_color(drawer.stroke));
            //                 let fill = settings.style.get_color(drawer.fill);
            //
            //                 let mut builder = BuffersBuilder::new(
            //                     &mut self.buffer,
            //                     WithColor {
            //                         stroke: stroke.color,
            //                         fill,
            //                     },
            //                 );
            //
            //                 for feature in &layer.features {
            //                     match &feature.geo {
            //                         Geometry::Point { x, y } => {
            //                             let center = Point::new(*x, *y);
            //                             if fill != Color32::TRANSPARENT {
            //                                 self.fill_tessellator.tessellate_circle(
            //                                     center,
            //                                     0.001,
            //                                     &self.fill_options,
            //                                     &mut builder,
            //                                 )?;
            //                             }
            //                             if !stroke.is_empty() {
            //                                 self.stroke_tessellator.tessellate_circle(
            //                                     center,
            //                                     0.001,
            //                                     &self.stroke_options,
            //                                     &mut builder,
            //                                 )?;
            //                             }
            //                         }
            //                         Geometry::Lines { path } => {
            //                             if !stroke.is_empty() {
            //                                 self.stroke_tessellator.tessellate_path(
            //                                     path,
            //                                     &self.stroke_options,
            //                                     &mut builder,
            //                                 )?;
            //                             }
            //                         }
            //                         Geometry::Polygon { polygons } => {
            //                             for polygon in polygons {
            //                                 if fill != Color32::TRANSPARENT {
            //                                     self.fill_tessellator.tessellate_path(
            //                                         &polygon.path,
            //                                         &self.fill_options,
            //                                         &mut builder,
            //                                     )?;
            //                                 }
            //                                 if !stroke.is_empty() {
            //                                     self.stroke_tessellator.tessellate_path(
            //                                         &polygon.path,
            //                                         &self.stroke_options,
            //                                         &mut builder,
            //                                     )?;
            //                                 }
            //                             }
            //                         }
            //                     }
            //                 }
            //             }
        //}
    }
}

pub struct Tessellator {
    pub f_tess: FillTessellator,
    pub f_opt: FillOptions,
    pub s_tess: StrokeTessellator,
    pub s_opt: StrokeOptions,
    pub buffer: VertexBuffers<Vertex, u32>,
    pub scale: f32,
}

impl Tessellator {
    pub fn new() -> Tessellator {
        Tessellator {
            f_tess: Default::default(),
            f_opt: FillOptions::DEFAULT
                .with_intersections(false)
                .with_tolerance(1.0 / 1000.0),
            s_tess: Default::default(),
            s_opt: StrokeOptions::DEFAULT
                .with_line_width(0.0005)
                .with_tolerance(1.0 / 1000.0),
            buffer: Default::default(),
            scale: 0.0,
        }
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    #[inline(always)]
    fn prepare_draw(&mut self, fill: Color32, stroke: Stroke) {
        self.s_opt.line_width = stroke.width / self.scale;
    }

    #[inline(always)]
    fn get_builder(
        fill: Color32,
        stroke: Stroke,
        buffer: &mut VertexBuffers<Vertex, u32>,
    ) -> BuffersBuilder<Vertex, u32, WithColor> {
        BuffersBuilder::new(
            buffer,
            WithColor {
                stroke: stroke.color,
                fill,
            },
        )
    }

    pub fn circle(
        &mut self,
        center: Pos2,
        radius: f32,
        fill: Color32,
        stroke: Stroke,
    ) -> TessellationResult {
        if radius == 0.0  {
            return Ok(());
        }

        let center = Point2D::new(center.x, center.y);
        let radius = radius / self.scale;

        self.prepare_draw(fill, stroke);
        let mut output = Self::get_builder(fill, stroke, &mut self.buffer);

        if fill != Color32::TRANSPARENT {
            self.f_tess
                .tessellate_circle(center, radius, &self.f_opt, &mut output)?;
        }

        if !stroke.is_empty() {
            self.s_tess
                .tessellate_circle(center, radius, &self.s_opt, &mut output)?;
        }
        Ok(())
    }

    pub fn path(&mut self, path: &Path, fill: Color32, stroke: Stroke) -> TessellationResult {
        self.prepare_draw(fill, stroke);
        let mut output = Self::get_builder(fill, stroke, &mut self.buffer);

        if fill != Color32::TRANSPARENT {
            self.f_tess
                .tessellate_path(path, &self.f_opt, &mut output)?;
        }

        if !stroke.is_empty() {
            self.s_tess
                .tessellate_path(path, &self.s_opt, &mut output)?;
        }
        Ok(())
    }

    pub fn polygon(&mut self, polygon: &Vec<Path>, fill: Color32, stroke: Stroke) -> TessellationResult {
        for path in polygon {
            self.path(path, fill, stroke)?;
        }
        Ok(())
    }

    pub fn geometry(&mut self, geometry: &Geometry, style: GeometryStyle) -> TessellationResult  {
        match geometry {
            Geometry::Point { x, y } => {
                self.circle(Pos2::new(*x, *y), style.circle_radius, style.circle_fill, style.circle_stroke)
            }
            Geometry::Line { path } => {
                self.path(path, Color32::TRANSPARENT, style.line_stroke)
            }
            Geometry::Polygon { polygons: polygon } => {
                self.polygon(polygon, style.polygon_fill, style.polygon_stroke)
            }
        }
    }

    pub fn reset(&mut self) {
        self.buffer.vertices.clear();
        self.buffer.indices.clear();
    }
}

pub struct GeometryStyle {
    pub circle_radius: f32,
    pub circle_fill: Color32,
    pub circle_stroke: Stroke,

    pub line_stroke: Stroke,

    pub polygon_fill: Color32,
    pub polygon_stroke: Stroke,
}

impl GeometryStyle {
    pub fn new() -> GeometryStyle {
        GeometryStyle {
            circle_radius: 0.0,
            circle_fill: Default::default(),
            circle_stroke: Default::default(),
            line_stroke: Default::default(),
            polygon_fill: Default::default(),
            polygon_stroke: Default::default()
        }
    }

    pub fn circle(self, radius: f32, fill: Color32, stroke: Stroke) -> Self {
        GeometryStyle {
            circle_radius: radius,
            circle_fill: fill,
            circle_stroke: stroke,
            ..self
        }
    }

    pub fn line(self, stroke: Stroke) -> Self {
        GeometryStyle {
            line_stroke: stroke,
            ..self
        }
    }

    pub fn polygon(self, fill: Color32, stroke: Stroke) -> Self {
        GeometryStyle {
            polygon_fill: fill,
            polygon_stroke: stroke,
            ..self
        }
    }
}
