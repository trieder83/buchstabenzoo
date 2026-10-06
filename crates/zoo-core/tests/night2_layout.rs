//! GAME-LEVEL-NIGHT-2 unit tests (LAYOUT-N2-001…011, LAYOUT-N1-014) on `assets/levels/night-2.toml`
//! and the garden gate of `night-1.toml`. (LAYOUT-N2-012/013 are e2e tests, N2-014 a manual review.)

mod common;

use std::collections::{BTreeMap, BTreeSet};

use glam::{IVec2, Vec2};
use zoo_core::level::{
    cell_center, cell_of, CellKind, ElementType, Grid, Level, LevelData, Surface,
};
use zoo_core::nav::{flood_fill, min_cost, Cost};
use zoo_core::wander::hiding_area;
use zoo_core::Rect;

const SPEC: &str = "specs/10-gameplay/levels/night-2.md";
const ANIMALS: [&str; 3] = ["snake", "chameleon", "poison_dart_frog"];
const GATE: &str = "barrier_n1_garden";

// ------------------------------------------------------------------ helpers

fn night2_level() -> Level {
    Level::new(common::night2())
}

/// The whole game with the garden gate open (and nothing else).
fn joined_gate_open() -> Level {
    let mut level = Level::new(common::zoo_with_night2());
    assert!(level.open_barrier(GATE), "{GATE} is a barrier");
    level
}

fn night1_spawn() -> IVec2 {
    IVec2::new(-28, 29)
}

fn part(data: &LevelData, id: &str) -> usize {
    data.part_index(id).unwrap_or_else(|| panic!("{id} joined"))
}

fn reachable_near(grid: &Grid, reach: &[bool], rect: Rect) -> bool {
    Rect::new(rect.x - 1, rect.z - 1, rect.w + 2, rect.d + 2)
        .cells()
        .any(|c| grid.index(c).is_some_and(|k| reach[k]) && grid.is_walkable(c, false))
}

fn walkable_adjacent(grid: &Grid, rect: Rect) -> Vec<IVec2> {
    Rect::new(rect.x - 1, rect.z - 1, rect.w + 2, rect.d + 2)
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

fn walkable_within(grid: &Grid, p: Vec2, r: f32) -> Vec<IVec2> {
    let n = r.ceil() as i32 + 1;
    Rect::new(
        p.x.floor() as i32 - n,
        p.y.floor() as i32 - n,
        2 * n + 1,
        2 * n + 1,
    )
    .cells()
    .filter(|&c| grid.is_walkable(c, false) && cell_center(c).distance(p) <= r)
    .collect()
}

fn time_cost() -> Cost {
    let mp = zoo_core::player::MoveParams::default();
    Cost::Time {
        path_speed: mp.speed_on(Surface::Path),
        grass_speed: mp.speed_on(Surface::Grass),
    }
}

fn is_border(b: Rect, c: IVec2) -> bool {
    c.x == b.x || c.x == b.x + b.w - 1 || c.y == b.z || c.y == b.z + b.d - 1
}

fn rects_overlap(a: Rect, b: Rect) -> bool {
    a.cells().any(|c| b.contains(c))
}

fn rect_center(r: Rect) -> Vec2 {
    Vec2::new(r.x as f32 + r.w as f32 / 2.0, r.z as f32 + r.d as f32 / 2.0)
}

fn parse_ints(s: &str) -> Vec<i32> {
    s.split(',')
        .filter_map(|p| p.trim().replace('−', "-").parse().ok())
        .collect()
}

fn paren_after(text: &str, name: &str) -> Option<Vec<i32>> {
    let i = text.find(name)? + name.len();
    let rest = text[i..].trim_start().strip_prefix('(')?;
    let end = rest.find(')')?;
    Some(parse_ints(&rest[..end])).filter(|v| !v.is_empty())
}

fn raw(rel: &str) -> toml::Table {
    common::read(rel)
        .parse::<toml::Table>()
        .unwrap_or_else(|e| panic!("{rel}: {e}"))
}

fn raw_list<'a>(t: &'a toml::Table, key: &str) -> Vec<&'a toml::Table> {
    t.get(key)
        .and_then(|v| v.as_array())
        .map(|a| a.iter().filter_map(|v| v.as_table()).collect())
        .unwrap_or_default()
}

fn raw_str<'a>(t: &'a toml::Table, key: &str) -> Option<&'a str> {
    t.get(key).and_then(|v| v.as_str())
}

// ------------------------------------------------------------------ LAYOUT-N2-001

