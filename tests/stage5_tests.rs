use pokecraft_bevy::pokemon::data::{
    DexRegistry, calc_stats, moves_at, make_mon, is_physical,
};
use pokecraft_bevy::pokemon::encounters::{EncounterClock, roll_wild_encounter};
use pokecraft_bevy::battle::{
    Battler, calc_gen3_damage, roll_catch,
};
use pokecraft_bevy::npc::{NpcRegistry, DialogueState, interact_with_npc};

fn load_dex() -> DexRegistry {
    let paths = ["assets", "../assets"];
    for p in paths {
        if let Ok(dex) = DexRegistry::load_from_dir(p) {
            return dex;
        }
    }
    panic!("Could not load pokemon.json for Stage 5 tests");
}

fn load_npcs() -> NpcRegistry {
    let paths = ["assets", "../assets"];
    for p in paths {
        if let Ok(reg) = NpcRegistry::load_from_dir(p) {
            return reg;
        }
    }
    panic!("Could not load world.json for Stage 5 NPC tests");
}

#[test]
fn test_dex_registry_and_stats() {
    let dex = load_dex();

    assert!(!dex.data.species.is_empty(), "Species dictionary loaded");
    assert!(!dex.data.moves.is_empty(), "Moves dictionary loaded");
    assert!(!dex.data.types.is_empty(), "Type chart loaded");

    // Test Pikachu stats at Level 50 with 31 IVs
    let pikachu = dex.species("PIKACHU").expect("PIKACHU should exist");
    let ivs = [31; 6];
    let (hp, atk, def, _spa, _spd, spe) = calc_stats(pikachu, 50, &ivs);

    // Pikachu base: 35 HP, 55 Atk, 30 Def, 50 SpA, 40 SpD, 90 Spe
    // HP: ((2 * 35 + 31) * 50) / 100 + 50 + 10 = 50 + 50 + 10 = 110
    assert_eq!(hp, 110);
    // Atk: ((2 * 55 + 31) * 50) / 100 + 5 = 70 + 5 = 75
    assert_eq!(atk, 75);
    // Def: ((2 * 30 + 31) * 50) / 100 + 5 = 45 + 5 = 50
    assert_eq!(def, 50);
    // Spe: ((2 * 90 + 31) * 50) / 100 + 5 = 105 + 5 = 110
    assert_eq!(spe, 110);

    // Test Learnset at level 26 (Pikachu learns THUNDERBOLT at 26)
    let moves = moves_at(pikachu, 26);
    assert!(!moves.is_empty());
    assert!(moves.contains(&"THUNDERBOLT".to_string()));
    assert!(moves.len() <= 4, "Maximum 4 moves retained");
}

#[test]
fn test_gen3_damage_formulas() {
    let dex = load_dex();

    // Setup: Level 50 Charmeleon vs Level 50 Bulbasaur
    let charmeleon = dex.species("CHARMELEON").unwrap();
    let bulbasaur = dex.species("BULBASAUR").unwrap();

    let mon_char = make_mon("CHARMELEON", 50, &dex).unwrap();
    let mon_bulb = make_mon("BULBASAUR", 50, &dex).unwrap();

    let battler_char = Battler::new(mon_char);
    let battler_bulb = Battler::new(mon_bulb);

    // 1. FLAMETHROWER (Fire vs Grass/Poison) -> Super effective (2x)
    let flamethrower = dex.move_def("FLAMETHROWER").unwrap();
    let res = calc_gen3_damage(
        &battler_char,
        &battler_bulb,
        flamethrower,
        charmeleon,
        bulbasaur,
        &dex,
        100,
    );

    assert!(!res.immune);
    assert!(res.eff >= 2.0, "Fire is super effective against Grass");
    assert!(res.damage > 80, "STAB super-effective Flamethrower deals heavy damage");

    // 2. THUNDER_SHOCK vs Ground (Immune -> 0x)
    let geodude = dex.species("GEODUDE").unwrap();
    let mon_geo = make_mon("GEODUDE", 50, &dex).unwrap();
    let battler_geo = Battler::new(mon_geo);
    let thundershock = dex.move_def("THUNDER_SHOCK").unwrap();

    let res_immune = calc_gen3_damage(
        &battler_char,
        &battler_geo,
        thundershock,
        charmeleon,
        geodude,
        &dex,
        100,
    );
    assert!(res_immune.immune, "Electric should be immune against Ground type");
    assert_eq!(res_immune.damage, 0);

    // 3. Physical vs Special classification
    assert!(is_physical("NORMAL"));
    assert!(is_physical("FIGHTING"));
    assert!(!is_physical("FIRE"));
    assert!(!is_physical("WATER"));
}

