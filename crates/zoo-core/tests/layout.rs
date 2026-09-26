//! GAME-LAYOUT and GAME-LEVEL-1 unit tests on `assets/levels/level-1.toml`.

mod common;

use std::collections::BTreeSet;

use glam::IVec2;
use zoo_core::level::{cell_center, CellKind, ElementType, Grid, Level, Surface};
use zoo_core::nav::{flood_fill, min_cost, Cost};

fn solid_types() -> [ElementType; 6] {
    [
        ElementType::Enclosure,
        ElementType::Building,
        ElementType::Landmark,
        ElementType::Barrier,
        ElementType::Boundary,
        ElementType::Decoration,
    ]
}

/// Walkable cells inside or 8-adjacent to an element's rectangle that the flood fill reached.
fn reachable_near(grid: &Grid, reach: &[bool], rect: zoo_core::Rect) -> bool {
    let grown = zoo_core::Rect::new(rect.x - 1, rect.z - 1, rect.w + 2, rect.d + 2);
    grown
        .cells()
        .any(|c| grid.index(c).is_some_and(|k| reach[k]) && grid.is_walkable(c, false))
}

// LAYOUT-001, LAYOUT-L1-001
#[test]
fn layout_001_l1_001_everything_reachable_from_spawn() {
    let data = common::level1();
    let level = Level::new(data.clone());
    let grid = level.grid();
    let reach = flood_fill(grid, data.spawn.cell(), false);
    let mut missing = Vec::new();
    for e in &data.elements {
        let must = match e.ty {
            ElementType::Enclosure
            | ElementType::Building
            | ElementType::Landmark
            | ElementType::Barrier
            | ElementType::HidingPlace => true,
            // hedges and walls need not be reachable (GAME-LEVEL-1 §3)
            _ => false,
        };
        if must && !reachable_near(grid, &reach, e.rect) {
            missing.push(e.id.clone());
        }
    }
    assert!(missing.is_empty(), "not reachable from spawn: {missing:?}");
}

// LAYOUT-002, LAYOUT-L1-002
#[test]
fn layout_002_l1_002_border_sealed() {
    let data = common::level1();
    let level = Level::new(data.clone());
    let grid = level.grid();
    let b = data.level.bounds;
    let mut open = Vec::new();
    for c in b.cells() {
        let border = c.x == b.x || c.x == b.x + b.w - 1 || c.y == b.z || c.y == b.z + b.d - 1;
        if border && grid.is_walkable(c, true) {
            open.push(c);
        }
    }
    assert!(open.is_empty(), "walkable border cells: {open:?}");
    // The flood fill only ever sees cells inside the bounds; with a solid border it cannot
    // leave them (cells outside do not exist in the grid).
    let reach = flood_fill(grid, data.spawn.cell(), false);
    assert!(reach.iter().filter(|r| **r).count() > 100);
}

// LAYOUT-003, LAYOUT-L1-003
#[test]
fn layout_003_l1_003_no_solid_overlaps_and_no_path_under_solid() {
    let data = common::level1();
    let overlaps = data.solid_overlaps();
    assert!(overlaps.is_empty(), "overlapping solids: {overlaps:?}");
    let grid = Level::new(data.clone()).grid().clone();
    let mut covered = Vec::new();
    for e in data.elements_of(ElementType::Path) {
        for c in e.rect.cells() {
            if let Some(s) = grid.solid_element(c) {
                covered.push((e.id.clone(), data.elements[s].id.clone(), c));
            }
        }
    }
    assert!(covered.is_empty(), "path cells under solids: {covered:?}");
}

// LAYOUT-004, LAYOUT-L1-010 (barrier part; mission part in rescue.rs)
#[test]
fn layout_004_opened_barrier_becomes_walkable_and_reachable() {
    let data = common::level1();
    let mut level = Level::new(data.clone());
    let rect = data.element("barrier_ne_tree").unwrap().rect;
    assert!(rect
        .cells()
        .all(|c| level.grid().kind(c) == CellKind::Solid));
    assert!(level.open_barrier("barrier_ne_tree"));
    assert!(rect.cells().all(|c| level.grid().is_walkable(c, false)));
    let reach = flood_fill(level.grid(), data.spawn.cell(), false);
    assert!(rect.cells().all(|c| reach[level.grid().index(c).unwrap()]));
    for other in ["barrier_north_gate", "barrier_east_repair"] {
        let r = data.element(other).unwrap().rect;
        assert!(
            r.cells().all(|c| level.grid().kind(c) == CellKind::Solid),
            "{other}"
        );
    }
    assert_eq!(level.exit_barriers(), vec!["barrier_ne_tree".to_owned()]);
}

