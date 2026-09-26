//! Prop collision (GAME-PLAYER §7): the player is a circle of radius 0.3 m that cannot
//! overlap solid cells or the collision shapes of placed props; she slides along them.
//!
//! Footprints are defined per prop model in model space (x = model right, z = world +Z =
//! the model's front / level south at yaw 0) and placed with the same position and yaw as
//! the rendered model ([`crate::scene::LevelScene`]), so what is drawn is what blocks.

use glam::{IVec2, Vec2, Vec3};

use crate::coords::world_to_level;
use crate::level::{cell_center, cell_of, Grid, Rect};
use crate::scene::Placement;

/// Player collision radius in metres (GAME-PLAYER §7).
pub const PLAYER_RADIUS_M: f32 = 0.3;

/// Overlaps smaller than this (metres) are ignored (numerical slack after a push-out).
const EPS: f32 = 1e-4;

/// A collision shape in model space (see module docs).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LocalShape {
    Circle { x: f32, z: f32, r: f32 },
    Box { x: f32, z: f32, hx: f32, hz: f32 },
}

/// Ground footprint of a prop model (GAME-LAYOUT "Collision footprints", values measured
/// from the `.glb` cross-section 0.05–1.4 m, proposal Q-087; LAYOUT-018). Models not listed
/// (tiles, fences, hedges and walls on solid cells, ground decoration such as grass tufts,
/// lily pads, flower beds, reeds, ducks) have none.
pub fn footprint(model: &str) -> &'static [LocalShape] {
    use LocalShape::{Box as B, Circle as C};
    match model {
        // single post, panel tilted back (towards -Z)
        "info_board" => &[B {
            x: 0.0,
            z: -0.06,
            hx: 0.51,
            hz: 0.32,
        }],
        // two posts at x = ±1.09; the gate opening between them stays free (Q-086: the
        // panel becomes a gate arch)
        "enclosure_sign" => &[
            C {
                x: -1.09,
                z: -0.05,
                r: 0.12,
            },
            C {
                x: 1.09,
                z: -0.05,
                r: 0.12,
            },
        ],
        "map_board" => &[B {
            x: 0.0,
            z: 0.04,
            hx: 1.08,
            hz: 0.19,
        }],
        "food_box" => &[B {
            x: 0.0,
            z: 0.0,
            hx: 0.31,
            hz: 0.30,
        }],
        "food_box_stack" => &[B {
            x: 0.0,
            z: 0.0,
            hx: 0.65,
            hz: 0.37,
        }],
        "tree_round" => &[C {
            x: 0.0,
            z: 0.0,
            r: 0.45,
        }],
        "tree_grove" => &[C {
            x: 0.0,
            z: 0.0,
            r: 0.35,
        }],
        "tree_eucalyptus" => &[C {
            x: 0.0,
            z: 0.0,
            r: 0.20,
        }],
        "bush" => &[C {
            x: 0.0,
            z: 0.05,
            r: 0.67,
        }],
        "rock" => &[
            C {
                x: -0.14,
                z: 0.03,
                r: 0.49,
            },
            C {
                x: 0.37,
                z: 0.03,
                r: 0.49,
            },
        ],
        // box on the leaf extents (−0.46…+0.67; −0.87…+0.72): a circle would reach 0.24 m
        // beyond the mesh (LAYOUT-018)
        "bamboo" => &[B {
            x: 0.105,
            z: -0.075,
            hx: 0.575,
            hz: 0.80,
        }],
        "road_block" => &[B {
            x: 0.0,
            z: 0.0,
            hx: 1.05,
            hz: 0.42,
        }],
        "repair_sign" => &[B {
            x: 0.0,
            z: 0.0,
            hx: 0.38,
            hz: 0.12,
        }],
        "zookeeper_cart" => &[B {
            x: 0.22,
            z: 0.0,
            hx: 1.17,
            hz: 0.57,
        }],
        "traffic_cone" => &[C {
            x: 0.0,
            z: 0.0,
            r: 0.2,
        }],
        "gate_zoo_closed" => &[B {
            x: 0.0,
            z: 0.0,
            hx: 1.56,
            hz: 0.36,
        }],
        "fallen_tree" => &[B {
            x: -0.04,
            z: -0.32,
            hx: 1.22,
            hz: 2.14,
        }],
        // handrails along both deck edges (deck along X, 2.5 m wide)
        "bridge_wood" => &[
            B {
                x: 0.0,
                z: 1.25,
                hx: 1.7,
                hz: 0.06,
            },
            B {
                x: 0.0,
                z: -1.25,
                hx: 1.7,
                hz: 0.06,
            },
        ],
        _ => &[],
    }
}

