//! GAME-CART (P3): the golf carts — data, boarding, driving, collision, parking check, save,
//! night lights (CART-001..011, 015..021, 028).

mod common;

use glam::{IVec2, Vec2};
use zoo_core::animals::AnimalState;
use zoo_core::cart::{box_shape, CartLock, LeaveResult, DRIVE_ERROR_DEG};
use zoo_core::game::{GameEvent, Interaction, Target};
use zoo_core::level::{cell_center, cell_of, CellKind, Surface};
use zoo_core::Game;

const DT: f32 = 1.0 / 30.0;

fn game() -> Game {
    common::zoo_game(11)
}

fn with_key(mut g: Game) -> Game {
    g.has_cart_key = true;
    g.key_box_open = true;
    g
}

fn open_all_levels(g: &mut Game) {
    for id in g
        .level
        .data
        .parts
        .iter()
        .map(|p| p.id.clone())
        .collect::<Vec<_>>()
    {
        for e in g.level.data.elements.clone() {
            if e.ty == zoo_core::ElementType::Barrier
                && e.transition.as_deref().is_some_and(|t| t.ends_with(&id))
            {
                g.level.open_barrier(&e.id);
            }
        }
    }
}

fn run(g: &mut Game, seconds: f32, input: Vec2) {
    let n = (seconds / DT).round() as usize;
    for _ in 0..n {
        g.update(DT, input);
    }
}

/// Stands the player at the boarding cell of cart `i` and gets in.
fn board(g: &mut Game, i: usize) {
    let stand = g.carts[i].stand;
    g.player.pos = cell_center(stand);
    g.player.facing = (g.carts[i].pos - g.player.pos).normalize();
    assert!(g.board_cart(i), "board cart {i}");
}

/// A straight run of `len` metres where the box is free all along, on the given surface.
fn find_run(g: &Game, surface: Surface, len: f32) -> (Vec2, Vec2) {
    let b = g.level.grid().bounds();
    for z in b.z..b.z + b.d {
        for x in b.x..b.x + b.w {
            for dir in [Vec2::X, Vec2::Y, Vec2::NEG_X, Vec2::NEG_Y] {
                let start = cell_center(IVec2::new(x, z));
                let ok = (0..=(len * 2.0) as usize).all(|k| {
                    let p = start + dir * (k as f32 * 0.5);
                    !g.cart_pose_overlaps(p, dir)
                        && g.level.grid().surface(cell_of(p)) == Some(surface)
                        && (0..3).all(|s| {
                            let q = p + Vec2::new(-dir.y, dir.x) * (s as f32 - 1.0) * 0.9;
                            g.level.grid().surface(cell_of(q)) == Some(surface)
                        })
                });
                if ok {
                    return (start, dir);
                }
            }
        }
    }
    panic!("no {surface:?} run of {len} m");
}

fn put_cart(g: &mut Game, i: usize, pos: Vec2, dir: Vec2) {
    g.carts[i].pos = pos;
    g.carts[i].dir = dir;
    if g.seated == Some(i) {
        g.player.pos = pos;
    }
}

// CART-001
#[test]
fn cart_001_three_carts_at_their_parking_poses() {
    let g = game();
    let table = [
        ("cart_l1", Vec2::new(4.0, 3.5), Vec2::Y),
        ("cart_l2", Vec2::new(28.5, 31.7), Vec2::X),
        ("cart_l3", Vec2::new(15.5, 51.0), Vec2::X),
    ];
    assert_eq!(g.carts.len(), 3);
    for (id, pos, dir) in table {
        let c = g.cart(id).unwrap_or_else(|| panic!("{id}"));
        assert!(c.pos.distance(pos) < 1e-4, "{id} pos {}", c.pos);
        assert!(c.dir.distance(dir) < 1e-4, "{id} dir");
        assert!(g.level.cart_shapes().len() == 3);
    }
}

