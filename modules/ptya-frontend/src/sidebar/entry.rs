use egui::{Rect, Response, Sense, Vec2};
use log::warn;
use ptya_core::app::AppId;
use ptya_core::ui::components::ProgressSpinner;
use ptya_core::ui::util::draw_icon;
use ptya_core::ui::{Pui, INTERACTIVE_SIZE, ROUNDING, VISUAL_SIZE};

pub struct SidebarEntry {
	pub id: AppId,
}

impl SidebarEntry {
	pub fn draw(&mut self, ui: &mut Pui) -> Response {
		const SIZE: f32 = INTERACTIVE_SIZE * 1.15;
		let (rect, response) =
			ui.allocate_exact_size(Vec2::new(SIZE, SIZE), Sense::click_and_drag());

		if let Some(app) = ui.sys.app.apps().get_mut(&self.id) {
			let app_loaded = app.app().is_some();
			// Render panel
			let color = ui.color().ascend(1.0);
			ui.painter().rect_filled(rect, ROUNDING, color.bg());
			let pos = rect.center();

			draw_icon(
				ui.painter(),
				app.manifest().icon,
				pos,
				SIZE,
				if app_loaded {
					color.fg
				} else {
					color.fg.linear_multiply(0.1)
				},
			);

			if !app_loaded {
				ProgressSpinner::new(None).draw(
					ui,
					Rect::from_center_size(pos, Vec2::new(VISUAL_SIZE, VISUAL_SIZE)),
					response.id.with("loading"),
				)
			}
		} else {
			warn!(
				"App {:?} is a SidebarEntry but does not exist in system",
				self.id
			);
		}

		response
	}
}
