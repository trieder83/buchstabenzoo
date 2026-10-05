//! GAME-LAYOUT "Joining levels" (LAYOUT-021…024) and the GAME-LEVEL-2 / GAME-LEVEL-3 unit
//! tests (LAYOUT-L2-*, LAYOUT-L3-*) on `assets/levels/level-{1,2,3}.toml`.

mod common;

use std::collections::{BTreeMap, BTreeSet};

use glam::{IVec2, Vec2};
use zoo_core::level::{
    cell_center, cell_of, CellKind, ElementType, Grid, Level, LevelData, Surface,
};
use zoo_core::nav::{flood_fill, min_cost, Cost};
use zoo_core::wander::{hiding_area, home_area};

/// The joined zoo with the given barriers open.
fn zoo_level(open: &[&str]) -> Level {
    let mut level = Level::new(common::zoo());
    for b in open {
        assert!(level.open_barrier(b), "{b} is a barrier");
    }
    level
}

const L2_OPEN: [&str; 1] = ["barrier_ne_tree"];
const L3_OPEN: [&str; 3] = [
    "barrier_ne_tree",
    "barrier_l2_construction",
    "barrier_north_gate",
];

fn part_bounds(data: &LevelData, id: &str) -> zoo_core::Rect {
    data.parts[data.part_index(id).unwrap()].bounds
}

fn reach_of(grid: &Grid, start: IVec2) -> Vec<bool> {
    flood_fill(grid, start, false)
}

/// Walkable cells inside or 8-adjacent to a rectangle that the flood fill reached.
fn reachable_near(grid: &Grid, reach: &[bool], rect: zoo_core::Rect) -> bool {
    zoo_core::Rect::new(rect.x - 1, rect.z - 1, rect.w + 2, rect.d + 2)
        .cells()
        .any(|c| grid.index(c).is_some_and(|k| reach[k]) && grid.is_walkable(c, false))
}

fn walkable_adjacent(grid: &Grid, rect: zoo_core::Rect) -> Vec<IVec2> {
    zoo_core::Rect::new(rect.x - 1, rect.z - 1, rect.w + 2, rect.d + 2)
        .cells()
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
    let r = zoo_core::Rect::new(spot.x - 3, spot.y - 3, 7, 7);
    r.cells()
        .filter(|&c| {
            grid.is_walkable(c, false) && cell_center(c).distance(cell_center(spot)) <= 2.0
        })
        .collect()
}

fn time_cost() -> Cost {
    let mp = zoo_core::player::MoveParams::default();
    Cost::Time {
        path_speed: mp.speed_on(Surface::Path),
        grass_speed: mp.speed_on(Surface::Grass),
    }
}

// ------------------------------------------------------------------ LAYOUT-021…024

// LAYOUT-021, LAYOUT-L2-002, LAYOUT-L3-002: disjoint bounds; entry cells are path cells on
// the border, edge-adjacent to their barrier in the earlier level; every other border cell
// is solid.
#[test]
fn layout_021_l2_002_l3_002_entries_and_sealed_borders() {
    let levels = [common::level1(), common::level2(), common::level3()];
    for (i, a) in levels.iter().enumerate() {
        for b in &levels[i + 1..] {
            let (ra, rb) = (a.level.bounds, b.level.bounds);
            assert!(
                !ra.cells().any(|c| rb.contains(c)),
                "{} and {} overlap",
                a.level.id,
                b.level.id
            );
        }
    }
    let zoo = common::zoo();
    let level = Level::new(zoo.clone());
    let grid = level.grid();
    for data in &levels[1..] {
        let b = data.level.bounds;
        let entry_cells: BTreeSet<(i32, i32)> = data
            .entries
            .iter()
            .flat_map(|e| e.cells.cells())
            .map(|c| (c.x, c.y))
            .collect();
        assert!(!entry_cells.is_empty(), "{}: no [[entry]]", data.level.id);
        for e in &data.entries {
            let from = levels.iter().find(|l| l.level.id == e.from_level).unwrap();
            let barrier = from.element(&e.barrier).unwrap_or_else(|| {
                panic!("{}: barrier {} not in {}", e.id, e.barrier, e.from_level)
            });
            assert_eq!(barrier.ty, ElementType::Barrier);
            for c in e.cells.cells() {
                let border =
                    c.x == b.x || c.x == b.x + b.w - 1 || c.y == b.z || c.y == b.z + b.d - 1;
                assert!(border, "{}: {c} not on the border", e.id);
                assert!(grid.has_path(c), "{}: {c} not a path cell", e.id);
                let touches = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y]
                    .iter()
                    .any(|d| barrier.rect.contains(c + *d));
                assert!(touches, "{}: {c} not next to {}", e.id, e.barrier);
            }
        }
        let mut open = Vec::new();
        for c in b.cells() {
            let border = c.x == b.x || c.x == b.x + b.w - 1 || c.y == b.z || c.y == b.z + b.d - 1;
            if border && !entry_cells.contains(&(c.x, c.y)) && grid.is_walkable(c, true) {
                open.push(c);
            }
        }
        assert!(
            open.is_empty(),
            "{}: walkable border cells {open:?}",
            data.level.id
        );
    }
    // cells between the levels do not exist for movement
    let between = IVec2::new(40, 0); // east of level 1, south of level 2
    assert_eq!(grid.kind(between), CellKind::OutOfBounds);
}

