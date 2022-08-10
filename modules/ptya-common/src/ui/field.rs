use crate::Settings;
use egui::{FontFamily, FontId, FontSelection, Rect, Response, Rounding, Style, TextBuffer, TextEdit, TextStyle, Ui, Vec2, Widget, WidgetText};

pub struct Field<'a, 's> {
    text: TextEdit<'a>,
    settings: &'s Settings,
}

impl<'a, 's> Field<'a, 's> {
    pub fn new(text: &'a mut dyn TextBuffer, settings: &'s Settings) -> Field<'a, 's> {
        Field {
	        text: TextEdit::singleline(text),
	        settings,
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
	    // why they use extreme? it looks like shit
	    ui.style_mut().visuals.extreme_bg_color = self.settings.style.bg_2;
	    let response = self.text
		    .margin(self.settings.margin - Vec2::new(5.0, 5.0)).desired_width(400.0)
		    .font(FontSelection::Style(TextStyle::Body))
		    .text_color(self.settings.style.fg_4)
		    .ui(ui);

	    response
    }
}
