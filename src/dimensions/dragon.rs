use bevy::prelude::*;
use super::end::EndCrystal;

pub const DRAGON_MAX_HP: i32 = 200;
pub const DRAGON_EXHAUST_HP: i32 = 50;
pub const DRAGON_HEAL_RANGE: f32 = 16.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DragonPhase {
    Circle,
    Strafe,
    Charge,
    Perch,
    Exhausted,
    Dying,
    Dead,
}

#[derive(Debug, Clone)]
pub enum DragonEvent {
    None,
    Fireball { x: f32, y: f32, tx: f32, ty: f32 },
    Breath { x: f32, y: f32 },
    Healed { crystal_id: usize, amount: i32 },
    Exhausted,
    Defeated { xp: i32 },
}

#[derive(Resource, Debug, Clone)]
pub struct EnderDragonBoss {
    pub hp: i32,
    pub x: f32,
    pub y: f32,
    pub alt: f32,
    pub phase: DragonPhase,
    pub phase_timer: f32,
    pub healing_crystal: Option<usize>,
    pub center_x: f32,
    pub center_y: f32,
    pub orbit_angle: f32,
}

impl Default for EnderDragonBoss {
    fn default() -> Self {
        Self {
            hp: DRAGON_MAX_HP,
            x: 124.0,
            y: 102.0,
            alt: 60.0,
            phase: DragonPhase::Circle,
            phase_timer: 0.0,
            healing_crystal: None,
            center_x: 100.0,
            center_y: 100.0,
            orbit_angle: 0.0,
        }
    }
}

impl EnderDragonBoss {
    pub fn new(center_x: f32, center_y: f32) -> Self {
        Self {
            center_x,
            center_y,
            x: center_x + 24.0,
            y: center_y + 2.0,
            ..default()
        }
    }

    pub fn is_alive(&self) -> bool {
        self.phase != DragonPhase::Dead && self.phase != DragonPhase::Dying
    }

    pub fn damage(&mut self, amount: i32) {
        if !self.is_alive() {
            return;
        }
        self.hp = (self.hp - amount).max(0);
        if self.hp <= 0 {
            self.phase = DragonPhase::Dying;
            self.phase_timer = 0.0;
        } else if self.hp <= DRAGON_EXHAUST_HP && self.phase != DragonPhase::Exhausted {
            self.phase = DragonPhase::Exhausted;
            self.phase_timer = 0.0;
        }
    }

    pub fn tick(
        &mut self,
        dt: f32,
        crystals: &mut [EndCrystal],
        player_x: f32,
        player_y: f32,
    ) -> DragonEvent {
        if self.phase == DragonPhase::Dead {
            return DragonEvent::None;
        }

        self.phase_timer += dt;

        // Dying sequence
        if self.phase == DragonPhase::Dying {
            if self.phase_timer >= 5.0 {
                self.phase = DragonPhase::Dead;
                return DragonEvent::Defeated { xp: 12_000 };
            }
            return DragonEvent::None;
        }

        // Exhausted phase (perched for Pokemon turn-based battle)
        if self.phase == DragonPhase::Exhausted {
            self.alt = 0.0;
            self.x = self.center_x;
            self.y = self.center_y;
            return DragonEvent::Exhausted;
        }

        // Crystal healing logic
        let mut nearest_crystal = None;
        let mut min_dist = DRAGON_HEAL_RANGE;
        for c in crystals.iter() {
            if c.alive {
                let dist = ((c.x as f32 - self.x).powi(2) + (c.y as f32 - self.y).powi(2)).sqrt();
                if dist < min_dist {
                    min_dist = dist;
                    nearest_crystal = Some(c.id);
                }
            }
        }

        self.healing_crystal = nearest_crystal;
        if let Some(cid) = self.healing_crystal {
            if self.hp < DRAGON_MAX_HP {
                self.hp = (self.hp + 1).min(DRAGON_MAX_HP);
                return DragonEvent::Healed {
                    crystal_id: cid,
                    amount: 1,
                };
            }
        }

        // Phase state machine
        match self.phase {
            DragonPhase::Circle => {
                self.orbit_angle += 0.5 * dt;
                self.x = self.center_x + self.orbit_angle.cos() * 28.0;
                self.y = self.center_y + self.orbit_angle.sin() * 28.0;
                self.alt = 50.0;

                if self.phase_timer >= 10.0 {
                    self.phase = DragonPhase::Strafe;
                    self.phase_timer = 0.0;
                }
            }
            DragonPhase::Strafe => {
                // Fly across and shoot fireball
                let progress = self.phase_timer / 4.0;
                self.x = self.center_x - 30.0 + progress * 60.0;
                self.alt = 40.0;

                if self.phase_timer >= 2.0 && self.phase_timer - dt < 2.0 {
                    return DragonEvent::Fireball {
                        x: self.x,
                        y: self.y,
                        tx: player_x,
                        ty: player_y,
                    };
                }

                if self.phase_timer >= 4.0 {
                    self.phase = DragonPhase::Perch;
                    self.phase_timer = 0.0;
                }
            }
            DragonPhase::Perch => {
                // Descend to center fountain and breathe breath clouds
                self.x = self.center_x;
                self.y = self.center_y;
                self.alt = (40.0 - self.phase_timer * 10.0).max(0.0);

                if self.phase_timer >= 3.0 && self.phase_timer - dt < 3.0 {
                    return DragonEvent::Breath {
                        x: self.center_x,
                        y: self.center_y,
                    };
                }

                if self.phase_timer >= 6.0 {
                    self.phase = DragonPhase::Charge;
                    self.phase_timer = 0.0;
                }
            }
            DragonPhase::Charge => {
                // Swoop towards player position
                let dx = player_x - self.x;
                let dy = player_y - self.y;
                let dist = (dx * dx + dy * dy).sqrt().max(1.0);
                self.x += (dx / dist) * 15.0 * dt;
                self.y += (dy / dist) * 15.0 * dt;
                self.alt = 20.0;

                if self.phase_timer >= 5.0 {
                    self.phase = DragonPhase::Circle;
                    self.phase_timer = 0.0;
                }
            }
            _ => {}
        }

        DragonEvent::None
    }
}
