//! FAM-011..013 (user report 2026-10-01): the baby is ALWAYS with the female — beside her at
//! the hiding place, while she follows the child, inside the fence once she is home, after a
//! save restore — and it takes treats like the adults.

mod common;

use glam::Vec2;
use zoo_core::game::Target;
use zoo_core::garden::Treat;
use zoo_core::level::{cell_center, cell_of};
use zoo_core::{AnimalState, Game};

const DT: f32 = 1.0 / 60.0;

fn run(g: &mut Game, seconds: f32) {
    for _ in 0..(seconds / DT) as usize {
        g.update(DT, Vec2::ZERO);
    }
}

/// A fresh game whose zebra pair is still out, restored from a save that already has the
/// baby flag (an older save / a pair re-opened by the pair migration).
fn escaped_pair_with_baby(seed: u64) -> Game {
    let g = common::night_game(seed);
    let mut s = g.to_save();
    s.babies = vec!["zebra".to_string()];
    let h = Game::from_save(common::zoo_with_night(), &s).expect("restores");
    assert!(h.animals.iter().any(|a| a.id() == "zebra" && a.member == 1));
    h
}

fn female(g: &Game) -> usize {
    g.group("zebra")
        .into_iter()
        .find(|&j| g.animals[j].member == 1)
        .unwrap()
}

fn inside_fence(g: &Game, p: Vec2) -> bool {
    let e = g.animal("zebra").unwrap().enclosure;
    let r = g.level.data.elements[e].rect;
    p.x > r.x as f32 && p.x < (r.x + r.w) as f32 && p.y > r.z as f32 && p.y < (r.z + r.d) as f32
}

// FAM-011: the baby of an escaped pair stands beside the female at the hiding place (not
// somewhere else) and, once the pair is led home, ends up inside the fence next to her.
#[test]
fn fam_011_baby_beside_the_escaped_female_then_inside_with_her() {
    let mut g = escaped_pair_with_baby(5);
    let f = female(&g);
    let b = g.baby_states.get("zebra").expect("a baby exists").clone();
    assert!(
        b.pos.distance(g.animals[f].pos) < 4.0,
        "baby beside the female at the hiding place: {:?} vs {:?}",
        b.pos,
        g.animals[f].pos
    );
    assert!(!inside_fence(&g, b.pos));
    assert!(g.debug_send_home("zebra"));
    run(&mut g, 10.0);
    let f = female(&g);
    assert_eq!(g.animals[f].state, AnimalState::InEnclosure);
    let b = g.baby_states.get("zebra").unwrap();
    assert!(
        inside_fence(&g, b.pos),
        "baby inside the fence: {:?}",
        b.pos
    );
    assert!(
        g.animals[f].wander_area().contains(cell_of(b.pos)),
        "baby in the female's area"
    );
}

// FAM-012: the baby keeps up with the female while she follows the child.
#[test]
fn fam_012_baby_follows_the_following_female() {
    let mut g = escaped_pair_with_baby(5);
    let f = female(&g);
    g.player.pos = g.animals[f].pos + Vec2::new(0.0, 1.0);
    g.player.facing = Vec2::new(0.0, -1.0);
    g.animals[f].state = AnimalState::Following;
    // the child walks to the garden-side fence of the zebra enclosure (outside)
    let e = g.animals[f].enclosure;
    let st = g.feed_spot(e).unwrap().stand;
    let start = g.animals[f].pos;
    // the child walks along the nav path to the feeding spot (outside the fence), 1.2 m/s
    let path = zoo_core::nav::find_path(g.level.grid(), cell_of(start), cell_of(st), true)
        .expect("a way to the enclosure");
    for k in 0..(60.0 / DT) as usize {
        let i = ((k as f32 * DT * 1.2) as usize + 2).min(path.len() - 1);
        g.player.pos = cell_center(path[i]);
        g.update(DT, Vec2::ZERO);
        let fp = g.animals[f].pos;
        let b = g.baby_states.get("zebra").unwrap();
        assert!(b.pos.distance(fp) < 12.0, "baby stays with her (t={k})");
    }
    let fp = g.animals[f].pos;
    assert!(
        fp.distance(start) > 3.0,
        "the female really walked: {start:?} -> {fp:?}"
    );
    let b = g.baby_states.get("zebra").unwrap();
    assert!(
        b.pos.distance(fp) < 5.0,
        "baby within the 5 m leash of the following female: {:?} vs {:?}",
        b.pos,
        fp
    );
}

// FAM-013: a baby that is somewhere outside while the pair is home is put inside (safety
// net: never left outside), and it can be given a treat like the adults.
#[test]
fn fam_013_baby_outside_while_home_is_brought_in_and_takes_treats() {
    let mut g = common::night_game(5);
    g.debug_send_home("zebra");
    g.drain_events();
    g.garden.basket.carrots = 4;
    assert_eq!(g.give_treat("zebra", Treat::Carrot), Some(true));
    // misplace the baby far outside
    let f = female(&g);
    let out = g.animals[f].pos + Vec2::new(0.0, 30.0);
    g.baby_states.get_mut("zebra").unwrap().pos = out;
    run(&mut g, 2.0);
    let b = g.baby_states.get("zebra").unwrap().clone();
    assert!(inside_fence(&g, b.pos), "baby back inside: {:?}", b.pos);
    // the child stands in front of the baby: the treat is offered
    g.player.pos = b.pos + Vec2::new(0.0, 0.6);
    g.player.facing = Vec2::new(0.0, -1.0);
    let it = g
        .interactables()
        .into_iter()
        .find(|i| i.target == Target::Treat { animal: "zebra" })
        .expect("treat target");
    let d_baby = it.point.distance(g.player.pos);
    let d_adults = g
        .group("zebra")
        .into_iter()
        .map(|j| g.animals[j].pos.distance(g.player.pos))
        .fold(f32::MAX, f32::min);
    assert!(
        d_baby <= d_adults + 1e-3,
        "the target point includes the baby (baby {d_baby}, adults {d_adults})"
    );
    let _ = cell_center(cell_of(b.pos));
}
