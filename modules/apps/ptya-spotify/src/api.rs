use librespot::core::authentication::Credentials;
use librespot::core::cache::Cache;
use librespot::core::config::SessionConfig;
use librespot::core::session::{Session, SessionError};
use librespot::playback::audio_backend;
use librespot::playback::config::{AudioFormat, PlayerConfig};
use librespot::playback::mixer::softmixer::SoftMixer;
use librespot::playback::mixer::{Mixer, MixerConfig};
use librespot::playback::player::{Player, PlayerEventChannel};
use rspotify::{AuthCodeSpotify, Token};
use std::string::FromUtf8Error;
use std::time::Duration;
use tokio::time::Instant;

pub struct SpotifyApi {
	web_api: AuthCodeSpotify,
	token: Option<Token>,
	last_refresh: Instant,

	session: Session,
	pub credentials: Credentials,

	audio_format: AudioFormat,
	player: Player,
	player_events: PlayerEventChannel,
	mixer: SoftMixer,
}

impl SpotifyApi {
	pub async fn new(credentials: Credentials) -> Result<SpotifyApi, SessionError> {
		let cache = Cache::new(
			None,
			Some("./data/spotify/"),
			Some("./data/spotify/audio/"),
			None,
		)?;
		let session_config = SessionConfig::default();
		let player_config = PlayerConfig::default();
		let audio_format = AudioFormat::default();

		let (session, credentials) =
			Session::connect(session_config, credentials, Some(cache), false).await?;

		let mixer = SoftMixer::open(MixerConfig::default());
		let backend = audio_backend::find(None).unwrap();
		let (player, player_events) = Player::new(
			player_config,
			session.clone(),
			mixer.get_soft_volume(),
			move || backend(None, audio_format),
		);

		Ok(SpotifyApi {
			web_api: Default::default(),
			token: None,
			last_refresh: Instant::now(),
			session,
			credentials,
			audio_format,
			player,
			player_events,
			mixer,
		})
	}

	pub async fn get_web_api(&mut self) -> Result<&mut AuthCodeSpotify, UpdateTokenError> {
		self.check_token().await?;
		Ok(&mut self.web_api)
	}

	async fn update_token(&mut self) -> Result<(), UpdateTokenError> {
		let client_id = "d420a117a32841c2b3474932e49fb54b";
		let scopes = "user-read-private,playlist-read-private,playlist-read-collaborative,playlist-modify-public,playlist-modify-private,user-follow-modify,user-follow-read,user-library-read,user-library-modify,user-top-read,user-read-recently-played";
		let url = format!(
			"hm://keymaster/token/authenticated?client_id={}&scope={}",
			client_id, scopes
		);
		let response = self
			.session
			.mercury()
			.get(url)
			.await
			.ok()
			.ok_or(UpdateTokenError::Mercury)?;

		let payload = response
			.payload
			.first()
			.ok_or(UpdateTokenError::EmptyPayload)?;
		let data = String::from_utf8(payload.clone())?;
		let token: Token = serde_json::from_str(&data)?;
		self.token = Some(token.clone());
		*self.web_api.token.lock().await.unwrap() = Some(token);
		Ok(())
	}

	async fn check_token(&mut self) -> Result<(), UpdateTokenError> {
		let mut update = false;
		if let Some(token) = &self.token {
			let update_padding = Duration::from_secs(60 * 5);
			let refresh_at = self.last_refresh
				+ token
					.expires_in
					.to_std()
					.ok()
					.ok_or(UpdateTokenError::Unknown)?
					.saturating_sub(update_padding);
			// if its behind it will be default, meaning we are checking if we have gone past the refresh mark
			if refresh_at.elapsed() != Duration::default() {
				update = true;
			}
		} else {
			update = true;
		}

		if update {
			self.update_token().await?;
		}

		Ok(())
	}
}

#[derive(thiserror::Error, Debug)]
pub enum UpdateTokenError {
	#[error("Unknown")]
	Unknown,
	#[error("Request error")]
	Mercury,
	#[error("Payload Empty")]
	EmptyPayload,
	#[error("Payload not a UTF-8 string `{0}`")]
	FromString(#[from] FromUtf8Error),
	#[error("Failed to parse token `{0}`")]
	SerdeParse(#[from] serde_json::Error),
}