// LAYOUT-022, LAYOUT-L2-001 (sealed part), LAYOUT-L3-002 (flood part): exactly the
// barriers of completed transitions open → only unlocked levels are reachable.
#[test]
fn layout_022_joined_levels_unlock_in_order() {
    let zoo = common::zoo();
    let b2 = part_bounds(&zoo, "level_2");
    let b3 = part_bounds(&zoo, "level_3");
    let spawn1 = zoo.parts[0].spawn.cell();
    let spawn2 = zoo.parts[1].spawn.cell();
    let spawn3 = zoo.parts[2].spawn.cell();
    let count_in = |level: &Level, r: zoo_core::Rect| {
        let reach = reach_of(level.grid(), spawn1);
        r.cells()
            .filter(|&c| level.grid().index(c).is_some_and(|k| reach[k]))
            .count()
    };
    let closed = zoo_level(&[]);
    assert_eq!(count_in(&closed, b2), 0, "level 2 reachable while locked");
    assert_eq!(count_in(&closed, b3), 0, "level 3 reachable while locked");
    let l2 = zoo_level(&L2_OPEN);
    let reach = reach_of(l2.grid(), spawn1);
    assert!(reach[l2.grid().index(spawn2).unwrap()], "level-2 spawn");
    assert_eq!(count_in(&l2, b3), 0, "level 3 reachable with level 2 open");
    let l3 = zoo_level(&L3_OPEN);
    let reach = reach_of(l3.grid(), spawn1);
    assert!(reach[l3.grid().index(spawn3).unwrap()], "level-3 spawn");
    // walking back: from the level-3 spawn to the level-1 spawn
    let back = reach_of(l3.grid(), spawn3);
    assert!(back[l3.grid().index(spawn1).unwrap()]);
}

// LAYOUT-023, LAYOUT-L3-003 (interior part): an enterable building's interior and door are
// walkable floor (surface path), its other cells solid, the door next to a walkable cell
// outside.
#[test]
fn layout_023_enterable_building() {
    let zoo = common::zoo_with_night();
    let level = Level::new(zoo.clone());
    let grid = level.grid();
    let houses: Vec<_> = zoo.elements.iter().filter(|e| e.is_enterable()).collect();
    let mut ids: Vec<&str> = houses.iter().map(|e| e.id.as_str()).collect();
    ids.sort_unstable();
    // every building with a door (LAYOUT-041): the zookeeper houses of levels 1, 2 and 3 (one
    // bed each, GAME-NIGHT, LAYOUT-047), the night house of night_1, the food storages and the food hut
    assert_eq!(
        ids,
        [
            "food_storage",
            "food_storage_2",
            "food_storage_3",
            "food_storage_n1",
            "night_house",
            "zookeeper_house_1",
            "zookeeper_house_2",
            "zookeeper_house_3"
        ]
    );
    for e in houses {
        let inner = e.interior.unwrap();
        let door = e.door_cell().unwrap();
        for c in e.rect.cells() {
            let open = inner.contains(c) || c == door;
            if open {
                assert_eq!(
                    grid.kind(c),
                    CellKind::Walkable(Surface::Path),
                    "{}: {c}",
                    e.id
                );
            } else {
                assert_eq!(grid.kind(c), CellKind::Solid, "{}: {c}", e.id);
            }
        }
        let outside = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y]
            .iter()
            .map(|d| door + *d)
            .any(|c| !e.rect.contains(c) && grid.is_walkable(c, false));
        assert!(outside, "{}: door leads nowhere", e.id);
    }
}

/// Element / scenery kinds a riddle relies on (scenery of the hiding places).
fn riddle_kinds(zoo: &LevelData) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for h in &zoo.hiding_places {
        for id in &h.scenery {
            let kind = zoo
                .element(id)
                .and_then(|e| e.kind.clone())
                .or_else(|| {
                    zoo.scenery
                        .iter()
                        .find(|s| &s.id == id)
                        .map(|s| s.kind.clone())
                })
                .unwrap_or_else(|| panic!("{}: scenery {id} missing", h.id));
            out.push((h.id.clone(), kind));
        }
    }
    out
}

// LAYOUT-024, LAYOUT-L2-008 / L3-008 (uniqueness part), LAYOUT-L2-014 (kind once): every
// kind a riddle relies on occurs once in the joined map. Shared carriers of several places
// (the stream of the three goldfish places) are the same element.
#[test]
fn layout_024_riddle_kinds_unique_zoo_wide() {
    let zoo = common::zoo();
    let mut owners: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for h in &zoo.hiding_places {
        for id in &h.scenery {
            owners
                .entry(id.clone())
                .or_default()
                .insert(h.animal.clone());
        }
    }
    // Background kinds are not riddle details on their own: water bodies shared by the places
    // of one animal (the riddles name the waterfall / wheel / willow), the level-1 tree areas
    // and outer walls, and the level-1 rock hill (one hill built from three elements).
    let background = [
        "stream",
        "river",
        "pond",
        "trees",
        "tree_grove",
        "zoo_wall",
        "hedge",
        "rock_hill",
    ];
    let mut problems = Vec::new();
    for (place, kind) in riddle_kinds(&zoo) {
        if background.contains(&kind.as_str()) {
            continue;
        }
        let n = zoo
            .elements
            .iter()
            .filter(|e| e.kind.as_deref() == Some(kind.as_str()))
            .count()
            + zoo.scenery.iter().filter(|s| s.kind == kind).count();
        if n != 1 {
            problems.push(format!("{place}: kind {kind} occurs {n} times"));
        }
    }
    for (id, animals) in owners {
        let kind = zoo
            .element(&id)
            .and_then(|e| e.kind.clone())
            .unwrap_or_default();
        if animals.len() > 1 && !background.contains(&kind.as_str()) {
            problems.push(format!("{id} is scenery of several animals {animals:?}"));
        }
    }
    assert!(problems.is_empty(), "{problems:#?}");
}

// ------------------------------------------------------------------ per level

struct LevelCase {
    id: &'static str,
    open: &'static [&'static str],
    spec: &'static str,
    animals: &'static [&'static str],
    /// Heights for the sight test (m).
    heights: &'static [(&'static str, f32)],
}

const L2: LevelCase = LevelCase {
    id: "level_2",
    open: &L2_OPEN,
    spec: "specs/10-gameplay/levels/level-2.md",
    animals: &["koala", "elephant", "giraffe", "lion"],
    heights: &[
        ("koala", 0.9),
        ("elephant", 3.0),
        ("giraffe", 4.5),
        ("lion", 1.5),
    ],
};

