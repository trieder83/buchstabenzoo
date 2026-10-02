//! LAYOUT-031: every gate / door opening of the joined zoo has its gate or door model that
//! fills the opening and opens by the rules of GAME-LAYOUT "Gates and doors"; buildings with
//! a model keep their roof visible in first person (CAMV-022, the rule in `view`).

mod common;

use glam::Vec2;
use zoo_core::level::ElementType;
use zoo_core::scene::{building_model, LevelScene, OpeningKind};
use zoo_core::view::{roof_hidden, ViewMode};
use zoo_core::AnimalState;

#[test]
fn layout_031_every_opening_has_its_gate_or_door_model() {
    let data = common::zoo_with_night();
    let s = LevelScene::build(&data);
    let model_of = |o: &zoo_core::scene::Opening| s.placements[o.placement].model;
    for o in &s.openings {
        // no gap > 5 cm on either side, never wider than the opening (no overlap with posts)
        let gap = (o.opening_m - o.model_m) / 2.0;
        assert!((-0.001..=0.05).contains(&gap), "{:?}: gap {gap} m", o.kind);
    }
    // enclosure gates: gate_wood (outdoor) or glass_door (indoor, night house model)
    for e in data.elements_of(ElementType::Enclosure) {
        let Some(_) = e.gate else { continue };
        let o =
            s.openings
                .iter()
                .find(|o| match &o.kind {
                    OpeningKind::EnclosureGate { enclosure }
                    | OpeningKind::GlassDoor { enclosure } => enclosure == &e.id,
                    _ => false,
                })
                .unwrap_or_else(|| panic!("{}: no gate model", e.id));
        let want = if e.indoor { "glass_door" } else { "gate_wood" };
        assert_eq!(model_of(o), want, "{}", e.id);
    }
    // building doors: door_wood in every door cell
    for e in data.elements_of(ElementType::Building) {
        let Some(dc) = e.door_cell() else { continue };
        let o = s
            .openings
            .iter()
            .find(|o| matches!(&o.kind, OpeningKind::BuildingDoor { building, .. } if building == &e.id))
            .unwrap_or_else(|| panic!("{}: no door model", e.id));
        assert_eq!(model_of(o), "door_wood");
        assert!(o.center.distance(zoo_core::level::cell_center(dc)) < 1e-4);
        let hinge = zoo_core::coords::world_to_level(s.placements[o.placement].pos);
        assert!(
            hinge.distance(zoo_core::level::cell_center(dc)) < 0.75,
            "{}: hinge {hinge} at the door cell {dc}",
            e.id
        );
    }
    // the garden gate, the moon door, the entrance turnstiles, the barrier gate
    assert!(s
        .openings
        .iter()
        .any(|o| matches!(o.kind, OpeningKind::GardenGate { .. }) && model_of(o) == "garden_gate"));
    assert!(s
        .openings
        .iter()
        .any(|o| matches!(o.kind, OpeningKind::MoonDoor { .. }) && model_of(o) == "moon_door"));
    assert_eq!(
        s.placements
            .iter()
            .filter(|p| p.model == "turnstile")
            .count(),
        3,
        "three turnstile lanes under the entrance arch"
    );
    // the barrier gate is a level gate now (LAYOUT-036): `gate_zoo` with leaves
    assert!(!s.placements.iter().any(|p| p.model == "gate_zoo_closed"));
    assert!(s.openings.iter().any(|o| matches!(
        &o.kind,
        OpeningKind::LevelGate { barrier } if barrier == "barrier_north_gate"
    ) && model_of(o) == "gate_zoo"));
    // building models where the footprint fits (the rest stays procedural)
    for (id, model) in [
        ("zookeeper_house_1", "zookeeper_house"),
        ("food_storage", "food_storage"),
        ("food_storage_2", "food_storage"),
        ("food_storage_3", "food_storage"),
        ("entrance_gate", "entrance_arch"),
        ("food_storage_n1", "food_hut"),
        ("night_house", "night_house"),
    ] {
        let e = data.element(id).unwrap();
        let (spec, ..) = building_model(e).unwrap_or_else(|| panic!("{id}: no model"));
        assert_eq!(spec.model, model);
        assert!(s.building_models.iter().any(|b| b.element == id));
    }
}

