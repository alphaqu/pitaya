use std::rc::Rc;
use egui::TextureId;
use glium::backend::Context;
use glium::texture::{MipmapsOption, SrgbFormat, SrgbTexture2d};
use crate::app::app::{App, Manifest};

pub struct AppContainer {
	pub id: Option<TextureId>,
	pub framebuffer: Rc<SrgbTexture2d>,
	pub dirty: bool,
	pub app: Box<dyn App>,
	manifest: Manifest,
}

impl AppContainer {
	pub fn new(
		ctx: &Rc<Context>,
		manifest: Manifest,
		app: Box<dyn App>,
	) -> AppContainer {
		AppContainer {
			id: None,
			framebuffer: Rc::new(SrgbTexture2d::empty_with_format(
				ctx,
				SrgbFormat::U8U8U8U8,
				MipmapsOption::NoMipmap,
				0,
				0,
			)
				.unwrap()),
			dirty: false,
			app,
			manifest,
		}
	}

	pub fn manifest(&self) -> &Manifest {
		&self.manifest
	}

	pub fn app(&mut self) -> &mut dyn App {
		self.app.as_mut()
	}
}
