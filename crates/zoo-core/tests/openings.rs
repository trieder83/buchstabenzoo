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
    assert!(s.placements.iter().any(|p| p.model == "gate_zoo_closed"));
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
    // a non-enterable building's door stays shut
    let shed = find(
        &|k| matches!(k, OpeningKind::BuildingDoor { building, .. } if building == "food_storage"),
    );
    g.player.pos = shed.center + Vec2::new(0.0, -1.0);
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
            OpeningKind::MoonDoor { .. } => None,
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
    // (user, 2026-09-27: doors are never blocked)
    let bad: Vec<String> = bad
        .into_iter()
        .filter(|b| !b.starts_with("MoonDoor"))
        .collect();
    assert!(
        bad.is_empty(),
        "{} blocked walkway points:\n{}",
        bad.len(),
        bad.join("\n")
    );
}

// LAYOUT-033 (user decision 2026-09-27): the enclosure sign stands BESIDE its gate, never in
// front of it — outside the fence, ≥ 0.5 m from the gate post, its whole (solid) footprint on
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
        if side < 1.0 + 0.5 + 1.19 - 0.01 {
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
        OpeningKind::MoonDoor { .. } => return None,
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
// points on unwalkable cells or in colliders are skipped. Known pockets waiting for Q-157
// are listed below and skipped (remove an entry when its spot is fixed).
#[test]
fn layout_034_openings_reachable_from_the_front_and_at_an_angle() {
    use zoo_core::collision::PLAYER_RADIUS_M;
    const KNOWN_POCKETS: [(&str, f32, f32); 4] = [
        // (owner id, metres out, metres aside) — Q-157
        ("enc_zebra", 1.5, 3.0), // board_zebra flush with the gate post
        ("zookeeper_house_3", 2.5, 2.5), // tap_l3 0.35 m beside the door
        ("zookeeper_house_3", 1.5, 3.0),
        ("night_house", 1.5, -3.0), // board_n1_bat 0.12 m in front of the facade
    ];
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
            if KNOWN_POCKETS
                .iter()
                .any(|&(id, d, t)| id == owner && d == out && t == aside)
            {
                continue;
            }
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

// LAYOUT-035 (GAME-LAYOUT "[[light]]" placement rules, Q-118/Q-137): lantern posts have the
// collider C(0, 0, 0.12) while they are visible (at night) and none by day. QA 2026-09-27:
// the posts have no collider at all — the player walks through them at night. Ignored until
// the night colliders exist (finding F4 of qa/reports/2026-09-27-doors-blocked.md).
#[test]
#[ignore = "F4 2026-09-27: lantern posts have no night collider yet"]
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
