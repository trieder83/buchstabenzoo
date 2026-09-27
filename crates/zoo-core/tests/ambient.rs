//! GAME-AMBIENT unit tests: ducks on the river, frogs at the pond (AMB-001…004, 006, 008).

mod common;

use glam::Vec2;
use zoo_core::ambient::{
    Ambient, AmbientKind, ADULT_DUCKS, DUCKLINGS, DUCK_DIP_S, DUCK_FLAP_S, DUCK_FLEE_TRIGGER_M,
    DUCK_MIN_GAP_M, DUCK_PREEN_S, FROG_CROAK_S, FROG_FLEE_TRIGGER_M, FROG_HOP_S,
};
use zoo_core::animals::AnimTable;
use zoo_core::level::{cell_of, Level};
use zoo_core::scene::LevelScene;
use zoo_core::LevelData;

const DT: f32 = 1.0 / 60.0;

fn ambient(data: &LevelData, seed: u64) -> (LevelScene, Ambient) {
    let scene = LevelScene::build(data);
    let amb = Ambient::new(data, &scene, seed);
    (scene, amb)
}

fn spawn(data: &LevelData) -> Vec2 {
    zoo_core::level::cell_center(data.spawn.cell())
}

fn ducks(a: &Ambient) -> impl Iterator<Item = (usize, &zoo_core::ambient::AmbientAnimal)> {
    a.animals
        .iter()
        .enumerate()
        .filter(|(_, x)| x.kind == AmbientKind::Duck)
}

// AMB-001: 2–4 ducks on the river, on open water, ≥ 0.6 m apart; the ducklings are in the
// water too. No ducks on the level-3 stream (riddle guard), frogs only at the pond.
#[test]
fn amb_001_ducks_on_the_river() {
    for data in [common::level1(), common::zoo()] {
        let (scene, a) = ambient(&data, 3);
        let n = ducks(&a).count();
        assert!((2..=4).contains(&n), "{n} ducks");
        assert_eq!(n, ADULT_DUCKS);
        let ducklings = a
            .animals
            .iter()
            .filter(|x| x.kind == AmbientKind::Duckling)
            .count();
        assert_eq!(ducklings, DUCKLINGS);
        for x in a.animals.iter().filter(|x| x.kind != AmbientKind::Frog) {
            assert!(
                scene.water.is_river_water(x.pos),
                "duck on land at {:?}",
                x.pos
            );
            let river = &scene.water.rivers[scene.water.river_of(cell_of(x.pos)).unwrap()];
            assert_eq!(river.ids[0], "river_n", "ducks only on the level-1 river");
        }
        let d: Vec<Vec2> = ducks(&a).map(|(_, x)| x.pos).collect();
        for i in 0..d.len() {
            for j in i + 1..d.len() {
                assert!(
                    d[i].distance(d[j]) >= DUCK_MIN_GAP_M,
                    "ducks {i}/{j} too close"
                );
            }
        }
        let frogs: Vec<_> = a
            .animals
            .iter()
            .filter(|x| x.kind == AmbientKind::Frog)
            .collect();
        assert_eq!(frogs.len(), 2);
        let pond = data.element("pond_water").unwrap().rect;
        assert!(frogs
            .iter()
            .all(|f| pond.contains(cell_of(f.pos)) && f.on_pad()));
    }
}

// AMB-002: in 60 s every duck swims, every adult plays dip/preen/flap, nobody leaves the water
// or touches a bridge pile or stone.
#[test]
fn amb_002_ducks_swim_act_and_stay_in_the_water() {
    let data = common::level1();
    let (scene, mut a) = ambient(&data, 11);
    let far = spawn(&data);
    let obstacles: Vec<_> = scene.water.obstacles.iter().filter(|o| o.river).collect();
    assert!(obstacles.len() >= 7, "bridge piles + stones: {obstacles:?}");
    for _ in 0..(60 * 60) {
        a.update(DT, far);
        for x in a.animals.iter().filter(|x| x.kind != AmbientKind::Frog) {
            assert!(
                scene.water.is_river_water(x.pos),
                "{:?} left the water",
                x.pos
            );
            for o in &obstacles {
                assert!(
                    x.pos.distance(o.pos) >= o.radius + 0.14,
                    "{:?} overlaps an obstacle at {:?}",
                    x.pos,
                    o.pos
                );
            }
        }
    }
    for x in a.animals.iter().filter(|x| x.kind != AmbientKind::Frog) {
        assert!(
            x.distance_swum > 1.0,
            "{:?} swam {}",
            x.kind,
            x.distance_swum
        );
        if x.kind == AmbientKind::Duck {
            assert!(x.actions_played >= 1);
        }
    }
}

