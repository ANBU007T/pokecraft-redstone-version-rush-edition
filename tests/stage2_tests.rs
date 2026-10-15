use pokecraft_bevy::assets::world_data::WorldData;
use pokecraft_bevy::player::movement::Direction;
use std::path::PathBuf;

fn get_assets_path() -> PathBuf {
    let candidates = [
        PathBuf::from("assets"),
        PathBuf::from("../assets"),
        PathBuf::from("../../assets"),
        PathBuf::from("c:/Users/anbu/Downloads/POKEMC/PC-Rust/assets"),
    ];
    for p in &candidates {
        if p.join("world/world.json").exists() {
            return p.clone();
        }
    }
    PathBuf::from("assets")
}

#[test]
fn test_world_data_loaded_correctly() {
    let assets_dir = get_assets_path();
    let world_data = WorldData::load_from_dir(&assets_dir).expect("WorldData should load");

    assert_eq!(world_data.meta.w, 408);
    assert_eq!(world_data.meta.h, 1062);
    assert_eq!(world_data.meta.outdoor.w, 408);
    assert_eq!(world_data.meta.outdoor.h, 400);

    // Verify 16 doors and 701 warps
    assert_eq!(world_data.meta.doors.len(), 16);
    assert_eq!(world_data.meta.warps.len(), 701);
}

#[test]
fn test_collision_rules() {
    let assets_dir = get_assets_path();
    let world_data = WorldData::load_from_dir(&assets_dir).expect("WorldData should load");

    // Spawn point at Pallet Town (66, 268) should be passable
    assert!(!world_data.is_solid(66, 268), "Spawn tile (66, 268) must be passable");

    // The house wall or roof at (65, 267) or fence must be solid
    // Pallet Town player house is at (66, 267) doorway. Tile to left (65, 267) is house wall
    assert!(world_data.is_solid(65, 267), "House wall at (65, 267) must be solid");

    // Boundary check: negative coordinates or past width/height must be solid / blocked
    assert!(world_data.is_solid(-1, 0), "Negative x coordinate must be solid/blocked");
    assert!(world_data.is_solid(0, -1), "Negative y coordinate must be solid/blocked");
    assert!(world_data.is_solid(500, 200), "Out of bounds coordinate must be solid/blocked");

    // Zone 2 void must be solid
    assert!(world_data.is_void(0, 500), "Unused indoor space (0, 500) must be void");
    assert!(world_data.is_solid(0, 500), "Void must be solid");
}

#[test]
fn test_ledge_jump_mechanics() {
    let assets_dir = get_assets_path();
    let world_data = WorldData::load_from_dir(&assets_dir).expect("WorldData should load");

    // Route 1 ledge at (65, 210) has beh 0x3b (Down)
    let jump_dir = world_data.jump_dir(65, 210);
    assert_eq!(jump_dir, Some(Direction::Down), "Route 1 ledge at (65, 210) should jump Down");

    // Hop target (65, 211) must be passable
    assert!(!world_data.is_solid(65, 211), "Ledge landing tile (65, 211) must be passable");

    // Grass tile at (66, 268) has no jump ledge
    assert_eq!(world_data.jump_dir(66, 268), None, "Normal grass tile should not have jump ledge");
}

#[test]
fn test_pallet_town_warps() {
    let assets_dir = get_assets_path();
    let world_data = WorldData::load_from_dir(&assets_dir).expect("WorldData should load");

    // 1. Steve's House 1F Door (66, 267)
    let house_door_warp = world_data.warp_at(66, 267).expect("Warp at (66, 267) must exist");
    assert_eq!(house_door_warp.trig, "bump");
    assert_eq!(house_door_warp.dest.as_deref(), Some("MAP_PALLET_TOWN_PLAYERS_HOUSE_1F"));
    assert_eq!(house_door_warp.ax, Some(311));
    assert_eq!(house_door_warp.ay, Some(688));

    // 2. Steve's House Exit Mat (311, 688)
    let house_exit_warp = world_data.warp_at(311, 688).expect("Warp at (311, 688) must exist");
    assert_eq!(house_exit_warp.trig, "arrowS");
    assert_eq!(house_exit_warp.dest.as_deref(), Some("MAP_PALLET_TOWN"));
    assert_eq!(house_exit_warp.ax, Some(66));
    assert_eq!(house_exit_warp.ay, Some(267));

    // 3. Steve's House Stairs to 2F (317, 682)
    let stairs_to_2f = world_data.warp_at(317, 682).expect("Stairs at (317, 682) must exist");
    assert_eq!(stairs_to_2f.trig, "stairsE");
    assert_eq!(stairs_to_2f.dest.as_deref(), Some("MAP_PALLET_TOWN_PLAYERS_HOUSE_2F"));

    // 4. Oak's Lab Door (76, 273)
    let oak_door = world_data.warp_at(76, 273).expect("Oak's lab door warp must exist");
    assert_eq!(oak_door.trig, "bump");
    assert_eq!(oak_door.dest.as_deref(), Some("MAP_PALLET_TOWN_PROFESSOR_OAKS_LAB"));
    assert_eq!(oak_door.ax, Some(308));
    assert_eq!(oak_door.ay, Some(625));
}

