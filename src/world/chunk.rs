use bevy::prelude::*;
use std::collections::HashSet;
use crate::config::*;
use crate::assets::GameAssets;
use crate::assets::world_data::WorldData;
use crate::player::movement::Player;

#[derive(Component)]
pub struct ChunkTile {
    pub cx: i32,
    pub cy: i32,
}

#[derive(Resource, Default)]
pub struct LoadedChunks {
    pub chunks: HashSet<(i32, i32)>,
}

pub fn update_chunks_system(
    mut commands: Commands,
    assets: Res<GameAssets>,
    world_data: Res<WorldData>,
    player_query: Query<&Player>,
    mut loaded: ResMut<LoadedChunks>,
    tile_query: Query<(Entity, &ChunkTile)>,
) {
    let Ok(player) = player_query.single() else {
        return;
    };

    let player_cx = (player.tx / CHUNK_SIZE as i32).max(0);
    let player_cy = (player.ty / CHUNK_SIZE as i32).max(0);

    let max_cx = (world_data.meta.w / CHUNK_SIZE) as i32;
    let max_cy = (world_data.meta.h / CHUNK_SIZE) as i32;

    // Load a 3x3 grid of chunks centered on the player
    let radius = 1;
    let min_x = (player_cx - radius).max(0);
    let max_x = (player_cx + radius).min(max_cx);
    let min_y = (player_cy - radius).max(0);
    let max_y = (player_cy + radius).min(max_cy);

    // Despawn tiles for chunks that are no longer in radius
    for (entity, tile) in tile_query.iter() {
        if tile.cx < min_x || tile.cx > max_x || tile.cy < min_y || tile.cy > max_y {
            commands.entity(entity).despawn();
            loaded.chunks.remove(&(tile.cx, tile.cy));
        }
    }

    // Spawn newly entered chunks
    for cy in min_y..=max_y {
        for cx in min_x..=max_x {
            if loaded.chunks.contains(&(cx, cy)) {
                continue;
            }
            loaded.chunks.insert((cx, cy));

            for ty in 0..CHUNK_SIZE as i32 {
                for tx in 0..CHUNK_SIZE as i32 {
                    let gx = cx * (CHUNK_SIZE as i32) + tx;
                    let gy = cy * (CHUNK_SIZE as i32) + ty;

                    if !world_data.is_inside(gx, gy) || world_data.is_void(gx, gy) {
                        continue;
                    }

                    let tile_id = world_data.tile_at(gx, gy);
                    let x_pos = gx as f32 * TILE_SIZE + 8.0;
                    let y_pos = -(gy as f32 * TILE_SIZE + 8.0);

                    // Base tile sprite (Z = 0.0)
                    commands.spawn((
                        ChunkTile { cx, cy },
                        Sprite {
                            image: assets.atlas_image.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: assets.atlas_layout.clone(),
                                index: tile_id as usize,
                            }),
                            ..default()
                        },
                        Transform::from_xyz(x_pos, y_pos, 0.0),
                    ));

                    // Top overhang tile sprite (Z = 2.0) if cell has a canopy or roof
                    if world_data.has_top_at(gx, gy) {
                        commands.spawn((
                            ChunkTile { cx, cy },
                            Sprite {
                                image: assets.atlas_top_image.clone(),
                                texture_atlas: Some(TextureAtlas {
                                    layout: assets.atlas_layout.clone(),
                                    index: tile_id as usize,
                                }),
                                ..default()
                            },
                            Transform::from_xyz(x_pos, y_pos, 2.0),
                        ));
                    }
                }
            }
        }
    }
}
