use crate::app::app::{App, Manifest};
use crate::{System, Task};
use std::future::Future;

pub struct AppContainer {
	app: Option<Box<dyn App>>,
	manifest: Manifest,

	task: Task<Box<dyn App>>,
}

impl AppContainer {
	pub fn new(
		sys: &System,
		manifest: Manifest,
		launcher: impl Future<Output = Box<dyn App>> + Send + 'static,
	) -> AppContainer {
		let mut task = Task::new(&sys.runtime);
		task.launch(launcher).unwrap();

		AppContainer {
			app: None,
			manifest,
			task,
		}
	}

	pub fn manifest(&self) -> &Manifest {
		&self.manifest
	}

	pub fn app(&mut self) -> Option<&mut Box<dyn App>> {
		if self.app.is_none() {
			if let Some(app) = self.task.try_recv() {
				self.app = Some(app);
			}
		}

		self.app.as_mut()
	}
}