#[test]
fn layout_031_gates_and_doors_open_by_the_rules() {
    let mut g = common::night_game(1);
    let openings = g.level.openings().to_vec();
    let find =
        |f: &dyn Fn(&OpeningKind) -> bool| openings.iter().find(|o| f(&o.kind)).unwrap().clone();
    // building door of an enterable house: open while the player passes, else closed
    let door = find(
        &|k| matches!(k, OpeningKind::BuildingDoor { building, .. } if building == "zookeeper_house_1"),
    );
    g.player.pos = door.center + Vec2::new(1.0, 0.0);
    assert!(g.opening_open(&door));
    g.player.pos = door.center + Vec2::new(4.0, 0.0);
    assert!(!g.opening_open(&door));
    // every building with a door is enterable (LAYOUT-041): the food storage door opens too
    let shed = find(
        &|k| matches!(k, OpeningKind::BuildingDoor { building, .. } if building == "food_storage"),
    );
    assert!(matches!(
        shed.kind,
        OpeningKind::BuildingDoor {
            enterable: true,
            ..
        }
    ));
    g.player.pos = shed.center + Vec2::new(0.0, -1.0);
    assert!(g.opening_open(&shed));
    g.player.pos = shed.center + Vec2::new(0.0, -4.0);
    assert!(!g.opening_open(&shed));
    // enclosure gate: only while leading animals near it
    let gate = find(
        &|k| matches!(k, OpeningKind::EnclosureGate { enclosure } if enclosure == "enc_zebra"),
    );
    g.player.pos = gate.center + Vec2::new(0.0, -1.5);
    g.player.pos = gate.center;
    assert!(!g.opening_open(&gate), "closed when not leading");
    let z = g.animal_index("zebra").unwrap();
    g.animals[z].state = AnimalState::Following;
    g.animals[z].pos = gate.center;
    assert!(g.opening_open(&gate), "open while leading");
    g.player.pos = gate.center + Vec2::new(0.0, 8.0);
    assert!(!g.opening_open(&gate), "closed behind them");
    // the moon door: open with its barrier (at night)
    let moon = find(&|k| matches!(k, OpeningKind::MoonDoor { .. }));
    assert!(!g.opening_open(&moon), "closed by day");
    assert!(g.debug_set_daytime("night"));
    assert!(g.opening_open(&moon), "open at night");
}

#[test]
fn camv_022_roof_hidden_only_in_the_zoo_view() {
    assert!(roof_hidden(true, ViewMode::Zoo));
    assert!(roof_hidden(true, ViewMode::LookAround));
    // first person keeps the roof and its ceiling (also while gliding in)
    assert!(!roof_hidden(true, ViewMode::FirstPerson));
    assert!(!roof_hidden(false, ViewMode::Zoo));
}

// LAYOUT-032, GAME-LAYOUT "Gates and doors" (user request 2026-09-27): no prop, board or furniture blocks
// a door or gate — the opening and a ≥ 1 m walkway in front of it (the player's body width)
// stay free; for enterable buildings and the moon door on both sides.
#[test]
fn layout_032_doors_and_gates_keep_a_free_walkway() {
    use zoo_core::collision::PLAYER_RADIUS_M;
    use zoo_core::scene::rot_level;
    let data = common::zoo_with_night();
    let s = LevelScene::build(&data);
    let level = zoo_core::Level::new(data.clone());
    let owner = |o: &zoo_core::scene::Opening| -> Option<(zoo_core::Rect, bool)> {
        match &o.kind {
            OpeningKind::BuildingDoor {
                building,
                enterable,
            } => data.element(building).map(|e| (e.rect, *enterable)),
            OpeningKind::EnclosureGate { enclosure } | OpeningKind::GlassDoor { enclosure } => {
                data.element(enclosure).map(|e| (e.rect, false))
            }
            OpeningKind::GardenGate { garden } => data
                .gardens
                .iter()
                .find(|g| &g.id == garden)
                .map(|g| (g.rect, true)),
            OpeningKind::MoonDoor { .. } | OpeningKind::LevelGate { .. } => None,
        }
    };
    let mut bad = Vec::new();
    for o in &s.openings {
        let yaw = s.placements[o.placement].yaw;
        let n = rot_level(Vec2::NEG_Y, yaw); // model +Z (its front) in level space
        let along = Vec2::new(-n.y, n.x);
        let (sides, inside_too): (Vec<Vec2>, bool) = match owner(o) {
            Some((r, both)) => {
                let rc = Vec2::new(r.x as f32 + r.w as f32 / 2.0, r.z as f32 + r.d as f32 / 2.0);
                let out = if (o.center + n - rc).length() > (o.center - n - rc).length() {
                    n
                } else {
                    -n
                };
                (if both { vec![out, -out] } else { vec![out] }, both)
            }
            None => (vec![n, -n], true),
        };
        let _ = inside_too;
        for side in sides {
            for d in [0.6f32, 1.0, 1.4] {
                for t in [-0.2f32, 0.0, 0.2] {
                    let p = o.center + side * d + along * t;
                    let cell = zoo_core::level::cell_of(p);
                    let walkable = level.grid().is_walkable(cell, true);
                    let blocked = level.colliders().overlaps(p, PLAYER_RADIUS_M * 0.9);
                    if !walkable || blocked {
                        bad.push(format!(
                            "{:?} at {} side {side}: {} {}",
                            o.kind,
                            o.center,
                            if walkable { "" } else { "not walkable" },
                            if blocked { "blocked by a collider" } else { "" }
                        ));
                    }
                }
            }
        }
    }
    // exception: the moon door's cells are a barrier by day (open at night: LAYOUT-028); the
    // food-box rows keep a gap in front of the storage doors since Q-150 was answered
    // (user, 2026-09-27: doors are never blocked); level gates are closed behind their
    // barrier until the level unlocks (their open walkway: LAYOUT-036 in level_gates.rs)
    let bad: Vec<String> = bad
        .into_iter()
        .filter(|b| !b.starts_with("MoonDoor") && !b.starts_with("LevelGate"))
        .collect();
    assert!(
        bad.is_empty(),
        "{} blocked walkway points:\n{}",
        bad.len(),
        bad.join("\n")
    );
}

