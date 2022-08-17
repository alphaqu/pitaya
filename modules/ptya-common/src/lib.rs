use crate::app::{AppComp, AppContainer, Manifest};
use crate::asset::AssetComp;
use crate::color::ColorComp;
use crate::font::load_fonts;
use crate::icon::IconComp;
use crate::notification::NotificationComp;
use crate::settings::{Settings, INTERACTIVE_SIZE, SPACING_SIZE};
use ui::widgets::progress_spinner::Spinner;
use crate::util::task::Task;
use anyways::{ext::AuditExt, Result};
use egui::panel::TopBottomSide;
use egui::{CentralPanel, Frame, Label, ProgressBar, Style, TextStyle, TopBottomPanel, Ui, Vec2, Visuals, Widget};
use epaint::text::FontDefinitions;
use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use std::fs::File;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::thread::sleep;
use std::time::{Duration, Instant};
use tokio::runtime::Runtime;
use egui::style::Spacing;
use epaint::FontFamily::Proportional;
use epaint::FontId;
use crate::color::color::{ColorState, ColorType};
use crate::ui::animation::AnimationComp;
use crate::ui::WidgetApp;

pub mod app;
pub mod asset;
pub mod color;
mod font;
pub mod icon;
pub mod notification;
pub mod settings;
pub mod ui;
pub mod util;

/// System is the core Pitaya runtime. It holds all of the important backend components.
pub struct System {
    pub gl_ctx: Rc<glium::backend::Context>,
    pub ctx: egui::Context,

    pub runtime: Arc<Runtime>,

    pub animation: AnimationComp,
    pub asset: AssetComp,
    pub notification: NotificationComp,
    pub settings: Settings,
    pub app: Arc< RwLock<AppComp>>,
    pub color: ColorComp,
    pub icon: IconComp,

    core: SystemCore,
}

impl System {
    pub fn new(ctx: egui::Context, gl_ctx: Rc<glium::backend::Context>) -> anyways::Result<System> {
        CombinedLogger::init(vec![
            TermLogger::new(
                LevelFilter::Trace,
                Config::default(),
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ),
            WriteLogger::new(
                LevelFilter::Info,
                Config::default(),
                File::create("pitaya.log").unwrap(),
            ),
        ])
        .wrap_err("Failed to init logging")?;
        ctx.set_style(Style {
            text_styles: [
                (TextStyle::Heading, FontId::new(90.0, Proportional)),
                (
                    TextStyle::Name("Heading2".into()),
                    FontId::new(75.0, Proportional),
                ),
                (
                    TextStyle::Name("Context".into()),
                    FontId::new(69.0, Proportional),
                ),
                (TextStyle::Body, FontId::new(35.0, Proportional)),
                (TextStyle::Monospace, FontId::new(42.0, Proportional)),
                (TextStyle::Button, FontId::new(35.0, Proportional)),
                (TextStyle::Small, FontId::new(30.0, Proportional)),
            ]
                .into(),
            spacing: Spacing {
                item_spacing: Vec2::new(25.0, 25.0),
                button_padding: Vec2::new(24.0, 12.0),
                interact_size: Vec2::new(INTERACTIVE_SIZE, INTERACTIVE_SIZE),
                ..Spacing::default()
            },
            visuals: Visuals {
                clip_rect_margin: SPACING_SIZE,
                ..Visuals::default()
            },
            ..Style::default()
        });

        let asset = AssetComp::new();
        let notification = NotificationComp::new();

        let runtime = Arc::new(Runtime::new().wrap_err("Failed to init async runtime")?);

        // Loading task
        let splash_text = Arc::new(RwLock::new(SystemLoadingStage::Preparing));
        let mut task = Task::new(&runtime);

        let t_asset = asset.clone();
        let t_notification = notification.clone();
        let t_splash_text = splash_text.clone();
        task.launch(async move {
            *t_splash_text.write().unwrap() = SystemLoadingStage::Settings;
            let settings = Settings::new(&t_asset, &t_notification).await;

            *t_splash_text.write().unwrap() = SystemLoadingStage::Color;
            let color = ColorComp::new(&t_asset, &settings).await?;

            *t_splash_text.write().unwrap() = SystemLoadingStage::Fonts;
            let fonts = load_fonts(&t_asset).await?;

            *t_splash_text.write().unwrap() = SystemLoadingStage::Icons;
            let icon = IconComp::new(&t_asset).await?;

            Ok(AlmostLoadedSystem {
                system: LoadedSystem {
                    settings,
                    color,
                    icon,
                },
                fonts,
            })
        })
        .unwrap();

        Ok(System {
            gl_ctx,
            ctx,
            runtime,
            animation: AnimationComp::new(),
            asset,
            notification,
            settings: Default::default(),
            app: Arc::new(RwLock::new(AppComp::init())),
            color: ColorComp::init(),
            icon: IconComp::init(),
            core: SystemCore::Loading { splash_text, task },
        })
    }

