use crate::feature::{Feature, FeatureFields, FeatureInstance, FeatureInstanceType};
use crate::geometry::{FeatureStyle, Geometry};
use crate::layer::LayerData;
use crate::types::MapVertex;
use glium::backend::Context;
use glium::index::PrimitiveType;
use glium::{IndexBuffer, VertexBuffer};
use std::rc::Rc;

pub struct TileData {
    pub layers: Vec<LayerData>,
}

pub struct TileMeshBuilder {
    indices: Vec<u32>,
    vertices: Vec<MapVertex>,
    features: Vec<Feature>,
}

impl TileMeshBuilder {
    pub fn new() -> TileMeshBuilder {
        TileMeshBuilder {
            indices: vec![],
            vertices: vec![],
            features: vec![],
        }
    }

    pub fn push_feature<G: Geometry>(
        &mut self,
        geometry: G,
        styler: G::FeatureStyle,
        fields: FeatureFields,
        scale: f32,
        zoom: u8,
    ) where
        FeatureInstance<G>: Into<FeatureInstanceType>,
    {
        // Compile the feature
        let style = styler.get_style(zoom as f32, scale, &fields);
        let (vertices, indices) = geometry.compile(&style);

        // Builder Vertices
        let start = self.vertices.len();
        self.vertices.extend(&vertices);
        let vertices_range = start..self.vertices.len();

        // Builder Indices
        for idx in indices {
            self.indices.push(start as u32 + idx);
        }

        self.features.push(Feature {
            vertices_range,
            vertices,
            fields,
            ty: FeatureInstance {
                old_style: style,
                geometry,
                styler,
            }
            .into(),
        });
    }

    pub fn build(self, ctx: &Rc<Context>) -> TileMesh {
        TileMesh {
            vertices: VertexBuffer::new(ctx, &self.vertices).unwrap(),
            indices: IndexBuffer::new(ctx, PrimitiveType::TrianglesList, &self.indices).unwrap(),
            features: self.features,
        }
    }
}

pub struct TileMesh {
    pub vertices: VertexBuffer<MapVertex>,
    pub indices: IndexBuffer<u32>,

    features: Vec<Feature>,
}

impl TileMesh {
    pub fn update(&mut self, zoom: f32, scale: f32) {
        for feature in &mut self.features {
            feature.update(zoom, scale, &mut self.vertices);
        }
    }
}
