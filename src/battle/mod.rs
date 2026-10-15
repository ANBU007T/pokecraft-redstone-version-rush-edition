use bevy::prelude::*;
use crate::pokemon::data::{Pokemon, MoveDef, SpeciesDef, DexRegistry, is_physical};

const STAGE_MULTIPLIERS: [(i32, i32); 13] = [
    (10, 40), (10, 35), (10, 30), (10, 25), (10, 20), (10, 15),
    (10, 10),
    (15, 10), (20, 10), (25, 10), (30, 10), (35, 10), (40, 10),
];

const ACC_MULTIPLIERS: [(i32, i32); 13] = [
    (33, 100), (36, 100), (43, 100), (50, 100), (60, 100), (75, 100),
    (1, 1),
    (133, 100), (166, 100), (2, 1), (233, 100), (133, 50), (3, 1),
];

const CRIT_STAGES: [usize; 5] = [16, 8, 4, 3, 2];

pub fn stage_apply(val: i32, stage: i32) -> i32 {
    let idx = (stage + 6).clamp(0, 12) as usize;
    let (n, d) = STAGE_MULTIPLIERS[idx];
    (val * n) / d
}

#[derive(Debug, Clone)]
pub struct Battler {
    pub mon: Pokemon,
    pub stages: [i32; 7], // 0: atk, 1: def, 2: spa, 3: spd, 4: spe, 5: acc, 6: eva
    pub conf: i32,
    pub flinch: bool,
    pub recharge: bool,
    pub charge: Option<String>,
    pub invuln: bool,
    pub protect: bool,
    pub protect_chain: usize,
    pub focus: bool,
}

impl Battler {
    pub fn new(mon: Pokemon) -> Self {
        Self {
            mon,
            stages: [0; 7],
            conf: 0,
            flinch: false,
            recharge: false,
            charge: None,
            invuln: false,
            protect: false,
            protect_chain: 0,
            focus: false,
        }
    }

