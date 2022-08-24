use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ColorConfig {
    pub dark_mode: bool,
    pub theme: ThemeTag,
}

impl Default for ColorConfig {
    fn default() -> Self {
        ColorConfig {
            dark_mode: true,
            theme: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub enum ThemeTag {
    #[default]
    Pitaya,
    Custom([u8; 3]),
}