// CART-011 / CART-020
#[test]
fn cart_011_020_locked_until_key_and_level() {
    let mut g = game();
    for i in 0..3 {
        assert_eq!(g.cart_lock(i), Some(CartLock::NoKey));
    }
    g.has_cart_key = true;
    assert_eq!(g.cart_lock(0), None, "level 1 cart with the key");
    assert_eq!(g.cart_lock(1), Some(CartLock::ClosedLevel));
    assert_eq!(g.cart_lock(2), Some(CartLock::ClosedLevel));
    open_all_levels(&mut g);
    assert_eq!(g.cart_lock(1), None);
    assert_eq!(g.cart_lock(2), None);
}

// CART-021 (core part): tapping a locked cart gives the locked feedback
#[test]
fn cart_021_locked_feedback_interaction() {
    let mut g = game();
    g.player.pos = cell_center(g.carts[0].stand);
    g.player.facing = (g.carts[0].pos - g.player.pos).normalize();
    assert_eq!(
        g.available_target(),
        Some(Target::Cart {
            id: "cart_l1".into()
        })
    );
    let r = g.interact();
    assert_eq!(
        r,
        Some(Interaction::CartLocked {
            id: "cart_l1".into(),
            reason: CartLock::NoKey
        })
    );
    assert!(g.drain_events().contains(&GameEvent::CartLocked {
        closed_level: false
    }));
    assert!(g.seated.is_none());
    // with the key but a closed level
    g.has_cart_key = true;
    g.player.pos = cell_center(g.carts[1].stand);
    let r = g.interact();
    assert_eq!(
        r,
        Some(Interaction::CartLocked {
            id: "cart_l2".into(),
            reason: CartLock::ClosedLevel
        })
    );
}

// CART-002
#[test]
fn cart_002_board_and_get_out_driver_side_first() {
    let mut g = with_key(game());
    g.player.pos = cell_center(g.carts[0].stand);
    g.player.facing = (g.carts[0].pos - g.player.pos).normalize();
    let r = g.interact();
    assert!(matches!(r, Some(Interaction::CartBoarded { .. })), "{r:?}");
    assert_eq!(g.seated, Some(0));
    assert_eq!(g.available_target(), Some(Target::GetOut));
    let (pos, dir) = (g.carts[0].pos, g.carts[0].dir);
    let r = g.interact();
    assert_eq!(
        r,
        Some(Interaction::CartLeft {
            id: "cart_l1".into()
        })
    );
    assert!(g.seated.is_none());
    assert_eq!((g.carts[0].pos, g.carts[0].dir), (pos, dir), "cart stays");
    // the exit is on the driver's side (left of the heading) when free
    let left = Vec2::new(-dir.y, dir.x);
    assert!(
        (g.player.pos - pos).dot(left) > 0.5,
        "left side: {}",
        g.player.pos
    );
    // the parked box is solid again and she is not inside it
    assert!(!box_shape(pos, dir).overlaps(g.player.pos, 0.3));
    // out of reach: not available
    g.player.pos = pos + Vec2::new(6.0, 6.0);
    assert!(!matches!(g.available_target(), Some(Target::Cart { .. })));
}

// CART-003
#[test]
fn cart_003_speed_on_path_and_grass() {
    for (surface, want) in [(Surface::Path, 4.5), (Surface::Grass, 2.0)] {
        let mut g = with_key(game());
        board(&mut g, 0);
        let len = if surface == Surface::Path { 9.0 } else { 6.0 };
        let (start, dir) = find_run(&g, surface, len);
        put_cart(&mut g, 0, start, dir);
        let mut max_acc = 0.0f32;
        run(&mut g, 1.0, dir);
        let p0 = g.carts[0].pos;
        let mut last = g.carts[0].speed;
        for _ in 0..15 {
            g.update(DT, dir);
            let v = g.carts[0].speed;
            max_acc = max_acc.max((v - last).abs() / DT);
            last = v;
        }
        let d = g.carts[0].pos.distance(p0);
        let v = d / 0.5;
        assert!((v - want).abs() <= want * 0.05, "{surface:?}: {v} m/s");
        assert!(max_acc <= 10.0 + 0.5, "{surface:?}: acceleration {max_acc}");
    }
}

