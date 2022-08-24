pub use crate::app::app::App;
pub use crate::app::app::AppId;
pub use crate::app::app::Manifest;
pub use crate::app::container::AppContainer;
use crate::System;
use ahash::AHashMap;
use egui::mutex::{Mutex, MutexGuard};
pub use semver::Version;
use std::future::Future;

mod app;
mod container;

pub const API_VERSION: u64 = 0;

pub struct AppManager {
	apps: Mutex<AHashMap<AppId, AppContainer>>,
}

impl AppManager {
	pub fn new() -> AppManager {
		AppManager {
			apps: Default::default(),
		}
	}

	pub fn load_app(
		&self,
		sys: &System,
		manifest: Manifest,
		launcher: impl Future<Output = Box<dyn App>> + Send + 'static,
	) {
		let container = AppContainer::new(sys, manifest, launcher);
		self.apps.lock().insert(
			AppId {
				id: container.manifest().id.clone(),
			},
			container,
		);
	}

	pub fn apps(&self) -> MutexGuard<'_, AHashMap<AppId, AppContainer>> {
		self.apps.lock()
	}
}
