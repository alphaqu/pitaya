use crate::ui::Pui;
use egui::Id;
use glium::framebuffer::SimpleFrameBuffer;
use semver::Version;
use crate::System;

pub trait App {
	/// Runs every frame.
	fn tick(&mut self, ui: &mut Pui, fb: &mut SimpleFrameBuffer);

	/// Runs when the system settings get applied.
	fn update(&mut self, system: &System);
}

#[derive(Clone)]
pub struct Manifest {
	pub id: String,
	pub name: String,
	pub icon: u32,
	pub version: Version,
	//pub icon: Icon,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub struct AppId {
	pub id: String,
}

impl AppId {
	pub fn egui_id(&self) -> Id {
		Id::new("pitaya@app_id").with(&self.id)
	}
}
