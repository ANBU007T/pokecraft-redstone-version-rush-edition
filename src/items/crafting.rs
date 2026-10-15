use std::collections::HashMap;
use super::types::{ItemRegistry, ItemStack, RecipeDef, Inventory};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CraftResult {
    pub id: String,
    pub count: usize,
}

pub fn fits(want: &str, id: &str, registry: &ItemRegistry) -> bool {
    if want == id {
        return true;
    }
    if let Some(tag_name) = want.strip_prefix('#') {
        if let Some(members) = registry.data.tags.get(tag_name) {
            return members.iter().any(|m| m == id);
        }
    }
    false
}

/// Trims a 2D grid of items (represented as a 1D slice of size w*h) down to its bounding box.
/// Returns a 2D matrix of Option<String> of trimmed rows and columns.
pub fn trim(grid: &[Option<ItemStack>], w: usize, h: usize) -> Option<Vec<Vec<Option<String>>>> {
    let mut x0 = w;
    let mut y0 = h;
    let mut x1 = 0;
    let mut y1 = 0;
    let mut found = false;

    for y in 0..h {
        for x in 0..w {
            if grid[y * w + x].is_some() {
                found = true;
                x0 = x0.min(x);
                y0 = y0.min(y);
                x1 = x1.max(x);
                y1 = y1.max(y);
            }
        }
    }

    if !found {
        return None;
    }

    let mut rows = Vec::new();
    for y in y0..=y1 {
        let mut r = Vec::new();
        for x in x0..=x1 {
            r.push(grid[y * w + x].as_ref().map(|s| s.id.clone()));
        }
        rows.push(r);
    }
    Some(rows)
}

pub fn match_shaped(
    rec: &RecipeDef,
    rows: &[Vec<Option<String>>],
    registry: &ItemRegistry,
) -> bool {
    let r_rows = match &rec.rows {
        Some(r) => r,
        None => return false,
    };
    let key = match &rec.key {
        Some(k) => k,
        None => return false,
    };

    if r_rows.len() != rows.len() {
        return false;
    }

    // Check normal and mirrored horizontally
    for mirror in [false, true] {
        let mut ok = true;
        for (y, line) in r_rows.iter().enumerate() {
            let line_chars: Vec<char> = line.chars().collect();
            if line_chars.len() != rows[y].len() {
                ok = false;
                break;
            }

            for (x, row_val) in rows[y].iter().enumerate() {
                let ch = if mirror {
                    line_chars[line_chars.len() - 1 - x]
                } else {
                    line_chars[x]
                };

                let want: Option<&String> = if ch == ' ' {
                    None
                } else {
                    key.get(&ch.to_string())
                };

                match (want, row_val) {
                    (None, None) => {}
                    (Some(w), Some(h)) => {
                        if !fits(w, h, registry) {
                            ok = false;
                            break;
                        }
                    }
                    _ => {
                        ok = false;
                        break;
                    }
                }
            }

            if !ok {
                break;
            }
        }

        if ok {
            return true;
        }
    }

    false
}

pub fn match_shapeless(
    rec: &RecipeDef,
    grid: &[Option<ItemStack>],
    registry: &ItemRegistry,
) -> bool {
    let ins = match &rec.ins {
        Some(i) => i,
        None => return false,
    };

    let mut have: Vec<String> = grid
        .iter()
        .filter_map(|s| s.as_ref().map(|st| st.id.clone()))
        .collect();
    have.sort();

    let mut want: Vec<String> = ins.clone();
    want.sort();

    if have.len() != want.len() {
        return false;
    }

    let has_tags = want.iter().any(|k| k.starts_with('#'));
    if !has_tags {
        return have == want;
    }

    // Backtracking match for tags
    let mut used = vec![false; have.len()];
    fn backtrack(
        k: usize,
        want: &[String],
        have: &[String],
        used: &mut [bool],
        registry: &ItemRegistry,
    ) -> bool {
        if k == want.len() {
            return true;
        }
        for i in 0..have.len() {
            if !used[i] && fits(&want[k], &have[i], registry) {
                used[i] = true;
                if backtrack(k + 1, want, have, used, registry) {
                    return true;
                }
                used[i] = false;
            }
        }
        false
    }

    backtrack(0, &want, &have, &mut used, registry)
}

