use std::path::PathBuf;
use pokecraft_bevy::save::{
    SaveManager, SaveState, SaveMetadata, PokemonSaveData, PcBoxSaveData, BlockSaveData,
    NUM_SAVE_SLOTS, CURRENT_SAVE_VERSION,
};
use pokecraft_bevy::config::settings::GameSettings;
use pokecraft_bevy::audio::AudioController;
use pokecraft_bevy::items::types::ItemStack;

fn temp_save_dir(test_name: &str) -> PathBuf {
    let p = std::env::temp_dir().join("pokecraft_tests").join(test_name);
    let _ = std::fs::remove_dir_all(&p);
    std::fs::create_dir_all(&p).unwrap();
    p
}

#[test]
fn test_six_slot_save_system() {
    let save_dir = temp_save_dir("save_system");
    let mgr = SaveManager::new(&save_dir);

    assert_eq!(NUM_SAVE_SLOTS, 6, "Exactly 6 manual save slots");

    // Initially all 6 slots are empty
    let slots = mgr.list_slots();
    assert!(slots.iter().all(|s| s.is_none()));

    // Construct sample save state
    let state = SaveState {
        version: CURRENT_SAVE_VERSION.to_string(),
        meta: SaveMetadata {
            slot: 0,
            timestamp: 1729000000,
            player_name: "STEVE".to_string(),
            badges: 2,
            location: "CERULEAN CITY".to_string(),
            party_count: 3,
            play_time_secs: 7200,
        },
        player_tx: 120,
        player_ty: 450,
        player_dir: "down".to_string(),
        dimension: "kanto".to_string(),
        hotbar_sel: 2,
        inventory: vec![
            Some(ItemStack::new("iron_sword", 1)),
            Some(ItemStack::new("oak_log", 64)),
        ],
        armor: vec![Some(ItemStack::new("iron_helmet", 1))],
        party: vec![PokemonSaveData {
            uid: 101,
            species: "CHARMELEON".to_string(),
            name: "CHARMELEON".to_string(),
            level: 24,
            exp: 14200,
            ivs: [28, 30, 25, 29, 27, 31],
            max_hp: 68,
            hp: 68,
            moves: vec![
                ("EMBER".to_string(), 25, 25),
                ("SLASH".to_string(), 20, 20),
            ],
            status: None,
            shiny: false,
            ball: "poke".to_string(),
        }],
        pc_boxes: vec![PcBoxSaveData {
            name: "BOX 1".to_string(),
            slots: vec![Some(PokemonSaveData {
                uid: 102,
                species: "PIDGEY".to_string(),
                name: "PIDGEY".to_string(),
                level: 8,
                exp: 512,
                ivs: [15, 12, 18, 10, 14, 20],
                max_hp: 26,
                hp: 26,
                moves: vec![("GUST".to_string(), 35, 35)],
                status: None,
                shiny: false,
                ball: "poke".to_string(),
            })],
        }],
        money: 4500,
        placed_blocks: vec![BlockSaveData {
            x: 121,
            y: 450,
            id: "crafting_table".to_string(),
        }],
    };

    // Save to slot 0
    assert!(mgr.save_slot(0, &state).is_ok());

    // Slot listing should now show Slot 0
    let slots_after = mgr.list_slots();
    assert!(slots_after[0].is_some());
    assert_eq!(slots_after[0].as_ref().unwrap().player_name, "STEVE");
    assert_eq!(slots_after[0].as_ref().unwrap().location, "CERULEAN CITY");
    assert!(slots_after[1..].iter().all(|s| s.is_none()));

    // Load from slot 0
    let loaded = mgr.load_slot(0).expect("Should load saved slot 0");
    assert_eq!(loaded.version, CURRENT_SAVE_VERSION);
    assert_eq!(loaded.player_tx, 120);
    assert_eq!(loaded.player_ty, 450);
    assert_eq!(loaded.player_dir, "down");
    assert_eq!(loaded.money, 4500);
    assert_eq!(loaded.party.len(), 1);
    assert_eq!(loaded.party[0].species, "CHARMELEON");
    assert_eq!(loaded.pc_boxes[0].slots[0].as_ref().unwrap().species, "PIDGEY");
    assert_eq!(loaded.placed_blocks[0].id, "crafting_table");

    // Delete slot 0
    assert!(mgr.delete_slot(0));
    assert!(mgr.load_slot(0).is_err());
}

#[test]
fn test_game_settings_and_controls_remapping() {
    let dir = temp_save_dir("settings");
    let settings_file = dir.join("settings.json");

    let mut settings = GameSettings::default();
    assert_eq!(settings.master_volume, 1.0);
    assert_eq!(settings.music_volume, 0.7);
    assert_eq!(settings.sfx_volume, 0.8);

    // Remap sprint from ShiftLeft to Space
    assert_eq!(settings.get_key("sprint"), Some("ShiftLeft"));
    settings.remap("sprint", "Space");
    assert_eq!(settings.get_key("sprint"), Some("Space"));

    // Save settings
    assert!(settings.save_to_file(&settings_file).is_ok());

    // Reload settings
    let loaded_settings = GameSettings::load_from_file(&settings_file).unwrap();
    assert_eq!(loaded_settings.get_key("sprint"), Some("Space"));
    assert_eq!(loaded_settings.music_volume, 0.7);
}

#[test]
fn test_audio_controller_and_tracks() {
    let mut audio = AudioController::default();
    let mut settings = GameSettings::default();
    settings.master_volume = 0.5;
    settings.music_volume = 0.8;

    audio.sync_with_settings(&settings);
    assert!((audio.bgm_volume - 0.4).abs() < 1e-4);

    // Track resolution for maps
    assert_eq!(AudioController::bgm_track_for_map("MAP_ROUTE1"), "route1");
    assert_eq!(AudioController::bgm_track_for_map("MAP_PALLET_TOWN"), "pallet");
    assert_eq!(AudioController::bgm_track_for_map("MAP_VIRIDIAN_FOREST"), "viridian_forest");
    assert_eq!(AudioController::bgm_track_for_map("MAP_POKEMON_CENTER_1F"), "poke_center");
    assert_eq!(AudioController::bgm_track_for_map("MAP_THE_END"), "the_end");

    // Paths
    assert_eq!(AudioController::cry_path("PIKACHU"), "cries/pikachu.wav");
    assert_eq!(AudioController::music_path("battle"), "music/battle.ogg");
}
