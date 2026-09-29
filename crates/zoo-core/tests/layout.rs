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
            // sparse woods are walkable: every free cell of them (LAYOUT-L1-022)
            ElementType::Decoration if e.is_sparse() => true,
            // hedges and walls need not be reachable (GAME-LEVEL-1 §3)
            _ => false,
        };
        if must && !reachable_near(grid, &reach, e.rect) {
            missing.push(e.id.clone());
        }
    }
    for h in &data.hiding_places {
        if !reachable_near(grid, &reach, h.rect) {
            missing.push(h.id.clone());
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
            // the street runs on under the level-transition barriers (LAYOUT-040)
            if let Some(s) = grid
                .solid_element(c)
                .filter(|&s| data.elements[s].ty != ElementType::Barrier)
            {
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
    // paths, and the interior and door cells of enterable buildings (LAYOUT-023, Q-092)
    let path_cells: BTreeSet<(i32, i32)> = data
        .elements_of(ElementType::Path)
        .flat_map(|e| e.rect.cells().collect::<Vec<_>>())
        .chain(
            data.elements
                .iter()
                .filter(|e| e.is_enterable())
                .flat_map(|e| {
                    e.rect
                        .cells()
                        .filter(|c| e.is_open_cell(*c))
                        .collect::<Vec<_>>()
                }),
        )
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
    let spot = |id: &str| data.hiding_place(id).unwrap().spot_cell();
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
            4.1,
        ),
        (
            "food storage -> zebra info board",
            door.clone(),
            board("board_zebra"),
            8.0,
            4.1,
        ),
        // FIX-056: loc_pond moved to the north shore (22 m haze rule), so the zebra board's
        // next neighbour is the panda board; the pond is reached from the panda board.
        // Q-171 (2026-09-28): `board_zebra` moved south of the gate — the ring stop between
        // the storage and the panda board is the zebra enclosure's gate (proposal Q-176).
        (
            "zebra gate -> panda info board",
            walkable_adjacent(grid, data.element("enc_zebra").unwrap().gate.unwrap()),
            board("board_panda"),
            18.1,
            9.7,
        ),
        (
            "pond -> panda info board",
            pond.clone(),
            board("board_panda"),
            11.8,
            7.2,
        ),
        (
            "panda info board -> river",
            board("board_panda"),
            river.clone(),
            10.8,
            7.2,
        ),
        (
            "river -> hippo info board",
            river.clone(),
            board("board_hippo"),
            10.4,
            5.6,
        ),
        (
            "hippo info board -> cave",
            board("board_hippo"),
            cave.clone(),
            15.2,
            8.2,
        ),
        (
            "cave -> food storage door",
            cave.clone(),
            door.clone(),
            10.1,
            6.1,
        ),
        (
            "hippo info board -> food storage door",
            board("board_hippo"),
            door.clone(),
            16.2,
            8.7,
        ),
        (
            "spawn -> map board",
            spawn.clone(),
            board("map_board"),
            6.4,
            3.6,
        ),
    ];
    // GAME-PLAYER §6 (2026-09-26): path 1.93 m/s, grass 0.98 m/s
    let mp = zoo_core::player::MoveParams::default();
    let time = Cost::Time {
        path_speed: mp.speed_on(zoo_core::level::Surface::Path),
        grass_speed: mp.speed_on(zoo_core::level::Surface::Grass),
    };
    assert!((mp.walk_speed - 1.93).abs() < 1e-6);
    let mut too_slow = Vec::new();
    let mut drifted = Vec::new();
    for (name, from, to, spec_m, spec_s) in &pairs {
        assert!(
            !from.is_empty() && !to.is_empty(),
            "{name}: empty point set"
        );
        let d = min_cost(grid, from, to, Cost::Distance);
        let t = min_cost(grid, from, to, time);
        println!("{name}: {d:.1} m (spec {spec_m}), {t:.1} s (spec {spec_s})");
        // the numbers in the GAME-LEVEL-1 table are reproduced by this cost model
        if (d - spec_m).abs() > 0.1 || (t - spec_s).abs() > 0.1 {
            drifted.push(format!("{name}: {d:.1} m, {t:.1} s"));
        }
        if t > 10.0 {
            too_slow.push((name.to_string(), t));
        }
    }
    // not neighbours (for information in the spec)
    for (name, from, to) in [
        (
            "panda info board -> food storage",
            board("board_panda"),
            door.clone(),
        ),
        (
            "zebra board -> panda board",
            board("board_zebra"),
            board("board_panda"),
        ),
        (
            "hippo board -> zebra board",
            board("board_hippo"),
            board("board_zebra"),
        ),
        ("zebra board -> pond", board("board_zebra"), pond.clone()),
        (
            "zebra board -> zebra gate",
            board("board_zebra"),
            walkable_adjacent(grid, data.element("enc_zebra").unwrap().gate.unwrap()),
        ),
    ] {
        println!("{name}: {:.1} s", min_cost(grid, &from, &to, time));
    }
    assert!(drifted.is_empty(), "table drifted: {drifted:#?}");
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
        (
            "loc_meadow",
            &["tall_grass", "wildflowers", "butterflies"][..],
        ),
        ("loc_sand", &["sand", "dry", "yellow_ground"][..]),
        ("loc_mud", &["mud", "brown_ground", "wet"][..]),
        ("loc_shade", &["shade", "big_trees", "zoo_wall"][..]),
        (
            "loc_bamboo",
            &["bamboo_thicket", "green_stalks", "taller_than_wall"][..],
        ),
        (
            "loc_leaves",
            &["leaf_pile", "red_yellow_leaves", "rake"][..],
        ),
    ];
    for (id, feats) in expect {
        let e = data.hiding_place(id).unwrap();
        for f in feats {
            assert!(e.features.iter().any(|x| x == f), "{id} lacks {f}");
        }
        // every scenery id the place names exists
        for sc in &e.scenery {
            assert!(
                data.scenery.iter().any(|x| &x.id == sc) || data.element(sc).is_some(),
                "{id}: scenery {sc} missing"
            );
        }
    }
    let spot = data.hiding_place("loc_river").unwrap().spot();
    let bridges: Vec<_> = data
        .elements_of(ElementType::Path)
        .filter(|e| e.kind.as_deref() == Some("bridge"))
        .collect();
    assert!(bridges.iter().any(|b| b.rect.distance_to(spot) <= 4.0));
    let pond = data.hiding_place("loc_pond").unwrap().rect;
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
    for h in &data.hiding_places {
        let spot = h.spot_cell();
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
        for e in data.elements_of(ty).filter(|e| e.is_solid()) {
            let c = IVec2::new(e.rect.x, e.rect.z);
            let gate = e.gate.is_some_and(|g| g.contains(c));
            if !gate && data.level.bounds.contains(c) {
                assert_eq!(grid.kind(c), CellKind::Solid, "{}", e.id);
            }
        }
    }
}

