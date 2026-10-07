//! ART-SOUND "Playback": cue decisions in zoo-core (ASND-010 … ASND-016).

mod common;

use glam::Vec2;
use zoo_core::game::GameEvent;
use zoo_core::level::Level;
use zoo_core::rng::Pcg32;
use zoo_core::scene::OpeningKind;
use zoo_core::sound::*;
use zoo_core::{AnimalState, Food};

fn names(reqs: Vec<CueRequest>) -> Vec<String> {
    reqs.into_iter().map(|r| r.cue).collect()
}

// ASND-010: the loudness chain stays quiet.
#[test]
fn asnd_010_gain_chain_is_quiet() {
    assert_eq!(MASTER_GAIN, 0.35);
    assert_eq!(group_of("ui_tap").gain(), 0.6);
    assert_eq!(group_of("step_grass").gain(), 0.5);
    assert_eq!(group_of("animal_zebra_call").gain(), 0.8);
    let mut all: Vec<String> = CUES.iter().map(|c| (*c).to_owned()).collect();
    all.extend(
        [
            "animal_zebra_call",
            "animal_koala_happy",
            "animal_hippo_refuse",
        ]
        .map(String::from),
    );
    for c in &all {
        let g = peak_gain(c);
        assert!(g > 0.0 && g <= MAX_PEAK_GAIN, "{c}: {g}");
        assert!((cue_gain(c, 0.0) - g).abs() < 1e-6);
    }
    assert!((peak_gain("step_path") - 0.175).abs() < 1e-6);
    assert!((peak_gain("ui_success") - 0.21).abs() < 1e-6);
    assert!((peak_gain("animal_zebra_call") - 0.28).abs() < 1e-6);
}

// ASND-011: one footfall per half cycle, none while standing.
#[test]
fn asnd_011_footfall_clock() {
    let mut c = FootfallClock::default();
    let dt = 1.0 / 60.0;
    let rate = 1.38;
    assert!(!c.advance(dt * rate, false));
    let mut steps = 0;
    for _ in 0..120 {
        if c.advance(dt * rate, true) {
            steps += 1;
        }
    }
    // 2 s × 1.38 = 2.76 clip seconds → steps at 0, 0.4, … 2.4 s = 7 footfalls
    assert_eq!(steps, 7);
    // standing: nothing, even for a long time
    for _ in 0..120 {
        assert!(!c.advance(dt * 0.8, false));
    }
    // a new walk starts with a step at once
    assert!(c.advance(dt * rate, true));
    assert!(!c.advance(dt * rate, true));
}

// ASND-012: the surface under the player.
#[test]
fn asnd_012_step_surface() {
    let data = common::level1();
    let level = Level::new(data.clone());
    let centre = |kind: &str| {
        let e = data
            .elements
            .iter()
            .find(|e| e.kind.as_deref() == Some(kind))
            .expect(kind);
        Vec2::new(
            e.rect.x as f32 + e.rect.w as f32 / 2.0,
            e.rect.z as f32 + e.rect.d as f32 / 2.0,
        )
    };
    assert_eq!(step_surface(&level, centre("bridge")), StepSurface::Wood);
    assert_eq!(step_surface(&level, centre("jetty")), StepSurface::Wood);
    let sand = data
        .hiding_places
        .iter()
        .find(|e| e.features.iter().any(|f| f == "sand"))
        .expect("sand landmark");
    let p = Vec2::new(
        sand.rect.x as f32 + sand.rect.w as f32 / 2.0,
        sand.rect.z as f32 + sand.rect.d as f32 / 2.0,
    );
    assert_eq!(step_surface(&level, p), StepSurface::Sand);
    // pool shallows: a pool cell of the hippo pool
    let pool = data
        .enclosure_features
        .iter()
        .find(|f| f.is_pool())
        .unwrap();
    let mut found = false;
    for x in pool.rect.x..pool.rect.x + pool.rect.w {
        for z in pool.rect.z..pool.rect.z + pool.rect.d {
            let q = Vec2::new(x as f32 + 0.5, z as f32 + 0.5);
            if zoo_core::wander::water_depth(&level, q) > 0.9 {
                assert_eq!(step_surface(&level, q), StepSurface::Water);
                found = true;
            }
        }
    }
    assert!(found);
    // path and grass cells exist and map to their cue
    let grid = level.grid();
    let mut seen = (false, false);
    for x in -10..60 {
        for z in -10..60 {
            let c = glam::IVec2::new(x, z);
            let q = Vec2::new(x as f32 + 0.5, z as f32 + 0.5);
            let s = step_surface(&level, q);
            match (grid.surface(c), s) {
                (Some(zoo_core::Surface::Path), StepSurface::Path) => seen.0 = true,
                (Some(zoo_core::Surface::Grass), StepSurface::Grass) => seen.1 = true,
                _ => {}
            }
        }
    }
    assert!(seen.0 && seen.1, "{seen:?}");
    assert_eq!(StepSurface::Sand.cue(), "step_sand");
    assert_eq!(StepSurface::Water.cue(), "step_water");
}

