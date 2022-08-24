use crate::api::SpotifyApi;
use crate::SpotifyController;

pub mod control;
mod home;

pub(crate) struct AppPage {
	api: SpotifyApi,
}

impl AppPage {
	pub fn new(api: SpotifyApi) -> AppPage {
		AppPage { api }
	}
}

pub enum AppPageView {
	Albums(),
}
