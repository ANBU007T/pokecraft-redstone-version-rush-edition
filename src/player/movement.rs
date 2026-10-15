use bevy::prelude::*;
use crate::config::*;
use crate::assets::world_data::WorldData;
use crate::world::transition::{WarpState, WarpPhase};
use crate::world::blocks::WorldBlocks;
use crate::ui::inventory::InventoryScreenState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Direction {
    #[default]
    Down,
    Up,
    Left,
    Right,
}

impl Direction {
    pub fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Down => (0, 1),
            Direction::Up => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    pub fn walk_frames(&self) -> [usize; 3] {
        match self {
            Direction::Down => [0, 1, 2],
            Direction::Up => [3, 4, 5],
            Direction::Left => [6, 7, 8],
            Direction::Right => [9, 10, 11],
        }
    }

    pub fn from_str_name(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "up" => Some(Direction::Up),
            "down" => Some(Direction::Down),
            "left" => Some(Direction::Left),
            "right" => Some(Direction::Right),
            _ => None,
        }
    }
}

#[derive(Component)]
pub struct Player {
    pub tx: i32,
    pub ty: i32,
    pub from_x: i32,
    pub from_y: i32,
    pub fx: f32,
    pub fy: f32,
    pub dir: Direction,
    pub moving: bool,
    pub prog: f32,
    pub speed: f32,
    pub dist: i32,
    pub sprinting: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            tx: 66,
            ty: 268,
            from_x: 66,
            from_y: 268,
            fx: 66.0,
            fy: 268.0,
            dir: Direction::Down,
            moving: false,
            prog: 0.0,
            speed: WALK_TIME,
            dist: 1,
            sprinting: false,
        }
    }
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            tx: x,
            ty: y,
            from_x: x,
            from_y: y,
            fx: x as f32,
            fy: y as f32,
            ..default()
        }
    }

    pub fn current_frame(&self) -> usize {
        let frames = self.dir.walk_frames();
        if !self.moving {
            frames[0]
        } else {
            let step_idx = match (self.prog * 4.0).floor() as usize % 4 {
                0 => 0,
                1 => 1,
                2 => 0,
                _ => 2,
            };
            frames[step_idx]
        }
    }
}

pub fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    world_data: Res<WorldData>,
    world_blocks: Res<WorldBlocks>,
    mut warp_state: ResMut<WarpState>,
    inv_screen: Option<Res<InventoryScreenState>>,
    mut query: Query<(&mut Player, &mut Transform, &mut Sprite)>,
) {
    let dt = time.delta_secs();

    let is_inv_open = inv_screen.map(|s| s.open).unwrap_or(false);

    for (mut player, mut transform, mut sprite) in query.iter_mut() {
        // While warp sequence is running, player movement input is disabled
        if warp_state.current.is_some() {
            let lift = if player.moving && player.dist == 2 {
                (player.prog * std::f32::consts::PI).sin() * 8.0
            } else {
                0.0
            };
            transform.translation.x = player.fx * TILE_SIZE + 8.0;
            transform.translation.y = -player.fy * TILE_SIZE + 1.0 + lift;
            transform.translation.z = 1.0;
            if let Some(atlas) = sprite.texture_atlas.as_mut() {
                atlas.index = player.current_frame();
            }
            continue;
        }

        if is_inv_open {
            transform.translation.x = player.fx * TILE_SIZE + 8.0;
            transform.translation.y = -player.fy * TILE_SIZE + 1.0;
            transform.translation.z = 1.0;
            continue;
        }

        let is_sprint = keyboard_input.pressed(KeyCode::ShiftLeft)
            || keyboard_input.pressed(KeyCode::ShiftRight);
        player.sprinting = is_sprint;
        let current_speed = if is_sprint { SPRINT_TIME } else { WALK_TIME };

        let mut input_dir = None;
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            input_dir = Some(Direction::Up);
        } else if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            input_dir = Some(Direction::Down);
        } else if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            input_dir = Some(Direction::Left);
        } else if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            input_dir = Some(Direction::Right);
        }

        if player.moving {
            player.prog += dt / player.speed;
            if player.prog >= 1.0 {
                player.prog = 0.0;
                player.moving = false;
                player.fx = player.tx as f32;
                player.fy = player.ty as f32;
                player.from_x = player.tx;
                player.from_y = player.ty;
                player.dist = 1;

                // Check for step warp at current cell (e.g. escalator)
                if let Some(sw) = world_data.warp_at(player.tx, player.ty) {
                    if sw.trig == "step" {
                        warp_state.current = Some(WarpPhase::FadeOut {
                            timer: 0.0,
                            warp: sw.clone(),
                        });
                        continue;
                    }
                }

                // If direction key is still held, attempt chaining next step
                if let Some(dir) = input_dir {
                    try_start_step(&mut player, dir, &world_data, &world_blocks, &mut warp_state, current_speed);
                }
            } else {
                player.fx = player.from_x as f32 + (player.tx - player.from_x) as f32 * player.prog;
                player.fy = player.from_y as f32 + (player.ty - player.from_y) as f32 * player.prog;
            }
        } else if let Some(dir) = input_dir {
            try_start_step(&mut player, dir, &world_data, &world_blocks, &mut warp_state, current_speed);
        }

        // Apply jump elevation lift if dist == 2 (authentic parabolic hop arc)
        let lift = if player.moving && player.dist == 2 {
            (player.prog * std::f32::consts::PI).sin() * 8.0
        } else {
            0.0
        };

        // Update Transform in Bevy coordinates
        transform.translation.x = player.fx * TILE_SIZE + 8.0;
        transform.translation.y = -player.fy * TILE_SIZE + 1.0 + lift;
        transform.translation.z = 1.0;

        // Update animation frame in Sprite
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
            atlas.index = player.current_frame();
        }
    }
}

