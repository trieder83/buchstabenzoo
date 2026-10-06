//! CART-018 NEVER STUCK (cart): seeded random stick sequences from each parking pose in the
//! joined zoo with animals and obstacles around, including a save / restore in the middle.

mod common;

use glam::Vec2;
use zoo_core::animals::AnimalState;
use zoo_core::cart::{distance_to_box, LeaveResult};
use zoo_core::level::{cell_center, cell_of};
use zoo_core::rng::Pcg32;
use zoo_core::Game;

const DT: f32 = 1.0 / 30.0;

fn prepare(seed: u64) -> Game {
    let mut g = common::zoo_game(seed);
    g.has_cart_key = true;
    g.key_box_open = true;
    for e in g.level.data.elements.clone() {
        if e.ty == zoo_core::ElementType::Barrier && e.kind.as_deref() != Some("moon_door") {
            g.level.open_barrier(&e.id);
        }
    }
    g
}

fn check(g: &Game, what: &str) {
    let k = g.seated.expect("seated");
    let c = &g.carts[k];
    assert!(
        !g.cart_pose_overlaps(c.pos, c.dir),
        "{what}: overlaps at {}",
        c.pos
    );
    assert!(
        !g.cart_cell_blocked(cell_of(c.pos)),
        "{what}: centre on a blocked cell"
    );
    assert_eq!(g.cart_resets, 0, "{what}: reset needed");
    assert!(
        c.speed >= 0.0 || c.reversing,
        "{what}: negative speed without the wedge escape"
    );
    for a in &g.animals {
        if a.state != AnimalState::InBowl {
            assert!(
                distance_to_box(c.pos, c.dir, a.pos) >= 0.29,
                "{what}: animal {} inside the box",
                a.id()
            );
        }
    }
}

// CART-018
#[test]
fn cart_018_never_stuck_fuzz() {
    for seed in 1..=3u64 {
        for cart in 0..3usize {
            let mut g = prepare(seed);
            let mut rng = Pcg32::new(seed * 77 + cart as u64);
            // animals and ducks around the cart's start
            let home = g.carts[cart].home_pos;
            for (n, a) in g.animals.iter_mut().enumerate().take(6) {
                let off = Vec2::new(rng.below(30) as f32 - 15.0, rng.below(30) as f32 - 15.0);
                a.pos = home + off;
                a.state = if n % 2 == 0 {
                    AnimalState::Escaped
                } else {
                    AnimalState::Following
                };
            }
            for _ in 0..4 {
                let off = Vec2::new(rng.below(24) as f32 - 12.0, rng.below(24) as f32 - 12.0);
                g.cart_obstacles.push((home + off, 0.3));
            }
            g.player.pos = cell_center(g.carts[cart].stand);
            g.player.facing = (home - g.player.pos).normalize();
            assert!(g.board_cart(cart));
            let mut input = Vec2::ZERO;
            let mut travelled = 0.0f32;
            for step in 0..2000 {
                if step % 75 == 0 {
                    input = match rng.below(6) {
                        0 => Vec2::ZERO,
                        _ => {
                            let a = rng.below(360) as f32 * std::f32::consts::PI / 180.0;
                            Vec2::new(a.cos(), a.sin()) * (0.4 + rng.below(7) as f32 / 10.0)
                        }
                    };
                }
                let before = g.carts[g.seated.unwrap()].pos;
                g.update(DT, input);
                travelled += g.carts[g.seated.unwrap()].pos.distance(before);
                check(&g, &format!("seed {seed} cart {cart} step {step}"));
                if step == 1000 {
                    // save / restore in the middle
                    let json = serde_json::to_string(&g.to_save()).unwrap();
                    let s = serde_json::from_str(&json).unwrap();
                    let obstacles = g.cart_obstacles.clone();
                    g = Game::from_save(common::zoo(), &s).expect("restore");
                    g.cart_obstacles = obstacles;
                    for e in g.level.data.elements.clone() {
                        if e.ty == zoo_core::ElementType::Barrier {
                            g.level.open_barrier(&e.id);
                        }
                    }
                    assert_eq!(g.seated, Some(cart), "seat restored");
                }
            }
            eprintln!("seed {seed} cart {cart}: drove {travelled:.0} m");
            assert!(travelled > 10.0, "it drove: {travelled}");
            // from every end state get-out works after a few more driven metres
            g.cart_obstacles.clear();
            let mut left = false;
            for k in 0..400 {
                if g.leave_cart() == LeaveResult::Left {
                    left = true;
                    break;
                }
                let a = (k / 45) as f32 * 1.9;
                g.update(DT, Vec2::new(a.cos(), a.sin()));
            }
            assert!(left, "seed {seed} cart {cart}: could not get out");
            assert_eq!(g.cart_resets, 0);
            assert!(g.seated.is_none());
        }
    }
}

// CART-018 (wedge): a cart nosed into the nook between the plaza bench and the rock hill cannot
// turn or reverse; held for 3 s it is lifted to a roomy pose and can drive on.
#[test]
fn cart_018_wedged_cart_is_rescued() {
    let mut g = prepare(1);
    g.player.pos = cell_center(g.carts[0].stand);
    assert!(g.board_cart(0));
    let dir = Vec2::new(0.8386707, 0.54463893);
    g.carts[0].pos = Vec2::new(7.679454, 6.1714706);
    g.carts[0].dir = dir;
    g.player.pos = g.carts[0].pos;
    assert!(!g.cart_pose_overlaps(g.carts[0].pos, dir));
    for _ in 0..(6.0 / DT) as usize {
        g.update(DT, -dir);
    }
    assert!(g.cart_rescues >= 1, "rescued");
    assert_eq!(g.cart_resets, 0);
    let before = g.carts[0].pos;
    for _ in 0..(3.0 / DT) as usize {
        g.update(DT, Vec2::new(0.0, 1.0));
    }
    assert!(
        g.carts[0].pos.distance(before) > 2.0,
        "drives on after the rescue"
    );
    let mut left = false;
    for k in 0..300 {
        if g.leave_cart() == LeaveResult::Left {
            left = true;
            break;
        }
        g.update(
            DT,
            Vec2::new(((k / 40) as f32 * 1.3).cos(), ((k / 40) as f32 * 1.3).sin()),
        );
    }
    assert!(left);
}