const L3: LevelCase = LevelCase {
    id: "level_3",
    open: &L3_OPEN,
    spec: "specs/10-gameplay/levels/level-3.md",
    animals: &["monkey", "goldfish", "snow_fox"],
    heights: &[("monkey", 1.1), ("goldfish", 0.3), ("snow_fox", 0.9)],
};

fn part_data(id: &str) -> LevelData {
    match id {
        "level_2" => common::level2(),
        _ => common::level3(),
    }
}

// LAYOUT-L2-001, LAYOUT-L3-001: everything of the level is reachable from its spawn and
// from the level-1 spawn in the joined map.
#[test]
fn layout_l2_001_l3_001_reachable() {
    for case in [L2, L3] {
        let zoo = common::zoo();
        let level = zoo_level(case.open);
        let grid = level.grid();
        let k = zoo.part_index(case.id).unwrap();
        let from_spawn = reach_of(grid, zoo.parts[k].spawn.cell());
        let from_l1 = reach_of(grid, zoo.parts[0].spawn.cell());
        let mut missing = Vec::new();
        for e in zoo.elements.iter().filter(|e| e.part == k) {
            let must = matches!(
                e.ty,
                ElementType::Enclosure
                    | ElementType::Building
                    | ElementType::Landmark
                    | ElementType::Barrier
            );
            if must {
                for reach in [&from_spawn, &from_l1] {
                    if !reachable_near(grid, reach, e.rect) {
                        missing.push(e.id.clone());
                    }
                }
            }
        }
        for h in zoo.hiding_places.iter().filter(|h| h.part == k) {
            if !reachable_near(grid, &from_spawn, h.rect) {
                missing.push(h.id.clone());
            }
        }
        assert!(missing.is_empty(), "{}: not reachable {missing:?}", case.id);
    }
}

// LAYOUT-L2-003, LAYOUT-L3-003: no solid overlaps, no path under a solid element.
#[test]
fn layout_l2_003_l3_003_no_overlaps() {
    for case in [L2, L3] {
        let data = part_data(case.id);
        let overlaps = data.solid_overlaps();
        assert!(overlaps.is_empty(), "{}: {overlaps:?}", case.id);
        let grid = Level::new(common::zoo()).grid().clone();
        let mut covered = Vec::new();
        let zoo = common::zoo();
        for e in data.elements_of(ElementType::Path) {
            for c in e.rect.cells() {
                // the street runs on under the level-transition barriers (LAYOUT-040)
                if let Some(s) = grid
                    .solid_element(c)
                    .filter(|&s| zoo.elements[s].ty != ElementType::Barrier)
                {
                    covered.push((e.id.clone(), s, c));
                }
            }
        }
        assert!(
            covered.is_empty(),
            "{}: path under solid {covered:?}",
            case.id
        );
    }
}

fn parse_ints(s: &str) -> Vec<i32> {
    s.split(',')
        .filter_map(|p| p.trim().replace('−', "-").parse().ok())
        .collect()
}

fn paren_after(text: &str, name: &str) -> Option<Vec<i32>> {
    let i = text.find(name)? + name.len();
    let rest = text[i..].trim_start();
    let rest = rest.strip_prefix('(')?;
    let end = rest.find(')')?;
    Some(parse_ints(&rest[..end])).filter(|v| !v.is_empty())
}

// LAYOUT-L2-004, LAYOUT-L3-004 (LAYOUT-005): the spec's element table and the data agree.
#[test]
fn layout_l2_004_l3_004_spec_table_matches_data() {
    for case in [L2, L3] {
        let data = part_data(case.id);
        let md = common::read(case.spec);
        let section = md.split("## Elements").nth(1).expect("Elements section");
        let section = section.split("\n## ").next().unwrap();
        let mut ids = BTreeSet::new();
        let mut diffs = Vec::new();
        for line in section.lines() {
            let cols: Vec<&str> = line.split('|').map(str::trim).collect();
            if cols.len() < 5 || !cols[1].starts_with('`') {
                continue;
            }
            let id = cols[1].trim_matches('`').to_owned();
            ids.insert(id.clone());
            let Some(e) = data.element(&id) else {
                diffs.push(format!("{id}: not in the data"));
                continue;
            };
            let mut parts = cols[2].splitn(2, '(');
            let ty = parts.next().unwrap().trim();
            let kind = parts.next().map(|k| k.trim_end_matches(')').trim());
            if e.ty.as_str() != ty {
                diffs.push(format!("{id}: type {ty} vs {}", e.ty.as_str()));
            }
            if let Some(k) = kind {
                if e.kind.as_deref() != Some(k) {
                    diffs.push(format!("{id}: kind {k} vs {:?}", e.kind));
                }
            }
            let r = parse_ints(cols[3]);
            if r != [e.rect.x, e.rect.z, e.rect.w, e.rect.d] {
                diffs.push(format!("{id}: rect {r:?} vs {:?}", e.rect));
            }
            let notes = cols[4];
            if let Some(g) = paren_after(notes, "gate") {
                let dg = e.gate.map(|r| vec![r.x, r.z, r.w, r.d]);
                if dg.as_ref() != Some(&g) {
                    diffs.push(format!("{id}: gate {g:?} vs {dg:?}"));
                }
            }
            if let Some(d) = paren_after(notes, "door at cell") {
                if e.door.map(|a| a.to_vec()) != Some(d.clone()) {
                    diffs.push(format!("{id}: door {d:?} vs {:?}", e.door));
                }
            }
            if let Some(i) = paren_after(notes, "walkable interior") {
                let di = e.interior.map(|r| vec![r.x, r.z, r.w, r.d]);
                if di.as_ref() != Some(&i) {
                    diffs.push(format!("{id}: interior {i:?} vs {di:?}"));
                }
            }
        }
        let data_ids: BTreeSet<String> = data.elements.iter().map(|e| e.id.clone()).collect();
        if ids != data_ids {
            diffs.push(format!(
                "ids differ: spec only {:?}, data only {:?}",
                ids.difference(&data_ids).collect::<Vec<_>>(),
                data_ids.difference(&ids).collect::<Vec<_>>()
            ));
        }
        assert!(diffs.is_empty(), "{}: {diffs:#?}", case.id);
    }
}

