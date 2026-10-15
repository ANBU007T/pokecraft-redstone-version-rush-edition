use bevy::prelude::*;
use bevy::camera::{ScalingMode, Projection};
use crate::config::*;
use crate::assets::world_data::WorldData;
use crate::player::movement::Player;

#[derive(Component)]
pub struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, camera_follow_player_system);
    }
}

fn setup_camera(mut commands: Commands) {
    let mut proj = OrthographicProjection::default_2d();
    proj.scaling_mode = ScalingMode::Fixed {
        width: VW,
        height: VH,
    };

    // Pallet Town spawn is at x: 66, y: 268
    let initial_x = 66.0 * TILE_SIZE + 8.0;
    let initial_y = -(268.0 * TILE_SIZE + 8.0);

    commands.spawn((
        Camera2d,
        Projection::Orthographic(proj),
        MainCamera,
        Transform::from_xyz(initial_x, initial_y, 0.0),
    ));
}

fn camera_follow_player_system(
    player_query: Query<&Player>,
    world_data: Res<WorldData>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    let Ok(player) = player_query.single() else {
        return;
    };
    let Ok(mut cam_transform) = camera_query.single_mut() else {
        return;
    };

    // Calculate player center
    let player_x = player.fx * TILE_SIZE + 8.0;
    let player_y = -(player.fy * TILE_SIZE + 8.0);

    let target_cam_x: f32;
    let target_cam_y: f32;

    if world_data.is_outdoor(player.tx, player.ty) {
        let half_vw = VW / 2.0;
        let half_vh = VH / 2.0;
        let max_map_x = world_data.meta.outdoor.w as f32 * TILE_SIZE;
        let max_map_y = world_data.meta.outdoor.h as f32 * TILE_SIZE;

        target_cam_x = player_x.clamp(half_vw, max_map_x - half_vw);
        target_cam_y = player_y.clamp(-(max_map_y - half_vh), -half_vh);
    } else {
        target_cam_x = player_x;
        target_cam_y = player_y;
    }

    // Pixel snap to prevent tile-bleeding/seams
    cam_transform.translation.x = target_cam_x.round();
    cam_transform.translation.y = target_cam_y.round();
    cam_transform.translation.z = 0.0;
}
