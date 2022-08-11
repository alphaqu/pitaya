use std::fs::File;
use std::sync::Arc;
use std::time::Duration;
use egui::Vec2;
use rayon::{ThreadPool, ThreadPoolBuilder};
use simplelog::{ColorChoice, CombinedLogger, Config, LevelFilter, TerminalMode, TermLogger, WriteLogger};
use tokio::runtime::Runtime;
use tokio::time::Instant;
use settings::{LayoutSettings, Settings};
use crate::apps::app::EGuiApplication;
use crate::apps::Apps;
use crate::settings::color::ColorSettings;
use crate::settings::style::StyleSettings;

pub mod apps;
pub mod settings;
pub mod util;
pub mod ui;

pub struct System {
	pub thread_pool: Arc<ThreadPool>,
	pub runtime: Arc<Runtime>,
	pub settings: Settings,
	pub apps: Apps
}

impl System {
	pub fn new(theme: StyleSettings) -> System {
		CombinedLogger::init(
			vec![
				TermLogger::new(LevelFilter::Trace, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
				WriteLogger::new(LevelFilter::Info, Config::default(), File::create("pitaya.log").unwrap()),
			]
		).unwrap();

		System {
			thread_pool: Arc::new(ThreadPoolBuilder::new().build().unwrap()),
			runtime: Arc::new( Runtime::new().unwrap()),
			settings: Settings {
				max_widgets: 3,
				//rounding: 25.0,
				//margin: Vec2::new(25.0, 25.0),
				style: theme,
				color: ColorSettings::default(),
				layout: LayoutSettings::new(),
			},
			apps: Apps::new()
		}
	}
}