/// A collision shape in level coordinates `(x, z)`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    Circle {
        c: Vec2,
        r: f32,
    },
    /// Oriented box: centre, unit axis `u` (the second axis is its perpendicular) and half
    /// extents along `u` / the perpendicular.
    Box {
        c: Vec2,
        u: Vec2,
        half: Vec2,
    },
}

impl Shape {
    /// Axis-aligned cell box.
    pub fn cell(c: IVec2) -> Shape {
        Shape::Box {
            c: cell_center(c),
            u: Vec2::X,
            half: Vec2::splat(0.5),
        }
    }

    /// Places a model-space shape with a model's world position and yaw.
    pub fn place(local: LocalShape, pos: Vec3, yaw: f32) -> Shape {
        Self::place_scaled(local, pos, yaw, 1.0)
    }

    /// [`Shape::place`] for a uniformly scaled model.
    pub fn place_scaled(local: LocalShape, pos: Vec3, yaw: f32, scale: f32) -> Shape {
        let local = match local {
            LocalShape::Circle { x, z, r } => LocalShape::Circle {
                x: x * scale,
                z: z * scale,
                r: r * scale,
            },
            LocalShape::Box { x, z, hx, hz } => LocalShape::Box {
                x: x * scale,
                z: z * scale,
                hx: hx * scale,
                hz: hz * scale,
            },
        };
        // Model x axis and z axis expressed in level coordinates (world z is flipped).
        let (s, c) = yaw.sin_cos();
        let ax = Vec2::new(c, s);
        let az = Vec2::new(s, -c);
        let origin = world_to_level(pos);
        match local {
            LocalShape::Circle { x, z, r } => Shape::Circle {
                c: origin + ax * x + az * z,
                r,
            },
            LocalShape::Box { x, z, hx, hz } => Shape::Box {
                c: origin + ax * x + az * z,
                u: ax,
                half: Vec2::new(hx, hz),
            },
        }
    }

    /// Level-space bounding box `(min, max)`.
    pub fn aabb(&self) -> (Vec2, Vec2) {
        match *self {
            Shape::Circle { c, r } => (c - Vec2::splat(r), c + Vec2::splat(r)),
            Shape::Box { c, u, half } => {
                let v = Vec2::new(-u.y, u.x);
                let e = (u * half.x).abs() + (v * half.y).abs();
                (c - e, c + e)
            }
        }
    }

    /// Vector that moves a circle at `p` with radius `r` out of this shape (zero if they do
    /// not overlap by more than `EPS`).
    pub fn push_out(&self, p: Vec2, r: f32) -> Vec2 {
        match *self {
            Shape::Circle { c, r: rc } => {
                let d = p - c;
                let len = d.length();
                let pen = r + rc - len;
                if pen <= EPS {
                    return Vec2::ZERO;
                }
                let n = if len > 1e-6 { d / len } else { Vec2::X };
                n * pen
            }
            Shape::Box { c, u, half } => {
                let v = Vec2::new(-u.y, u.x);
                let d = p - c;
                let q = Vec2::new(d.dot(u), d.dot(v));
                let closest = q.clamp(-half, half);
                let off = q - closest;
                let dist = off.length();
                if dist > 1e-6 {
                    let pen = r - dist;
                    if pen <= EPS {
                        return Vec2::ZERO;
                    }
                    let n = off / dist;
                    return (u * n.x + v * n.y) * pen;
                }
                // Centre inside the box: leave along the axis of least penetration.
                let gap = half - q.abs();
                if gap.x < gap.y {
                    u * (gap.x + r) * q.x.signum()
                } else {
                    v * (gap.y + r) * q.y.signum()
                }
            }
        }
    }