// LAYOUT-L1-013 (Q-194 answered 2026-09-29: more real food boxes stand inside the storage
// too, may repeat a food).
#[test]
fn layout_l1_013_food_boxes_in_front_of_storage_reachable() {
    let data = common::level1();
    let storage = data.element("food_storage").unwrap().rect;
    let outside: Vec<_> = data
        .food_boxes
        .iter()
        .filter(|b| !storage.contains(zoo_core::level::cell_of(b.pos())))
        .collect();
    let inside: Vec<_> = data
        .food_boxes
        .iter()
        .filter(|b| storage.contains(zoo_core::level::cell_of(b.pos())))
        .collect();
    assert_eq!(outside.len(), 10);
    assert!((2..=6).contains(&inside.len()), "{} inside", inside.len());
    let level = Level::new(data.clone());
    let grid = level.grid();
    let spawn = data.spawn.cell();
    for b in outside {
        let pos = b.pos();
        // in front of the south facade
        assert!(
            pos.y < storage.z as f32 && pos.y > storage.z as f32 - 1.0,
            "{}",
            b.food
        );
        // along the facade; since Q-150 (doors are never blocked) the two boxes moved out of
        // the door gap stand at the row's ends, up to 1 m beyond the facade corners
        assert!(
            pos.x > storage.x as f32 - 1.0 && pos.x < (storage.x + storage.w) as f32 + 1.0,
            "{}",
            b.food
        );
    }
    // every box (outside or inside) has a reachable, passable standing cell in front
    for b in &data.food_boxes {
        let front = zoo_core::level::cell_of(b.pos() + b.facing() * 1.1);
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

/// Walking-time cost model of GAME-PLAYER §6 (path 1.93 m/s, grass 0.98 m/s).
fn time_cost() -> Cost {
    let mp = zoo_core::player::MoveParams::default();
    Cost::Time {
        path_speed: mp.speed_on(Surface::Path),
        grass_speed: mp.speed_on(Surface::Grass),
    }
}

// LAYOUT-L1-018 (and the "Own board → spot" column of GAME-LEVEL-1 "Hiding places"): every
// new candidate is ≤ 10 s from its listed neighbour.
#[test]
fn layout_l1_018_new_hiding_places_near_a_neighbour() {
    let data = common::level1();
    let level = Level::new(data.clone());
    let grid = level.grid();
    let near = |id: &str| near_spot(grid, data.hiding_place(id).unwrap().spot_cell());
    let adj = |id: &str| walkable_adjacent(grid, data.element(id).unwrap().rect);
    let pairs: Vec<(&str, Vec<IVec2>, Vec<IVec2>)> = vec![
        (
            "loc_meadow -> bridge",
            near("loc_meadow"),
            adj("bridge_river"),
        ),
        (
            "loc_sand -> barrier_north_gate",
            near("loc_sand"),
            adj("barrier_north_gate"),
        ),
        (
            "loc_mud -> barrier_north_gate",
            near("loc_mud"),
            adj("barrier_north_gate"),
        ),
        ("loc_shade -> loc_mud", near("loc_shade"), near("loc_mud")),
        (
            "loc_bamboo -> map_board",
            near("loc_bamboo"),
            adj("map_board"),
        ),
        (
            "loc_leaves -> loc_meadow",
            near("loc_leaves"),
            near("loc_meadow"),
        ),
    ];
    let mut slow = Vec::new();
    for (name, from, to) in pairs {
        assert!(!from.is_empty() && !to.is_empty(), "{name}");
        let t = min_cost(grid, &from, &to, time_cost());
        println!("{name}: {t:.1} s");
        if t > 10.0 {
            slow.push((name, t));
        }
    }
    // own info board → spot (information for the spec table)
    for h in &data.hiding_places {
        let board = data
            .elements
            .iter()
            .find(|e| {
                e.kind.as_deref() == Some("info_board")
                    && e.enclosure.as_deref() == Some(&format!("enc_{}", h.animal))
            })
            .unwrap();
        let t = min_cost(grid, &adj(&board.id), &near(&h.id), time_cost());
        let d = cell_center(h.spot_cell())
            .distance(cell_center(IVec2::new(board.rect.x, board.rect.z)));
        println!("board -> {}: {d:.1} m straight, {t:.1} s", h.id);
    }
    assert!(slow.is_empty(), "over 10 s: {slow:?}");
}

// LAYOUT-L1-014, LAYOUT-L1-024, LAYOUT-014: wander areas of the 9 candidates (rect clip,
// Q-085) — spot inside, ≥ 9 cells, a cell ≥ 2 m from the spot, no solid cell, no path cell
// except the cave floor, inside the rect, disjoint between animals, the cell counts of the
// GAME-LEVEL-1 table and no tree/bush collider inside.
#[test]
fn layout_l1_014_024_wander_areas() {
    use zoo_core::wander::hiding_area;
    let data = common::level1();
    let level = Level::new(data.clone());
    let grid = level.grid();
    let expected = [
        ("loc_river", 12),
        ("loc_meadow", 16),
        ("loc_sand", 22),
        ("loc_pond", 14),
        ("loc_mud", 22),
        ("loc_shade", 12),
        ("loc_cave", 9),
        ("loc_bamboo", 21),
        ("loc_leaves", 9),
    ];
    let mut areas = Vec::new();
    for (id, n) in expected {
        let h = data.hiding_place(id).unwrap();
        let area = hiding_area(&level, h);
        let cells: Vec<IVec2> = area.cells().map(|(c, _)| c).collect();
        assert_eq!(cells.len(), n, "{id}: {} cells", cells.len());
        assert!(area.contains(h.spot_cell()), "{id}: spot");
        assert!(cells.len() >= 9);
        assert!(
            cells
                .iter()
                .any(|&c| cell_center(c).distance(h.spot()) >= 2.0),
            "{id}: no cell 2 m from the spot"
        );
        let cave = |c: IVec2| {
            data.elements.iter().any(|e| {
                e.ty == ElementType::Path && e.kind.as_deref() == Some("cave") && e.rect.contains(c)
            })
        };
        for &c in &cells {
            assert!(h.rect.contains(c), "{id}: {c} outside its rect");
            if h.wander_on != "water" {
                assert!(grid.is_walkable(c, false), "{id}: solid cell {c}");
            }
            assert!(!grid.has_path(c) || cave(c), "{id}: path cell {c}");
            // no tree or bush collider in the cell
            let tree = level
                .colliders()
                .shapes()
                .iter()
                .any(|s| s.overlaps(cell_center(c), 0.3));
            assert!(!tree, "{id}: a collider in {c}");
        }
        areas.push((h.animal.clone(), h.id.clone(), h.rect, cells));
    }
    for (i, a) in areas.iter().enumerate() {
        for b in &areas[i + 1..] {
            if a.0 == b.0 {
                continue;
            }
            assert!(
                !a.3.iter().any(|c| b.3.contains(c)),
                "{} and {} share wander cells",
                a.1,
                b.1
            );
            assert!(
                !a.2.cells().any(|c| b.2.contains(c)),
                "rects of {} and {} overlap",
                a.1,
                b.1
            );
        }
    }
}

// LAYOUT-L1-015, LAYOUT-014: ≥ 3 candidates per animal; all 27 combinations keep the spots
// pairwise ≥ 12 m apart.
#[test]
fn layout_l1_015_every_combination_spread() {
    let data = common::level1();
    let per: Vec<Vec<_>> = ["zebra", "hippo", "panda"]
        .iter()
        .map(|a| data.hiding_places_of(a).collect::<Vec<_>>())
        .collect();
    for p in &per {
        assert!(p.len() >= 3);
    }
    let mut n = 0;
    for a in &per[0] {
        for b in &per[1] {
            for c in &per[2] {
                for (x, y) in [(a, b), (a, c), (b, c)] {
                    let d = x.spot().distance(y.spot());
                    assert!(d >= 12.0 - 1e-4, "{} – {}: {d:.1}", x.id, y.id);
                }
                n += 1;
            }
        }
    }
    assert_eq!(n, 27);
}

// LAYOUT-L1-016: scenery is not solid, overlaps no solid element and no path, lies inside
// its hiding place's rect, and each kind occurs once.
#[test]
fn layout_l1_016_scenery() {
    let data = common::level1();
    let level = Level::new(data.clone());
    let mut kinds = BTreeSet::new();
    for sc in &data.scenery {
        assert!(kinds.insert(sc.kind.clone()), "{} twice", sc.kind);
        for c in sc.rect.cells() {
            assert!(level.grid().is_walkable(c, false), "{}: solid {c}", sc.id);
            assert!(!level.grid().has_path(c), "{}: path {c}", sc.id);
        }
        let h = data
            .hiding_place(sc.hiding_place.as_deref().unwrap())
            .unwrap();
        assert!(
            sc.rect.cells().all(|c| h.rect.contains(c)),
            "{} outside {}",
            sc.id,
            h.id
        );
    }
    assert_eq!(kinds.len(), 5);
}

// LAYOUT-L1-017: the legacy `[[element]] type = "hiding_place"` mirrors are gone (Q-080
// answered: deleted when zoo-core migrated to `[[hiding_place]]`).
#[test]
fn layout_l1_017_no_legacy_hiding_place_elements() {
    let data = common::level1();
    assert_eq!(data.elements_of(ElementType::HidingPlace).count(), 0);
    assert_eq!(data.hiding_places.len(), 9);
}

// LAYOUT-L1-020, LAYOUT-020: the hippo pool (35–60 % of the inner cells, inside the fence,
// not next to the gate, ramp inside), `home_wander_on` with water, and the home wander area
// (≥ 9 cells per surface, no gate cell, water not next to the gate, water and grass joined
// only over the ramp, connected from the grass cell inside the gate).
#[test]
fn layout_l1_020_layout_020_hippo_pool_and_home_area() {
    use zoo_core::wander::{home_area, home_entry, AreaCell};
    let data = common::level1();
    let level = Level::new(data.clone());
    let (ei, enc) = data
        .elements
        .iter()
        .enumerate()
        .find(|(_, e)| e.id == "enc_hippo")
        .unwrap();
    let gate = enc.gate.unwrap();
    let pool = data
        .features_of("enc_hippo")
        .find(|f| f.is_pool())
        .expect("hippo_pool");
    let inner = enc.rect.cells().filter(|c| !gate.contains(*c)).count();
    let pool_cells = pool.rect.cells().count();
    let share = pool_cells as f32 / inner as f32;
    assert_eq!((pool_cells, inner), (56, 130));
    assert!((0.35..=0.60).contains(&share), "{share}");
    assert!(pool.rect.cells().all(|c| enc.rect.contains(c)));
    assert!(pool
        .rect
        .cells()
        .all(|c| !gate.cells().any(|g| (g - c).abs().max_element() <= 1)));
    let ramp = pool.ramp.unwrap();
    assert!(ramp.cells().all(|c| pool.rect.contains(c)));
    assert!(enc.home_surfaces().contains(&"water"));
    // home area of every enclosure (LAYOUT-020)
    for (i, e) in data.elements.iter().enumerate() {
        if e.ty != ElementType::Enclosure {
            continue;
        }
        let area = home_area(&level, i);
        let g = e.gate.unwrap();
        let land = area.cells().filter(|(_, k)| *k == AreaCell::Land).count();
        let water = area.cells().filter(|(_, k)| *k != AreaCell::Land).count();
        assert!(land >= 9, "{}: {land} grass cells", e.id);
        if e.home_surfaces().contains(&"water") {
            assert!(water >= 9, "{}: {water} water cells", e.id);
        }
        for (c, k) in area.cells() {
            assert!(!g.contains(c), "{}: gate cell {c}", e.id);
            if k != AreaCell::Land {
                assert!(
                    !g.cells().any(|x| (x - c).abs().max_element() <= 1),
                    "{}: water next to the gate",
                    e.id
                );
            }
        }
        let entry = home_entry(&level, i).unwrap();
        assert!(
            area.contains(entry),
            "{}: entry {entry} not in the area",
            e.id
        );
        // connected from the entry cell; the way from grass into the water always leads
        // over a ramp cell (the rim is 0.4 m high)
        for (c, k) in area.cells() {
            let route = area
                .route(entry, c)
                .unwrap_or_else(|| panic!("{}: {c} not connected", e.id));
            if k == AreaCell::Water {
                assert!(
                    route.iter().any(|r| area.class(*r) == Some(AreaCell::Ramp)),
                    "{}: {c} reached without the ramp",
                    e.id
                );
            }
        }
    }
    let area = home_area(&level, ei);
    let land = area.cells().filter(|(_, k)| *k == AreaCell::Land).count();
    let water = area.cells().filter(|(_, k)| *k != AreaCell::Land).count();
    assert_eq!(
        (land, water),
        (55, 56),
        "GAME-LEVEL-1: 55 grass + 56 pool cells"
    );
    assert!(pool.rect.cells().all(|c| area.contains(c)));
}

// LAYOUT-L1-022, LAYOUT-015: sparse woods — colliders inside the rect and outside paths,
// scenery and hiding-place rects; clear gap ≥ 1.8 m between obstacles (touching colliders
// form one obstacle); the player can cross west–east and south–north; every free cell
// centre is reachable from the spawn.
#[test]
fn layout_l1_022_layout_015_sparse_woods() {
    let data = common::level1();
    let level = Level::new(data.clone());
    let grid = level.grid();
    let reach = flood_fill(grid, data.spawn.cell(), false);
    for id in ["trees_nw", "trees_ne"] {
        let e = data.element(id).unwrap();
        assert!(e.is_sparse() && !e.is_solid());
        let r = e.rect;
        let (lo, hi) = (
            glam::Vec2::new(r.x as f32, r.z as f32),
            glam::Vec2::new((r.x + r.w) as f32, (r.z + r.d) as f32),
        );
        // spec radii (GAME-LEVEL-1 "Woods"): tree_round 0.45, bush 0.70
        let obs: Vec<(glam::Vec2, f32)> = e
            .trees
            .iter()
            .map(|t| (t.pos(), if t.model == "bush" { 0.70 } else { 0.45 }))
            .collect();
        for (p, rad) in &obs {
            assert!(
                p.x - rad >= lo.x && p.x + rad <= hi.x && p.y - rad >= lo.y && p.y + rad <= hi.y,
                "{id}: collider at {p} leaves the rect"
            );
            let cell = zoo_core::level::cell_of(*p);
            assert!(!grid.has_path(cell), "{id}: tree on a path");
            for h in &data.hiding_places {
                assert!(!h.rect.contains(cell), "{id}: tree in {}", h.id);
            }
            for sc in &data.scenery {
                assert!(!sc.rect.contains(cell), "{id}: tree in {}", sc.id);
            }
        }
        // clusters of touching colliders
        let n = obs.len();
        let mut group: Vec<usize> = (0..n).collect();
        fn root(g: &mut Vec<usize>, i: usize) -> usize {
            if g[i] != i {
                let r = root(g, g[i]);
                g[i] = r;
            }
            g[i]
        }
        for i in 0..n {
            for j in i + 1..n {
                let gap = obs[i].0.distance(obs[j].0) - obs[i].1 - obs[j].1;
                if gap <= 0.05 {
                    let (a, b) = (root(&mut group, i), root(&mut group, j));
                    group[a] = b;
                }
            }
        }
        for i in 0..n {
            for j in i + 1..n {
                if root(&mut group, i) == root(&mut group, j) {
                    continue;
                }
                let gap = obs[i].0.distance(obs[j].0) - obs[i].1 - obs[j].1;
                assert!(gap >= 1.8 - 1e-3, "{id}: gap {gap:.2} m");
            }
        }
        // free cell centres reachable from the spawn
        for c in r.cells() {
            if grid.is_passable(c, false) {
                assert!(reach[grid.index(c).unwrap()], "{id}: {c} not reachable");
            }
        }
        // crossing: a scripted player walks along each middle row / column
        let params = zoo_core::player::MoveParams::default();
        let mid = glam::Vec2::new((lo.x + hi.x) / 2.0, (lo.y + hi.y) / 2.0);
        for (from, to) in [
            (
                glam::Vec2::new(lo.x - 0.5, mid.y),
                glam::Vec2::new(hi.x + 0.5, mid.y),
            ),
            (
                glam::Vec2::new(mid.x, lo.y - 0.5),
                glam::Vec2::new(mid.x, hi.y + 0.5),
            ),
        ] {
            let from = zoo_core::level::cell_center(zoo_core::level::cell_of(from));
            let to = zoo_core::level::cell_center(zoo_core::level::cell_of(to));
            let path = zoo_core::nav::find_path(
                grid,
                zoo_core::level::cell_of(from),
                zoo_core::level::cell_of(to),
                false,
            );
            assert!(path.is_some(), "{id}: no way across {from} → {to}");
            let mut p = zoo_core::Player::new(from, glam::Vec2::X, &params);
            let mut ap = zoo_core::nav::Autopilot::new(to);
            let mut t = 0.0;
            while let Some(dir) = ap.input(grid, p.pos, false) {
                p.step_with(grid, level.colliders(), &params, dir, 1.0 / 60.0, false);
                t += 1.0 / 60.0;
                assert!(t < 60.0, "{id}: stuck crossing at {}", p.pos);
            }
        }
    }
}

// LAYOUT-L1-023, LAYOUT-016: the dense grove is never entered and has a bush border on every
// walkable side (LAYOUT-019).
#[test]
fn layout_l1_023_layout_016_dense_grove() {
    let data = common::level1();
    let level = Level::new(data.clone());
    let grid = level.grid();
    let e = data.element("grove_center").unwrap();
    assert_eq!(e.density.as_deref(), Some("dense"));
    let reach = flood_fill(grid, data.spawn.cell(), true);
    for c in e.rect.cells() {
        assert!(!reach[grid.index(c).unwrap()], "grove cell {c} reachable");
    }
    let scene = zoo_core::scene::LevelScene::build(&data);
    let bushes: Vec<glam::Vec2> = scene
        .placements
        .iter()
        .filter(|p| p.model == "bush")
        .map(|p| zoo_core::coords::world_to_level(p.pos))
        .collect();
    let r = e.rect;
    // walkable sides: west (x = -6), east (x = 5), north (z = 27); every metre of them has a
    // bush within 1.0 m
    for z in r.z..r.z + r.d {
        for (x_edge, x_out) in [(r.x as f32, r.x - 1), ((r.x + r.w) as f32, r.x + r.w)] {
            if !grid.is_walkable(IVec2::new(x_out, z), false) {
                continue;
            }
            let p = glam::Vec2::new(x_edge, z as f32 + 0.5);
            assert!(
                bushes.iter().any(|b| b.distance(p) <= 1.0),
                "no border bush near {p}"
            );
        }
    }
    for x in r.x..r.x + r.w {
        let z_out = r.z + r.d;
        if grid.is_walkable(IVec2::new(x, z_out), false) {
            let p = glam::Vec2::new(x as f32 + 0.5, (r.z + r.d) as f32);
            assert!(
                bushes.iter().any(|b| b.distance(p) <= 1.0),
                "no border bush near {p}"
            );
        }
    }
}

/// Share of level points (y = 0) inside the view of the GAME-PLAYER §2 camera (pitch 55°,
/// 35° vertical FOV, look-at 0.7 m above the player's feet) looking along `forward`.
fn on_screen(
    player: glam::Vec2,
    forward: glam::Vec2,
    dist: f32,
    aspect: f32,
    pts: &[glam::Vec2],
) -> Vec<bool> {
    use glam::camera::rh::{proj::opengl, view::look_at_mat4};
    use zoo_core::coords::{level_to_world, WORLD_UP};
    let target = level_to_world(player) + WORLD_UP * 0.7;
    let fwd = level_to_world(forward).normalize();
    let pitch = 55f32.to_radians();
    let eye = target - fwd * dist * pitch.cos() + WORLD_UP * dist * pitch.sin();
    let vp = opengl::perspective(35f32.to_radians(), aspect, 0.1, 200.0)
        * look_at_mat4(eye, target, WORLD_UP);
    pts.iter()
        .map(|p| {
            let c = vp * level_to_world(*p).extend(1.0);
            c.w > 0.0 && (c.x / c.w).abs() <= 1.0 && (c.y / c.w).abs() <= 1.0
        })
        .collect()
}

// LAYOUT-L1-021: the pool is visible from the path in front of the hippo gate.
#[test]
fn layout_l1_021_pool_visible_from_the_gate() {
    let data = common::level1();
    let pool = data.features_of("enc_hippo").find(|f| f.is_pool()).unwrap();
    let cells: Vec<IVec2> = pool.rect.cells().collect();
    let pts: Vec<glam::Vec2> = cells.iter().map(|&c| cell_center(c)).collect();
    let ramp = pool.ramp.unwrap();
    for stand in [IVec2::new(8, 15), IVec2::new(8, 16)] {
        let p = cell_center(stand);
        let seen = on_screen(p, glam::Vec2::X, 14.0, 1080.0 / 2340.0, &pts);
        let n = seen.iter().filter(|s| **s).count();
        let ramps = cells
            .iter()
            .zip(&seen)
            .filter(|(c, s)| **s && ramp.contains(**c))
            .count();
        assert!(
            n as f32 >= 0.25 * pts.len() as f32,
            "portrait east from {stand}: {n}/{}",
            pts.len()
        );
        assert!(ramps >= 2, "portrait east from {stand}: {ramps} ramp cells");
        let seen = on_screen(p, glam::Vec2::Y, 14.0, 2340.0 / 1080.0, &pts);
        let n = seen.iter().filter(|s| **s).count();
        assert!(
            n as f32 >= 0.9 * pts.len() as f32,
            "landscape north from {stand}: {n}/{}",
            pts.len()
        );
    }
}

// LAYOUT-026 (Q-066): every river / stream element has a flow, the pieces of each river
// (incl. the bridges over it) chain along the flow with consistent 90° bends, and broken
// data is rejected.
#[test]
fn layout_026_rivers_have_a_flow_and_consistent_bends() {
    let zoo = common::zoo();
    for e in &zoo.elements {
        let flowing =
            e.ty == ElementType::Landmark && matches!(e.kind.as_deref(), Some("river" | "stream"));
        assert_eq!(flowing, e.flow.is_some(), "{}: flow key", e.id);
    }
    let paths = zoo_core::water::river_paths(&zoo).expect("rivers chain");
    let ids: Vec<Vec<String>> = paths.iter().map(|p| p.ids.clone()).collect();
    assert_eq!(
        ids,
        vec![
            vec!["river_n", "bridge_river", "river_mid", "river_e"],
            vec!["stream_l3"]
        ]
    );
    // level 1: straight south, one left bend of radius 1.5 m, straight east
    let arcs = paths[0]
        .pieces
        .iter()
        .filter(|p| matches!(p, zoo_core::water::PathPiece::Arc { .. }))
        .count();
    assert_eq!(arcs, 1);
    assert_eq!(paths[0].half_width, 1.5);

    let text = common::read("assets/levels/level-1.toml");
    let missing = text.replacen("flow = \"E\"\n", "", 1);
    assert!(
        zoo_core::LevelData::from_toml_str(&missing).is_err(),
        "missing flow"
    );
    let bad_value = text.replacen("flow = \"E\"", "flow = \"east\"", 1);
    assert!(zoo_core::LevelData::from_toml_str(&bad_value).is_err());
    for (flow, why) in [("N", "reversed"), ("W", "bend at the wrong end")] {
        let t = text.replacen("flow = \"E\"", &format!("flow = \"{flow}\""), 1);
        let d = zoo_core::LevelData::from_toml_str(&t).unwrap();
        assert!(zoo_core::water::river_paths(&d).is_err(), "{why}");
    }
}
