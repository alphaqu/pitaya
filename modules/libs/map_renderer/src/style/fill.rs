use crate::geometry::fill::PolygonGeometry;
use crate::style::Style;
use crate::types::MapVertex;
use mathie::Vec2D;

#[derive(Copy, Clone)]
pub struct FillStyleInput<'a> {
	pub outer: &'a [Vec2D<f32>],
	pub inner: &'a [Vec<Vec2D<f32>>],
}

impl<'a> From<&'a PolygonGeometry> for FillStyleInput<'a> {
	fn from(geometry: &'a PolygonGeometry) -> Self {
		FillStyleInput {
			outer: &geometry.outer,
			inner: &geometry.inner,
		}
	}
}

#[derive(Copy, Clone)]
pub struct FillStyle {
	pub color: [f32; 4],
}

impl FillStyle {
	pub fn new(color: [u8; 3]) -> FillStyle {
		FillStyle {
			color: [
				color[0] as f32 / 255.0,
				color[1] as f32 / 255.0,
				color[2] as f32 / 255.0,
				1.0,
			],
		}
	}
}

impl Style for FillStyle {
	type Input<'a> = FillStyleInput<'a>;

	fn get_len(input: Self::Input<'_>) -> usize {
		let mut len = input.outer.len();
		for inner in input.inner {
			len += inner.len();
		}
		len
	}

	fn compile(&self, input: Self::Input<'_>, v: &mut Vec<MapVertex>, i: &mut Vec<u32>) {
		let start = v.len();
		let len = Self::get_len(input);
		let mut data = Vec::with_capacity(len * 2);
		let mut hole_indices = Vec::with_capacity(input.inner.len());

		for vec in input.outer {
			data.push(vec.x() * 4096.0);
			data.push(vec.y() * 4096.0);
			v.push(MapVertex {
				a_pos: [vec.x(), vec.y()],
				a_color: self.color,
			})
		}

		for ring in input.inner {
			hole_indices.push(data.len() / 2);
			for vec in ring.iter().rev() {
				data.push(vec.x() * 4096.0);
				data.push(vec.y() * 4096.0);
				v.push(MapVertex {
					a_pos: [vec.x(), vec.y()],
					a_color: self.color,
				})
			}
		}

		for idx in earcutr::earcut(&data, &hole_indices, 2) {
			i.push((start + idx) as u32);
		}
	}

	fn needs_update(&self, old_styler: Self) -> bool {
		self.color != old_styler.color
	}

	fn update(&self, _: Self::Input<'_>, v: &mut [MapVertex], _: Option<Self>) {
		for vertex in v {
			vertex.a_color = self.color;
		}
	}

	fn prepare(&mut self, scale: f32) {}
}
