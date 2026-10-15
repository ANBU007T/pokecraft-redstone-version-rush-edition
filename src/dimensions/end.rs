use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct EndCrystal {
    pub id: usize,
    pub x: i32,
    pub y: i32,
    pub alive: bool,
    pub caged: bool,
}

impl EndCrystal {
    pub fn new(id: usize, x: i32, y: i32, caged: bool) -> Self {
        Self {
            id,
            x,
            y,
            alive: true,
            caged,
        }
    }

    /// Explodes crystal when damaged, returns 50 explosion damage dealt
    pub fn explode(&mut self) -> i32 {
        if self.alive {
            self.alive = false;
            50
        } else {
            0
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct EndDimensionState {
    pub center_x: i32,
    pub center_y: i32,
    pub island_radius: f32,
    pub crystals: Vec<EndCrystal>,
    pub exit_portal_active: bool,
    pub dragon_egg_spawned: bool,
}

impl Default for EndDimensionState {
    fn default() -> Self {
        let center_x = 100;
        let center_y = 100;
        let island_radius = 48.0;

        // 10 obsidian pillars arranged in a circle of radius 32 around the fountain
        let mut crystals = Vec::with_capacity(10);
        for i in 0..10 {
            let angle = (i as f32 * std::f32::consts::TAU) / 10.0;
            let px = center_x + (angle.cos() * 32.0).round() as i32;
            let py = center_y + (angle.sin() * 32.0).round() as i32;
            let caged = i == 2 || i == 7; // Authentic: 2 caged pillars, 8 uncaged
            crystals.push(EndCrystal::new(i, px, py, caged));
        }

        Self {
            center_x,
            center_y,
            island_radius,
            crystals,
            exit_portal_active: false,
            dragon_egg_spawned: false,
        }
    }
}

impl EndDimensionState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn alive_crystals_count(&self) -> usize {
        self.crystals.iter().filter(|c| c.alive).count()
    }

    pub fn activate_exit_portal(&mut self) {
        self.exit_portal_active = true;
        self.dragon_egg_spawned = true;
    }
}
