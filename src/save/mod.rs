use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::items::types::ItemStack;

pub const NUM_SAVE_SLOTS: usize = 6;
pub const CURRENT_SAVE_VERSION: &str = "1.1.1";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SaveMetadata {
    pub slot: usize,
    pub timestamp: u64,
    pub player_name: String,
    pub badges: usize,
    pub location: String,
    pub party_count: usize,
    pub play_time_secs: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PokemonSaveData {
    pub uid: u64,
    pub species: String,
    pub name: String,
    pub level: usize,
    pub exp: i32,
    pub ivs: [i32; 6],
    pub max_hp: i32,
    pub hp: i32,
    pub moves: Vec<(String, i32, i32)>, // (id, pp, max_pp)
    pub status: Option<String>,
    pub shiny: bool,
    pub ball: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct PcBoxSaveData {
    pub name: String,
    pub slots: Vec<Option<PokemonSaveData>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct BlockSaveData {
    pub x: i32,
    pub y: i32,
    pub id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SaveState {
    pub version: String,
    pub meta: SaveMetadata,
    pub player_tx: i32,
    pub player_ty: i32,
    pub player_dir: String,
    pub dimension: String,
    pub hotbar_sel: usize,
    pub inventory: Vec<Option<ItemStack>>,
    pub armor: Vec<Option<ItemStack>>,
    pub party: Vec<PokemonSaveData>,
    pub pc_boxes: Vec<PcBoxSaveData>,
    pub money: i32,
    pub placed_blocks: Vec<BlockSaveData>,
}

pub struct SaveManager {
    base_dir: PathBuf,
}

impl SaveManager {
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Self {
        let p = base_dir.as_ref().to_path_buf();
        let _ = fs::create_dir_all(&p);
        Self { base_dir: p }
    }

    fn slot_path(&self, slot: usize) -> PathBuf {
        self.base_dir.join(format!("slot_{}.json", slot))
    }

    pub fn save_slot(&self, slot: usize, state: &SaveState) -> Result<(), Box<dyn std::error::Error>> {
        if slot >= NUM_SAVE_SLOTS {
            return Err("Invalid save slot index".into());
        }
        let json = serde_json::to_string_pretty(state)?;
        let path = self.slot_path(slot);
        fs::write(&path, json)?;
        Ok(())
    }

    pub fn load_slot(&self, slot: usize) -> Result<SaveState, Box<dyn std::error::Error>> {
        if slot >= NUM_SAVE_SLOTS {
            return Err("Invalid save slot index".into());
        }
        let path = self.slot_path(slot);
        let content = fs::read_to_string(&path)?;
        let state: SaveState = serde_json::from_str(&content)?;
        Ok(state)
    }

    pub fn delete_slot(&self, slot: usize) -> bool {
        let path = self.slot_path(slot);
        if path.exists() {
            fs::remove_file(path).is_ok()
        } else {
            false
        }
    }

    pub fn list_slots(&self) -> [Option<SaveMetadata>; NUM_SAVE_SLOTS] {
        let mut list: [Option<SaveMetadata>; NUM_SAVE_SLOTS] = std::array::from_fn(|_| None);
        for slot in 0..NUM_SAVE_SLOTS {
            if let Ok(state) = self.load_slot(slot) {
                list[slot] = Some(state.meta);
            }
        }
        list
    }
}
