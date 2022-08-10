use crate::apps::app::AppInfo;
use crate::ui::button::Button;
use crate::ui::field::Field;
use crate::{EGuiApplication, app_icon, Settings};
use egui::{Align2, Color32, FontId, Painter, Rect, Stroke, Ui, Widget};

pub mod button;
pub mod field;
pub mod animation;
pub mod layout;

pub struct WidgetApp {
    field: String,
}

impl WidgetApp {
    pub fn new() -> WidgetApp {
        WidgetApp {
            field: "hi".to_string(),
        }
    }

    pub fn app_info() -> AppInfo {
        AppInfo {
            id: "widgets".to_string(),
            name: "Widget".to_string(),
            icon: app_icon!("./icon.png"),
        }
    }
}

impl EGuiApplication for WidgetApp {
    fn tick(&mut self, ui: &mut Ui, settings: &Settings) {
        let painter = ui.painter();
        let rect = ui.max_rect();
        painter.rect(rect, 25.0, settings.style.bg_0, Stroke::none());
        painter.text(
            rect.center(),
            Align2::CENTER_CENTER,
            format!("{} x {}", rect.width(), rect.height()),
            FontId::proportional(50.0),
            Color32::WHITE,
        );
        
        ui.horizontal(|ui| {
            Button::new("hello", settings).ui(ui);
            Field::new(&mut self.field, settings).ui(ui);
        });
    }
}
