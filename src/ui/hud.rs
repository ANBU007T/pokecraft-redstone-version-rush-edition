use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use crate::config::*;
use crate::assets::GameAssets;
use crate::camera::MainCamera;
use crate::items::{Inventory, ItemRegistry};

#[derive(Component)]
pub struct HotbarBase;

#[derive(Component)]
pub struct HotbarSelector;

#[derive(Component)]
pub struct HotbarIcon {
    pub slot: usize,
}

pub fn setup_hud(mut commands: Commands, assets: Res<GameAssets>) {
    // 1. Hotbar background quad (182x22)
    commands.spawn((
        HotbarBase,
        Sprite {
            image: assets.hotbar_image.clone(),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 50.0),
    ));

    // 2. Hotbar selection box (24x23)
    commands.spawn((
        HotbarSelector,
        Sprite {
            image: assets.hotbar_selection_image.clone(),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 50.5),
    ));

    // 3. Nine item icon slots (16x16)
    for i in 0..9 {
        commands.spawn((
            HotbarIcon { slot: i },
            Sprite {
                image: assets.icons_image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: assets.icons_layout.clone(),
                    index: 0,
                }),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 51.0),
        ));
    }
}

pub fn update_hud_system(
    inventory: Res<Inventory>,
    registry: Res<ItemRegistry>,
    cam_query: Query<&Transform, With<MainCamera>>,
    mut base_query: Query<(&mut Transform, &mut Visibility), (With<HotbarBase>, Without<MainCamera>, Without<HotbarSelector>, Without<HotbarIcon>)>,
    mut sel_query: Query<(&mut Transform, &mut Visibility), (With<HotbarSelector>, Without<MainCamera>, Without<HotbarBase>, Without<HotbarIcon>)>,
    mut icon_query: Query<(&HotbarIcon, &mut Transform, &mut Sprite, &mut Visibility), (Without<MainCamera>, Without<HotbarBase>, Without<HotbarSelector>)>,
) {
    let Ok(cam_tf) = cam_query.single() else {
        return;
    };

    let cx = cam_tf.translation.x;
    let cy = cam_tf.translation.y;

    // Bottom of the screen in camera coordinates
    let bottom_y = cy - (VH / 2.0) + 13.0;

    // Update hotbar base position
    if let Ok((mut tf, _)) = base_query.single_mut() {
        tf.translation.x = cx;
        tf.translation.y = bottom_y;
    }

    // Update hotbar selection box position
    let x0 = cx - 91.0;
    let sel_x = x0 + 10.0 + (inventory.sel as f32 * 20.0);
    if let Ok((mut tf, _)) = sel_query.single_mut() {
        tf.translation.x = sel_x;
        tf.translation.y = bottom_y;
    }

    // Update item icons
    for (icon, mut tf, mut sprite, mut vis) in icon_query.iter_mut() {
        let slot_x = x0 + 11.0 + (icon.slot as f32 * 20.0);
        tf.translation.x = slot_x;
        tf.translation.y = bottom_y;

        if let Some(item_stack) = &inventory.slots[icon.slot] {
            if let Some(it) = registry.get(&item_stack.id) {
                *vis = Visibility::Inherited;
                if let Some(atlas) = sprite.texture_atlas.as_mut() {
                    atlas.index = it.icon_index;
                }
            } else {
                *vis = Visibility::Hidden;
            }
        } else {
            *vis = Visibility::Hidden;
        }
    }
}

pub fn hotbar_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_wheel: MessageReader<MouseWheel>,
    mut inventory: ResMut<Inventory>,
) {
    if keyboard_input.just_pressed(KeyCode::Digit1) { inventory.sel = 0; }
    else if keyboard_input.just_pressed(KeyCode::Digit2) { inventory.sel = 1; }
    else if keyboard_input.just_pressed(KeyCode::Digit3) { inventory.sel = 2; }
    else if keyboard_input.just_pressed(KeyCode::Digit4) { inventory.sel = 3; }
    else if keyboard_input.just_pressed(KeyCode::Digit5) { inventory.sel = 4; }
    else if keyboard_input.just_pressed(KeyCode::Digit6) { inventory.sel = 5; }
    else if keyboard_input.just_pressed(KeyCode::Digit7) { inventory.sel = 6; }
    else if keyboard_input.just_pressed(KeyCode::Digit8) { inventory.sel = 7; }
    else if keyboard_input.just_pressed(KeyCode::Digit9) { inventory.sel = 8; }

    for ev in mouse_wheel.read() {
        if ev.y > 0.0 {
            inventory.sel = (inventory.sel + 8) % 9;
        } else if ev.y < 0.0 {
            inventory.sel = (inventory.sel + 1) % 9;
        }
    }
}
