//! GAME-HOUSE (HOUSE-001…017, HOUSE-020): animal houses with door-only entry on levels 1–3.

mod common;

use std::collections::{BTreeSet, VecDeque};

use glam::{IVec2, Vec2};
use zoo_core::house::{self, House};
use zoo_core::level::{cell_center, cell_of, ElementType, Level, LevelData, Rect};
use zoo_core::wander::{home_area, AreaCell, WanderArea};
use zoo_core::{AnimalState, Game};

const DT: f32 = 1.0 / 60.0;

/// The seven houses of the spec table: id, enclosure, model, footprint, interior, door,
/// door side.
type Row = (
    &'static str,
    &'static str,
    &'static str,
    [i32; 4],
    [i32; 4],
    [i32; 4],
    &'static str,
);
const TABLE: [Row; 7] = [
    (
        "zebra_shelter",
        "enc_zebra",
        "stone_arch_shelter",
        [-20, 15, 5, 4],
        [-19, 16, 3, 2],
        [-19, 15, 2, 1],
        "-z",
    ),
    (
        "panda_shelter",
        "enc_panda",
        "panda_shelter",
        [-6, 37, 5, 5],
        [-5, 38, 3, 3],
        [-2, 38, 1, 2],
        "+x",
    ),
    (
        "hippo_hut",
        "enc_hippo",
        "hut_wood",
        [14, 11, 5, 4],
        [15, 12, 3, 2],
        [14, 12, 1, 2],
        "-x",
    ),
    (
        "koala_shelter",
        "enc_koala",
        "koala_shelter",
        [27, 39, 4, 4],
        [28, 40, 2, 2],
        [30, 40, 1, 2],
        "+x",
    ),
    (
        "elephant_house",
        "enc_elephant",
        "elephant_house",
        [55, 28, 4, 4],
        [56, 29, 2, 2],
        [58, 29, 1, 2],
        "+x",
    ),
    (
        "giraffe_house",
        "enc_giraffe",
        "giraffe_house",
        [41, 50, 6, 6],
        [42, 51, 4, 4],
        [43, 50, 2, 1],
        "-z",
    ),
    (
        "monkey_house",
        "enc_monkey",
        "monkey_house",
        [-4, 87, 5, 5],
        [-3, 88, 3, 3],
        [0, 88, 1, 2],
        "+x",
    ),
];

/// Game size (m) of the housed species (ART-ANIMALS "Game sizes").
fn species_size(enclosure: &str) -> f32 {
    match enclosure {
        "enc_zebra" => 2.23,
        "enc_hippo" => 1.7,
        "enc_panda" => 1.1,
        "enc_koala" => 0.9,
        "enc_elephant" => 3.0,
        "enc_giraffe" => 4.5,
        "enc_monkey" => 1.1,
        _ => panic!("no house species {enclosure}"),
    }
}

fn zoo() -> LevelData {
    static DATA: std::sync::OnceLock<LevelData> = std::sync::OnceLock::new();
    DATA.get_or_init(common::zoo).clone()
}

fn zoo_level() -> Level {
    Level::new(zoo())
}

fn rect_overlap(a: Rect, b: Rect) -> bool {
    a.x < b.x + b.w && b.x < a.x + a.w && a.z < b.z + b.d && b.z < a.z + a.d
}

fn rect_within(inner: Rect, outer: Rect) -> bool {
    inner.x >= outer.x
        && inner.z >= outer.z
        && inner.x + inner.w <= outer.x + outer.w
        && inner.z + inner.d <= outer.z + outer.d
}

fn manhattan(a: IVec2, b: IVec2) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn enc_index(level: &Level, id: &str) -> usize {
    level
        .data
        .elements
        .iter()
        .position(|e| e.id == id)
        .unwrap_or_else(|| panic!("no element {id}"))
}

/// The zoo game with every barrier open and the player far from all animals.
fn open_zoo_game(seed: u64) -> Game {
    let mut g = Game::new(common::zoo_with_night(), seed).unwrap();
    let barriers: Vec<String> = g
        .level
        .data
        .elements
        .iter()
        .filter(|e| e.ty == ElementType::Barrier && e.id != "moon_door")
        .map(|e| e.id.clone())
        .collect();
    for b in &barriers {
        g.level.open_barrier(b);
    }
    g.player.pos = cell_center(g.level.data.spawn.cell());
    g
}

