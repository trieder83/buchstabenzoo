//! GAME-LEVEL-NIGHT-1 unit tests (LAYOUT-N1-001…011) on `assets/levels/night-1.toml`, and the
//! GAME-LAYOUT night / light / event rules LAYOUT-027…030 on every level file.
//! (LAYOUT-N1-012 is an e2e test, LAYOUT-N1-013 a manual review.)

mod common;

use std::collections::{BTreeMap, BTreeSet};

use glam::{IVec2, Vec2};
use zoo_core::daytime::{Phase, CELEBRATION_S, DUSK_S, MORNING_S, SLEEP_S};
use zoo_core::level::{
    cell_center, cell_of, CellKind, ElementType, Grid, Level, LevelData, Surface,
};
use zoo_core::nav::{flood_fill, min_cost, Cost};
use zoo_core::wander::hiding_area;
use zoo_core::{Game, Rect};

const NIGHT_SPEC: &str = "specs/10-gameplay/levels/night-1.md";
const NIGHT_ANIMALS: [&str; 3] = ["hedgehog", "bat", "owl"];
const MOON_DOOR: &str = "moon_door";

// ------------------------------------------------------------------ helpers

fn night_level() -> Level {
    Level::new(common::night1())
}

/// Levels 1–3 + `night_1` joined, the moon door open (and nothing else).
fn joined_moon_open() -> Level {
    let mut level = Level::new(common::zoo_with_night());
    assert!(level.open_barrier(MOON_DOOR), "moon_door is a barrier");
    level
}

fn entry_cell() -> IVec2 {
    IVec2::new(-25, 29)
}

/// Walkable cells inside or 8-adjacent to a rectangle that the flood fill reached.
fn reachable_near(grid: &Grid, reach: &[bool], rect: Rect) -> bool {
    Rect::new(rect.x - 1, rect.z - 1, rect.w + 2, rect.d + 2)
        .cells()
        .any(|c| grid.index(c).is_some_and(|k| reach[k]) && grid.is_walkable(c, false))
}

/// Walkable cells edge-adjacent to a rectangle (outside it).
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

/// Walkable cells whose centre is ≤ `r` m from a point.
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

fn near_spot(grid: &Grid, spot: IVec2) -> Vec<IVec2> {
    walkable_within(grid, cell_center(spot), 2.0)
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

/// The integers in the parentheses right after `name` in a notes text.
fn paren_after(text: &str, name: &str) -> Option<Vec<i32>> {
    let i = text.find(name)? + name.len();
    let rest = text[i..].trim_start();
    let rest = rest.strip_prefix('(')?;
    let end = rest.find(')')?;
    Some(parse_ints(&rest[..end])).filter(|v| !v.is_empty())
}

/// A level file as a raw TOML table (for the lists the typed `LevelData` does not keep:
/// `[[event_spot]]`, `[[garden]]`, `opens_at`).
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

fn raw_rect(t: &toml::Table, key: &str) -> Option<Rect> {
    let a = t.get(key)?.as_array()?;
    let v: Vec<i32> = a
        .iter()
        .filter_map(|x| x.as_integer())
        .map(|x| x as i32)
        .collect();
    (v.len() == 4).then(|| Rect::new(v[0], v[1], v[2], v[3]))
}

fn raw_str<'a>(t: &'a toml::Table, key: &str) -> Option<&'a str> {
    t.get(key).and_then(|v| v.as_str())
}

const LEVEL_FILES: [&str; 5] = [
    "assets/levels/level-1.toml",
    "assets/levels/level-2.toml",
    "assets/levels/level-3.toml",
    "assets/levels/night-1.toml",
    "assets/levels/night-2.toml",
];

/// Garden rects of every level file (`[[garden]] rect`).
fn garden_rects() -> Vec<(String, Rect)> {
    let mut out = Vec::new();
    for f in LEVEL_FILES {
        let t = raw(f);
        for g in raw_list(&t, "garden") {
            let id = raw_str(g, "id").unwrap_or("?").to_owned();
            out.push((id, raw_rect(g, "rect").expect("garden rect")));
        }
    }
    out
}

fn night_part(data: &LevelData) -> usize {
    data.part_index("night_1").expect("night_1 joined")
}

// ------------------------------------------------------------------ LAYOUT-N1-001

// LAYOUT-N1-001: level-1 + night-1 joined with the moon door open → every enclosure,
// building, landmark, info board, food box and hiding place of night_1 has a walkable cell
// reachable from the level-1 spawn next to it or inside it.
#[test]
fn layout_n1_001_everything_reachable_through_the_open_moon_door() {
    let level = joined_moon_open();
    let data = &level.data;
    let grid = level.grid();
    let k = night_part(data);
    let reach = flood_fill(grid, data.parts[0].spawn.cell(), false);
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
        if !reachable_near(grid, &reach, h.rect) {
            missing.push(h.id.clone());
        }
    }
    // 3 enclosures + 2 buildings + 4 landmarks + 3 boards + 6 food boxes (4 outside + 2
    // inside, Q-194 answered 2026-09-29) + 9 places
    assert_eq!(checked, 27, "things checked");
    assert!(
        missing.is_empty(),
        "not reachable from the level-1 spawn: {missing:?}"
    );
}

// ------------------------------------------------------------------ LAYOUT-N1-002