    pub fn overlaps(&self, p: Vec2, r: f32) -> bool {
        self.push_out(p, r) != Vec2::ZERO
    }
}

/// All prop collision shapes of a level with a per-cell index for fast queries.
#[derive(Debug, Clone, Default)]
pub struct Colliders {
    shapes: Vec<Shape>,
    bounds: Rect,
    cells: Vec<Vec<u32>>,
}

impl Colliders {
    /// Builds the shapes of every placement whose model has a footprint.
    pub fn from_placements(placements: &[Placement], bounds: Rect) -> Self {
        let mut shapes = Vec::new();
        for p in placements {
            for &l in footprint(p.model) {
                shapes.push(Shape::place_scaled(l, p.pos, p.yaw, p.scale));
            }
        }
        Self::from_shapes(shapes, bounds)
    }

    pub fn from_shapes(shapes: Vec<Shape>, bounds: Rect) -> Self {
        let mut cells = vec![Vec::new(); (bounds.w.max(0) * bounds.d.max(0)) as usize];
        for (i, s) in shapes.iter().enumerate() {
            let (lo, hi) = s.aabb();
            let (a, b) = (cell_of(lo), cell_of(hi));
            for z in a.y..=b.y {
                for x in a.x..=b.x {
                    let c = IVec2::new(x, z);
                    if bounds.contains(c) {
                        let k = ((c.y - bounds.z) * bounds.w + (c.x - bounds.x)) as usize;
                        cells[k].push(i as u32);
                    }
                }
            }
        }
        Self {
            shapes,
            bounds,
            cells,
        }
    }

    pub fn shapes(&self) -> &[Shape] {
        &self.shapes
    }

    /// Indices of shapes that may touch a circle at `p` with radius `r` (deduplicated into
    /// `out`, which is cleared first).
    pub fn candidates(&self, p: Vec2, r: f32, out: &mut Vec<u32>) {
        out.clear();
        let (a, b) = (cell_of(p - Vec2::splat(r)), cell_of(p + Vec2::splat(r)));
        for z in a.y..=b.y {
            for x in a.x..=b.x {
                let c = IVec2::new(x, z);
                if !self.bounds.contains(c) {
                    continue;
                }
                let k = ((c.y - self.bounds.z) * self.bounds.w + (c.x - self.bounds.x)) as usize;
                for &i in &self.cells[k] {
                    if !out.contains(&i) {
                        out.push(i);
                    }
                }
            }
        }
    }

    pub fn shape(&self, i: u32) -> &Shape {
        &self.shapes[i as usize]
    }

    /// Whether a circle at `p` overlaps any prop shape.
    pub fn overlaps(&self, p: Vec2, r: f32) -> bool {
        let mut buf = Vec::new();
        self.candidates(p, r, &mut buf);
        buf.iter().any(|&i| self.shape(i).overlaps(p, r))
    }
}

/// Everything solid for a circle at `p`: blocked cells (from `walkable`) and prop shapes.
/// Cells and shapes listed in `exempt_cells` / `exempt_shapes` are ignored (things the
/// circle already overlapped before the move, e.g. a gate that closed under the player).
pub struct Blockers<'a> {
    pub grid: &'a Grid,
    pub colliders: &'a Colliders,
    pub allow_gates: bool,
    pub exempt_cells: Vec<IVec2>,
    pub exempt_shapes: Vec<u32>,
    scratch: Vec<u32>,
}

