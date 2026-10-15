use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use bevy::prelude::*;

#[derive(Debug, Clone, Deserialize)]
pub struct BlockSpec {
    #[serde(default)]
    pub look: Option<String>,
    pub hard: f32,
    pub tool: Option<String>,
    pub need: Option<usize>,
    pub sound: Option<String>,
    pub drop: Option<String>,
    pub flush: Option<bool>,
    pub walk: Option<bool>,
    pub plant: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ToolSpec {
    pub kind: String,
    pub tier: usize,
    pub speed: f32,
    pub dur: usize,
    pub dmg: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FoodSpec {
    pub hunger: i32,
    pub sat: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ArmorSpec {
    pub slot: String,
    pub points: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ItemDef {
    pub name: String,
    #[serde(default = "default_stack")]
    pub stack: usize,
    #[serde(rename = "iconIndex")]
    pub icon_index: usize,
    pub block: Option<BlockSpec>,
    pub tool: Option<ToolSpec>,
    pub food: Option<FoodSpec>,
    pub armor: Option<ArmorSpec>,
    pub fuel: Option<i32>,
}

fn default_stack() -> usize {
    64
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecipeDef {
    pub out: String,
    #[serde(default = "default_count")]
    pub n: usize,
    pub rows: Option<Vec<String>>,
    pub key: Option<HashMap<String, String>>,
    pub ins: Option<Vec<String>>,
}

fn default_count() -> usize {
    1
}

#[derive(Debug, Clone, Deserialize)]
pub struct ItemsData {
    pub items: HashMap<String, ItemDef>,
    pub blocks: HashMap<String, usize>,
    #[serde(default)]
    pub recipes: Vec<RecipeDef>,
    #[serde(default)]
    pub tags: HashMap<String, Vec<String>>,
    #[serde(default)]
    pub smelt: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub campfire: HashMap<String, String>,
}

#[derive(Resource)]
pub struct ItemRegistry {
    pub data: ItemsData,
}

impl ItemRegistry {
    pub fn load_from_dir<P: AsRef<Path>>(base_path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let json_path = base_path.as_ref().join("mc/items.json");
        let content = fs::read_to_string(&json_path)?;
        let data: ItemsData = serde_json::from_str(&content)?;
        Ok(Self { data })
    }

    pub fn get(&self, id: &str) -> Option<&ItemDef> {
        self.data.items.get(id)
    }

    pub fn block_look(&self, key: &str) -> Option<usize> {
        self.data.blocks.get(key).copied()
    }

    pub fn smelt_of(&self, id: &str) -> Option<(&str, f32)> {
        let val = self.data.smelt.get(id)?;
        if let Some(arr) = val.as_array() {
            let out_id = arr.get(0)?.as_str()?;
            let xp = arr.get(1).and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
            Some((out_id, xp))
        } else if let Some(s) = val.as_str() {
            Some((s, 0.0))
        } else {
            None
        }
    }

    pub fn campfire_of(&self, id: &str) -> Option<&str> {
        self.data.campfire.get(id).map(|s| s.as_str())
    }

    pub fn fuel_of(&self, id: &str) -> i32 {
        self.get(id).and_then(|it| it.fuel).unwrap_or(0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemStack {
    pub id: String,
    pub count: usize,
    pub dmg: usize,
}

impl ItemStack {
    pub fn new(id: impl Into<String>, count: usize) -> Self {
        Self {
            id: id.into(),
            count,
            dmg: 0,
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct Inventory {
    /// 36 slots: 0..8 Hotbar, 9..35 Main Inventory
    pub slots: [Option<ItemStack>; 36],
    /// 4 slots: 0 Helmet, 1 Chestplate, 2 Leggings, 3 Boots
    pub armor: [Option<ItemStack>; 4],
    /// Active hotbar index: 0..8
    pub sel: usize,
}

impl Default for Inventory {
    fn default() -> Self {
        let mut inv = Self {
            slots: std::array::from_fn(|_| None),
            armor: std::array::from_fn(|_| None),
            sel: 0,
        };

        // Give Steve authentic starter tools in hotbar for testing & play
        inv.slots[0] = Some(ItemStack::new("iron_pickaxe", 1));
        inv.slots[1] = Some(ItemStack::new("iron_axe", 1));
        inv.slots[2] = Some(ItemStack::new("iron_shovel", 1));
        inv.slots[3] = Some(ItemStack::new("iron_sword", 1));
        inv.slots[4] = Some(ItemStack::new("oak_log", 32));
        inv.slots[5] = Some(ItemStack::new("cobblestone", 64));
        inv.slots[6] = Some(ItemStack::new("apple", 16));
        inv.slots[7] = Some(ItemStack::new("crafting_table", 4));
        inv.slots[8] = Some(ItemStack::new("white_bed", 1));

        inv
    }
}

impl Inventory {
    pub fn held(&self) -> Option<&ItemStack> {
        self.slots[self.sel].as_ref()
    }

    pub fn held_mut(&mut self) -> Option<&mut ItemStack> {
        self.slots[self.sel].as_mut()
    }

    pub fn add(&mut self, id: &str, mut count: usize, max_stack: usize) -> usize {
        if max_stack > 1 {
            // First pass: stack into existing non-damaged stacks
            for slot in self.slots.iter_mut() {
                if count == 0 {
                    break;
                }
                if let Some(s) = slot {
                    if s.id == id && s.dmg == 0 && s.count < max_stack {
                        let take = (max_stack - s.count).min(count);
                        s.count += take;
                        count -= take;
                    }
                }
            }
        }

        // Second pass: fill empty slots
        for slot in self.slots.iter_mut() {
            if count == 0 {
                break;
            }
            if slot.is_none() {
                let take = max_stack.min(count);
                *slot = Some(ItemStack {
                    id: id.to_string(),
                    count: take,
                    dmg: 0,
                });
                count -= take;
            }
        }

        count // Returns remaining count that did not fit
    }

    pub fn remove(&mut self, id: &str, mut count: usize) -> bool {
        for slot in self.slots.iter_mut().rev() {
            if count == 0 {
                break;
            }
            if let Some(s) = slot {
                if s.id == id {
                    let take = s.count.min(count);
                    s.count -= take;
                    count -= take;
                    if s.count == 0 {
                        *slot = None;
                    }
                }
            }
        }
        count == 0
    }

    pub fn use_held(&mut self) {
        if let Some(s) = &mut self.slots[self.sel] {
            if s.count > 1 {
                s.count -= 1;
            } else {
                self.slots[self.sel] = None;
            }
        }
    }

    pub fn wear_held(&mut self, amount: usize, dur: usize) -> bool {
        let mut broke = false;
        if let Some(s) = &mut self.slots[self.sel] {
            s.dmg += amount;
            if s.dmg >= dur {
                broke = true;
            }
        }
        if broke {
            self.slots[self.sel] = None;
        }
        broke
    }
}

pub fn break_info(spec: &BlockSpec, held_tool: Option<&ToolSpec>) -> (f32, bool) {
    if spec.hard < 0.0 {
        return (f32::INFINITY, false);
    }
    if spec.hard == 0.0 {
        return (1.0, true);
    }

    let mut speed = 1.0;
    let right_tool = if let (Some(t), Some(req_tool)) = (held_tool, &spec.tool) {
        t.kind == *req_tool
    } else {
        false
    };

    if right_tool {
        if let Some(t) = held_tool {
            speed = t.speed;
        }
    }

    let mut harvest = true;
    if let Some(need_tier) = spec.need {
        if let Some(t) = held_tool {
            harvest = right_tool && t.tier >= need_tier;
        } else {
            harvest = false;
        }
    }

    let divisor = if harvest { 30.0 } else { 100.0 };
    let damage_per_tick = speed / spec.hard / divisor;
    let ticks = (1.0 / damage_per_tick).ceil().max(1.0);
    (ticks, harvest)
}
