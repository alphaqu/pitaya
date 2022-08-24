use crate::animation::{extend, Easing, Lerp};
use crate::color::ColorTag;
use crate::ui::components::Text;
use crate::ui::util::{alloc_intractable, draw_icon};
use crate::ui::{Pui, SPACING_SIZE, VISUAL_SIZE};
use egui::{
	Align2, FontFamily, FontId, Rect, Response, RichText, Rounding, Stroke, Vec2, WidgetText,
};
use ptya_icon::icon;
use std::ops::Deref;

pub struct Slider {
	pub text: WidgetText,
	pub color: ColorTag,

	pub decline_allowed: bool,
}

impl Slider {
	pub fn new(text: impl Into<String>, decline: bool) -> Slider {
		Slider {
			text: WidgetText::RichText(RichText::new(text).strong()),
			color: ColorTag::Primary,
			decline_allowed: decline,
		}
	}

	fn get_slide_position(ui: &mut Pui, response: &Response, decline_allowed: bool) -> (f32, f32) {
		let id = response.id;

		//  let mut back_state = State::new(id.with("back"), 0.0, 1.0, ctx);
		let mut current_pos = if let Some(pos) = ui.data().get_temp(id) {
			pos
		} else {
			0.0f32
		};

		const DISTANCE: f32 = (VISUAL_SIZE + SPACING_SIZE) / 2.0;

		current_pos += response.drag_delta().x;
		if response.dragged() {
			ui.data()
				.insert_temp(id, current_pos.clamp(-(DISTANCE + 10.0), DISTANCE + 10.0));
		} else if response.drag_released() || response.drag_started() {
			ui.data().remove::<f32>(id);
		}

		let slide_pos =
			(current_pos / DISTANCE).clamp(if decline_allowed { -1.0 } else { 0.0 }, 1.0);

		let ani = &ui.sys().animation;
		let mut press_ani = ani.get(id.with("press"));
		let mut progress_ani = ani.get(id);

		if response.drag_released() {
			let mut speed = 1.0;
			if slide_pos.abs() >= 1.0 {
				press_ani
					.set_easing(Easing::EaseOut)
					.set_from(slide_pos.clamp(-1.0, 1.0))
					.set_to(0.0)
					.begin_with_speed(3.0);
				speed = 3.0;
			}

			progress_ani
				.set_from(slide_pos)
				.set_to(0.0)
				.set_easing(Easing::EaseOut)
				.begin_with_speed(speed);
		}

		// let progress = back_state.get_progress(ctx);
		if slide_pos != 0.0 {
			(slide_pos, press_ani.get_value())
		} else {
			(progress_ani.get_value(), press_ani.get_value())
		}
	}

	fn slide_lerp<L: Lerp>(slide_pos: f32, decline: &L, idle: &L, accept: &L) -> L {
		if slide_pos == 0.0 {
			idle.clone()
		} else if slide_pos > 0.0 {
			idle.lerp(accept, slide_pos)
		} else {
			idle.lerp(decline, slide_pos.abs())
		}
	}

