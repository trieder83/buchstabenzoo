//! Discovery (GAME-RESCUE §1: RESC-014…016, Q-082) and wandering animals (GAME-ANIMALS
//! "Animal states": ANIM-008…012) on level 1.

mod common;

use std::collections::{BTreeMap, BTreeSet};

use glam::{IVec2, Vec2};
use zoo_core::animals::AnimalState;
use zoo_core::game::pick_hiding_places;
use zoo_core::level::{cell_center, cell_of, CellKind, Surface};
use zoo_core::rng::Pcg32;
use zoo_core::wander::{home_area, AreaCell};
use zoo_core::Game;

const DT: f32 = 1.0 / 60.0;

fn idle(g: &mut Game, seconds: f32) {
    for _ in 0..(seconds / DT).round() as usize {
        g.update(DT, Vec2::ZERO);
    }
}

/// Puts the player far away from every animal (spawn plaza) so nobody notices her.
fn player_away(g: &mut Game) {
    g.player.pos = cell_center(g.level.data.spawn.cell());
}

// RESC-014: 1 000 seeds — every candidate is picked at least once, no two animals share a
// place and the chosen places of one playthrough are ≥ 12 m apart.
#[test]
fn resc_014_picks_cover_every_candidate_and_spread() {
    let data = common::level1();
    let mut seen: BTreeMap<String, usize> = BTreeMap::new();
    for seed in 0..1000u64 {
        let g = Game::new(data.clone(), seed).unwrap();
        let places: Vec<_> = g
            .animals
            .iter()
            .map(|a| data.hiding_place(&a.hiding_place).unwrap())
            .collect();
        for (i, a) in places.iter().enumerate() {
            assert_eq!(a.animal, g.animals[i].id());
            *seen.entry(a.id.clone()).or_default() += 1;
            for (j, b) in places.iter().enumerate().skip(i + 1) {
                if g.animals[i].id() == g.animals[j].id() {
                    continue; // the two of a pair share one hiding place (GAME-FAMILY §1)
                }
                assert_ne!(a.id, b.id);
                let d = a.spot().distance(b.spot());
                assert!(
                    d >= 12.0 - 1e-4,
                    "seed {seed}: {} – {} {d:.1} m",
                    a.id,
                    b.id
                );
            }
        }
    }
    for h in &data.hiding_places {
        let n = seen.get(&h.id).copied().unwrap_or(0);
        assert!(n > 0, "{} never picked", h.id);
    }
    assert_eq!(seen.len(), 9);
}

// Q-082: a new game avoids each animal's place of the previous game.
#[test]
fn resc_014_new_game_avoids_previous_places() {
    let data = common::level1();
    for seed in 0..200u64 {
        let first = Game::new(data.clone(), seed).unwrap();
        let avoid: BTreeMap<String, String> = first
            .animals
            .iter()
            .map(|a| (a.id().to_owned(), a.hiding_place.clone()))
            .collect();
        let next = Game::new_avoiding(data.clone(), seed + 7919, &avoid).unwrap();
        for a in &next.animals {
            assert_ne!(Some(&a.hiding_place), avoid.get(a.id()), "seed {seed}");
        }
    }
}

// Q-082: with no valid combination among the draws, the pick falls back to the first valid
// combination in data order (here: only one combination keeps the spots ≥ 12 m apart).
#[test]
fn resc_014_fallback_first_valid_combination() {
    let mut data = common::level1();
    // move every hippo candidate spot next to loc_river except loc_shade
    for h in data.hiding_places.iter_mut() {
        if h.animal == "hippo" && h.id != "loc_shade" {
            h.animal_spot = [8, 33];
        }
        if h.animal == "zebra" {
            h.animal_spot = [8, 32];
        }
    }
    let animals = ["zebra", "hippo", "panda"];
    for seed in 0..50u64 {
        let mut rng = Pcg32::new(seed);
        let picks = pick_hiding_places(&data, &animals, &mut rng, &BTreeMap::new()).unwrap();
        assert_eq!(picks[1], "loc_shade", "seed {seed}: {picks:?}");
    }
}

// RESC-016 / ANIM-011: a restored game keeps every animal at its chosen place (or wandering
// around it) and continues exactly like the game without the reload.
#[test]
fn resc_016_anim_011_restore_keeps_places_and_wandering() {
    let mut g = common::game(21);
    player_away(&mut g);
    idle(&mut g, 37.0);
    let json = g.to_save().to_json();
    let mut r = Game::from_save_json(common::level1(), &json).unwrap();
    for (a, b) in g.animals.iter().zip(&r.animals) {
        assert_eq!(a.hiding_place, b.hiding_place);
        assert_eq!(a.pos, b.pos);
        assert_eq!(a.facing, b.facing);
        assert_eq!(a.wander, b.wander);
        let place = g.level.data.hiding_place(&a.hiding_place).unwrap();
        assert!(b.pos.distance(place.spot()) <= 3.0 + 1e-3);
    }
    idle(&mut g, 45.0);
    idle(&mut r, 45.0);
    for (a, b) in g.animals.iter().zip(&r.animals) {
        assert_eq!(a.pos, b.pos, "{} diverged after restore", a.id());
        assert_eq!(a.wander, b.wander);
    }
}

