use crate::feature::FeatureFields;
use crate::geometry::path::PathGeometry;
use crate::types::MapVertex;

pub mod path;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub enum GeometryKind {
    Path,
}

pub enum GeometryData {
    Path(PathGeometry),
}

impl GeometryData {
    pub fn get_kind(&self) -> GeometryKind {
        match &self {
            GeometryData::Path(_) => GeometryKind::Path,
        }
    }
}

pub trait FeatureStyle<S> {
     fn get_style(&self, zoom: f32, scale: f32, fields: &FeatureFields) -> S;
}

pub trait Geometry {
    type Style;
    type FeatureStyle: FeatureStyle<Self::Style>;
    /// This also only gets compiled once for the same reasons as [Self::get_vertices_len]
    fn compile(&self,style: &Self::Style) -> (Vec<MapVertex>, Vec<u32>);

    /// Asks the geometry to update the mesh (if that is required) with the new style.
    /// # Arguments
    ///
    /// * `vertices`: The vertices in the current feature mesh.
    /// * `old_style`: Last used style. If its empty this mesh is new.
    /// * `new_style`: The new style.
    ///
    /// returns: If the mesh should be updated.
    fn update(
        &self,
        vertices: &mut [MapVertex],
        old_style: Option<&Self::Style>,
        new_style: &Self::Style,
    ) -> bool;
}
