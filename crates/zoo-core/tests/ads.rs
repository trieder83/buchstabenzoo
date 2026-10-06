//! GAME-ADS: ad boards in the level data, seeded campaign assignment, reading range.
mod common;

use glam::Vec2;
use zoo_core::ads::{
    assign_slots, assign_slots_by_level, near_board, AdBoardData, AdVariant, MIN_BOARDS_PER_SLOT,
    SLOTS,
};
use zoo_core::level::cell_center;
use zoo_core::{CellKind, ElementType, LevelData};

fn levels() -> [(&'static str, LevelData); 3] {
    [
        ("level_1", common::level1()),
        ("level_2", common::level2()),
        ("level_3", common::level3()),
    ]
}

fn board(id: &str, x: f32, z: f32, facing: &str) -> AdBoardData {
    AdBoardData {
        id: id.into(),
        pos: [x, z],
        facing: facing.into(),
        variant: AdVariant::Post,
        part: 0,
    }
}

// ADS-001
#[test]
fn ads_001_every_day_level_has_4_post_boards_clear_of_everything() {
    for (name, data) in levels() {
        let n = data.ad_boards.len();
        assert_eq!(n, 4, "{name}: {n} ad boards");
        assert!(data.ad_boards.iter().all(|b| b.variant == AdVariant::Post));
        let mut ids: Vec<&str> = data.ad_boards.iter().map(|b| b.id.as_str()).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), n, "{name}: duplicate board ids");
        for b in &data.ad_boards {
            let c = b.pos();
            let half = b.footprint_half();
            // never on / in front of an enclosure, info board, building door, hiding place,
            // scenery, garden, item, food box, water source, lamp
            let rect_near = |r: zoo_core::Rect, margin: f32| {
                c.x + half.x > r.x as f32 - margin
                    && c.x - half.x < (r.x + r.w) as f32 + margin
                    && c.y + half.y > r.z as f32 - margin
                    && c.y - half.y < (r.z + r.d) as f32 + margin
            };
            for e in &data.elements {
                let margin = match (e.ty, e.kind.as_deref()) {
                    (ElementType::HidingPlace, _) => 0.0,
                    _ => 0.0,
                };
                assert!(
                    !rect_near(e.rect, margin),
                    "{name}: board {} overlaps or crowds {} ({:?})",
                    b.id,
                    e.id,
                    e.ty
                );
                if e.kind.as_deref() == Some("info_board") {
                    let centre = Vec2::new(e.rect.x as f32 + 0.5, e.rect.z as f32 + 0.5);
                    assert!(
                        c.distance(centre) > 4.0,
                        "{name}: {} near info board {}",
                        b.id,
                        e.id
                    );
                }
                if let (ElementType::Enclosure, Some(g)) = (e.ty, e.gate) {
                    assert!(
                        c.distance(Vec2::new(g.x as f32, g.z as f32)) > 5.0,
                        "{}: near gate {}",
                        b.id,
                        e.id
                    );
                }
                if let Some(d) = e.door {
                    assert!(
                        c.distance(cell_center(glam::IVec2::new(d[0], d[1]))) > 5.0,
                        "{}: near door {}",
                        b.id,
                        e.id
                    );
                }
            }
            for h in &data.hiding_places {
                assert!(
                    !rect_near(h.rect, 0.0),
                    "{name}: {} in hiding place {}",
                    b.id,
                    h.id
                );
            }
            for s in &data.scenery {
                assert!(
                    !rect_near(s.rect, 0.0),
                    "{name}: {} in scenery {}",
                    b.id,
                    s.id
                );
            }
            for g in &data.gardens {
                assert!(
                    !rect_near(g.rect, 3.0),
                    "{name}: {} at garden {}",
                    b.id,
                    g.id
                );
            }
            for f in &data.food_boxes {
                assert!(c.distance(f.pos()) > 4.0, "{name}: {} near food box", b.id);
            }
            for l in &data.lights {
                if let Some(p) = l.pos() {
                    assert!(c.distance(p) > 1.6, "{name}: {} near lamp", b.id);
                }
            }
            // stands on open walkable ground (not path, not solid) inside the level
            let cells = if b.facing().x.abs() > 0.5 {
                (1, 3)
            } else {
                (3, 1)
            };
            let (x0, z0) = (
                (c.x - cells.0 as f32 / 2.0).round() as i32,
                (c.y - cells.1 as f32 / 2.0).round() as i32,
            );
            let grid = zoo_core::Grid::build(&data, &Default::default());
            for dx in 0..cells.0 {
                for dz in 0..cells.1 {
                    let cell = glam::IVec2::new(x0 + dx, z0 + dz);
                    assert!(
                        matches!(
                            grid.kind(cell),
                            CellKind::Walkable(zoo_core::Surface::Grass)
                        ),
                        "{name}: {} stands on {:?} at {cell}",
                        b.id,
                        grid.kind(cell)
                    );
                }
            }
        }
    }
}

