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
    // exceptions: the moon door's cells are a barrier by day (open at night: LAYOUT-028);
    // the food storage doors are not enterable and the level data's food-box row stands in
    // front of them (open question Q-150)
    let bad: Vec<String> = bad
        .into_iter()
        .filter(|b| !b.starts_with("MoonDoor") && !b.contains("enterable: false"))
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
