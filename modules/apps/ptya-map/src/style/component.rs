use std::ops::Range;
use lyon_tessellation::TessellationResult;
use crate::graphics::painter::Tessellator;
use crate::style::filter::ComponentFilter;
use crate::style::style::{CircleStyle, PolygonStyle, LineStyle};
use crate::tile::{Feature, Geometry};

pub struct Component {
	pub layer_name: String,
	pub style: ComponentStyle,

	pub filter: ComponentFilter,
	pub zoom_range: Range<u8>
}

impl Component {
	pub fn draw(&self, feature: &Feature, zoom: f32, tess: &mut Tessellator) -> TessellationResult {
		if !self.filter.matches(feature) {
			return Ok(());
		}

		match (&feature.geo, &self.style){
			(Geometry::Point { x, y }, ComponentStyle::Circle(style)) => {
				style.draw(*x, *y, zoom, tess)
			}
			(Geometry::Line { path }, ComponentStyle::Line(style)) => {
				style.draw(path, zoom, tess)
			}
			(Geometry::Polygon { polygons }, ComponentStyle::Polygon(style)) => {
				style.draw(polygons, zoom, tess)
			}
			_ => Ok(())
		}
	}
}
pub enum ComponentStyle {
	Circle(CircleStyle),
	Line(LineStyle),
	Polygon(PolygonStyle),
}