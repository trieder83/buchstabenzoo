//! Wandering animals (GAME-ANIMALS "Animal states", ANIM-008…012): wander areas of hiding
//! places and enclosures (GAME-LAYOUT "Hiding places", "Enclosure features and wandering at
//! home", proposal Q-085) and the seeded wander steps.
//!
//! An area is a set of grid cells with a class (`Land`, `Water`, `Ramp`). Animals move from
//! cell centre to cell centre over 4-neighbours; land and water connect only through ramp
//! cells. Everything is deterministic: pauses and targets come from the game RNG.

use std::collections::{BTreeMap, VecDeque};

use glam::{IVec2, Vec2};

use crate::level::{cell_center, cell_of, CellKind, ElementType, HidingPlaceData, Level, Surface};
use crate::rng::Pcg32;

/// Wander speed (GAME-ANIMALS: ≈ 0.5 m/s; ANIM-008: never above 0.6 m/s).
pub const WANDER_SPEED: f32 = 0.5;
/// Pause between two wander walks, seconds (GAME-ANIMALS: 6–15 s, seeded).
pub const PAUSE_RANGE_S: (f32, f32) = (6.0, 15.0);
/// An escaped animal stops and faces the player within this distance (ANIM-009).
pub const STOP_NEAR_PLAYER_M: f32 = 3.0;
/// *Proposal (Q-097):* an escaped animal that is out of the player's reach (e.g. far out in
/// the pond) comes towards her when she is within this distance, up to the cell of its
/// wander area nearest to her, so the child can always show the food.
pub const NOTICE_PLAYER_M: f32 = 5.0;
/// Of 10 new home wander targets, this many are water cells for animals with `water` in
/// `home_wander_on` (proposal Q-085: hippos spend most of their time in the pool).
pub const WATER_TARGETS_OF_10: u32 = 7;

/// Class of a wander cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AreaCell {
    Land,
    Water,
    /// Pool entry ramp: connects land and water.
    Ramp,
}

impl AreaCell {
    /// Whether an animal may step between two neighbouring cells of these classes.
    fn connects(self, other: AreaCell) -> bool {
        self == other || self == AreaCell::Ramp || other == AreaCell::Ramp
    }
}

/// Cells an animal may wander on.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WanderArea {
    cells: BTreeMap<(i32, i32), AreaCell>,
}

impl WanderArea {
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    pub fn contains(&self, c: IVec2) -> bool {
        self.cells.contains_key(&(c.x, c.y))
    }

    pub fn class(&self, c: IVec2) -> Option<AreaCell> {
        self.cells.get(&(c.x, c.y)).copied()
    }

    /// All cells in a fixed (sorted) order.
    pub fn cells(&self) -> impl Iterator<Item = (IVec2, AreaCell)> + '_ {
        self.cells.iter().map(|(&(x, z), &k)| (IVec2::new(x, z), k))
    }

    fn insert(&mut self, c: IVec2, k: AreaCell) {
        self.cells.insert((c.x, c.y), k);
    }

    /// Keeps only the cells 4-connected to `start` (respecting ramp connections).
    fn keep_connected(&mut self, start: IVec2) {
        let reach = self.reachable(start);
        self.cells.retain(|&(x, z), _| reach.contains_key(&(x, z)));
    }

    /// Breadth-first search from `start` over the area; `prev` links for every reached cell.
    fn reachable(&self, start: IVec2) -> BTreeMap<(i32, i32), Option<(i32, i32)>> {
        let mut prev = BTreeMap::new();
        if !self.contains(start) {
            return prev;
        }
        prev.insert((start.x, start.y), None);
        let mut queue = VecDeque::from([start]);
        while let Some(c) = queue.pop_front() {
            let kc = self.class(c).expect("in area");
            for d in [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y] {
                let n = c + d;
                let Some(kn) = self.class(n) else { continue };
                if !kc.connects(kn) || prev.contains_key(&(n.x, n.y)) {
                    continue;
                }
                prev.insert((n.x, n.y), Some((c.x, c.y)));
                queue.push_back(n);
            }
        }
        prev
    }

    /// Shortest 4-neighbour route inside the area from `from` to `to` (excluding `from`,
    /// including `to`), or `None`.
    pub fn route(&self, from: IVec2, to: IVec2) -> Option<Vec<IVec2>> {
        let prev = self.reachable(from);
        let mut k = (to.x, to.y);
        prev.get(&k)?;
        let mut out = Vec::new();
        while let Some(&Some(p)) = prev.get(&k) {
            out.push(IVec2::new(k.0, k.1));
            k = p;
        }
        out.reverse();
        Some(out)
    }

    /// The area cell nearest to a point (ties: sorted order).
    pub fn nearest(&self, p: Vec2) -> Option<IVec2> {
        self.cells().map(|(c, _)| c).min_by(|a, b| {
            cell_center(*a)
                .distance_squared(p)
                .total_cmp(&cell_center(*b).distance_squared(p))
        })
    }
}

