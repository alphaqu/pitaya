

use egui::{Align2, Color32, FontFamily, FontId, Painter, Pos2, Rect, Response, Rounding, Sense, Ui, Vec2};
use crate::ui::{INTERACTIVE_SIZE, SPACING_SIZE};

pub fn alloc_intractable(ui: &mut Ui, content_width: f32) -> (Rect, Response, Rounding) {
	let (rect, response) = ui.allocate_at_least(
		Vec2::new(
			SPACING_SIZE + content_width + SPACING_SIZE,
			INTERACTIVE_SIZE,
		),
		Sense::click_and_drag(),
	);

	(rect, response, Rounding::same(INTERACTIVE_SIZE / 2.0))
}

// icon is the Code point of the icon which you can get at https://fonts.google.com/icons.
// Please always put a comment of what the icon actually is or use the helpful icon! macro located in ptya-icon
pub fn draw_icon(painter: &Painter, icon: u32, pos: Pos2, size: f32, color: Color32) {
	let char = char::from_u32(icon).expect("Could not parse icon char");
	painter.text(pos - Vec2::new(0.0, size / 1.75), Align2::CENTER_TOP, char, FontId::new(
		size,
		FontFamily::Name("Icons".into()),
	), color);
}