pub const TILE_SIZE: f32 = 16.0;
pub const VW: f32 = 240.0;
pub const VH: f32 = 160.0;
pub const WINDOW_WIDTH: f32 = 960.0;
pub const WINDOW_HEIGHT: f32 = 640.0;

pub const WORLD_WIDTH: usize = 408;
pub const WORLD_HEIGHT: usize = 1062;
pub const OUTDOOR_WIDTH: usize = 408;
pub const OUTDOOR_HEIGHT: usize = 400;

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_PIXELS: f32 = (CHUNK_SIZE as f32) * TILE_SIZE; // 256.0

pub const WALK_TIME: f32 = 0.24;   // seconds per tile (~4.17 tiles/s)
pub const SPRINT_TIME: f32 = 0.15; // seconds per tile (~6.67 tiles/s, 1.6x)
pub const SWIM_TIME: f32 = 0.42;   // seconds per tile
pub const BOAT_TIME: f32 = 0.17;   // seconds per tile

pub const ATLAS_COLS: u32 = 64;
pub const ATLAS_ROWS: u32 = 141;

pub const STEVE_FRAME_WIDTH: u32 = 24;
pub const STEVE_FRAME_HEIGHT: u32 = 32;
pub const STEVE_TOTAL_FRAMES: u32 = 65;

// Steve origin: [12, 30], FOOT = 13
pub const FOOT_OFFSET: f32 = 13.0;

pub mod settings;
pub use settings::*;

