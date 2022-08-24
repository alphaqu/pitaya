use crate::app::app::{App, Manifest};

pub struct AppContainer {
	app: Box<dyn App>,
	manifest: Manifest,
}

impl AppContainer {
	pub fn new(
		manifest: Manifest,
		app: Box<dyn App>,
	) -> AppContainer {
		AppContainer {
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
