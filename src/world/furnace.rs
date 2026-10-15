use serde::{Deserialize, Serialize};
use crate::items::types::{ItemRegistry, ItemStack};

pub const FURNACE_COOK_TIME: i32 = 200; // 10 seconds at 20 ticks/sec

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FurnaceState {
    pub input: Option<ItemStack>,
    pub fuel: Option<ItemStack>,
    pub output: Option<ItemStack>,
    pub burn: i32,
    pub burn_max: i32,
    pub cook: i32,
    pub xp: f32,
}

impl Default for FurnaceState {
    fn default() -> Self {
        Self {
            input: None,
            fuel: None,
            output: None,
            burn: 0,
            burn_max: 0,
            cook: 0,
            xp: 0.0,
        }
    }
}

impl FurnaceState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Ticks the furnace by 1 tick (1/20s). Returns true if the furnace state changed.
    pub fn tick(&mut self, registry: &ItemRegistry) -> bool {
        let mut changed = false;

        let smelt_recipe = self.input.as_ref().and_then(|inp| registry.smelt_of(&inp.id));
        let can_output = if let Some((out_id, _)) = smelt_recipe {
            let max_stack = registry.get(out_id).map(|d| d.stack).unwrap_or(64);
            match &self.output {
                None => true,
                Some(out) => out.id == out_id && out.count < max_stack,
            }
        } else {
            false
        };

        if self.burn > 0 {
            self.burn -= 1;
            changed = true;
        }

        // Ignite new fuel if burnt out and ready to smelt
        if self.burn <= 0 && can_output {
            if let Some(fuel_stack) = &mut self.fuel {
                let fuel_ticks = registry.fuel_of(&fuel_stack.id);
                if fuel_ticks > 0 {
                    self.burn = fuel_ticks;
                    self.burn_max = fuel_ticks;
                    changed = true;

                    if fuel_stack.id == "lava_bucket" {
                        *fuel_stack = ItemStack::new("bucket", 1);
                    } else if fuel_stack.count > 1 {
                        fuel_stack.count -= 1;
                    } else {
                        self.fuel = None;
                    }
                }
            }
        }

        // Cooking process
        if self.burn > 0 && can_output {
            let (out_id, xp_gain) = smelt_recipe.unwrap();
            self.cook += 1;
            changed = true;

            if self.cook >= FURNACE_COOK_TIME {
                self.cook = 0;

                // Consume 1 input
                if let Some(inp) = &mut self.input {
                    if inp.count > 1 {
                        inp.count -= 1;
                    } else {
                        self.input = None;
                    }
                }

                // Produce output
                if let Some(out) = &mut self.output {
                    out.count += 1;
                } else {
                    self.output = Some(ItemStack::new(out_id, 1));
                }

                self.xp += xp_gain;
            }
        } else if self.cook > 0 {
            self.cook = (self.cook - 2).max(0);
            changed = true;
        }

        changed
    }

    /// Collects the output from the furnace and retrieves accumulated XP
    pub fn take_output(&mut self) -> (Option<ItemStack>, f32) {
        let out = self.output.take();
        let xp = self.xp;
        self.xp = 0.0;
        (out, xp)
    }

    pub fn is_lit(&self) -> bool {
        self.burn > 0
    }
}
