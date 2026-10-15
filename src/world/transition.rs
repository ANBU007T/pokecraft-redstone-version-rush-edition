use bevy::prelude::*;
use crate::config::*;
use crate::assets::GameAssets;
use crate::assets::world_data::{WorldData, WarpDef};
use crate::camera::MainCamera;
use crate::player::movement::{Player, Direction};

#[derive(Debug, Clone)]
pub enum WarpPhase {
    /// Door opens in front of player
    Open { timer: f32, warp: WarpDef, anim: usize },
    /// Player walks up into doorway
    WalkIn { timer: f32, warp: WarpDef, anim: usize },
    /// Screen fades to black
    FadeOut { timer: f32, warp: WarpDef },
    /// Screen fades from black to clear, destination door open
    FadeIn { timer: f32, warp: WarpDef, door_anim: Option<(i32, i32, usize)> },
    /// Player walks forward out of doorway, destination door closes
    WalkOut { timer: f32, warp: WarpDef, door_anim: Option<(i32, i32, usize)> },
}

#[derive(Resource, Default)]
pub struct WarpState {
    pub current: Option<WarpPhase>,
}

#[derive(Resource, Default)]
pub struct ScreenFade {
    pub alpha: f32,
}

#[derive(Component)]
pub struct DoorSprite;

#[derive(Component)]
pub struct ScreenFadeOverlay;