// LAYOUT-006
#[test]
fn layout_006_surfaces() {
    let data = common::level1();
    let grid = Level::new(data.clone()).grid().clone();
    let path_cells: BTreeSet<(i32, i32)> = data
        .elements_of(ElementType::Path)
        .flat_map(|e| e.rect.cells().collect::<Vec<_>>())
        .map(|c| (c.x, c.y))
        .collect();
    let (mut n_path, mut n_grass) = (0, 0);
    for c in data.level.bounds.cells() {
        if let CellKind::Walkable(s) = grid.kind(c) {
            let expected = if path_cells.contains(&(c.x, c.y)) {
                n_path += 1;
                Surface::Path
            } else {
                n_grass += 1;
                Surface::Grass
            };
            assert_eq!(s, expected, "cell {c}");
        }
    }
    assert!(n_path > 0 && n_grass > 0);
    // spot checks against the map in GAME-LEVEL-1
    assert_eq!(
        grid.kind(IVec2::new(0, 2)),
        CellKind::Walkable(Surface::Path)
    ); // spawn
    assert_eq!(
        grid.kind(IVec2::new(-9, 12)),
        CellKind::Walkable(Surface::Grass)
    );
    assert_eq!(
        grid.kind(IVec2::new(11, 29)),
        CellKind::Walkable(Surface::Path)
    ); // bridge
    assert_eq!(
        grid.kind(IVec2::new(11, 30)),
        CellKind::Walkable(Surface::Path)
    );
    assert_eq!(grid.kind(IVec2::new(11, 31)), CellKind::Solid); // river
}

/// Parses the element table of GAME-LEVEL-1 (`| `id` | type (kind) | x, z, w, d | notes |`).
struct SpecRow {
    id: String,
    ty: String,
    kind: Option<String>,
    rect: [i32; 4],
    notes: String,
}

fn parse_ints(s: &str) -> Vec<i32> {
    s.split(',').filter_map(|p| p.trim().parse().ok()).collect()
}

/// `name (a, b, …)` inside free text.
fn paren_after(text: &str, name: &str) -> Option<Vec<i32>> {
    let i = text.find(name)? + name.len();
    let rest = text[i..].trim_start();
    let rest = rest.strip_prefix('(')?;
    let end = rest.find(')')?;
    Some(parse_ints(&rest[..end])).filter(|v| !v.is_empty())
}

fn spec_elements() -> Vec<SpecRow> {
    let md = common::read("specs/10-gameplay/levels/level-1.md");
    let section = md.split("## Elements").nth(1).expect("Elements section");
    let section = section.split("\n## ").next().unwrap();
    let mut rows = Vec::new();
    for line in section.lines() {
        let cols: Vec<&str> = line.split('|').map(str::trim).collect();
        if cols.len() < 5 || !cols[1].starts_with('`') {
            continue;
        }
        let id = cols[1].trim_matches('`').to_owned();
        let tycol = cols[2].replace("*(proposal)*", "");
        let mut parts = tycol.splitn(2, '(');
        let ty = parts.next().unwrap().trim().to_owned();
        let kind = parts
            .next()
            .map(|k| k.trim_end_matches(')').trim().to_owned());
        let r = parse_ints(cols[3]);
        assert_eq!(r.len(), 4, "rect of {id}");
        rows.push(SpecRow {
            id,
            ty,
            kind,
            rect: [r[0], r[1], r[2], r[3]],
            notes: cols[4].to_owned(),
        });
    }
    rows
}

// LAYOUT-005, LAYOUT-L1-004
#[test]
fn layout_005_l1_004_spec_table_matches_data() {
    let data = common::level1();
    let spec = spec_elements();
    let spec_ids: BTreeSet<_> = spec.iter().map(|r| r.id.clone()).collect();
    let data_ids: BTreeSet<_> = data.elements.iter().map(|e| e.id.clone()).collect();
    assert_eq!(spec_ids, data_ids, "element ids differ");
    let mut diffs = Vec::new();
    for row in &spec {
        let e = data.element(&row.id).unwrap();
        if e.ty.as_str() != row.ty {
            diffs.push(format!("{}: type {} vs {}", row.id, row.ty, e.ty.as_str()));
        }
        if let Some(k) = &row.kind {
            if e.kind.as_deref() != Some(k.as_str()) {
                diffs.push(format!("{}: kind {k} vs {:?}", row.id, e.kind));
            }
        }
        let r = e.rect;
        if [r.x, r.z, r.w, r.d] != row.rect {
            diffs.push(format!(
                "{}: rect {:?} vs {:?}",
                row.id,
                row.rect,
                [r.x, r.z, r.w, r.d]
            ));
        }
        if let Some(g) = paren_after(&row.notes, "gate") {
            let dg = e.gate.map(|r| vec![r.x, r.z, r.w, r.d]);
            if dg.as_ref() != Some(&g) {
                diffs.push(format!("{}: gate {g:?} vs {dg:?}", row.id));
            }
        }
        if let Some(s) = paren_after(&row.notes, "animal spot") {
            if e.animal_spot.map(|a| a.to_vec()) != Some(s.clone()) {
                diffs.push(format!(
                    "{}: animal spot {s:?} vs {:?}",
                    row.id, e.animal_spot
                ));
            }
        }
        if let Some(d) = paren_after(&row.notes, "door at cell") {
            if e.door.map(|a| a.to_vec()) != Some(d.clone()) {
                diffs.push(format!("{}: door {d:?} vs {:?}", row.id, e.door));
            }
        }
    }
    assert!(diffs.is_empty(), "spec vs data: {diffs:#?}");
}