// ASND-013: events → cues.
#[test]
fn asnd_013_events_to_cues() {
    let z = |s: &str| s.to_owned();
    assert_eq!(
        names(cues_for_event(&GameEvent::FoodTaken {
            food: Food::Melons,
            returned: None
        })),
        ["pickup_food"]
    );
    assert_eq!(
        names(cues_for_event(&GameEvent::ItemPutDown {
            id: z("fish_bowl")
        })),
        ["drop_item"]
    );
    assert_eq!(
        names(cues_for_event(&GameEvent::ItemPutDown {
            id: z("food:carrot")
        })),
        ["drop_food"]
    );
    assert_eq!(
        names(cues_for_event(&GameEvent::Refuse {
            animal: z("zebra"),
            enclosure: z("enc_hippo")
        })),
        ["ui_refuse", "animal_zebra_refuse"]
    );
    let r = cues_for_event(&GameEvent::StartedFollowing { animal: z("zebra") });
    assert_eq!(r[0].cue, "animal_zebra_happy");
    assert_eq!(r[0].animal.as_deref(), Some("zebra"));
    assert_eq!(
        names(cues_for_event(&GameEvent::MissionComplete {
            animal: z("zebra")
        })),
        ["ui_success"]
    );
    assert!(cues_for_event(&GameEvent::NightFell).is_empty());
    assert!(cues_for_event(&GameEvent::Waiting { animal: z("zebra") }).is_empty());
    // seeded variation: same seed, same numbers; pitch within ±5 %
    let (mut a, mut b) = (Pcg32::new(7), Pcg32::new(7));
    for _ in 0..50 {
        let (va, ra) = variation(&mut a);
        assert_eq!((va, ra), variation(&mut b));
        assert!((0.95..=1.05).contains(&ra));
    }
}

// ASND-014: door cues.
#[test]
fn asnd_014_door_cues() {
    let b = OpeningKind::BuildingDoor {
        building: "house".into(),
        enterable: true,
    };
    let g = OpeningKind::EnclosureGate {
        enclosure: "enc".into(),
    };
    let glass = OpeningKind::GlassDoor {
        enclosure: "enc".into(),
    };
    let moon = OpeningKind::MoonDoor {
        barrier: "b".into(),
    };
    assert_eq!(door_cue(&b, true), "door_wood_open");
    assert_eq!(door_cue(&b, false), "door_wood_close");
    assert_eq!(door_cue(&g, true), "gate_open");
    assert_eq!(door_cue(&g, false), "gate_close");
    assert_eq!(door_cue(&glass, true), "glass_door");
    assert_eq!(door_cue(&glass, false), "glass_door");
    assert_eq!(door_cue(&moon, true), "moon_door");
    for k in [&b, &g, &glass, &moon] {
        for open in [true, false] {
            assert!(CUES.contains(&door_cue(k, open)));
        }
    }
}