// LAYOUT-N2-001: with the garden gate open, every enclosure, building, landmark, board, food box
// and hiding place of night_2 has a walkable cell next to / inside it, reachable from the night_1
// spawn.
#[test]
fn layout_n2_001_everything_reachable_through_the_open_gate() {
    let level = joined_gate_open();
    let data = &level.data;
    let grid = level.grid();
    let k = part(data, "night_2");
    let reach = flood_fill(grid, night1_spawn(), false);
    let mut missing = Vec::new();
    let mut checked = 0;
    for e in data.elements.iter().filter(|e| e.part == k) {
        let must = matches!(
            e.ty,
            ElementType::Enclosure | ElementType::Building | ElementType::Landmark
        ) || e.kind.as_deref() == Some("info_board");
        if must {
            checked += 1;
            if !reachable_near(grid, &reach, e.rect) {
                missing.push(e.id.clone());
            }
        }
    }
    for b in data.food_boxes.iter().filter(|b| b.part == k) {
        checked += 1;
        let c = cell_of(b.pos());
        if !reachable_near(grid, &reach, Rect::new(c.x, c.y, 1, 1)) {
            missing.push(format!("food box {}", b.food));
        }
    }
    for h in data.hiding_places.iter().filter(|h| h.part == k) {
        checked += 1;
        if !reachable_near(
            grid,
            &reach,
            Rect::new(h.animal_spot[0], h.animal_spot[1], 1, 1),
        ) {
            missing.push(h.id.clone());
        }
    }
    assert!(checked >= 20, "only {checked} things checked");
    assert!(missing.is_empty(), "unreachable: {missing:?}");
}

// ------------------------------------------------------------------ LAYOUT-N2-002

// LAYOUT-N2-002: the gate closed → no cell of night_2 reachable from the night_1 spawn; every
// border cell of night_2 except (−73, 25) and (−73, 26) is solid; the night_1 cells of the
// barrier are the gate.
#[test]
fn layout_n2_002_sealed_while_the_gate_is_closed() {
    let level = Level::new(common::zoo_with_night2());
    assert!(!level.is_barrier_open(GATE));
    let data = &level.data;
    let grid = level.grid();
    let b = data.parts[part(data, "night_2")].bounds;
    assert_eq!([b.x, b.z, b.w, b.d], [-120, 6, 48, 48], "night_2 bounds");
    for gates_allowed in [false, true] {
        let reach = flood_fill(grid, night1_spawn(), gates_allowed);
        let reached: Vec<IVec2> = b
            .cells()
            .filter(|&c| grid.index(c).is_some_and(|k| reach[k]))
            .collect();
        assert!(reached.is_empty(), "night_2 reachable: {reached:?}");
    }
    let entry: BTreeSet<(i32, i32)> = [(-73, 25), (-73, 26)].into_iter().collect();
    let mut open = Vec::new();
    for c in b.cells().filter(|&c| is_border(b, c)) {
        if entry.contains(&(c.x, c.y)) {
            assert_eq!(grid.kind(c), CellKind::Walkable(Surface::Path), "entry {c}");
        } else if grid.kind(c) != CellKind::Solid {
            open.push((c, grid.kind(c)));
        }
    }
    assert!(open.is_empty(), "non-solid border cells: {open:?}");
    let gate = data.element(GATE).expect("barrier_n1_garden");
    assert_eq!(gate.ty, ElementType::Barrier);
    assert_eq!(
        [gate.rect.x, gate.rect.z, gate.rect.w, gate.rect.d],
        [-72, 25, 2, 2]
    );
    for c in gate.rect.cells() {
        assert_eq!(grid.kind(c), CellKind::Solid, "closed gate cell {c}");
    }
    // open: the cells are walkable
    let open_level = joined_gate_open();
    for c in gate.rect.cells() {
        assert!(
            open_level.grid().is_walkable(c, false),
            "open gate cell {c}"
        );
    }
}

// ------------------------------------------------------------------ LAYOUT-N2-003

// LAYOUT-N2-003: no two solid elements share a cell, no path cell lies under a solid element; the
// bounds are disjoint from all other levels.
#[test]
fn layout_n2_003_no_overlaps_and_disjoint_bounds() {
    let zoo = common::zoo_with_night2();
    assert!(
        zoo.solid_overlaps().is_empty(),
        "{:?}",
        zoo.solid_overlaps()
    );
    let k = part(&zoo, "night_2");
    let b = zoo.parts[k].bounds;
    for (i, p) in zoo.parts.iter().enumerate() {
        if i != k {
            assert!(!rects_overlap(b, p.bounds), "bounds overlap {}", p.id);
        }
    }
    let solids: Vec<_> = zoo
        .elements
        .iter()
        .filter(|e| e.is_solid() && e.part == k)
        .collect();
    for p in zoo
        .elements
        .iter()
        .filter(|e| e.ty == ElementType::Path && e.part == k)
    {
        for s in &solids {
            if let Some(c) = p.rect.cells().find(|c| s.rect.contains(*c)) {
                // the enterable buildings' hall and door are path cells inside the building rect
                if !s.is_open_cell(c) {
                    panic!("path {} under solid {} at {c}", p.id, s.id);
                }
            }
        }
    }
}