impl<'a> Blockers<'a> {
    /// Blockers with the exemptions of a circle currently at `p`.
    pub fn at(grid: &'a Grid, colliders: &'a Colliders, allow_gates: bool, p: Vec2) -> Self {
        let mut b = Self {
            grid,
            colliders,
            allow_gates,
            exempt_cells: Vec::new(),
            exempt_shapes: Vec::new(),
            scratch: Vec::new(),
        };
        b.exempt_cells = b.blocked_cells(p, PLAYER_RADIUS_M);
        let mut cand = Vec::new();
        colliders.candidates(p, PLAYER_RADIUS_M, &mut cand);
        b.exempt_shapes = cand
            .into_iter()
            .filter(|&i| colliders.shape(i).overlaps(p, PLAYER_RADIUS_M))
            .collect();
        b
    }

    fn blocked_cells(&self, p: Vec2, r: f32) -> Vec<IVec2> {
        let (a, b) = (cell_of(p - Vec2::splat(r)), cell_of(p + Vec2::splat(r)));
        let mut out = Vec::new();
        for z in a.y..=b.y {
            for x in a.x..=b.x {
                let c = IVec2::new(x, z);
                if !self.grid.is_walkable(c, self.allow_gates) && Shape::cell(c).overlaps(p, r) {
                    out.push(c);
                }
            }
        }
        out
    }

    /// Sum of push-out vectors of every (non-exempt) blocker overlapping the circle.
    fn push(&mut self, p: Vec2, r: f32) -> Vec2 {
        let mut total = Vec2::ZERO;
        let (a, b) = (cell_of(p - Vec2::splat(r)), cell_of(p + Vec2::splat(r)));
        for z in a.y..=b.y {
            for x in a.x..=b.x {
                let c = IVec2::new(x, z);
                if !self.grid.is_walkable(c, self.allow_gates) && !self.exempt_cells.contains(&c) {
                    total += Shape::cell(c).push_out(p, r);
                }
            }
        }
        let mut cand = std::mem::take(&mut self.scratch);
        self.colliders.candidates(p, r, &mut cand);
        for &i in &cand {
            if !self.exempt_shapes.contains(&i) {
                total += self.colliders.shape(i).push_out(p, r);
            }
        }
        self.scratch = cand;
        total
    }

    /// Resolves a circle moved to `p`: pushes it out of blockers (sliding). Returns `None` if
    /// no free position close to `p` is found.
    pub fn resolve(&mut self, p: Vec2, r: f32) -> Option<Vec2> {
        let mut q = p;
        for _ in 0..6 {
            let push = self.push(q, r);
            if push == Vec2::ZERO {
                return Some(q);
            }
            q += push;
        }
        (self.push(q, r) == Vec2::ZERO).then_some(q)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placed_box_follows_yaw() {
        // A box 1 × 0.5 turned to face east (yaw +90°) is 0.5 wide along level x.
        let s = Shape::place(
            LocalShape::Box {
                x: 0.0,
                z: 0.0,
                hx: 0.5,
                hz: 0.25,
            },
            Vec3::new(2.0, 0.0, -3.0),
            std::f32::consts::FRAC_PI_2,
        );
        let (lo, hi) = s.aabb();
        assert!((lo - Vec2::new(1.75, 2.5)).length() < 1e-5, "{lo}");
        assert!((hi - Vec2::new(2.25, 3.5)).length() < 1e-5, "{hi}");
    }

    #[test]
    fn push_out_of_circle_and_box() {
        let c = Shape::Circle {
            c: Vec2::ZERO,
            r: 0.5,
        };
        let p = c.push_out(Vec2::new(0.6, 0.0), 0.3);
        assert!((p.x - 0.2).abs() < 1e-5 && p.y.abs() < 1e-6);
        let b = Shape::cell(IVec2::ZERO);
        let p = b.push_out(Vec2::new(1.1, 0.5), 0.3);
        assert!((p.x - 0.2).abs() < 1e-5);
        assert_eq!(b.push_out(Vec2::new(1.4, 0.5), 0.3), Vec2::ZERO);
    }
}