fn walkable_adjacent(grid: &Grid, rect: zoo_core::Rect) -> Vec<IVec2> {
    zoo_core::Rect::new(rect.x - 1, rect.z - 1, rect.w + 2, rect.d + 2)
        .cells()
        // "next to" = sharing an edge (4-neighbourhood), as in the GAME-LEVEL-1 table
        .filter(|&c| {
            let side_x = (c.x == rect.x - 1 || c.x == rect.x + rect.w)
                && c.y >= rect.z
                && c.y < rect.z + rect.d;
            let side_z = (c.y == rect.z - 1 || c.y == rect.z + rect.d)
                && c.x >= rect.x
                && c.x < rect.x + rect.w;
            (side_x || side_z) && grid.is_walkable(c, false)
        })
        .collect()
}

fn near_spot(grid: &Grid, spot: IVec2) -> Vec<IVec2> {
    let b = grid.bounds();
    b.cells()
        .filter(|&c| {
            grid.is_walkable(c, false) && cell_center(c).distance(cell_center(spot)) <= 2.0
        })
        .collect()
}

// LAYOUT-L1-005
#[test]
fn layout_l1_005_walking_times_between_neighbours() {
    let data = common::level1();
    let level = Level::new(data.clone());
    let grid = level.grid();
    let rect = |id: &str| data.element(id).unwrap().rect;
    let spot = |id: &str| data.element(id).unwrap().animal_spot_cell().unwrap();
    let spawn = vec![data.spawn.cell()];
    let door_cell = data.element("food_storage").unwrap().door_cell().unwrap();
    let door = vec![door_cell - IVec2::Y]; // the cell in front of the south door
    let board = |id: &str| walkable_adjacent(grid, rect(id));
    let river = near_spot(grid, spot("loc_river"));
    let pond = near_spot(grid, spot("loc_pond"));
    let cave = near_spot(grid, spot("loc_cave"));
    // (name, from cells, to cells, table distance m, table time s)
    type Pair<'a> = (&'a str, Vec<IVec2>, Vec<IVec2>, f32, f32);
    let pairs: Vec<Pair> = vec![
        (
            "spawn -> food storage door",
            spawn.clone(),
            door.clone(),
            8.0,
            4.6,
        ),
        (
            "food storage -> zebra info board",
            door.clone(),
            board("board_zebra"),
            10.2,
            6.2,
        ),
        (
            "zebra info board -> pond",
            board("board_zebra"),
            pond.clone(),
            7.8,
            5.9,
        ),
        (
            "pond -> panda info board",
            pond.clone(),
            board("board_panda"),
            9.9,
            6.0,
        ),
        (
            "panda info board -> river",
            board("board_panda"),
            river.clone(),
            10.4,
            7.1,
        ),
        (
            "river -> hippo info board",
            river.clone(),
            board("board_hippo"),
            13.4,
            7.7,
        ),
        (
            "hippo info board -> cave",
            board("board_hippo"),
            cave.clone(),
            11.2,
            6.7,
        ),
        (
            "cave -> food storage door",
            cave.clone(),
            door.clone(),
            10.1,
            6.8,
        ),
        (
            "hippo info board -> food storage door",
            board("board_hippo"),
            door.clone(),
            12.2,
            7.3,
        ),
        (
            "spawn -> map board",
            spawn.clone(),
            board("map_board"),
            6.4,
            3.9,
        ),
    ];
    // GAME-PLAYER §6 (2026-09-26): path 1.75 m/s, grass 0.98 m/s
    let mp = zoo_core::player::MoveParams::default();
    let time = Cost::Time {
        path_speed: mp.speed_on(zoo_core::level::Surface::Path),
        grass_speed: mp.speed_on(zoo_core::level::Surface::Grass),
    };
    assert!((mp.walk_speed - 1.75).abs() < 1e-6);
    let mut too_slow = Vec::new();
    for (name, from, to, spec_m, spec_s) in &pairs {
        assert!(
            !from.is_empty() && !to.is_empty(),
            "{name}: empty point set"
        );
        let d = min_cost(grid, from, to, Cost::Distance);
        let t = min_cost(grid, from, to, time);
        println!("{name}: {d:.1} m (spec {spec_m}), {t:.1} s (spec {spec_s})");
        // the numbers in the GAME-LEVEL-1 table are reproduced by this cost model
        assert!(
            (d - spec_m).abs() <= 0.1 && (t - spec_s).abs() <= 0.1,
            "{name}: table drifted"
        );
        if t > 10.0 {
            too_slow.push((name.to_string(), t));
        }
    }
    assert!(
        too_slow.is_empty(),
        "neighbour pairs over 10 s: {too_slow:?}"
    );
}