// AMB-003: the player on the bank within 2.5 m of a duck → it swims ≥ 4 m away within 3 s.
#[test]
fn amb_003_ducks_flee_from_the_player() {
    let data = common::level1();
    let (scene, mut a) = ambient(&data, 5);
    let far = spawn(&data);
    for _ in 0..(8 * 60) {
        a.update(DT, far);
    }
    let level = Level::new(data.clone());
    for (k, _) in ducks(&a).map(|(k, x)| (k, x.pos)).collect::<Vec<_>>() {
        let mut b = a.clone();
        let x = &b.animals[k];
        let (s, c) = x.river_coords();
        let path = &scene.water.rivers[0];
        let side = if c >= 0.0 { 1.0 } else { -1.0 };
        let bank = path.point(s, side * (path.half_width + 0.5));
        assert!(
            level.grid().is_walkable(cell_of(bank), false),
            "bank {bank:?}"
        );
        assert!(bank.distance(x.pos) < DUCK_FLEE_TRIGGER_M);
        for _ in 0..(3 * 60) {
            b.update(DT, bank);
        }
        let d = b.animals[k].pos.distance(bank);
        assert!(d >= 4.0, "duck {k} only {d} m away after 3 s");
        assert!(scene.water.is_river_water(b.animals[k].pos));
    }
}

// AMB-004: same seed and inputs → identical behaviour.
#[test]
fn amb_004_deterministic() {
    let data = common::level1();
    let run = |seed: u64| {
        let (_, mut a) = ambient(&data, seed);
        let mut trace = Vec::new();
        for k in 0..(40 * 60) {
            let t = k as f32 * DT;
            let player = Vec2::new(-12.0 + t * 0.5, 23.0);
            a.update(DT, player);
            if k % 30 == 0 {
                trace.extend(a.animals.iter().map(|x| (x.pos, x.action.map(|y| y.clip))));
            }
        }
        trace
    };
    assert_eq!(run(9), run(9));
    assert_ne!(run(9), run(10));
}

// AMB-006: the player comes within 2 m of a frog (on the jetty) → it hops into the water.
#[test]
fn amb_006_frogs_hop_into_the_water() {
    let data = common::level1();
    let (scene, mut a) = ambient(&data, 2);
    let level = Level::new(data.clone());
    let jetty_tip = Vec2::new(-10.7, 22.4);
    assert!(level.grid().is_walkable(cell_of(jetty_tip), false));
    let (k, d) = a
        .animals
        .iter()
        .enumerate()
        .filter(|(_, x)| x.kind == AmbientKind::Frog)
        .map(|(k, x)| (k, x.pos.distance(jetty_tip)))
        .min_by(|p, q| p.1.total_cmp(&q.1))
        .unwrap();
    assert!(
        d < FROG_FLEE_TRIGGER_M,
        "nearest frog {d} m from the jetty tip"
    );
    for _ in 0..(3 * 60) {
        a.update(DT, jetty_tip);
    }
    let f = &a.animals[k];
    assert!(f.in_water && !f.on_pad(), "frog still on its pad");
    assert!(
        scene.water.is_water(f.pos),
        "frog in the water at {:?}",
        f.pos
    );
    // calm again (player gone): back on a pad
    let far = spawn(&data);
    let mut back = false;
    for _ in 0..(30 * 60) {
        a.update(DT, far);
        back |= a.animals[k].on_pad();
    }
    assert!(back, "frog climbed back onto a pad");
    // frogs croak and hop while sitting
    let played: u32 = a
        .animals
        .iter()
        .filter(|x| x.kind == AmbientKind::Frog)
        .map(|x| x.actions_played)
        .sum();
    assert!(played >= 4);
}

// AMB-008: at least one duck within 8 m of the bridge 80 % of the time over 5 min.
#[test]
fn amb_008_ducks_stay_near_the_bridge() {
    let data = common::level1();
    let b = data.element("bridge_river").unwrap().rect;
    let bridge = Vec2::new(b.x as f32 + 1.5, b.z as f32 + 1.5);
    for seed in [1, 2, 3] {
        let (_, mut a) = ambient(&data, seed);
        let player = spawn(&data);
        let (mut near, mut total) = (0, 0);
        for k in 0..(300 * 60) {
            a.update(DT, player);
            if k % 30 == 0 {
                total += 1;
                if ducks(&a).any(|(_, x)| x.pos.distance(bridge) <= 8.0) {
                    near += 1;
                }
            }
        }
        let share = near as f32 / total as f32;
        assert!(share >= 0.8, "seed {seed}: {share}");
    }
}