// ANIM-011: same seed and inputs → identical wandering.
#[test]
fn anim_011_wandering_is_deterministic() {
    let run = || {
        let mut g = common::game(5);
        player_away(&mut g);
        g.debug_send_home("zebra");
        idle(&mut g, 90.0);
        g.animals.iter().map(|a| a.pos).collect::<Vec<_>>()
    };
    assert_eq!(run(), run());
}

/// Whether an escaped animal at `p` stands on its wander surface.
fn on_surface(g: &Game, wander_on: &str, c: IVec2) -> bool {
    let grid = g.level.grid();
    match wander_on {
        "water" => g
            .level
            .data
            .elements
            .iter()
            .any(|e| matches!(e.kind.as_deref(), Some("pond" | "river")) && e.rect.contains(c)),
        "cave" => grid.kind(c) == CellKind::Walkable(Surface::Path),
        _ => grid.kind(c) == CellKind::Walkable(Surface::Grass),
    }
}

// ANIM-008: escaped animals over 120 s move at least twice, never farther than 3 m from the
// spot, never onto another surface / solid cell or into a prop, never faster than 0.6 m/s.
#[test]
fn anim_008_escaped_animals_wander_within_their_place() {
    for seed in [1u64, 2, 3, 4, 11, 29] {
        let mut g = common::game(seed);
        player_away(&mut g);
        let n = g.animals.len();
        let mut walks = vec![0usize; n];
        let mut was_walking = vec![false; n];
        let mut prev: Vec<Vec2> = g.animals.iter().map(|a| a.pos).collect();
        for _ in 0..(120.0 / DT) as usize {
            g.update(DT, Vec2::ZERO);
            for (i, a) in g.animals.iter().enumerate() {
                let place = g.level.data.hiding_place(&a.hiding_place).unwrap();
                assert_eq!(a.state, AnimalState::Escaped);
                let d = a.pos.distance(place.spot());
                assert!(d <= 3.0 + 1e-3, "{} {d:.2} m from its spot", a.id());
                let c = cell_of(a.pos);
                assert!(
                    on_surface(&g, &place.wander_on, c) || c == place.spot_cell(),
                    "{} left its surface at {}",
                    a.id(),
                    a.pos
                );
                assert!(place.rect.contains(c), "{} outside {}", a.id(), place.id);
                assert!(
                    !g.level.colliders().overlaps(a.pos, 0.05),
                    "{} inside a prop at {}",
                    a.id(),
                    a.pos
                );
                let speed = a.pos.distance(prev[i]) / DT;
                assert!(speed <= 0.6 + 1e-3, "{} at {speed} m/s", a.id());
                let walking = a.is_wandering();
                if walking && !was_walking[i] {
                    walks[i] += 1;
                }
                was_walking[i] = walking;
                prev[i] = a.pos;
            }
        }
        for (i, a) in g.animals.iter().enumerate() {
            assert!(
                walks[i] >= 2,
                "seed {seed}: {} walked {} times",
                a.id(),
                walks[i]
            );
        }
    }
}

// ANIM-009: within 3 m of the player an escaped animal stops wandering and faces her.
#[test]
fn anim_009_stops_and_faces_the_player() {
    for seed in 0..12u64 {
        let mut g = common::game(seed);
        player_away(&mut g);
        idle(&mut g, 20.0);
        for k in 0..g.animals.len() {
            let a = g.animals[k].clone();
            // a walkable standing point within 1.6 m of the animal
            let stand = g
                .level
                .data
                .level
                .bounds
                .cells()
                .filter(|&c| g.level.grid().is_passable(c, false))
                .map(cell_center)
                .filter(|p| (0.8..=1.6).contains(&p.distance(a.pos)))
                .min_by(|x, y| x.distance(a.pos).total_cmp(&y.distance(a.pos)));
            let Some(stand) = stand else { continue };
            g.player.pos = stand;
            g.update(DT, Vec2::ZERO);
            let before = g.animals[k].pos;
            idle(&mut g, 16.0);
            let after = &g.animals[k];
            assert_eq!(
                after.pos,
                before,
                "seed {seed}: {} kept wandering",
                after.id()
            );
            let to = (g.player.pos - after.pos).normalize();
            assert!(
                after.facing.dot(to) > 0.99,
                "{} does not face the player",
                after.id()
            );
            player_away(&mut g);
        }
    }
}

