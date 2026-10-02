//! FAM-014..018: the baby follows its mother playfully — a loose leash, loops and detours,
//! sniffing, bursts with hops — and never gets lost, clips or leaves the enclosure.

mod common;

use glam::Vec2;
use zoo_core::baby::{self, Mode};
use zoo_core::level::{cell_center, cell_of};
use zoo_core::{AnimalState, Game};

const DT: f32 = 1.0 / 60.0;

fn out_pair(seed: u64) -> Game {
    let g = common::night_game(seed);
    let mut s = g.to_save();
    s.babies = vec!["zebra".to_string()];
    Game::from_save(common::zoo_with_night(), &s).expect("restores")
}

fn female(g: &Game) -> usize {
    g.group("zebra")
        .into_iter()
        .find(|&j| g.animals[j].member == 1)
        .unwrap()
}

struct Stats {
    max_dist: f32,
    mean_dist: f32,
    max_lateral: f32,
    modes: Vec<Mode>,
    max_speed_factor: f32,
    mother_moved: f32,
}

/// The child walks the nav path back and forth (1.2 m/s) for `seconds`; the female follows.
fn walk_run(g: &mut Game, seconds: f32) -> Stats {
    let f = female(g);
    g.player.pos = g.animals[f].pos + Vec2::new(0.0, 1.0);
    g.animals[f].state = AnimalState::Following;
    let e = g.animals[f].enclosure;
    let st = g.feed_spot(e).unwrap().stand;
    let start = g.animals[f].pos;
    let path =
        zoo_core::nav::find_path(g.level.grid(), cell_of(start), cell_of(st), true).expect("a way");
    let n = path.len();
    assert!(n > 10);
    let path_speed = g.move_params.speed_on(zoo_core::level::Surface::Path);
    let mut s = Stats {
        max_dist: 0.0,
        mean_dist: 0.0,
        max_lateral: 0.0,
        modes: Vec::new(),
        max_speed_factor: 0.0,
        mother_moved: 0.0,
    };
    let (mut sum, mut cnt) = (0.0, 0);
    let mut s_along = 0.0f32;
    let mut prev_m = g.animals[f].pos;
    let mut prev_b = g.baby_states["zebra"].pos;
    let mut heading = Vec2::Y;
    for k in 0..(seconds / DT) as usize {
        s_along += DT * 1.2;
        // ping-pong along the path
        let period = (n - 1) as f32 * 2.0;
        let u = s_along % period;
        let i = if u <= (n - 1) as f32 { u } else { period - u };
        g.player.pos = cell_center(path[i as usize]);
        g.update(DT, Vec2::ZERO);
        let m = g.animals[f].pos;
        let b = g.baby_states.get("zebra").unwrap();
        let moved = m - prev_m;
        if moved.length() > 0.2 * DT {
            heading = moved.normalize();
        }
        s.mother_moved += moved.length();
        prev_m = m;
        let step = (b.pos - prev_b).length() / DT;
        s.max_speed_factor = s.max_speed_factor.max(step / path_speed);
        prev_b = b.pos;
        let d = b.pos.distance(m);
        s.max_dist = s.max_dist.max(d);
        if k > (5.0 / DT) as usize {
            sum += d;
            cnt += 1;
            s.max_lateral = s.max_lateral.max(((b.pos - m).dot(heading.perp())).abs());
        }
        if !s.modes.contains(&b.play.mode) {
            s.modes.push(b.play.mode);
        }
        // FAM-017: never on a blocked cell, hop within bounds
        let c = cell_of(b.pos);
        assert!(
            g.level.grid().is_passable(c, true),
            "baby on cell {c:?}, t={k}"
        );
        assert!((0.0..=baby::HOP_HEIGHT_M + 1e-4).contains(&b.play.hop));
        if b.play.mode != Mode::Burst && b.play.mode != Mode::Idle {
            assert_eq!(b.play.hop, 0.0);
        }
    }
    s.mean_dist = sum / cnt as f32;
    s
}