// CART-004: never overlaps fences, barriers, water, walls, doors, gates, the level border
#[test]
fn cart_004_never_enters_solids_gates_doors_gardens() {
    let g0 = with_key(game());
    let data = &g0.level.data;
    let mut targets: Vec<(String, IVec2)> = Vec::new();
    for e in &data.elements {
        use zoo_core::ElementType::*;
        let c = match e.ty {
            Enclosure => e.gate.map(|g| IVec2::new(g.x, g.z)),
            Building => e.door_cell(),
            Barrier | Landmark | Boundary => Some(IVec2::new(e.rect.x, e.rect.z)),
            _ => None,
        };
        if let Some(c) = c {
            targets.push((e.id.clone(), c));
        }
    }
    assert!(targets.len() > 20);
    let mut tried = 0;
    for (id, target) in targets.iter() {
        let mut g = with_key(game());
        board(&mut g, 0);
        // a free pose 4-6 m from the target, facing it
        let t = cell_center(*target);
        let mut found = None;
        'o: for r in [3.5f32, 4.5, 6.0, 8.0] {
            for k in 0..32 {
                let a = k as f32 * std::f32::consts::TAU / 32.0;
                let p = t + Vec2::new(a.cos(), a.sin()) * r;
                let dir = (t - p).normalize();
                if !g.cart_pose_overlaps(p, dir) && g.level.grid().is_walkable(cell_of(p), false) {
                    found = Some((p, dir));
                    break 'o;
                }
            }
        }
        let Some((p, dir)) = found else { continue };
        tried += 1;
        put_cart(&mut g, 0, p, dir);
        for _ in 0..240 {
            g.update(DT, dir);
            let c = &g.carts[0];
            assert!(
                !g.cart_pose_overlaps(c.pos, c.dir),
                "{id}: overlaps at {}",
                c.pos
            );
            assert!(
                matches!(g.level.grid().kind(cell_of(c.pos)), CellKind::Walkable(_)),
                "{id}: centre on a non-walkable cell"
            );
            assert!(!g.cart_cell_blocked(cell_of(c.pos)), "{id}: banned cell");
        }
    }
    assert!(tried >= 12, "{tried}");
}

// CART-005
#[test]
fn cart_005_stops_before_animals_and_ducks() {
    for duck in [false, true] {
        let mut g = with_key(game());
        board(&mut g, 0);
        let (start, dir) = find_run(&g, Surface::Path, 9.0);
        put_cart(&mut g, 0, start, dir);
        let victim = start + dir * 7.0;
        if duck {
            g.cart_obstacles.push((victim, 0.3));
        } else {
            g.animals[0].pos = victim;
            g.animals[0].state = AnimalState::Escaped;
        }
        let r = if duck { 0.3 } else { 0.45 };
        let mut min_clear = f32::INFINITY;
        for _ in 0..300 {
            g.update(DT, dir);
            let c = &g.carts[0];
            min_clear = min_clear.min(zoo_core::cart::distance_to_box(c.pos, c.dir, victim) - r);
        }
        assert!(min_clear >= 0.45, "duck={duck}: clearance {min_clear}");
        assert!(g.carts[0].pos.distance(start) > 3.0, "it did drive");
        assert!(g.carts[0].speed < 0.01);
    }
}

// no animal ever stands inside a cart's box
#[test]
fn cart_005_animals_are_pushed_out_of_the_box() {
    let mut g = with_key(game());
    let c = g.carts[0].clone();
    g.animals[0].pos = c.pos;
    g.animals[0].state = AnimalState::Escaped;
    g.update(DT, Vec2::ZERO);
    let a = g.animals[0].pos;
    assert!(
        zoo_core::cart::distance_to_box(c.pos, c.dir, a) >= 0.29,
        "animal at {a}"
    );
}

