pub mod hud;
pub mod inventory;
pub mod menu;

use bevy::prelude::*;
use self::hud::{setup_hud, update_hud_system, hotbar_input_system};
use self::inventory::{
    setup_inventory_screen, inventory_toggle_system, update_inventory_screen_system,
    InventoryScreenState,
};
use self::menu::{menu_input_system, MenuManager};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InventoryScreenState>()
            .init_resource::<MenuManager>()
            .add_systems(Startup, (setup_hud, setup_inventory_screen))
            .add_systems(
                Update,
                (
                    hotbar_input_system,
                    update_hud_system,
                    inventory_toggle_system,
                    update_inventory_screen_system,
                    menu_input_system,
                ),
            );
    }
}
