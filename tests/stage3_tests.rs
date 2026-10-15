use pokecraft_bevy::assets::world_data::WorldData;
use pokecraft_bevy::items::{ItemRegistry, Inventory, ItemStack, break_info};
use pokecraft_bevy::world::blocks::{WorldBlocks, PlacedBlock, can_place_block};
use std::path::PathBuf;

fn get_assets_path() -> PathBuf {
    let candidates = [
        PathBuf::from("assets"),
        PathBuf::from("../assets"),
        PathBuf::from("../../assets"),
        PathBuf::from("c:/Users/anbu/Downloads/POKEMC/PC-Rust/assets"),
    ];
    for p in &candidates {
        if p.join("world/world.json").exists() {
            return p.clone();
        }
    }
    PathBuf::from("assets")
}

#[test]
fn test_item_registry_loading() {
    let assets_dir = get_assets_path();
    let registry = ItemRegistry::load_from_dir(&assets_dir).expect("ItemRegistry should load");

    assert_eq!(registry.data.items.len(), 265, "Total items should be 265");
    assert_eq!(registry.data.blocks.len(), 280, "Total block textures should be 280");

    // Check pickaxe specs
    let pick = registry.get("iron_pickaxe").expect("iron_pickaxe must exist");
    assert_eq!(pick.name, "Iron Pickaxe");
    let tool = pick.tool.as_ref().expect("iron_pickaxe must have tool spec");
    assert_eq!(tool.kind, "pickaxe");
    assert_eq!(tool.tier, 3);
    assert_eq!(tool.speed, 6.0);
    assert_eq!(tool.dur, 250);

    // Check block specs
    let log = registry.get("oak_log").expect("oak_log must exist");
    let block = log.block.as_ref().expect("oak_log must have block spec");
    assert_eq!(block.hard, 2.0);
    assert_eq!(block.tool.as_deref(), Some("axe"));
}

#[test]
fn test_inventory_management() {
    let mut inv = Inventory {
        slots: std::array::from_fn(|_| None),
        armor: std::array::from_fn(|_| None),
        sel: 0,
    };

    // 1. Add items up to max stack
    let left = inv.add("cobblestone", 64, 64);
    assert_eq!(left, 0);
    assert_eq!(inv.slots[0], Some(ItemStack::new("cobblestone", 64)));

    // 2. Add overflow into next slot
    let left2 = inv.add("cobblestone", 10, 64);
    assert_eq!(left2, 0);
    assert_eq!(inv.slots[1], Some(ItemStack::new("cobblestone", 10)));

    // 3. Held item check
    inv.sel = 0;
    assert_eq!(inv.held().map(|s| s.id.as_str()), Some("cobblestone"));

    // 4. Use held item
    inv.use_held();
    assert_eq!(inv.held().map(|s| s.count), Some(63));

    // 5. Wear held tool
    inv.slots[2] = Some(ItemStack::new("iron_sword", 1));
    inv.sel = 2;
    let broke = inv.wear_held(10, 250);
    assert!(!broke);
    assert_eq!(inv.held().map(|s| s.dmg), Some(10));
    let broke_final = inv.wear_held(240, 250);
    assert!(broke_final);
    assert!(inv.held().is_none(), "Broken tool must be removed from slot");
}

#[test]
fn test_block_placement_and_collision() {
    let assets_dir = get_assets_path();
    let world_data = WorldData::load_from_dir(&assets_dir).expect("WorldData should load");
    let mut world_blocks = WorldBlocks::default();

    // Pallet Town spawn is at (66, 268)
    let p_tx = 66;
    let p_ty = 268;

    // Cannot place directly on player's position
    assert!(!can_place_block(&world_data, &world_blocks, p_tx, p_ty, 66, 268));

    // Cannot place on house door warp (66, 267)
    assert!(!can_place_block(&world_data, &world_blocks, p_tx, p_ty, 66, 267));

    // Can place in front on open grass (66, 269)
    assert!(can_place_block(&world_data, &world_blocks, p_tx, p_ty, 66, 269));

    // Place an oak planks block
    world_blocks.set(66, 269, PlacedBlock::new("oak_planks", false, false));

    // The cell is now solid
    assert!(world_blocks.is_solid(66, 269), "Placed solid block must block movement");

    // Cannot place again on already placed solid block
    assert!(!can_place_block(&world_data, &world_blocks, p_tx, p_ty, 66, 269));

    // Mine / remove block
    let removed = world_blocks.remove(66, 269);
    assert_eq!(removed.map(|b| b.id), Some("oak_planks".into()));
    assert!(!world_blocks.is_solid(66, 269), "Cell must no longer be solid after block removal");
}

#[test]
fn test_mining_break_info() {
    let assets_dir = get_assets_path();
    let registry = ItemRegistry::load_from_dir(&assets_dir).expect("ItemRegistry should load");

    let stone = registry.get("stone").expect("stone must exist");
    let stone_spec = stone.block.as_ref().expect("stone block spec must exist");

    let pick = registry.get("iron_pickaxe").expect("iron_pickaxe must exist");
    let pick_tool = pick.tool.as_ref();

    // Mining stone with iron pickaxe
    let (pick_ticks, pick_harvest) = break_info(stone_spec, pick_tool);
    assert!(pick_harvest, "Iron pickaxe must harvest stone");
    assert!(pick_ticks <= 15.0, "Iron pickaxe should break stone quickly (<= 15 ticks)");

    // Mining stone with bare hands
    let (hand_ticks, hand_harvest) = break_info(stone_spec, None);
    assert!(!hand_harvest, "Bare hands cannot harvest stone");
    assert!(hand_ticks > pick_ticks * 5.0, "Hand mining stone must be much slower than pickaxe");
}