// ASND-015: an escaped animal notices the player.
#[test]
fn asnd_015_animal_notices_player() {
    let mut game = common::game(17);
    let i = game
        .animals
        .iter()
        .position(|a| a.state == AnimalState::Escaped && game.in_scope(a))
        .expect("escaped animal in scope");
    let spot = game.animals[i].pos;
    let id = game.animals[i].id();
    let mut t = NoticeTracker::default();
    game.player.pos = spot + Vec2::new(30.0, 0.0);
    assert!(t.update(&game, 0.1).is_empty());
    game.player.pos = spot + Vec2::new(5.0, 0.0);
    assert_eq!(t.update(&game, 0.1), vec![id]);
    // out and back in within the cooldown: silent
    game.player.pos = spot + Vec2::new(30.0, 0.0);
    t.update(&game, 1.0);
    game.player.pos = spot + Vec2::new(5.0, 0.0);
    assert!(t.update(&game, 1.0).is_empty());
    // staying near: not again, even after the cooldown
    assert!(t.update(&game, 30.0).is_empty());
    // ... but again after 20 s out and back
    game.player.pos = spot + Vec2::new(30.0, 0.0);
    t.update(&game, 25.0);
    game.player.pos = spot + Vec2::new(5.0, 0.0);
    assert_eq!(t.update(&game, 0.1), vec![id]);
    // a following animal never calls
    for a in game.animals.iter_mut().filter(|a| a.id() == id) {
        a.state = AnimalState::Following;
    }
    game.player.pos = spot + Vec2::new(30.0, 0.0);
    t.update(&game, 30.0);
    game.player.pos = game.animals[i].pos;
    assert!(!t.update(&game, 0.1).contains(&id));
}

// ASND-016: distance law.
#[test]
fn asnd_016_distance_gain() {
    for (d, g) in [(0.0, 1.0), (2.0, 1.0), (8.5, 0.5), (15.0, 0.0), (40.0, 0.0)] {
        assert!((distance_gain(d) - g).abs() < 1e-5, "{d}");
    }
    assert!(cue_gain("step_path", 20.0) == 0.0);
}

// ASND-022: the night cricket bed (ambient loop) is quiet and follows the phase.
#[test]
fn asnd_022_ambient_target_follows_the_phase() {
    use zoo_core::daytime::Phase;
    const { assert!(AMBIENT_GAIN > 0.0 && AMBIENT_GAIN <= 0.05) };
    assert_eq!(AMBIENT_FADE_S, 3.0);
    for p in Phase::ALL {
        let t = ambient_target(p);
        match p {
            Phase::Dusk | Phase::Night => assert_eq!(t, AMBIENT_GAIN, "{p:?}"),
            _ => assert_eq!(t, 0.0, "{p:?}"),
        }
    }
    // the host clamp in web/src/audio.ts is the same number
    let ts = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../web/src/audio.ts"
    ))
    .unwrap();
    let line = ts
        .lines()
        .find(|l| l.contains("export const AMBIENT_GAIN_MAX"))
        .expect("AMBIENT_GAIN_MAX in audio.ts");
    let v: f32 = line
        .split('=')
        .nth(1)
        .unwrap()
        .trim()
        .trim_end_matches(';')
        .parse()
        .unwrap();
    assert_eq!(v, AMBIENT_GAIN);
}

const CART_CUES: [&str; 11] = [
    "cart_board",
    "cart_get_out",
    "cart_horn",
    "cart_bump",
    "cart_locked",
    "cart_park_refuse",
    "key_box_open",
    "key_pickup",
    "lock_wheel_tick",
    "lock_wrong",
    "lock_ok",
];

// ASND-031: groups and peak gains of the golf cart cues
#[test]
fn asnd_031_cart_cue_groups_and_peaks() {
    for c in CART_CUES {
        assert!(CUES.contains(&c), "{c} in CUES");
        let g = peak_gain(c);
        assert!(g > 0.0 && g <= MAX_PEAK_GAIN, "{c}: {g}");
    }
    for c in ["cart_board", "cart_horn", "cart_bump", "cart_park_refuse"] {
        assert_eq!(group_of(c), Group::Cart, "{c}");
        assert!((peak_gain(c) - 0.28).abs() < 1e-6);
    }
    for c in ["key_box_open", "key_pickup"] {
        assert_eq!(group_of(c), Group::Pickups, "{c}");
        assert!((peak_gain(c) - 0.245).abs() < 1e-6);
    }
    for c in ["lock_wheel_tick", "lock_wrong", "lock_ok"] {
        assert_eq!(group_of(c), Group::Ui, "{c}");
        assert!((peak_gain(c) - 0.21).abs() < 1e-6);
    }
}

