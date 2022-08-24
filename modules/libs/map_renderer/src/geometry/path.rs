use mathie::Vec2D;

pub struct MultiPathGeometry {
	pub paths: Vec<PathGeometry>,
}

pub struct PathGeometry {
	pub points: Vec<Vec2D<f32>>,
}
