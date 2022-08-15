use glium::framebuffer::SimpleFrameBuffer;
use crate::ui::button::Button;
use crate::ui::field::Field;
use egui::{Align2, Color32, FontId, Painter, Rect, Sense, Stroke, Ui, Widget};
use crate::app::{AppImpl, Manifest};
use crate::{Settings, System};
use crate::color::color::{ColorState, ColorType};
use crate::settings::ROUNDING_SIZE;

pub mod button;
pub mod field;
pub mod animation;
pub mod layout;
pub mod spinner;

pub struct WidgetApp {
    field: String,
}

impl WidgetApp {
    pub fn new() -> WidgetApp {
        WidgetApp {
            field: "hi".to_string(),
        }
    }

    pub fn app_info() -> Manifest {
        Manifest {
            id: "widgets".to_string(),
            //icon: Icon::MaterialSymbols("widgets".to_string()),
        }
    }
}

impl AppImpl for WidgetApp {
    fn update(&mut self, system: &System) {
        todo!()
    }

    fn tick(&mut self, ui: &mut Ui, fb: &mut SimpleFrameBuffer, system: &System) {
        let painter = ui.painter();
        let rect = ui.max_rect();
        painter.rect(rect.shrink(10.0), ROUNDING_SIZE, system.color.bg(1.0, ColorType::Primary, ColorState::Idle), Stroke::new(10.0, Color32::RED));
        painter.text(
            rect.center(),
            Align2::CENTER_CENTER,
            format!("{} x {}", rect.width(), rect.height()),
            FontId::proportional(50.0),
            Color32::WHITE,
        );
        ui.horizontal(|ui| {
            Button::new("hello", system).ui(ui);
            Field::new(&mut self.field, system).ui(ui);
        });
    }

    //fn tick(&mut self, ui: &mut Ui, settings: &Settings) {
   //    let painter = ui.painter();
   //    let rect = ui.max_rect();
   //    painter.rect(rect, 25.0, settings.color.bg(1.0, ColorType::Primary), Stroke::none());
   //    painter.text(
   //        rect.center(),
   //        Align2::CENTER_CENTER,
   //        format!("{} x {}", rect.width(), rect.height()),
   //        FontId::proportional(50.0),
   //        Color32::WHITE,
   //    );
   //
   //    ui.horizontal(|ui| {
   //        Button::new("hello", settings).ui(ui);
   //        Field::new(&mut self.field, settings).ui(ui);
   //    });
   //}
}
