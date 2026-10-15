use bevy::prelude::*;
use super::data::{Pokemon, heal_mon};

pub const PARTY_MAX: usize = 5; // Steve is the 6th member in PokéCRAFT

#[derive(Resource, Default, Debug, Clone)]
pub struct PlayerParty {
    pub members: Vec<Pokemon>,
}

impl PlayerParty {
    pub fn new() -> Self {
        Self {
            members: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.members.len()
    }

    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.members.len() >= PARTY_MAX
    }

    pub fn add(&mut self, mon: Pokemon) -> bool {
        if self.is_full() {
            return false;
        }
        self.members.push(mon);
        true
    }

    pub fn remove(&mut self, index: usize) -> Option<Pokemon> {
        if index < self.members.len() {
            Some(self.members.remove(index))
        } else {
            None
        }
    }

    pub fn swap(&mut self, a: usize, b: usize) -> bool {
        if a < self.members.len() && b < self.members.len() {
            self.members.swap(a, b);
            true
        } else {
            false
        }
    }

    pub fn heal_all(&mut self) {
        for mon in &mut self.members {
            heal_mon(mon);
        }
    }

    pub fn first_alive(&self) -> Option<usize> {
        self.members.iter().position(|m| m.hp > 0)
    }

    pub fn all_fainted(&self) -> bool {
        self.members.iter().all(|m| m.hp <= 0)
    }
}
