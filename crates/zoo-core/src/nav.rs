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
    NEIGHBOURS.iter().filter_map(move |&(dx, dz)| {
        let n = c + IVec2::new(dx, dz);
        if !grid.is_walkable(n, allow_gates) {
            return None;
        }
        if dx != 0 && dz != 0 {
            let a = grid.is_walkable(c + IVec2::new(dx, 0), allow_gates);
            let b = grid.is_walkable(c + IVec2::new(0, dz), allow_gates);
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
    let mut dist = vec![f32::INFINITY; grid.len()];
    let mut prev = vec![None; grid.len()];
    let mut heap = BinaryHeap::new();
    for &s in sources {
        if let Some(k) = grid.index(s) {
            if grid.is_walkable(s, allow_gates) {
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
        for (n, len) in neighbours(grid, c, allow_gates) {
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
pub fn find_path(grid: &Grid, from: IVec2, to: IVec2, allow_gates: bool) -> Option<Vec<IVec2>> {
    // Search backwards from the target so `prev` walks from `from` towards `to`.
    let f = dijkstra(grid, &[to], Cost::Distance, allow_gates);
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
