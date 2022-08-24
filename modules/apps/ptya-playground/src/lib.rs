use egui::Color32;
use glium::framebuffer::SimpleFrameBuffer;
use ptya_core::app::{App, Manifest, Version};
use ptya_core::ui::{Pui, ROUNDING};
use ptya_icon::icon;

pub fn manifest() -> Manifest {
    Manifest {
        id: "playground".to_string(),
        name: "Playground".to_string(),
        icon: icon!("attractions"),
        version: Version::new(0, 1, 1),
    }
}

pub async fn load() -> Box<dyn App> {
    Box::new(PlaygroundApp {})
}

pub struct PlaygroundApp {

}

impl App for PlaygroundApp {
    fn tick(&mut self, ui: &mut Pui, fb: &mut SimpleFrameBuffer) {
        ui.painter()
            .rect_filled(ui.max_rect(), ROUNDING, Color32::BLACK);
    }
}
