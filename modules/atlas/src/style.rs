use crate::feature::FeatureFields;
use crate::geometry::{FeatureStyle, GeometryKind};
use crate::predicate::{Predicate, ValueSelector};
use crate::types::Color;
use fxhash::FxHashMap;
use std::path::Path;

pub struct MapStyle {
    pub layers: FxHashMap<String, LayerStyle>,
}

pub struct LayerStyle {
    pub features: Vec<FeatureStyler>,
}

pub struct FeatureStyler {
    pub predicates: Vec<Predicate>,
    pub style: FeatureStyleType,
}

#[derive(Clone)]
pub enum FeatureStyleType {
    Path(PathFeatureStyle),
}

impl FeatureStyleType {
    pub fn get_kind(&self) -> GeometryKind {
        match &self {
            FeatureStyleType::Path(_) => GeometryKind::Path,
        }
    }
}

#[derive(Clone)]
pub struct PathFeatureStyle {
    pub color: ValueSelector<Color>,
    pub width: ValueSelector<f32>,
}

impl FeatureStyle<PathStyle> for PathFeatureStyle {
    fn get_style(&self, zoom: f32, scale: f32, fields: &FeatureFields) -> PathStyle {
        PathStyle {
            color: self.color.get_value(zoom, fields),
            width: self.width.get_value(zoom, fields) * scale,
        }
    }
}
pub struct PathStyle {
    pub color: Color,
    pub width: f32,
}