    pub fn tick(&mut self) -> Result<()> {
        CentralPanel::default().frame(Frame::none().fill(self.color.bg(0.0, ColorType::Primary, ColorState::Idle))).show(&self.ctx.clone(), |ui| {
            ui.centered_and_justified(|ui| {
                let mut progress = None;
                match &mut self.core {
                    SystemCore::Loading { splash_text, task } => {
                        let lock = splash_text.read().unwrap();
                        progress = Some(lock.get_progress());
                    }
                    SystemCore::Loaded { uptime } => {
                        progress = None;
                    }
                };
                Spinner::new(progress, self).ui(ui);
                //ProgressBar::new(progress).desired_width(INTERACTIVE_SIZE * 10.0).show_percentage().animate(true).text(text).ui(ui);
            });
        });

        let mut out = None;
        match &mut self.core {
            SystemCore::Loading { task, .. } => {
                if let Some(value) = task.poll() {
                    let system = value?;
                    self.ctx.set_fonts(system.fonts);
                    out = Some(system.system);
                }
            }
            SystemCore::Loaded { .. } => {}
        }

        if let Some(value) = out {
            let mut app = AppComp::init();
            app.load_app(AppContainer::new(&self.gl_ctx, WidgetApp::app_info(), Box::new(WidgetApp::new())));
            self.app = Arc::new(RwLock::new(app));
            self.color = value.color;
            self.icon = value.icon;
            self.settings = value.settings;
            self.core = SystemCore::Loaded {
                uptime: Instant::now(),
            };
        }

        Ok(())
    }

    pub fn is_loaded(&self) -> bool {
        matches!(&self.core, SystemCore::Loaded { .. })
    }

    // pub fn new_old(theme: StyleSettings) -> System {
    // 		CombinedLogger::init(
    // 			vec![
    // 				TermLogger::new(LevelFilter::Trace, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
    // 				WriteLogger::new(LevelFilter::Info, Config::default(), File::create("pitaya.log").unwrap()),
    // 			]
    // 		).unwrap();
    //
    // 		let mut themes: Vec<Theme> = Vec::new();
    // 		for entry in fs::read_dir("./assets/theme").unwrap().flatten() {
    // 			if let Ok(file) = File::open(entry.path()) {
    // 				if let Ok(theme) = serde_json::from_reader(file)  {
    // 					themes.push(theme);
    // 				}
    // 			}
    // 		}
    //
    // 		let option = themes.iter().find(|v| v.id == "pitaya-dark").unwrap_or_else(|| &themes[0]);
    //
    // 		System {
    // 			thread_pool: Arc::new(ThreadPoolBuilder::new().build().unwrap()),
    // 			runtime: Arc::new( Runtime::new().unwrap()),
    // 			settings: Settings {
    // 				max_widgets: 3,
    // 				//rounding: 25.0,
    // 				//margin: Vec2::new(25.0, 25.0),
    // 				color: ColorSettings::new(option).unwrap(),
    // 				themes,
    // 				style: theme,
    // 				layout: LayoutSettings::new(),
    // 			},
    // 			icon: IconComp::new(&fs::read("./assets/fonts/Icons.ttf").unwrap()),
    // 			apps: Apps::new()
    // 		}
    // 	}
}

pub enum SystemLoadingStage {
    Preparing,
    Settings,
    Applications,
    Color,
    Fonts,
    Icons,
}

impl SystemLoadingStage {
    pub fn get_text(&self) -> String {
        match self {
            SystemLoadingStage::Preparing => "Initializing".to_string(),
            SystemLoadingStage::Settings => "Initializing Settings".to_string(),
            SystemLoadingStage::Applications => "Initializing Applications".to_string(),
            SystemLoadingStage::Color => "Initializing Colors".to_string(),
            SystemLoadingStage::Fonts => "Initializing Fonts".to_string(),
            SystemLoadingStage::Icons => "Initializing Icons".to_string(),
        }
    }

    pub fn get_progress(&self) -> f32 {
        match self {
            SystemLoadingStage::Preparing => 1.0 / 7.0,
            SystemLoadingStage::Settings => 2.0 / 7.0,
            SystemLoadingStage::Applications => 3.0 / 7.0,
            SystemLoadingStage::Color => 4.0 / 7.0,
            SystemLoadingStage::Fonts => 5.0 / 7.0,
            SystemLoadingStage::Icons => 6.0 / 7.0,
        }
    }
}

#[allow(clippy::large_enum_variant)]
pub enum SystemCore {
    Loading {
        splash_text: Arc<RwLock<SystemLoadingStage>>,
        task: Task<anyways::Result<AlmostLoadedSystem>>,
    },
    Loaded {
        uptime: Instant,
    },
}

pub struct AlmostLoadedSystem {
    pub system: LoadedSystem,
    pub fonts: FontDefinitions,
}

pub struct LoadedSystem {
    pub settings: Settings,
    pub color: ColorComp,
    pub icon: IconComp,
}
