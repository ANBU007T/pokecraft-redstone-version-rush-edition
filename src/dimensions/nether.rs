use bevy::prelude::*;
use std::collections::HashSet;

pub const NETHER_SCALE: i32 = 8; // 8:1 coordinate ratio between Overworld and Nether

pub fn overworld_to_nether(x: i32, y: i32) -> (i32, i32) {
    (x / NETHER_SCALE, y / NETHER_SCALE)
}

pub fn nether_to_overworld(x: i32, y: i32) -> (i32, i32) {
    (x * NETHER_SCALE, y * NETHER_SCALE)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortalInterior {
    pub x: i32,
    pub y: i32,
    pub w: usize,
    pub h: usize,
    pub cells: Vec<(i32, i32)>,
}

/// Validates whether a rectangular interior of floor is enclosed by obsidian frames (2x3 to 21x21)
pub fn find_portal_interior<F>(
    start_x: i32,
    start_y: i32,
    is_obsidian: F,
) -> Option<PortalInterior>
where
    F: Fn(i32, i32) -> bool,
{
    if is_obsidian(start_x, start_y) {
        return None;
    }

    let mut seen = HashSet::new();
    let mut q = vec![(start_x, start_y)];
    seen.insert((start_x, start_y));

    let mut x0 = start_x;
    let mut x1 = start_x;
    let mut y0 = start_y;
    let mut y1 = start_y;

    let mut cells = Vec::new();

    while let Some((cx, cy)) = q.pop() {
        cells.push((cx, cy));
        x0 = x0.min(cx);
        x1 = x1.max(cx);
        y0 = y0.min(cy);
        y1 = y1.max(cy);

        if x1 - x0 >= 21 || y1 - y0 >= 21 {
            return None; // Exceeds Minecraft's maximum portal size (21x21)
        }

        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let nx = cx + dx;
            let ny = cy + dy;

            if is_obsidian(nx, ny) {
                continue; // Frame border
            }

            if seen.insert((nx, ny)) {
                q.push((nx, ny));
            }
        }
    }

    let iw = (x1 - x0 + 1) as usize;
    let ih = (y1 - y0 + 1) as usize;

    // Must be a complete rectangle of at least 2x3 or 3x2 interior
    if cells.len() != iw * ih || iw.min(ih) < 2 || iw.max(ih) < 3 {
        return None;
    }

    // Verify all border cells surrounding the rectangle are obsidian frames
    for x in (x0 - 1)..=(x1 + 1) {
        if !is_obsidian(x, y0 - 1) || !is_obsidian(x, y1 + 1) {
            return None;
        }
    }
    for y in y0..=y1 {
        if !is_obsidian(x0 - 1, y) || !is_obsidian(x1 + 1, y) {
            return None;
        }
    }

    Some(PortalInterior {
        x: x0,
        y: y0,
        w: iw,
        h: ih,
        cells,
    })
}