// ADS-001 (the joined zoo keeps the boards)
#[test]
fn ads_001_joined_zoo_has_all_boards_with_parts() {
    let zoo = common::zoo();
    assert_eq!(zoo.ad_boards.len(), 12);
    assert_eq!(zoo.ad_boards.iter().filter(|b| b.part == 1).count(), 4);
    // the whole game: 3 day levels x 4 + 2 night levels x 3 (user request 2026-10-06)
    let all = common::zoo_with_night2();
    assert_eq!(all.ad_boards.len(), 18);
    for part in 0..5 {
        let n = all.ad_boards.iter().filter(|b| b.part == part).count();
        assert_eq!(n, if part < 3 { 4 } else { 3 }, "part {part}");
    }
}

fn counts_by_part(all: &LevelData, slots: &[u8], part: usize) -> [usize; SLOTS] {
    let mut c = [0; SLOTS];
    for (b, &s) in all.ad_boards.iter().zip(slots) {
        if b.part == part {
            c[s as usize] += 1;
        }
    }
    c
}

// ADS-002 (every level shows all three slots, any seed)
#[test]
fn ads_002_every_level_has_all_three_slots() {
    let all = common::zoo_with_night2();
    let ids: Vec<&str> = all.ad_boards.iter().map(|b| b.id.as_str()).collect();
    let parts: Vec<usize> = all.ad_boards.iter().map(|b| b.part).collect();
    for seed in 0..300 {
        let slots = assign_slots_by_level(&ids, &parts, seed);
        let mut total = [0; SLOTS];
        for part in 0..5 {
            let c = counts_by_part(&all, &slots, part);
            assert!(c.iter().all(|&n| n >= 1), "seed {seed} part {part}: {c:?}");
            if part >= 3 {
                assert_eq!(c, [1, 1, 1], "seed {seed} night part {part}");
            }
            for k in 0..SLOTS {
                total[k] += c[k];
            }
        }
        assert!(total.iter().all(|&n| n >= MIN_BOARDS_PER_SLOT), "{total:?}");
    }
    // same seed -> same deal; another seed changes it
    let a = assign_slots_by_level(&ids, &parts, 5);
    assert_eq!(a, assign_slots_by_level(&ids, &parts, 5));
    assert!((6..30).any(|s| assign_slots_by_level(&ids, &parts, s) != a));
}

