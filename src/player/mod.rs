pub mod movement;
pub mod mining;

use bevy::prelude::*;
use crate::assets::GameAssets;
use crate::assets::world_data::WorldData;
use self::movement::{Player, player_movement_system};
use self::mining::{MiningState, mining_and_placing_system};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MiningState>()
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (player_movement_system, mining_and_placing_system));
    }
}

fn spawn_player(
    mut commands: Commands,
    assets: Res<GameAssets>,
    world_data: Res<WorldData>,
) {
    // Find Pallet Town spawn coordinates from world.json
    let spawn_pos = world_data
        .meta
        .towns
        .iter()
        .find(|t| t.id == "MAP_PALLET_TOWN")
        .and_then(|t| t.land.as_ref())
        .map(|land| (land.x, land.y))
        .unwrap_or((66, 268));

    let player = Player::new(spawn_pos.0, spawn_pos.1);
    let start_x = player.fx * crate::config::TILE_SIZE + 8.0;
    let start_y = -player.fy * crate::config::TILE_SIZE + 1.0;

    commands.spawn((
        player,
        Sprite {
            image: assets.steve_image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.steve_layout.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(start_x, start_y, 1.0),
    ));
}
