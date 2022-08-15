use crate::{Settings, System};
use egui::{epaint::TextShape, Color32, Response, Sense, Stroke, TextStyle, Ui, Vec2, Widget, WidgetText, RichText, Rounding};
use crate::color::color::{ColorState, ColorType};
use crate::settings::{INTERACTIVE_SIZE, SPACING_SIZE};

pub struct Button<'a> {
    pub text: WidgetText,
    pub color: ColorType,
    pub system: &'a System
}

impl<'a> Button<'a> {
    pub fn new(text: impl Into<String>, system: &System) -> Button {
        Button {
            text: WidgetText::RichText(RichText::new(text).strong()),
	        color: ColorType::Primary,
	        system
        }
    }
}

impl<'a> Widget for Button<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let text = self
            .text
            .into_galley(ui, None, f32::INFINITY, TextStyle::Button);
        let text_size = text.size();
        let (rect, response) = ui.allocate_at_least(
            Vec2::new(
	            INTERACTIVE_SIZE + text_size.x + SPACING_SIZE,
	            INTERACTIVE_SIZE + SPACING_SIZE
            ),
            Sense::click_and_drag(),
        );
	    let state = ColorState::new(&response);
        let rect = rect.shrink(SPACING_SIZE);
        let painter = ui.painter();

	  //  let visuals = ui.style().interact(&response);
	    let bg = self.system.color.bg(2.0, self.color, state);
	    painter.rect(rect, Rounding::same(INTERACTIVE_SIZE / 2.0), bg, Stroke::none());

	    let fg = self.system.color.bg(22.0, self.color, state);
	    painter.add(TextShape {
            pos: rect.center() - (text_size / 2.0),
            galley: text.galley,
            underline: Default::default(),
            override_text_color: Some(fg),
            angle: 0.0,
        });

        response
    }
}