fn night_levels() -> [(&'static str, LevelData); 2] {
    [("night_1", common::night1()), ("night_2", common::night2())]
}

/// Clearances every board keeps (ADS-001 / ADS-035): not on an element or hiding place /
/// scenery / garden (except the wall a poster hangs on), 4 m from info boards and food boxes,
/// 5 m from gates and doors, 1.6 m from lamps.
fn check_clear(name: &str, data: &LevelData, b: &AdBoardData, wall: Option<&str>) {
    // a flat poster needs no walkway: 4 m to a gate / door instead of 5 m
    let open_min = if b.variant == AdVariant::Poster {
        4.0
    } else {
        5.0
    };
    let c = b.pos();
    let half = b.footprint_half();
    let rect_near = |r: zoo_core::Rect, margin: f32| {
        c.x + half.x > r.x as f32 - margin
            && c.x - half.x < (r.x + r.w) as f32 + margin
            && c.y + half.y > r.z as f32 - margin
            && c.y - half.y < (r.z + r.d) as f32 + margin
    };
    for e in &data.elements {
        if wall == Some(e.id.as_str()) {
            continue;
        }
        assert!(
            !rect_near(e.rect, 0.0),
            "{name}: {} overlaps {}",
            b.id,
            e.id
        );
        if e.kind.as_deref() == Some("info_board") {
            let centre = Vec2::new(e.rect.x as f32 + 0.5, e.rect.z as f32 + 0.5);
            assert!(
                c.distance(centre) >= 4.0,
                "{name}: {} near info board {}",
                b.id,
                e.id
            );
        }
        if let (ElementType::Enclosure, Some(g)) = (e.ty, e.gate) {
            assert!(
                c.distance(Vec2::new(g.x as f32, g.z as f32)) > open_min,
                "{name}: {} near gate",
                b.id
            );
        }
        if let Some(d) = e.door {
            assert!(
                c.distance(cell_center(glam::IVec2::new(d[0], d[1]))) > open_min,
                "{name}: {} near door {}",
                b.id,
                e.id
            );
        }
    }
    for h in &data.hiding_places {
        assert!(
            !rect_near(h.rect, 1.0),
            "{name}: {} at hiding place {}",
            b.id,
            h.id
        );
    }
    for sc in &data.scenery {
        assert!(
            !rect_near(sc.rect, 1.0),
            "{name}: {} at scenery {}",
            b.id,
            sc.id
        );
    }
    for g in &data.gardens {
        assert!(
            !rect_near(g.rect, 3.0),
            "{name}: {} at garden {}",
            b.id,
            g.id
        );
    }
    for f in &data.food_boxes {
        // stock crates inside the hut the poster hangs on stand behind its wall
        let behind_wall = wall
            .and_then(|w| data.element(w))
            .is_some_and(|e| e.rect.contains(zoo_core::level::cell_of(f.pos())));
        if behind_wall {
            continue;
        }
        assert!(
            c.distance(f.pos()) >= 4.0,
            "{name}: {} near a food box",
            b.id
        );
    }
    for l in &data.lights {
        if let Some(p) = l.pos() {
            assert!(c.distance(p) > 1.6, "{name}: {} near lamp {}", b.id, l.id);
        }
    }
}

// ADS-001 / ADS-035: night levels have exactly 3 boards (posters on walls, a post on grass)
#[test]
fn ads_035_night_levels_have_three_boards_posters_on_walls() {
    for (name, data) in night_levels() {
        assert_eq!(data.ad_boards.len(), 3, "{name}");
        let mut ids: Vec<&str> = data.ad_boards.iter().map(|b| b.id.as_str()).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), 3, "{name}: duplicate ids");
        let grid = zoo_core::Grid::build(&data, &Default::default());
        let walkable = |cell: glam::IVec2| matches!(grid.kind(cell), CellKind::Walkable(_));
        let cell = |x: f32, z: f32| glam::IVec2::new(x.floor() as i32, z.floor() as i32);
        let mut posters = 0;
        let flyers = data
            .ad_boards
            .iter()
            .filter(|b| b.variant == AdVariant::Flyer)
            .count();
        assert!(flyers <= 1, "{name}: at most one flyer per night level");
        for b in &data.ad_boards {
            let c = b.pos();
            if b.variant != AdVariant::Poster {
                // post / flyer: same rules as the day boards (open grass under the footprint)
                check_clear(name, &data, b, None);
                let h = b.footprint_half();
                for dx in [-h.x + 0.1, 0.0, h.x - 0.1] {
                    for dz in [-1.0, 0.0, 1.0] {
                        let k = grid.kind(cell(c.x + dx, c.y + dz));
                        assert!(
                            matches!(k, CellKind::Walkable(zoo_core::Surface::Grass)),
                            "{name}: {} stands on {k:?}",
                            b.id
                        );
                    }
                }
                continue;
            }
            posters += 1;
            assert_eq!(b.facing, "-z", "{name}: {} must face the camera side", b.id);
            // the wall: solid behind the whole picture width, a single element
            let mut wall: Option<&str> = None;
            for dx in [-1.0_f32, 0.0, 1.0] {
                let behind = cell(c.x + dx, c.y + 0.5);
                assert!(
                    !walkable(behind),
                    "{name}: {} no wall behind at dx {dx}",
                    b.id
                );
                let owner = data.elements.iter().find(|e| {
                    e.rect.contains(behind) || e.footprint_extra.iter().any(|r| r.contains(behind))
                });
                if let Some(e) = owner {
                    assert!(
                        wall.is_none_or(|w| w == e.id),
                        "{name}: {} spans two walls",
                        b.id
                    );
                    wall = Some(e.id.as_str());
                }
            }
            check_clear(name, &data, b, wall);
            // open stand area 2 m wide, 3 m deep in front, no tree canopy / building in the view
            for dx in [-1.0_f32, 0.0, 1.0] {
                for dz in [0.5_f32, 1.5, 2.5] {
                    let k = grid.kind(cell(c.x + dx, c.y - dz));
                    assert!(
                        matches!(k, CellKind::Walkable(_)),
                        "{name}: {} stand area blocked at ({dx}, -{dz}): {k:?}",
                        b.id
                    );
                }
            }
            for e in &data.elements {
                if e.ty == ElementType::Path || wall == Some(e.id.as_str()) {
                    continue;
                }
                let (x0, x1, z0, z1) = (c.x - 2.0, c.x + 2.0, c.y - 6.0, c.y);
                let hit = (e.rect.x as f32) < x1
                    && ((e.rect.x + e.rect.w) as f32) > x0
                    && (e.rect.z as f32) < z1
                    && ((e.rect.z + e.rect.d) as f32) > z0;
                assert!(
                    !hit,
                    "{name}: {} hidden behind {} (view from the south)",
                    b.id, e.id
                );
            }
        }
        assert!(posters >= 2, "{name}: night boards default to posters");
    }
}

