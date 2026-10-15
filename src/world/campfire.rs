use serde::{Deserialize, Serialize};
use crate::items::types::ItemRegistry;

pub const CAMPFIRE_COOK_TIME: i32 = 600; // 30 seconds at 20 ticks/sec

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampfireSlot {
    pub id: String,
    pub cook_ticks: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampfireState {
    pub lit: bool,
    pub items: [Option<CampfireSlot>; 4],
}

impl Default for CampfireState {
    fn default() -> Self {
        Self {
            lit: true,
            items: [None, None, None, None],
        }
    }
}

impl CampfireState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a cookable item to the campfire. Returns true if added.
    pub fn add_item(&mut self, id: &str, registry: &ItemRegistry) -> bool {
        if registry.campfire_of(id).is_none() {
            return false;
        }

        for slot in &mut self.items {
            if slot.is_none() {
                *slot = Some(CampfireSlot {
                    id: id.to_string(),
                    cook_ticks: 0,
                });
                return true;
            }
        }
        false
    }

    /// Ticks the campfire. If any items finish cooking, returns their cooked IDs to drop into the world.
    pub fn tick(&mut self, registry: &ItemRegistry) -> Vec<String> {
        let mut finished = Vec::new();

        for slot in &mut self.items {
            if let Some(item) = slot {
                if self.lit {
                    item.cook_ticks += 1;
                    if item.cook_ticks >= CAMPFIRE_COOK_TIME {
                        if let Some(cooked_id) = registry.campfire_of(&item.id) {
                            finished.push(cooked_id.to_string());
                        }
                        *slot = None;
                    }
                } else if item.cook_ticks > 0 {
                    // Heat drains when campfire is unlit
                    item.cook_ticks = (item.cook_ticks - 2).max(0);
                }
            }
        }

        finished
    }

    pub fn extinguish(&mut self) -> bool {
        if self.lit {
            self.lit = false;
            true
        } else {
            false
        }
    }

    pub fn ignite(&mut self) -> bool {
        if !self.lit {
            self.lit = true;
            true
        } else {
            false
        }
    }
}
