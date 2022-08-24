pub mod animation {
	pub use ptya_animation::*;
}
pub mod app;
pub mod asset {
	pub use ptya_asset::*;
}
pub mod color {
	pub use ptya_color::*;
}
pub mod config;
pub mod task;
pub mod ui;

use crate::config::Config;
use crate::task::Task;
use anyways::ext::AuditExt;
use anyways::Result;
use egui::{CentralPanel, Color32, Frame, Spinner, Widget};
use glium::backend::Context;
use log::{info, LevelFilter};
use ptya_animation::AnimationManager;
use ptya_asset::{AssetManager, Location};
use ptya_color::ColorManager;
use simplelog::{ColorChoice, CombinedLogger, TermLogger, TerminalMode, WriteLogger};
use std::fs::File;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;
use tokio::join;
use tokio::runtime::Runtime;

use crate::app::AppManager;
use crate::ui::UiAssets;

pub struct System {
	pub gl_ctx: Rc<Context>,
	pub egui_ctx: egui::Context,
	pub runtime: Arc<Runtime>,

	pub app: AppManager,

	task: Task<Result<InitializedSystem>>,
	inner: Option<InitializedSystem>,
}

impl System {
	pub fn new(ctx: egui::Context, gl_ctx: Rc<Context>) -> Result<System> {
		init_logging().wrap_err("Failed to init logging")?;
		let runtime = Arc::new(Runtime::new().wrap_err("Failed to init multithreaded runtime.")?);

		let mut task = Task::new(&runtime);
		task.launch(async {
			info!("Launching inner system");
			InitializedSystem::new().await
		})
		.unwrap();

		Ok(System {
			gl_ctx: gl_ctx,
			egui_ctx: ctx,
			runtime,
			app: AppManager::new(),
			task,
			inner: None,
		})
	}

	pub fn is_loaded(&self) -> bool {
		self.inner.is_some()
	}

	pub fn tick(&mut self) -> Result<bool> {
		let mut updated = false;
		if self.inner.is_none() {
			CentralPanel::default()
				.frame(Frame::none().fill(Color32::BLACK))
				.show(&self.egui_ctx.clone(), |ui| {
					ui.centered_and_justified(|ui| {
						Spinner::new().size(40.0).ui(ui);
					});
				});

			if let Some(value) = self.task.try_recv() {
				let mut system = value.wrap_err("Failed to initialize pitaya")?;
				if let Some(assets) = system.assets.take() {
					assets.apply(self.egui_ctx.clone());
				}

				self.app.apps().clear();
				self.inner = Some(system);
				info!("Initialized system");
				updated = true;
			}
		} else {
			self.animation.tick(&self.egui_ctx);
		}

		Ok(updated)
	}
}

impl Deref for System {
	type Target = InitializedSystem;

	fn deref(&self) -> &Self::Target {
		self.inner
			.as_ref()
			.expect("System is not fully initialized")
	}
}

impl DerefMut for System {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.inner
			.as_mut()
			.expect("System is not fully initialized")
	}
}

pub struct InitializedSystem {
	pub asset: AssetManager,
	pub color: ColorManager,
	pub animation: AnimationManager,
	assets: Option<UiAssets>,
}

impl InitializedSystem {
	pub async fn new() -> Result<InitializedSystem> {
		let asset: AssetManager = AssetManager::new()
			.await
			.wrap_err("Failed to init asset manager")?;

		let config: Config = asset
			.get_data(Location::Config, "config.json")
			.await
			.wrap_err("Failed to read config")?;

		let color = ColorManager::new(config.color);
		let animation = AnimationManager::new(config.animation);
		let ui = UiAssets::new(&asset);
		let (color, animation, ui) = join!(color, animation, ui);
		let ui = ui.wrap_err("Failed to init ui")?;
		Ok(InitializedSystem {
			asset,
			color,
			animation,
			assets: Some(ui),
		})
	}
}

fn init_logging() -> Result<()> {
	CombinedLogger::init(vec![
		TermLogger::new(
			LevelFilter::Trace,
			simplelog::Config::default(),
			TerminalMode::Mixed,
			ColorChoice::Auto,
		),
		WriteLogger::new(
			LevelFilter::Info,
			simplelog::Config::default(),
			File::create("pitaya.log").wrap_err("Failed to create log file.")?,
		),
	])
	.wrap_err("Failed to init logging")?;
	Ok(())
}
