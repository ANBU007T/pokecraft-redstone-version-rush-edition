use bevy::prelude::*;
use std::collections::HashMap;
use crate::config::*;
use crate::assets::GameAssets;
use crate::assets::world_data::WorldData;
use crate::items::ItemRegistry;
use super::furnace::FurnaceState;
use super::campfire::CampfireState;

#[derive(Debug, Clone)]
pub struct PlacedBlock {
    pub id: String,
    pub walk: bool,
    pub flush: bool,
    pub furnace: Option<FurnaceState>,
    pub campfire: Option<CampfireState>,
}

impl PlacedBlock {
    pub fn new(id: impl Into<String>, walk: bool, flush: bool) -> Self {
        let id_str = id.into();
        let furnace = if id_str == "furnace" {
            Some(FurnaceState::new())
        } else {
            None
        };
        let campfire = if id_str == "campfire" {
            Some(CampfireState::new())
        } else {
            None
        };
        Self {
            id: id_str,
            walk,
            flush,
            furnace,
            campfire,
        }
    }
}

#[derive(Resource, Default)]
pub struct WorldBlocks {
    pub blocks: HashMap<(i32, i32), PlacedBlock>,
}

#[derive(Component)]
pub struct BlockTile {
    pub x: i32,
    pub y: i32,
}

impl WorldBlocks {
    pub fn get(&self, x: i32, y: i32) -> Option<&PlacedBlock> {
        self.blocks.get(&(x, y))
    }

    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut PlacedBlock> {
        self.blocks.get_mut(&(x, y))
    }

    pub fn set(&mut self, x: i32, y: i32, block: PlacedBlock) {
        self.blocks.insert((x, y), block);
    }

    pub fn remove(&mut self, x: i32, y: i32) -> Option<PlacedBlock> {
        self.blocks.remove(&(x, y))
    }

    pub fn is_solid(&self, x: i32, y: i32) -> bool {
        if let Some(b) = self.get(x, y) {
            !b.walk && !b.flush
        } else {
            false
        }
    }
}

pub fn can_place_block(
    world_data: &WorldData,
    world_blocks: &WorldBlocks,
    player_tx: i32,
    player_ty: i32,
    x: i32,
    y: i32,
) -> bool {
    // Cannot place on the player's current tile
    if player_tx == x && player_ty == y {
        return false;
    }
    // Must be inside the world bounds and not void
    if !world_data.is_inside(x, y) || world_data.is_void(x, y) {
        return false;
    }
    // Cannot place on warps or doors
    if world_data.warp_at(x, y).is_some() || world_data.door_at(x, y).is_some() {
        return false;
    }
    // Cannot place on existing placed solid block
    if world_blocks.is_solid(x, y) {
        return false;
    }
    // Base tile must be passable ground
    if world_data.is_solid(x, y) {
        return false;
    }
    true
}

pub fn update_block_sprites_system(
    mut commands: Commands,
    assets: Res<GameAssets>,
    registry: Res<ItemRegistry>,
    world_blocks: Res<WorldBlocks>,
    block_tiles_query: Query<(Entity, &BlockTile)>,
) {
    if !world_blocks.is_changed() {
        return;
    }

    // Build map of currently spawned block entities
    let mut spawned: HashMap<(i32, i32), Entity> = HashMap::new();
    for (entity, tile) in block_tiles_query.iter() {
        if !world_blocks.blocks.contains_key(&(tile.x, tile.y)) {
            commands.entity(entity).despawn();
        } else {
            spawned.insert((tile.x, tile.y), entity);
        }
    }

    // Spawn new placed blocks
    for (&(x, y), block) in &world_blocks.blocks {
        if spawned.contains_key(&(x, y)) {
            continue;
        }

        let look_idx = registry.block_look(&block.id).unwrap_or(0);
        let x_pos = x as f32 * TILE_SIZE + 8.0;
        let y_pos = -(y as f32 * TILE_SIZE + 8.0) + 4.0; // 24px tall sprite offset

        commands.spawn((
            BlockTile { x, y },
            Sprite {
                image: assets.blocks_image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: assets.blocks_layout.clone(),
                    index: look_idx,
                }),
                ..default()
            },
            Transform::from_xyz(x_pos, y_pos, 0.8), // Between ground (0.0) and Steve (1.0)
        ));
    }
}

pub fn tick_containers_system(
    time: Res<Time>,
    registry: Res<ItemRegistry>,
    mut world_blocks: ResMut<WorldBlocks>,
    mut timer: Local<Timer>,
) {
    if timer.duration().is_zero() {
        *timer = Timer::from_seconds(0.05, TimerMode::Repeating); // 20 Hz
    }
    timer.tick(time.delta());
    if !timer.just_finished() {
        return;
    }

    for block in world_blocks.blocks.values_mut() {
        if let Some(f) = &mut block.furnace {
            f.tick(&registry);
        }
        if let Some(c) = &mut block.campfire {
            let _popped = c.tick(&registry);
        }
    }
}