fn run(g: &mut Game, seconds: f32) {
    for _ in 0..(seconds / DT).round() as usize {
        g.update(DT, Vec2::ZERO);
    }
}

fn housed_ids() -> Vec<&'static str> {
    vec![
        "zebra", "hippo", "panda", "koala", "elephant", "giraffe", "monkey",
    ]
}

// HOUSE-001
#[test]
fn house_001_geometry_of_every_house() {
    let data = zoo();
    let level = zoo_level();
    let grid = level.grid();
    let houses = house::houses(&data);
    assert_eq!(houses.len(), 7);
    for h in &houses {
        let enc = data.element(&h.enclosure).expect("enclosure");
        let f = h.footprint;
        assert!(
            rect_within(f, enc.rect),
            "{}: footprint inside the enclosure",
            h.id
        );
        assert!(
            rect_within(h.interior, f),
            "{}: interior inside the footprint",
            h.id
        );
        assert!(
            rect_within(h.door, f),
            "{}: door inside the footprint",
            h.id
        );
        assert!(
            !rect_overlap(h.interior, h.door),
            "{}: interior and door disjoint",
            h.id
        );
        assert!(
            h.interior.w >= 2 && h.interior.d >= 2,
            "{}: interior >= 2 x 2",
            h.id
        );
        // door: 1 deep, 2 or 3 wide, on the named edge
        let (across, depth) = match h.door_side {
            house::DoorSide::NegX | house::DoorSide::PosX => (h.door.d, h.door.w),
            _ => (h.door.w, h.door.d),
        };
        assert_eq!(depth, 1, "{}", h.id);
        assert!((2..=3).contains(&across), "{}: door width", h.id);
        let on_edge = match h.door_side {
            house::DoorSide::NegX => h.door.x == f.x,
            house::DoorSide::PosX => h.door.x == f.x + f.w - 1,
            house::DoorSide::NegZ => h.door.z == f.z,
            house::DoorSide::PosZ => h.door.z == f.z + f.d - 1,
        };
        assert!(on_edge, "{}: door on the edge it faces", h.id);
        let out = h.door_side.out();
        for c in h.door.cells() {
            assert!(
                h.interior.contains(c - out),
                "{}: inner side touches the interior",
                h.id
            );
            let front = c + out;
            assert!(
                !f.contains(front) && enc.rect.contains(front),
                "{}: door front",
                h.id
            );
            assert!(
                !h.is_wall(front) && grid.is_walkable(front, false) || enc.rect.contains(front),
                "{}",
                h.id
            );
        }
        // overlaps
        if let Some(g) = enc.gate {
            assert!(!rect_overlap(f, g), "{}: gate", h.id);
            for gc in g.cells() {
                for wc in f.cells().filter(|&c| h.is_wall(c)) {
                    assert!(
                        manhattan(gc, wc) >= 2,
                        "{}: wall {wc} next to gate cell {gc}",
                        h.id
                    );
                }
            }
        }
        for other in data.enclosure_features.iter().filter(|o| o.id != h.id) {
            assert!(
                !rect_overlap(f, other.rect),
                "{} overlaps {}",
                h.id,
                other.id
            );
            for st in &other.edge_stones {
                assert!(
                    !f.contains(cell_of(Vec2::from(*st))),
                    "{}: edge stone {st:?}",
                    h.id
                );
            }
        }
        for p in data.props.iter().filter(|p| p.building.is_none()) {
            assert!(
                !f.contains(cell_of(Vec2::from(p.pos))),
                "{}: prop {}",
                h.id,
                p.id
            );
        }
        for hp in &data.hiding_places {
            assert!(
                !rect_overlap(f, hp.rect),
                "{}: hiding place {}",
                h.id,
                hp.id
            );
        }
        for (_, pos, _) in &g_food_boxes(&data) {
            assert!(!f.contains(cell_of(*pos)), "{}: food box", h.id);
        }
        // the old reserved hut kind is gone from the data
        assert!(data.enclosure_features.iter().all(|x| x.kind != "hut"));
    }
}

