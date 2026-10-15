use bevy::prelude::*;
use crate::config::settings::GameSettings;

#[derive(Resource, Debug, Clone)]
pub struct AudioController {
    pub current_bgm: Option<String>,
    pub bgm_volume: f32,
    pub sfx_volume: f32,
    pub cries_volume: f32,
    pub is_muted: bool,
}

impl Default for AudioController {
    fn default() -> Self {
        Self {
            current_bgm: None,
            bgm_volume: 0.7,
            sfx_volume: 0.8,
            cries_volume: 0.8,
            is_muted: false,
        }
    }
}

impl AudioController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn sync_with_settings(&mut self, settings: &GameSettings) {
        self.bgm_volume = settings.master_volume * settings.music_volume;
        self.sfx_volume = settings.master_volume * settings.sfx_volume;
        self.cries_volume = settings.master_volume * settings.cries_volume;
    }

    pub fn play_bgm(&mut self, track: &str) {
        if self.current_bgm.as_deref() != Some(track) {
            self.current_bgm = Some(track.to_string());
        }
    }

    pub fn stop_bgm(&mut self) {
        self.current_bgm = None;
    }

    pub fn bgm_track_for_map(map_name: &str) -> &'static str {
        if map_name.contains("PALLET") {
            "pallet"
        } else if map_name.contains("ROUTE1") {
            "route1"
        } else if map_name.contains("VIRIDIAN_FOREST") {
            "viridian_forest"
        } else if map_name.contains("PEWTER") || map_name.contains("VIRIDIAN") {
            "pewter"
        } else if map_name.contains("POKEMON_CENTER") || map_name.contains("MART") {
            "poke_center"
        } else if map_name.contains("GYM") {
            "gym"
        } else if map_name.contains("NETHER") {
            "nether"
        } else if map_name.contains("END") {
            "the_end"
        } else {
            "pallet"
        }
    }

    pub fn cry_path(species: &str) -> String {
        format!("cries/{}.wav", species.to_lowercase())
    }

    pub fn music_path(track: &str) -> String {
        format!("music/{}.ogg", track)
    }
}