// FAM-014: a loose leash: never lost, mean distance 1.8..4 m, not a straight line behind her.
#[test]
fn fam_014_leash_and_detours_while_she_follows() {
    let mut g = out_pair(5);
    let s = walk_run(&mut g, 120.0);
    assert!(
        s.mother_moved > 100.0,
        "she really walked: {}",
        s.mother_moved
    );
    assert!(s.max_dist < 8.0, "never farther than 8 m: {}", s.max_dist);
    assert!(
        (1.8..4.0).contains(&s.mean_dist),
        "mean distance {}",
        s.mean_dist
    );
    assert!(s.max_lateral > 0.5, "lateral deviation {}", s.max_lateral);
    assert!(s.modes.len() >= 3, "varied behaviour: {:?}", s.modes);
    assert!(
        s.max_speed_factor <= baby::BURST_FACTOR + 0.05,
        "{}",
        s.max_speed_factor
    );
}

// FAM-015: at home the baby plays inside her area within a few metres of her.
#[test]
fn fam_015_plays_at_home_inside_the_enclosure() {
    let mut g = out_pair(5);
    g.debug_send_home("zebra");
    for _ in 0..(10.0 / DT) as usize {
        g.update(DT, Vec2::ZERO);
    }
    let f = female(&g);
    assert_eq!(g.animals[f].state, AnimalState::InEnclosure);
    let walk = g.move_params.speed_on(zoo_core::level::Surface::Path);
    let (mut sum, mut cnt, mut maxd) = (0.0, 0, 0.0f32);
    let mut modes = Vec::new();
    let mut prev = g.baby_states["zebra"].pos;
    for _ in 0..(120.0 / DT) as usize {
        g.update(DT, Vec2::ZERO);
        let f = female(&g);
        let b = &g.baby_states["zebra"];
        assert!(
            g.animals[f].wander_area().contains(cell_of(b.pos)),
            "inside her area: {:?}",
            b.pos
        );
        assert!(((b.pos - prev).length() / DT) <= walk * baby::BURST_FACTOR + 0.05);
        prev = b.pos;
        let d = b.pos.distance(g.animals[f].pos);
        sum += d;
        cnt += 1;
        maxd = maxd.max(d);
        if !modes.contains(&b.play.mode) {
            modes.push(b.play.mode);
        }
    }
    let mean = sum / cnt as f32;
    assert!(maxd < 6.0, "max distance {maxd}");
    assert!((1.8..4.0).contains(&mean), "mean distance {mean}");
    for m in [Mode::Roam, Mode::Sniff, Mode::Burst] {
        assert!(modes.contains(&m), "{m:?} used: {modes:?}");
    }
}

// FAM-016: deterministic; a far baby runs back; the 30 m safety net stays.
#[test]
fn fam_016_deterministic_and_never_lost() {
    let trace = |seed: u64| {
        let mut g = out_pair(seed);
        let mut v = Vec::new();
        let f = female(&g);
        g.player.pos = g.animals[f].pos + Vec2::new(0.0, 1.0);
        for k in 0..(60.0 / DT) as usize {
            g.update(DT, Vec2::ZERO);
            if k % 30 == 0 {
                let b = &g.baby_states["zebra"];
                v.push((b.pos.x.to_bits(), b.pos.y.to_bits(), b.play.mode));
            }
        }
        v
    };
    assert_eq!(trace(7), trace(7));
    assert_ne!(trace(7), trace(8));

    let mut g = out_pair(5);
    let f = female(&g);
    let m = g.animals[f].pos;
    // a baby ~12 m away on a walkable cell: runs back within 10 s
    let far = (6..30)
        .map(|d| m + Vec2::new(d as f32, 0.0))
        .find(|&p| p.distance(m) >= 12.0 && g.level.grid().is_passable(cell_of(p), false))
        .expect("a far cell");
    g.baby_states.get_mut("zebra").unwrap().pos = far;
    for _ in 0..(10.0 / DT) as usize {
        g.update(DT, Vec2::ZERO);
    }
    let f = female(&g);
    let d = g.baby_states["zebra"].pos.distance(g.animals[f].pos);
    assert!(d < baby::LEASH_SOFT_M, "ran back: {d}");
    // beyond 30 m: placed beside her at once
    let f = female(&g);
    g.baby_states.get_mut("zebra").unwrap().pos = g.animals[f].pos + Vec2::new(40.0, 0.0);
    g.update(DT, Vec2::ZERO);
    let f = female(&g);
    assert!(g.baby_states["zebra"].pos.distance(g.animals[f].pos) < 5.0);
}