// CART-006
#[test]
fn cart_006_followers_wait_while_driving() {
    let mut g = with_key(game());
    let i = 0;
    g.animals[i].state = AnimalState::Following;
    g.animals[i].pos = g.carts[0].pos + Vec2::new(-2.0, 0.0);
    let from = g.animals[i].pos;
    board(&mut g, 0);
    assert!(g.drain_events().iter().any(|e| matches!(
        e,
        GameEvent::CartBoarded {
            followers: true,
            ..
        }
    )));
    let (start, dir) = find_run(&g, Surface::Path, 12.0);
    put_cart(&mut g, 0, start, dir);
    g.animals[i].pos = start - dir * 1.8;
    let from = if (from - g.animals[i].pos).length() > 0.0 {
        g.animals[i].pos
    } else {
        from
    };
    run(&mut g, 4.0, dir);
    assert!(g.player.pos.distance(from) > 15.0 || g.carts[0].pos.distance(from) > 12.0);
    assert!(g.animals[i].waiting);
    assert!(g.animals[i].pos.distance(from) < 0.5, "it stays");
    // back on foot within 5 m: it follows again
    g.player.pos = from + dir * 3.0;
    g.seated = None;
    g.update(DT, Vec2::ZERO);
    run(&mut g, 0.5, Vec2::ZERO);
    assert!(!g.animals[i].waiting);
}

// CART-007
#[test]
fn cart_007_carried_things_come_along() {
    let mut g = with_key(game());
    g.carry.take(&zoo_core::FoodBox {
        food: zoo_core::Food::Hay,
    });
    g.has_cart_key = true;
    board(&mut g, 0);
    run(&mut g, 2.0, Vec2::Y);
    assert!(g.has_cart_key);
    assert!(g.carry.food().is_some());
    // drive back to a free spot and get out
    for _ in 0..30 {
        if g.leave_cart() == LeaveResult::Left {
            break;
        }
        run(&mut g, 0.5, Vec2::new(1.0, 0.0));
    }
    assert!(g.seated.is_none());
    assert_eq!(g.carry.food(), Some(zoo_core::Food::Hay));
}

// CART-008
#[test]
fn cart_008_no_panels_or_interactions_while_seated() {
    let mut g = with_key(game());
    board(&mut g, 0);
    // next to the first info board
    let board_pt = g.interactables().len();
    assert_eq!(board_pt, 0, "nothing offered while seated");
    let e = g
        .level
        .data
        .elements
        .iter()
        .find(|e| e.kind.as_deref() == Some("info_board"))
        .cloned()
        .unwrap();
    let at = cell_center(IVec2::new(e.rect.x, e.rect.z));
    g.carts[0].pos = at;
    g.player.pos = at;
    run(&mut g, 2.0, Vec2::ZERO);
    assert!(g.panel.open.is_none());
    assert_eq!(g.available_target(), Some(Target::GetOut));
    assert!(g.interactables().is_empty());
}