// LAYOUT-L1-007
#[test]
fn layout_l1_007_hiding_place_features() {
    let data = common::level1();
    let expect = [
        ("loc_river", &["flowing_water", "bridge", "ducks"][..]),
        ("loc_pond", &["still_water", "water_lilies", "frogs"][..]),
        ("loc_cave", &["dark", "cool", "stone", "echo"][..]),
    ];
    for (id, feats) in expect {
        let e = data.element(id).unwrap();
        for f in feats {
            assert!(e.features.iter().any(|x| x == f), "{id} lacks {f}");
        }
    }
    let spot = cell_center(
        data.element("loc_river")
            .unwrap()
            .animal_spot_cell()
            .unwrap(),
    );
    let bridges: Vec<_> = data
        .elements_of(ElementType::Path)
        .filter(|e| e.kind.as_deref() == Some("bridge"))
        .collect();
    assert!(bridges.iter().any(|b| b.rect.distance_to(spot) <= 4.0));
    let pond = data.element("loc_pond").unwrap().rect;
    for e in &data.elements {
        let water_or_bridge =
            e.kind.as_deref() == Some("bridge") || e.kind.as_deref() == Some("river");
        if water_or_bridge {
            assert!(
                !e.rect.cells().any(|c| pond.contains(c)),
                "{} inside loc_pond",
                e.id
            );
        }
    }
}

// LAYOUT-L1-008
#[test]
fn layout_l1_008_only_river_and_pond_water_one_bridge() {
    let data = common::level1();
    let water_kinds = [
        "river", "pond", "water", "pool", "fountain", "lake", "trough",
    ];
    let water: Vec<_> = data
        .elements
        .iter()
        .filter(|e| e.kind.as_deref().is_some_and(|k| water_kinds.contains(&k)))
        .map(|e| e.id.as_str())
        .collect();
    assert!(water
        .iter()
        .all(|id| id.starts_with("river_") || *id == "pond_water"));
    let bridges: Vec<_> = data
        .elements
        .iter()
        .filter(|e| e.kind.as_deref() == Some("bridge"))
        .map(|e| e.id.as_str())
        .collect();
    assert_eq!(bridges, ["bridge_river"]);
}

// LAYOUT-L1-009
#[test]
fn layout_l1_009_animal_spots_within_range_of_walkable_cell() {
    let data = common::level1();
    let grid = Level::new(data.clone()).grid().clone();
    for h in data.elements_of(ElementType::HidingPlace) {
        let spot = h.animal_spot_cell().unwrap();
        assert!(
            !near_spot(&grid, spot).is_empty(),
            "{} unreachable spot",
            h.id
        );
    }
}

#[test]
fn grid_bounds_cover_all_solid_types() {
    // sanity: every solid element type is treated as solid
    let data = common::level1();
    let grid = Level::new(data.clone()).grid().clone();
    for ty in solid_types() {
        for e in data.elements_of(ty) {
            let c = IVec2::new(e.rect.x, e.rect.z);
            let gate = e.gate.is_some_and(|g| g.contains(c));
            if !gate && data.level.bounds.contains(c) {
                assert_eq!(grid.kind(c), CellKind::Solid, "{}", e.id);
            }
        }
    }
}

// LAYOUT-L1-013
#[test]
fn layout_l1_013_food_boxes_in_front_of_storage_reachable() {
    let data = common::level1();
    assert_eq!(data.food_boxes.len(), 10);
    let storage = data.element("food_storage").unwrap().rect;
    let level = Level::new(data.clone());
    let grid = level.grid();
    let spawn = data.spawn.cell();
    for b in &data.food_boxes {
        let pos = b.pos();
        // in front of the south facade
        assert!(
            pos.y < storage.z as f32 && pos.y > storage.z as f32 - 1.0,
            "{}",
            b.food
        );
        assert!(pos.x > storage.x as f32 && pos.x < (storage.x + storage.w) as f32);
        let front = zoo_core::level::cell_of(pos + b.facing() * 1.1);
        assert!(
            grid.is_passable(front, false),
            "{}: front cell blocked",
            b.food
        );
        assert!(
            zoo_core::nav::find_path(grid, spawn, front, false).is_some(),
            "{}: front cell not reachable",
            b.food
        );
    }
}
