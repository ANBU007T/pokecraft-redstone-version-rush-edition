use bevy::prelude::*;
use super::data::DexRegistry;

#[derive(Resource, Default)]
pub struct EncounterClock {
    pub steps: usize,
    pub buff: i32,
    pub prev_zone: Option<String>,
}

impl EncounterClock {
    pub fn reset(&mut self) {
        self.steps = 0;
        self.buff = 0;
    }

    /// Ticks the authentic FireRed encounter clock upon stepping on an encounter tile.
    /// Returns true if a wild battle is triggered.
    pub fn step(&mut self, rate: i32, zone_key: &str, rng_seed: u32) -> bool {
        if rate <= 0 {
            return false;
        }

        let min_steps = if rate >= 80 {
            0
        } else if rate < 10 {
            8
        } else {
            8 - (rate / 10) as usize
        };

        let changed = self.prev_zone.as_deref() != Some(zone_key);
        self.prev_zone = Some(zone_key.to_string());

        let roll100 = (rng_seed % 100) as i32;
        if self.steps < min_steps {
            self.steps += 1;
            if roll100 >= 5 {
                return false;
            }
        }

        if changed && roll100 >= 60 {
            return false;
        }

        let p = (rate * 16 + (self.buff * 16) / 200).min(1600);
        let roll1600 = (rng_seed % 1600) as i32;
        if roll1600 < p {
            self.reset();
            return true;
        }

        self.buff += rate;
        false
    }
}

pub fn roll_wild_encounter(
    map_id: &str,
    kind: &str,
    registry: &DexRegistry,
    seed: u32,
) -> Option<(String, usize)> {
    let map_data = registry.data.wild.get(map_id)?;
    let slot_data = match kind {
        "land" => map_data.land.as_ref()?,
        "water" => map_data.water.as_ref()?,
        _ => return None,
    };

    if slot_data.slots.is_empty() || slot_data.mons.is_empty() {
        return None;
    }

    let total: i32 = slot_data.slots.iter().sum();
    if total <= 0 {
        return None;
    }

    let mut r = (seed as i32) % total;
    let mut chosen_idx = slot_data.mons.len() - 1;
    for (i, &weight) in slot_data.slots.iter().enumerate() {
        r -= weight;
        if r < 0 {
            chosen_idx = i;
            break;
        }
    }

    let (species, min_lv, max_lv) = &slot_data.mons[chosen_idx];
    let level_range = (max_lv - min_lv + 1).max(1);
    let level = min_lv + ((seed as usize / 17) % level_range);

    Some((species.clone(), level))
}
