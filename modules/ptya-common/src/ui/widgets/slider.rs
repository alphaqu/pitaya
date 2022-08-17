use crate::settings::{SPACING_SIZE, VISUAL_SIZE};
use crate::ui::animation::lerp::Lerp;

use crate::ui::animation::Easing;
use crate::ui::widgets::alloc_intractable;
use crate::ui::widgets::text::Text;
use crate::util::extend;
use crate::{AnimationComp, ColorState, ColorType, System, Vec2, INTERACTIVE_SIZE};
use egui::{Align2, Context, Rect, Response, RichText, Sense, Ui, WidgetText};
use epaint::{FontFamily, FontId, Rounding, Stroke};
use std::ops::Deref;

pub struct Slider<'a> {
    pub text: WidgetText,
    pub color: ColorType,

    pub decline_allowed: bool,
    pub system: &'a mut System,
}

impl<'a> Slider<'a> {
    pub fn new(text: impl Into<String>, decline: bool, system: &mut System) -> Slider {
        Slider {
            text: WidgetText::RichText(RichText::new(text).strong()),
            color: ColorType::Primary,
            decline_allowed: decline,
            system,
        }
    }

    fn get_slide_position(
        ctx: &Context,
        response: &Response,
        ani: &'a mut AnimationComp,
        decline_allowed: bool,
    ) -> (f32, f32) {
        let id = response.id;

        let time = ani.time;
        //  let mut back_state = State::new(id.with("back"), 0.0, 1.0, ctx);
        let mut current_pos = if let Some(pos) = ctx.data().get_temp(id) {
            pos
        } else {
            0.0f32
        };

        const DISTANCE: f32 = (VISUAL_SIZE + SPACING_SIZE) / 2.0;

        current_pos += response.drag_delta().x;
        if response.dragged() {
            ctx.data().insert_temp(id, current_pos.clamp(-(DISTANCE + 10.0), (DISTANCE + 10.0)));
        } else if response.drag_released() || response.drag_started() {
            ctx.data().remove::<f32>(id);
        }

        let slide_pos = (current_pos / DISTANCE)
            .clamp(if decline_allowed { -1.0 } else { 0.0 }, 1.0);

        if response.drag_released() {
            let mut speed = 1.0;
            if slide_pos.abs() >= 1.0  {
                ani.get(id.with("press")).set_current(slide_pos.clamp(-1.0, 1.0))
                    .set_easing(Easing::EaseOut)
                    .start(time, 0.0, 3.0);
                speed = 3.0;
            }

            ani.get(id)
                .set_current(slide_pos)
                .set_easing(Easing::EaseOut)
                .start(time, 0.0, speed);
        }

        let mut progress = ani.get(id).get_value(time);
        let press_progress =  ani.get(id.with("press")).get_value(time);
        // let progress = back_state.get_progress(ctx);
        if slide_pos != 0.0 {
            (slide_pos, press_progress)
        } else {
            (progress, press_progress)
        }
    }

    fn slide_lerp<L: Lerp>(slide_pos: f32, decline: &L, idle: &L, accept: &L) -> L {
        if slide_pos == 0.0 {
            return idle.clone();
        } else if slide_pos > 0.0 {
            idle.lerp(accept, slide_pos)
        } else {
            idle.lerp(decline, slide_pos.abs())
        }
    }

    pub fn show(self, ui: &mut Ui) -> SliderResponse {
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
        let (slide_pos, press_slide_phone
        ) = {
            let ani = &mut self.system.animation;
            Self::get_slide_position(ui.ctx(), &response, ani, self.decline_allowed)
        };
        let color = Self::slide_lerp(
            slide_pos,
            self.system.color.color.get_style(ColorType::Red),
            self.system.color.color.get_style(ColorType::Primary),
            self.system.color.color.get_style(ColorType::Green),
        );

        // Color setup
        let bg = self.system.color.compose(
            2.0,
            self.system.color.color.neutral.color,
            color.color,
            ColorState::Idle,
        );
        let fg = self
            .system
            .color
            .color
            .neutral
            .on_color
            .lerp(&color.color, slide_pos.abs());

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
                    rect.min.x += (text.width() / 2.0)   - (SPACING_SIZE / 4.0);
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
            self.system
                .color
                .compose_bg(3.5, self.system.color.color.green.color, ColorState::Idle).linear_multiply(1.0 - slide_pos.abs()),
            Stroke::none(),
        );

        if self.decline_allowed {
            painter.rect(
                {
                    let mut rect = Rect::from_min_max(rect.min, rect.center_bottom());
                    //rect.min.x += (SPACING_SIZE / 2.0);
                   // rect.min.y += HEIGHT_SHRINK;
                   // rect.max.y -= HEIGHT_SHRINK;
                    rect.max.x -= (text.width() / 2.0)  - (SPACING_SIZE / 4.0);
                    rect
                },
                Rounding {
                       ne: SPACING_SIZE / 2.0,
                       se: SPACING_SIZE / 2.0,
                       ..rounding
                   },
                self.system
                    .color
                    .compose_bg(3.5, self.system.color.color.red.color, ColorState::Idle).linear_multiply(1.0 - slide_pos.abs()),
                Stroke::none(),
            );
        }

        // Text Track
        painter.rect(
            text_track_rect.translate(item_offset).shrink2(Vec2::new(SPACING_SIZE / 4.0, SPACING_SIZE / 2.0)),
            rounding,
            bg,
            Stroke::none(),
        );

        // Icons
        let stuff_pos = slide_pos.abs();
        if self.decline_allowed {
            self.system.icon.draw(
                painter,
                "keyboard_double_arrow_left",
                decline_pos + item_offset,
                VISUAL_SIZE,
                self.system
                    .color
                    .color
                    .red
                    .color
                    .linear_multiply(1.0 - extend(0.0..0.5, stuff_pos.clamp(0.0, 0.5))),
            );

            self.system.icon.draw(
                painter,
                "done",
                decline_pos + item_offset,
                VISUAL_SIZE,
                self.system
                    .color
                    .color
                    .green
                    .color
                    .linear_multiply(extend(0.5..1.0, slide_pos.clamp(0.5, 1.0))),
            );
        }

        // painter.rect_filled(Rect::from_center_size(accept_pos + item_offset, Vec2::new(VISUAL_SIZE, VISUAL_SIZE)), 0.0, Color32::RED.linear_multiply(0.1));
        {
            self.system.icon.draw(
                painter,
                "keyboard_double_arrow_right",
                accept_pos + item_offset,
                VISUAL_SIZE,
                self.system
                    .color
                    .color
                    .green
                    .color
                    .linear_multiply(1.0 - extend(0.0..0.5, stuff_pos.clamp(0.0, 0.5))),
            );

            if self.decline_allowed {
                self.system.icon.draw(
                    painter,
                    "close",
                    accept_pos + item_offset,
                    VISUAL_SIZE,
                    self.system
                        .color
                        .color
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
            self.system.color.color.get_style(ColorType::Green)
        } else {
            self.system.color.color.get_style(ColorType::Red)
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