#[test]
fn test_gen3_catch_formula() {
    let dex = load_dex();
    let caterpie = dex.species("CATERPIE").unwrap();
    let mon_cat = make_mon("CATERPIE", 3, &dex).unwrap();

    // Caterpie has catchRate 255. At full HP with Poke Ball (mult = 10 -> 1.0x):
    // odds > 254 -> guaranteed 4 shakes / instant catch
    let battler_cat = Battler::new(mon_cat);
    let (caught, shakes) = roll_catch(&battler_cat, caterpie, 10, 12345);
    assert!(caught, "Caterpie with catch rate 255 should be caught");
    assert_eq!(shakes, 4);

    // Test with low catch rate boss (ENDER DRAGON catchRate 3)
    let dragon = dex.species("ENDER_DRAGON").unwrap();
    let mut mon_dragon = make_mon("ENDER_DRAGON", 70, &dex).unwrap();
    let battler_dragon = Battler::new(mon_dragon.clone());
    // At full HP with normal poke ball, should not catch easily
    let (caught_dragon, _) = roll_catch(&battler_dragon, dragon, 10, 99999);
    assert!(!caught_dragon, "Full HP Ender Dragon should not be caught with standard Poke Ball");

    // Asleep Ender Dragon at 1 HP with Ultra Ball (mult = 20) has vastly increased odds
    mon_dragon.hp = 1;
    mon_dragon.status = Some("SLP".to_string());
    let battler_dragon_low = Battler::new(mon_dragon);
    let (_, shakes_low) = roll_catch(&battler_dragon_low, dragon, 20, 12345);
    assert!(shakes_low > 0, "Weakened asleep Ender Dragon should wobble the ball");
}

#[test]
fn test_npc_registry_and_nurse_joy() {
    let npcs = load_npcs();
    assert_eq!(npcs.npcs.len(), 473, "All 473 NPCs loaded from world.json");

    // Find Nurse Joy
    let nurse = npcs.npcs.iter().find(|n| n.role == "nurse").expect("Nurse Joy should exist");
    assert_eq!(nurse.role, "nurse");

    let dex = load_dex();
    let mut pikachu = make_mon("PIKACHU", 25, &dex).unwrap();
    pikachu.hp = 10;
    pikachu.status = Some("PAR".to_string());
    pikachu.moves[0].pp = 0;

    let mut party = vec![pikachu];
    let mut dialogue = DialogueState::default();

    interact_with_npc(nurse, &mut dialogue, &mut party);

    assert!(dialogue.active);
    assert_eq!(dialogue.speaker.as_deref(), Some("NURSE JOY"));
    assert_eq!(party[0].hp, party[0].max_hp, "Nurse Joy heals HP to max");
    assert!(party[0].status.is_none(), "Nurse Joy cures paralysis");
    assert_eq!(party[0].moves[0].pp, party[0].moves[0].max_pp, "Nurse Joy restores PP");

    // Dialogue page progression
    assert_eq!(dialogue.current_page, 0);
    assert!(dialogue.advance());
    assert_eq!(dialogue.current_page, 1);
    assert!(dialogue.advance());
    assert_eq!(dialogue.current_page, 2);
    assert!(!dialogue.advance(), "Dialogue finishes after page 2");
    assert!(!dialogue.active);
}

#[test]
fn test_wild_encounter_clock_and_roll() {
    let dex = load_dex();
    let mut clock = EncounterClock::default();

    // Map: Route 1 ("MAP_ROUTE1")
    let rate = dex.data.wild.get("MAP_ROUTE1")
        .and_then(|w| w.land.as_ref())
        .map(|l| l.rate)
        .unwrap_or(20);

    // Simulate 30 steps through Route 1 tall grass
    let mut triggered = false;
    for step_num in 0..30 {
        let seed = (step_num * 1337 + 42) as u32;
        if clock.step(rate, "MAP_ROUTE1", seed) {
            triggered = true;
            break;
        }
    }
    assert!(triggered, "Encounter clock should trigger wild battle within 30 steps");

    // Roll wild encounter on Route 1
    let rolled = roll_wild_encounter("MAP_ROUTE1", "land", &dex, 98765);
    assert!(rolled.is_some());
    let (species, level) = rolled.unwrap();
    assert!(species == "PIDGEY" || species == "RATTATA");
    assert!(level >= 2 && level <= 5, "Route 1 encounters are level 2-5");
}
