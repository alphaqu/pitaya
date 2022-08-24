use egui::{Align2, FontFamily, FontId, Response, RichText, Stroke, WidgetText};
use crate::animation::{Easing, Lerp};
use crate::color::ColorTag;
use crate::ui::components::Text;
use crate::ui::Pui;
use crate::ui::util::alloc_intractable;

pub struct Button {
    pub text: WidgetText,
    pub color: ColorTag,
    pub action: ColorTag,
}

impl Button {
    pub fn new(text: impl Into<String>, action: ColorTag) -> Button {
        Button {
            text: WidgetText::RichText(RichText::new(text).strong()),
            color: ColorTag::Primary,
	        action,
        }
    }

    pub fn ui(self, ui: &mut Pui) -> Response {
        let text = Text::new(
            ui,
            self.text,
            None,
            FontId::new(40.0, FontFamily::Name("Roboto-Medium".into())),
        );

        let (rect, response, rounding) = alloc_intractable(ui, text.width());

        let mut click_animation = ui.sys().animation.get::<f32>(response.id);
        if response.clicked() || (response.drag_released() && response.hovered()) {
            click_animation
                .set_easing(Easing::EaseOut)
	            .set_from(1.0)
	            .set_to(0.0)
                .begin();
        }
        let click_value = click_animation.get_value();

	    let color = ui.color().ascend(1.0);

	    let button = color.get(self.color);
        let pressed = color.get(self.action);
        let accent = {
            if click_value != 0.0 {
                button.lerp(pressed, click_value)
            } else {
                button.clone()
            }
        };

	    let bg = color.group_bg(&accent);
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
	        color.fg.lerp(&pressed.color, click_value),
        );

        response
    }
}