	pub fn show(self, ui: &mut Pui) -> SliderResponse {
		// Layout
		const FULL_TARGET_W: f32 = VISUAL_SIZE + SPACING_SIZE;
		let decline_size = self.decline_allowed as u8 as f32 * FULL_TARGET_W;
		let accept_size = FULL_TARGET_W;

		let text = Text::new(
			ui,
			self.text,
			None,
			FontId::new(40.0, FontFamily::Name("Roboto-Medium".into())),
		);

		let (rect, response, rounding) =
			alloc_intractable(ui, decline_size + text.width() + accept_size);

		// Slider math
		let (slide_pos, press_slide_phone) =
			{ Self::get_slide_position(ui, &response, self.decline_allowed) };
		let color = ui.color().ascend(1.0);
		let colors = Self::slide_lerp(slide_pos, &color.red, color.get(self.color), &color.green);

		// Color setup
		let bg = color.group_bg(&colors);
		let fg = color.fg.lerp(&colors.color, slide_pos.abs());

		// Drawing
		let painter = ui.painter();

		let decline_pos = rect.left_center() + Vec2::new((VISUAL_SIZE / 2.0) + SPACING_SIZE, 0.0);
		let accept_pos = rect.right_center() - Vec2::new((VISUAL_SIZE / 2.0) + SPACING_SIZE, 0.0);
		let item_offset = Vec2::new((FULL_TARGET_W / 2.0) * slide_pos, 0.0);

		// Track
		painter.rect_filled(rect, rounding, bg);

		let left_shrink = Vec2::new(decline_size, 0.0);
		let right_shrink = Vec2::new(accept_size, 0.0);

		let text_track_rect = Rect::from_min_max(rect.min + left_shrink, rect.max - right_shrink);
		// Confirm Track
		painter.rect(
			{
				let mut rect = Rect::from_min_max(rect.center_top(), rect.max - right_shrink);
				rect.max.x += right_shrink.x;
				if self.decline_allowed {
					rect.min.x += (text.width() / 2.0) - (SPACING_SIZE / 4.0);
				} else {
					rect.min.x += SPACING_SIZE;
				}
				// rect.max.y += HEIGHT_SHRINK;

				rect
			},
			Rounding {
				nw: SPACING_SIZE / 2.0,
				sw: SPACING_SIZE / 2.0,
				..rounding
			},
			color
				.ascend(1.0)
				.tag_bg(ColorTag::Green)
				.linear_multiply(1.0 - slide_pos.abs()),
			Stroke::none(),
		);

		if self.decline_allowed {
			painter.rect(
				{
					let mut rect = Rect::from_min_max(rect.min, rect.center_bottom());
					//rect.min.x += (SPACING_SIZE / 2.0);
					// rect.min.y += HEIGHT_SHRINK;
					// rect.max.y -= HEIGHT_SHRINK;
					rect.max.x -= (text.width() / 2.0) - (SPACING_SIZE / 4.0);
					rect
				},
				Rounding {
					ne: SPACING_SIZE / 2.0,
					se: SPACING_SIZE / 2.0,
					..rounding
				},
				color
					.ascend(1.0)
					.tag_bg(ColorTag::Red)
					.linear_multiply(1.0 - slide_pos.abs()),
				Stroke::none(),
			);
		}

		// Text Track
		painter.rect(
			text_track_rect
				.translate(item_offset)
				.shrink2(Vec2::new(SPACING_SIZE / 4.0, SPACING_SIZE / 2.0)),
			rounding,
			bg,
			Stroke::none(),
		);

		// Icons
		let stuff_pos = slide_pos.abs();
		if self.decline_allowed {
			draw_icon(
				painter,
				icon!("keyboard_double_arrow_left"),
				decline_pos + item_offset,
				VISUAL_SIZE,
				color
					.red
					.color
					.linear_multiply(1.0 - extend(0.0..0.5, stuff_pos.clamp(0.0, 0.5))),
			);

			draw_icon(
				painter,
				icon!("done"),
				decline_pos + item_offset,
				VISUAL_SIZE,
				color
					.green
					.color
					.linear_multiply(extend(0.5..1.0, slide_pos.clamp(0.5, 1.0))),
			);
		}

		{
			draw_icon(
				painter,
				icon!("keyboard_double_arrow_right"),
				accept_pos + item_offset,
				VISUAL_SIZE,
				color
					.green
					.color
					.linear_multiply(1.0 - extend(0.0..0.5, stuff_pos.clamp(0.0, 0.5))),
			);

			if self.decline_allowed {
				draw_icon(
					painter,
					icon!("close"),
					accept_pos + item_offset,
					VISUAL_SIZE,
					color
						.red
						.color
						.linear_multiply(extend(0.5..1.0, slide_pos.min(-0.5).abs())),
				);
			}
		}

		// Text
		text.draw(
			painter,
			rect.left_center() + Vec2::new(SPACING_SIZE + decline_size, 0.0) + item_offset,
			Align2::LEFT_CENTER,
			fg,
		);

		let stroke_color = if press_slide_phone > 0.0 {
			&color.green
		} else {
			&color.red
		};

		let stroke = Stroke::new(press_slide_phone.abs() * 5.0, stroke_color.color);
		painter.rect_stroke(rect, rounding, stroke);

		let mut confirm = false;
		let mut decline = false;
		if response.drag_released() && slide_pos != 0.0 {
			if slide_pos > 0.0 {
				confirm = true;
			} else if self.decline_allowed {
				decline = true;
			}
		}

		SliderResponse {
			response,
			confirm,
			decline,
		}
	}
}

#[derive(Clone)]
pub struct SliderResponse {
	pub response: Response,
	confirm: bool,
	decline: bool,
}

impl Deref for SliderResponse {
	type Target = Response;

	fn deref(&self) -> &Self::Target {
		&self.response
	}
}

impl SliderResponse {
	pub fn confirm(&self) -> bool {
		self.confirm
	}

	pub fn decline(&self) -> bool {
		self.decline
	}
}