/// A named point set of the walking tables.
fn point(zoo: &LevelData, grid: &Grid, name: &str) -> Vec<IVec2> {
    if let Some(h) = zoo.hiding_place(name) {
        return near_spot(grid, h.spot_cell());
    }
    let cell = |x: i32, z: i32| vec![IVec2::new(x, z)];
    match name {
        "spawn2" => vec![zoo.parts[1].spawn.cell()],
        "spawn3" => vec![zoo.parts[2].spawn.cell()],
        "storage2" => cell(38, 30),
        "storage3" => cell(3, 60),
        "house_door" => cell(-8, 60),
        "bowl" => cell(-8, 64),
        "entry_s" => zoo_core::Rect::new(-9, 48, 3, 1).cells().collect(),
        "construction" => {
            walkable_adjacent(grid, zoo.element("barrier_l2_construction").unwrap().rect)
        }
        id => walkable_adjacent(grid, zoo.element(id).unwrap_or_else(|| panic!("{id}")).rect),
    }
}

// LAYOUT-L2-005, LAYOUT-L3-005: walking-table pairs and each hiding place → neighbour are
// ≤ 10 s (1.93 / 0.98 m/s); the spec numbers are reproduced within 0.5 s.
#[test]
fn layout_l2_005_l3_005_walking_times() {
    let l2: &[(&str, &str, f32)] = &[
        ("spawn2", "map_board_l2", 2.8),
        ("spawn2", "storage2", 5.9),
        ("storage2", "board_koala", 4.9),
        ("storage2", "board_lion", 5.4),
        ("board_lion", "board_elephant", 6.7),
        ("board_elephant", "board_giraffe", 8.8),
        ("board_giraffe", "board_koala", 4.1),
        ("board_koala", "loc_log_pile", 6.6),
        ("loc_treehouse", "loc_blossom_tree", 5.1),
        ("loc_fountain", "spawn2", 2.1),
        ("loc_lookout_tower", "board_lion", 4.7),
        ("loc_train", "board_lion", 8.7),
        ("loc_playground", "loc_tallest_tree", 7.4),
        ("loc_blossom_tree", "loc_deckchairs", 3.4),
        ("loc_stage", "board_giraffe", 7.0),
        ("loc_deckchairs", "loc_stage", 4.3),
        ("loc_big_ball", "loc_deckchairs", 3.3),
        ("loc_sun_rocks", "construction", 2.8),
        ("loc_log_pile", "construction", 2.3),
    ];
    let l3: &[(&str, &str, f32)] = &[
        ("spawn3", "map_board_l3", 0.7),
        ("spawn3", "loc_pirate_ship", 4.9),
        ("loc_pirate_ship", "storage3", 4.8),
        ("storage3", "house_door", 5.7),
        ("house_door", "bowl", 2.1),
        ("storage3", "board_snow_fox", 3.4),
        ("board_snow_fox", "house_door", 2.8),
        ("storage3", "board_goldfish", 7.5),
        ("board_goldfish", "board_monkey", 8.8),
        ("house_door", "entry_s", 6.2),
        ("board_snow_fox", "entry_s", 6.4),
        ("loc_carousel", "entry_s", 3.4),
        ("loc_trampoline", "loc_carousel", 3.4),
        ("loc_waterfall", "loc_sprinkler", 5.4),
        ("loc_water_wheel", "loc_laundry", 4.9),
        ("loc_willow", "house_door", 7.4),
        ("loc_ice_cream_kiosk", "board_monkey", 9.4),
        ("loc_sprinkler", "board_monkey", 6.3),
        ("loc_laundry", "board_monkey", 6.9),
    ];
    let zoo = common::zoo();
    let level = zoo_level(&L3_OPEN);
    let grid = level.grid();
    let mut too_slow = Vec::new();
    let mut drifted = Vec::new();
    for (from, to, spec) in l2.iter().chain(l3) {
        let a = point(&zoo, grid, from);
        let b = point(&zoo, grid, to);
        assert!(!a.is_empty() && !b.is_empty(), "{from} → {to}: empty");
        let t = min_cost(grid, &a, &b, time_cost());
        println!("{from} → {to}: {t:.1} s (spec {spec})");
        if t > 10.0 + 1e-3 {
            too_slow.push(format!("{from} → {to}: {t:.1} s"));
        }
        if (t - spec).abs() > 0.5 {
            drifted.push(format!("{from} → {to}: {t:.1} s vs spec {spec}"));
        }
    }
    assert!(too_slow.is_empty(), "over 10 s: {too_slow:#?}");
    assert!(drifted.is_empty(), "spec tables drifted: {drifted:#?}");
}

/// Whether level points at a height are inside the view of the GAME-PLAYER §2 camera.
fn on_screen(player: Vec2, forward: Vec2, dist: f32, aspect: f32, pts: &[(Vec2, f32)]) -> bool {
    use glam::camera::rh::{proj::opengl, view::look_at_mat4};
    use zoo_core::coords::{level_to_world, level_to_world_at, WORLD_UP};
    let target = level_to_world(player) + WORLD_UP * 0.7;
    let fwd = level_to_world(forward).normalize();
    let pitch = 55f32.to_radians();
    let eye = target - fwd * dist * pitch.cos() + WORLD_UP * dist * pitch.sin();
    let vp = opengl::perspective(35f32.to_radians(), aspect, 0.1, 200.0)
        * look_at_mat4(eye, target, WORLD_UP);
    pts.iter().any(|(p, h)| {
        let c = vp * level_to_world_at(*p, *h).extend(1.0);
        c.w > 0.0 && (c.x / c.w).abs() <= 1.0 && (c.y / c.w).abs() <= 1.0
    })
}

