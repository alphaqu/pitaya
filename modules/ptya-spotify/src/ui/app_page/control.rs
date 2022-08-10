use egui::Ui;
use librespot::core::authentication::Credentials;
use librespot::core::cache::Cache;
use librespot::core::config::SessionConfig;
use librespot::core::session::{Session, SessionError};
use librespot::playback::audio_backend;
use librespot::playback::config::{AudioFormat, PlayerConfig};
use librespot::playback::mixer::softmixer::SoftMixer;
use librespot::playback::mixer::{Mixer, MixerConfig};
use librespot::playback::player::{Player, PlayerEventChannel};

pub struct SpotifyController {

}

impl SpotifyController {


    pub fn draw(&mut self, ui: &mut Ui) {}
}
