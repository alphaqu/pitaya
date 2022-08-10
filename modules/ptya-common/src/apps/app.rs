use std::rc::Rc;
use glium::framebuffer::{RenderBuffer, SimpleFrameBuffer};
use egui::{Context, Id, Painter, Rect, Ui};
use image::RgbaImage;
use crate::Settings;

pub trait EGuiApplication {
	fn tick(&mut self, ui: &mut Ui, settings: &Settings);
}

pub trait OpenGLApplication {
	fn tick(&mut self, ui: &mut Ui, ctx: &Rc<glium::backend::Context>, fb: &mut SimpleFrameBuffer, rect: egui::Rect, settings: &Settings);
}

pub enum AppInstance {
	EGui(Box<dyn EGuiApplication>),
	OpenGL {
		ctx: Rc<glium::backend::Context>,
		buffer: Rc<RenderBuffer>,
		app: Box<dyn OpenGLApplication>,
	}
}

pub struct AppInfo {
	pub id: String,
	pub name: String,
	pub icon: RgbaImage,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct AppId {
	pub id: String,
}

impl AppId {
	pub fn egui_id(&self) -> egui::Id {
		Id::new("pitaya@app_id").with(&self.id)
	}
}