/// Wander area of a hiding place (GAME-LEVEL-1 "Hiding places"): cells whose centre is within
/// `wander_radius_m` of the spot centre, whose surface matches `wander_on`, that lie inside
/// the place's `rect` (proposal Q-085) and are 4-connected to the spot. `grass` = walkable
/// non-path cell not blocked by a prop; `water` = cell of a water landmark (`water_kinds`,
/// default pond/river/stream); `cave` = walkable path cell of kind `cave`.
pub fn hiding_area(level: &Level, place: &HidingPlaceData) -> WanderArea {
    let data = &level.data;
    let grid = level.grid();
    let spot = place.spot_cell();
    let centre = place.spot();
    let kinds: Vec<&str> = if place.water_kinds.is_empty() {
        vec!["pond", "river", "stream"]
    } else {
        place.water_kinds.iter().map(String::as_str).collect()
    };
    let is_water = |c: IVec2| {
        data.elements.iter().any(|e| {
            e.ty == ElementType::Landmark
                && e.kind.as_deref().is_some_and(|k| kinds.contains(&k))
                && e.rect.contains(c)
        })
    };
    let is_cave = |c: IVec2| {
        grid.is_walkable(c, false)
            && data.elements.iter().any(|e| {
                e.ty == ElementType::Path && e.kind.as_deref() == Some("cave") && e.rect.contains(c)
            })
    };
    let (class, ok): (AreaCell, Box<dyn Fn(IVec2) -> bool>) = match place.wander_on.as_str() {
        "water" => (AreaCell::Water, Box::new(is_water)),
        "cave" => (AreaCell::Land, Box::new(is_cave)),
        _ => (
            AreaCell::Land,
            Box::new(|c: IVec2| {
                grid.kind(c) == CellKind::Walkable(Surface::Grass) && !grid.is_prop_blocked(c)
            }),
        ),
    };
    let mut area = WanderArea::default();
    for c in place.rect.cells() {
        if cell_center(c).distance(centre) <= place.wander_radius_m + 1e-4 && ok(c) {
            area.insert(c, class);
        }
    }
    // the spot always belongs to the area (the animal starts there)
    area.insert(spot, area.class(spot).unwrap_or(class));
    area.keep_connected(spot);
    area
}

/// Home wander area of an enclosure element (GAME-LAYOUT "Enclosure features and wandering
/// at home"): `grass` = enclosure cells that are not gate cells, not covered by an enclosure
/// feature (pools, reserved building areas) and whose centre is not blocked by a prop;
/// `water` = the cells of the enclosure's pool (its ramp cells connect water and grass).
/// Only cells connected to the grass cell inside the gate are kept.
pub fn home_area(level: &Level, enclosure: usize) -> WanderArea {
    let data = &level.data;
    let grid = level.grid();
    let enc = &data.elements[enclosure];
    let surfaces = enc.home_surfaces();
    let grass = surfaces.contains(&"grass");
    let water = surfaces.contains(&"water");
    let gate = enc.gate;
    let features: Vec<_> = data.features_of(&enc.id).collect();
    let mut area = WanderArea::default();
    for c in enc.rect.cells() {
        if gate.is_some_and(|g| g.contains(c)) {
            continue;
        }
        match features.iter().find(|f| f.rect.contains(c)) {
            Some(f) if f.is_pool() => {
                if water {
                    let k = if f.is_ramp(c) {
                        AreaCell::Ramp
                    } else {
                        AreaCell::Water
                    };
                    area.insert(c, k);
                }
            }
            Some(_) => {}
            None => {
                if grass && !grid.is_prop_blocked(c) {
                    area.insert(c, AreaCell::Land);
                }
            }
        }
    }
    if let Some(start) = home_entry(level, enclosure).and_then(|c| {
        if area.contains(c) {
            Some(c)
        } else {
            area.nearest(cell_center(c))
        }
    }) {
        area.keep_connected(start);
    }
    area
}

/// The enclosure cell just inside the gate (where an arriving animal steps first).
pub fn home_entry(level: &Level, enclosure: usize) -> Option<IVec2> {
    let enc = &level.data.elements[enclosure];
    let g = enc.gate?;
    let r = enc.rect;
    let first = IVec2::new(g.x, g.z);
    // inward direction: from the side the gate lies on towards the inside
    let inward = if g.w == 1 && g.x == r.x {
        IVec2::X
    } else if g.w == 1 && g.x == r.x + r.w - 1 {
        IVec2::NEG_X
    } else if g.d == 1 && g.z == r.z {
        IVec2::Y
    } else {
        IVec2::NEG_Y
    };
    Some(first + inward)
}

