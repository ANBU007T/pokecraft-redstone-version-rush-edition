use bevy::prelude::*;
use crate::assets::GameAssets;
use crate::camera::MainCamera;
use crate::items::{Inventory, ItemStack, ItemRegistry, craft, consume_craft_grid};
use crate::world::blocks::WorldBlocks;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ContainerMode {
    #[default]
    Inventory,
    CraftingTable,
    Furnace((i32, i32)),
}

#[derive(Resource, Default)]
pub struct InventoryScreenState {
    pub open: bool,
    pub mode: ContainerMode,
    pub cursor: Option<ItemStack>,
    pub crafting_grid: [Option<ItemStack>; 9],
    pub craft_result: Option<ItemStack>,
}

impl InventoryScreenState {
    pub fn open_inventory(&mut self) {
        self.open = true;
        self.mode = ContainerMode::Inventory;
    }

    pub fn open_crafting_table(&mut self) {
        self.open = true;
        self.mode = ContainerMode::CraftingTable;
    }

    pub fn open_furnace(&mut self, pos: (i32, i32)) {
        self.open = true;
        self.mode = ContainerMode::Furnace(pos);
    }

    pub fn close(&mut self, inv: &mut Inventory) {
        self.open = false;

        // Return held cursor item to inventory
        if let Some(c) = self.cursor.take() {
            inv.add(&c.id, c.count, 64);
        }

        // Return crafting grid items to inventory
        for slot in self.crafting_grid.iter_mut() {
            if let Some(s) = slot.take() {
                inv.add(&s.id, s.count, 64);
            }
        }

        self.craft_result = None;
        self.mode = ContainerMode::Inventory;
    }

    pub fn update_crafting_result(&mut self, registry: &ItemRegistry) {
        let res = match self.mode {
            ContainerMode::Inventory => craft(&self.crafting_grid[0..4], 2, 2, registry),
            ContainerMode::CraftingTable => craft(&self.crafting_grid[0..9], 3, 3, registry),
            ContainerMode::Furnace(_) => None,
        };
        self.craft_result = res.map(|r| ItemStack::new(r.id, r.count));
    }

    pub fn take_craft_result(&mut self, _inv: &mut Inventory, registry: &ItemRegistry) -> bool {
        if let Some(result) = self.craft_result.take() {
            // Add crafted item to inventory or cursor
            if let Some(cur) = &mut self.cursor {
                if cur.id == result.id && cur.count + result.count <= 64 {
                    cur.count += result.count;
                } else {
                    self.craft_result = Some(result);
                    return false;
                }
            } else {
                self.cursor = Some(result);
            }

            // Consume crafting ingredients
            let size = if self.mode == ContainerMode::CraftingTable { 9 } else { 4 };
            consume_craft_grid(&mut self.crafting_grid[0..size]);

            // Re-evaluate recipe
            self.update_crafting_result(registry);
            true
        } else {
            false
        }
    }
}

#[derive(Component)]
pub struct InventoryWindow;

#[derive(Component)]
pub struct InventorySlotIcon {
    pub slot_type: SlotType,
    pub index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotType {
    Main(usize),      // 0..26 (inv slots 9..35)
    Hotbar(usize),    // 0..8 (inv slots 0..8)
    CraftGrid(usize), // 0..8 (0..3 for 2x2, 0..8 for 3x3)
    CraftResult,
    FurnaceInput,
    FurnaceFuel,
    FurnaceOutput,
}

pub fn setup_inventory_screen(mut commands: Commands, assets: Res<GameAssets>) {
    // Inventory background window (176x166 px)
    commands.spawn((
        InventoryWindow,
        Sprite {
            image: assets.inventory_image.clone(),
            rect: Some(Rect::new(0.0, 0.0, 176.0, 166.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 60.0),
        Visibility::Hidden,
    ));

    // Hotbar slots (0..8)
    for i in 0..9 {
        commands.spawn((
            InventorySlotIcon {
                slot_type: SlotType::Hotbar(i),
                index: i,
            },
            Sprite {
                image: assets.icons_image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: assets.icons_layout.clone(),
                    index: 0,
                }),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 61.0),
            Visibility::Hidden,
        ));
    }

    // Main inventory slots (0..26)
    for i in 0..27 {
        commands.spawn((
            InventorySlotIcon {
                slot_type: SlotType::Main(i),
                index: 9 + i,
            },
            Sprite {
                image: assets.icons_image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: assets.icons_layout.clone(),
                    index: 0,
                }),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 61.0),
            Visibility::Hidden,
        ));
    }

    // 3x3 crafting grid (0..8)
    for i in 0..9 {
        commands.spawn((
            InventorySlotIcon {
                slot_type: SlotType::CraftGrid(i),
                index: i,
            },
            Sprite {
                image: assets.icons_image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: assets.icons_layout.clone(),
                    index: 0,
                }),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 61.0),
            Visibility::Hidden,
        ));
    }

    // Craft result slot
    commands.spawn((
        InventorySlotIcon {
            slot_type: SlotType::CraftResult,
            index: 0,
        },
        Sprite {
            image: assets.icons_image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.icons_layout.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 61.0),
        Visibility::Hidden,
    ));

    // Furnace slots
    commands.spawn((
        InventorySlotIcon {
            slot_type: SlotType::FurnaceInput,
            index: 0,
        },
        Sprite {
            image: assets.icons_image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.icons_layout.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 61.0),
        Visibility::Hidden,
    ));

    commands.spawn((
        InventorySlotIcon {
            slot_type: SlotType::FurnaceFuel,
            index: 0,
        },
        Sprite {
            image: assets.icons_image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.icons_layout.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 61.0),
        Visibility::Hidden,
    ));

    commands.spawn((
        InventorySlotIcon {
            slot_type: SlotType::FurnaceOutput,
            index: 0,
        },
        Sprite {
            image: assets.icons_image.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.icons_layout.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 61.0),
        Visibility::Hidden,
    ));
}