// LAYOUT-N1-002: moon door closed (day) → no cell of night_1 reachable from the level-1
// spawn; every border cell of night_1 except (−25, 29) and (−25, 30) is solid.
#[test]
fn layout_n1_002_sealed_by_day() {
    let level = Level::new(common::zoo_with_night());
    assert!(!level.is_barrier_open(MOON_DOOR));
    let data = &level.data;
    let grid = level.grid();
    let b = data.parts[night_part(data)].bounds;
    assert_eq!([b.x, b.z, b.w, b.d], [-72, 6, 48, 48], "night_1 bounds");
    let reach = flood_fill(grid, data.parts[0].spawn.cell(), false);
    let reached: Vec<IVec2> = b
        .cells()
        .filter(|&c| grid.index(c).is_some_and(|k| reach[k]))
        .collect();
    assert!(
        reached.is_empty(),
        "night_1 cells reachable by day: {reached:?}"
    );
    // also with gates allowed (the leading path)
    let reach_g = flood_fill(grid, data.parts[0].spawn.cell(), true);
    assert!(
        !b.cells().any(|c| grid.index(c).is_some_and(|k| reach_g[k])),
        "night_1 reachable through a gate"
    );
    let entry: BTreeSet<(i32, i32)> = [(-25, 29), (-25, 30)].into_iter().collect();
    let mut open = Vec::new();
    for c in b.cells().filter(|&c| is_border(b, c)) {
        if entry.contains(&(c.x, c.y)) {
            assert_eq!(
                grid.kind(c),
                CellKind::Walkable(Surface::Path),
                "entry cell {c}"
            );
        } else if grid.kind(c) != CellKind::Solid {
            open.push((c, grid.kind(c)));
        }
    }
    assert!(open.is_empty(), "non-solid border cells: {open:?}");
}

// ------------------------------------------------------------------ LAYOUT-N1-003

// LAYOUT-N1-003: no two solid elements share a cell, no path cell lies under a solid
// element; the bounds are disjoint from levels 1–3.
#[test]
fn layout_n1_003_no_overlaps_and_disjoint_bounds() {
    let data = common::night1();
    let overlaps = data.solid_overlaps();
    assert!(overlaps.is_empty(), "solid overlaps: {overlaps:?}");
    let grid = night_level().grid().clone();
    let mut covered = Vec::new();
    for e in data.elements_of(ElementType::Path) {
        for c in e.rect.cells() {
            if let Some(s) = grid.solid_element(c) {
                // the street runs under a closed barrier (LAYOUT-040: `barrier_n1_garden`)
                if data.elements[s].ty == ElementType::Barrier {
                    continue;
                }
                covered.push((e.id.clone(), data.elements[s].id.clone(), c));
            }
        }
    }
    assert!(covered.is_empty(), "path under solid: {covered:?}");
    let nb = data.level.bounds;
    for day in [common::level1(), common::level2(), common::level3()] {
        assert!(
            !rects_overlap(nb, day.level.bounds),
            "night_1 bounds overlap {}",
            day.level.id
        );
    }
}

// ------------------------------------------------------------------ LAYOUT-N1-004

// LAYOUT-N1-004 (LAYOUT-005): the spec's element table and night-1.toml list the same ids,
// types and rectangles (plus the gates, doors, interior and model_rect named in the notes).
#[test]
fn layout_n1_004_spec_table_matches_data() {
    let data = common::night1();
    let md = common::read(NIGHT_SPEC);
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
    assert!(diffs.is_empty(), "{diffs:#?}");
}

// ------------------------------------------------------------------ LAYOUT-N1-005

/// A named point set of the "Walking distances" table.
fn point(level: &Level, name: &str) -> Vec<IVec2> {
    let data = &level.data;
    let grid = level.grid();
    if let Some(h) = data.hiding_place(name) {
        return near_spot(grid, h.spot_cell());
    }
    let house = data.element("night_house").unwrap();
    let hall = house.interior.unwrap();
    let hall_in_front_of_gate = |enc: &str| -> Vec<IVec2> {
        let gate = data.element(enc).unwrap().gate.unwrap();
        walkable_adjacent(grid, gate)
            .into_iter()
            .filter(|c| hall.contains(*c))
            .collect()
    };
    match name {
        "moon door" => data.entries[0].cells.cells().collect(),
        "food boxes" => data
            .food_boxes
            .iter()
            .flat_map(|b| {
                walkable_within(grid, b.pos(), 2.0)
                    .into_iter()
                    .filter(|c| (cell_center(*c) - b.pos()).dot(b.facing()) > 0.0)
                    .collect::<Vec<_>>()
            })
            .collect(),
        "map board" => walkable_adjacent(grid, data.element("map_board_n1").unwrap().rect),
        "hedgehog board" => {
            walkable_adjacent(grid, data.element("board_n1_hedgehog").unwrap().rect)
        }
        "bat board" => walkable_adjacent(grid, data.element("board_n1_bat").unwrap().rect),
        "owl board" => walkable_adjacent(grid, data.element("board_n1_owl").unwrap().rect),
        "night-house door" => vec![house.door_cell().unwrap()],
        "hedgehog gate" => hall_in_front_of_gate("enc_n1_hedgehog"),
        "owl gate" => hall_in_front_of_gate("enc_n1_owl"),
        "telescope" => walkable_adjacent(grid, data.element("telescope_n1").unwrap().rect),
        other => panic!("unknown point {other}"),
    }
}

