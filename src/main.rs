pub mod config;
pub mod assets;
pub mod camera;
pub mod player;
pub mod world;
pub mod items;
pub mod ui;
pub mod pokemon;
pub mod battle;
pub mod npc;
pub mod save;
pub mod audio;
pub mod dimensions;
pub mod net;

use bevy::prelude::*;
use crate::config::*;
use crate::assets::GameAssets;
use crate::assets::world_data::WorldData;
use crate::items::{ItemRegistry, Inventory};
use crate::items::bag::PlayerBag;
use crate::pokemon::data::DexRegistry;
use crate::pokemon::party::PlayerParty;
use crate::pokemon::pc::PcStorage;
use crate::npc::{NpcRegistry, DialogueState};
use crate::battle::BattleState;
use crate::dimensions::end::EndDimensionState;
use crate::dimensions::dragon::{EnderDragonBoss, DRAGON_MAX_HP};
use crate::net::MultiplayerState;
use crate::audio::AudioController;
use crate::config::settings::GameSettings;
use crate::camera::CameraPlugin;
use crate::player::PlayerPlugin;
use crate::world::WorldPlugin;
use crate::ui::UIPlugin;

fn find_assets_dir() -> std::path::PathBuf {
    let candidates = [
        std::path::PathBuf::from("assets"),
        std::path::PathBuf::from("../assets"),
        std::path::PathBuf::from("../../assets"),
        std::path::PathBuf::from("c:/Users/anbu/Downloads/POKEMC/PC-Rust/assets"),
    ];
    for p in &candidates {
        if p.join("world/world.json").exists() {
            return p.clone();
        }
    }
    std::path::PathBuf::from("assets")
}

fn main() {
    // Dynamically locate game assets
    let asset_dir = find_assets_dir();
    let world_data = WorldData::load_from_dir(&asset_dir)
        .expect("Failed to load world data from assets directory");
    let item_registry = ItemRegistry::load_from_dir(&asset_dir)
        .expect("Failed to load items data from assets directory");
    let dex_registry = DexRegistry::load_from_dir(&asset_dir)
        .expect("Failed to load dex data from assets directory");
    let npc_registry = NpcRegistry::load_from_dir(&asset_dir)
        .expect("Failed to load npc data from assets directory");

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "PokéCRAFT (Native Bevy v1.1.1)".into(),
                        resolution: (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    file_path: asset_dir.to_string_lossy().to_string(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .insert_resource(world_data)
        .insert_resource(item_registry)
        .insert_resource(dex_registry)
        .insert_resource(npc_registry)
        .insert_resource(Inventory::default())
        .insert_resource(PlayerParty::default())
        .insert_resource(PcStorage::default())
        .insert_resource(PlayerBag::default())
        .insert_resource(BattleState::default())
        .insert_resource(DialogueState::default())
        .insert_resource(EndDimensionState::default())
        .insert_resource(EnderDragonBoss::default())
        .insert_resource(MultiplayerState::default())
        .insert_resource(AudioController::default())
        .insert_resource(GameSettings::default())
        .add_systems(PreStartup, setup_assets)
        .add_plugins((CameraPlugin, WorldPlugin, PlayerPlugin, UIPlugin))
        .add_systems(Update, diagnostics_system)
        .run();
}

fn setup_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let game_assets = GameAssets::new(&asset_server, &mut texture_atlas_layouts);
    commands.insert_resource(game_assets);
}

fn diagnostics_system(
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    assets: Res<GameAssets>,
    world_data: Res<WorldData>,
    registry: Res<ItemRegistry>,
    dex: Res<DexRegistry>,
    npcs: Res<NpcRegistry>,
    inventory: Res<Inventory>,
    world_blocks: Res<crate::world::blocks::WorldBlocks>,
    chunk_tiles: Query<&crate::world::chunk::ChunkTile>,
    player_q: Query<(&crate::player::movement::Player, &Transform, &Sprite)>,
    cam_q: Query<&Transform, With<crate::camera::MainCamera>>,
    inv_state: Res<crate::ui::inventory::InventoryScreenState>,
    dragon: Res<EnderDragonBoss>,
    mut written: Local<bool>,
) {
    if !*written && time.elapsed_secs() > 1.0 {
        *written = true;
        let mut log = String::new();
        log.push_str(&format!("Elapsed: {:.2}s\n", time.elapsed_secs()));
        log.push_str(&format!("Atlas Load State: {:?}\n", asset_server.get_load_state(&assets.atlas_image)));
        log.push_str(&format!("Steve Load State: {:?}\n", asset_server.get_load_state(&assets.steve_image)));
        log.push_str(&format!("Blocks Load State: {:?}\n", asset_server.get_load_state(&assets.blocks_image)));
        log.push_str(&format!("Icons Load State: {:?}\n", asset_server.get_load_state(&assets.icons_image)));
        log.push_str(&format!("Destroy Load State: {:?}\n", asset_server.get_load_state(&assets.destroy_image)));
        log.push_str(&format!("Hotbar Load State: {:?}\n", asset_server.get_load_state(&assets.hotbar_image)));
        log.push_str(&format!("Inventory GUI Load State: {:?}\n", asset_server.get_load_state(&assets.inventory_image)));
        log.push_str(&format!("Crafting Table GUI Load State: {:?}\n", asset_server.get_load_state(&assets.crafting_table_image)));
        log.push_str(&format!("Furnace GUI Load State: {:?}\n", asset_server.get_load_state(&assets.furnace_image)));
        log.push_str(&format!("Total Items in Registry: {}\n", registry.data.items.len()));
        log.push_str(&format!("Total Blocks in Registry: {}\n", registry.data.blocks.len()));
        log.push_str(&format!("Total Recipes in Registry: {}\n", registry.data.recipes.len()));
        log.push_str(&format!("Total Pokemon Species in Dex: {}\n", dex.data.species.len()));
        log.push_str(&format!("Total Moves in Dex: {}\n", dex.data.moves.len()));
        log.push_str(&format!("Total NPCs in World: {}\n", npcs.npcs.len()));
        log.push_str(&format!("Ender Dragon Boss HP: {} / {}\n", dragon.hp, DRAGON_MAX_HP));
        log.push_str(&format!("Active Hotbar Selection: {}\n", inventory.sel));
        log.push_str(&format!("Active Held Item: {:?}\n", inventory.held()));
        log.push_str(&format!("Placed Blocks Count: {}\n", world_blocks.blocks.len()));
        log.push_str(&format!("Inventory Screen Open: {}\n", inv_state.open));
        log.push_str(&format!("Chunk Tiles Count: {}\n", chunk_tiles.iter().count()));
        if let Ok((p, tf, sp)) = player_q.single() {
            let map_name = world_data.map_at(p.tx, p.ty).map(|m| m.name.as_str()).unwrap_or("Unknown");
            let is_outdoor = world_data.is_outdoor(p.tx, p.ty);
            log.push_str(&format!("Player: ({}, {}) map: '{}' (outdoor: {}), fx: {:.2}, fy: {:.2}, dir: {:?}, frame: {}\n",
                p.tx, p.ty, map_name, is_outdoor, p.fx, p.fy, p.dir, sp.texture_atlas.as_ref().map(|a| a.index).unwrap_or(0)));
            log.push_str(&format!("Player Transform: {:?}\n", tf.translation));
        }
        if let Ok(cam_tf) = cam_q.single() {
            log.push_str(&format!("Camera Transform: {:?}\n", cam_tf.translation));
        }
        let _ = std::fs::write("engine_diag.txt", log);
    }
}
