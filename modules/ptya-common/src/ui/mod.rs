use crate::app::{AppImpl, Manifest};
use crate::color::color::{ColorGroup, ColorState, ColorType};
use crate::settings::ROUNDING_SIZE;
use crate::ui::widgets::button::Button;
use crate::{Settings, System, INTERACTIVE_SIZE, SPACING_SIZE};
use egui::style::Spacing;
use egui::{Align2, Color32, FontId, Painter, Rect, Sense, Stroke, Ui, Vec2, Widget};
use glium::framebuffer::SimpleFrameBuffer;
use widgets::slider::Slider;

pub mod animation;
pub mod layout;
mod rect;

pub mod widgets;

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
    fn update(&mut self, system: &mut System) {
        todo!()
    }

    fn tick(&mut self, ui: &mut Ui, fb: &mut SimpleFrameBuffer, system: &mut System) {
        let painter = ui.painter();
        let rect = ui.max_rect();
        painter.text(
            rect.center(),
            Align2::CENTER_CENTER,
            format!("{} x {}", rect.width(), rect.height()),
            FontId::proportional(50.0),
            Color32::WHITE,
        );
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.label("buttons");
                Button::new("Download Update", ColorType::Green, system).show(ui);
                Button::new("Later", ColorType::Yellow, system).show(ui);
            });

            ui.group(|ui| {
                ui.label("slider");
                Slider::new("Answer", true, system).show(ui);
                Slider::new("Confirm", false, system).show(ui);
            });
        });

        return;
        {
            fn draw_color(ui: &mut Ui, color: Color32, fg: Color32, name: &str) {
                let (rect, response) = ui.allocate_at_least(
                    Vec2::new(INTERACTIVE_SIZE * 4.0, INTERACTIVE_SIZE * 1.25),
                    Sense::click_and_drag(),
                );
                ui.painter().rect_filled(rect, 0.0, color);
                let pos2 = rect.left_top() + Vec2::new(25.0, 25.0);
                ui.painter()
                    .text(pos2, Align2::LEFT_TOP, name, FontId::proportional(30.0), fg);
            }

            fn draw_color_group(ui: &mut Ui, group: &ColorGroup, name: &str) {
                ui.horizontal(|ui| {
                    draw_color(ui, group.color, group.on_color, name);
                    draw_color(ui, group.on_color, group.color, &format!("On {name}"));
                    draw_color(
                        ui,
                        group.color_container,
                        group.on_color_container,
                        &format!("{name} Container"),
                    );
                    draw_color(
                        ui,
                        group.on_color_container,
                        group.color_container,
                        &format!("On {name} Container"),
                    );
                });
            }

            ui.vertical(|ui| {
                ui.style_mut().spacing.item_spacing = Vec2::new(0.0, 10.0);
                draw_color_group(ui, &system.color.color.primary, "Primary");
                draw_color_group(ui, &system.color.color.secondary, "Secondary");
                draw_color_group(ui, &system.color.color.tertiary, "Tertiary");
                draw_color_group(ui, &system.color.color.red, "Red");
                draw_color_group(ui, &system.color.color.orange, "Orange");
                draw_color_group(ui, &system.color.color.yellow, "Yellow");
                draw_color_group(ui, &system.color.color.green, "Green");
                draw_color_group(ui, &system.color.color.blue, "Blue");
                draw_color_group(ui, &system.color.color.neutral, "Neutral");
            });
        }
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
