use mathie::Vec2D;

pub struct MultiPolygonGeometry {
	pub polygons: Vec<PolygonGeometry>,
}

pub struct PolygonGeometry {
	pub outer: Vec<Vec2D<f32>>,
	pub inner: Vec<Vec<Vec2D<f32>>>,
}