fn g_food_boxes(data: &LevelData) -> Vec<((), Vec2, ())> {
    data.food_boxes
        .iter()
        .map(|b| ((), Vec2::from(b.pos), ()))
        .collect()
}

// HOUSE-002
#[test]
fn house_002_table_and_data_agree() {
    let data = zoo();
    for (id, enc, model, rect, interior, door, side) in TABLE {
        let h = house::house_of(&data, enc).unwrap_or_else(|| panic!("{enc}: no house"));
        assert_eq!(h.id, id);
        assert_eq!(h.model, model);
        assert_eq!(h.footprint, Rect::from(rect), "{id}");
        assert_eq!(h.interior, Rect::from(interior), "{id}");
        assert_eq!(h.door, Rect::from(door), "{id}");
        assert_eq!(h.door_side, house::DoorSide::parse(side).unwrap(), "{id}");
    }
    assert_eq!(house::houses(&data).len(), 7);
    for enc in ["enc_lion", "enc_snow_fox", "enc_goldfish"] {
        assert!(house::house_of(&data, enc).is_none(), "{enc} has no house");
    }
}

// HOUSE-003
#[test]
fn house_003_door_and_roof_heights() {
    let data = zoo();
    for h in house::houses(&data) {
        let size = species_size(&h.enclosure);
        assert!(
            h.door_height_m >= size + 0.5 - 1e-4,
            "{}: door {} for {size}",
            h.id,
            h.door_height_m
        );
        assert!(h.roof_height_m >= h.door_height_m + 0.4 - 1e-4, "{}", h.id);
    }
    let g = house::house_of(&data, "enc_giraffe").unwrap();
    assert!(g.door_height_m >= 5.0);
    let w = if g.door_side == house::DoorSide::NegZ || g.door_side == house::DoorSide::PosZ {
        g.door.w
    } else {
        g.door.d
    };
    assert!(w >= 2);
}

// HOUSE-004
#[test]
fn house_004_home_area_has_no_wall_cells() {
    let level = zoo_level();
    for h in house::houses(&level.data) {
        let i = enc_index(&level, &h.enclosure);
        let enc = &level.data.elements[i];
        let area = home_area(&level, i);
        for c in h.footprint.cells() {
            if h.is_wall(c) {
                assert!(!area.contains(c), "{}: wall {c} in the area", h.id);
            } else {
                assert_eq!(area.class(c), Some(AreaCell::Land), "{}: {c}", h.id);
            }
        }
        for (c, _) in area.cells() {
            assert!(enc.rect.contains(c), "{}: {c} outside the enclosure", h.id);
        }
        assert!(
            area.contains(zoo_core::wander::home_entry(&level, i).unwrap()),
            "{}: entry",
            h.id
        );
    }
}

fn bfs(area: &WanderArea, start: IVec2, skip: &dyn Fn(IVec2) -> bool) -> BTreeSet<(i32, i32)> {
    let mut seen = BTreeSet::new();
    seen.insert((start.x, start.y));
    let mut q = VecDeque::from([start]);
    while let Some(c) = q.pop_front() {
        for d in [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y] {
            let n = c + d;
            if area.contains(n) && !skip(n) && seen.insert((n.x, n.y)) {
                q.push_back(n);
            }
        }
    }
    seen
}

// HOUSE-005
#[test]
fn house_005_interior_is_reached_only_over_the_door() {
    let level = zoo_level();
    for h in house::houses(&level.data) {
        let i = enc_index(&level, &h.enclosure);
        let area = home_area(&level, i);
        let entry = zoo_core::wander::home_entry(&level, i).unwrap();
        let all = bfs(&area, entry, &|_| false);
        assert_eq!(all.len(), area.len(), "{}: pockets", h.id);
        for c in h.interior.cells() {
            assert!(
                all.contains(&(c.x, c.y)),
                "{}: interior {c} unreachable",
                h.id
            );
        }
        let no_door = bfs(&area, entry, &|c| h.is_door(c));
        for c in h.interior.cells() {
            assert!(
                !no_door.contains(&(c.x, c.y)),
                "{}: interior {c} reachable without the door",
                h.id
            );
        }
        let open = h.footprint.cells().filter(|&c| h.is_wall(c)).count()
            + h.interior.cells().count()
            + h.door.cells().count();
        assert_eq!(open as i32, h.footprint.w * h.footprint.d, "{}", h.id);
    }
}