pub fn setup_fade_overlay(mut commands: Commands) {
    commands.spawn((
        ScreenFadeOverlay,
        Sprite {
            color: Color::srgba(0.0, 0.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(VW * 2.0, VH * 2.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 99.0),
    ));
}

pub fn fade_overlay_system(
    fade: Res<ScreenFade>,
    cam_query: Query<&Transform, With<MainCamera>>,
    mut overlay_query: Query<(&mut Transform, &mut Sprite), (With<ScreenFadeOverlay>, Without<MainCamera>)>,
) {
    if let Ok(cam_tf) = cam_query.single() {
        if let Ok((mut overlay_tf, mut sprite)) = overlay_query.single_mut() {
            overlay_tf.translation.x = cam_tf.translation.x;
            overlay_tf.translation.y = cam_tf.translation.y;
            overlay_tf.translation.z = 99.0;
            sprite.color = Color::srgba(0.0, 0.0, 0.0, fade.alpha.clamp(0.0, 1.0));
        }
    }
}

pub fn warp_transition_system(
    time: Res<Time>,
    mut warp_state: ResMut<WarpState>,
    mut screen_fade: ResMut<ScreenFade>,
    world_data: Res<WorldData>,
    mut player_query: Query<&mut Player>,
    mut door_query: Query<(Entity, &mut Transform, &mut Sprite), With<DoorSprite>>,
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    let dt = time.delta_secs();
    let Ok(mut player) = player_query.single_mut() else { return; };

    let Some(phase) = warp_state.current.clone() else {
        screen_fade.alpha = 0.0;
        for (entity, _, _) in door_query.iter() {
            commands.entity(entity).despawn();
        }
        return;
    };

    match phase {
        WarpPhase::Open { mut timer, warp, anim } => {
            timer += dt;
            let frame = ((timer / 0.08).floor() as usize).min(2);
            update_or_spawn_door(&mut commands, &assets, &mut door_query, warp.x, warp.y, anim, frame);

            if timer >= 0.26 {
                player.dir = Direction::Up;
                player.from_x = player.tx;
                player.from_y = player.ty;
                player.tx = warp.x;
                player.ty = warp.y;
                player.moving = true;
                player.speed = WALK_TIME;
                player.prog = 0.0;
                warp_state.current = Some(WarpPhase::WalkIn { timer: 0.0, warp, anim });
            } else {
                warp_state.current = Some(WarpPhase::Open { timer, warp, anim });
            }
        }
        WarpPhase::WalkIn { mut timer, warp, anim } => {
            timer += dt;
            update_or_spawn_door(&mut commands, &assets, &mut door_query, warp.x, warp.y, anim, 2);

            player.prog += dt / player.speed;
            if player.prog >= 1.0 {
                player.prog = 0.0;
                player.moving = false;
                player.fx = player.tx as f32;
                player.fy = player.ty as f32;
                player.from_x = player.tx;
                player.from_y = player.ty;

                for (entity, _, _) in door_query.iter() {
                    commands.entity(entity).despawn();
                }

                warp_state.current = Some(WarpPhase::FadeOut { timer: 0.0, warp });
            } else {
                player.fx = player.from_x as f32 + (player.tx - player.from_x) as f32 * player.prog;
                player.fy = player.from_y as f32 + (player.ty - player.from_y) as f32 * player.prog;
                warp_state.current = Some(WarpPhase::WalkIn { timer, warp, anim });
            }
        }
        WarpPhase::FadeOut { mut timer, warp } => {
            timer += dt;
            screen_fade.alpha = (timer / 0.27).min(1.0);

            if timer >= 0.28 {
                let (Some(dest_x), Some(dest_y)) = (warp.ax, warp.ay) else {
                    warp_state.current = None;
                    screen_fade.alpha = 0.0;
                    return;
                };

                player.tx = dest_x;
                player.ty = dest_y;
                player.from_x = dest_x;
                player.from_y = dest_y;
                player.fx = dest_x as f32;
                player.fy = dest_y as f32;
                player.moving = false;
                player.prog = 0.0;
                if let Some(ref adir_str) = warp.adir {
                    if let Some(d) = Direction::from_str_name(adir_str) {
                        player.dir = d;
                    }
                }

                let dest_door_anim = if let Some(dest_door) = world_data.door_at(dest_x, dest_y) {
                    Some((dest_x, dest_y, dest_door.anim))
                } else if let Some(sw) = world_data.warp_at(dest_x, dest_y) {
                    if sw.trig == "bump" {
                        sw.door.map(|anim| (dest_x, dest_y, anim))
                    } else {
                        None
                    }
                } else {
                    None
                };

                if let Some((dx, dy, anim)) = dest_door_anim {
                    update_or_spawn_door(&mut commands, &assets, &mut door_query, dx, dy, anim, 2);
                }

                warp_state.current = Some(WarpPhase::FadeIn { timer: 0.0, warp, door_anim: dest_door_anim });
            } else {
                warp_state.current = Some(WarpPhase::FadeOut { timer, warp });
            }
        }
        WarpPhase::FadeIn { mut timer, warp, door_anim } => {
            timer += dt;
            screen_fade.alpha = (1.0 - timer / 0.27).max(0.0);

            if let Some((dx, dy, anim)) = door_anim {
                update_or_spawn_door(&mut commands, &assets, &mut door_query, dx, dy, anim, 2);
            }

            if timer >= 0.28 {
                screen_fade.alpha = 0.0;
                if warp.amove.as_deref() == Some("walk") {
                    let (dx, dy) = player.dir.delta();
                    let nx = player.tx + dx;
                    let ny = player.ty + dy;
                    if !world_data.is_solid(nx, ny) {
                        player.from_x = player.tx;
                        player.from_y = player.ty;
                        player.tx = nx;
                        player.ty = ny;
                        player.moving = true;
                        player.speed = WALK_TIME;
                        player.prog = 0.0;
                        warp_state.current = Some(WarpPhase::WalkOut { timer: 0.0, warp, door_anim });
                        return;
                    }
                }
                for (entity, _, _) in door_query.iter() {
                    commands.entity(entity).despawn();
                }
                warp_state.current = None;
            } else {
                warp_state.current = Some(WarpPhase::FadeIn { timer, warp, door_anim });
            }
        }
        WarpPhase::WalkOut { mut timer, warp, door_anim } => {
            timer += dt;
            if let Some((dx, dy, anim)) = door_anim {
                let frame = ((2.0 - timer / 0.1).floor() as usize).min(2);
                update_or_spawn_door(&mut commands, &assets, &mut door_query, dx, dy, anim, frame);
            }

            player.prog += dt / player.speed;
            if player.prog >= 1.0 {
                player.prog = 0.0;
                player.moving = false;
                player.fx = player.tx as f32;
                player.fy = player.ty as f32;
                player.from_x = player.tx;
                player.from_y = player.ty;

                for (entity, _, _) in door_query.iter() {
                    commands.entity(entity).despawn();
                }
                warp_state.current = None;
            } else {
                player.fx = player.from_x as f32 + (player.tx - player.from_x) as f32 * player.prog;
                player.fy = player.from_y as f32 + (player.ty - player.from_y) as f32 * player.prog;
                warp_state.current = Some(WarpPhase::WalkOut { timer, warp, door_anim });
            }
        }
    }
}

fn update_or_spawn_door(
    commands: &mut Commands,
    assets: &GameAssets,
    door_query: &mut Query<(Entity, &mut Transform, &mut Sprite), With<DoorSprite>>,
    x: i32,
    y: i32,
    anim: usize,
    frame: usize,
) {
    let index = (frame * 16 + anim).min(47);
    let x_pos = x as f32 * TILE_SIZE + 8.0;
    let y_pos = -(y as f32 * TILE_SIZE + 8.0);

    if let Ok((_, mut tf, mut sprite)) = door_query.single_mut() {
        tf.translation.x = x_pos;
        tf.translation.y = y_pos;
        tf.translation.z = 0.5;
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            atlas.index = index;
        }
    } else {
        commands.spawn((
            DoorSprite,
            Sprite {
                image: assets.doors_image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: assets.doors_layout.clone(),
                    index,
                }),
                ..default()
            },
            Transform::from_xyz(x_pos, y_pos, 0.5),
        ));
    }
}
