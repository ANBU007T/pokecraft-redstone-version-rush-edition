pub mod chunk;
pub mod transition;
pub mod blocks;
pub mod furnace;
pub mod campfire;

use bevy::prelude::*;
use self::chunk::{update_chunks_system, LoadedChunks};
use self::transition::{
    setup_fade_overlay, fade_overlay_system, warp_transition_system,
    WarpState, ScreenFade,
};
use self::blocks::{update_block_sprites_system, tick_containers_system, WorldBlocks};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadedChunks>()
            .init_resource::<WarpState>()
            .init_resource::<ScreenFade>()
            .init_resource::<WorldBlocks>()
            .add_systems(Startup, setup_fade_overlay)
            .add_systems(
                Update,
                (
                    update_chunks_system,
                    warp_transition_system,
                    fade_overlay_system,
                    update_block_sprites_system,
                    tick_containers_system,
                ),
            );
    }
}
