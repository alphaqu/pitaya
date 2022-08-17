use crate::settings::SPACING_SIZE;
use crate::ui::animation::lerp::Lerp;
use crate::ui::animation::Easing;
use crate::ui::widgets::alloc_intractable;
use crate::ui::widgets::text::Text;
use crate::{ColorState, ColorType, System, INTERACTIVE_SIZE};
use egui::{Align2, Response, RichText, Sense, Ui, Vec2, WidgetText};
use epaint::{FontFamily, FontId, Stroke};

pub struct Button<'a> {
    pub text: WidgetText,
    pub color: ColorType,
    pub action: ColorType,
    pub system: &'a mut System,
}

impl<'a> Button<'a> {
    pub fn new(text: impl Into<String>, action: ColorType, system: &'a mut System) -> Button<'a> {
        Button {
            text: WidgetText::RichText(RichText::new(text).strong()),
            color: ColorType::Primary,
	        action,
	        system,
        }
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let text = Text::new(
            ui,
            self.text,
            None,
            FontId::new(40.0, FontFamily::Name("Roboto-Medium".into())),
        );

        let (rect, response, rounding) = alloc_intractable(ui, text.width());

        let time = self.system.animation.time;
        let click_animation = self.system.animation.get(response.id);

        if response.clicked() || (response.drag_released() && response.hovered()) {
            click_animation
                .set_easing(Easing::EaseOut)
                .set_current(1.0)
                .start(time, 0.0, 3.0);
        }
        let click_value = click_animation.get_value(time);

        let button = self.system.color.color.get_style(self.color);
        let pressed = self.system.color.color.get_style(self.action);
        let accent = {
            if click_value != 0.0 {
                button.lerp(pressed, click_value)
            } else {
                button.clone()
            }
        };

        let bg = self.system.color.compose(
	        2.0,
	        self.system.color.color.neutral.color,
	        accent.color,
	        ColorState::new(&response),
        );

        let painter = ui.painter();
        painter.rect(
	        rect,
	        rounding,
	        bg,
	        Stroke::new(click_value * 5.0, pressed.color),
        );
        text.draw(
	        painter,
	        rect.center(),
	        Align2::CENTER_CENTER,
	        self.system
                .color
                .color
                .neutral
                .on_color
                .lerp(&pressed.color, click_value),
        );

        response
    }
}
