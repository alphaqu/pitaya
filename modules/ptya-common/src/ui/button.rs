use crate::Settings;
use egui::{epaint::TextShape, Color32, Response, Sense, Stroke, TextStyle, Ui, Vec2, Widget, WidgetText, RichText, Rounding};

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
        let text = self
            .text
            .into_galley(ui, None, f32::INFINITY, TextStyle::Button);
        let size = text.size();
	    let spacing = self.settings.layout.spacing_size;
        let (rect, response) = ui.allocate_at_least(
            Vec2::new(
	            self.settings.layout.interactive_size + size.x + spacing,
	            self.settings.layout.interactive_size + spacing
            ),
            Sense::click_and_drag(),
        );

        let rect = rect.shrink(spacing);
        let painter = ui.painter();

	    let visuals = ui.style().interact(&response);

	    let stroke = if response.hovered() {
		    ui.style().visuals.widgets.hovered.bg_stroke
	    } else {
		    Stroke::none()
	    };

	    painter.rect(rect, Rounding::same(f32::INFINITY), visuals.bg_fill, visuals.bg_stroke);

	    let text_color = if ui.is_enabled() {
		    self.settings.style.fg_4
	    } else {
		    self.settings.style.fg_2
	    };

        painter.add(TextShape {
            pos: rect.center(),
            galley: text.galley,
            underline: Default::default(),
            override_text_color: Some(text_color),
            angle: 0.0,
        });

        response
    }
}