// HOUSE-006
#[test]
fn house_006_free_area_stays_connected_and_pools_unchanged() {
    let level = zoo_level();
    let mut bare = zoo();
    bare.enclosure_features.retain(|f| !f.is_animal_house());
    let bare_level = Level::new(bare);
    for h in house::houses(&level.data) {
        let i = enc_index(&level, &h.enclosure);
        let with = home_area(&level, i);
        let without = home_area(&bare_level, i);
        let outside = |c: IVec2| h.footprint.contains(c);
        let free: Vec<IVec2> = with
            .cells()
            .map(|(c, _)| c)
            .filter(|&c| !outside(c))
            .collect();
        let entry = zoo_core::wander::home_entry(&level, i).unwrap();
        let reach = bfs(&with, entry, &|c| h.footprint.contains(c));
        assert_eq!(reach.len(), free.len(), "{}: free area connected", h.id);
        assert!(
            free.len() as f32 >= 0.55 * without.len() as f32,
            "{}: {} of {}",
            h.id,
            free.len(),
            without.len()
        );
        let count = |a: &WanderArea, k: AreaCell| a.cells().filter(|&(_, c)| c == k).count();
        assert_eq!(
            count(&with, AreaCell::Water),
            count(&without, AreaCell::Water),
            "{}",
            h.id
        );
        assert_eq!(
            count(&with, AreaCell::Ramp),
            count(&without, AreaCell::Ramp),
            "{}",
            h.id
        );
        if h.id == "hippo_hut" || h.id == "elephant_house" {
            assert_eq!(
                count(&with, AreaCell::Water),
                56 - count(&with, AreaCell::Ramp),
                "{}",
                h.id
            );
        }
    }
}

// HOUSE-007
#[test]
fn house_007_routes_cross_a_door_and_no_wall() {
    let level = zoo_level();
    for h in house::houses(&level.data) {
        let i = enc_index(&level, &h.enclosure);
        let area = home_area(&level, i);
        let cells: Vec<IVec2> = area.cells().map(|(c, _)| c).collect();
        // every 3rd origin keeps the test fast; every target is covered
        for &a in cells.iter().step_by(3) {
            for &b in &cells {
                if a == b {
                    continue;
                }
                let Some(route) = area.route(a, b) else {
                    continue;
                };
                let mut prev = a;
                for &c in &route {
                    assert_eq!(
                        (c - prev).abs().element_sum(),
                        1,
                        "{}: 4-neighbour steps",
                        h.id
                    );
                    assert!(!h.is_wall(c), "{}: route on a wall {c}", h.id);
                    prev = c;
                }
                if h.is_interior(b) && !h.is_interior(a) {
                    assert!(
                        route.iter().any(|&c| h.is_door(c)) || h.is_door(a),
                        "{}: route into the house without a door cell",
                        h.id
                    );
                }
            }
        }
    }
}

