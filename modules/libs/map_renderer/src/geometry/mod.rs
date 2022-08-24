use crate::geometry::fill::MultiPolygonGeometry;
use crate::geometry::path::MultiPathGeometry;

pub mod fill;
pub mod path;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[repr(C)]
pub enum GeometryKind {
	Path,
	Fill,
}

pub enum GeometryData {
	Path(MultiPathGeometry),
	Fill(MultiPolygonGeometry),
}

impl GeometryData {
	pub fn get_kind(&self) -> GeometryKind {
		match &self {
			GeometryData::Path(_) => GeometryKind::Path,
			GeometryData::Fill(_) => GeometryKind::Fill,
		}
	}
}
