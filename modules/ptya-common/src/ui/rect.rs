use serde::de::Unexpected::Str;
use egui::{Direction, Painter, Rect, Vec2};
use epaint::{Color32, Mesh, RectShape, Rounding, Stroke, Tessellator};
use crate::ui::animation::lerp::Lerp;
use crate::util::extend;

pub struct GradientFill {
	pub rect: Rect,
	pub rounding: Rounding,

	pub stroke_width: f32,
	pub from: Color32,
	pub to: Color32,
	pub dir: Direction
}

impl GradientFill {
	pub fn draw(&self, painter: &Painter) {
		let mut options = *painter.ctx().tessellation_options();
		let mut tesselator = Tessellator::new(
			painter.ctx().pixels_per_point(),
			options,
			[0, 0],
			vec![],
		);

		let mut out = Mesh::default();
		tesselator.tessellate_rect(&RectShape {
			rect: self.rect,
			rounding: self.rounding,
			fill: Color32::WHITE,
			stroke: Stroke::new(self.stroke_width, Color32::WHITE),
		}, &mut out);

		for vertex in &mut out.vertices {
			let pos = ((vertex.pos - self.rect.min) / self.rect.size()).clamp(
				Vec2::new(0.0, 0.0),
				Vec2::new(1.0, 1.0),
			);
			let t = extend(0.0..1.0, match self.dir {
				Direction::LeftToRight => pos.x,
				Direction::RightToLeft => 1.0 - pos.x,
				Direction::TopDown => pos.y,
				Direction::BottomUp => 1.0 - pos.y,
			}.clamp(0.0, 1.0));

			let a = (vertex.color.a() as f32 / 255.0);
			let from = self.from.linear_multiply(a);
			let to =  self.to.linear_multiply(a);
			vertex.color = from.lerp(&to, t);
		}

		painter.add(out);
	}
}

pub enum RectColor {
	Gradient {
		from: Color32,
		to: Color32,
		dir: Direction
	},
	Solid {
		color: Color32
	}
}