// LAYOUT-L2-006, LAYOUT-L3-006: no wander cell of any candidate (0.5 m, animal height,
// perch + 1 m) is on screen from next to its own info board or gate (portrait 1080×2340,
// 8 rotations, zoom 10/14/20 m).
#[test]
fn layout_l2_006_l3_006_sight_test() {
    let zoo = common::zoo();
    let level = zoo_level(&L3_OPEN);
    let grid = level.grid();
    let mut seen = Vec::new();
    for case in [L2, L3] {
        for &animal in case.animals {
            let enc = zoo
                .elements
                .iter()
                .find(|e| e.ty == ElementType::Enclosure && e.animal.as_deref() == Some(animal))
                .unwrap();
            let board = zoo
                .elements
                .iter()
                .find(|e| {
                    e.kind.as_deref() == Some("info_board")
                        && e.enclosure.as_deref() == Some(enc.id.as_str())
                })
                .unwrap();
            let mut stands = walkable_adjacent(grid, board.rect);
            stands.extend(walkable_adjacent(grid, enc.gate.unwrap()));
            let height = case.heights.iter().find(|(a, _)| *a == animal).unwrap().1;
            for h in zoo.hiding_places_of(animal) {
                let area = hiding_area(&level, h);
                let mut pts = Vec::new();
                for (c, _) in area.cells() {
                    let p = cell_center(c);
                    pts.push((p, 0.5));
                    pts.push((p, height));
                    if let Some(ph) = h.perch_height_m {
                        pts.push((p, ph + 1.0));
                    }
                }
                'stand: for s in &stands {
                    for k in 0..8 {
                        let a = (k as f32 * 45f32).to_radians();
                        let fwd = Vec2::new(a.sin(), a.cos());
                        for dist in [10.0, 14.0, 20.0] {
                            if on_screen(cell_center(*s), fwd, dist, 1080.0 / 2340.0, &pts) {
                                seen.push(format!("{} from {s} (rot {k}, {dist} m)", h.id));
                                break 'stand;
                            }
                        }
                    }
                }
            }
        }
    }
    assert!(
        seen.is_empty(),
        "hiding places visible from their board/gate: {seen:#?}"
    );
}

// LAYOUT-L2-007, LAYOUT-L3-007: the riddle details are in the features; scenery ids exist.
#[test]
fn layout_l2_007_l3_007_features() {
    let zoo = common::zoo();
    let expect: &[(&str, &[&str])] = &[
        ("loc_treehouse", &["tree_house", "rope_ladder"]),
        ("loc_tallest_tree", &["tallest_tree"]),
        ("loc_blossom_tree", &["pink_blossoms", "falling_petals"]),
        ("loc_fountain", &["water_jet", "stone_basin", "coins"]),
        ("loc_log_pile", &["stacked_logs", "sawdust"]),
        ("loc_big_ball", &["giant_ball"]),
        (
            "loc_lookout_tower",
            &["wooden_tower", "stairs", "high_platform"],
        ),
        ("loc_train", &["train", "bell"]),
        ("loc_playground", &["slide", "swings"]),
        ("loc_sun_rocks", &["flat_rocks", "full_sun"]),
        ("loc_stage", &["stage", "drums"]),
        ("loc_deckchairs", &["striped_deckchairs", "sunshade"]),
        (
            "loc_pirate_ship",
            &["ship", "mast", "sail", "flag", "treasure_chest"],
        ),
        ("loc_carousel", &["carousel", "wooden_horses"]),
        ("loc_trampoline", &["trampoline"]),
        ("loc_waterfall", &["falling_water", "foam"]),
        ("loc_water_wheel", &["water_wheel", "clattering"]),
        ("loc_willow", &["weeping_willow", "hanging_branches"]),
        (
            "loc_ice_cream_kiosk",
            &["freezer_chest", "cold_air", "cones"],
        ),
        ("loc_sprinkler", &["sprinkler", "cold_drops", "rainbow"]),
        ("loc_laundry", &["white_sheets", "washing_line"]),
    ];
    for (id, feats) in expect {
        let h = zoo.hiding_place(id).unwrap_or_else(|| panic!("{id}"));
        for f in *feats {
            assert!(h.features.iter().any(|x| x == f), "{id} lacks {f}");
        }
        for sc in &h.scenery {
            assert!(
                zoo.scenery.iter().any(|x| &x.id == sc) || zoo.element(sc).is_some(),
                "{id}: scenery {sc} missing"
            );
        }
    }
}

// LAYOUT-L2-008, LAYOUT-L3-008: landmark kinds exactly once; only the giant tree is above
// 7 m; no rocks in the lion enclosure; slide/swings only in level 2; no bridge / jetty at
// the stream.
#[test]
fn layout_l2_008_l3_008_kinds_once() {
    let zoo = common::zoo();
    let count = |k: &str| {
        zoo.elements
            .iter()
            .filter(|e| e.kind.as_deref() == Some(k))
            .count()
            + zoo.scenery.iter().filter(|s| s.kind == k).count()
    };
    for k in [
        "fountain",
        "treehouse",
        "giant_tree",
        "blossom_tree",
        "play_ball",
        "log_pile",
        "lookout_tower",
        "zoo_train",
        "slide",
        "swings",
        "stage",
        "deckchairs",
        "flat_rocks",
        "petal_carpet",
        "waterfall",
        "mill_hut",
        "willow",
        "pirate_ship",
        "carousel",
        "kiosk",
        "washing_line",
        "trampoline",
        "wet_lawn",
        "bark_mulch",
    ] {
        assert_eq!(count(k), 1, "{k}");
    }
    for e in &zoo.elements {
        let tree = e.kind.as_deref().is_some_and(|k| {
            k.contains("tree") || k == "trees" || k == "willow" || k == "treehouse"
        });
        if tree && e.id != "tree_giant_e" {
            assert!(e.height_m.unwrap_or(0.0) <= 7.0, "{} taller than 7 m", e.id);
        }
    }
    let lion = zoo.element("enc_lion").unwrap();
    let _ = lion; // notes are free text (not parsed): the scene places no rock in enc_lion
    let l2 = zoo.part_index("level_2").unwrap();
    for e in &zoo.elements {
        if matches!(e.kind.as_deref(), Some("slide" | "swings")) {
            assert_eq!(e.part, l2, "{}", e.id);
        }
    }
    let stream = zoo.element("stream_l3").unwrap().rect;
    let grown = zoo_core::Rect::new(stream.x - 1, stream.z - 1, stream.w + 2, stream.d + 2);
    for e in &zoo.elements {
        if matches!(e.kind.as_deref(), Some("bridge" | "jetty")) {
            assert!(
                !e.rect.cells().any(|c| grown.contains(c)),
                "{} at the stream",
                e.id
            );
        }
    }
    // the scene puts no rock model into the lion enclosure (riddle guard for loc_sun_rocks)
    let scene = zoo_core::scene::LevelScene::build(&zoo);
    let enc = zoo.element("enc_lion").unwrap().rect;
    for p in scene.placements.iter().filter(|p| p.model == "rock") {
        let l = zoo_core::coords::world_to_level(p.pos);
        assert!(
            !enc.contains(zoo_core::level::cell_of(l)),
            "rock in enc_lion"
        );
    }
}