// HOUSE-008
#[test]
fn house_008_animals_rest_in_their_house_and_use_the_door() {
    // one game per species of its own level (the other animals stay out: no nightfall)
    let species: [(&str, u8); 7] = [
        ("zebra", 1),
        ("hippo", 1),
        ("panda", 1),
        ("koala", 2),
        ("elephant", 2),
        ("giraffe", 2),
        ("monkey", 3),
    ];
    for (id, lv) in species {
        let data = match lv {
            1 => common::level1(),
            2 => common::level2(),
            _ => common::level3(),
        };
        let mut g = Game::new(data, 8).unwrap();
        g.player.pos = cell_center(g.level.data.spawn.cell());
        assert!(g.debug_send_home(id), "{id}");
        g.drain_events();
        let i = g.animal_index(id).unwrap();
        let h = g.houses[g.animals[i].enclosure].clone().expect("house");
        let area = home_area(&g.level, g.animals[i].enclosure);
        let mut prev = cell_of(g.animals[i].pos);
        let (mut rested, mut left) = (0usize, false);
        for k in 0..(600.0 / DT) as usize {
            if k % (60 * 60) == 0 {
                // a changed hint signature restarts the stall timer (its walking-to-the-child
                // routes are slow in debug builds and no subject of this test)
                if g.carry.food().is_some() {
                    let _ = g.carry.consume();
                } else {
                    g.carry.take(&zoo_core::FoodBox {
                        food: zoo_core::Food::Hay,
                    });
                }
            }
            g.update(DT, Vec2::ZERO);
            let c = cell_of(g.animals[i].pos);
            assert!(area.contains(c), "{id}: left its area at {c}");
            assert!(!h.is_wall(c), "{id}: on a wall cell {c}");
            if c != prev {
                let (was_in, now_in) = (h.is_interior(prev), h.is_interior(c));
                if was_in != now_in {
                    // between the room and anything else: only over a door cell
                    let door = if now_in { prev } else { c };
                    assert!(h.is_door(door), "{id}: {prev} -> {c} skipped the door");
                }
                left |= was_in && !now_in;
            }
            if h.is_interior(c) && g.animals[i].wander.route.is_empty() {
                rested += 1;
            }
            prev = c;
        }
        assert!(rested > 60, "{id}: never rested inside ({rested})");
        assert!(left, "{id}: never left the house");
    }
}

fn put_inside(g: &mut Game, i: usize, rest: f32) -> House {
    let h = g.houses[g.animals[i].enclosure].clone().unwrap();
    let c = h.interior_cells()[0];
    g.animals[i].pos = cell_center(c);
    g.animals[i].wander.route.clear();
    g.animals[i].wander.rest_s = rest;
    g.animals[i].wander.pause_s = 30.0;
    h
}

// HOUSE-009
#[test]
fn house_009_resting_animal_comes_out_for_the_child_and_feeding() {
    for id in housed_ids() {
        let mut g = open_zoo_game(9);
        assert!(g.debug_send_home(id));
        let i = g.animal_index(id).unwrap();
        let h = put_inside(&mut g, i, 18.0);
        // far away: it stays and rests
        run(&mut g, 2.0);
        assert!(
            h.is_interior(cell_of(g.animals[i].pos)),
            "{id}: rests while nobody is near"
        );
        // the child 4 m from the door front
        g.player.pos = h.door_front_center() + h.door_side.out().as_vec2() * 4.0;
        run(&mut g, 2.0);
        let a = &g.animals[i];
        assert!(
            !h.is_interior(cell_of(a.pos))
                || a.wander
                    .route
                    .last()
                    .is_some_and(|c| !h.footprint.contains(*c)),
            "{id}: did not start out within 2 s"
        );
        run(&mut g, 60.0);
        assert!(
            !h.footprint.contains(cell_of(g.animals[i].pos))
                || h.is_door(cell_of(g.animals[i].pos))
                || id == "x"
        );
    }
}

#[test]
fn house_009_feed_spot_is_outside_and_reached() {
    for id in ["zebra", "hippo", "panda", "giraffe", "elephant", "monkey"] {
        let mut g = open_zoo_game(10);
        assert!(g.debug_send_home(id));
        g.drain_events();
        let i = g.animal_index(id).unwrap();
        let e = g.animals[i].enclosure;
        let h = put_inside(&mut g, i, 18.0);
        let s = g.feed_spot(e).unwrap().clone();
        for c in s.cells {
            assert!(
                !h.footprint.contains(c),
                "{id}: feed spot cell {c} in the house"
            );
        }
        g.garden.basket.carrots = 2;
        g.player.pos = s.stand;
        g.player.facing = (cell_center(s.cells[0]) - s.stand).normalize();
        run(&mut g, 60.0);
        let a = &g.animals[g.animal_index(id).unwrap()];
        let c = cell_center(s.cell_for(usize::from(a.member)));
        assert!(
            a.pos.distance(c) < 0.6,
            "{id}: at {} not at its spot {c}",
            a.pos
        );
    }
}

