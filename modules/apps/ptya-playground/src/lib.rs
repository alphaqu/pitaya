use egui::{Color32, Sense};
use glium::framebuffer::SimpleFrameBuffer;
use ptya_core::app::{App, Manifest, Version};
use ptya_core::color::ColorTag;
use ptya_core::System;
use ptya_core::ui::{Pui, ROUNDING};
use ptya_core::ui::components::{Button, Slider};
use ptya_icon::icon;

pub fn manifest() -> Manifest {
	Manifest {
		id: "playground".to_string(),
		name: "Playground".to_string(),
		icon: icon!("attractions"),
		version: Version::new(0, 1, 1),
	}
}

pub fn load() -> Box<dyn App> {
	Box::new(PlaygroundApp {})
}

pub struct PlaygroundApp {}

impl App for PlaygroundApp {
	fn tick(&mut self, ui: &mut Pui, fb: &mut SimpleFrameBuffer) {
		//ui.painter().rect_filled(ui.max_rect(), 0.0, ui.color().ascend(10.0).tag_bg(ColorTag::Red));
		Button::new("hello", ColorTag::Blue).ui(ui);
		//Slider::new("hello", false).show(ui);
		//if ui.interact(ui.max_rect(), ui.id().with("69420"), Sense::click_and_drag()).hovered() {
		//	println!("hover playground");
		//}
	}

	fn update(&mut self, system: &System) {

	}
}
