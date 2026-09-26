//! Grid navigation: flood fill (LAYOUT-001/002), walking times (LAYOUT-L1-005) and paths for
//! following animals (RESC-006 "walk around obstacles").
//!
//! Movement graph: 8-neighbour, diagonal step = √2 m, no corner cutting (a diagonal step
//! needs both orthogonal neighbours walkable).

use std::cmp::Ordering;
use std::collections::BinaryHeap;

use glam::IVec2;

use crate::level::{Grid, Surface};

const NEIGHBOURS: [(i32, i32); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

/// Walkable neighbours of `c` with their step length in metres.
pub fn neighbours(
    grid: &Grid,
    c: IVec2,
    allow_gates: bool,
) -> impl Iterator<Item = (IVec2, f32)> + '_ {
    neighbours_with(c, move |n| grid.is_walkable(n, allow_gates))
}

/// Neighbours of `c` accepted by `ok` (same rules as [`neighbours`]).
pub fn neighbours_with<F: Fn(IVec2) -> bool>(
    c: IVec2,
    ok: F,
) -> impl Iterator<Item = (IVec2, f32)> {
    NEIGHBOURS.iter().filter_map(move |&(dx, dz)| {
        let n = c + IVec2::new(dx, dz);
        if !ok(n) {
            return None;
        }
        if dx != 0 && dz != 0 {
            let a = ok(c + IVec2::new(dx, 0));
            let b = ok(c + IVec2::new(0, dz));
            if !(a && b) {
                return None;
            }
            return Some((n, std::f32::consts::SQRT_2));
        }
        Some((n, 1.0))
    })
}

/// Cells reachable from `start` over walkable cells (indexed like the grid).
pub fn flood_fill(grid: &Grid, start: IVec2, allow_gates: bool) -> Vec<bool> {
    let mut seen = vec![false; grid.len()];
    let Some(s) = grid.index(start) else {
        return seen;
    };
    if !grid.is_walkable(start, allow_gates) {
        return seen;
    }
    seen[s] = true;
    let mut stack = vec![start];
    while let Some(c) = stack.pop() {
        for (n, _) in neighbours(grid, c, allow_gates) {
            let k = grid.index(n).expect("walkable cells are in bounds");
            if !seen[k] {
                seen[k] = true;
                stack.push(n);
            }
        }
    }
    seen
}

/// Cost model for [`dijkstra`].
#[derive(Debug, Clone, Copy)]
pub enum Cost {
    /// Metres.
    Distance,
    /// Seconds with the given speeds (m/s); a step costs half its length at each end cell's speed.
    Time { path_speed: f32, grass_speed: f32 },
}

#[derive(Copy, Clone, PartialEq)]
struct Node(f32, usize);
impl Eq for Node {}
impl Ord for Node {
    fn cmp(&self, o: &Self) -> Ordering {
        o.0.total_cmp(&self.0).then_with(|| o.1.cmp(&self.1))
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}

/// Result of a multi-source Dijkstra.
pub struct Field {
    pub cost: Vec<f32>,
    pub prev: Vec<Option<usize>>,
}

/// Multi-source shortest paths over walkable cells.
pub fn dijkstra(grid: &Grid, sources: &[IVec2], cost: Cost, allow_gates: bool) -> Field {
    dijkstra_with(grid, sources, cost, &|c| grid.is_walkable(c, allow_gates))
}

/// Multi-source shortest paths over the cells accepted by `ok`.
pub fn dijkstra_with(
    grid: &Grid,
    sources: &[IVec2],
    cost: Cost,
    ok: &dyn Fn(IVec2) -> bool,
) -> Field {
    let mut dist = vec![f32::INFINITY; grid.len()];
    let mut prev = vec![None; grid.len()];
    let mut heap = BinaryHeap::new();
    for &s in sources {
        if let Some(k) = grid.index(s) {
            if ok(s) {
                dist[k] = 0.0;
                heap.push(Node(0.0, k));
            }
        }
    }
    let inv_speed = |c: IVec2| match cost {
        Cost::Distance => 1.0,
        Cost::Time {
            path_speed,
            grass_speed,
        } => match grid.surface(c) {
            Some(Surface::Path) => 1.0 / path_speed,
            _ => 1.0 / grass_speed,
        },
    };
    while let Some(Node(d, k)) = heap.pop() {
        if d > dist[k] {
            continue;
        }
        let c = grid.cell_at(k);
        let ic = inv_speed(c);
        for (n, len) in neighbours_with(c, ok) {
            let nk = grid.index(n).expect("in bounds");
            let nd = d + len * 0.5 * (ic + inv_speed(n));
            if nd < dist[nk] {
                dist[nk] = nd;
                prev[nk] = Some(k);
                heap.push(Node(nd, nk));
            }
        }
    }
    Field { cost: dist, prev }
}

/// Minimum cost from any of `from` to any of `to`.
pub fn min_cost(grid: &Grid, from: &[IVec2], to: &[IVec2], cost: Cost) -> f32 {
    let f = dijkstra(grid, from, cost, false);
    to.iter()
        .filter_map(|&c| grid.index(c).map(|k| f.cost[k]))
        .fold(f32::INFINITY, f32::min)
}

/// Shortest cell path from `from` to `to` (both included), or `None` if unreachable.
/// Avoids cells whose centre is blocked by a prop (GAME-PLAYER §7), except `from`/`to`.
pub fn find_path(grid: &Grid, from: IVec2, to: IVec2, allow_gates: bool) -> Option<Vec<IVec2>> {
    // Search backwards from the target so `prev` walks from `from` towards `to`.
    let ok = |c: IVec2| {
        grid.is_walkable(c, allow_gates) && (c == from || c == to || !grid.is_prop_blocked(c))
    };
    let f = dijkstra_with(grid, &[to], Cost::Distance, &ok);
    let mut k = grid.index(from)?;
    if !f.cost[k].is_finite() {
        return None;
    }
    let mut path = vec![from];
    while let Some(p) = f.prev[k] {
        k = p;
        path.push(grid.cell_at(k));
    }
    Some(path)
}

/// Scripted walking to a level point along a grid path (debug / e2e scripted player, PROD-POC
/// M4): gives joystick input each frame; the player still moves with the normal speed and
/// collision rules.
#[derive(Debug, Clone)]
pub struct Autopilot {
    pub target: glam::Vec2,
    path: Vec<IVec2>,
}

impl Autopilot {
    /// Distance at which the target counts as reached (m).
    pub const ARRIVED_M: f32 = 0.12;

    pub fn new(target: glam::Vec2) -> Self {
        Self {
            target,
            path: Vec::new(),
        }
    }

    /// Joystick input (unit length) towards the next waypoint, or `None` when arrived or
    /// when no path exists.
    pub fn input(&mut self, grid: &Grid, pos: glam::Vec2, allow_gates: bool) -> Option<glam::Vec2> {
        use crate::level::{cell_center, cell_of};
        if pos.distance(self.target) <= Self::ARRIVED_M {
            return None;
        }
        let here = cell_of(pos);
        let goal = cell_of(self.target);
        let aim = if here == goal {
            self.target
        } else {
            if !self.path.contains(&here) {
                self.path = find_path(grid, here, goal, allow_gates)?;
            }
            let k = self.path.iter().position(|&c| c == here)?;
            match self.path.get(k + 1) {
                Some(&n) if n != goal => cell_center(n),
                _ => self.target,
            }
        };
        let d = aim - pos;
        (d.length() > 1e-4).then(|| d.normalize())
    }
}