// CART-009
#[test]
fn cart_009_save_and_restore_while_seated() {
    let mut g = with_key(game());
    board(&mut g, 0);
    let (start, dir) = find_run(&g, Surface::Path, 9.0);
    put_cart(&mut g, 0, start, dir);
    run(&mut g, 2.0, dir);
    let (pos, d) = (g.carts[0].pos, g.carts[0].dir);
    let s = g.to_save();
    assert_eq!(s.seated.as_deref(), Some("cart_l1"));
    let json = serde_json::to_string(&s).unwrap();
    let s2: zoo_core::save::SaveState = serde_json::from_str(&json).unwrap();
    let r = Game::from_save(common::zoo(), &s2).unwrap();
    assert_eq!(r.seated, Some(0));
    assert!(r.carts[0].pos.distance(pos) < 1e-3);
    assert!(r.carts[0].dir.distance(d) < 1e-3);
    assert!(r.player.pos.distance(pos) < 1e-3);
    // other carts stay parked
    assert!(r.carts[1].pos.distance(r.carts[1].home_pos) < 1e-4);
    // a pose that is no longer walkable: the nearest free pose within 4 m, else the parking pose
    let mut bad = s2.clone();
    let wall = g
        .level
        .data
        .elements
        .iter()
        .find(|e| e.ty == zoo_core::ElementType::Enclosure)
        .unwrap()
        .rect;
    bad.carts[0].x = (wall.x + wall.w / 2) as f32;
    bad.carts[0].z = (wall.z + wall.d / 2) as f32;
    let r = Game::from_save(common::zoo(), &bad).unwrap();
    let c = &r.carts[0];
    assert!(!r.cart_pose_overlaps(c.pos, c.dir) || c.pos == c.home_pos);
    assert!(
        c.pos.distance(Vec2::new(bad.carts[0].x, bad.carts[0].z)) <= 4.0 + 1e-3
            || c.pos == c.home_pos
    );
}

// CART-015: data shape and the spec table
#[test]
fn cart_015_level_files_have_the_carts() {
    let rows = [
        (
            "level-1",
            "cart_l1",
            [4.0, 3.5],
            "+z",
            [3, 2, 2, 3],
            [2, 3],
            [5.5, 2.5],
            "",
        ),
        (
            "level-2",
            "cart_l2",
            [28.5, 31.7],
            "+x",
            [27, 31, 3, 2],
            [28, 30],
            [26.9, 31.5],
            "level_2",
        ),
        (
            "level-3",
            "cart_l3",
            [15.5, 51.0],
            "+x",
            [14, 50, 3, 2],
            [15, 52],
            [13.5, 50.5],
            "level_3",
        ),
    ];
    for (file, id, pos, facing, rect, stand, sign, locked) in rows {
        let d = zoo_core::LevelData::from_toml_str(&common::read(&format!(
            "assets/levels/{file}.toml"
        )))
        .unwrap();
        assert_eq!(d.carts.len(), 1, "{file}");
        let c = &d.carts[0];
        assert_eq!(c.id, id);
        assert_eq!(c.pos, pos);
        assert_eq!(c.facing, facing);
        assert_eq!(c.rect, zoo_core::Rect::from(rect));
        assert_eq!(c.stand, stand);
        assert_eq!(c.sign_pos, sign);
        assert_eq!(c.locked_until, locked);
        assert_eq!(c.model, "golf_cart");
        // the oriented box lies inside the rect
        let shape = box_shape(c.pos(), c.heading());
        let (lo, hi) = shape.aabb();
        assert!(
            lo.x >= rect[0] as f32 - 1e-4 && lo.y >= rect[1] as f32 - 1e-4,
            "{id}"
        );
        assert!(
            hi.x <= (rect[0] + rect[2]) as f32 + 1e-4 && hi.y <= (rect[1] + rect[3]) as f32 + 1e-4,
            "{id}"
        );
    }
}