// HOUSE-010
#[test]
fn house_010_nightfall_sends_everyone_in() {
    let mut g = open_zoo_game(11);
    let ids = housed_ids();
    for id in &ids {
        assert!(g.debug_send_home(id));
    }
    assert!(g.debug_send_home("lion"));
    g.drain_events();
    run(&mut g, 20.0);
    let lion = g.animal_index("lion").unwrap();
    let lion_pos = g.animals[lion].pos;
    g.daytime.force(zoo_core::daytime::Phase::Night);
    run(&mut g, 60.0);
    for id in &ids {
        let group = g.group(id);
        let h = g.houses[g.animals[group[0]].enclosure].clone().unwrap();
        let inside: Vec<Vec2> = group
            .iter()
            .map(|&j| g.animals[j].pos)
            .filter(|&p| h.is_interior(cell_of(p)))
            .collect();
        assert_eq!(inside.len(), group.len(), "{id}: inside after 60 s");
        let cells: std::collections::BTreeSet<_> = inside
            .iter()
            .map(|&p| (cell_of(p).x, cell_of(p).y))
            .collect();
        assert_eq!(cells.len(), inside.len(), "{id}: different interior cells");
    }
    assert_eq!(
        g.animals[lion].pos, lion_pos,
        "the lion has no house and is unaffected"
    );
}

// HOUSE-011
#[test]
fn house_011_hiding_places_and_escaped_animals_avoid_houses() {
    let level = zoo_level();
    let houses = house::houses(&level.data);
    for hp in &level.data.hiding_places {
        let area = zoo_core::wander::hiding_area(&level, hp);
        for (c, _) in area.cells() {
            assert!(
                houses.iter().all(|h| !h.footprint.contains(c)),
                "{}: {c}",
                hp.id
            );
        }
    }
    let mut g = open_zoo_game(12);
    for _ in 0..(120.0 / DT) as usize {
        g.update(DT, Vec2::ZERO);
        for a in g
            .animals
            .iter()
            .filter(|a| a.state != AnimalState::InEnclosure)
        {
            let c = cell_of(a.pos);
            assert!(
                houses.iter().all(|h| !h.footprint.contains(c)),
                "{} on a house cell {c}",
                a.id()
            );
        }
    }
}

// HOUSE-012
#[test]
fn house_012_player_never_enters_a_house() {
    let level = zoo_level();
    let grid = level.grid();
    for h in house::houses(&level.data) {
        let enc = level.data.element(&h.enclosure).unwrap().rect;
        let centre = Vec2::new(
            h.footprint.x as f32 + h.footprint.w as f32 / 2.0,
            h.footprint.z as f32 + h.footprint.d as f32 / 2.0,
        );
        for dir in [Vec2::X, Vec2::NEG_X, Vec2::Y, Vec2::NEG_Y] {
            let reach = if dir.x != 0.0 {
                enc.w as f32 + 12.0
            } else {
                enc.d as f32 + 12.0
            };
            let mut start = centre + dir * reach;
            // step in until walkable
            while !grid.is_walkable(cell_of(start), false) && (start - centre).length() > 3.0 {
                start -= dir;
            }
            if !grid.is_walkable(cell_of(start), false) {
                continue;
            }
            let mut g = Game::new(zoo(), 1).unwrap();
            g.player.pos = start;
            for _ in 0..(20.0 / DT) as usize {
                let to = centre - g.player.pos;
                g.update(DT, to.normalize_or_zero());
                let c = cell_of(g.player.pos);
                assert!(
                    !h.footprint.contains(c),
                    "{}: player on house cell {c} from {dir}",
                    h.id
                );
                assert!(
                    g.level.grid().is_walkable(c, false),
                    "{}: left the walkable cells at {c}",
                    h.id
                );
            }
        }
    }
}

