use std::iter::Map;
use mathie::Vec2D;
use crate::geometry::Geometry;
use crate::style::{PathFeatureStyle, PathStyle};
use crate::tessellation::{compile_indices, get_len_path, tessellate_path};
use crate::types::{compile_delaunator, Color, MapVertex};

pub struct PathGeometry {
    pub paths: Vec<Vec<Vec2D<f32>>>,
}

impl Geometry for PathGeometry {
    type Style = PathStyle;
    type FeatureStyle = PathFeatureStyle;
    //     fn get_vertices_len(&self) -> usize {
    //         let mut out = 0;
    //         for path in &self.paths {
    //             out += path.len() * 2;
    //         }
    //         out
    //     }

    fn compile(&self, style: &Self::Style) -> (Vec<MapVertex>, Vec<u32>) {
        let mut vertices_out = Vec::new();
        let mut indices = Vec::new();
        let mut pos = 0;
        let color = style.color.to_raw();
        for points in &self.paths {
            let len = get_len_path(points);
            let mut output = Vec::with_capacity(len);
            for _ in 0..len {
                output.push(MapVertex  {
                    a_pos: [0.0, 0.0],
                    a_color: [0.0, 0.0, 0.0, 0.0]
                })
            }
            tessellate_path(points, style.width , color, &mut output);
            for idx in compile_indices(&output, &[]) {
                indices.push(idx + (pos as u32))
            }
            vertices_out.append(&mut output);
            pos += len;
        }

        (vertices_out, indices)
    }

    fn update(
        &self,
        vertices: &mut [MapVertex],
        old_style: Option<&Self::Style>,
        new_style: &Self::Style,
    ) -> bool {
        let mut width_changed = false;
        let mut color_changed = false;
        if let Some(style) = old_style {
            if style.width != new_style.width {
                width_changed = true;
            }

            if style.color != new_style.color {
                color_changed = true;
            }
        } else {
            width_changed = true;
            color_changed = true;
        }

        let color = new_style.color.to_raw();
        if width_changed {
            let mut pos = 0;
            for points in &self.paths {
                let len = get_len_path(points);
                tessellate_path(points, new_style.width, color, &mut vertices[pos..(pos + len)]);
                pos += len;
            }
        } else if color_changed {
            for vertex in vertices {
                vertex.a_color = color;
            }
        }

        width_changed || color_changed
    }
}