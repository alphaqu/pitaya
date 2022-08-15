use crate::{Settings, System};
use egui::{FontFamily, FontId, FontSelection, Rect, Response, Rounding, Style, TextBuffer, TextEdit, TextStyle, Ui, Vec2, Widget, WidgetText};
use crate::color::color::{ColorState, ColorType};
use crate::settings::SPACING_SIZE;

pub struct Field<'a, 's> {
    text: TextEdit<'a>,
    system: &'s System,
}

impl<'a, 's> Field<'a, 's> {
    pub fn new(text: &'a mut dyn TextBuffer, settings: &'s System) -> Field<'a, 's> {
        Field {
	        text: TextEdit::singleline(text),
	        system: settings,
        }
    }

	pub fn hint(self, hint_text: impl Into<WidgetText>) -> Self {
		Field {
			text: self.text.hint_text(hint_text),
			..self
		}
	}

	pub fn password(self, password: bool) -> Self {
		Field {
			text: self.text.password(password),
			..self
		}
	}
}

impl<'a, 's> Widget for Field<'a, 's> {
    fn ui(self, ui: &mut Ui) -> Response {
	    // TODO custom field
	    let fg = self.system.color.fg(ColorType::Primary, ColorState::Idle);

	    // why they use extreme? it looks like shit
	    let response = self.text
		    .margin(Vec2::splat(SPACING_SIZE) - Vec2::new(5.0, 5.0)).desired_width(400.0)
		    .font(FontSelection::Style(TextStyle::Body))
		    .text_color(fg)
		    .ui(ui);

	    response
    }
}
