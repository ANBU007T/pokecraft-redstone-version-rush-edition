use pokecraft_bevy::dimensions::nether::{
    overworld_to_nether, nether_to_overworld, find_portal_interior, NETHER_SCALE,
};
use pokecraft_bevy::dimensions::end::EndDimensionState;
use pokecraft_bevy::dimensions::dragon::{
    EnderDragonBoss, DragonPhase, DragonEvent, DRAGON_MAX_HP, DRAGON_EXHAUST_HP,
};

#[test]
fn test_nether_coordinates_and_portal_frame() {
    assert_eq!(NETHER_SCALE, 8);
    assert_eq!(overworld_to_nether(800, 400), (100, 50));
    assert_eq!(nether_to_overworld(100, 50), (800, 400));

    // Construct a standard 2x3 portal frame in grid
    // Obsidian border: x in [0, 3], y in [0, 4]
    // Interior: x in [1, 2], y in [1, 3] (2 wide, 3 tall)
    let is_obsidian = |x: i32, y: i32| -> bool {
        if x < 0 || x > 3 || y < 0 || y > 4 {
            return false;
        }
        // Border is obsidian, interior is not
        x == 0 || x == 3 || y == 0 || y == 4
    };

    let interior = find_portal_interior(1, 1, is_obsidian);
    assert!(interior.is_some(), "Should detect valid 2x3 portal interior");
    let p = interior.unwrap();
    assert_eq!(p.w, 2);
    assert_eq!(p.h, 3);
    assert_eq!(p.cells.len(), 6);
    assert!(p.cells.contains(&(1, 1)));
    assert!(p.cells.contains(&(2, 3)));

    // Broken frame (missing top obsidian block at (1, 4))
    let is_broken_obsidian = |x: i32, y: i32| -> bool {
        if x == 1 && y == 4 {
            return false; // Broken frame
        }
        is_obsidian(x, y)
    };
    let broken = find_portal_interior(1, 1, is_broken_obsidian);
    assert!(broken.is_none(), "Incomplete obsidian ring cannot light portal");
}

#[test]
fn test_end_dimension_pillars_and_crystals() {
    let mut end = EndDimensionState::new();

    assert_eq!(end.crystals.len(), 10, "10 obsidian pillars with crystals");
    assert_eq!(end.alive_crystals_count(), 10);

    // 2 caged pillars, 8 uncaged pillars
    let caged_count = end.crystals.iter().filter(|c| c.caged).count();
    let uncaged_count = end.crystals.iter().filter(|c| !c.caged).count();
    assert_eq!(caged_count, 2);
    assert_eq!(uncaged_count, 8);

    // Exploding a crystal deals 50 damage
    assert_eq!(end.crystals[0].explode(), 50);
    assert!(!end.crystals[0].alive);
    assert_eq!(end.alive_crystals_count(), 9);

    // Secondary hit on already dead crystal deals 0
    assert_eq!(end.crystals[0].explode(), 0);

    // Exit portal activation
    assert!(!end.exit_portal_active);
    assert!(!end.dragon_egg_spawned);
    end.activate_exit_portal();
    assert!(end.exit_portal_active);
    assert!(end.dragon_egg_spawned);
}

#[test]
fn test_ender_dragon_phases_and_healing() {
    let mut end = EndDimensionState::new();
    let mut dragon = EnderDragonBoss::new(100.0, 100.0);

    assert_eq!(dragon.hp, DRAGON_MAX_HP);
    assert_eq!(DRAGON_MAX_HP, 200);
    assert_eq!(DRAGON_EXHAUST_HP, 50);
    assert_eq!(dragon.phase, DragonPhase::Circle);
    assert!(dragon.is_alive());

    // Place dragon near crystal 0 to test healing beam
    dragon.hp = 150;
    dragon.x = end.crystals[0].x as f32 + 2.0;
    dragon.y = end.crystals[0].y as f32 + 2.0;

    let evt = dragon.tick(0.1, &mut end.crystals, 100.0, 100.0);
    match evt {
        DragonEvent::Healed { crystal_id, amount } => {
            assert_eq!(crystal_id, 0);
            assert_eq!(amount, 1);
            assert_eq!(dragon.hp, 151);
        }
        _ => panic!("Dragon should heal from nearby End Crystal"),
    }

    // Damage dragon down to 50 HP -> triggers Exhausted phase for Pokemon battle!
    dragon.damage(101);
    assert_eq!(dragon.hp, 50);
    assert_eq!(dragon.phase, DragonPhase::Exhausted);

    let evt_exhaust = dragon.tick(0.1, &mut end.crystals, 100.0, 100.0);
    assert!(matches!(evt_exhaust, DragonEvent::Exhausted));
    assert_eq!(dragon.alt, 0.0, "Exhausted dragon lands on exit fountain bedrock");

    // Final defeat: dragon reaches 0 HP
    dragon.damage(50);
    assert_eq!(dragon.hp, 0);
    assert_eq!(dragon.phase, DragonPhase::Dying);

    // Advance 5 seconds of death sequence
    let evt_dead = dragon.tick(5.1, &mut end.crystals, 100.0, 100.0);
    match evt_dead {
        DragonEvent::Defeated { xp } => {
            assert_eq!(xp, 12_000, "Ender Dragon victory awards 12,000 XP");
            assert_eq!(dragon.phase, DragonPhase::Dead);
        }
        _ => panic!("Dragon should transition to Defeated event after death timer"),
    }
}