// Q-097 (proposal): an animal out of reach (far out in the pond) comes to the player when
// she stands at the shore within 5 m, so the food can always be shown. Since FIX-056 the
// loc_pond wander area is the west half of the pond (22 m haze rule), shown from the north
// shore next to the ring path.
#[test]
fn escaped_hippo_in_the_pond_comes_to_the_shore() {
    let seed = (0..200u64)
        .find(|&s| common::game(s).animal("hippo").unwrap().hiding_place == "loc_pond")
        .unwrap();
    let mut g = common::game(seed);
    player_away(&mut g);
    // push the hippo to the far west part of its area
    let far = g
        .animal("hippo")
        .unwrap()
        .wander_area()
        .cells()
        .map(|(c, _)| c)
        .min_by_key(|c| c.x)
        .unwrap();
    let i = g.animal_index("hippo").unwrap();
    g.animals[i].pos = cell_center(far);
    g.player.pos = Vec2::new(-15.5, 28.5); // north shore, beside the ring path
    g.player.facing = Vec2::NEG_Y;
    idle(&mut g, 12.0);
    // a pair (Q-308): at least one of the two hippos comes within reach (they keep a gap)
    let d = g
        .group("hippo")
        .iter()
        .map(|&j| g.animals[j].pos.distance(g.player.pos))
        .fold(f32::INFINITY, f32::min);
    assert!(d <= 2.0, "hippo stays {d:.2} m away");
    assert_eq!(
        g.available_target(),
        Some(zoo_core::Target::Animal { animal: "hippo" })
    );
}

// ANIM-010: animals at home wander inside their enclosure only and never stand on a gate.
#[test]
fn anim_010_home_animals_wander_inside_the_enclosure() {
    let mut g = common::game(3);
    player_away(&mut g);
    for a in ["zebra", "hippo", "panda"] {
        assert!(g.debug_send_home(a));
    }
    // stay in daylight (at night the animals lie down, GAME-NIGHT rule 1)
    g.daytime = Default::default();
    let mut moved = BTreeSet::new();
    let start: Vec<Vec2> = g.animals.iter().map(|a| a.pos).collect();
    for _ in 0..(120.0 / DT) as usize {
        g.update(DT, Vec2::ZERO);
        for (i, a) in g.animals.iter().enumerate() {
            let enc = &g.level.data.elements[a.enclosure];
            let c = cell_of(a.pos);
            assert!(
                enc.rect.contains(c),
                "{} left its enclosure at {}",
                a.id(),
                a.pos
            );
            assert!(
                !enc.gate.unwrap().contains(c),
                "{} on a gate cell {c}",
                a.id()
            );
            assert!(
                a.wander_area().contains(c),
                "{} outside its home area",
                a.id()
            );
            if a.pos.distance(start[i]) > 0.5 {
                moved.insert(a.id());
            }
        }
    }
    assert_eq!(moved.len(), 3, "not every animal wandered: {moved:?}");
}

// ANIM-012: the hippo at home over 600 s stays in its home area, crosses between grass and
// pool only over the ramp and spends more than half of the time on pool cells.
#[test]
fn anim_012_hippo_uses_the_pool_over_the_ramp() {
    let mut g = common::game(8);
    player_away(&mut g);
    assert!(g.debug_send_home("hippo"));
    let i = g.animal_index("hippo").unwrap();
    let area = home_area(&g.level, g.animals[i].enclosure);
    let mut prev = area.class(cell_of(g.animals[i].pos)).unwrap();
    let (mut pool, mut total) = (0usize, 0usize);
    let mut crossings = 0;
    for _ in 0..(600.0 / DT) as usize {
        g.update(DT, Vec2::ZERO);
        let c = cell_of(g.animals[i].pos);
        let k = area
            .class(c)
            .unwrap_or_else(|| panic!("hippo left its area at {c}"));
        assert!(
            !(prev == AreaCell::Land && k == AreaCell::Water
                || prev == AreaCell::Water && k == AreaCell::Land),
            "hippo crossed the rim at {c}"
        );
        if k != prev {
            crossings += 1;
        }
        prev = k;
        total += 1;
        if k != AreaCell::Land {
            pool += 1;
        }
    }
    let share = pool as f32 / total as f32;
    assert!(
        share > 0.5,
        "only {:.0} % of the time in the pool",
        share * 100.0
    );
    assert!(crossings >= 2, "never walked in and out ({crossings})");
}