    pub fn stat(&self, idx: usize) -> i32 {
        let base = match idx {
            0 => self.mon.atk,
            1 => self.mon.def,
            2 => self.mon.spa,
            3 => self.mon.spd,
            4 => self.mon.spe,
            _ => 100,
        };
        stage_apply(base, self.stages[idx])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleOutcome {
    Won,
    Lost,
    Ran,
    Caught,
}

#[derive(Debug, Clone)]
pub struct DamageResult {
    pub damage: i32,
    pub crit: bool,
    pub eff: f32,
    pub immune: bool,
}

/// Computes authentic Generation 3 damage formula
pub fn calc_gen3_damage(
    att: &Battler,
    def: &Battler,
    mv: &MoveDef,
    att_sp: &SpeciesDef,
    def_sp: &SpeciesDef,
    registry: &DexRegistry,
    rng_val: usize,
) -> DamageResult {
    let move_type = &mv.move_type;
    let eff = registry.type_multiplier(move_type, &def_sp.types);
    if eff == 0.0 {
        return DamageResult {
            damage: 0,
            crit: false,
            eff: 0.0,
            immune: true,
        };
    }

    // Fixed damage moves
    if mv.effect == "LEVEL_DAMAGE" {
        return DamageResult {
            damage: att.mon.level as i32,
            crit: false,
            eff: 1.0,
            immune: false,
        };
    }
    if mv.effect == "SONICBOOM" {
        return DamageResult {
            damage: 20,
            crit: false,
            eff: 1.0,
            immune: false,
        };
    }
    if mv.effect == "DRAGON_RAGE" {
        return DamageResult {
            damage: 40,
            crit: false,
            eff: 1.0,
            immune: false,
        };
    }

    let phys = is_physical(move_type);
    let crit_stage = ((if mv.effect == "HIGH_CRITICAL" { 1 } else { 0 }) + (if att.focus { 2 } else { 0 })).min(4);
    let crit = (rng_val % CRIT_STAGES[crit_stage]) == 0;

    let mut a_stage = if phys { att.stages[0] } else { att.stages[2] };
    let mut d_stage = if phys { def.stages[1] } else { def.stages[3] };

    // Critical hits ignore negative attack stages and positive defense stages
    if crit {
        if a_stage < 0 {
            a_stage = 0;
        }
        if d_stage > 0 {
            d_stage = 0;
        }
    }

    let a_val = stage_apply(if phys { att.mon.atk } else { att.mon.spa }, a_stage);
    let mut d_val = stage_apply(if phys { def.mon.def } else { def.mon.spd }, d_stage).max(1);

    if mv.effect == "EXPLOSION" {
        d_val = (d_val / 2).max(1);
    }

    let power = mv.power;
    let lv = att.mon.level as i32;

    // Gen 3 formula: ((((2 * Level / 5 + 2) * Attack * Power) / Defense) / 50)
    let mut dmg = (((2 * lv / 5 + 2) * a_val * power) / d_val) / 50;
    if phys && att.mon.status.as_deref() == Some("BRN") {
        dmg /= 2;
    }
    if dmg == 0 {
        dmg = 1;
    }
    dmg += 2;

    if crit {
        dmg *= 2;
    }

    // STAB (Same-Type Attack Bonus): 1.5x
    if move_type != "MYSTERY" && att_sp.types.iter().any(|t| t == move_type) {
        dmg = (dmg * 15) / 10;
    }

    // Type effectiveness rows
    let rows = registry.type_rows(move_type, &def_sp.types);
    for r in rows {
        dmg = (dmg * r) / 10;
    }

    // Random variance: 85% to 100%
    let random_factor = 100 - ((rng_val / 7) % 16) as i32;
    dmg = (dmg * random_factor) / 100;
    if dmg < 1 {
        dmg = 1;
    }

    DamageResult {
        damage: dmg,
        crit,
        eff,
        immune: false,
    }
}

/// Checks whether an attack hits based on accuracy and evasion stages
pub fn check_accuracy(
    att: &Battler,
    def: &Battler,
    mv: &MoveDef,
    rng_val: usize,
) -> bool {
    if mv.acc == 0 || mv.effect == "ALWAYS_HIT" {
        return true;
    }
    let st = (att.stages[5] - def.stages[6]).clamp(-6, 6);
    let idx = (st + 6) as usize;
    let (n, d) = ACC_MULTIPLIERS[idx];
    let calc = (mv.acc * n) / d;
    ((rng_val % 100) as i32) + 1 <= calc
}

/// Generation 3 Poké Ball Catch Formula
/// Returns (caught, number_of_shakes)
pub fn roll_catch(
    def: &Battler,
    def_sp: &SpeciesDef,
    ball_mult_x10: i32,
    rng_seed: usize,
) -> (bool, usize) {
    let max_hp = def.mon.max_hp;
    let cur_hp = def.mon.hp;

    let mut odds = (def_sp.catch_rate * ball_mult_x10) / 10;
    odds = (odds * (max_hp * 3 - cur_hp * 2)) / (max_hp * 3).max(1);

    if def.mon.status.as_deref() == Some("SLP") || def.mon.status.as_deref() == Some("FRZ") {
        odds *= 2;
    } else if def.mon.status.is_some() {
        odds = (odds * 15) / 10;
    }

    if odds > 254 {
        return (true, 4);
    }

    let b = 1048560 / (((16711680 / odds.max(1)) as f64).sqrt().sqrt() as i32).max(1);
    let mut shakes = 0;
    let mut current_seed = rng_seed;
    for _ in 0..4 {
        current_seed = (current_seed.wrapping_mul(1103515245).wrapping_add(12345)) & 0x7fffffff;
        let roll = (current_seed % 65536) as i32;
        if roll < b {
            shakes += 1;
        } else {
            break;
        }
    }

    (shakes == 4, shakes)
}

#[derive(Resource, Default)]
pub struct BattleState {
    pub active: bool,
    pub player: Option<Battler>,
    pub enemy: Option<Battler>,
    pub message_queue: Vec<String>,
    pub current_message: Option<String>,
    pub outcome: Option<BattleOutcome>,
}

impl BattleState {
    pub fn start_wild_battle(&mut self, player_mon: Pokemon, wild_mon: Pokemon) {
        self.active = true;
        self.player = Some(Battler::new(player_mon));
        let enemy_name = wild_mon.name.clone();
        self.enemy = Some(Battler::new(wild_mon));
        self.message_queue.clear();
        self.message_queue.push(format!("Wild {} appeared!", enemy_name));
        self.outcome = None;
    }

    pub fn advance_message(&mut self) -> bool {
        if !self.message_queue.is_empty() {
            self.current_message = Some(self.message_queue.remove(0));
            true
        } else {
            self.current_message = None;
            false
        }
    }
}
