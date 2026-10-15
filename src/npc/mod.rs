use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::Deserialize;
use bevy::prelude::*;
use crate::pokemon::data::{Pokemon, heal_mon};

#[derive(Debug, Clone, Deserialize)]
pub struct NpcDef {
    pub x: i32,
    pub y: i32,
    pub map: String,
    pub gfx: String,
    pub face: String,
    #[serde(rename = "move")]
    pub move_type: String,
    pub hidden: bool,
    pub role: String,
    pub mart: Option<String>,
    #[serde(default)]
    pub text: Vec<String>,
    pub script: Option<String>,
}

#[derive(Resource, Default)]
pub struct NpcRegistry {
    pub npcs: Vec<NpcDef>,
    pub by_pos: HashMap<(i32, i32), usize>,
}

impl NpcRegistry {
    pub fn load_from_dir<P: AsRef<Path>>(base_path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let json_path = base_path.as_ref().join("world/world.json");
        let content = fs::read_to_string(&json_path)?;
        let val: serde_json::Value = serde_json::from_str(&content)?;
        let npcs_val = val.get("npcs").ok_or("Missing npcs in world.json")?;
        let npcs: Vec<NpcDef> = serde_json::from_value(npcs_val.clone())?;

        let mut by_pos = HashMap::new();
        for (i, npc) in npcs.iter().enumerate() {
            by_pos.insert((npc.x, npc.y), i);
        }

        Ok(Self { npcs, by_pos })
    }

    pub fn get_at(&self, x: i32, y: i32) -> Option<&NpcDef> {
        self.by_pos.get(&(x, y)).map(|&idx| &self.npcs[idx])
    }
}

#[derive(Resource, Default)]
pub struct DialogueState {
    pub active: bool,
    pub speaker: Option<String>,
    pub pages: Vec<String>,
    pub current_page: usize,
}

impl DialogueState {
    pub fn start(&mut self, speaker: Option<String>, pages: Vec<String>) {
        if pages.is_empty() {
            return;
        }
        self.active = true;
        self.speaker = speaker;
        self.pages = pages;
        self.current_page = 0;
    }

    pub fn advance(&mut self) -> bool {
        if self.current_page + 1 < self.pages.len() {
            self.current_page += 1;
            true
        } else {
            self.close();
            false
        }
    }

    pub fn close(&mut self) {
        self.active = false;
        self.speaker = None;
        self.pages.clear();
        self.current_page = 0;
    }

    pub fn current_text(&self) -> Option<&str> {
        if self.active && self.current_page < self.pages.len() {
            Some(&self.pages[self.current_page])
        } else {
            None
        }
    }
}

/// Handles NPC interaction when player presses action button (Z, Space, Enter)
pub fn interact_with_npc(
    npc: &NpcDef,
    dialogue: &mut DialogueState,
    party: &mut [Pokemon],
) {
    match npc.role.as_str() {
        "nurse" => {
            // Nurse Joy restores full party HP, PP, and cures status
            for mon in party.iter_mut() {
                heal_mon(mon);
            }
            dialogue.start(
                Some("NURSE JOY".to_string()),
                vec![
                    "Hello, and welcome to the\nPOKéMON CENTER.".to_string(),
                    "We've restored your POKéMON\nto full health!".to_string(),
                    "We hope to see you again!".to_string(),
                ],
            );
        }
        "mom" => {
            for mon in party.iter_mut() {
                heal_mon(mon);
            }
            dialogue.start(
                Some("MOM".to_string()),
                vec![
                    "MOM: Welcome home, honey!\nYou look tired.".to_string(),
                    "You should take a quick rest.".to_string(),
                    "There, now you and your POKéMON\nare full of energy!".to_string(),
                ],
            );
        }
        "clerk" => {
            dialogue.start(
                Some("CLERK".to_string()),
                vec![
                    "Welcome to the POKé MART!\nHow may I serve you today?".to_string(),
                ],
            );
        }
        _ => {
            // Standard NPC dialogue from world.json
            if !npc.text.is_empty() {
                dialogue.start(None, npc.text.clone());
            } else {
                dialogue.start(None, vec!["...".to_string()]);
            }
        }
    }
}
