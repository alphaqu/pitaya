use egui::{Align2, Color32, FontSelection, Painter, Pos2, Rect, Ui, Vec2, WidgetText};
use egui::widget_text::WidgetTextGalley;

pub struct Text {
	galley: WidgetTextGalley
}

impl Text {
	pub fn new(ui: &Ui, text: impl Into<WidgetText>, wrap_width: Option<f32>, font: impl Into<FontSelection>) -> Text {
		let text =text.into();
		let galley = text.into_galley(
			ui, wrap_width.map(|_| true), wrap_width.unwrap_or(f32::INFINITY), font
		);
		Text {
			galley
		}
	}

	pub fn height(&self) -> f32 {
		self.galley.size().y
	}

	pub fn width(&self) -> f32 {
		self.galley.size().x
	}

	pub fn size(&self) -> Vec2 {
		self.galley.size()
	}

	pub fn text(&self) -> &str {
		self.galley.text()
	}

	pub fn draw(&self, painter: &Painter, pos: Pos2, anchor: Align2, color: Color32)  {
		let rect = anchor.anchor_rect(Rect::from_min_size(pos, self.size()));
		painter.galley_with_color(rect.min, self.galley.galley.clone(), color);
	}
}