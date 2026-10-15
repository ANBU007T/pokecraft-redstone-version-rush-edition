use pokecraft_bevy::items::{
    ItemRegistry, ItemStack, Inventory, craft, consume_craft_grid, craftable,
};
use pokecraft_bevy::world::furnace::FurnaceState;
use pokecraft_bevy::world::campfire::{CampfireState, CAMPFIRE_COOK_TIME};

fn load_test_registry() -> ItemRegistry {
    // Look in current or parent directory for assets
    let paths = ["assets", "../assets"];
    for p in paths {
        if let Ok(reg) = ItemRegistry::load_from_dir(p) {
            return reg;
        }
    }
    panic!("Could not load items.json for Stage 4 tests");
}

#[test]
fn test_shaped_and_shapeless_crafting() {
    let registry = load_test_registry();

    // 1. Shapeless recipe: 1 oak_log -> 4 oak_planks
    let grid_planks = [
        Some(ItemStack::new("oak_log", 1)),
        None,
        None,
        None,
    ];
    let res = craft(&grid_planks, 2, 2, &registry);
    assert!(res.is_some(), "oak_log should craft into oak_planks");
    let out = res.unwrap();
    assert_eq!(out.id, "oak_planks");
    assert_eq!(out.count, 4);

    // 2. Shaped recipe: 2 vertical oak_planks -> 4 sticks (2x2 grid)
    let grid_sticks = [
        Some(ItemStack::new("oak_planks", 1)),
        None,
        Some(ItemStack::new("oak_planks", 1)),
        None,
    ];
    let res_stick = craft(&grid_sticks, 2, 2, &registry);
    assert!(res_stick.is_some(), "2 vertical planks should craft sticks");
    let out_stick = res_stick.unwrap();
    assert_eq!(out_stick.id, "stick");
    assert_eq!(out_stick.count, 4);

    // 3. 3x3 Shaped recipe: Wooden Pickaxe
    // Row 0: plank, plank, plank
    // Row 1: air,   stick, air
    // Row 2: air,   stick, air
    let mut grid_pick = vec![None; 9];
    grid_pick[0] = Some(ItemStack::new("oak_planks", 1));
    grid_pick[1] = Some(ItemStack::new("oak_planks", 1));
    grid_pick[2] = Some(ItemStack::new("oak_planks", 1));
    grid_pick[4] = Some(ItemStack::new("stick", 1));
    grid_pick[7] = Some(ItemStack::new("stick", 1));

    let res_pick = craft(&grid_pick, 3, 3, &registry);
    assert!(res_pick.is_some(), "Crafting table should craft wooden_pickaxe");
    let out_pick = res_pick.unwrap();
    assert_eq!(out_pick.id, "wooden_pickaxe");
    assert_eq!(out_pick.count, 1);

    // 4. Test Horizontal Mirror: Wooden Axe (left-facing vs right-facing head)
    // Normal:
    // plank, plank, air
    // plank, stick, air
    // air,   stick, air
    let mut grid_axe_normal = vec![None; 9];
    grid_axe_normal[0] = Some(ItemStack::new("oak_planks", 1));
    grid_axe_normal[1] = Some(ItemStack::new("oak_planks", 1));
    grid_axe_normal[3] = Some(ItemStack::new("oak_planks", 1));
    grid_axe_normal[4] = Some(ItemStack::new("stick", 1));
    grid_axe_normal[7] = Some(ItemStack::new("stick", 1));

    let res_axe_norm = craft(&grid_axe_normal, 3, 3, &registry);
    assert!(res_axe_norm.is_some(), "Normal wooden axe should match");

    // Mirrored:
    // air, plank, plank
    // air, stick, plank
    // air, stick, air
    let mut grid_axe_mirrored = vec![None; 9];
    grid_axe_mirrored[1] = Some(ItemStack::new("oak_planks", 1));
    grid_axe_mirrored[2] = Some(ItemStack::new("oak_planks", 1));
    grid_axe_mirrored[4] = Some(ItemStack::new("stick", 1));
    grid_axe_mirrored[5] = Some(ItemStack::new("oak_planks", 1));
    grid_axe_mirrored[7] = Some(ItemStack::new("stick", 1));

    let res_axe_mir = craft(&grid_axe_mirrored, 3, 3, &registry);
    assert!(res_axe_mir.is_some(), "Mirrored wooden axe should match");
    assert_eq!(res_axe_mir.unwrap().id, "wooden_axe");

    // 5. Grid consumption test
    let mut test_consume = grid_axe_normal.clone();
    consume_craft_grid(&mut test_consume);
    assert!(test_consume.iter().all(|s| s.is_none()), "All ingredients consumed");
}