// HOUSE-013
#[test]
fn house_013_open_archway_without_leaf_and_a_free_front() {
    let data = zoo();
    let scene = zoo_core::scene::LevelScene::build(&data);
    for h in house::houses(&data) {
        for o in &scene.openings {
            let c = cell_of(o.center);
            assert!(
                !h.footprint.contains(c),
                "{}: gate/door model at {c} (no door leaf)",
                h.id
            );
        }
        for c in h.door_front() {
            let m = cell_center(c);
            for p in data.props.iter().filter(|p| p.building.is_none()) {
                assert!(
                    Vec2::from(p.pos).distance(m) >= 1.0,
                    "{}: prop {} on the door front",
                    h.id,
                    p.id
                );
            }
            for sc in &data.scenery {
                assert!(
                    !sc.rect.contains(c),
                    "{}: scenery {} on the door front",
                    h.id,
                    sc.id
                );
            }
        }
    }
}

// HOUSE-014
#[test]
fn house_014_roof_cut_away_near_the_door_with_an_animal_inside() {
    let mut g = open_zoo_game(14);
    assert!(g.debug_send_home("zebra"));
    let i = g.animal_index("zebra").unwrap();
    let h = put_inside(&mut g, i, 18.0);
    let near = h.door_front_center() + Vec2::new(0.0, -7.0);
    let far = h.door_front_center() + Vec2::new(0.0, -9.0);
    assert!(h.cutaway(near, true));
    assert!(!h.cutaway(far, true));
    assert!(!h.cutaway(near, false));
    g.player.pos = near;
    assert!(g.house_cutaway("zebra_shelter"));
    g.player.pos = far;
    assert!(!g.house_cutaway("zebra_shelter"));
    assert!(!g.house_cutaway("giraffe_house"));
}

// HOUSE-015
#[test]
fn house_015_save_restore_inside_a_house_and_old_saves() {
    let mut a = open_zoo_game(15);
    assert!(a.debug_send_home("giraffe"));
    let i = a.animal_index("giraffe").unwrap();
    put_inside(&mut a, i, 16.0);
    run(&mut a, 4.0);
    let save = a.to_save();
    let mut b = Game::from_save(common::zoo_with_night(), &save).unwrap();
    assert!((b.animals[i].wander.rest_s - a.animals[i].wander.rest_s).abs() < 1e-4);
    for _ in 0..(60.0 / DT) as usize {
        a.update(DT, Vec2::ZERO);
        b.update(DT, Vec2::ZERO);
        assert_eq!(a.animals[i].pos, b.animals[i].pos);
    }
    // an old save has no `rest_s`
    let mut v: serde_json::Value = serde_json::from_str(&save.to_json()).unwrap();
    for an in v["animals"].as_array_mut().unwrap() {
        an.as_object_mut().unwrap().remove("rest_s");
    }
    let old = Game::from_save_json(common::zoo_with_night(), &v.to_string()).unwrap();
    assert_eq!(old.animals[i].wander.rest_s, 0.0);
}

// HOUSE-016 (the follow-the-hints fuzz HINT-021 / HINT-019 also place animals inside their
// houses, see `messy_state` in hints.rs): from states with animals inside, the hint has a
// mission-level target and a save/restore changes nothing.
#[test]
fn house_016_states_with_animals_inside_have_hints_and_restore() {
    use zoo_core::hints::{candidates, HintTracker, PRIO_UNSTARTED};
    for seed in 0..30u64 {
        let mut g = open_zoo_game(seed);
        let ids = housed_ids();
        for (k, id) in ids.iter().enumerate() {
            if (seed + k as u64).is_multiple_of(2) {
                g.debug_send_home(id);
                let i = g.animal_index(id).unwrap();
                put_inside(&mut g, i, 15.0);
            }
        }
        let _ = g.debug_set_daytime(["day", "dusk", "night"][seed as usize % 3]);
        run(&mut g, 5.0 + (seed % 7) as f32 * 10.0);
        let t = HintTracker::default();
        if g.any_mission_open() && !g.daytime.is_night() {
            let c = candidates(&g, &t);
            assert!(
                c.first().is_some_and(|h| h.priority <= PRIO_UNSTARTED),
                "seed {seed}: {c:?}"
            );
            // no hint points into a house (HOUSE-017)
            let houses: Vec<House> = house::houses(&g.level.data);
            for h in &c {
                let cell = cell_of(h.pos);
                assert!(
                    houses.iter().all(|x| !x.footprint.contains(cell)),
                    "seed {seed}: hint {h:?} in a house"
                );
            }
        }
        let save = g.to_save();
        let b = Game::from_save(common::zoo_with_night(), &save).unwrap();
        assert_eq!(b.to_save().animals, save.animals, "seed {seed}");
    }
}