// LAYOUT-L2-009, LAYOUT-L3-009: every spot is within 2 m of a walkable cell; the goldfish
// spots are stream cells with a bank cell within 2 m and stream-only wander areas.
#[test]
fn layout_l2_009_l3_009_spots_in_reach() {
    let zoo = common::zoo();
    let level = zoo_level(&L3_OPEN);
    let grid = level.grid();
    let stream = zoo.element("stream_l3").unwrap().rect;
    for h in zoo.hiding_places.iter().filter(|h| h.part > 0) {
        assert!(
            !near_spot(grid, h.spot_cell()).is_empty(),
            "{}: out of reach",
            h.id
        );
        if h.animal == "goldfish" {
            assert!(
                stream.contains(h.spot_cell()),
                "{}: spot not in the stream",
                h.id
            );
            let area = hiding_area(&level, h);
            assert!(
                area.cells().all(|(c, _)| stream.contains(c)),
                "{}: leaves the stream",
                h.id
            );
        }
    }
}

// LAYOUT-L2-011, LAYOUT-L3-011: 10 food boxes (one per food) in front of the level's
// storage, each with a reachable walkable standing cell within 2 m; plus 2-6 more real,
// labelled food boxes inside the storage (Q-194 answered 2026-09-29), foods may repeat.
#[test]
fn layout_l2_011_l3_011_food_boxes() {
    for (case, storage) in [(L2, "food_storage_2"), (L3, "food_storage_3")] {
        let data = part_data(case.id);
        let rect = data.element(storage).unwrap().rect;
        let outside: Vec<_> = data
            .food_boxes
            .iter()
            .filter(|b| !rect.contains(cell_of(b.pos())))
            .collect();
        let inside: Vec<_> = data
            .food_boxes
            .iter()
            .filter(|b| rect.contains(cell_of(b.pos())))
            .collect();
        assert_eq!(outside.len(), 10, "{}", case.id);
        let foods: BTreeSet<&str> = outside.iter().map(|b| b.food.as_str()).collect();
        assert_eq!(foods.len(), 10);
        assert!(
            (2..=6).contains(&inside.len()),
            "{}: {} inside food boxes",
            case.id,
            inside.len()
        );
        let zoo = common::zoo();
        let level = zoo_level(case.open);
        let grid = level.grid();
        let k = zoo.part_index(case.id).unwrap();
        let reach = reach_of(grid, zoo.parts[k].spawn.cell());
        for b in &data.food_boxes {
            assert!(
                rect.distance_to(b.pos()) <= 1.0,
                "{}: far from {storage}",
                b.food
            );
            let ok = zoo_core::Rect::new(b.pos().x as i32 - 3, b.pos().y as i32 - 3, 7, 7)
                .cells()
                .any(|c| {
                    grid.is_passable(c, false)
                        && cell_center(c).distance(b.pos()) <= 2.0
                        && reach[grid.index(c).unwrap()]
                });
            assert!(ok, "{}: no standing cell", b.food);
        }
    }
}

// LAYOUT-L2-013, LAYOUT-L3-012 (LAYOUT-014): ≥ 3 candidates per animal, wander areas ≥ 9
// cells inside their rect, disjoint between animals, and every candidate in a spread
// combination (57 of 81 / 12 of 27; FIX-056).
#[test]
fn layout_l2_013_l3_012_wander_areas_and_spread() {
    for (case, valid) in [(L2, 57), (L3, 12)] {
        let zoo = common::zoo();
        let level = zoo_level(case.open);
        let mut areas = Vec::new();
        let per: Vec<Vec<_>> = case
            .animals
            .iter()
            .map(|a| zoo.hiding_places_of(a).collect::<Vec<_>>())
            .collect();
        for (a, list) in case.animals.iter().zip(&per) {
            assert!(list.len() >= 3, "{a}: {} candidates", list.len());
            for h in list {
                let area = hiding_area(&level, h);
                let cells: Vec<IVec2> = area.cells().map(|(c, _)| c).collect();
                assert!(cells.len() >= 9, "{}: {} cells", h.id, cells.len());
                assert!(
                    cells.iter().all(|c| h.rect.contains(*c)),
                    "{}: outside rect",
                    h.id
                );
                areas.push((h.animal.clone(), h.id.clone(), h.rect, cells));
            }
        }
        for (i, a) in areas.iter().enumerate() {
            for b in &areas[i + 1..] {
                if a.0 != b.0 {
                    assert!(!a.3.iter().any(|c| b.3.contains(c)), "{} / {}", a.1, b.1);
                    assert!(
                        !a.2.cells().any(|c| b.2.contains(c)),
                        "rects {} / {}",
                        a.1,
                        b.1
                    );
                }
            }
        }
        // combinations
        let mut idx = vec![0usize; per.len()];
        let mut n_valid = 0;
        let mut used: BTreeSet<String> = BTreeSet::new();
        loop {
            let set: Vec<_> = per.iter().zip(&idx).map(|(l, &i)| l[i]).collect();
            let ok = set.iter().enumerate().all(|(i, x)| {
                set[i + 1..]
                    .iter()
                    .all(|y| x.spot().distance(y.spot()) >= 12.0 - 1e-4)
            });
            if ok {
                n_valid += 1;
                used.extend(set.iter().map(|h| h.id.clone()));
            }
            let mut k = 0;
            loop {
                if k == idx.len() {
                    break;
                }
                idx[k] += 1;
                if idx[k] < per[k].len() {
                    break;
                }
                idx[k] = 0;
                k += 1;
            }
            if k == idx.len() {
                break;
            }
        }
        assert_eq!(n_valid, valid, "{}: valid combinations", case.id);
        for list in &per {
            for h in list {
                assert!(used.contains(&h.id), "{} in no valid combination", h.id);
            }
        }
    }
}