// ------------------------------------------------------------------ LAYOUT-N2-004

// LAYOUT-N2-004 (LAYOUT-005): the spec's element table and night-2.toml list the same ids, types,
// rectangles, gates, doors, interiors, model_rects and flags.
#[test]
fn layout_n2_004_spec_table_matches_data() {
    let data = common::night2();
    let md = common::read(SPEC);
    let section = md.split("\n## Elements").nth(1).expect("Elements section");
    let section = section.split("\n## ").next().unwrap();
    let mut ids = BTreeSet::new();
    let mut diffs = Vec::new();
    for line in section.lines() {
        let cols: Vec<&str> = line.split('|').map(str::trim).collect();
        if cols.len() < 5 || !cols[1].starts_with('`') {
            continue;
        }
        let id = cols[1].trim_matches('`').to_owned();
        if !ids.insert(id.clone()) {
            diffs.push(format!("{id}: listed twice in the spec"));
        }
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
        let as_vec = |r: Option<Rect>| r.map(|r| vec![r.x, r.z, r.w, r.d]);
        if let Some(g) = paren_after(notes, "gate") {
            if as_vec(e.gate).as_ref() != Some(&g) {
                diffs.push(format!("{id}: gate {g:?} vs {:?}", e.gate));
            }
        }
        if let Some(d) = paren_after(notes, "`door`") {
            if e.door.map(|a| a.to_vec()) != Some(d.clone()) {
                diffs.push(format!("{id}: door {d:?} vs {:?}", e.door));
            }
        }
        if let Some(i) = paren_after(notes, "`interior`") {
            if as_vec(e.interior).as_ref() != Some(&i) {
                diffs.push(format!("{id}: interior {i:?} vs {:?}", e.interior));
            }
        }
        if let Some(m) = paren_after(notes, "`model_rect`") {
            if as_vec(e.model_rect).as_ref() != Some(&m) {
                diffs.push(format!("{id}: model_rect {m:?} vs {:?}", e.model_rect));
            }
        }
        if notes.contains("`indoor = true`") && !e.indoor {
            diffs.push(format!("{id}: indoor in the spec, not in the data"));
        }
        if notes.contains("`terrarium = true`") && !e.terrarium {
            diffs.push(format!("{id}: terrarium in the spec, not in the data"));
        }
        if notes.contains("`pair = true`") && !e.pair {
            diffs.push(format!("{id}: pair in the spec, not in the data"));
        }
        if notes.contains("`density = \"dense\"`") && e.density.as_deref() != Some("dense") {
            diffs.push(format!(
                "{id}: dense in the spec, {:?} in the data",
                e.density
            ));
        }
        if let Some(i) = notes.find("board of `") {
            let rest = &notes[i + "board of `".len()..];
            let enc = &rest[..rest.find('`').unwrap()];
            if e.enclosure.as_deref() != Some(enc) {
                diffs.push(format!("{id}: board of {enc} vs {:?}", e.enclosure));
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
    // the scenery table: same ids and rects as the `[[scenery]]` entries
    let sc_section = md.split("\n## Scenery").nth(1).expect("Scenery section");
    let sc_section = sc_section.split("\n## ").next().unwrap();
    let mut sc_ids = BTreeSet::new();
    for line in sc_section.lines() {
        let cols: Vec<&str> = line.split('|').map(str::trim).collect();
        if cols.len() < 5 || !cols[1].starts_with('`') {
            continue;
        }
        let id = cols[1].trim_matches('`').to_owned();
        sc_ids.insert(id.clone());
        match data.scenery.iter().find(|s| s.id == id) {
            None => diffs.push(format!("scenery {id}: not in the data")),
            Some(s) => {
                let r = parse_ints(cols[3]);
                if r != [s.rect.x, s.rect.z, s.rect.w, s.rect.d] {
                    diffs.push(format!("scenery {id}: rect {r:?} vs {:?}", s.rect));
                }
            }
        }
    }
    let data_sc: BTreeSet<String> = data.scenery.iter().map(|s| s.id.clone()).collect();
    if sc_ids != data_sc {
        diffs.push(format!(
            "scenery ids differ: spec {sc_ids:?}, data {data_sc:?}"
        ));
    }
    assert!(diffs.is_empty(), "{diffs:#?}");
}

// ------------------------------------------------------------------ LAYOUT-N2-005

fn point(level: &Level, name: &str) -> Vec<IVec2> {
    let data = &level.data;
    let grid = level.grid();
    if let Some(h) = data.hiding_place(name) {
        return walkable_within(grid, cell_center(h.spot_cell()), 2.0);
    }
    let house = data.element("terrarium_house").unwrap();
    let hall_cells = house.interior_cells();
    let hall_in_front_of_gate = |enc: &str| -> Vec<IVec2> {
        let gate = data.element(enc).unwrap().gate.unwrap();
        walkable_adjacent(grid, gate)
            .into_iter()
            .filter(|c| hall_cells.contains(c))
            .collect()
    };
    match name {
        "entry" => data.entries[0].cells.cells().collect(),
        "food boxes" => data
            .food_boxes
            .iter()
            .filter(|b| {
                !data
                    .element("food_storage_n2")
                    .unwrap()
                    .rect
                    .contains(cell_of(b.pos()))
            })
            .flat_map(|b| {
                walkable_within(grid, b.pos(), 2.0)
                    .into_iter()
                    .filter(|c| (cell_center(*c) - b.pos()).dot(b.facing()) > 0.0)
                    .collect::<Vec<_>>()
            })
            .collect(),
        "map board" => walkable_adjacent(grid, data.element("map_board_n2").unwrap().rect),
        "snake board" => walkable_adjacent(grid, data.element("board_n2_snake").unwrap().rect),
        "chameleon board" => {
            walkable_adjacent(grid, data.element("board_n2_chameleon").unwrap().rect)
        }
        "frog board" => walkable_adjacent(grid, data.element("board_n2_frog").unwrap().rect),
        "house door" => vec![house.door_cell().unwrap()],
        "snake gate" => hall_in_front_of_gate("enc_n2_snake"),
        "frog gate" => hall_in_front_of_gate("enc_n2_frog"),
        other => panic!("unknown point {other}"),
    }
}

// LAYOUT-N2-005: with 1.93 m/s on paths and 1.45 m/s on grass, each leg of "Walking and pacing" is
// ≤ 10 s, the entry → first board ≤ 15 s; measured values are printed next to the estimates.
#[test]
fn layout_n2_005_walking_times() {
    let mp = zoo_core::player::MoveParams::default();
    assert!(
        (mp.speed_on(Surface::Path) - 1.93).abs() < 0.01,
        "path speed"
    );
    assert!(
        (mp.speed_on(Surface::Grass) - 1.45).abs() < 0.01,
        "grass speed"
    );
    let table: &[(&str, &str, f32)] = &[
        ("entry", "food boxes", 7.5),
        ("entry", "map board", 2.0),
        ("food boxes", "frog board", 8.0),
        ("food boxes", "house door", 7.0),
        ("house door", "snake gate", 4.0),
        ("house door", "frog gate", 4.0),
        ("loc_stone_wall", "loc_pumpkins", 7.0),
        ("loc_pumpkins", "loc_rowing_boat", 6.0),
        ("loc_rowing_boat", "loc_ferns", 8.0),
        ("loc_ferns", "loc_rain_barrel", 5.5),
        ("loc_lanterns", "loc_palm", 2.5),
        ("loc_palm", "loc_stepping_stones", 7.0),
        ("loc_stepping_stones", "loc_vine_arch", 7.0),
        ("loc_stepping_stones", "food boxes", 9.0),
    ];
    let level = night2_level();
    let grid = level.grid();
    let mut too_slow = Vec::new();
    for (from, to, spec) in table {
        let (a, b) = (point(&level, from), point(&level, to));
        assert!(
            !a.is_empty() && !b.is_empty(),
            "{from} → {to}: empty point set"
        );
        let t = min_cost(grid, &a, &b, time_cost());
        println!("LAYOUT-N2-005: {from} → {to}: {t:.1} s (spec {spec})");
        if t > 10.0 + 1e-3 {
            too_slow.push(format!("{from} → {to}: {t:.1} s"));
        }
    }
    let first = min_cost(
        grid,
        &point(&level, "entry"),
        &point(&level, "frog board"),
        time_cost(),
    );
    println!("LAYOUT-N2-005: entry → first board: {first:.1} s");
    assert!(first <= 15.0, "entry → first board {first:.1} s");
    assert!(too_slow.is_empty(), "over 10 s: {too_slow:#?}");
}

// ------------------------------------------------------------------ LAYOUT-N2-006

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

fn standing_points(level: &Level, animal: &str) -> Vec<IVec2> {
    let data = &level.data;
    let grid = level.grid();
    let enc = data
        .elements
        .iter()
        .find(|e| e.ty == ElementType::Enclosure && e.animal.as_deref() == Some(animal))
        .unwrap_or_else(|| panic!("{animal}: no enclosure"));
    let board = data
        .elements
        .iter()
        .find(|e| {
            e.kind.as_deref() == Some("info_board") && e.enclosure.as_deref() == Some(&enc.id)
        })
        .unwrap_or_else(|| panic!("{animal}: no board"));
    let mut stands = walkable_within(grid, rect_center(board.rect), 2.5);
    stands.extend(walkable_adjacent(grid, enc.gate.unwrap()));
    stands.sort_by_key(|c| (c.x, c.y));
    stands.dedup();
    stands
}

// LAYOUT-N2-006: for snake, chameleon, poison dart frog, every candidate, the player on every
// walkable cell ≤ 2.5 m from the own board or in front of the own gate, the zoo camera at every
// 45° rotation and 10 / 14 / 20 m on 1080×2340 → no wander cell centre (0.5 m, 1.0 m, perch + 1 m)
// is on screen, and every one is ≥ 22 m from the standing points (CAMV-008, Q-110, FIX-056).
#[test]
fn layout_n2_006_sight_test_and_haze_margin() {
    let level = night2_level();
    let mut seen = Vec::new();
    let mut close = Vec::new();
    let mut nearest = (f32::MAX, String::new());
    for animal in ANIMALS {
        let stands = standing_points(&level, animal);
        assert!(!stands.is_empty(), "{animal}: nowhere to stand");
        for h in level.data.hiding_places_of(animal) {
            let area = hiding_area(&level, h);
            let mut pts = Vec::new();
            for (c, _) in area.cells() {
                let p = cell_center(c);
                pts.push((p, 0.5));
                pts.push((p, 1.0));
                if let Some(ph) = h.perch_height_m {
                    pts.push((p, ph + 1.0));
                }
                for s in &stands {
                    let d = p.distance(cell_center(*s));
                    if d < nearest.0 {
                        nearest = (d, format!("{} {c} ↔ {s}", h.id));
                    }
                    if d < 22.0 {
                        close.push(format!("{}: wander cell {c} is {d:.1} m from {s}", h.id));
                    }
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
    println!(
        "LAYOUT-N2-006: nearest wander cell {:.1} m — {}",
        nearest.0, nearest.1
    );
    assert!(
        seen.is_empty(),
        "wander cells on screen from board/gate: {seen:#?}"
    );
    assert!(
        close.is_empty(),
        "wander cells < 22 m from a standing point: {close:#?}"
    );
}

// ------------------------------------------------------------------ LAYOUT-N2-007

// LAYOUT-N2-007: each hiding place has ≥ 9 wander cells (the spec table's counts), a cell ≥ 2 m
// from the spot, no solid or path cell, lies inside its rect; places of different animals do not
// overlap; all 27 combinations keep the spots ≥ 12 m apart.
#[test]
fn layout_n2_007_wander_areas_and_spread() {
    let table: [(&str, usize); 9] = [
        ("loc_stone_wall", 0),
        ("loc_pumpkins", 0),
        ("loc_rowing_boat", 0),
        ("loc_lanterns", 0),
        ("loc_palm", 0),
        ("loc_vine_arch", 0),
        ("loc_stepping_stones", 0),
        ("loc_ferns", 0),
        ("loc_rain_barrel", 0),
    ];
    let spec_counts = spec_wander_counts();
    let level = night2_level();
    let data = &level.data;
    let grid = level.grid();
    let mut problems = Vec::new();
    let mut areas: Vec<(String, String, Rect, Vec<IVec2>)> = Vec::new();
    for animal in ANIMALS {
        let list: Vec<_> = data.hiding_places_of(animal).collect();
        if list.len() < 3 {
            problems.push(format!("{animal}: {} candidates", list.len()));
        }
        for h in list {
            let cells: Vec<IVec2> = hiding_area(&level, h).cells().map(|(c, _)| c).collect();
            println!("LAYOUT-N2-007: {} {} wander cells", h.id, cells.len());
            if cells.len() < 9 {
                problems.push(format!("{}: {} wander cells < 9", h.id, cells.len()));
            }
            if spec_counts.get(h.id.as_str()) != Some(&cells.len()) {
                problems.push(format!(
                    "{}: {} wander cells, spec table {:?}",
                    h.id,
                    cells.len(),
                    spec_counts.get(h.id.as_str())
                ));
            }
            if !cells
                .iter()
                .any(|c| cell_center(*c).distance(h.spot()) >= 2.0)
            {
                problems.push(format!("{}: no wander cell ≥ 2 m from the spot", h.id));
            }
            for c in &cells {
                if !h.rect.contains(*c) {
                    problems.push(format!("{}: wander cell {c} outside its rect", h.id));
                }
                if !grid.is_walkable(*c, false) || grid.solid_element(*c).is_some() {
                    problems.push(format!("{}: solid wander cell {c}", h.id));
                }
                if grid.has_path(*c) {
                    problems.push(format!("{}: path wander cell {c}", h.id));
                }
            }
            areas.push((h.animal.clone(), h.id.clone(), h.rect, cells));
        }
    }
    assert_eq!(areas.len(), table.len(), "9 candidates");
    for (id, _) in table {
        assert!(areas.iter().any(|a| a.1 == id), "{id} missing");
    }
    for (i, a) in areas.iter().enumerate() {
        for b in &areas[i + 1..] {
            if a.0 == b.0 {
                continue;
            }
            if rects_overlap(a.2, b.2) {
                problems.push(format!("rects {} / {} overlap", a.1, b.1));
            }
            if a.3.iter().any(|c| b.3.contains(c)) {
                problems.push(format!("wander areas {} / {} overlap", a.1, b.1));
            }
        }
    }
    let per: Vec<Vec<_>> = ANIMALS
        .iter()
        .map(|a| data.hiding_places_of(a).collect::<Vec<_>>())
        .collect();
    let mut combos = 0;
    let mut closest = (f32::MAX, String::new());
    for x in &per[0] {
        for y in &per[1] {
            for z in &per[2] {
                combos += 1;
                for (p, q) in [(x, y), (x, z), (y, z)] {
                    let d = p.spot().distance(q.spot());
                    if d < closest.0 {
                        closest = (d, format!("{} – {}", p.id, q.id));
                    }
                    if d < 12.0 - 1e-4 {
                        problems.push(format!("{} – {}: {d:.1} m < 12 m", p.id, q.id));
                    }
                }
            }
        }
    }
    assert_eq!(combos, 27);
    println!(
        "LAYOUT-N2-007: closest spots {:.1} m ({})",
        closest.0, closest.1
    );
    problems.sort();
    problems.dedup();
    assert!(problems.is_empty(), "{problems:#?}");
}

/// The "Wander cells" column of the spec's hiding-place table.
fn spec_wander_counts() -> BTreeMap<String, usize> {
    let md = common::read(SPEC);
    let section = md
        .split("\n## Hiding places")
        .nth(1)
        .expect("Hiding places");
    let section = section.split("\n## ").next().unwrap();
    let mut out = BTreeMap::new();
    for line in section.lines() {
        let cols: Vec<&str> = line.split('|').map(str::trim).collect();
        if cols.len() > 6 && cols[1].starts_with("`loc_") {
            if let Ok(n) = cols[6].parse::<usize>() {
                out.insert(cols[1].trim_matches('`').to_owned(), n);
            }
        }
    }
    out
}

// ------------------------------------------------------------------ LAYOUT-N2-008

// LAYOUT-N2-008: the features / scenery of every place exist, each place's features exist only
// there within the night levels, each riddle element kind occurs once in night_1 ∪ night_2.
#[test]
fn layout_n2_008_features_scenery_and_unique_kinds() {
    let zoo = common::zoo_with_night2();
    let night: BTreeSet<usize> = (0..zoo.parts.len())
        .filter(|&k| zoo.is_night_part(k))
        .collect();
    let expect: &[(&str, &[&str], &str)] = &[
        (
            "loc_stone_wall",
            &["stone_wall", "stacked_stones", "warm_stones"],
            "stone_wall",
        ),
        (
            "loc_pumpkins",
            &["pumpkins", "orange_fruit", "vines"],
            "pumpkin_patch",
        ),
        (
            "loc_rowing_boat",
            &["rowing_boat", "blue_boat", "oars"],
            "rowing_boat",
        ),
        (
            "loc_lanterns",
            &["paper_lanterns", "coloured_lights"],
            "lantern_tree",
        ),
        ("loc_palm", &["palm", "fan_leaves", "coconuts"], "palm_tree"),
        (
            "loc_vine_arch",
            &["wooden_arch", "creepers", "pink_flowers"],
            "vine_arch",
        ),
        (
            "loc_stepping_stones",
            &["stepping_stones", "wet_moss", "squelching"],
            "stepping_stones",
        ),
        (
            "loc_ferns",
            &["giant_ferns", "mist", "old_stump"],
            "fern_glade",
        ),
        (
            "loc_rain_barrel",
            &["barrel", "dripping", "eave"],
            "rain_barrel",
        ),
    ];
    let mut problems = Vec::new();
    for (id, feats, kind) in expect {
        let Some(h) = zoo.hiding_place(id) else {
            problems.push(format!("{id}: missing"));
            continue;
        };
        for f in *feats {
            if !h.features.iter().any(|x| x == f) {
                problems.push(format!("{id} lacks feature {f}"));
            }
        }
        for sc in &h.scenery {
            if !(zoo.scenery.iter().any(|x| &x.id == sc) || zoo.element(sc).is_some()) {
                problems.push(format!("{id}: scenery {sc} missing"));
            }
        }
        let n = zoo
            .elements
            .iter()
            .filter(|e| night.contains(&e.part) && e.kind.as_deref() == Some(kind))
            .count()
            + zoo.scenery.iter().filter(|s| s.kind == *kind).count();
        if n != 1 {
            problems.push(format!("kind {kind} occurs {n} times in the night levels"));
        }
    }
    let mut owners: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for h in zoo.hiding_places.iter().filter(|h| night.contains(&h.part)) {
        for f in &h.features {
            owners.entry(f.as_str()).or_default().push(h.id.as_str());
        }
    }
    for (f, places) in owners {
        if places.len() > 1 {
            problems.push(format!("feature {f} at several night places {places:?}"));
        }
    }
    assert!(problems.is_empty(), "{problems:#?}");
}

// ------------------------------------------------------------------ LAYOUT-N2-009

// LAYOUT-N2-009: the terrarium house's interior and door are walkable `path` cells, each terrarium
// has its gate edge-adjacent to an interior cell, `model_rect` contains the hall and the cases,
// all three enclosures carry `terrarium = true` and `indoor = true`, and the boards stand outside.
#[test]
fn layout_n2_009_terrarium_house() {
    let level = night2_level();
    let data = &level.data;
    let grid = level.grid();
    let house = data.element("terrarium_house").expect("terrarium_house");
    assert_eq!(house.ty, ElementType::Building);
    let hall = house.interior_cells();
    let door = house.door_cell().expect("door");
    for c in hall.iter().copied().chain([door]) {
        assert_eq!(
            grid.kind(c),
            CellKind::Walkable(Surface::Path),
            "hall/door {c}"
        );
    }
    let model = house.model_rect.expect("model_rect");
    assert!(
        hall.iter().all(|c| model.contains(*c)),
        "hall outside model_rect"
    );
    let cases: Vec<_> = data
        .elements
        .iter()
        .filter(|e| e.ty == ElementType::Enclosure)
        .collect();
    let mut animals: Vec<&str> = cases.iter().filter_map(|e| e.animal.as_deref()).collect();
    animals.sort_unstable();
    assert_eq!(
        animals,
        ["chameleon", "poison_dart_frog", "snake"],
        "terrariums"
    );
    for e in cases {
        assert!(
            e.indoor && e.terrarium && e.pair,
            "{}: indoor + terrarium + pair",
            e.id
        );
        let gate = e.gate.expect("gate");
        let to_hall = gate
            .cells()
            .flat_map(|c| {
                [
                    c + IVec2::X,
                    c + IVec2::NEG_X,
                    c + IVec2::Y,
                    c + IVec2::NEG_Y,
                ]
            })
            .any(|c| hall.contains(&c));
        assert!(to_hall, "{}: gate not next to the hall", e.id);
        assert!(
            e.rect.cells().all(|c| model.contains(c)),
            "{} outside model_rect",
            e.id
        );
        assert!(
            e.rect.cells().count() >= 12,
            "{}: fewer than 12 home cells",
            e.id
        );
        let board = data
            .elements
            .iter()
            .find(|b| {
                b.kind.as_deref() == Some("info_board") && b.enclosure.as_deref() == Some(&e.id)
            })
            .unwrap_or_else(|| panic!("{}: no board", e.id));
        assert!(
            !rects_overlap(board.rect, house.rect) && !rects_overlap(board.rect, model),
            "{} inside the house",
            board.id
        );
    }
}

// ------------------------------------------------------------------ LAYOUT-N2-010

// LAYOUT-N2-010: the food boxes: outside fish, crickets, flies, beetles (one each); inside eggs and
// frozen_insects; every box has a reachable standing cell within 2 m in front of it.
#[test]
fn layout_n2_010_food_boxes() {
    let level = night2_level();
    let data = &level.data;
    let grid = level.grid();
    let hut = data.element("food_storage_n2").unwrap().rect;
    let (mut outside, mut inside): (Vec<&str>, Vec<&str>) = (Vec::new(), Vec::new());
    for b in &data.food_boxes {
        if hut.contains(cell_of(b.pos())) {
            inside.push(&b.food);
        } else {
            outside.push(&b.food);
        }
    }
    outside.sort_unstable();
    inside.sort_unstable();
    assert_eq!(outside, ["beetles", "crickets", "fish", "flies"]);
    assert_eq!(inside, ["eggs", "frozen_insects"]);
    let reach = flood_fill(grid, data.parts[0].spawn.cell(), false);
    for b in &data.food_boxes {
        let ok = walkable_within(grid, b.pos(), 2.0).into_iter().any(|c| {
            grid.is_passable(c, false)
                && (cell_center(c) - b.pos()).dot(b.facing()) > 0.0
                && reach[grid.index(c).unwrap()]
        });
        assert!(ok, "{}: no reachable standing cell in front", b.food);
    }
}

// ------------------------------------------------------------------ LAYOUT-N2-011

// LAYOUT-N2-011: lantern posts / string ends on walkable cells outside hiding rects and scenery,
// exactly one board_lamp per board and the map board, every attach id exists, the house has a hall
// light and one `indoor` light per terrarium with the spec colours.
#[test]
fn layout_n2_011_lights() {
    let level = night2_level();
    let data = &level.data;
    let grid = level.grid();
    let mut problems = Vec::new();
    for l in &data.lights {
        if let Some(a) = &l.attach {
            if data.element(a).is_none() {
                problems.push(format!("{}: attach {a} missing", l.id));
            }
        }
        if !matches!(l.kind.as_str(), "lantern_post" | "string_lights") {
            continue;
        }
        for p in l.points() {
            let c = cell_of(p);
            if !grid.is_walkable(c, false) {
                problems.push(format!("{}: {p} on a non-walkable cell {c}", l.id));
            }
            for h in &data.hiding_places {
                if h.rect.contains(c) {
                    problems.push(format!("{}: {p} inside {}", l.id, h.id));
                }
            }
            for s in &data.scenery {
                if s.rect.contains(c) {
                    problems.push(format!("{}: {p} inside scenery {}", l.id, s.id));
                }
            }
        }
    }
    for b in data
        .elements
        .iter()
        .filter(|e| matches!(e.kind.as_deref(), Some("info_board" | "map_board")))
    {
        let n = data
            .lights
            .iter()
            .filter(|l| l.kind == "board_lamp" && l.attach.as_deref() == Some(&b.id))
            .count();
        if n != 1 {
            problems.push(format!("{}: {n} board lamps", b.id));
        }
    }
    let house = data.element("terrarium_house").unwrap();
    let model = house.model_rect.unwrap();
    for (attach, color) in [
        ("terrarium_house", "#FFE9B0"),
        ("enc_n2_snake", "#F2A93B"),
        ("enc_n2_chameleon", "#9B6BE0"),
        ("enc_n2_frog", "#3CC7A0"),
    ] {
        let lights: Vec<_> = data
            .lights
            .iter()
            .filter(|l| l.kind == "indoor" && l.attach.as_deref() == Some(attach))
            .collect();
        if lights.len() != 1 {
            problems.push(format!("{attach}: {} indoor lights", lights.len()));
        }
        for l in lights {
            if !l
                .color
                .as_deref()
                .is_some_and(|c| c.eq_ignore_ascii_case(color))
            {
                problems.push(format!("{}: colour {:?}, spec {color}", l.id, l.color));
            }
            if !l.pos().is_some_and(|p| model.contains(cell_of(p))) {
                problems.push(format!("{}: not inside the terrarium house", l.id));
            }
        }
    }
    assert!(problems.is_empty(), "{problems:#?}");
}

// ------------------------------------------------------------------ LAYOUT-N1-014

// LAYOUT-N1-014: night-1.toml has `path_n1_gate` and `barrier_n1_garden` (unlock_after night_1,
// opens_at night, transition night_1->night_2); the gate band z 24..26 holds no wander cell, so the
// existing hiding places of night_1 are unchanged (LAYOUT-N1-006/007/008 stay green).
#[test]
fn layout_n1_014_garden_gate_in_night_1() {
    let t = raw("assets/levels/night-1.toml");
    let els = raw_list(&t, "element");
    let gate = els
        .iter()
        .find(|e| raw_str(e, "id") == Some(GATE))
        .expect("barrier_n1_garden");
    assert_eq!(raw_str(gate, "type"), Some("barrier"));
    assert_eq!(raw_str(gate, "unlock_after"), Some("night_1"));
    assert_eq!(raw_str(gate, "opens_at"), Some("night"));
    assert_eq!(raw_str(gate, "transition"), Some("night_1->night_2"));
    let level = Level::new(common::zoo_with_night2());
    let data = &level.data;
    let k1 = part(data, "night_1");
    let path = data.element("path_n1_gate").expect("path_n1_gate");
    assert_eq!(path.ty, ElementType::Path);
    for h in data.hiding_places.iter().filter(|h| h.part == k1) {
        for (c, _) in hiding_area(&level, h).cells() {
            assert!(
                !(24..=26).contains(&c.y) || c.x > -64,
                "{}: wander cell {c} in the gate band",
                h.id
            );
        }
    }
    // the gate band is walkable path from the ring to the gate once the gate is open
    let open = joined_gate_open();
    for x in -72..=-65 {
        for z in 25..=26 {
            assert_eq!(
                open.grid().kind(IVec2::new(x, z)),
                CellKind::Walkable(Surface::Path)
            );
        }
    }
    // the west hedge is split around the gate
    for id in ["hedge_n1_west", "hedge_n1_west_n"] {
        assert!(data.element(id).is_some(), "{id}");
    }
}
