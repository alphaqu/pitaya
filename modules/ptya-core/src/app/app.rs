use std::future::Future;
use glium::framebuffer::SimpleFrameBuffer;
use semver::Version;
use egui::{Id};
use crate::System;
use crate::ui::Pui;

pub trait App: Send {
	/// Runs every frame.
	fn tick(&mut self, ui: &mut Pui, fb: &mut SimpleFrameBuffer);
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