// LAYOUT-N1-005: with 1.93 m/s on paths and 1.45 m/s on grass, each pair of the "Walking
// distances" table is ≤ 10 s. The measured values are printed next to the spec's scratch
// estimates (drift is reported, not asserted: the spec calls them estimates).
#[test]
fn layout_n1_005_walking_times() {
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
        ("moon door", "food boxes", 7.0),
        ("moon door", "map board", 2.1),
        ("food boxes", "bat board", 3.1),
        ("bat board", "hedgehog board", 0.5),
        ("bat board", "owl board", 2.8),
        ("bat board", "night-house door", 0.5),
        ("night-house door", "hedgehog gate", 4.3),
        ("night-house door", "owl gate", 4.3),
        ("hedgehog board", "telescope", 4.1),
        ("telescope", "loc_fir", 7.9),
        ("loc_fir", "loc_hilltop", 5.3),
        ("loc_mushrooms", "loc_flowerpots", 6.1),
        ("loc_flowerpots", "loc_brush_pile", 6.1),
        ("loc_windmill", "loc_fireflies", 1.0),
        ("loc_fireflies", "loc_moon_pond", 7.9),
        ("loc_moon_pond", "loc_hollow_tree", 6.9),
        ("loc_moon_pond", "food boxes", 7.2),
    ];
    // "for information" legs of the spec (not required to be ≤ 10 s)
    let info: &[(&str, &str, f32)] = &[
        ("telescope", "loc_hilltop", 9.5),
        ("loc_hilltop", "loc_mushrooms", 9.7),
        ("loc_brush_pile", "loc_windmill", 9.6),
    ];
    let level = night_level();
    let grid = level.grid();
    let mut too_slow = Vec::new();
    let mut drift = Vec::new();
    for (from, to, spec) in table.iter().chain(info) {
        let a = point(&level, from);
        let b = point(&level, to);
        assert!(
            !a.is_empty() && !b.is_empty(),
            "{from} → {to}: empty point set"
        );
        let t = min_cost(grid, &a, &b, time_cost());
        println!("LAYOUT-N1-005: {from} → {to}: {t:.1} s (spec {spec})");
        if (t - spec).abs() > 0.5 {
            drift.push(format!("{from} → {to}: {t:.1} s vs spec {spec}"));
        }
        let required = table.iter().any(|(f, x, _)| f == from && x == to);
        if required && t > 10.0 + 1e-3 {
            too_slow.push(format!("{from} → {to}: {t:.1} s"));
        }
    }
    if !drift.is_empty() {
        println!("LAYOUT-N1-005: spec estimates off by > 0.5 s: {drift:#?}");
    }
    assert!(too_slow.is_empty(), "over 10 s: {too_slow:#?}");
}

// ------------------------------------------------------------------ LAYOUT-N1-006

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

/// Standing points of an animal (CAMV-008): walkable cells ≤ 2.5 m from its own info board,
/// and the walkable cells in front of its own gate.
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