// ADS-035: a poster adds no collider and has its board lamp at night
#[test]
fn ads_035_poster_has_no_collider_and_a_lamp() {
    let data = common::night2();
    let scene = zoo_core::scene::LevelScene::build(&data);
    let night = zoo_core::night_scene::NightScene::build(&data);
    for b in data
        .ad_boards
        .iter()
        .filter(|b| b.variant == AdVariant::Poster)
    {
        assert!(scene
            .decals
            .iter()
            .any(|d| d.id == zoo_core::ads::texture_id(&b.id)));
        let c = b.pos();
        assert!(
            !scene.box_colliders.iter().any(
                |s| matches!(s, zoo_core::collision::Shape::Box { c: p, .. } if p.distance(c) < 0.5)
            ),
            "{}: poster must not add a collider",
            b.id
        );
        assert!(
            night
                .lamps
                .iter()
                .any(|l| l.id == b.id && l.kind == "board_lamp"),
            "{}: no lamp",
            b.id
        );
    }
}

fn counts(slots: &[u8]) -> [usize; SLOTS] {
    let mut c = [0; SLOTS];
    for &s in slots {
        c[s as usize] += 1;
    }
    c
}

// ADS-002
#[test]
fn ads_002_each_slot_on_at_least_two_boards() {
    let zoo = common::zoo();
    let ids: Vec<&str> = zoo.ad_boards.iter().map(|b| b.id.as_str()).collect();
    for seed in 0..200 {
        let c = counts(&assign_slots(&ids, seed));
        assert!(
            c.iter().all(|&n| n >= MIN_BOARDS_PER_SLOT),
            "seed {seed}: {c:?}"
        );
        assert_eq!(c.iter().sum::<usize>(), ids.len());
    }
    // 6 boards: exactly 2 each
    let six = ["a", "b", "c", "d", "e", "f"];
    assert_eq!(counts(&assign_slots(&six, 3)), [2, 2, 2]);
}

