use lyon::geom::euclid::default::Rect;
use lyon::geom::euclid::{Point2D, Size2D, Vector2D};
use lyon::math::Point;
use lyon::path::{Path, Winding};
use lyon::path::builder::{BorderRadii, NoAttributes};
use lyon::path::path::Builder;
use lyon::tessellation::{BuffersBuilder, FillBuilder, FillOptions, FillTessellator, FillVertex, FillVertexConstructor, VertexBuffers};
use lyon::tessellation::geometry_builder::simple_builder;
use egui::{Direction, Pos2};
use epaint::{Color32, Mesh, Rounding, Shape, Vec2, Vertex};
use crate::ui::animation::lerp::Lerp;

pub struct Gradient {
	pub mesh: Mesh
}

impl Gradient {
	pub fn circle(pos: Pos2, radius: f32, fill: GradientFill) -> Gradient  {
		let rect = egui::Rect::from_center_size(pos, Vec2::splat(radius * 2.0));
		let mesh = Self::tessellate(rect, fill, |builder| {
			let rect = Rect::new(
				Point2D::new(rect.min.x, rect.min.y),
				Size2D::new(rect.width(), rect.height()),
			);
			builder.add_circle(rect.center(), radius,Winding::Positive);
		});

		Gradient  {
			mesh
		}
	}

	pub fn rect(rect: egui::Rect, rounding: Rounding, fill: GradientFill) -> Gradient {
		let mesh = Self::tessellate(rect, fill, |builder| {
			let rect = Rect::new(
				Point2D::new(rect.min.x, rect.min.y),
				Size2D::new(rect.width(), rect.height()),
			);
			if rounding == Rounding::none() {
				builder.add_rectangle(&rect.to_box2d(), Winding::Positive);
			} else {
				builder.add_rounded_rectangle(&rect.to_box2d(), &BorderRadii {
					top_left: rounding.nw,
					top_right: rounding.ne,
					bottom_left: rounding.sw,
					bottom_right: rounding.se
				}, Winding::Positive);
			}
		});

		Gradient  {
			mesh
		}
	}

	fn tessellate(area: egui::Rect, fill: GradientFill, func: impl FnOnce(&mut NoAttributes<FillBuilder>)) -> Mesh {
		struct Ctor {
			fill: GradientFill,
			area: egui::Rect,
		};

		impl FillVertexConstructor<Vertex> for Ctor {
			fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
				let position = vertex.position();
				let pos = Pos2::new(
					position.x,
					position.y,
				);
				let gradient_pos = ((pos - self.area.min) / self.area.size()).clamp(
					Vec2::new(0.0, 0.0),
					Vec2::new(1.0, 1.0),
				);
				let t = match self.fill.direction {
					Direction::LeftToRight => gradient_pos.x,
					Direction::RightToLeft => 1.0 - gradient_pos.x,
					Direction::TopDown => gradient_pos.y,
					Direction::BottomUp => 1.0 - gradient_pos.y,
				};

				Vertex {
					pos,
					uv: Default::default(),
					color: self.fill.from.lerp(&self.fill.to, t)
				}
			}
		}

		let mut buffers: VertexBuffers<Vertex, u32> = VertexBuffers::new();
		{
			let mut vertex_builder = BuffersBuilder::new(&mut buffers, Ctor { fill, area });
			let mut tessellator = FillTessellator::new();
			let options = FillOptions::default().with_intersections(false).with_tolerance(0.0001);
			let mut builder = tessellator.builder(&options, &mut vertex_builder);
			func(&mut builder);
			builder.build().unwrap();
		}
		Mesh {
			indices: buffers.indices,
			vertices: buffers.vertices,
			texture_id: Default::default()
		}
	}
}

impl Into<Shape> for Gradient {
	fn into(self) -> Shape {
		Shape::Mesh(self.mesh)
	}
}

pub struct GradientFill {
	pub from: Color32,
	pub to: Color32,
	pub direction: Direction,
}