// CART-016: the parking rects are harmless
#[test]
fn cart_016_parking_rects_pass_the_checks() {
    let mut g = game();
    open_all_levels(&mut g);
    let grid = g.level.grid();
    for c in &g.carts {
        let d = g.level.data.carts.iter().find(|d| d.id == c.id).unwrap();
        for cell in d.rect.cells() {
            assert!(
                matches!(grid.kind(cell), CellKind::Walkable(_)),
                "{}: cell {cell} walkable",
                c.id
            );
            assert!(!g.cart_cell_blocked(cell), "{}: cell {cell}", c.id);
            assert!(!g
                .level
                .data
                .hiding_places
                .iter()
                .any(|h| h.rect.contains(cell)));
            assert!(!g
                .level
                .data
                .items
                .iter()
                .any(|i| i.stand.map(IVec2::from) == Some(cell)));
        }
        let stand = c.stand;
        assert!(grid.is_passable(stand, false), "{}: boarding cell", c.id);
        assert!(
            g.cart_distance(
                g.carts.iter().position(|x| x.id == c.id).unwrap(),
                cell_center(stand)
            ) <= 1.5
        );
        // the parked box (with the others parked) does not disconnect the grid
        let mut h = common::zoo_game(3);
        h.level.set_cart_shapes(Vec::new());
        assert_eq!(
            h.parking_reason(c.home_pos, c.home_dir),
            None,
            "{}: parking check",
            c.id
        );
    }
}

// CART-017
#[test]
fn cart_017_no_reversing_turns_on_the_spot() {
    let mut g = with_key(game());
    board(&mut g, 0);
    let (start, dir) = find_run(&g, Surface::Path, 9.0);
    put_cart(&mut g, 0, start, dir);
    let back = -dir;
    let mut t = 0.0;
    let mut min_speed = 0.0f32;
    loop {
        g.update(DT, back);
        t += DT;
        min_speed = min_speed.min(g.carts[0].speed);
        if g.carts[0].dir.dot(back) > 0.999 || t > 3.0 {
            break;
        }
    }
    assert!(t <= 1.3, "turned in {t} s");
    assert!(
        g.carts[0].pos.distance(start) <= 0.2,
        "moved {}",
        g.carts[0].pos.distance(start)
    );
    assert!(min_speed >= 0.0);
    run(&mut g, 1.5, back);
    assert!(g.carts[0].pos.distance(start) > 1.5, "then drives forward");
    let _ = DRIVE_ERROR_DEG;
}

// CART-019: the parking check refuses a harmful spot; she stays seated
#[test]
fn cart_019_parking_check_refuses_gates_doors_stand_cells() {
    let mut g = with_key(game());
    board(&mut g, 0);
    // an item's stand cell (the key box in front of the zookeeper house)
    let it = g
        .level
        .data
        .items
        .iter()
        .find(|i| i.id == "key_box_l1")
        .unwrap();
    let stand = it.stand.unwrap();
    let at = Vec2::new(stand[0] as f32 + 0.5, stand[1] as f32 + 0.5);
    let door = g
        .level
        .data
        .elements
        .iter()
        .find(|e| e.is_enterable())
        .and_then(|e| e.door_cell())
        .unwrap();
    for spot in [
        at,
        cell_center(door + IVec2::new(0, -1)),
        cell_center(door + IVec2::new(0, 1)),
    ] {
        // a pose near the spot that is physically free
        let mut pose = None;
        for dx in -4..=4 {
            for dz in -4..=4 {
                let p = spot + Vec2::new(dx as f32, dz as f32) * 0.5;
                if !g.cart_pose_overlaps(p, Vec2::Y)
                    && pose.is_none_or(|(_, d)| p.distance(spot) < d)
                {
                    pose = Some((p, p.distance(spot)));
                }
            }
        }
        let (p, _) = pose.expect("a free pose near the spot");
        g.carts[0].pos = p;
        g.carts[0].dir = Vec2::Y;
        g.player.pos = p;
        g.drain_events();
        assert!(!g.parking_ok(p, Vec2::Y), "spot {spot}: parking refused");
        assert_eq!(g.leave_cart(), LeaveResult::NoPark);
        assert_eq!(g.seated, Some(0));
        assert!(g.drain_events().contains(&GameEvent::CartNoPark));
    }
    // back on the parking pose it works
    let (p, d) = (g.carts[0].home_pos, g.carts[0].home_dir);
    put_cart(&mut g, 0, p, d);
    assert_eq!(g.leave_cart(), LeaveResult::Left);
}