pub fn inventory_toggle_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<InventoryScreenState>,
    mut inv: ResMut<Inventory>,
    registry: Res<ItemRegistry>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        if state.open {
            state.close(&mut inv);
        } else {
            state.open_inventory();
            state.update_crafting_result(&registry);
        }
    } else if keyboard_input.just_pressed(KeyCode::Escape) && state.open {
        state.close(&mut inv);
    }
}

pub fn update_inventory_screen_system(
    state: Res<InventoryScreenState>,
    inv: Res<Inventory>,
    registry: Res<ItemRegistry>,
    world_blocks: Res<WorldBlocks>,
    assets: Res<GameAssets>,
    cam_query: Query<&Transform, With<MainCamera>>,
    mut window_query: Query<(&mut Transform, &mut Sprite, &mut Visibility), (With<InventoryWindow>, Without<InventorySlotIcon>, Without<MainCamera>)>,
    mut icons_query: Query<(&InventorySlotIcon, &mut Transform, &mut Sprite, &mut Visibility), (Without<InventoryWindow>, Without<MainCamera>)>,
) {
    let Ok(cam_tf) = cam_query.single() else {
        return;
    };

    let cx = cam_tf.translation.x;
    let cy = cam_tf.translation.y;

    let is_open = state.open;
    let window_vis = if is_open { Visibility::Inherited } else { Visibility::Hidden };

    if let Ok((mut tf, mut sprite, mut vis)) = window_query.single_mut() {
        tf.translation.x = cx;
        tf.translation.y = cy;
        *vis = window_vis;

        if is_open {
            match state.mode {
                ContainerMode::Inventory => {
                    sprite.image = assets.inventory_image.clone();
                }
                ContainerMode::CraftingTable => {
                    sprite.image = assets.crafting_table_image.clone();
                }
                ContainerMode::Furnace(_) => {
                    sprite.image = assets.furnace_image.clone();
                }
            }
        }
    }

    if !is_open {
        for (_, _, _, mut vis) in icons_query.iter_mut() {
            *vis = Visibility::Hidden;
        }
        return;
    }

    // Top-left corner of the 176x166 window
    let x0 = cx - 88.0;
    let y0 = cy + 83.0;

    // Check furnace block state if in furnace mode
    let furnace_block = if let ContainerMode::Furnace((fx, fy)) = state.mode {
        world_blocks.get(fx, fy).and_then(|b| b.furnace.as_ref())
    } else {
        None
    };

    for (slot_icon, mut tf, mut sprite, mut vis) in icons_query.iter_mut() {
        let (rel_x, rel_y, stack, active) = match slot_icon.slot_type {
            SlotType::Main(idx) => {
                let row = idx / 9;
                let col = idx % 9;
                (8.0 + col as f32 * 18.0 + 8.0, 84.0 + row as f32 * 18.0 + 8.0, &inv.slots[9 + idx], true)
            }
            SlotType::Hotbar(idx) => {
                (8.0 + idx as f32 * 18.0 + 8.0, 142.0 + 8.0, &inv.slots[idx], true)
            }
            SlotType::CraftGrid(idx) => {
                match state.mode {
                    ContainerMode::Inventory => {
                        if idx < 4 {
                            let row = idx / 2;
                            let col = idx % 2;
                            (98.0 + col as f32 * 18.0 + 8.0, 18.0 + row as f32 * 18.0 + 8.0, &state.crafting_grid[idx], true)
                        } else {
                            (0.0, 0.0, &None, false)
                        }
                    }
                    ContainerMode::CraftingTable => {
                        let row = idx / 3;
                        let col = idx % 3;
                        (30.0 + col as f32 * 18.0 + 8.0, 17.0 + row as f32 * 18.0 + 8.0, &state.crafting_grid[idx], true)
                    }
                    ContainerMode::Furnace(_) => (0.0, 0.0, &None, false),
                }
            }
            SlotType::CraftResult => {
                match state.mode {
                    ContainerMode::Inventory => (154.0 + 8.0, 28.0 + 8.0, &state.craft_result, true),
                    ContainerMode::CraftingTable => (124.0 + 8.0, 35.0 + 8.0, &state.craft_result, true),
                    ContainerMode::Furnace(_) => (0.0, 0.0, &None, false),
                }
            }
            SlotType::FurnaceInput => {
                if let Some(f) = furnace_block {
                    (56.0 + 8.0, 17.0 + 8.0, &f.input, true)
                } else {
                    (0.0, 0.0, &None, false)
                }
            }
            SlotType::FurnaceFuel => {
                if let Some(f) = furnace_block {
                    (56.0 + 8.0, 53.0 + 8.0, &f.fuel, true)
                } else {
                    (0.0, 0.0, &None, false)
                }
            }
            SlotType::FurnaceOutput => {
                if let Some(f) = furnace_block {
                    (116.0 + 8.0, 35.0 + 8.0, &f.output, true)
                } else {
                    (0.0, 0.0, &None, false)
                }
            }
        };

        if !active {
            *vis = Visibility::Hidden;
            continue;
        }

        tf.translation.x = x0 + rel_x;
        tf.translation.y = y0 - rel_y;

        if let Some(s) = stack {
            if let Some(it) = registry.get(&s.id) {
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