// AMB-010: clip durations used by the ambient logic match `animal_anims.toml`.
#[test]
fn amb_010_clip_durations_match_the_anim_table() {
    let t =
        AnimTable::from_toml_str(&common::read("assets/models/animals/animal_anims.toml")).unwrap();
    for (a, clips) in [
        ("duck", &["swim", "idle", "dip", "flap", "preen"][..]),
        ("duckling", &["swim", "idle", "dip", "flap", "preen"][..]),
        ("frog", &["idle", "croak", "hop", "swim"][..]),
    ] {
        for c in clips {
            assert!(t.clip(a, c).is_some(), "{a}.{c}");
        }
    }
    let dur = |a: &str, c: &str| t.clip(a, c).map(|x| x.frames as f32 / 30.0).unwrap();
    for (a, c, want) in [
        ("duck", "dip", DUCK_DIP_S),
        ("duck", "flap", DUCK_FLAP_S),
        ("duck", "preen", DUCK_PREEN_S),
        ("duckling", "dip", DUCK_DIP_S),
        ("frog", "croak", FROG_CROAK_S),
        ("frog", "hop", FROG_HOP_S),
    ] {
        assert!((dur(a, c) - want).abs() < 1e-5, "{a}.{c}");
    }
}

// AMB-009: butterflies flutter only over the meadow (the scenery that lists them,
// loc_meadow), within its area and at flower height — never in the garden.
#[test]
fn amb_009_butterflies_only_over_the_meadow() {
    use zoo_core::ambient::{BUTTERFLIES_PER_AREA, BUTTERFLY_HEIGHT_M};
    let data = common::zoo();
    let (_, mut a) = ambient(&data, 4);
    let meadow = data
        .scenery
        .iter()
        .find(|s| s.hiding_place.as_deref() == Some("loc_meadow"))
        .unwrap()
        .rect;
    assert_eq!(a.butterflies.len(), BUTTERFLIES_PER_AREA);
    let mut poses = Vec::new();
    let mut rested = false;
    let player = spawn(&data);
    for _ in 0..(30 * 60) {
        a.update(DT, player);
        a.butterfly_poses(&mut poses);
        for (b, p) in a.butterflies.iter().zip(&poses) {
            let l = Vec2::new(p.pos.x, -p.pos.z);
            assert!(
                meadow.distance_to(l) <= 0.5,
                "butterfly left the meadow: {l:?}"
            );
            assert!(
                (BUTTERFLY_HEIGHT_M.0..=BUTTERFLY_HEIGHT_M.1).contains(&p.pos.y),
                "height {}",
                p.pos.y
            );
            rested |= b.at(a.time).2;
        }
    }
    assert!(rested, "butterflies land on flowers now and then");
}

// NIGHT-013 (GAME-AMBIENT at night, art plan): ducks sleep (no swimming, `sleep` pose, no
// actions), butterflies are hidden, frogs croak more often than by day.
#[test]
fn night_013_ambient_at_night() {
    let data = common::level1();
    let player = spawn(&data);
    let croaks = |night: bool| {
        let (_, mut a) = ambient(&data, 5);
        a.night = night;
        let mut n = 0;
        let mut was = vec![false; a.animals.len()];
        for _ in 0..(120 * 60) {
            a.update(DT, player);
            for (i, x) in a.animals.iter().enumerate() {
                let c =
                    x.kind == AmbientKind::Frog && x.action.is_some_and(|act| act.clip == "croak");
                if c && !was[i] {
                    n += 1;
                }
                was[i] = c;
            }
        }
        n
    };
    assert!(croaks(true) > croaks(false), "frogs croak more at night");
    let (_, mut a) = ambient(&data, 5);
    a.night = true;
    let start: Vec<Vec2> = ducks(&a).map(|(_, d)| d.pos).collect();
    for _ in 0..(30 * 60) {
        a.update(DT, player);
    }
    for ((_, d), p) in ducks(&a).zip(start) {
        assert!(
            d.pos.distance(p) < 0.05,
            "a sleeping duck swam {}",
            d.pos.distance(p)
        );
        assert!(d.action.is_none());
    }
    let mut poses = Vec::new();
    a.poses(&mut poses);
    assert!(poses
        .iter()
        .filter(|p| p.model == "duck")
        .all(|p| p.idle_clip == "sleep"));
    let mut bf = Vec::new();
    a.butterfly_poses(&mut bf);
    assert!(bf.is_empty(), "no butterflies at night");
}
