use crate::geometry::path::PathGeometry;
use crate::geometry::{FeatureStyle, Geometry, GeometryData};
use crate::types::MapVertex;
use fxhash::FxHashMap;
use std::ops::Range;
use glium::VertexBuffer;

pub struct FeatureData {
    pub fields: FeatureFields,
    pub geometry: GeometryData,
}

pub struct Feature {
    pub(crate) vertices_range: Range<usize>,
    pub(crate) vertices: Vec<MapVertex>,
    pub(crate) fields: FeatureFields,
    pub(crate) ty: FeatureInstanceType,
}

impl Feature {
    pub fn update(&mut self, zoom: f32, scale: f32, vertices: &mut VertexBuffer<MapVertex>) {
        let old_len = self.vertices.len();
        let changed = match &mut self.ty {
            FeatureInstanceType::Path(path) => path.update(zoom, scale,&self.fields, &mut self.vertices),
        };
        
        if self.vertices.len() != old_len {
            panic!("Length of the vertices changed. This is really bad and someone forgot to read.")
        }

        if changed {
            let slice = vertices.slice_mut(self.vertices_range.clone()).unwrap();
            slice.write(&self.vertices);
        }
    }
}

pub enum FeatureInstanceType {
    Path(FeatureInstance<PathGeometry>),
}

pub struct FeatureInstance<G: Geometry> {
    pub(crate) old_style: G::Style,
    pub(crate) geometry: G,
    pub(crate) styler: G::FeatureStyle,
}

impl Into<FeatureInstanceType> for FeatureInstance<PathGeometry> {
    fn into(self) -> FeatureInstanceType {
        FeatureInstanceType::Path(self)
    }
}

impl<G: Geometry> FeatureInstance<G> {
    pub fn update(
        &mut self,
        zoom: f32,
        scale: f32,
        fields: &FeatureFields,
        vertices: &mut [MapVertex],
    ) -> bool {
        let style = self.styler.get_style(zoom, scale,fields);
        let changed = self
            .geometry
            .update(vertices, Some(&self.old_style), &style);
        self.old_style = style;
        changed
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    F32(f32),
    F64(f64),
    I64(i64),
    U64(u64),
    Bool(bool),
}

impl Value {
    pub fn to_str(&self) -> Option<&str> {
        match self {
            Value::String(string) => Some(&**string),
            Value::F32(_) => None,
            Value::F64(_) => None,
            Value::I64(_) => None,
            Value::U64(_) => None,
            Value::Bool(_) => None,
        }
    }

    pub fn to_f64(&self) -> Option<f64> {
        match self {
            Value::String(_) => None,
            Value::F32(v) => Some(*v as f64),
            Value::F64(v) => Some(*v as f64),
            Value::I64(v) => Some(*v as f64),
            Value::U64(v) => Some(*v as f64),
            Value::Bool(_) => None,
        }
    }
}
pub struct FeatureFields {
    pub fields: FxHashMap<String, Value>,
}
