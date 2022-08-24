use egui::{Spinner, Ui, Widget};

use librespot::core::authentication::Credentials;
use librespot::core::session::SessionError;
use ptya_common::settings::Settings;

use crate::api::SpotifyApi;
use crate::SpotifyLogin;
use ptya_common::ui::button::Button;
use ptya_common::ui::field::Field;
use ptya_common::util::task::{Task, TaskAlreadyInProgress};
use ptya_common::System;

pub(crate) struct LoginPage {
	pub username: String,
	pub password: String,

	pub login: Task<Result<SpotifyApi, SessionError>>,
	pub error_message: Option<String>,
}

impl LoginPage {
	pub fn new(system: &System, login: Option<&SpotifyLogin>) -> LoginPage {
		let login_task = Task::new(system);

		let (username, password) = login
			.map(|v| (v.username.clone(), v.password.clone()))
			.unwrap_or_default();

		let mut page = LoginPage {
			username,
			password,
			login: login_task,
			error_message: None,
		};

		if login.is_some() {
			page.login().unwrap();
		}
		page
	}

	pub fn login(&mut self) -> Result<(), TaskAlreadyInProgress> {
		let credentials = Credentials::with_password(&self.username, &self.password);
		self.login.launch_future(SpotifyApi::new(credentials))
	}

	pub fn draw(&mut self, ui: &mut Ui, settings: &Settings) -> Option<SpotifyApi> {
		if self.login.in_progress() {
			ui.vertical_centered(|ui| {
				ui.add_space((ui.max_rect().height() / 2.0) - 25.0);
				Spinner::new().size(100.0).ui(ui);
			});

			if let Some(value) = self.login.poll() {
				match value {
					Ok(value) => {
						return Some(value);
					}
					Err(err) => {
						self.error_message = Some(format!("{err}"));
					}
				}
			}
		} else {
			ui.vertical_centered(|ui| {
				ui.heading("Spotify Login");
				ui.separator();
				if let Some(err) = &self.error_message {
					ui.label(err);
				}
				ui.add_space(50.0);

				Field::new(&mut self.username, settings)
					.hint("Username")
					.ui(ui);
				Field::new(&mut self.password, settings)
					.password(true)
					.hint("Password")
					.ui(ui);
				ui.add_space(50.0);

				if Button::new("Login", settings).ui(ui).clicked() {
					if self.username.is_empty() {
						self.error_message = Some("Username empty".to_string());
					} else if self.password.is_empty() {
						self.error_message = Some("Password empty".to_string());
					} else {
						println!("Starting login");
						if self.login().is_err() {
							println!("fuck");
						}
					}
				}
			});
		}
		None
	}
}
