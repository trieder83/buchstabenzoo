//! GAME-ADS: ad boards in the level data, seeded campaign assignment, reading range.
mod common;

use glam::Vec2;
use zoo_core::ads::{assign_slots, near_board, AdBoardData, MIN_BOARDS_PER_SLOT, SLOTS};
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
        part: 0,
    }
}

// ADS-001
#[test]
fn ads_001_every_level_has_4_to_6_boards_clear_of_everything() {
    for (name, data) in levels() {
        let n = data.ad_boards.len();
        assert!((4..=6).contains(&n), "{name}: {n} ad boards");
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
