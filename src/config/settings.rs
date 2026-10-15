use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use bevy::prelude::*;

#[derive(Resource, Serialize, Deserialize, Clone, Debug)]
pub struct GameSettings {
    pub master_volume: f32,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub cries_volume: f32,
    pub toggle_sprint: bool,
    pub text_speed: u8, // 1: Fast, 2: Mid, 3: Slow
    pub key_bindings: HashMap<String, String>,
}

impl Default for GameSettings {
    fn default() -> Self {
        let mut key_bindings = HashMap::new();
        key_bindings.insert("up".to_string(), "KeyW".to_string());
        key_bindings.insert("down".to_string(), "KeyS".to_string());
        key_bindings.insert("left".to_string(), "KeyA".to_string());
        key_bindings.insert("right".to_string(), "KeyD".to_string());
        key_bindings.insert("action_a".to_string(), "KeyZ".to_string());
        key_bindings.insert("action_b".to_string(), "KeyX".to_string());
        key_bindings.insert("inventory".to_string(), "KeyE".to_string());
        key_bindings.insert("menu".to_string(), "Enter".to_string());
        key_bindings.insert("sprint".to_string(), "ShiftLeft".to_string());

        Self {
            master_volume: 1.0,
            music_volume: 0.7,
            sfx_volume: 0.8,
            cries_volume: 0.8,
            toggle_sprint: false,
            text_speed: 1,
            key_bindings,
        }
    }
}

impl GameSettings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn remap(&mut self, action: &str, key_name: &str) {
        self.key_bindings.insert(action.to_string(), key_name.to_string());
    }

    pub fn get_key(&self, action: &str) -> Option<&str> {
        self.key_bindings.get(action).map(|s| s.as_str())
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let settings: Self = serde_json::from_str(&content)?;
        Ok(settings)
    }
}
