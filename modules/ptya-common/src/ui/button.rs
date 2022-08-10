use crate::Settings;
use egui::{epaint::TextShape, Color32, Response, Sense, Stroke, TextStyle, Ui, Vec2, Widget, WidgetText, RichText};

pub struct Button<'a> {
    pub text: WidgetText,
    pub settings: &'a Settings
}

impl<'a> Button<'a> {
    pub fn new(text: impl Into<String>, settings: &Settings) -> Button {
        Button {
            text: WidgetText::RichText(RichText::new(text).strong()),
	        settings
        }
    }
}

impl<'a> Widget for Button<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
	    let padding = self.settings.layout.button_padding;
        let text = self
            .text
            .into_galley(ui, None, f32::INFINITY, TextStyle::Button);

        let size = text.size();
        let (rect, response) = ui.allocate_at_least(
            size + (padding * 2.0) + (self.settings.margin * 2.0),
            Sense::click_and_drag(),
        );

        let rect = rect.shrink2(self.settings.margin);
        let painter = ui.painter();

	    let visuals = ui.style().interact(&response);

	    let stroke = if response.hovered() {
		    ui.style().visuals.widgets.hovered.bg_stroke
	    } else {
		    Stroke::none()
	    };

	    painter.rect(rect, self.settings.layout.button_rounding, visuals.bg_fill, visuals.bg_stroke);

	    let text_color = if ui.is_enabled() {
		    self.settings.style.fg_4
	    } else {
		    self.settings.style.fg_2
	    };

        let pos = rect.left_top() + Vec2::new(padding.x, padding.y);
        painter.add(TextShape {
            pos,
            galley: text.galley,
            underline: Default::default(),
            override_text_color: Some(text_color),
            angle: 0.0,
        });

        response
    }
}
