//! Placeholder geometry of a round enterable house (the terrarium house of `night_2`,
//! LAYOUT-N2-020): wall mass cells, curved outer walls along the stepped arc, glass fronts
//! between the hall and the cases and a stepped dome roof. The real conservatory model
//! replaces it later.

use super::{colors, BoxPlacement, BuildingModel, Fallback, LevelScene};
use crate::coords::level_to_world_at;
use crate::level::{cell_center, Element, ElementType, LevelData, Rect};
use glam::{IVec2, Vec2, Vec3};
use std::collections::BTreeSet;

/// Frame colour of the glass fronts (pale blue-white, like the `glass_door` frame).
const GLASS_FRAME: [f32; 3] = [0.80, 0.90, 0.95];
/// Upper dome layers: pale glass-green.
const DOME_GLASS: [f32; 3] = [0.40, 0.60, 0.36];
/// Warm timber walls and the green grass-roof dome of the approved concept
/// (`art/environment/env_terrarium_house/overview_v2.jpg`).
const WALL_WOOD: [f32; 3] = [0.62, 0.40, 0.23];
const DOME_GREEN: [f32; 3] = [0.30, 0.50, 0.30];
/// Plank floor (LAYOUT-N2-024) and its board lines.
const FLOOR_WOOD: [f32; 3] = [0.80, 0.58, 0.36];
const FLOOR_LINE: [f32; 3] = [0.56, 0.37, 0.21];
/// Top of the plank floor (m): just above the path tile under it.
const FLOOR_TOP_M: f32 = crate::ground::PATH_TOP_M + 0.03;
/// Centre height of the emblem plaque above the door (m).
const EMBLEM_Y_M: f32 = 3.7;
/// Thickness of the thin outer wall of a case (m).
const CASE_WALL_M: f32 = 0.3;
/// Height of the glass front's sill and the glass frame top (m).
const SILL_M: f32 = 0.7;
const FRAME_TOP_M: f32 = 2.3;
/// Height of one dome layer (m) and the overhang of the lowest one (m).
const DOME_STEP_M: f32 = 0.5;
const EAVE_M: f32 = 0.15;

/// Whether an enterable building has a round plan (extra footprint rects).
pub fn is_round(e: &Element) -> bool {
    e.is_enterable() && !e.footprint_extra.is_empty()
}

const DIRS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

