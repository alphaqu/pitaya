use glium::implement_vertex;
use delaunator::Point;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Color(pub [u8; 4]);

impl Color {
	pub fn to_raw(self) -> [f32; 4] {
		[
			self.0[0] as f32 / 255.0,
			self.0[1] as f32 / 255.0,
			self.0[2] as f32 / 255.0,
			self.0[3] as f32 / 255.0,
		]
	}
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct MapVertex {
	pub a_pos: [f32; 2],
	pub a_color: [f32; 4],
}

implement_vertex!(MapVertex, a_pos, a_color);


pub(crate) fn compile_delaunator(points: &[Point]) -> Vec<u32> {
	delaunator::triangulate(points).triangles.into_iter().map(|v| v as u32).collect()
}