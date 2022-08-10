mod ui;
mod api;

use crate::ui::app_page::AppPage;
use crate::ui::login_page::LoginPage;
use egui::{Painter, Rect, Stroke, Ui};
use ptya_common::apps::app::{EGuiApplication, AppInfo};
use ptya_common::settings::Settings;
use ptya_common::{app_icon, System};
use ui::app_page::control::SpotifyController;

pub struct Spotify {
    pub data: SpotifyAppData,
    page: SpotifyAppPage,
}

impl EGuiApplication for Spotify {
    fn tick(&mut self, ui: &mut Ui, settings: &Settings) {
        ui.painter().rect(ui.max_rect(), 25.0, settings.style.bg_0, Stroke::none());
        
        match &mut self.page {
            SpotifyAppPage::Login(page) => {
                if let Some(api) = page.draw(ui, settings) {
                    match String::from_utf8(api.credentials.auth_data.clone()) {
                        Ok(password) => {
                            self.data.login = Some(SpotifyLogin {
                                username: api.credentials.username.clone(),
                                password,
                            });
                            self.page = SpotifyAppPage::App(AppPage::new(api));
                        }
                        Err(err) => {
                            println!("Failed to read password {}", err);
                        }
                    }
                }
            }
            SpotifyAppPage::App(_) => {}
        }
    }
}

impl Spotify {
    pub fn new(system: &System, data: SpotifyAppData) -> Spotify {
        Spotify {
            page: SpotifyAppPage::Login(LoginPage::new(system, data.login.as_ref())),
            data,
        }
    }

    pub fn app_info() -> AppInfo {
        AppInfo {
            id: "spotify".to_string(),
            name: "Spotify".to_string(),
            icon: app_icon!("../icon.png"),
        }
    }
}

pub(crate) enum SpotifyAppPage {
    Login(LoginPage),
    App(AppPage),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SpotifyAppData {
    pub login: Option<SpotifyLogin>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SpotifyLogin {
    pub username: String,
    pub password: String,
}