#[test]
fn test_craftable_recipes_query() {
    let registry = load_test_registry();
    let mut inv = Inventory::default();
    inv.slots[0] = Some(ItemStack::new("oak_log", 10));

    let craftables = craftable(&inv, 2, &registry);
    assert!(!craftables.is_empty(), "Should find craftable recipes for oak_log");
    let has_planks = craftables.iter().any(|r| r.out == "oak_planks");
    assert!(has_planks, "Craftable list should contain oak_planks");
}

#[test]
fn test_furnace_smelting_and_fuel() {
    let registry = load_test_registry();
    let mut furnace = FurnaceState::new();

    // Setup: Raw Iron + Coal
    furnace.input = Some(ItemStack::new("raw_iron", 2));
    furnace.fuel = Some(ItemStack::new("coal", 1));

    assert!(!furnace.is_lit());

    // 1st tick: furnace ignites coal (1600 ticks burn)
    furnace.tick(&registry);
    assert!(furnace.is_lit(), "Furnace should ignite coal");
    assert_eq!(furnace.burn_max, 1600);
    assert_eq!(furnace.burn, 1600);
    assert!(furnace.fuel.is_none(), "Coal consumed into fuel burn time");

    // Advance 199 more ticks -> total 200 cook ticks
    for _ in 0..199 {
        furnace.tick(&registry);
    }

    assert_eq!(furnace.cook, 0, "Cook timer resets upon recipe completion");
    assert!(furnace.output.is_some(), "Output should have smelted item");
    let out = furnace.output.as_ref().unwrap();
    assert_eq!(out.id, "iron_ingot");
    assert_eq!(out.count, 1);
    assert_eq!(furnace.input.as_ref().unwrap().count, 1, "1 raw iron remains");
    assert!(furnace.xp > 0.0, "Smelting iron yields XP");

    // Take output and XP
    let (taken_stack, taken_xp) = furnace.take_output();
    assert_eq!(taken_stack.unwrap().id, "iron_ingot");
    assert!(taken_xp > 0.0);
    assert_eq!(furnace.xp, 0.0, "XP cleared after taking");
}

#[test]
fn test_campfire_cooking_and_heat_drain() {
    let registry = load_test_registry();
    let mut campfire = CampfireState::new();

    assert!(campfire.lit);

    // Place beef and cod onto campfire (actual Minecraft item IDs from items.json)
    let added_beef = campfire.add_item("beef", &registry);
    let added_cod = campfire.add_item("cod", &registry);
    assert!(added_beef, "beef can be placed on campfire");
    assert!(added_cod, "cod can be placed on campfire");

    // Cannot add non-cookable item
    let added_stone = campfire.add_item("cobblestone", &registry);
    assert!(!added_stone, "cobblestone cannot be placed on campfire");

    // Cook for 300 ticks (halfway)
    for _ in 0..300 {
        let popped = campfire.tick(&registry);
        assert!(popped.is_empty(), "Items not cooked yet at halfway");
    }

    // Extinguish campfire
    assert!(campfire.extinguish());
    assert!(!campfire.lit);

    // Tick unlit campfire -> heat should drain
    for _ in 0..50 {
        campfire.tick(&registry);
    }
    assert!(campfire.items[0].as_ref().unwrap().cook_ticks < 300, "Heat drains when unlit");

    // Reignite campfire
    assert!(campfire.ignite());
    assert!(campfire.lit);

    // Cook until finish (600 ticks)
    let mut all_popped = Vec::new();
    for _ in 0..CAMPFIRE_COOK_TIME {
        let popped = campfire.tick(&registry);
        all_popped.extend(popped);
    }

    assert_eq!(all_popped.len(), 2, "Both items should finish cooking and pop out");
    assert!(all_popped.contains(&"cooked_beef".to_string()));
    assert!(all_popped.contains(&"cooked_cod".to_string()));
    assert!(campfire.items.iter().all(|s| s.is_none()), "Campfire slots clear after popping");
}
