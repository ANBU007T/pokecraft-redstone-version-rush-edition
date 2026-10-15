use bevy::prelude::*;
use super::types::ItemRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PocketType {
    Items,
    KeyItems,
    PokeBalls,
    Berries,
}

pub fn classify_item(id: &str) -> PocketType {
    if id.ends_with("_ball") || id == "poke_ball" {
        PocketType::PokeBalls
    } else if id.ends_with("_berry") || id == "berry" {
        PocketType::Berries
    } else if id == "bicycle" || id == "town_map" || id.ends_with("_rod") || id.ends_with("_key") {
        PocketType::KeyItems
    } else {
        PocketType::Items
    }
}

#[derive(Resource, Debug, Clone)]
pub struct PlayerBag {
    pub money: i32,
}

impl Default for PlayerBag {
    fn default() -> Self {
        Self {
            money: 3000, // Starter funds (3,000 Yen)
        }
    }
}

impl PlayerBag {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn can_afford(&self, price: i32) -> bool {
        self.money >= price
    }

    pub fn spend_money(&mut self, amount: i32) -> bool {
        if self.can_afford(amount) {
            self.money -= amount;
            true
        } else {
            false
        }
    }

    pub fn add_money(&mut self, amount: i32) {
        self.money = (self.money + amount).min(999_999);
    }

    pub fn sale_price(&self, id: &str, registry: &ItemRegistry) -> i32 {
        if let Some(_item_def) = registry.get(id) {
            // Authentic half-price sell value
            let base_price = if id == "poke_ball" { 200 } else if id == "potion" { 300 } else { 100 };
            base_price / 2
        } else {
            0
        }
    }
}