/// Evaluates a crafting grid of size w*h and returns the recipe output if matched
pub fn craft(
    grid: &[Option<ItemStack>],
    w: usize,
    h: usize,
    registry: &ItemRegistry,
) -> Option<CraftResult> {
    let rows = trim(grid, w, h)?;
    let row_count = rows.len();
    let col_count = rows[0].len();
    if row_count > h || col_count > w {
        return None;
    }

    for r in &registry.data.recipes {
        if let Some(r_rows) = &r.rows {
            let max_r_width = r_rows.iter().map(|l| l.chars().count()).max().unwrap_or(0);
            if r_rows.len() <= h && max_r_width <= w && match_shaped(r, &rows, registry) {
                return Some(CraftResult {
                    id: r.out.clone(),
                    count: r.n,
                });
            }
        } else if let Some(ins) = &r.ins {
            if ins.len() <= w * h && match_shapeless(r, grid, registry) {
                return Some(CraftResult {
                    id: r.out.clone(),
                    count: r.n,
                });
            }
        }
    }

    None
}

/// Consumes 1 item from each non-empty slot in the crafting grid.
/// Empty buckets are left behind if a bucket of water/milk is consumed.
pub fn consume_craft_grid(grid: &mut [Option<ItemStack>]) {
    for slot in grid.iter_mut() {
        if let Some(st) = slot {
            if st.id == "water_bucket" || st.id == "milk_bucket" || st.id == "lava_bucket" {
                *slot = Some(ItemStack::new("bucket", 1));
            } else if st.count > 1 {
                st.count -= 1;
            } else {
                *slot = None;
            }
        }
    }
}

/// Finds all craftable recipes from current inventory for given grid size (2 or 3)
pub fn craftable<'a>(
    inv: &Inventory,
    size: usize,
    registry: &'a ItemRegistry,
) -> Vec<&'a RecipeDef> {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for slot in &inv.slots {
        if let Some(st) = slot {
            *counts.entry(&st.id).or_insert(0) += st.count;
        }
    }

    let have_count = |want: &str| -> usize {
        if let Some(tag_name) = want.strip_prefix('#') {
            if let Some(members) = registry.data.tags.get(tag_name) {
                let mut sum = 0;
                for m in members {
                    sum += counts.get(m.as_str()).copied().unwrap_or(0);
                }
                return sum;
            }
        }
        counts.get(want).copied().unwrap_or(0)
    };

    let mut out = Vec::new();

    for r in &registry.data.recipes {
        let mut need: HashMap<&str, usize> = HashMap::new();
        if let Some(r_rows) = &r.rows {
            let max_r_width = r_rows.iter().map(|l| l.chars().count()).max().unwrap_or(0);
            if r_rows.len() > size || max_r_width > size {
                continue;
            }
            let key = match &r.key {
                Some(k) => k,
                None => continue,
            };
            for line in r_rows {
                for ch in line.chars() {
                    if ch != ' ' {
                        if let Some(item_key) = key.get(&ch.to_string()) {
                            *need.entry(item_key.as_str()).or_insert(0) += 1;
                        }
                    }
                }
            }
        } else if let Some(ins) = &r.ins {
            if ins.len() > size * size {
                continue;
            }
            for i in ins {
                *need.entry(i.as_str()).or_insert(0) += 1;
            }
        }

        let mut ok = true;
        for (item_req, qty) in need {
            if have_count(item_req) < qty {
                ok = false;
                break;
            }
        }

        if ok {
            out.push(r);
        }
    }

    out
}
