use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::Deserialize;
use bevy::prelude::*;
use crate::player::movement::Direction;

#[derive(Debug, Clone, Deserialize)]
pub struct OutdoorBounds {
    pub w: usize,
    pub h: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TownLand {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Town {
    pub id: String,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub land: Option<TownLand>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MapDef {
    pub id: String,
    pub mapsec: Option<String>,
    pub name: String,
    pub r#type: Option<String>,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub kind: String,
    #[serde(default)]
    pub room: bool,
    #[serde(default)]
    pub show: Option<bool>,
    pub music: Option<String>,
    pub flash: Option<bool>,
    pub parent: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DoorDef {
    pub style: String,
    pub name: String,
    pub mid: i32,
    pub pair: i32,
    pub closed: u16,
    pub open: u16,
    pub anim: usize,
    pub sound: String,
    pub cells: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WarpDef {
    pub x: i32,
    pub y: i32,
    pub map: String,
    pub trig: String,
    pub to: Option<usize>,
    pub dest: Option<String>,
    pub ax: Option<i32>,
    pub ay: Option<i32>,
    pub adir: Option<String>,
    pub amove: Option<String>,
    pub door: Option<usize>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WorldMetadata {
    pub w: usize,
    pub h: usize,
    pub cols: u32,
    pub outdoor: OutdoorBounds,
    pub towns: Vec<Town>,
    pub maps: Vec<MapDef>,
    #[serde(default)]
    pub doors: Vec<DoorDef>,
    #[serde(default)]
    pub warps: Vec<WarpDef>,
    pub entries: Vec<serde_json::Value>,
}

#[derive(Resource)]
pub struct WorldData {
    pub meta: WorldMetadata,
    pub grid: Vec<u16>,
    pub ground: Vec<u16>,
    pub col: Vec<u8>,
    pub beh: Vec<u8>,
    pub zone: Vec<u8>,
    pub area: Vec<u16>,
    pub prot: Vec<u8>,
    pub has_top: Vec<bool>,
    // Fast lookup tables
    pub warp_by_cell: HashMap<(i32, i32), WarpDef>,
    pub door_by_closed: HashMap<u16, DoorDef>,
    pub door_by_open: HashMap<u16, DoorDef>,
}

impl WorldData {
    pub fn load_from_dir<P: AsRef<Path>>(base_path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let base = base_path.as_ref();
        let world_json_path = base.join("world/world.json");
        let grid_bin_path = base.join("world/grid.bin");
        let ground_bin_path = base.join("world/ground.bin");
        let col_bin_path = base.join("world/col.bin");
        let beh_bin_path = base.join("world/beh.bin");
        let zone_bin_path = base.join("world/zone.bin");
        let area_bin_path = base.join("world/area.bin");
        let prot_bin_path = base.join("world/prot.bin");

        let json_str = fs::read_to_string(&world_json_path)?;
        let meta: WorldMetadata = serde_json::from_str(&json_str)?;

        let grid_bytes = fs::read(&grid_bin_path)?;
        let grid: Vec<u16> = grid_bytes
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect();

        let ground_bytes = fs::read(&ground_bin_path)?;
        let ground: Vec<u16> = ground_bytes
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect();

        let col = fs::read(&col_bin_path)?;
        let beh = fs::read(&beh_bin_path)?;
        let zone = fs::read(&zone_bin_path)?;
        let prot = fs::read(&prot_bin_path)?;

        let area_bytes = fs::read(&area_bin_path)?;
        let area: Vec<u16> = area_bytes
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect();

        let mut has_top = Vec::with_capacity(meta.entries.len());
        for entry in &meta.entries {
            let is_top = if let Some(arr) = entry.as_array() {
                arr.get(6).and_then(|v| v.as_i64()).unwrap_or(0) != 0
            } else {
                false
            };
            has_top.push(is_top);
        }

        let mut warp_by_cell = HashMap::new();
        for warp in &meta.warps {
            warp_by_cell.insert((warp.x, warp.y), warp.clone());
        }

        let mut door_by_closed = HashMap::new();
        let mut door_by_open = HashMap::new();
        for door in &meta.doors {
            door_by_closed.insert(door.closed, door.clone());
            door_by_open.insert(door.open, door.clone());
        }

        Ok(Self {
            meta,
            grid,
            ground,
            col,
            beh,
            zone,
            area,
            prot,
            has_top,
            warp_by_cell,
            door_by_closed,
            door_by_open,
        })
    }

    #[inline]
    pub fn is_inside(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.meta.w && (y as usize) < self.meta.h
    }

    #[inline]
    pub fn is_outdoor(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.meta.outdoor.w && (y as usize) < self.meta.outdoor.h
    }

    #[inline]
    pub fn is_void(&self, x: i32, y: i32) -> bool {
        if !self.is_inside(x, y) {
            return true;
        }
        let idx = (y as usize) * self.meta.w + (x as usize);
        self.zone.get(idx).copied().unwrap_or(2) >= 2
    }

    #[inline]
    pub fn is_solid(&self, x: i32, y: i32) -> bool {
        if !self.is_inside(x, y) {
            return true;
        }
        let idx = (y as usize) * self.meta.w + (x as usize);
        if self.zone.get(idx).copied().unwrap_or(2) >= 2 {
            return true;
        }
        self.col.get(idx).copied().unwrap_or(1) == 1
    }

    #[inline]
    pub fn jump_dir(&self, x: i32, y: i32) -> Option<Direction> {
        if !self.is_inside(x, y) {
            return None;
        }
        let idx = (y as usize) * self.meta.w + (x as usize);
        match self.beh.get(idx).copied().unwrap_or(0) {
            0x38 => Some(Direction::Right),
            0x39 => Some(Direction::Left),
            0x3A => Some(Direction::Up),
            0x3B => Some(Direction::Down),
            _ => None,
        }
    }

    #[inline]
    pub fn tile_at(&self, x: i32, y: i32) -> u16 {
        if self.is_inside(x, y) {
            let idx = (y as usize) * self.meta.w + (x as usize);
            self.grid.get(idx).copied().unwrap_or(0)
        } else {
            0
        }
    }

    #[inline]
    pub fn has_top_at(&self, x: i32, y: i32) -> bool {
        let t = self.tile_at(x, y);
        self.has_top.get(t as usize).copied().unwrap_or(false)
    }

    #[inline]
    pub fn warp_at(&self, x: i32, y: i32) -> Option<&WarpDef> {
        self.warp_by_cell.get(&(x, y))
    }

    #[inline]
    pub fn door_at(&self, x: i32, y: i32) -> Option<&DoorDef> {
        if !self.is_inside(x, y) {
            return None;
        }
        let idx = (y as usize) * self.meta.w + (x as usize);
        if self.prot.get(idx).copied().unwrap_or(0) == 2 {
            let tile = self.tile_at(x, y);
            self.door_by_closed.get(&tile).or_else(|| self.door_by_open.get(&tile))
        } else {
            None
        }
    }

    #[inline]
    pub fn door_closed(&self, x: i32, y: i32) -> Option<&DoorDef> {
        let door = self.door_at(x, y)?;
        let tile = self.tile_at(x, y);
        if tile == door.closed {
            Some(door)
        } else {
            None
        }
    }

    #[inline]
    pub fn map_at(&self, x: i32, y: i32) -> Option<&MapDef> {
        if !self.is_inside(x, y) {
            return None;
        }
        let idx = (y as usize) * self.meta.w + (x as usize);
        let a = self.area.get(idx).copied().unwrap_or(0);
        if a > 0 {
            self.meta.maps.get((a - 1) as usize)
        } else {
            None
        }
    }
}
