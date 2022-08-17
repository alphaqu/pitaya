use log::error;
use crate::{AssetComp, NotificationComp};
use crate::notification::{Notification, Severity};
use serde::{Serialize, Deserialize};
use crate::asset::Location;
use anyways::ext::AuditExt;

pub const ANIMATION_TIME: f32 = 0.25;

pub const SWIPE_SIZE: f32 = 200.0;
pub const MAX_WIDGETS: usize = 3;

pub const SPACING_SIZE: f32 = 25.0;
pub const BUTTON_SIDE_SPACING_SIZE: f32 = 50.0;
pub const ROUNDING_SIZE: f32 = 24.0;
pub const VISUAL_SIZE: f32 = 40.0;
pub const INTERACTIVE_SIZE: f32 = 90.0;

pub const WIDGET_WIDTH: f32 = 440.0;
pub const WIDGET_ADD_SIZE: f32 = 150.0;

#[derive(Serialize, Deserialize)]
pub struct Settings {
	pub current_theme: String,
}

impl Settings {
	pub async fn new(asset: &AssetComp, notification: &NotificationComp) -> Settings {
		let settings = match Self::load_save(asset).await {
			Ok(settings) => {
				settings
			}
			Err(err) => {
				error!("{err}");
				notification.spawn(Notification {
					title: "Settings reset to default.".to_string(),
					description: "The settings save was unable to load. You will get this error until you fix the config or contact support.".to_string(),
					severity: Severity::Error,
					duration: Severity::Error.get_duration()
				}).await;
				Settings::default()
			}
		};

		settings
	}

	pub async fn load_save(asset: &AssetComp) -> anyways::Result<Settings> {
		let data = asset.read_file(Location::User, "settings.json").await.wrap_err("Failed to find file")?;
		let settings: Settings = serde_json::from_slice(&data).wrap_err("Failed to parse")?;
		Ok(settings)
	}
}

impl Default for Settings {
	fn default() -> Self {
		Settings {
			current_theme: "pitaya-dark".to_string()
		}
	}
}