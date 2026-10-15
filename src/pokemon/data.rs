use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::Deserialize;
use bevy::prelude::*;

#[derive(Debug, Clone, Deserialize)]
pub struct MoveDef {
    pub name: String,
    pub effect: String,
    pub power: i32,
    #[serde(rename = "type")]
    pub move_type: String,
    pub acc: i32,
    pub pp: i32,
    pub chance: i32,
    pub priority: i32,
    pub target: String,
    pub contact: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SpeciesDef {
    pub name: String,
    pub base: Vec<i32>, // [hp, atk, def, spa, spd, spe]
    pub types: Vec<String>,
    pub growth: String,
    #[serde(rename = "catchRate")]
    pub catch_rate: i32,
    #[serde(rename = "expYield")]
    pub exp_yield: i32,
    pub learnset: Vec<(usize, String)>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WildSlot {
    pub rate: i32,
    pub slots: Vec<i32>,
    pub mons: Vec<(String, usize, usize)>, // (species, min_level, max_level)
}

#[derive(Debug, Clone, Deserialize)]
pub struct WildMap {
    pub land: Option<WildSlot>,
    pub water: Option<WildSlot>,
    pub fishing: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DexData {
    pub exp: HashMap<String, Vec<i32>>,
    pub moves: HashMap<String, MoveDef>,
    pub species: HashMap<String, SpeciesDef>,
    pub types: Vec<(String, String, i32, bool)>, // (att_type, def_type, multiplier_x10, foresight)
    pub wild: HashMap<String, WildMap>,
}

#[derive(Resource)]
pub struct DexRegistry {
    pub data: DexData,
}

impl DexRegistry {
    pub fn load_from_dir<P: AsRef<Path>>(base_path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let json_path = base_path.as_ref().join("data/pokemon.json");
        let content = fs::read_to_string(&json_path)?;
        let data: DexData = serde_json::from_str(&content)?;
        Ok(Self { data })
    }

    pub fn species(&self, key: &str) -> Option<&SpeciesDef> {
        self.data.species.get(key)
    }

    pub fn move_def(&self, key: &str) -> Option<&MoveDef> {
        self.data.moves.get(key)
    }

    pub fn type_rows(&self, move_type: &str, def_types: &[String]) -> Vec<i32> {
        let mut out = Vec::new();
        for (a, d, mul, foresight) in &self.data.types {
            if *foresight {
                continue;
            }
            if a == move_type && def_types.iter().any(|dt| dt == d) {
                out.push(*mul);
            }
        }
        out
    }

    pub fn type_multiplier(&self, move_type: &str, def_types: &[String]) -> f32 {
        let rows = self.type_rows(move_type, def_types);
        let mut m = 1.0f32;
        for mul in rows {
            m *= (mul as f32) / 10.0;
        }
        m
    }

    pub fn exp_for(&self, growth: &str, level: usize) -> i32 {
        if let Some(table) = self.data.exp.get(growth) {
            let idx = level.clamp(1, 100);
            if idx < table.len() {
                return table[idx];
            }
        }
        0
    }
}

pub fn is_physical(move_type: &str) -> bool {
    matches!(
        move_type,
        "NORMAL"
            | "FIGHTING"
            | "FLYING"
            | "POISON"
            | "GROUND"
            | "ROCK"
            | "BUG"
            | "GHOST"
            | "STEEL"
            | "MYSTERY"
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PokemonMove {
    pub id: String,
    pub pp: i32,
    pub max_pp: i32,
}

#[derive(Debug, Clone)]
pub struct Pokemon {
    pub uid: u64,
    pub species: String,
    pub name: String,
    pub level: usize,
    pub exp: i32,
    pub ivs: [i32; 6], // [hp, atk, def, spa, spd, spe]
    pub max_hp: i32,
    pub hp: i32,
    pub atk: i32,
    pub def: i32,
    pub spa: i32,
    pub spd: i32,
    pub spe: i32,
    pub moves: Vec<PokemonMove>,
    pub status: Option<String>,
    pub sleep: i32,
    pub shiny: bool,
    pub ball: String,
    pub is_steve: bool,
}

pub fn calc_stats(sp: &SpeciesDef, level: usize, ivs: &[i32; 6]) -> (i32, i32, i32, i32, i32, i32) {
    let b = &sp.base;
    let lv = level as i32;
    let max_hp = ((2 * b[0] + ivs[0]) * lv) / 100 + lv + 10;
    let st = |i: usize| -> i32 { ((2 * b[i] + ivs[i]) * lv) / 100 + 5 };
    (max_hp, st(1), st(2), st(3), st(4), st(5))
}

pub fn moves_at(sp: &SpeciesDef, level: usize) -> Vec<String> {
    let mut known = Vec::new();
    for (lv, m) in &sp.learnset {
        if *lv > level {
            break;
        }
        if let Some(pos) = known.iter().position(|x| x == m) {
            known.remove(pos);
        }
        known.push(m.clone());
    }
    if known.len() > 4 {
        known.drain(0..known.len() - 4);
    }
    known
}

static mut NEXT_UID: u64 = 1;

pub fn make_mon(key: &str, level: usize, registry: &DexRegistry) -> Option<Pokemon> {
    let sp = registry.species(key)?;
    let ivs = [
        rand_iv(),
        rand_iv(),
        rand_iv(),
        rand_iv(),
        rand_iv(),
        rand_iv(),
    ];
    let (max_hp, atk, def, spa, spd, spe) = calc_stats(sp, level, &ivs);
    let moves_list = moves_at(sp, level);
    let moves = moves_list
        .into_iter()
        .map(|m_id| {
            let pp = registry.move_def(&m_id).map(|d| d.pp).unwrap_or(35);
            PokemonMove {
                id: m_id,
                pp,
                max_pp: pp,
            }
        })
        .collect();

    let exp = registry.exp_for(&sp.growth, level);
    let uid = unsafe {
        let id = NEXT_UID;
        NEXT_UID += 1;
        id
    };

    Some(Pokemon {
        uid,
        species: key.to_string(),
        name: sp.name.clone(),
        level,
        exp,
        ivs,
        max_hp,
        hp: max_hp,
        atk,
        def,
        spa,
        spd,
        spe,
        moves,
        status: None,
        sleep: 0,
        shiny: false,
        ball: "poke".to_string(),
        is_steve: false,
    })
}

fn rand_iv() -> i32 {
    let uid = unsafe { NEXT_UID };
    ((uid.wrapping_mul(1103515245).wrapping_add(12345) / 65536) % 32) as i32
}

pub fn heal_mon(mon: &mut Pokemon) {
    mon.hp = mon.max_hp;
    mon.status = None;
    mon.sleep = 0;
    for m in &mut mon.moves {
        m.pp = m.max_pp;
    }
}