// LAYOUT-L2-014, LAYOUT-L3-013: scenery overlaps no solid element or path and lies inside
// its hiding place's rect.
#[test]
fn layout_l2_014_l3_013_scenery() {
    let zoo = common::zoo();
    let level = zoo_level(&L3_OPEN);
    for sc in zoo
        .scenery
        .iter()
        .filter(|s| zoo.part_at(IVec2::new(s.rect.x, s.rect.z)) != Some(0))
    {
        for c in sc.rect.cells() {
            assert!(level.grid().is_walkable(c, false), "{}: solid {c}", sc.id);
            assert!(!level.grid().has_path(c), "{}: path {c}", sc.id);
        }
        let h = zoo
            .hiding_place(sc.hiding_place.as_deref().unwrap())
            .unwrap();
        assert!(
            sc.rect.cells().all(|c| h.rect.contains(c)),
            "{} outside {}",
            sc.id,
            h.id
        );
    }
}

// LAYOUT-L2-016: the elephant pool covers 35–60 % of the enclosure, lies inside it, is not
// next to the gate, and the elephant's home area includes pool cells.
// LAYOUT-L3-015: the goldfish pond lies inside its enclosure, not next to the gate, and the
// goldfish's home area is pond cells only.
#[test]
fn layout_l2_016_l3_015_pools() {
    let zoo = common::zoo();
    let level = zoo_level(&L3_OPEN);
    for (enc_id, feature, only_water) in [
        ("enc_elephant", "elephant_pool", false),
        ("enc_goldfish", "goldfish_pond", true),
    ] {
        let k = zoo.elements.iter().position(|e| e.id == enc_id).unwrap();
        let enc = &zoo.elements[k];
        let f = zoo
            .enclosure_features
            .iter()
            .find(|f| f.id == feature)
            .unwrap();
        assert!(
            f.rect.cells().all(|c| enc.rect.contains(c)),
            "{feature} outside"
        );
        let gate = enc.gate.unwrap();
        assert!(
            !f.rect
                .cells()
                .any(|c| gate.is_adjacent(c) || gate.contains(c)),
            "{feature} at the gate"
        );
        if !only_water {
            let share = (f.rect.w * f.rect.d) as f32 / (enc.rect.w * enc.rect.d) as f32;
            assert!((0.35..=0.60).contains(&share), "{feature}: {share:.2}");
        }
        let home = home_area(&level, k);
        let water = home.cells().filter(|(c, _)| f.rect.contains(*c)).count();
        assert!(water >= 9, "{enc_id}: {water} pool cells at home");
        if only_water {
            assert!(
                home.cells().all(|(c, _)| f.rect.contains(c)),
                "{enc_id}: land cells"
            );
        }
    }
}

// LAYOUT-L3-010: the bowl stands on a reachable interior cell (next to the table), the tap
// has a walkable cell within 1.5 m, every stream bank cell is a fill point, and a walkable
// cell is edge-adjacent to the goldfish gate step.
#[test]
fn layout_l3_010_bowl_tap_banks_step() {
    let zoo = common::zoo();
    let level = zoo_level(&L3_OPEN);
    let grid = level.grid();
    let house = zoo.element("zookeeper_house_3").unwrap();
    let bowl = zoo.items.iter().find(|i| i.kind == "fish_bowl").unwrap();
    let inner = house.interior.unwrap();
    assert!(
        inner.contains(zoo_core::level::cell_of(bowl.pos())),
        "bowl not inside"
    );
    let reach = reach_of(grid, zoo.parts[2].spawn.cell());
    let stand = inner
        .cells()
        .filter(|&c| grid.is_passable(c, false) && reach[grid.index(c).unwrap()])
        .any(|c| cell_center(c).distance(bowl.pos()) <= 2.0);
    assert!(stand, "no reachable cell next to the table");
    let tap = zoo.water_sources.iter().find(|w| w.kind == "tap").unwrap();
    let tap_pos = Vec2::from(tap.pos.unwrap());
    let near = zoo_core::Rect::new(tap_pos.x as i32 - 2, tap_pos.y as i32 - 2, 5, 5)
        .cells()
        .any(|c| grid.is_walkable(c, false) && cell_center(c).distance(tap_pos) <= 1.5);
    assert!(near, "tap out of reach");
    let bank = zoo.water_sources.iter().find(|w| w.kind == "bank").unwrap();
    assert_eq!(bank.water.as_deref(), Some("stream_l3"));
    let enc = zoo.element("enc_goldfish").unwrap();
    let gate = enc.gate.unwrap();
    let front = gate
        .cells()
        .flat_map(|c| {
            [
                c + IVec2::X,
                c + IVec2::NEG_X,
                c + IVec2::Y,
                c + IVec2::NEG_Y,
            ]
        })
        .any(|c| !enc.rect.contains(c) && grid.is_walkable(c, false));
    assert!(front, "nothing in front of the gate step");
    // a player on any stream bank cell can fill the bowl (the stream edge within 2 m)
    let mut g = common::zoo_game(1);
    for b in L3_OPEN {
        g.level.open_barrier(b);
    }
    let stream = zoo.element("stream_l3").unwrap().rect;
    let mut banks = 0;
    for c in zoo_core::Rect::new(stream.x - 1, stream.z, stream.w + 2, stream.d).cells() {
        if stream.contains(c) || !grid.is_walkable(c, false) {
            continue;
        }
        g.player.pos = cell_center(c);
        let (_, q) = g.water_point().expect("a water point");
        assert!(q.distance(g.player.pos) <= 2.0, "bank {c}: {q}");
        banks += 1;
    }
    assert!(banks > 20, "{banks} bank cells");
}