// LAYOUT-N1-006: for hedgehog, bat, owl, every candidate, the player on every walkable cell
// ≤ 2.5 m from the own board or in front of the own gate, the zoo camera at every 45°
// rotation and 10 / 14 / 20 m on 1080×2340 → no wander cell centre (0.5 m, 1.0 m,
// perch + 1 m) is on screen, and every one is ≥ 22 m from the standing points (CAMV-008,
// Q-110; 22 m since the close-view haze grew by 30 %, FIX-056).
#[test]
fn layout_n1_006_sight_test_and_haze_margin() {
    let level = night_level();
    let mut seen = Vec::new();
    let mut close = Vec::new();
    let mut nearest = (f32::MAX, String::new());
    for animal in NIGHT_ANIMALS {
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
        "LAYOUT-N1-006: nearest wander cell {:.1} m — {}",
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

// ------------------------------------------------------------------ LAYOUT-N1-007

// LAYOUT-N1-007: each hiding place has ≥ 9 wander cells (the spec table's counts), a cell
// ≥ 2 m from the spot, no solid or path cell, lies inside its rect; places of different
// animals do not overlap; all 27 combinations keep the spots ≥ 12 m apart.
#[test]
fn layout_n1_007_wander_areas_and_spread() {
    let table: [(&str, usize); 9] = [
        ("loc_brush_pile", 26),
        ("loc_flowerpots", 25),
        ("loc_mushrooms", 26),
        ("loc_windmill", 13),
        ("loc_fireflies", 20),
        ("loc_hollow_tree", 11),
        ("loc_moon_pond", 25),
        ("loc_hilltop", 21),
        ("loc_fir", 22),
    ];
    let level = night_level();
    let data = &level.data;
    let grid = level.grid();
    let mut problems = Vec::new();
    let mut areas: Vec<(String, String, Rect, Vec<IVec2>)> = Vec::new();
    for animal in NIGHT_ANIMALS {
        let list: Vec<_> = data.hiding_places_of(animal).collect();
        if list.len() < 3 {
            problems.push(format!("{animal}: {} candidates", list.len()));
        }
        for h in list {
            let cells: Vec<IVec2> = hiding_area(&level, h).cells().map(|(c, _)| c).collect();
            let expect = table.iter().find(|(id, _)| *id == h.id).map(|x| x.1);
            if cells.len() < 9 {
                problems.push(format!("{}: {} wander cells < 9", h.id, cells.len()));
            }
            if expect != Some(cells.len()) {
                problems.push(format!(
                    "{}: {} wander cells, spec table {:?}",
                    h.id,
                    cells.len(),
                    expect
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
    assert_eq!(areas.len(), 9, "9 candidates");
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
    let per: Vec<Vec<_>> = NIGHT_ANIMALS
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
        "LAYOUT-N1-007: closest spots {:.1} m ({})",
        closest.0, closest.1
    );
    problems.sort();
    problems.dedup();
    assert!(problems.is_empty(), "{problems:#?}");
}

// ------------------------------------------------------------------ LAYOUT-N1-008

// LAYOUT-N1-008: features contain the CONT-MISSIONS details (the spec's "Features" column),
// every scenery id exists, each scenery kind and each riddle element kind occurs once in the
// night levels, firefly props appear only in firefly_meadow_n1; each place's features exist
// only there within the night levels (behaviour 8).
#[test]
fn layout_n1_008_features_scenery_and_unique_kinds() {
    let zoo = common::zoo_with_night();
    let night: BTreeSet<usize> = (0..zoo.parts.len())
        .filter(|&k| zoo.is_night_part(k))
        .collect();
    let expect: &[(&str, &[&str])] = &[
        (
            "loc_brush_pile",
            &["brush_pile", "twigs", "rustling", "hedge_corner"],
        ),
        (
            "loc_flowerpots",
            &[
                "flower_pots",
                "potting_bench",
                "night_flowers",
                "sweet_smell",
            ],
        ),
        (
            "loc_mushrooms",
            &["mushrooms", "moss", "earthy_smell", "old_tree"],
        ),
        ("loc_windmill", &["windmill", "turning_sails", "whirring"]),
        (
            "loc_fireflies",
            &["fireflies", "little_lights", "crooked_tree", "low_grass"],
        ),
        (
            "loc_hollow_tree",
            &["hollow_tree", "knot_hole", "thick_old_trunk"],
        ),
        (
            "loc_moon_pond",
            &["pond", "moon_reflection", "reeds", "wooden_post"],
        ),
        (
            "loc_hilltop",
            &["hill", "big_stone", "moonlight", "no_trees"],
        ),
        ("loc_fir", &["fir_tree", "pointed_top", "cones", "needles"]),
    ];
    let mut problems = Vec::new();
    for (id, feats) in expect {
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
    }
    // each place's features only there (within the night levels)
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
    // kinds once in the night levels
    let night_scenery: Vec<_> = zoo
        .scenery
        .iter()
        .filter(|s| {
            zoo.part_at(IVec2::new(s.rect.x, s.rect.z))
                .is_some_and(|k| night.contains(&k))
        })
        .collect();
    let count = |k: &str| {
        zoo.elements
            .iter()
            .filter(|e| night.contains(&e.part) && e.kind.as_deref() == Some(k))
            .count()
            + night_scenery.iter().filter(|s| s.kind == k).count()
    };
    let mut kinds: BTreeSet<String> = night_scenery.iter().map(|s| s.kind.clone()).collect();
    kinds.extend(
        [
            "windmill",
            "tree_hollow",
            "fir_tree",
            "hill",
            "pond",
            "potting_bench",
            "tree_crooked",
        ]
        .map(str::to_owned),
    );
    for k in &kinds {
        let n = count(k);
        if n != 1 {
            problems.push(format!("kind {k} occurs {n} times in the night levels"));
        }
    }
    // fireflies only in firefly_meadow_n1 (Q-115)
    for s in &night_scenery {
        if s.props.iter().any(|p| p.contains("firefly")) && s.id != "firefly_meadow_n1" {
            problems.push(format!("firefly props in {}", s.id));
        }
    }
    if !night_scenery
        .iter()
        .any(|s| s.id == "firefly_meadow_n1" && s.props.iter().any(|p| p == "firefly"))
    {
        problems.push("firefly_meadow_n1 has no firefly props".into());
    }
    for p in zoo.props.iter().filter(|p| night.contains(&p.part)) {
        if p.model.contains("firefly") {
            problems.push(format!("firefly [[prop]] {}", p.id));
        }
    }
    for e in zoo.elements.iter().filter(|e| night.contains(&e.part)) {
        if e.kind.as_deref().is_some_and(|k| k.contains("firefly")) {
            problems.push(format!("firefly element {}", e.id));
        }
    }
    assert!(problems.is_empty(), "{problems:#?}");
}

// ------------------------------------------------------------------ LAYOUT-N1-009

// LAYOUT-N1-009: the night house's interior and door are walkable with surface `path`, each
// indoor enclosure has its gate edge-adjacent to an interior cell, and its `model_rect`
// contains the hall and all three enclosures; the three boards stand outside the house
// (behaviour 5).
#[test]
fn layout_n1_009_night_house() {
    let level = night_level();
    let data = &level.data;
    let grid = level.grid();
    let house = data.element("night_house").expect("night_house");
    assert_eq!(house.ty, ElementType::Building);
    let hall = house.interior.expect("interior");
    let door = house.door_cell().expect("door");
    for c in hall.cells().chain([door]) {
        assert_eq!(
            grid.kind(c),
            CellKind::Walkable(Surface::Path),
            "hall/door {c}"
        );
    }
    let model = house.model_rect.expect("model_rect");
    assert!(
        hall.cells().all(|c| model.contains(c)),
        "hall outside model_rect"
    );
    let indoor: Vec<_> = data
        .elements
        .iter()
        .filter(|e| e.ty == ElementType::Enclosure && e.indoor)
        .collect();
    let mut animals: Vec<&str> = indoor.iter().filter_map(|e| e.animal.as_deref()).collect();
    animals.sort_unstable();
    assert_eq!(animals, ["bat", "hedgehog", "owl"], "indoor enclosures");
    for e in indoor {
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
            .any(|c| hall.contains(c));
        assert!(to_hall, "{}: gate not next to the hall", e.id);
        assert!(
            e.rect.cells().all(|c| model.contains(c)),
            "{} outside model_rect",
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

// ------------------------------------------------------------------ LAYOUT-N1-010

// LAYOUT-N1-010: the [[food_box]] list has every night food (beetles, fruit, worms, nectar)
// outside, plus 2-6 more real, labelled food boxes inside the hut that may repeat a food
// (Q-194 answered 2026-09-29); every box has a walkable cell centre within 2 m in front of it
// reachable from the entry.
#[test]
fn layout_n1_010_night_food_boxes() {
    let level = night_level();
    let data = &level.data;
    let grid = level.grid();
    let mut foods: Vec<&str> = data.food_boxes.iter().map(|b| b.food.as_str()).collect();
    foods.sort_unstable();
    foods.dedup();
    assert_eq!(foods, ["beetles", "fruit", "nectar", "worms"]);
    let hut = data.element("food_storage_n1").unwrap().rect;
    let inside = data
        .food_boxes
        .iter()
        .filter(|b| hut.contains(zoo_core::level::cell_of(b.pos())))
        .count();
    assert!((2..=6).contains(&inside), "{inside} inside food boxes");
    let reach = flood_fill(grid, entry_cell(), false);
    for b in &data.food_boxes {
        let ok = walkable_within(grid, b.pos(), 2.0).into_iter().any(|c| {
            grid.is_passable(c, false)
                && (cell_center(c) - b.pos()).dot(b.facing()) > 0.0
                && reach[grid.index(c).unwrap()]
        });
        assert!(ok, "{}: no reachable standing cell in front", b.food);
    }
}

// ------------------------------------------------------------------ LAYOUT-N1-011

// LAYOUT-N1-011: every lantern post / string end is on a walkable cell outside hiding-place
// rects and scenery, every board and the map board has exactly one board_lamp, every attach
// id exists, and the night house has an `indoor` light per enclosure with the Q-116 colours.
#[test]
fn layout_n1_011_night_lights() {
    let level = night_level();
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
    let house = data.element("night_house").unwrap();
    let model = house.model_rect.unwrap();
    for (enc, color) in [
        ("enc_n1_hedgehog", "#E8735A"),
        ("enc_n1_bat", "#5B7FE0"),
        ("enc_n1_owl", "#5B7FE0"),
    ] {
        let lights: Vec<_> = data
            .lights
            .iter()
            .filter(|l| l.kind == "indoor" && l.attach.as_deref() == Some(enc))
            .collect();
        if lights.len() != 1 {
            problems.push(format!("{enc}: {} indoor lights", lights.len()));
        }
        for l in lights {
            if !l
                .color
                .as_deref()
                .is_some_and(|c| c.eq_ignore_ascii_case(color))
            {
                problems.push(format!("{}: colour {:?}, Q-116 {color}", l.id, l.color));
            }
            if !l.pos().is_some_and(|p| model.contains(cell_of(p))) {
                problems.push(format!("{}: not inside the night house", l.id));
            }
        }
    }
    assert!(problems.is_empty(), "{problems:#?}");
}

// ------------------------------------------------------------------ LAYOUT-027

// LAYOUT-027: every level's [[light]] list — lantern posts and string-light ends on a
// walkable cell outside every hiding-place rect, scenery rect and garden and ≥ 0.7 m from
// every food box; every info board and map board has exactly one board_lamp; every outdoor
// enclosure gate has a lantern post within 3 m of the gate centre (indoor enclosures: an
// `indoor` light attached to them); every attach id exists (Q-118, Q-137).
#[test]
fn layout_027_light_placement_all_levels() {
    let level = Level::new(common::zoo_with_night());
    let data = &level.data;
    let grid = level.grid();
    let gardens = garden_rects();
    assert!(!gardens.is_empty(), "garden_veg is listed");
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
        let pts = l.points();
        if l.kind == "string_lights" && pts.len() < 2 {
            problems.push(format!("{}: string lights without two ends", l.id));
        }
        for p in pts {
            let c = cell_of(p);
            if !grid.is_walkable(c, false) {
                problems.push(format!(
                    "{}: {p} on non-walkable cell {c} ({:?})",
                    l.id,
                    grid.kind(c)
                ));
            }
            for h in data.hiding_places.iter().filter(|h| h.rect.contains(c)) {
                problems.push(format!("{}: {p} inside hiding place {}", l.id, h.id));
            }
            for s in data.scenery.iter().filter(|s| s.rect.contains(c)) {
                problems.push(format!("{}: {p} inside scenery {}", l.id, s.id));
            }
            for (g, _) in gardens.iter().filter(|(_, r)| r.contains(c)) {
                problems.push(format!("{}: {p} inside garden {g}", l.id));
            }
            for b in &data.food_boxes {
                let d = b.pos().distance(p);
                if d < 0.7 {
                    problems.push(format!("{}: {p} {d:.2} m from food box {}", l.id, b.food));
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
    for e in data.elements_of(ElementType::Enclosure) {
        if e.indoor {
            let n = data
                .lights
                .iter()
                .filter(|l| l.kind == "indoor" && l.attach.as_deref() == Some(&e.id))
                .count();
            if n == 0 {
                problems.push(format!(
                    "{}: indoor enclosure without an indoor light",
                    e.id
                ));
            }
        } else {
            let Some(gate) = e.gate else {
                problems.push(format!("{}: no gate", e.id));
                continue;
            };
            let gc = rect_center(gate);
            let near = data
                .lights
                .iter()
                .filter(|l| l.kind == "lantern_post")
                .filter_map(|l| l.pos())
                .map(|p| p.distance(gc))
                .fold(f32::INFINITY, f32::min);
            if near > 3.0 {
                problems.push(format!(
                    "{}: nearest lantern post {near:.2} m from the gate",
                    e.id
                ));
            }
        }
    }
    assert!(problems.is_empty(), "{problems:#?}");
}

// ------------------------------------------------------------------ LAYOUT-028

fn run(g: &mut Game, seconds: f32) {
    let dt = 1.0 / 60.0;
    for _ in 0..(seconds / dt).round() as usize {
        g.update(dt, Vec2::ZERO);
        g.drain_events();
    }
}

fn door_walkable(g: &Game) -> bool {
    let r = g.level.data.element(MOON_DOOR).unwrap().rect;
    r.cells().all(|c| g.level.grid().is_walkable(c, false))
}

fn door_closed(g: &Game) -> bool {
    let r = g.level.data.element(MOON_DOOR).unwrap().rect;
    !g.level.is_barrier_open(MOON_DOOR)
        && r.cells().all(|c| g.level.grid().kind(c) == CellKind::Solid)
}

/// "Never removed": the element stays, and its frame (pillars, sign) stays in the scene.
fn door_still_there(g: &Game) {
    let e = g
        .level
        .data
        .element(MOON_DOOR)
        .expect("moon_door never removed");
    assert_eq!(e.kind.as_deref(), Some("moon_door"));
    assert_eq!(g.moon_doors(), [MOON_DOOR]);
}

// LAYOUT-028: a moon door is closed by day and before its `unlock_after` level's nightfall,
// open at night afterwards (every night), closed again the next morning, never removed; it
// is not opened by the Q-091 exit-barrier rule (Q-133).
#[test]
fn layout_028_moon_door_opens_every_night_after_nightfall() {
    // the data: kind moon_door, opens_at = "night", unlock_after = "level_1"
    let l1 = raw("assets/levels/level-1.toml");
    let raw_door = raw_list(&l1, "element")
        .into_iter()
        .find(|e| raw_str(e, "id") == Some(MOON_DOOR))
        .expect("moon_door in level-1.toml");
    assert_eq!(raw_str(raw_door, "kind"), Some("moon_door"));
    assert_eq!(raw_str(raw_door, "opens_at"), Some("night"));
    assert_eq!(raw_str(raw_door, "unlock_after"), Some("level_1"));
    assert_eq!(raw_str(raw_door, "transition"), Some("level_1->night_1"));

    // not an exit barrier of the Q-091 rule, for any level
    let g = common::night_game(28);
    for part in &g.level.data.parts {
        assert!(
            !g.level
                .exit_barriers_of(&part.id)
                .contains(&MOON_DOOR.to_owned()),
            "moon door is an exit of {}",
            part.id
        );
        assert!(
            !g.level
                .barriers_unlocked_by(&part.id)
                .contains(&MOON_DOOR.to_owned()),
            "moon door unlocked by completing {}",
            part.id
        );
    }

    // by day, before any nightfall: closed
    let mut g = common::night_game(28);
    assert_eq!(g.daytime.phase, Phase::Day);
    assert!(door_closed(&g), "closed by day");
    run(&mut g, 5.0);
    assert!(door_closed(&g));

    // level 1 complete: still day during the celebration, dusk → closed; night → open
    for a in ["zebra", "hippo", "panda"] {
        assert!(g.debug_send_home(a));
    }
    assert!(door_closed(&g), "closed right after the last mission (day)");
    run(&mut g, CELEBRATION_S + DUSK_S * 0.5);
    assert_eq!(g.daytime.phase, Phase::Dusk);
    assert!(door_closed(&g), "closed at dusk");
    run(&mut g, DUSK_S * 0.5 + 0.5);
    assert_eq!(g.daytime.phase, Phase::Night);
    assert!(
        g.level.is_barrier_open(MOON_DOOR) && door_walkable(&g),
        "open at night"
    );
    door_still_there(&g);

    // the next morning: closed again (the Q-091 exits of level 1 open, the moon door does not)
    g.debug_next_morning();
    assert_eq!(g.daytime.phase, Phase::Day);
    assert!(door_closed(&g), "closed the next morning");
    door_still_there(&g);

    // the next night (the night zoo is waiting: sleep until the evening): open again
    assert!(g.daytime.sleep_until_evening());
    run(&mut g, SLEEP_S + DUSK_S + 1.0);
    assert_eq!(g.daytime.phase, Phase::Night, "the second night");
    assert!(g.level.is_barrier_open(MOON_DOOR), "open every night");
    g.debug_next_morning();
    assert!(door_closed(&g), "closed the second morning");

    // the night zoo complete: its exit opens the next morning; the moon door still opens
    // every night and closes every morning, never removed
    assert!(g.daytime.sleep_until_evening());
    run(&mut g, SLEEP_S + DUSK_S + 1.0);
    assert!(g.level.is_barrier_open(MOON_DOOR));
    for a in ["hedgehog", "bat", "owl"] {
        assert!(g.debug_send_home(a));
    }
    g.debug_next_morning();
    assert!(
        door_closed(&g),
        "closed after the night zoo is complete (morning)"
    );
    door_still_there(&g);

    // another level's nightfall (level 2 complete before level 1): night, but the moon door
    // stays closed — its `unlock_after` level had no nightfall yet — and the Q-091 morning
    // (level-2 exits) does not open it either
    let mut g = common::night_game(29);
    for a in ["koala", "elephant", "giraffe", "lion"] {
        assert!(g.debug_send_home(a), "{a}");
    }
    run(&mut g, CELEBRATION_S + DUSK_S + 0.5);
    assert_eq!(g.daytime.phase, Phase::Night, "level 2's nightfall");
    assert!(door_closed(&g), "closed before level 1's nightfall");
    g.debug_set_daytime("sleeping");
    run(&mut g, SLEEP_S + MORNING_S + 0.5);
    assert_eq!(g.daytime.phase, Phase::Day);
    assert!(
        door_closed(&g),
        "the Q-091 morning does not open the moon door"
    );
    door_still_there(&g);
}

// ------------------------------------------------------------------ LAYOUT-029

// LAYOUT-029: for every building with indoor enclosures (`indoor = true`): each indoor
// enclosure's gate is edge-adjacent to an `interior` cell of the building, the building's
// `model_rect` contains the building rect and the indoor enclosures, and none of the
// enclosure's boards lies inside the building rect (Q-134).
#[test]
fn layout_029_buildings_with_indoor_enclosures() {
    let data = common::zoo_with_night();
    let mut problems = Vec::new();
    let mut houses: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let indoor: Vec<_> = data
        .elements
        .iter()
        .filter(|e| e.ty == ElementType::Enclosure && e.indoor)
        .collect();
    assert!(!indoor.is_empty(), "no indoor enclosure in the data");
    for enc in indoor {
        let Some(gate) = enc.gate else {
            problems.push(format!("{}: no gate", enc.id));
            continue;
        };
        let next: Vec<IVec2> = gate
            .cells()
            .flat_map(|c| {
                [
                    c + IVec2::X,
                    c + IVec2::NEG_X,
                    c + IVec2::Y,
                    c + IVec2::NEG_Y,
                ]
            })
            .collect();
        let owners: Vec<_> = data
            .elements_of(ElementType::Building)
            .filter(|b| {
                b.interior
                    .is_some_and(|i| next.iter().any(|c| i.contains(*c)))
            })
            .collect();
        if owners.len() != 1 {
            problems.push(format!(
                "{}: gate next to the interior of {} buildings",
                enc.id,
                owners.len()
            ));
            continue;
        }
        let b = owners[0];
        houses.entry(b.id.clone()).or_default().push(enc.id.clone());
        let Some(model) = b.model_rect else {
            problems.push(format!("{}: no model_rect", b.id));
            continue;
        };
        if !b.rect.cells().all(|c| model.contains(c)) {
            problems.push(format!(
                "{}: model_rect does not contain the building rect",
                b.id
            ));
        }
        if !enc.rect.cells().all(|c| model.contains(c)) {
            problems.push(format!("{}: model_rect does not contain {}", b.id, enc.id));
        }
        for board in data.elements.iter().filter(|x| {
            x.kind.as_deref() == Some("info_board") && x.enclosure.as_deref() == Some(&enc.id)
        }) {
            if rects_overlap(board.rect, b.rect) {
                problems.push(format!("{} inside the building {}", board.id, b.id));
            }
        }
    }
    println!("LAYOUT-029: {houses:?}");
    assert!(problems.is_empty(), "{problems:#?}");
}

// ------------------------------------------------------------------ LAYOUT-030

// LAYOUT-030: every day level's [[event_spot]] list has one burglar_entry, one burglar_target
// naming an existing building and one burglar_hideout of walkable grass reachable from the
// target, overlapping no hiding-place rect, scenery, garden or path (Q-139).
#[test]
fn layout_030_burglar_event_spots() {
    let level = Level::new(common::zoo_with_night());
    let data = &level.data;
    let grid = level.grid();
    let gardens = garden_rects();
    let mut problems = Vec::new();
    for (file, level_id) in [
        ("assets/levels/level-1.toml", "level_1"),
        ("assets/levels/level-2.toml", "level_2"),
        ("assets/levels/level-3.toml", "level_3"),
    ] {
        let t = raw(file);
        let spots = raw_list(&t, "event_spot");
        let of = |k: &str| -> Vec<&toml::Table> {
            spots
                .iter()
                .copied()
                .filter(|s| raw_str(s, "kind") == Some(k))
                .collect()
        };
        let (entry, target, hideout) = (
            of("burglar_entry"),
            of("burglar_target"),
            of("burglar_hideout"),
        );
        for (k, list) in [
            ("burglar_entry", &entry),
            ("burglar_target", &target),
            ("burglar_hideout", &hideout),
        ] {
            if list.len() != 1 {
                problems.push(format!("{level_id}: {} {k}", list.len()));
            }
        }
        let part = data.part_index(level_id).unwrap();
        let Some(target) = target.first() else {
            continue;
        };
        let Some(building) = raw_str(target, "target").and_then(|id| data.element(id)) else {
            problems.push(format!(
                "{level_id}: burglar_target names no existing element"
            ));
            continue;
        };
        if building.ty != ElementType::Building || building.part != part {
            problems.push(format!(
                "{level_id}: target {} is a {} of part {}",
                building.id,
                building.ty.as_str(),
                building.part
            ));
        }
        let Some(hideout) = hideout.first().and_then(|h| raw_rect(h, "rect")) else {
            problems.push(format!("{level_id}: burglar_hideout without rect"));
            continue;
        };
        let mut reach = vec![false; grid.len()];
        for s in walkable_adjacent(grid, building.rect) {
            for (k, r) in flood_fill(grid, s, false).into_iter().enumerate() {
                reach[k] |= r;
            }
        }
        for c in hideout.cells() {
            if grid.kind(c) != CellKind::Walkable(Surface::Grass) {
                problems.push(format!(
                    "{level_id}: hideout cell {c} is {:?}",
                    grid.kind(c)
                ));
            } else if !grid.index(c).is_some_and(|k| reach[k]) {
                problems.push(format!(
                    "{level_id}: hideout cell {c} not reachable from {}",
                    building.id
                ));
            }
            if grid.has_path(c) {
                problems.push(format!("{level_id}: hideout cell {c} on a path"));
            }
        }
        for h in data
            .hiding_places
            .iter()
            .filter(|h| rects_overlap(h.rect, hideout))
        {
            problems.push(format!(
                "{level_id}: hideout overlaps hiding place {}",
                h.id
            ));
        }
        for s in data
            .scenery
            .iter()
            .filter(|s| rects_overlap(s.rect, hideout))
        {
            problems.push(format!("{level_id}: hideout overlaps scenery {}", s.id));
        }
        for (g, _) in gardens.iter().filter(|(_, r)| rects_overlap(*r, hideout)) {
            problems.push(format!("{level_id}: hideout overlaps garden {g}"));
        }
    }
    assert!(problems.is_empty(), "{problems:#?}");
}

// ------------------------------------------------------------------ LAYOUT-N1-015…017
// The secret trail through the middle grove (Q-365).

const OLD_GROVE: (i32, i32, i32, i32) = (-59, 18, 13, 15);

fn trail_cells(data: &LevelData) -> Vec<IVec2> {
    let cells: Vec<IVec2> = data
        .elements_of(ElementType::Path)
        .filter(|e| e.id.starts_with("path_n1_secret_"))
        .flat_map(|e| e.rect.cells())
        .collect();
    assert!(!cells.is_empty(), "no path_n1_secret_* elements");
    cells
}

// LAYOUT-N1-015: trail cells are walkable path cells inside the old grove rect, free of
// solid elements, colliders, hiding places, wander areas and boards; the grove parts are dense
// and view-blocking and cover (with the trail) exactly the old rect; both ends join the rings.
#[test]
fn layout_n1_015_secret_trail_cells() {
    let level = night_level();
    let data = &level.data;
    let grid = level.grid();
    let trail = trail_cells(data);
    let old = Rect::new(OLD_GROVE.0, OLD_GROVE.1, OLD_GROVE.2, OLD_GROVE.3);
    let set: BTreeSet<(i32, i32)> = trail.iter().map(|c| (c.x, c.y)).collect();
    assert_eq!(set.len(), trail.len(), "trail rects overlap");
    let mut problems = Vec::new();
    for &c in &trail {
        if !old.contains(c) {
            problems.push(format!("{c} outside the old grove"));
        }
        if !grid.is_walkable(c, false) || grid.surface(c) != Some(Surface::Path) {
            problems.push(format!("{c} not a walkable path cell"));
        }
        if grid.solid_element(c).is_some() {
            problems.push(format!("{c} under a solid element"));
        }
        let p = cell_center(c);
        if level
            .colliders()
            .shapes()
            .iter()
            .any(|s| s.push_out(p, 0.3) != Vec2::ZERO)
        {
            problems.push(format!("{c} touches a collider"));
        }
        for h in &data.hiding_places {
            if h.rect.contains(c) {
                problems.push(format!("{c} inside hiding place {}", h.id));
            }
        }
    }
    for h in &data.hiding_places {
        for (w, _) in hiding_area(&level, h).cells() {
            if set.contains(&(w.x, w.y)) {
                problems.push(format!("{} wander cell {w} on the trail", h.id));
            }
        }
    }
    // the grove parts + the trail tile the old rect exactly
    let parts: Vec<&zoo_core::level::Element> = data
        .elements
        .iter()
        .filter(|e| e.id.starts_with("grove_n1_center_"))
        .collect();
    assert!(parts.len() >= 2);
    for e in &parts {
        assert_eq!(e.density.as_deref(), Some("dense"), "{}", e.id);
        assert!(e.blocks_view, "{} must block the view", e.id);
    }
    for c in old.cells() {
        let n = parts.iter().filter(|e| e.rect.contains(c)).count()
            + usize::from(set.contains(&(c.x, c.y)));
        if n != 1 {
            problems.push(format!("{c} covered {n} times"));
        }
    }
    // ends: entrance top (z 32 → grass 33, 34 → ring_n at z 35), exit bottom (z 18 → 17, 16 → 15)
    for (x, zs) in [
        (-54, [33, 34, 35]),
        (-53, [33, 34, 35]),
        (-50, [17, 16, 15]),
        (-49, [17, 16, 15]),
    ] {
        for z in zs {
            if !grid.is_walkable(IVec2::new(x, z), false) {
                problems.push(format!("connection cell ({x}, {z}) not walkable"));
            }
        }
    }
    for (x, z) in [(-54, 35), (-53, 35), (-50, 15), (-49, 15)] {
        if grid.surface(IVec2::new(x, z)) != Some(Surface::Path) {
            problems.push(format!("ring cell ({x}, {z}) is not a path"));
        }
    }
    // the entry of the level reaches the trail
    let reach = flood_fill(grid, entry_cell(), false);
    for &c in &trail {
        if !reach[grid.index(c).unwrap()] {
            problems.push(format!("{c} not reachable from the entry"));
        }
    }
    assert!(problems.is_empty(), "{problems:#?}");
}

// LAYOUT-N1-016: never required. With every trail cell solid the whole walkable level is still
// reachable from the entry; with the trail the ring-north → ring-south walk is ≥ 3 s shorter.
#[test]
fn layout_n1_016_secret_trail_never_required() {
    let open = night_level();
    let trail = trail_cells(&open.data);
    let mut closed_data = open.data.clone();
    for e in closed_data
        .elements
        .iter_mut()
        .filter(|e| e.id.starts_with("path_n1_secret_"))
    {
        e.ty = ElementType::Decoration;
        e.kind = Some("tree_grove".into());
        e.density = Some("dense".into());
    }
    let closed = Level::new(closed_data);
    for &c in &trail {
        assert!(!closed.grid().is_walkable(c, false), "{c} still walkable");
    }
    let r_open = flood_fill(open.grid(), entry_cell(), false);
    let r_closed = flood_fill(closed.grid(), entry_cell(), false);
    let tset: BTreeSet<(i32, i32)> = trail.iter().map(|c| (c.x, c.y)).collect();
    let mut lost = Vec::new();
    for c in open.data.level.bounds.cells() {
        let k = open.grid().index(c).unwrap();
        if r_open[k] && !tset.contains(&(c.x, c.y)) && !r_closed[k] {
            lost.push(c);
        }
    }
    assert!(lost.is_empty(), "cells cut off without the trail: {lost:?}");
    let a = [IVec2::new(-53, 35)];
    let b = [IVec2::new(-50, 15)];
    let with = min_cost(open.grid(), &a, &b, time_cost());
    let without = min_cost(closed.grid(), &a, &b, time_cost());
    println!(
        "LAYOUT-N1-016: ring north → ring south {with:.1} s with the trail, {without:.1} s without"
    );
    assert!(without - with >= 3.0, "{with} vs {without}");
}

// LAYOUT-N1-017: the trail is 2 cells wide everywhere (collision-safe between the bush
// borders) and keeps ≥ 4 m from every wander cell and ≥ 8 m from boards, gates and food boxes.
#[test]
fn layout_n1_017_secret_trail_width_and_distance() {
    let level = night_level();
    let data = &level.data;
    let trail = trail_cells(data);
    let set: BTreeSet<(i32, i32)> = trail.iter().map(|c| (c.x, c.y)).collect();
    let has = |x: i32, z: i32| set.contains(&(x, z));
    for &c in &trail {
        let in_block = [(0, 0), (-1, 0), (0, -1), (-1, -1)].iter().any(|(dx, dz)| {
            let (x, z) = (c.x + dx, c.y + dz);
            has(x, z) && has(x + 1, z) && has(x, z + 1) && has(x + 1, z + 1)
        });
        assert!(
            in_block,
            "{c} is not part of a 2 × 2 block (trail too narrow)"
        );
    }
    let mut avoid: Vec<(String, Vec2)> = Vec::new();
    for h in &data.hiding_places {
        for (w, _) in hiding_area(&level, h).cells() {
            avoid.push((format!("wander cell of {}", h.id), cell_center(w)));
        }
    }
    let mut far: Vec<(String, Vec2)> = Vec::new();
    for e in &data.elements {
        if e.kind.as_deref() == Some("info_board") {
            far.push((e.id.clone(), rect_center(e.rect)));
        }
        if let Some(g) = e.gate {
            far.push((format!("gate of {}", e.id), rect_center(g)));
        }
    }
    for b in &data.food_boxes {
        far.push(("food box".into(), b.pos()));
    }
    let mut bad = Vec::new();
    for &c in &trail {
        let p = cell_center(c);
        for (n, q) in &avoid {
            if p.distance(*q) < 4.0 {
                bad.push(format!("{c} < 4 m from {n}"));
            }
        }
        for (n, q) in &far {
            if p.distance(*q) < 8.0 {
                bad.push(format!("{c} < 8 m from {n}"));
            }
        }
    }
    assert!(bad.is_empty(), "{:#?}", &bad[..bad.len().min(20)]);
}