impl LevelScene {
    /// A round enterable house: wall mass, case walls, glass fronts, dome roof, door.
    pub(super) fn round_building(&mut self, e: &Element, data: &LevelData) {
        let id = e.id.as_str();
        let height = e.height_m.unwrap_or(4.0);
        let wall_h = height * 0.7;
        let low = 1.0;
        let door = e.door_cell();
        let model = e.model_rect.unwrap_or(e.rect);
        let footprint: BTreeSet<(i32, i32)> = e.footprint_cells().map(|c| (c.x, c.y)).collect();
        let hall: BTreeSet<(i32, i32)> = e.interior_cells().iter().map(|c| (c.x, c.y)).collect();
        let cases: Vec<&Element> = data
            .elements_of(ElementType::Enclosure)
            .filter(|c| c.indoor && model.contains(IVec2::new(c.rect.x, c.rect.z)))
            .collect();
        let case_cells: BTreeSet<(i32, i32)> = cases
            .iter()
            .flat_map(|c| c.rect.cells().map(|p| (p.x, p.y)))
            .collect();
        let mut under: BTreeSet<(i32, i32)> = BTreeSet::new();
        under.extend(&footprint);
        under.extend(&hall);
        under.extend(&case_cells);
        let inside = |c: IVec2| under.contains(&(c.x, c.y));
        let first = self.boxes.len();

        // -- wall mass: every footprint cell as row runs, split around the door ----------
        let mut rects: Vec<Rect> = Vec::new();
        for r in std::iter::once(e.rect).chain(e.footprint_extra.iter().copied()) {
            match door.filter(|d| r.contains(*d)) {
                Some(d) if r.d == 1 => {
                    let left = Rect {
                        x: r.x,
                        z: r.z,
                        w: d.x - r.x,
                        d: 1,
                    };
                    let right = Rect {
                        x: d.x + 1,
                        z: r.z,
                        w: r.x + r.w - d.x - 1,
                        d: 1,
                    };
                    rects.extend([left, right].into_iter().filter(|q| q.w > 0));
                }
                _ => rects.push(r),
            }
        }
        let mut low_boxes = Vec::new();
        let mut high_boxes = Vec::new();
        for r in &rects {
            // 0.05 m inset on the sides that meet the outside (outlines), none towards a
            // neighbouring cell of the house
            let open_side = |mut cells: Vec<IVec2>| cells.drain(..).any(|c| !inside(c));
            let west = open_side((r.z..r.z + r.d).map(|z| IVec2::new(r.x - 1, z)).collect());
            let east = open_side((r.z..r.z + r.d).map(|z| IVec2::new(r.x + r.w, z)).collect());
            let south = open_side((r.x..r.x + r.w).map(|x| IVec2::new(x, r.z - 1)).collect());
            let north = open_side((r.x..r.x + r.w).map(|x| IVec2::new(x, r.z + r.d)).collect());
            let inset = |b: bool| if b { 0.05 } else { 0.0 };
            let (x0, x1) = (r.x as f32 + inset(west), (r.x + r.w) as f32 - inset(east));
            let (z0, z1) = (r.z as f32 + inset(south), (r.z + r.d) as f32 - inset(north));
            let c = Vec2::new((x0 + x1) / 2.0, (z0 + z1) / 2.0);
            low_boxes.push((c, Vec3::new(x1 - x0, low, z1 - z0)));
            high_boxes.push((c, Vec3::new(x1 - x0, wall_h - low, z1 - z0)));
        }
        // the door's own cell is a lintel and the door leaf only
        for (c, size) in &low_boxes {
            self.push_box(id, *c, 0.0, *size, WALL_WOOD);
        }
        // -- thin outer walls of the cases along the arc --------------------------------
        let mut edges: Vec<(Vec2, Vec2)> = Vec::new(); // (centre, size x/z)
        let t = CASE_WALL_M;
        for &(x, z) in &case_cells {
            let c = IVec2::new(x, z);
            for d in DIRS {
                if inside(c + d) {
                    continue;
                }
                let mid = Vec2::new(x as f32 + 0.5, z as f32 + 0.5) + d.as_vec2() * (0.5 - t / 2.0);
                let size = if d.x != 0 {
                    Vec2::new(t, 1.0)
                } else {
                    Vec2::new(1.0, t)
                };
                edges.push((mid, size));
            }
        }
        for (c, size) in &edges {
            self.push_box(id, *c, 0.0, Vec3::new(size.x, low, size.y), WALL_WOOD);
        }
        // -- glass fronts between hall and cases ----------------------------------------
        let gates: Vec<IVec2> = cases
            .iter()
            .filter_map(|c| c.gate)
            .flat_map(|g| g.cells().collect::<Vec<_>>())
            .collect();
        // panes and posts are keyed by their doubled coordinates so shared ones are drawn once
        let mut panes: BTreeSet<(i32, i32, bool)> = BTreeSet::new();
        let mut posts: BTreeSet<(i32, i32)> = BTreeSet::new();
        for case in &cases {
            for c in case.rect.cells() {
                for d in DIRS {
                    let n = c + d;
                    if !hall.contains(&(n.x, n.y)) {
                        continue;
                    }
                    let along_x = d.y != 0; // the pane runs along x for a north / south edge
                                            // edge centre in doubled coordinates
                    let (ex, ez) = (2 * c.x + 1 + d.x, 2 * c.y + 1 + d.y);
                    if !gates.contains(&c) {
                        panes.insert((ex, ez, along_x));
                    }
                    for s in [-1, 1] {
                        posts.insert(if along_x { (ex + s, ez) } else { (ex, ez + s) });
                    }
                }
            }
        }
        for (ex, ez, along_x) in panes {
            let edge = Vec2::new(ex as f32 / 2.0, ez as f32 / 2.0);
            let (lx, lz) = if along_x { (1.0, 0.08) } else { (0.08, 1.0) };
            self.push_box(id, edge, 0.0, Vec3::new(lx, SILL_M, lz), GLASS_FRAME);
            self.push_box(id, edge, FRAME_TOP_M, Vec3::new(lx, 0.1, lz), GLASS_FRAME);
        }
        for (px, pz) in posts {
            let p = Vec2::new(px as f32 / 2.0, pz as f32 / 2.0);
            self.push_box(
                id,
                p,
                0.0,
                Vec3::new(0.1, FRAME_TOP_M + 0.1, 0.1),
                GLASS_FRAME,
            );
        }

        for (c, size) in &high_boxes {
            self.push_box(id, *c, low, *size, WALL_WOOD);
        }
        if let Some(dc) = door {
            let c = cell_center(dc);
            self.push_box(
                id,
                Vec2::new(c.x, r_mid_z(e.rect)),
                2.2,
                Vec3::new(1.0, wall_h - 2.2, e.rect.d as f32 - 0.1),
                WALL_WOOD,
            );
            self.procedural_door(e, r_mid_z(e.rect), 'S');
        }

        for (c, size) in &edges {
            self.push_box(
                id,
                *c,
                low,
                Vec3::new(size.x, wall_h - low, size.y),
                WALL_WOOD,
            );
        }

        // -- wooden plank floor over the hall and the door cell (LAYOUT-N2-024) ---------------
        let mut floor: BTreeSet<(i32, i32)> = hall.clone();
        if let Some(d) = door {
            floor.insert((d.x, d.y));
        }
        for &(x, z) in &floor {
            // one slab per cell (a 1 m plank cell); a board line down the middle
            let c = Vec2::new(x as f32 + 0.5, z as f32 + 0.5);
            self.push_box(id, c, 0.0, Vec3::new(1.0, FLOOR_TOP_M, 1.0), FLOOR_WOOD);
            self.push_box(id, c, FLOOR_TOP_M, Vec3::new(0.05, 0.004, 0.96), FLOOR_LINE);
        }

        // -- the house emblem: a round plaque with the big snake over the door ---------------
        // (fallback only; the model has its own plaque, the decal below is on both)
        let animal = cases.first().and_then(|c| c.animal.clone());
        if let (Some(dc), Some(animal)) = (door, animal.as_deref()) {
            let centre = Vec2::new(dc.x as f32 + 0.5, e.rect.z as f32 - 0.2);
            self.pictogram_plate(
                format!("{id}:emblem"),
                animal,
                centre + Vec2::new(0.0, -0.05),
                Vec2::new(0.0, -1.0),
                0.0,
                EMBLEM_Y_M,
                Vec2::new(1.8, 1.2),
                e.part as u8,
            );
            // the plate box belongs to the fallback; the decal comes from the model's plaque
            self.decals.pop();
        }

        // -- stepped dome roof over the whole plan --------------------------------------
        let depth = |c: IVec2| -> i32 {
            // layers of cells that are surrounded by the house on all sides (Chebyshev)
            let mut k = 0;
            'grow: loop {
                for dx in -(k + 1)..=(k + 1) {
                    for dz in -(k + 1)..=(k + 1) {
                        if !inside(c + IVec2::new(dx, dz)) {
                            break 'grow;
                        }
                    }
                }
                k += 1;
                if k >= 6 {
                    break;
                }
            }
            k
        };
        let (min, max) = under.iter().fold(
            (IVec2::splat(i32::MAX), IVec2::splat(i32::MIN)),
            |(lo, hi), &(x, z)| (lo.min(IVec2::new(x, z)), hi.max(IVec2::new(x, z))),
        );
        for layer in 0..4 {
            for z in min.y..=max.y {
                let mut x = min.x;
                while x <= max.x {
                    let ok = |x: i32| {
                        let c = IVec2::new(x, z);
                        inside(c) && depth(c) >= layer * 2
                    };
                    if !ok(x) {
                        x += 1;
                        continue;
                    }
                    let start = x;
                    while x <= max.x && ok(x) {
                        x += 1;
                    }
                    let over = if layer == 0 { EAVE_M } else { 0.0 };
                    let (x0, x1) = (start as f32 - over, x as f32 + over);
                    let (z0, z1) = (z as f32 - over, z as f32 + 1.0 + over);
                    // keep the outlines of neighbouring rows apart a hair
                    let size = Vec3::new(x1 - x0, DOME_STEP_M, z1 - z0 - 0.04);
                    let c = Vec2::new((x0 + x1) / 2.0, (z0 + z1) / 2.0);
                    let color = if layer < 2 { DOME_GREEN } else { DOME_GLASS };
                    self.push_box(id, c, wall_h + layer as f32 * DOME_STEP_M, size, color);
                }
            }
        }