/// Depth factor 0…1 of an animal standing at `p`: 1 in water (a water landmark or a pool,
/// ramp cells blend along the ramp), 0 on land. The presentation sinks swimming animals by
/// this factor (GAME-LEVEL-1 "Hippo enclosure pool": only eyes, ears and back showing).
pub fn water_depth(level: &Level, p: Vec2) -> f32 {
    let c = cell_of(p);
    let data = &level.data;
    for f in data.enclosure_features.iter().filter(|f| f.is_pool()) {
        if !f.rect.contains(c) {
            continue;
        }
        if let Some(r) = f.ramp.filter(|r| r.contains(c)) {
            // ramp side = where the ramp starts (on land); depth grows away from it
            let t = match f.ramp_side.as_deref() {
                Some("+x") => (r.x + r.w) as f32 - p.x,
                Some("-z") => p.y - r.z as f32,
                Some("+z") => (r.z + r.d) as f32 - p.y,
                _ => p.x - r.x as f32,
            } / match f.ramp_side.as_deref() {
                Some("-z") | Some("+z") => r.d as f32,
                _ => r.w as f32,
            };
            return t.clamp(0.0, 1.0);
        }
        return 1.0;
    }
    let water = data.elements.iter().any(|e| {
        e.ty == ElementType::Landmark
            && matches!(e.kind.as_deref(), Some("pond" | "river" | "stream"))
            && e.rect.contains(c)
    });
    if water {
        1.0
    } else {
        0.0
    }
}

/// Wander state of one animal (saved, GAME-SAVE / ANIM-011).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Wander {
    /// Seconds left in the current pause.
    pub pause_s: f32,
    /// Remaining cells to walk through (cell centres); empty = resting.
    pub route: Vec<IVec2>,
}

/// Random pause length (seeded).
pub fn draw_pause(rng: &mut Pcg32) -> f32 {
    let u = rng.next_u32() as f32 / u32::MAX as f32;
    PAUSE_RANGE_S.0 + (PAUSE_RANGE_S.1 - PAUSE_RANGE_S.0) * u
}

/// Whether all 8 neighbours of a cell belong to the area (an animal resting there does not
/// poke its head through a fence or over the pool rim).
fn is_interior(area: &WanderArea, c: IVec2) -> bool {
    let Some(k) = area.class(c) else {
        return false;
    };
    (-1..=1).all(|dz| {
        (-1..=1).all(|dx| {
            area.class(c + IVec2::new(dx, dz))
                .is_some_and(|n| n == k || n == AreaCell::Ramp || k == AreaCell::Ramp)
        })
    })
}

/// Picks the next wander target in `area` (not the current cell). With `water_bias` 7 of 10
/// targets are water cells when the area has any (proposal Q-085). With `interior` targets
/// are cells whose 8 neighbours are in the area where possible (home: away from fences and
/// the pool rim; at a hiding place: away from its fences and banks).
pub fn draw_target(
    area: &WanderArea,
    here: IVec2,
    water_bias: bool,
    interior: bool,
    rng: &mut Pcg32,
) -> Option<IVec2> {
    let pick = |filter: &dyn Fn(AreaCell) -> bool, rng: &mut Pcg32| -> Option<IVec2> {
        let all: Vec<IVec2> = area
            .cells()
            .filter(|&(c, k)| c != here && filter(k))
            .map(|(c, _)| c)
            .collect();
        let inner: Vec<IVec2> = if interior {
            all.iter()
                .copied()
                .filter(|&c| is_interior(area, c))
                .collect()
        } else {
            Vec::new()
        };
        let list = if inner.is_empty() { all } else { inner };
        (!list.is_empty()).then(|| list[rng.below(list.len() as u32) as usize])
    };
    if water_bias && area.cells().any(|(_, k)| k == AreaCell::Water) {
        let want_water = rng.below(10) < WATER_TARGETS_OF_10;
        let class = if want_water {
            AreaCell::Water
        } else {
            AreaCell::Land
        };
        if let Some(c) = pick(&|k| k == class, rng) {
            return Some(c);
        }
    }
    pick(&|_| true, rng)
}

/// Moves `pos` along `route` by at most `max_step` metres; returns the distance moved and
/// the direction of the last move. Reached cells are removed from the route.
pub fn follow_route(pos: &mut Vec2, route: &mut Vec<IVec2>, max_step: f32) -> (f32, Vec2) {
    let mut left = max_step;
    let mut dir = Vec2::ZERO;
    let mut moved = 0.0;
    while left > 1e-6 {
        let Some(&next) = route.first() else { break };
        let to = cell_center(next) - *pos;
        let d = to.length();
        if d <= left {
            *pos = cell_center(next);
            if d > 1e-6 {
                dir = to / d;
            }
            left -= d;
            moved += d;
            route.remove(0);
        } else {
            dir = to / d;
            *pos += dir * left;
            moved += left;
            left = 0.0;
        }
    }
    (moved, dir)
}