// ASND-032: the events of the cue table
#[test]
fn asnd_032_cart_events_to_cues() {
    let z = |s: &str| s.to_owned();
    let one = |e: &GameEvent| {
        let mut r = cues_for_event(e);
        assert_eq!(r.len(), 1, "{e:?}");
        r.remove(0)
    };
    let r = one(&GameEvent::CartBoarded {
        id: z("cart_1"),
        followers: false,
    });
    assert_eq!(
        (r.cue.as_str(), r.cart.as_deref()),
        ("cart_board", Some("cart_1"))
    );
    let r = one(&GameEvent::CartLeft { id: z("cart_1") });
    assert_eq!(
        (r.cue.as_str(), r.cart.as_deref()),
        ("cart_get_out", Some("cart_1"))
    );
    for closed in [false, true] {
        assert_eq!(
            one(&GameEvent::CartLocked {
                closed_level: closed
            })
            .cue,
            "cart_locked"
        );
    }
    assert_eq!(one(&GameEvent::CartNoPark).cue, "cart_park_refuse");
    assert_eq!(one(&GameEvent::CartHorn).cue, "cart_horn");
    let r = one(&GameEvent::CartBump { strength: 0.6 });
    assert_eq!((r.cue.as_str(), r.strength), ("cart_bump", 0.6));
    let r = cues_for_event(&GameEvent::KeyBoxOpened);
    assert_eq!(names(r.clone()), ["key_box_open", "key_pickup"]);
    assert!(r.iter().all(|c| c.key_box));
    assert_eq!((r[0].delay, r[1].delay), (0.6, 1.2));
    // the wrong code has no core cue any more: the host plays lock_wrong with the shake
    assert!(cues_for_event(&GameEvent::WrongCode { tries: 1 }).is_empty());
    assert!(cues_for_event(&GameEvent::NoteRead).is_empty());
    // a seeded run gives the same variation numbers
    let (mut a, mut b) = (Pcg32::new(3), Pcg32::new(3));
    for _ in 0..20 {
        assert_eq!(variation(&mut a), variation(&mut b));
    }
}

// ASND-034: the engine law
#[test]
fn asnd_034_engine_gain_and_pitch() {
    assert!((engine_gain(0.0) - 0.07).abs() < 1e-6);
    assert!((engine_gain(4.5) - 0.28).abs() < 1e-6);
    assert!((engine_gain(20.0) - 0.28).abs() < 1e-6, "clamped above");
    assert!(
        (engine_gain(-1.5) - engine_gain(1.5)).abs() < 1e-9,
        "abs for the wedge reverse"
    );
    assert!((engine_rate(0.0) - 0.75).abs() < 1e-6);
    assert!((engine_rate(4.5) - 1.30).abs() < 1e-6);
    assert!((engine_rate(-1.5) - engine_rate(1.5)).abs() < 1e-9);
    let mut last = -1.0;
    for i in 0..=60 {
        let g = engine_gain(i as f32 * 0.1);
        assert!(g >= last && g <= ENGINE_GAIN_MAX + 1e-6);
        last = g;
    }
    let (p, g) = engine_layer_gains(2.0, 0.0);
    assert!((p - engine_gain(2.0)).abs() < 1e-6 && g == 0.0);
    let (p, g) = engine_layer_gains(2.0, 1.0);
    assert!(p == 0.0 && (g - 0.113).abs() < 2e-3, "{g}");
    assert!((engine_rate(2.0) - 0.994).abs() < 1e-2);
    let (p, g) = engine_layer_gains(4.5, 0.5);
    assert!((p - 0.14).abs() < 1e-6 && (g - 0.28 * 0.7 * 0.5).abs() < 1e-6);
    const { assert!(ENGINE_GAIN_MAX <= MAX_PEAK_GAIN) };
}

// ASND-035: the grass blend
#[test]
fn asnd_035_surface_blend() {
    assert_eq!(grass_blend_target(StepSurface::Path), 0.0);
    assert_eq!(grass_blend_target(StepSurface::Grass), 1.0);
    assert_eq!(grass_blend_target(StepSurface::Sand), 1.0);
    assert_eq!(grass_blend_target(StepSurface::Wood), 0.0);
    let mut b = 0.0;
    b = advance_blend(b, 1.0, 0.25);
    assert!((b - 0.5).abs() < 1e-6, "1 per 0.5 s");
    b = advance_blend(b, 1.0, 10.0);
    assert_eq!(b, 1.0);
    b = advance_blend(b, 0.0, 0.1);
    assert!((b - 0.8).abs() < 1e-6);
    assert_eq!(advance_blend(0.0, 0.0, 1.0), 0.0);
}