        // the placeholder is only the fallback of the house model (`terrarium_house`): the
        // model draws shell, roof, floor and plaque (its `shell_upper` and `roof` parts hide
        // while the player is inside, PLAY-028)
        let mut boxes: Vec<BoxPlacement> = self.boxes.drain(first..).collect();
        for b in &mut boxes {
            b.fadeable = false;
            b.part = e.part as u8;
        }
        let at = Vec2::new(model.x as f32 + model.w as f32 / 2.0, model.z as f32);
        let k = self.model_at_y("terrarium_house", at, 0.0, 0.0);
        self.placements[k].part = e.part as u8;
        self.building_models.push(BuildingModel {
            element: e.id.clone(),
            placement: k,
        });
        self.fallbacks.push(Fallback {
            model: "terrarium_house",
            boxes,
            placements: Vec::new(),
        });
        if let Some(animal) = animal.as_deref().filter(|_| door.is_some()) {
            self.terrarium_emblem(id, at, animal);
        }
        self.terrarium_cases(e, data);
    }

    /// A flat cream plate with a dark animal pictogram decal in front of a sign face
    /// (LAYOUT-N2-022): `face` = the point on the sign's front where the plate is centred
    /// (level), `out` the unit direction the sign faces (level), `centre_y` the height of its
    /// centre, `size` the plate (w, h) in m; `yaw` the box yaw of the sign.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn pictogram_plate(
        &mut self,
        source: String,
        animal: &str,
        face: Vec2,
        out: Vec2,
        yaw: f32,
        centre_y: f32,
        size: Vec2,
        part: u8,
    ) {
        let plate = Vec3::new(size.x, size.y, 0.02);
        self.boxes.push(BoxPlacement {
            pos: level_to_world_at(face + out * (plate.z / 2.0), centre_y - size.y / 2.0),
            size: plate,
            yaw,
            color: colors::CREAM,
            fadeable: false,
            source: format!("{source}:plate"),
            part,
        });
        self.pictogram_decal(
            source,
            animal,
            face + out * plate.z,
            out,
            centre_y,
            size.y * 0.45,
        );
    }

    /// The dark animal pictogram alone (no plate): `face` = the point on the surface where it
    /// is centred (level), `out` the unit direction the surface faces, `half_h` the half
    /// height of the picture (it is 1.5 times as wide as high).
    pub(super) fn pictogram_decal(
        &mut self,
        source: String,
        animal: &str,
        face: Vec2,
        out: Vec2,
        centre_y: f32,
        half_h: f32,
    ) {
        let n = crate::coords::level_to_world(out).normalize();
        let right = Vec3::Y.cross(n);
        let front = face + out * super::DECAL_LIFT_M;
        self.decals.push(super::Decal {
            id: format!("sign:{source}"),
            image: super::DecalImage::Texture(super::silhouette_path(animal)),
            center: level_to_world_at(front, centre_y),
            right: right * (half_h * 1.5),
            up: Vec3::Y * half_h,
        });
    }
}

/// The middle line (level z) of a one-row facade rect.
fn r_mid_z(r: Rect) -> f32 {
    r.z as f32 + r.d as f32 / 2.0
}