#[test]
fn test_viridian_city_pokemon_center_and_mart() {
    let assets_dir = get_assets_path();
    let world_data = WorldData::load_from_dir(&assets_dir).expect("WorldData should load");

    // 1. Viridian Pokémon Center 1F Door (74, 206)
    let center_door = world_data.warp_at(74, 206).expect("Viridian Center door must exist");
    assert_eq!(center_door.trig, "bump");
    assert_eq!(center_door.dest.as_deref(), Some("MAP_VIRIDIAN_CITY_POKEMON_CENTER_1F"));
    assert_eq!(center_door.ax, Some(268));
    assert_eq!(center_door.ay, Some(688));

    // Verify map identity of Pokémon Center 1F
    let center_map = world_data.map_at(268, 688).expect("Map at (268, 688) must exist");
    assert_eq!(center_map.id, "MAP_VIRIDIAN_CITY_POKEMON_CENTER_1F");
    assert!(center_map.room, "Pokémon Center must be an indoor room");

    // Escalator to 2F at (262, 686)
    let center_escalator_up = world_data.warp_at(262, 686).expect("Escalator up must exist");
    assert_eq!(center_escalator_up.trig, "step");
    assert_eq!(center_escalator_up.dest.as_deref(), Some("MAP_VIRIDIAN_CITY_POKEMON_CENTER_2F"));
    assert_eq!(center_escalator_up.ax, Some(285));
    assert_eq!(center_escalator_up.ay, Some(686));

    // Escalator back to 1F at (285, 686)
    let center_escalator_down = world_data.warp_at(285, 686).expect("Escalator down must exist");
    assert_eq!(center_escalator_down.trig, "step");
    assert_eq!(center_escalator_down.dest.as_deref(), Some("MAP_VIRIDIAN_CITY_POKEMON_CENTER_1F"));
    assert_eq!(center_escalator_down.ax, Some(262));
    assert_eq!(center_escalator_down.ay, Some(686));

    // 2. Viridian Mart Door (84, 199)
    let mart_door = world_data.warp_at(84, 199).expect("Viridian Mart door must exist");
    assert_eq!(mart_door.trig, "bump");
    assert_eq!(mart_door.dest.as_deref(), Some("MAP_VIRIDIAN_CITY_MART"));
    assert_eq!(mart_door.ax, Some(373));
    assert_eq!(mart_door.ay, Some(723));

    // Verify map identity of Mart
    let mart_map = world_data.map_at(373, 723).expect("Map at (373, 723) must exist");
    assert_eq!(mart_map.id, "MAP_VIRIDIAN_CITY_MART");
    assert!(mart_map.room, "Mart must be an indoor room");

    // Exit mat inside Mart at (373, 723)
    let mart_exit = world_data.warp_at(373, 723).expect("Mart exit warp must exist");
    assert_eq!(mart_exit.trig, "arrowS");
    assert_eq!(mart_exit.dest.as_deref(), Some("MAP_VIRIDIAN_CITY"));
    assert_eq!(mart_exit.ax, Some(84));
    assert_eq!(mart_exit.ay, Some(199));
}

#[test]
fn test_outdoor_vs_indoor_boundaries() {
    let assets_dir = get_assets_path();
    let world_data = WorldData::load_from_dir(&assets_dir).expect("WorldData should load");

    // Pallet Town (66, 268) is outdoor Kanto
    assert!(world_data.is_outdoor(66, 268));

    // Viridian City (74, 206) is outdoor Kanto
    assert!(world_data.is_outdoor(74, 206));

    // Player House 1F (311, 688) is indoor (y = 688 >= 400)
    assert!(!world_data.is_outdoor(311, 688));

    // Viridian Pokémon Center 1F (268, 688) is indoor
    assert!(!world_data.is_outdoor(268, 688));

    // Viridian Mart (373, 723) is indoor
    assert!(!world_data.is_outdoor(373, 723));
}
