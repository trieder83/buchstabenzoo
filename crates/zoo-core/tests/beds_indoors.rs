//! GAME-LAYOUT rule 16 / LAYOUT-047 (user request 2026-10-01: every bed is inside an enterable
//! building), LAYOUT-L2-018 and LAYOUT-L3-019: the beds of levels 1, 2 and 3 stand in their
//! zookeeper houses, the stand cell is reachable through the door, and at each level's nightfall
//! the 🧭 hint points at that bed (never stuck).

mod common;

use glam::{IVec2, Vec2};
use zoo_core::collision::PLAYER_RADIUS_M;
use zoo_core::daytime::{Phase, CELEBRATION_S, DUSK_S};
use zoo_core::hints::{candidates, HintKind, HintTracker};
use zoo_core::level::{cell_center, cell_of, Level, Surface};
use zoo_core::{nav, Game};

const L3_OPEN: [&str; 3] = [
    "barrier_ne_tree",
    "barrier_l2_construction",
    "barrier_north_gate",
];

/// LAYOUT-047: every bed item is indoors, on interior cells, reachable through the door.
#[test]
fn layout_047_every_bed_is_inside_an_enterable_zookeeper_house() {
    let data = common::zoo_with_night();
    let mut level = Level::new(data.clone());
    for b in L3_OPEN {
        assert!(level.open_barrier(b));
    }
    let grid = level.grid();
    let beds: Vec<_> = data.items.iter().filter(|it| it.kind == "bed").collect();
    // one bed per day level 1..3
    let mut per_part: Vec<usize> = beds.iter().map(|b| b.part).collect();
    per_part.sort();
    per_part.dedup();
    assert_eq!(per_part.len(), beds.len(), "one bed per level");
    for lv in ["level_1", "level_2", "level_3"] {
        let k = data.part_index(lv).unwrap();
        assert!(beds.iter().any(|b| b.part == k), "{lv} has no bed");
    }
    for bed in beds {
        let house = data
            .element(bed.building.as_deref().expect("bed has a building"))
            .expect("building exists");
        assert_eq!(house.kind.as_deref(), Some("zookeeper_house"), "{}", bed.id);
        let inner = house.interior.expect("enterable: interior");
        let door = house.door_cell().expect("enterable: door");
        // footprint 2 x 1 m on interior cells, not on the door cell
        for (dx, dz) in [(-0.9f32, -0.4f32), (0.9, -0.4), (-0.9, 0.4), (0.9, 0.4)] {
            let c = cell_of(bed.pos() + Vec2::new(dx, dz));
            assert!(inner.contains(c), "{}: {c} outside the interior", bed.id);
            assert_ne!(c, door, "{}", bed.id);
            assert!(grid.is_walkable(c, false), "{}: {c}", bed.id);
        }
        // stand cell: interior, walkable, 1.0-1.5 m from the bed, free of colliders
        let stand_cell = IVec2::from(bed.stand.expect("stand"));
        assert!(inner.contains(stand_cell), "{}: stand not inside", bed.id);
        assert_ne!(stand_cell, door);
        let stand = cell_center(stand_cell);
        let d = stand.distance(bed.pos());
        assert!(
            (1.0..=1.5).contains(&d),
            "{}: stand {d} m from the bed",
            bed.id
        );
        assert!(
            !level.colliders().overlaps(stand, PLAYER_RADIUS_M),
            "{}",
            bed.id
        );
        // the door touches a street cell outside the building
        let outside_street = [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .iter()
            .map(|&(x, z)| door + IVec2::new(x, z))
            .any(|c| !house.rect.contains(c) && grid.surface(c) == Some(Surface::Path));
        assert!(outside_street, "{}: no street at the door", bed.id);
        // reachable from the spawn of its level through the door
        let k = bed.part;
        let reach = nav::flood_fill(grid, data.parts[k].spawn.cell(), false);
        assert!(
            reach[grid.index(stand_cell).unwrap()],
            "{}: stand not reachable",
            bed.id
        );
    }
}

/// LAYOUT-L2-018 / LAYOUT-L3-019 (hint part): at the nightfall of level 2 / 3 the bed hint is
/// among the top targets and leads to the indoor stand cell of that level's bed.
#[test]
fn layout_l2_018_l3_019_hint_leads_to_the_indoor_bed_at_dusk() {
    let data = common::zoo_with_night();
    let mut g = Game::new(data.clone(), 5).unwrap();
    let step = |g: &mut Game, s: f32| {
        for _ in 0..(s * 60.0) as usize {
            g.update(1.0 / 60.0, Vec2::ZERO);
        }
        g.drain_events();
    };
    for lv in [
        (&["zebra", "hippo", "panda"][..], "level_1", "bed_l1"),
        (
            &["koala", "elephant", "giraffe", "lion"][..],
            "level_2",
            "bed_l2",
        ),
        (&["monkey", "goldfish", "snow_fox"][..], "level_3", "bed_l3"),
    ] {
        let (animals, level_id, bed_id) = lv;
        assert!(g.level_unlocked(level_id), "{level_id} unlocked");
        let k = data.part_index(level_id).unwrap();
        g.player.pos = cell_center(data.parts[k].spawn.cell());
        for a in animals {
            assert!(g.debug_send_home(a), "{a}");
        }
        step(&mut g, CELEBRATION_S + 1.0);
        let bed = data.items.iter().find(|it| it.id == bed_id).unwrap();
        let stand = cell_center(IVec2::from(bed.stand.unwrap()));
        // dusk: the bed is a hint target and the offered stand cell is this level's bed
        let t = HintTracker::default();
        let c = candidates(&g, &t);
        let hint = c
            .iter()
            .find(|h| h.kind == HintKind::Bed)
            .unwrap_or_else(|| panic!("{level_id}: no bed hint at dusk: {c:?}"));
        assert!(
            hint.stand.distance(stand) < 0.01,
            "{level_id}: {:?}",
            hint.stand
        );
        assert_eq!(g.bed_stand(g.player.pos), Some(stand), "{level_id}");
        // the stand cell is reachable over walkable cells from where the child stands
        let grid = g.level.grid();
        let reach = nav::flood_fill(grid, cell_of(g.player.pos), false);
        assert!(reach[grid.index(cell_of(stand)).unwrap()], "{level_id}");
        // at the stand cell facing the bed: sleeping is offered at night
        step(&mut g, DUSK_S + 0.5);
        assert_eq!(g.daytime.phase, Phase::Night, "{level_id}");
        g.player.pos = stand;
        g.player.facing = (bed.pos() - stand).normalize();
        assert_eq!(
            g.available_target(),
            Some(zoo_core::game::Target::Bed),
            "{level_id}"
        );
        if level_id == "level_1" {
            // the next level opens the morning after the night zoo is done (Q-133)
            for a in ["hedgehog", "bat", "owl"] {
                assert!(g.debug_send_home(a), "{a}");
            }
        }
        g.debug_next_morning();
        step(&mut g, 4.0);
    }
}

/// NIGHT-027 (user report 2026-10-02): when the child gets up she stands NEXT to the bed — on
/// its free stand cell, outside the bed's footprint and every collider, feet on the floor — never
/// inside or on top of the bed. Checked for the bed of every level, from several start positions.
#[test]
fn night_027_wakes_up_next_to_the_bed_not_in_it() {
    let data = common::zoo_with_night();
    for (bed_id, level_id) in [
        ("bed_l1", "level_1"),
        ("bed_l2", "level_2"),
        ("bed_l3", "level_3"),
    ] {
        let bed = data.items.iter().find(|it| it.id == bed_id).unwrap();
        let stand = cell_center(IVec2::from(bed.stand.unwrap()));
        for start in [stand, bed.pos(), bed.pos() + Vec2::new(0.4, 0.2)] {
            let mut g = Game::new(data.clone(), 5).unwrap();
            let k = data.part_index(level_id).unwrap();
            // every earlier level is open and the child is in this house
            for b in L3_OPEN {
                g.level.open_barrier(b);
            }
            assert!(g.level_unlocked(level_id));
            g.debug_set_daytime("night");
            g.player.pos = start;
            assert!(g.sleep(), "{bed_id}: sleeping from {start:?}");
            g.debug_next_morning();
            let p = g.player.pos;
            assert!(
                !g.level.colliders().overlaps(p, PLAYER_RADIUS_M),
                "{bed_id} from {start:?}: wakes inside a collider at {p:?}"
            );
            assert!(
                g.level.grid().is_walkable(cell_of(p), false),
                "{bed_id}: not walkable {p:?}"
            );
            let d = p.distance(bed.pos());
            assert!(
                (0.7..=2.3).contains(&d),
                "{bed_id}: {d:.2} m from the bed centre {p:?}"
            );
            let floor = g.level.ground_height(p);
            assert!(
                (g.player.y - floor).abs() < 0.02,
                "{bed_id}: feet {} vs floor {floor}",
                g.player.y
            );
            let _ = k;
        }
    }
}
