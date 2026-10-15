use bevy::prelude::*;
use super::data::Pokemon;
use super::party::PlayerParty;

pub const BOX_CAPACITY: usize = 30;
pub const TOTAL_BOXES: usize = 14;

#[derive(Debug, Clone)]
pub struct PcBox {
    pub name: String,
    pub slots: [Option<Pokemon>; BOX_CAPACITY],
}

impl Default for PcBox {
    fn default() -> Self {
        Self {
            name: "BOX 1".to_string(),
            slots: std::array::from_fn(|_| None),
        }
    }
}

impl PcBox {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            slots: std::array::from_fn(|_| None),
        }
    }

    pub fn count(&self) -> usize {
        self.slots.iter().filter(|s| s.is_some()).count()
    }

    pub fn is_full(&self) -> bool {
        self.count() >= BOX_CAPACITY
    }

    pub fn deposit(&mut self, mon: Pokemon) -> bool {
        for slot in &mut self.slots {
            if slot.is_none() {
                *slot = Some(mon);
                return true;
            }
        }
        false
    }

    pub fn withdraw(&mut self, slot_idx: usize) -> Option<Pokemon> {
        if slot_idx < BOX_CAPACITY {
            self.slots[slot_idx].take()
        } else {
            None
        }
    }
}

#[derive(Resource)]
pub struct PcStorage {
    pub current_box: usize,
    pub boxes: Vec<PcBox>,
}

impl Default for PcStorage {
    fn default() -> Self {
        let mut boxes = Vec::with_capacity(TOTAL_BOXES);
        for i in 1..=TOTAL_BOXES {
            boxes.push(PcBox::new(format!("BOX {}", i)));
        }
        Self {
            current_box: 0,
            boxes,
        }
    }
}

impl PcStorage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn current(&self) -> &PcBox {
        &self.boxes[self.current_box]
    }

    pub fn current_mut(&mut self) -> &mut PcBox {
        &mut self.boxes[self.current_box]
    }

    pub fn next_box(&mut self) {
        self.current_box = (self.current_box + 1) % self.boxes.len();
    }

    pub fn prev_box(&mut self) {
        if self.current_box == 0 {
            self.current_box = self.boxes.len() - 1;
        } else {
            self.current_box -= 1;
        }
    }

    pub fn total_stored(&self) -> usize {
        self.boxes.iter().map(|b| b.count()).sum()
    }

    /// Stores a Pokemon into the current box, or overflows to subsequent boxes if full
    pub fn deposit_anywhere(&mut self, mon: Pokemon) -> bool {
        // Try current box first
        if self.current_mut().deposit(mon.clone()) {
            return true;
        }
        // Try other boxes
        for b in &mut self.boxes {
            if b.deposit(mon.clone()) {
                return true;
            }
        }
        false
    }

    /// Deposits a Pokemon from the party into PC storage
    pub fn deposit_from_party(&mut self, party: &mut PlayerParty, party_idx: usize) -> bool {
        if party_idx >= party.len() {
            return false;
        }
        let mon = party.members[party_idx].clone();
        if self.deposit_anywhere(mon) {
            party.remove(party_idx);
            true
        } else {
            false
        }
    }

    /// Withdraws a Pokemon from the current PC box into the player's party
    pub fn withdraw_to_party(&mut self, party: &mut PlayerParty, slot_idx: usize) -> bool {
        if party.is_full() {
            return false;
        }
        if let Some(mon) = self.current_mut().withdraw(slot_idx) {
            party.add(mon);
            true
        } else {
            false
        }
    }
}