// a wedged cart can always be left (never stuck)
#[test]
fn cart_019_wedged_cart_can_be_left() {
    let mut g = with_key(game());
    board(&mut g, 0);
    g.carts[0].wedged_s = 10.0;
    let (p, d) = (g.carts[0].pos, g.carts[0].dir);
    // refused spot but wedged
    let it = g
        .level
        .data
        .items
        .iter()
        .find(|i| i.id == "key_box_l1")
        .unwrap()
        .pos();
    let _ = it;
    assert!(g.parking_ok(p, d));
    assert_eq!(g.leave_cart(), LeaveResult::Left);
    assert_eq!(g.cart_resets, 0);
}

// CART-028
#[test]
fn cart_028_headlights_only_while_driven_in_the_dark() {
    use zoo_core::daytime::Phase;
    let mut g = with_key(game());
    assert!(!g.cart_lights_on(0));
    board(&mut g, 0);
    assert!(!g.cart_lights_on(0), "day: off");
    g.daytime.force(Phase::Night);
    assert!(g.cart_lights_on(0), "night, driven: on");
    assert!(!g.cart_lights_on(1), "a parked cart: off");
    assert!(
        g.interactables().is_empty(),
        "no moon door / bed while seated"
    );
    assert_eq!(g.leave_cart(), LeaveResult::Left);
    assert!(!g.cart_lights_on(0));
}

// LAYOUT-048: the data of every [[cart]] of the joined zoo
#[test]
fn layout_048_cart_parking_data() {
    let g = common::zoo_game(2);
    let d = &g.level.data;
    let mut ids: Vec<&str> = d.carts.iter().map(|c| c.id.as_str()).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), d.carts.len(), "unique ids");
    // no night level has a cart
    assert!(common::zoo_with_night2()
        .carts
        .iter()
        .all(|c| !common::zoo_with_night2().is_night_part(c.part)));
    let grid = g.level.grid();
    for c in &d.carts {
        let near = |cell: IVec2, r: i32| {
            c.rect
                .cells()
                .any(|q| (q.x - cell.x).abs() <= r && (q.y - cell.y).abs() <= r)
        };
        for cell in c.rect.cells() {
            assert!(
                d.scenery.iter().all(|s| !s.rect.contains(cell)),
                "{}: scenery at {cell}",
                c.id
            );
            assert!(
                d.gardens.iter().all(|gd| !gd.rect.contains(cell)),
                "{}: garden at {cell}",
                c.id
            );
            for l in &d.lights {
                if let Some(p) = l.pos() {
                    assert_ne!(cell_of(p), cell, "{}: light post at {cell}", c.id);
                }
            }
        }
        // >= 1 cell (a free ring) from every gate, door, entry and barrier cell
        for e in &d.elements {
            use zoo_core::ElementType::*;
            if e.ty == Barrier {
                for cell in e.rect.cells() {
                    assert!(!near(cell, 1), "{}: barrier cell {cell} too close", c.id);
                }
            }
            if let Some(g) = e.gate {
                for cell in g.cells() {
                    assert!(!near(cell, 1), "{}: gate cell {cell} too close", c.id);
                }
            }
            if let Some(door) = e.door_cell() {
                assert!(!near(door, 1), "{}: door too close", c.id);
            }
        }
        for en in &d.entries {
            for cell in en.cells.cells() {
                assert!(!near(cell, 1), "{}: entry cell {cell} too close", c.id);
            }
        }
        // the sign stands on a free walkable cell outside the box
        let sign = cell_of(Vec2::from(c.sign_pos));
        assert!(
            matches!(grid.kind(sign), CellKind::Walkable(_)),
            "{}: sign cell",
            c.id
        );
        assert!(!box_shape(c.pos(), c.heading()).overlaps(Vec2::from(c.sign_pos), 0.1));
        assert!(grid.is_passable(c.stand_cell(), false), "{}: stand", c.id);
    }
}