// ADS-005
#[test]
fn ads_005_same_seed_same_assignment_other_seeds_differ() {
    let zoo = common::zoo();
    let ids: Vec<&str> = zoo.ad_boards.iter().map(|b| b.id.as_str()).collect();
    assert_eq!(assign_slots(&ids, 7), assign_slots(&ids, 7));
    let distinct: std::collections::BTreeSet<Vec<u8>> =
        (0..20).map(|s| assign_slots(&ids, s)).collect();
    assert!(
        distinct.len() >= 15,
        "only {} distinct assignments in 20 seeds",
        distinct.len()
    );
    // changing the order of the data does not change which board gets which slot
    let mut rev = ids.clone();
    rev.reverse();
    let a = assign_slots(&ids, 11);
    let mut b = assign_slots(&rev, 11);
    b.reverse();
    assert_eq!(a.len(), b.len());
    let order_independent = a.iter().zip(&b).filter(|(x, y)| x == y).count();
    assert!(order_independent > 0);
}

// ADS-007
#[test]
fn ads_007_reading_range_is_in_front_of_the_picture_only() {
    let b = [board("b", 10.0, 10.0, "-z")];
    let at = |x: f32, z: f32| near_board(&b, Vec2::new(x, z));
    assert_eq!(at(10.0, 8.5), Some(0), "in front, 1.5 m south");
    assert_eq!(at(11.5, 8.0), Some(0), "in front, a bit aside");
    assert_eq!(at(10.0, 6.8), Some(0), "3.2 m: still near (touch, ADS-007)");
    assert_eq!(at(12.5, 7.5), None, "too far (3.9 m)");
    assert_eq!(at(10.0, 11.5), None, "behind the board");
    assert_eq!(at(13.5, 9.5), None, "beside the board");
    let e = [board("e", 0.0, 0.0, "+x")];
    assert_eq!(near_board(&e, Vec2::new(2.0, 0.5)), Some(0));
    assert_eq!(near_board(&e, Vec2::new(-2.0, 0.5)), None);
    let two = [board("p", 10.0, 10.0, "-z"), board("q", 10.0, 7.5, "+z")];
    assert_eq!(
        near_board(&two, Vec2::new(10.0, 8.5)),
        Some(1),
        "the nearer board"
    );
}

// ADS-037: a flyer lies on walkable grass, is not solid, is read from any side within 2.5 m
#[test]
fn ads_037_flyer_is_flat_walkable_and_read_from_any_side() {
    let data = common::night1();
    let scene = zoo_core::scene::LevelScene::build(&data);
    let f = data
        .ad_boards
        .iter()
        .find(|b| b.variant == AdVariant::Flyer)
        .expect("night_1 uses its flyer");
    assert!(!scene.box_colliders.iter().any(
        |s| matches!(s, zoo_core::collision::Shape::Box { c, .. } if c.distance(f.pos()) < 1.0)
    ));
    let d = scene
        .decals
        .iter()
        .find(|d| d.id == zoo_core::ads::texture_id(&f.id))
        .unwrap();
    assert!(d.normal().y > 0.99, "lies flat, faces up");
    let at = |dx: f32, dz: f32| near_board(&data.ad_boards, f.pos() + Vec2::new(dx, dz));
    let idx = data.ad_boards.iter().position(|b| b.id == f.id);
    assert_eq!(at(0.0, -1.5), idx, "south");
    assert_eq!(at(1.5, 1.5), idx, "any side");
    assert_eq!(at(0.0, 3.0), None, "too far");
}