// LAYOUT-L2-010: with the four level-2 missions complete the construction fence opens (the
// next morning, GAME-NIGHT / Q-091) and the level-3 spawn is reachable; the fallen tree never
// closes again.
#[test]
fn layout_l2_010_construction_fence_opens_after_level_2() {
    let mut g = common::zoo_game(3);
    for a in ["zebra", "hippo", "panda"] {
        assert!(g.debug_send_home(a));
    }
    assert!(
        !g.level.is_barrier_open("barrier_ne_tree"),
        "only the next morning"
    );
    g.debug_next_morning();
    assert!(g.level.is_barrier_open("barrier_ne_tree"));
    assert!(!g.level.is_barrier_open("barrier_l2_construction"));
    assert!(g.level_unlocked("level_2") && !g.level_unlocked("level_3"));
    for a in ["koala", "elephant", "giraffe"] {
        assert!(g.debug_send_home(a));
        assert!(
            !g.level.is_barrier_open("barrier_l2_construction"),
            "after {a}"
        );
    }
    assert!(g.debug_send_home("lion"));
    assert!(!g.level.is_barrier_open("barrier_l2_construction"));
    g.debug_next_morning();
    assert!(g.level.is_barrier_open("barrier_l2_construction"));
    assert!(
        g.level.is_barrier_open("barrier_north_gate"),
        "second entry (Q-090)"
    );
    assert!(g.level.is_barrier_open("barrier_ne_tree"));
    assert!(g.level_unlocked("level_3"));
    let reach = reach_of(g.level.grid(), g.level.data.parts[0].spawn.cell());
    let s3 = g.level.data.parts[2].spawn.cell();
    assert!(reach[g.level.grid().index(s3).unwrap()]);
}

// LAYOUT-046 (rules 6, 14 — level design rules, user request 2026-09-30, Q-202 answered):
// from the spawn / entry of every day level the first step — an info board — is reachable
// within 15 s of walking (all barriers open), and every enclosure gate has a `path` cell
// within 2 m (rule 8: streets lead to the gates).
#[test]
fn layout_046_first_step_within_15_s_and_gates_on_streets() {
    let zoo = common::zoo();
    let level = zoo_level(&L3_OPEN);
    let grid = level.grid();
    for (k, part) in zoo.parts.iter().enumerate() {
        let spawn = [part.spawn.cell()];
        let boards: Vec<IVec2> = zoo
            .elements
            .iter()
            .filter(|e| {
                e.id.starts_with("board_") && zoo.part_at(IVec2::new(e.rect.x, e.rect.z)) == Some(k)
            })
            .flat_map(|e| walkable_adjacent(grid, e.rect))
            .collect();
        assert!(!boards.is_empty(), "{}: no info board", part.id);
        let t = min_cost(grid, &spawn, &boards, time_cost());
        assert!(
            t <= 15.0,
            "{}: first board {t:.1} s from the spawn",
            part.id
        );
    }
    let mut no_street = Vec::new();
    for e in zoo.elements.iter().filter(|e| e.animal.is_some()) {
        let Some(gate) = e.gate else { continue };
        let near_street = (gate.x - 2..gate.x + gate.w + 2)
            .flat_map(|x| (gate.z - 2..gate.z + gate.d + 2).map(move |z| IVec2::new(x, z)))
            .any(|c| grid.surface(c) == Some(zoo_core::level::Surface::Path));
        if !near_street {
            no_street.push(e.id.clone());
        }
    }
    assert!(
        no_street.is_empty(),
        "gates without a street within 2 m (rule 8): {no_street:?}"
    );
}

// AENV-014: the carousel, the ice cream kiosk, the slide and the swings are drawn by their
// `kit_landmarks_play` models (centred on the element rect, front south), each with a
// placeholder fallback; the footprint stays the solid element rect (cell collision).
#[test]
fn aenv_014_play_landmarks_use_their_models() {
    let zoo = common::zoo();
    let scene = zoo_core::scene::LevelScene::build(&zoo);
    for (id, model) in [
        ("carousel_sw", "carousel"),
        ("ice_cream_kiosk", "ice_cream_kiosk"),
        ("playground_se_slide", "playground_slide"),
        ("playground_se_swings", "playground_swings"),
        // AENV-016
        ("train_se", "zoo_train"),
        ("tree_blossom_ne", "blossom_tree"),
    ] {
        let e = zoo.element(id).unwrap();
        let c = Vec2::new(
            e.rect.x as f32 + e.rect.w as f32 / 2.0,
            e.rect.z as f32 + e.rect.d as f32 / 2.0,
        );
        let n = scene
            .placements
            .iter()
            .filter(|p| p.model == model)
            .inspect(|p| {
                let l = zoo_core::coords::world_to_level(p.pos);
                assert!(l.distance(c) < 0.01, "{model} at the centre of {id}");
                assert_eq!(p.yaw, 0.0, "{model}: front south");
            })
            .count();
        assert_eq!(n, 1, "{model} placed once");
        assert!(
            scene.fallbacks.iter().any(|f| f.model == model),
            "{model}: placeholder fallback"
        );
        // solid for the player: the element's cells
        let grid = Level::new(common::zoo());
        assert!(
            e.rect
                .cells()
                .all(|cell| grid.grid().kind(cell) == CellKind::Solid),
            "{id}: solid cells"
        );
    }
}
