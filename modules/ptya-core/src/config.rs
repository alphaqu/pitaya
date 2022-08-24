use serde::{Deserialize, Serialize};

use crate::animation::config::AnimationConfig;
use crate::color::config::ColorConfig;


#[derive(Serialize, Deserialize, Default)]
pub struct Config {
	pub color: ColorConfig,
	pub animation: AnimationConfig,
}