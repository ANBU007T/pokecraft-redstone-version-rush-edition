use bevy::prelude::*;
use crate::config::*;
use crate::assets::GameAssets;
use crate::assets::world_data::WorldData;
use crate::items::{Inventory, ItemRegistry, break_info, BlockSpec};
use crate::player::movement::Player;
use crate::world::blocks::{WorldBlocks, PlacedBlock, can_place_block};

#[derive(Resource, Default)]
pub struct MiningState {
    pub active: bool,
    pub target: (i32, i32),
    pub progress: f32,
    pub total_ticks: f32,
    pub is_placed_block: bool,
}

#[derive(Component)]
pub struct MiningCrack;

pub fn mining_and_placing_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut inventory: ResMut<Inventory>,
    mut inventory_ui: ResMut<crate::ui::inventory::InventoryScreenState>,
    world_data: Res<WorldData>,
    mut world_blocks: ResMut<WorldBlocks>,
    registry: Res<ItemRegistry>,
    mut mining: ResMut<MiningState>,
    player_query: Query<&Player>,
    mut crack_query: Query<(Entity, &mut Transform, &mut Sprite), With<MiningCrack>>,
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    let dt = time.delta_secs();
    let Ok(player) = player_query.single() else {
        return;
    };

    let (dx, dy) = player.dir.delta();
    let target_x = player.tx + dx;
    let target_y = player.ty + dy;

    // ----------------------------------------------------
    // 1. Container Interaction & Block Placement: KeyX or Right Mouse Button
    // ----------------------------------------------------
    let place_pressed = keyboard_input.just_pressed(KeyCode::KeyX)
        || mouse_input.just_pressed(MouseButton::Right);

    if place_pressed {
        // First check if interacting with an existing placed container block
        if let Some(placed) = world_blocks.get_mut(target_x, target_y) {
            if placed.id == "crafting_table" {
                inventory_ui.open_crafting_table();
                inventory_ui.update_crafting_result(&registry);
                return;
            } else if placed.id == "furnace" {
                inventory_ui.open_furnace((target_x, target_y));
                return;
            } else if placed.id == "campfire" {
                if let Some(campfire) = &mut placed.campfire {
                    if let Some(held) = inventory.held() {
                        if registry.campfire_of(&held.id).is_some() {
                            if campfire.add_item(&held.id, &registry) {
                                inventory.use_held();
                                return;
                            }
                        } else if held.id == "water_bucket" {
                            if campfire.extinguish() {
                                if let Some(s) = inventory.held_mut() {
                                    *s = crate::items::ItemStack::new("bucket", 1);
                                }
                                return;
                            }
                        } else if held.id == "flint_and_steel" {
                            if campfire.ignite() {
                                inventory.wear_held(1, 64);
                                return;
                            }
                        }
                    }
                }
            }
        }

        // Otherwise place held block
        if let Some(held) = inventory.held() {
            if let Some(it) = registry.get(&held.id) {
                if let Some(spec) = &it.block {
                    if can_place_block(&world_data, &world_blocks, player.tx, player.ty, target_x, target_y) {
                        let is_walk = spec.walk.unwrap_or(false);
                        let is_flush = spec.flush.unwrap_or(false);
                        world_blocks.set(
                            target_x,
                            target_y,
                            PlacedBlock::new(held.id.clone(), is_walk, is_flush),
                        );
                        inventory.use_held();
                    }
                }
            }
        }
    }

    // ----------------------------------------------------
    // 2. Mining / Breaking: KeyZ or Left Mouse Button (held)
    // ----------------------------------------------------
    let mine_held = keyboard_input.pressed(KeyCode::KeyZ)
        || mouse_input.pressed(MouseButton::Left);

    if !mine_held {
        mining.active = false;
        mining.progress = 0.0;
        // Despawn crack overlay
        for (entity, _, _) in crack_query.iter() {
            commands.entity(entity).despawn();
        }
        return;
    }

    // Check what target is in front of Steve
    let held_tool = inventory.held().and_then(|s| {
        registry.get(&s.id).and_then(|it| it.tool.as_ref())
    });

    let target_key = (target_x, target_y);
    if !mining.active || mining.target != target_key {
        // Find block spec for the target
        if let Some(b) = world_blocks.get(target_x, target_y) {
            let spec = registry.get(&b.id).and_then(|it| it.block.clone()).unwrap_or(BlockSpec {
                look: None,
                hard: 1.5,
                tool: None,
                need: None,
                sound: Some("stone".into()),
                drop: None,
                flush: None,
                walk: None,
                plant: None,
            });
            let (ticks, _) = break_info(&spec, held_tool);
            mining.active = true;
            mining.target = target_key;
            mining.progress = 0.0;
            mining.total_ticks = ticks;
            mining.is_placed_block = true;
        } else if world_data.is_inside(target_x, target_y) && !world_data.is_void(target_x, target_y) && world_data.is_solid(target_x, target_y) {
            // Overworld solid object (tree or rock)
            let spec = BlockSpec {
                look: None,
                hard: 2.0,
                tool: Some("axe".into()),
                need: None,
                sound: Some("wood".into()),
                drop: Some("oak_log".into()),
                flush: None,
                walk: None,
                plant: None,
            };
            let (ticks, _) = break_info(&spec, held_tool);
            mining.active = true;
            mining.target = target_key;
            mining.progress = 0.0;
            mining.total_ticks = ticks;
            mining.is_placed_block = false;
        } else {
            mining.active = false;
            mining.progress = 0.0;
            for (entity, _, _) in crack_query.iter() {
                commands.entity(entity).despawn();
            }
            return;
        }
    }

    // Progress mining
    mining.progress += (dt * 20.0) / mining.total_ticks.max(1.0);

    let stage = ((mining.progress * 10.0).floor() as usize).min(9);
    let x_pos = target_x as f32 * TILE_SIZE + 8.0;
    let y_pos = -(target_y as f32 * TILE_SIZE + 8.0);

    // Update or spawn destruction crack sprite
    if let Ok((_, mut tf, mut sprite)) = crack_query.single_mut() {
        tf.translation.x = x_pos;
        tf.translation.y = y_pos;
        tf.translation.z = 1.5;
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            atlas.index = stage;
        }
    } else {
        commands.spawn((
            MiningCrack,
            Sprite {
                image: assets.destroy_image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: assets.destroy_layout.clone(),
                    index: stage,
                }),
                ..default()
            },
            Transform::from_xyz(x_pos, y_pos, 1.5),
        ));
    }

    // Finish mining
    if mining.progress >= 1.0 {
        if mining.is_placed_block {
            if let Some(removed) = world_blocks.remove(target_x, target_y) {
                let drop_id = registry
                    .get(&removed.id)
                    .and_then(|it| it.block.as_ref())
                    .and_then(|b| b.drop.clone())
                    .unwrap_or(removed.id);
                inventory.add(&drop_id, 1, 64);
            }
        } else {
            // Overworld tree harvested
            inventory.add("oak_log", 1, 64);
        }

        // Wear held tool
        if let Some(s) = inventory.held() {
            if let Some(it) = registry.get(&s.id) {
                if let Some(t) = &it.tool {
                    let dur = t.dur;
                    inventory.wear_held(1, dur);
                }
            }
        }

        mining.active = false;
        mining.progress = 0.0;
        for (entity, _, _) in crack_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}
