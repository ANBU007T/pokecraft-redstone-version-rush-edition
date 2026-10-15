pub mod world_data;

use bevy::prelude::*;
use crate::config::*;

#[derive(Resource)]
pub struct GameAssets {
    pub atlas_image: Handle<Image>,
    pub atlas_top_image: Handle<Image>,
    pub atlas_layout: Handle<TextureAtlasLayout>,
    pub steve_image: Handle<Image>,
    pub steve_layout: Handle<TextureAtlasLayout>,
    pub doors_image: Handle<Image>,
    pub doors_layout: Handle<TextureAtlasLayout>,
    // Minecraft mechanics assets
    pub blocks_image: Handle<Image>,
    pub blocks_layout: Handle<TextureAtlasLayout>,
    pub icons_image: Handle<Image>,
    pub icons_layout: Handle<TextureAtlasLayout>,
    pub destroy_image: Handle<Image>,
    pub destroy_layout: Handle<TextureAtlasLayout>,
    pub hotbar_image: Handle<Image>,
    pub hotbar_selection_image: Handle<Image>,
    pub inventory_image: Handle<Image>,
    pub crafting_table_image: Handle<Image>,
    pub furnace_image: Handle<Image>,
    pub lit_progress_image: Handle<Image>,
    pub burn_progress_image: Handle<Image>,
}

impl GameAssets {
    pub fn new(
        asset_server: &AssetServer,
        texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    ) -> Self {
        let atlas_image = asset_server.load("world/atlas.png");
        let atlas_top_image = asset_server.load("world/atlas_top.png");

        let atlas_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::splat(16),
            ATLAS_COLS,
            ATLAS_ROWS,
            None,
            None,
        ));

        let steve_image = asset_server.load("mc/steve.png");
        let steve_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::new(STEVE_FRAME_WIDTH, STEVE_FRAME_HEIGHT),
            STEVE_TOTAL_FRAMES,
            1,
            None,
            None,
        ));

        let doors_image = asset_server.load("world/doors.png");
        let doors_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::splat(16),
            16,
            3,
            None,
            None,
        ));

        let blocks_image = asset_server.load("mc/blocks.png");
        let blocks_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::new(16, 24),
            32,
            9,
            None,
            None,
        ));

        let icons_image = asset_server.load("mc/icons.png");
        let icons_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::splat(16),
            32,
            7,
            None,
            None,
        ));

        let destroy_image = asset_server.load("mc/destroy.png");
        let destroy_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::splat(16),
            10,
            1,
            None,
            None,
        ));

        let hotbar_image = asset_server.load("mc/gui/hotbar.png");
        let hotbar_selection_image = asset_server.load("mc/gui/hotbar_selection.png");
        let inventory_image = asset_server.load("mc/gui/inventory.png");
        let crafting_table_image = asset_server.load("mc/gui/crafting_table.png");
        let furnace_image = asset_server.load("mc/gui/furnace.png");
        let lit_progress_image = asset_server.load("mc/gui/lit_progress.png");
        let burn_progress_image = asset_server.load("mc/gui/burn_progress.png");

        Self {
            atlas_image,
            atlas_top_image,
            atlas_layout,
            steve_image,
            steve_layout,
            doors_image,
            doors_layout,
            blocks_image,
            blocks_layout,
            icons_image,
            icons_layout,
            destroy_image,
            destroy_layout,
            hotbar_image,
            hotbar_selection_image,
            inventory_image,
            crafting_table_image,
            furnace_image,
            lit_progress_image,
            burn_progress_image,
        }
    }
}