// LAYOUT-033 (user decision 2026-09-27): the enclosure sign stands BESIDE its gate, never in
// front of it — outside the fence, ≥ 0.9 m from the gate post (Q-157, LAYOUT-038), its whole (solid) footprint on
// walkable cells, clear of the gate opening and the 1 m walkway in front of it, of the info
// board and the food boxes, with a walkable place in front to look at it.
#[test]
fn layout_033_enclosure_signs_stand_beside_the_gate() {
    use zoo_core::collision::PLAYER_RADIUS_M;
    use zoo_core::coords::world_to_level;
    let data = common::zoo_with_night();
    let s = LevelScene::build(&data);
    let level = zoo_core::Level::new(data.clone());
    let mut bad = Vec::new();
    for e in data.elements_of(ElementType::Enclosure) {
        let Some(g) = e.gate else { continue };
        if e.indoor {
            continue; // a board above the glass door (night house)
        }
        let gc = Vec2::new(g.x as f32 + g.w as f32 / 2.0, g.z as f32 + g.d as f32 / 2.0);
        let along = if g.w > g.d { Vec2::X } else { Vec2::Y };
        // the sign of this enclosure: the enclosure_sign nearest to its gate
        let Some(sign) = s
            .placements
            .iter()
            .filter(|p| p.model == "enclosure_sign")
            .map(|p| world_to_level(p.pos))
            .min_by(|a, b| a.distance(gc).total_cmp(&b.distance(gc)))
        else {
            bad.push(format!("{}: no sign", e.id));
            continue;
        };
        let side = (sign - gc).dot(along).abs();
        if side < 1.0 + 0.9 + 1.19 - 0.01 {
            bad.push(format!(
                "{}: sign {sign} not beside the gate ({side:.2} m along)",
                e.id
            ));
        }
        if (sign - gc).length() > 5.0 {
            bad.push(format!("{}: sign {sign} too far from the gate", e.id));
        }
        // the gate walkway stays free of colliders (LAYOUT-032 covers the rest)
        let out = if e
            .rect
            .contains(zoo_core::level::cell_of(gc + along.perp() * 1.0))
        {
            -along.perp()
        } else {
            along.perp()
        };
        for d in [0.6f32, 1.0] {
            for t in [-0.3f32, 0.0, 0.3] {
                let p = gc + out * d + along * t;
                if level.colliders().overlaps(p, PLAYER_RADIUS_M * 0.9) {
                    bad.push(format!("{}: gate walkway blocked at {p}", e.id));
                }
            }
        }
    }
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

/// The outward normal of an opening (level space): the axis direction whose 1.2 m probe
/// leaves the owner rect while the opposite probe stays inside it; `None` for the moon door
/// (walked through both ways, normal from the model yaw).
fn opening_normal(o: &zoo_core::scene::Opening, data: &zoo_core::LevelData) -> Option<Vec2> {
    let rect = match &o.kind {
        OpeningKind::BuildingDoor { building, .. } => data.element(building)?.rect,
        OpeningKind::EnclosureGate { enclosure } | OpeningKind::GlassDoor { enclosure } => {
            data.element(enclosure)?.rect
        }
        OpeningKind::GardenGate { garden } => data.gardens.iter().find(|g| &g.id == garden)?.rect,
        OpeningKind::MoonDoor { .. } | OpeningKind::LevelGate { .. } => return None,
    };
    let cell = zoo_core::level::cell_of;
    [Vec2::X, -Vec2::X, Vec2::Y, -Vec2::Y]
        .into_iter()
        .find(|&c| {
            !rect.contains(cell(o.center + c * 1.2)) && rect.contains(cell(o.center - c * 1.2))
        })
}

// LAYOUT-032 (QA 2026-09-27, "no door blocked by boxes or similar"): the collider check above
// misses things without a collider — night lamp posts and string-light posts (no collider
// yet, LAYOUT-035), food boxes, items and furniture props. None of them may stand in an
// opening or in the 1.4 m walkway in front of it (its width plus the player's radius), and no
// lamp post in the 2.5 m leading lane, on the sides the player or a led animal uses. No
// exception for the food storages (Q-150 answered 2026-09-27: the box row leaves a free gap
// ≥ 1.2 m wide in front of the door, the whole door opening inside it).
#[test]
fn layout_032_no_lamp_box_item_or_prop_in_a_walkway() {
    use zoo_core::collision::PLAYER_RADIUS_M;
    use zoo_core::coords::world_to_level;
    use zoo_core::night_scene::NightScene;
    let data = common::zoo_with_night();
    let s = LevelScene::build(&data);
    let night = NightScene::build(&data);
    // (what, position): everything that stands on the ground and is not a mounted lamp
    let mut things: Vec<(String, Vec2)> = Vec::new();
    for p in &night.placements {
        if matches!(p.model, "lantern_post" | "string_post" | "string_lights") {
            things.push((p.model.to_owned(), world_to_level(p.pos)));
        }
    }
    for f in &data.food_boxes {
        things.push((format!("food_box {}", f.food), f.pos()));
    }
    for it in &data.items {
        things.push((format!("item {}", it.id), it.pos()));
    }
    for pr in &data.props {
        things.push((format!("prop {}", pr.id), pr.pos()));
    }
    for w in &data.water_sources {
        if let Some(p) = w.pos {
            things.push((format!("water_source {}", w.id), Vec2::from(p)));
        }
    }
    let mut bad = Vec::new();
    for o in &s.openings {
        let sides = match (&o.kind, opening_normal(o, &data)) {
            (OpeningKind::BuildingDoor { enterable, .. }, Some(n)) => {
                if *enterable {
                    vec![n, -n]
                } else {
                    vec![n]
                }
            }
            (_, Some(n)) => vec![n, -n], // gates: the player outside, the animal inside
            (_, None) => {
                let n = rot_level_neg_y(s.placements[o.placement].yaw);
                vec![n, -n]
            }
        };
        let half = o.opening_m / 2.0 + PLAYER_RADIUS_M;
        for side in sides {
            let along = side.perp();
            for (what, p) in &things {
                let (d, t) = ((*p - o.center).dot(side), (*p - o.center).dot(along));
                // lamp posts: never on a gate's leading cells (GAME-LAYOUT [[light]] rules) —
                // the 2.5 m lane straight in front, the post's radius 0.12 m included
                let post = what.contains("post") || what.starts_with("string");
                let (depth, width) = if post {
                    (2.5, half + 0.12)
                } else {
                    (1.4, half)
                };
                if (0.0..=depth).contains(&d) && t.abs() <= width {
                    bad.push(format!(
                        "{:?}: {what} at {p} ({d:.2} m in front, {t:+.2} m aside)",
                        o.kind
                    ));
                }
            }
        }
    }
    // Q-150: the food-box row in front of a storage door has a gap ≥ 1.2 m around the door
    const BOX_HALF_M: f32 = 0.31; // `food_box` footprint half width
    for o in &s.openings {
        let OpeningKind::BuildingDoor { building, .. } = &o.kind else {
            continue;
        };
        let Some(n) = opening_normal(o, &data) else {
            continue;
        };
        let (mut left, mut right) = (f32::NEG_INFINITY, f32::INFINITY);
        for f in &data.food_boxes {
            let (d, t) = (
                (f.pos() - o.center).dot(n),
                (f.pos() - o.center).dot(n.perp()),
            );
            if (0.0..=1.4).contains(&d) && t.abs() < 3.0 {
                if t < 0.0 {
                    left = left.max(t + BOX_HALF_M);
                } else {
                    right = right.min(t - BOX_HALF_M);
                }
            }
        }
        if left > -0.5 || right < 0.5 || right - left < 1.2 {
            bad.push(format!(
                "{building}: food-box gap {left:+.2}…{right:+.2} m around the door (≥ 1.2 m, door ±0.5)"
            ));
        }
    }
    assert!(
        bad.is_empty(),
        "{} things in walkways:\n{}",
        bad.len(),
        bad.join("\n")
    );
}

fn rot_level_neg_y(yaw: f32) -> Vec2 {
    zoo_core::scene::rot_level(Vec2::NEG_Y, yaw)
}

// LAYOUT-034 (QA 2026-09-27): nothing beside an opening forms a pocket that catches the
// player on the way in. For every opening she may pass (doors of enterable buildings, the
// garden gate, enclosure gates and glass doors while leading an animal), she walks straight
// at it — from 3 m in front, from 2.5 m out and 2.5 m aside (≈ 45°) and from 1.5 m out and
// 3 m aside (along the frontage), both sides — and reaches the opening without getting stuck
// (< 5 cm progress in a second). Start
// points on unwalkable cells or in colliders are skipped. No exceptions (Q-157 answered
// 2026-09-27: `board_zebra`, `tap_l3` and the night-house boards were moved, LAYOUT-038).
#[test]
fn layout_034_openings_reachable_from_the_front_and_at_an_angle() {
    use zoo_core::collision::PLAYER_RADIUS_M;
    let data = common::zoo_with_night();
    let s = LevelScene::build(&data);
    let mut bad = Vec::new();
    let mut checked = 0;
    for o in &s.openings {
        let (owner, gate) = match &o.kind {
            OpeningKind::EnclosureGate { enclosure } | OpeningKind::GlassDoor { enclosure } => {
                (enclosure.clone(), true)
            }
            OpeningKind::BuildingDoor {
                building,
                enterable: true,
            } => (building.clone(), false),
            OpeningKind::GardenGate { garden } => (garden.clone(), false),
            _ => continue,
        };
        let n = opening_normal(o, &data).expect("opening normal");
        let a = n.perp();
        for (out, aside) in [
            (3.0f32, 0.0f32),
            (2.5, -2.5),
            (2.5, 2.5),
            (1.5, -3.0),
            (1.5, 3.0),
        ] {
            let mut g = common::night_game(1);
            let start = o.center + n * out + a * aside;
            if !g
                .level
                .grid()
                .is_walkable(zoo_core::level::cell_of(start), false)
                || g.level.colliders().overlaps(start, PLAYER_RADIUS_M + 0.03)
            {
                continue;
            }
            checked += 1;
            g.player.pos = start;
            let aim = o.center - n * if gate { 0.3 } else { 1.0 };
            let params = g.move_params;
            let mut reached = false;
            let mut last = g.player.pos;
            for frame in 1..=900 {
                let dir = (aim - g.player.pos).normalize_or_zero();
                if gate {
                    // leading: the gate cells are walkable (GAME-RESCUE)
                    g.player.step_with(
                        g.level.grid(),
                        g.level.colliders(),
                        &params,
                        dir,
                        1.0 / 60.0,
                        true,
                    );
                } else {
                    g.update(1.0 / 60.0, dir);
                }
                if (g.player.pos - o.center).dot(n) < if gate { 0.1 } else { -0.8 } {
                    reached = true;
                    break;
                }
                // stuck: less than 5 cm in the last second
                if frame % 60 == 0 {
                    if g.player.pos.distance(last) < 0.05 {
                        break;
                    }
                    last = g.player.pos;
                }
            }
            if !reached {
                bad.push(format!(
                    "{owner}: from {out} m out, {aside:+} m aside ({start}) stuck at {}",
                    g.player.pos
                ));
            }
        }
    }
    assert!(checked > 40, "only {checked} approaches checked");
    assert!(bad.is_empty(), "{}", bad.join("\n"));
}

// LAYOUT-038 (Q-157 answered 2026-09-27, "no pocket beside a door or gate"): nothing solid
// stands within 0.9 m beside the posts of an opening the player passes (doors of enterable
// buildings, enclosure gates and glass doors, the garden gate, the moon door) — neither a
// prop collider nor a solid cell sticking out of the wall / fence / hedge line (an info
// board cell, a tap, a sign) — so walking at the opening at 45° she never gets caught in the
// corner between such a thing and the wall. The door line itself (facade, fence, hedge
// cells) does not count. Non-enterable doors (food storages) are only walked up to; their
// box rows are covered by LAYOUT-032. The level gates (GAME-LAYOUT "Gates between the
// levels", LAYOUT-036) are openings too.
#[test]
fn layout_038_no_pocket_beside_a_door_or_gate() {
    use glam::IVec2;
    use zoo_core::collision::Shape;
    use zoo_core::level::cell_of;
    let data = common::zoo_with_night();
    let g = common::night_game(1);
    let s = LevelScene::build(&data);
    let grid = g.level.grid();
    const BESIDE_M: f32 = 0.9;
    // distance from a point to a collider shape (0 inside)
    let shape_dist = |sh: &Shape, p: Vec2| -> f32 {
        match *sh {
            Shape::Circle { c, r } => (p.distance(c) - r).max(0.0),
            Shape::Box { c, u, half } => {
                let d = p - c;
                let l = Vec2::new(d.dot(u), d.dot(u.perp()));
                (l.abs() - half).max(Vec2::ZERO).length()
            }
        }
    };
    let cell_dist = |p: Vec2, c: IVec2| {
        let min = c.as_vec2();
        (p - p.clamp(min, min + Vec2::ONE)).length()
    };
    // hedge / zoo-wall bands (and the level boundary) continue the wall line, whatever their
    // thickness
    let band = |c: IVec2| {
        data.elements.iter().any(|e| {
            (e.ty == ElementType::Boundary
                || matches!(e.kind.as_deref(), Some("hedge" | "zoo_wall")))
                && e.rect.contains(c)
        })
    };
    let mut bad = Vec::new();
    let mut checked = 0;
    for o in &s.openings {
        let passable = match &o.kind {
            OpeningKind::BuildingDoor { enterable, .. } => *enterable,
            _ => true,
        };
        if !passable {
            continue;
        }
        let gate_like = matches!(
            o.kind,
            OpeningKind::EnclosureGate { .. } | OpeningKind::GlassDoor { .. }
        );
        checked += 1;
        // a level gate's own pillars stand on its barrier cells (LAYOUT-036)
        let own = match &o.kind {
            OpeningKind::LevelGate { barrier } => data.element(barrier).map(|e| e.rect),
            _ => None,
        };
        let n = opening_normal(o, &data)
            .unwrap_or_else(|| rot_level_neg_y(s.placements[o.placement].yaw));
        let along = n.perp();
        let posts = [
            o.center + along * (o.opening_m / 2.0),
            o.center - along * (o.opening_m / 2.0),
        ];
        // the line of the opening (facade / fence / hedge row) and the opening's own cells
        // the row of the opening's own cells (gate / door cells): the fence / facade line
        let line_cell = cell_of(o.center - n * 0.25);
        let line_d = (line_cell.as_vec2() + Vec2::splat(0.5) - o.center).dot(n);
        let on_line = |c: IVec2| {
            let d = (c.as_vec2() + Vec2::splat(0.5) - o.center).dot(n);
            (d - line_d).abs() <= 0.05
        };
        let in_opening = |p: Vec2| (p - o.center).dot(along).abs() < o.opening_m / 2.0 - 0.01;
        for post in posts {
            for sh in g.level.colliders().shapes() {
                let d = shape_dist(sh, post);
                // thin fence runs (garden fence, 0.12 m) are walls, not things standing
                // beside the post
                let thin = match *sh {
                    Shape::Box { half, .. } => half.min_element() <= 0.07,
                    Shape::Circle { .. } => false,
                };
                // (the gate / door model's own posts and hinges stand on the post)
                if d < BESIDE_M && d > 0.02 && !thin {
                    let (lo, hi) = sh.aabb();
                    let c = (lo + hi) / 2.0;
                    if !in_opening(c) {
                        bad.push(format!(
                            "{:?} at {}: collider {:?} {d:.2} m beside the post {post}",
                            o.kind,
                            o.center,
                            sh.aabb()
                        ));
                    }
                }
            }
            let pc = zoo_core::level::cell_of(post);
            for z in pc.y - 2..=pc.y + 2 {
                for x in pc.x - 2..=pc.x + 2 {
                    let c = IVec2::new(x, z);
                    // (the opening's own cells, e.g. the 2 m deep moon door, are not beside it)
                    if grid.is_walkable(c, true)
                        || on_line(c)
                        || band(c)
                        || own.is_some_and(|r| r.contains(c))
                        || in_opening(c.as_vec2() + Vec2::splat(0.5))
                    {
                        continue;
                    }
                    let d = cell_dist(post, c);
                    // enclosures are walked into only by led animals: only the outside counts
                    let out = (c.as_vec2() + Vec2::splat(0.5) - o.center).dot(n);
                    if gate_like && out < 0.0 {
                        continue;
                    }
                    if d < BESIDE_M {
                        bad.push(format!(
                            "{:?} at {}: solid cell {c} {d:.2} m beside the post {post}",
                            o.kind, o.center
                        ));
                    }
                }
            }
        }
    }
    assert!(checked > 15, "only {checked} openings checked");
    assert!(bad.is_empty(), "{} pockets:\n{}", bad.len(), bad.join("\n"));
}

// LAYOUT-035 (GAME-LAYOUT "[[light]]" placement rules, Q-118/Q-137): lantern posts have the
// collider C(0, 0, 0.12) while they are visible (at night) and none by day (QA 2026-09-27
// finding F4 of qa/reports/2026-09-27-doors-blocked.md, fixed).
#[test]
fn layout_035_lantern_posts_are_solid_at_night_only() {
    let mut g = common::night_game(1);
    let post = g
        .level
        .data
        .lights
        .iter()
        .find(|l| l.kind == "lantern_post")
        .and_then(|l| l.pos())
        .expect("a lantern post");
    let walk = |g: &mut zoo_core::Game| {
        g.player.pos = post + Vec2::new(-1.2, 0.0);
        for _ in 0..90 {
            g.update(1.0 / 60.0, Vec2::X);
        }
        g.player.pos.x
    };
    assert!(
        walk(&mut g) > post.x + 0.5,
        "by day the (hidden) post is not solid"
    );
    assert!(g.debug_set_daytime("night"));
    assert!(
        walk(&mut g) < post.x - 0.12 - 0.26,
        "at night the post stops the player"
    );
}

// LAYOUT-039 (Q-173 answered 2026-09-28, GAME-LAYOUT "Doors are never blocked"): within 3 m
// of every door and gate (level gates and the moon door included) nothing solid stands
// 0.1–0.6 m in front of a wall, fence, hedge or facade — a thing there is either flush
// (gap < 0.1 m) or leaves a gap ≥ 0.6 m, so a child walking along the wall towards the
// opening never gets wedged into a slot behind it. Solid things: every prop collider (signs,
// taps, furniture, …) and, at night, the lantern posts; walls: every cell that is not
// walkable (gate cells count as open) and the thin garden-fence runs.
#[test]
fn layout_039_no_wall_gap_near_a_door_or_gate() {
    use glam::IVec2;
    use zoo_core::collision::Shape;
    const NEAR_M: f32 = 3.0;
    const GAP_M: (f32, f32) = (0.1, 0.6);
    let data = common::zoo_with_night();
    let s = LevelScene::build(&data);
    let mut g = common::night_game(1);
    g.level.set_night_solid(true); // lantern posts are solid at night (LAYOUT-035)
    let grid = g.level.grid();
    let thin = |sh: &Shape| match *sh {
        Shape::Box { half, .. } => half.min_element() <= 0.07,
        Shape::Circle { .. } => false,
    };
    // points on a shape's outline (≈ 5 cm apart)
    let outline = |sh: &Shape| -> Vec<Vec2> {
        match *sh {
            Shape::Circle { c, r } => (0..64)
                .map(|i| {
                    let a = i as f32 / 64.0 * std::f32::consts::TAU;
                    c + Vec2::new(a.cos(), a.sin()) * r
                })
                .collect(),
            Shape::Box { c, u, half } => {
                let v = u.perp();
                let mut pts = Vec::new();
                for (a, b, h) in [(u, v, half), (v, u, Vec2::new(half.y, half.x))] {
                    let n = ((h.x * 2.0) / 0.05).ceil().max(1.0) as i32;
                    for i in 0..=n {
                        let t = -h.x + 2.0 * h.x * i as f32 / n as f32;
                        pts.push(c + a * t + b * h.y);
                        pts.push(c + a * t - b * h.y);
                    }
                }
                pts
            }
        }
    };
    let shape_dist = |sh: &Shape, p: Vec2| -> f32 {
        match *sh {
            Shape::Circle { c, r } => (p.distance(c) - r).max(0.0),
            Shape::Box { c, u, half } => {
                let d = p - c;
                let l = Vec2::new(d.dot(u), d.dot(u.perp()));
                (l.abs() - half).max(Vec2::ZERO).length()
            }
        }
    };
    let cell_dist = |p: Vec2, c: IVec2| {
        let min = c.as_vec2();
        (p - p.clamp(min, min + Vec2::ONE)).length()
    };
    let shapes = g.level.colliders().shapes().to_vec();
    let mut bad = Vec::new();
    let mut checked = 0;
    for o in &s.openings {
        let n = opening_normal(o, &data)
            .unwrap_or_else(|| rot_level_neg_y(s.placements[o.placement].yaw));
        let along = n.perp();
        let seg: Vec<Vec2> = (-10..=10)
            .map(|i| o.center + along * (o.opening_m / 2.0) * (i as f32 / 10.0))
            .collect();
        for sh in shapes.iter().filter(|sh| !thin(sh)) {
            // the opening's own gate / door parts (a closed level gate) stand in its line
            let (lo, hi) = sh.aabb();
            let mid = (lo + hi) / 2.0 - o.center;
            if mid.dot(along).abs() < o.opening_m / 2.0 && mid.dot(n).abs() < 0.5 {
                continue;
            }
            let near = seg
                .iter()
                .map(|&p| shape_dist(sh, p))
                .fold(f32::MAX, f32::min);
            if near > NEAR_M {
                continue;
            }
            checked += 1;
            let pts = outline(sh);
            let (lo, hi) = sh.aabb();
            let (clo, chi) = (
                zoo_core::level::cell_of(lo - Vec2::ONE),
                zoo_core::level::cell_of(hi + Vec2::ONE),
            );
            let mut gap = f32::MAX;
            for z in clo.y..=chi.y {
                for x in clo.x..=chi.x {
                    let c = IVec2::new(x, z);
                    if grid.is_walkable(c, true) {
                        continue;
                    }
                    for &p in &pts {
                        gap = gap.min(cell_dist(p, c));
                    }
                }
            }
            for w in shapes.iter().filter(|w| thin(w)) {
                for &p in &pts {
                    gap = gap.min(shape_dist(w, p));
                }
            }
            if gap > GAP_M.0 && gap < GAP_M.1 {
                bad.push(format!(
                    "{:?} at {}: collider {:?} {gap:.2} m in front of a wall ({near:.2} m from the opening)",
                    o.kind,
                    o.center,
                    sh.aabb()
                ));
            }
        }
    }
    assert!(checked > 10, "only {checked} things near openings checked");
    assert!(
        bad.is_empty(),
        "{} wall gaps:\n{}",
        bad.len(),
        bad.join("\n")
    );
}

// LAYOUT-041 (user request 2026-09-28, "we can enter all unlocked doors"): every building
// with a door is enterable — it has an `interior`, its door cell touches an interior cell and
// a walkable cell outside, every interior cell is reachable from the door over interior
// cells, its door model is an enterable `BuildingDoor` that opens while the player is near,
// its roof hides inside (building model or procedural roof region), and every stock box /
// furniture inside is solid.
#[test]
fn layout_041_every_building_with_a_door_is_enterable() {
    use zoo_core::level::{cell_of, CellKind, Surface};
    let data = common::zoo_with_night();
    let s = LevelScene::build(&data);
    let g = common::night_game(1);
    let grid = g.level.grid();
    let mut n = 0;
    for e in data.elements_of(ElementType::Building) {
        let Some(door) = e.door_cell() else { continue };
        n += 1;
        let inner = e
            .interior
            .unwrap_or_else(|| panic!("{}: a door but no interior", e.id));
        assert!(e.is_enterable(), "{}", e.id);
        let n4 = [
            glam::IVec2::X,
            glam::IVec2::NEG_X,
            glam::IVec2::Y,
            glam::IVec2::NEG_Y,
        ];
        assert!(
            n4.iter().any(|d| inner.contains(door + *d)),
            "{}: door not next to the interior",
            e.id
        );
        assert!(
            n4.iter()
                .any(|d| !e.rect.contains(door + *d) && grid.is_walkable(door + *d, false)),
            "{}: door leads nowhere",
            e.id
        );
        // flood fill from the door over the building's open cells
        let mut seen = vec![door];
        let mut i = 0;
        while i < seen.len() {
            let c = seen[i];
            i += 1;
            for d in n4 {
                let q = c + d;
                if e.is_open_cell(q)
                    && grid.kind(q) == CellKind::Walkable(Surface::Path)
                    && !seen.contains(&q)
                {
                    seen.push(q);
                }
            }
        }
        for c in inner.cells() {
            assert!(
                seen.contains(&c),
                "{}: interior cell {c} not reachable",
                e.id
            );
        }
        // the door opens for the player (enterable), shut when she is away
        let o = s
            .openings
            .iter()
            .find(|o| matches!(&o.kind, OpeningKind::BuildingDoor { building, enterable: true } if *building == e.id))
            .unwrap_or_else(|| panic!("{}: no enterable door model", e.id));
        let mut gg = common::night_game(1);
        gg.player.pos = o.center;
        assert!(gg.opening_open(o), "{}: door stays shut", e.id);
        gg.player.pos = o.center + Vec2::new(3.0, 3.0);
        assert!(!gg.opening_open(o), "{}: door open while away", e.id);
        // roof hides inside (zoo view): model with roof nodes, or a procedural roof region
        assert!(
            s.building_models.iter().any(|b| b.element == e.id)
                || s.roof_boxes.iter().any(|(id, _)| *id == e.id),
            "{}: no roof to hide",
            e.id
        );
    }
    // 8 since the zookeeper house of level 2 (every bed is indoors, LAYOUT-047)
    assert_eq!(n, 8, "buildings with a door");
    // the stock boxes inside a building are solid (never walked through, Q-194); the labelled
    // food boxes stand outside (Q-181 answered)
    for p in data
        .props
        .iter()
        .filter(|p| matches!(p.model.as_str(), "food_box" | "food_box_stack"))
    {
        let e = data
            .element(p.building.as_deref().expect("stock box of a building"))
            .unwrap();
        assert!(
            e.rect.contains(cell_of(p.pos())),
            "{} outside {}",
            p.id,
            e.id
        );
        assert!(
            g.level.colliders().overlaps(p.pos(), 0.1)
                && !grid.is_walkable(cell_of(p.pos()), false),
            "{}: not solid",
            p.id
        );
    }
}