fn try_start_step(
    player: &mut Player,
    dir: Direction,
    world_data: &WorldData,
    world_blocks: &WorldBlocks,
    warp_state: &mut WarpState,
    current_speed: f32,
) {
    // 1. Check mat/stairs warp at current cell when stepping off in arrow direction
    if let Some(w) = world_data.warp_at(player.tx, player.ty) {
        let matches = match w.trig.as_str() {
            "arrowS" => dir == Direction::Down,
            "arrowN" => dir == Direction::Up,
            "arrowE" | "stairsE" => dir == Direction::Right,
            "arrowW" | "stairsW" => dir == Direction::Left,
            _ => false,
        };
        if matches {
            player.dir = dir;
            warp_state.current = Some(WarpPhase::FadeOut {
                timer: 0.0,
                warp: w.clone(),
            });
            return;
        }
    }

    let (dx, dy) = dir.delta();
    let nx = player.tx + dx;
    let ny = player.ty + dy;

    // 2. Check bump warp ahead (doors) when walking Up into a door
    if dir == Direction::Up {
        if let Some(ahead) = world_data.warp_at(nx, ny) {
            if ahead.trig == "bump" {
                player.dir = dir;
                let anim = ahead
                    .door
                    .or_else(|| world_data.door_at(nx, ny).map(|d| d.anim))
                    .unwrap_or(0);
                warp_state.current = Some(WarpPhase::Open {
                    timer: 0.0,
                    warp: ahead.clone(),
                    anim,
                });
                return;
            }
        }
    }

    // 3. Check ledge jump ahead (MB.JUMP 0x38..0x3B)
    if let Some(j_dir) = world_data.jump_dir(nx, ny) {
        if j_dir == dir {
            let jx = nx + dx;
            let jy = ny + dy;
            if !world_data.is_solid(jx, jy) && !world_blocks.is_solid(jx, jy) {
                player.dir = dir;
                player.moving = true;
                player.prog = 0.0;
                player.speed = 0.36; // Authentic ledge hop duration
                player.dist = 2;
                player.from_x = player.tx;
                player.from_y = player.ty;
                player.tx = jx;
                player.ty = jy;
                return;
            }
        } else {
            // Blocked by ledge facing different direction
            player.dir = dir;
            return;
        }
    }

    // 4. Check solid collision (placed Minecraft blocks or overworld solids)
    player.dir = dir;
    if world_blocks.is_solid(nx, ny) || world_data.is_solid(nx, ny) {
        // Solid wall, tree, building, fence, or placed block: turn only, do not move
        return;
    }

    // 5. Passable tile: start normal step
    player.moving = true;
    player.prog = 0.0;
    player.speed = current_speed;
    player.dist = 1;
    player.from_x = player.tx;
    player.from_y = player.ty;
    player.tx = nx;
    player.ty = ny;
}