// HOUSE-017
#[test]
fn house_017_inside_animal_is_home_and_never_a_stall_target() {
    let mut g = open_zoo_game(17);
    assert!(g.debug_send_home("zebra"));
    let i = g.animal_index("zebra").unwrap();
    put_inside(&mut g, i, 15.0);
    assert_eq!(g.animals[i].state, AnimalState::InEnclosure);
    assert!(g.mission("zebra").unwrap().complete);
    // the stall walk only concerns escaped animals of open missions
    assert_ne!(g.animals[i].state, AnimalState::Escaped);
    let t = zoo_core::hints::HintTracker::default();
    let houses = house::houses(&g.level.data);
    for h in zoo_core::hints::candidates(&g, &t) {
        let c = cell_of(h.pos);
        assert!(houses.iter().all(|x| !x.footprint.contains(c)), "{h:?}");
    }
}

// HOUSE-020
#[test]
fn house_020_placeholder_has_a_door_cutout_and_the_roof_height() {
    let data = zoo();
    let scene = zoo_core::scene::LevelScene::build(&data);
    for h in house::houses(&data) {
        assert!(
            !house::BUILT_MODELS.contains(&h.model.as_str()),
            "{}: model built, so no placeholder expected here",
            h.id
        );
        let mine: Vec<_> = scene
            .boxes
            .iter()
            .filter(|b| b.source == h.id || b.source.starts_with(&format!("{}:", h.id)))
            .collect();
        assert!(!mine.is_empty(), "{}: placeholder boxes", h.id);
        // never a closed box: no box that reaches from the ground to the lintel covers a
        // door cell
        for c in h.door.cells() {
            let m = cell_center(c);
            for b in &mine {
                let p = zoo_core::coords::world_to_level(b.pos);
                let (hx, hz) = (b.size.x / 2.0, b.size.z / 2.0);
                let covers = (m.x - p.x).abs() < hx && (m.y - p.y).abs() < hz;
                let bottom = b.pos.y;
                if covers {
                    assert!(
                        bottom >= h.door_height_m - 1e-3,
                        "{}: box {} blocks the doorway at {c}",
                        h.id,
                        b.source
                    );
                }
            }
        }
        // wall boxes reach roof_height_m; the roof slab sits on top of it
        let wall_top = mine
            .iter()
            .filter(|b| b.source == h.id)
            .map(|b| b.pos.y + b.size.y)
            .fold(0.0, f32::max);
        assert!(
            (wall_top - h.roof_height_m).abs() < 1e-3,
            "{}: wall {wall_top}",
            h.id
        );
        let walls = mine.iter().filter(|b| b.source == h.id).count();
        assert_eq!(
            walls,
            h.footprint.cells().filter(|&c| h.is_wall(c)).count(),
            "{}: one wall box per wall cell",
            h.id
        );
        assert!(
            scene.roof_boxes.iter().any(|(id, _)| *id == h.id),
            "{}: roof cut-away region",
            h.id
        );
    }
}

// the model list only names `.glb` files that exist
#[test]
fn house_020_built_models_exist_as_glb() {
    for m in house::BUILT_MODELS {
        let p = common::repo_root().join(format!("assets/models/buildings/{m}.glb"));
        assert!(p.exists(), "{m}");
    }
}

// HOUSE-006 (numbers of the spec table): land cells of the home area with the house
#[test]
fn house_006_home_area_cell_counts() {
    let level = zoo_level();
    let want = [
        ("enc_zebra", 113),
        ("enc_panda", 102),
        ("enc_hippo", 59),
        ("enc_koala", 68),
        ("enc_elephant", 86),
        ("enc_giraffe", 124),
        ("enc_monkey", 104),
    ];
    let mut got = Vec::new();
    for (enc, _) in want {
        let area = home_area(&level, enc_index(&level, enc));
        got.push((
            enc,
            area.cells().filter(|&(_, k)| k == AreaCell::Land).count(),
        ));
    }
    assert_eq!(got, want.to_vec());
}
