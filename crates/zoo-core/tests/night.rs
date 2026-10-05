//! GAME-NIGHT: nightfall, sleeping, the moon door, the night zoo `night_1` (NIGHT-001…009).

mod common;

use glam::Vec2;
use zoo_core::content::{riddle_key, Language, ReadingLevel};
use zoo_core::daytime::{Phase, CELEBRATION_S, DUSK_S, MORNING_S, SLEEP_S};
use zoo_core::game::{GameEvent, Target};
use zoo_core::level::cell_of;
use zoo_core::night::LANTERN_RADIUS_M;
use zoo_core::{AnimalState, Food, Game};

const DT: f32 = 1.0 / 60.0;
const DAY_1: [&str; 3] = ["zebra", "hippo", "panda"];
const NIGHT_1: [&str; 3] = ["hedgehog", "bat", "owl"];

fn run(g: &mut Game, seconds: f32) -> Vec<(f32, GameEvent)> {
    let mut out = Vec::new();
    let n = (seconds / DT).round() as usize;
    for k in 0..n {
        g.update(DT, Vec2::ZERO);
        out.extend(
            g.drain_events()
                .into_iter()
                .map(|e| ((k + 1) as f32 * DT, e)),
        );
    }
    out
}

/// Level 1 complete, then the game runs until full night.
fn night_game(seed: u64) -> Game {
    let mut g = common::night_game(seed);
    for a in DAY_1 {
        assert!(g.debug_send_home(a));
    }
    g.drain_events();
    run(&mut g, CELEBRATION_S + DUSK_S + 0.5);
    assert_eq!(g.daytime.phase, Phase::Night);
    g
}

fn door_walkable(g: &Game) -> bool {
    let r = g.level.data.element("moon_door").unwrap().rect;
    r.cells().all(|c| g.level.grid().is_walkable(c, false))
}

fn at_bed(g: &mut Game) {
    let bed = g.bed().expect("a bed in the zookeeper house").0;
    let stand = common_stand(g, bed);
    g.player.pos = stand;
    g.player.facing = (bed - stand).normalize_or(Vec2::Y);
}

/// A walkable cell centre 0.8–1.8 m from a point.
fn common_stand(g: &Game, p: Vec2) -> Vec2 {
    let grid = g.level.grid();
    zoo_core::Rect::new(p.x as i32 - 3, p.y as i32 - 3, 7, 7)
        .cells()
        .filter(|&c| grid.is_walkable(c, false))
        .map(zoo_core::level::cell_center)
        .filter(|q| (0.8..=1.8).contains(&q.distance(p)))
        .min_by(|a, b| a.distance(p).total_cmp(&b.distance(p)))
        .expect("a place to stand next to it")
}

#[test]
fn night_zoo_joins_with_the_day_zoo() {
    let z = common::zoo_with_night();
    let k = z.part_index("night_1").expect("night_1 joined");
    assert!(z.is_night_part(k) && !z.is_night_part(0));
    let g = common::night_game(1);
    for a in NIGHT_1 {
        let a = g.animal(a).expect("night animal");
        assert_eq!(a.state, AnimalState::Escaped);
        assert!(!g.in_scope(a), "the night zoo is closed by day");
    }
    assert_eq!(g.moon_doors(), ["moon_door"]);
}

// NIGHT-001: dusk starts once, after the celebration, and reaches night within 10–12 s.
#[test]
fn night_001_dusk_after_the_last_celebration() {
    let mut g = common::night_game(3);
    for a in ["zebra", "hippo"] {
        g.debug_send_home(a);
    }
    let ev = run(&mut g, 30.0);
    assert!(
        !ev.iter().any(|e| e.1 == GameEvent::DuskStarted),
        "not before the last animal"
    );
    g.debug_send_home("panda");
    let ev = g.drain_events();
    assert!(ev.contains(&GameEvent::LevelComplete {
        level: "level_1".into()
    }));
    let ev = run(&mut g, 40.0);
    let dusk = ev.iter().find(|e| e.1 == GameEvent::DuskStarted).unwrap().0;
    let night = ev.iter().find(|e| e.1 == GameEvent::NightFell).unwrap().0;
    assert!(dusk >= CELEBRATION_S - DT, "after the celebration: {dusk}");
    assert!((10.0..=12.0).contains(&(night - dusk)), "{}", night - dusk);
    assert_eq!(
        ev.iter().filter(|e| e.1 == GameEvent::DuskStarted).count(),
        1
    );
    // light: warm orange first, then deep blue
    assert_eq!(g.daytime.light().night, 1.0);
}

// NIGHT-002: before nightfall the moon door is closed; at night the door is open (walkable).
// The bed works by day too (NIGHT-033, user report 2026-10-04).
#[test]
fn night_002_moon_door_and_bed_at_night() {
    let mut g = common::night_game(4);
    assert!(!door_walkable(&g));
    at_bed(&mut g);
    assert_eq!(g.available_target(), Some(Target::Bed));
    let mut g = night_game(4);
    assert!(door_walkable(&g), "the moon door is open at night");
    assert!(g.level_unlocked("night_1"));
    at_bed(&mut g);
    assert_eq!(g.available_target(), Some(Target::Bed));
    assert!(g
        .interactables()
        .iter()
        .any(|i| matches!(i.target, Target::MoonDoor { .. })));
}

// NIGHT-003 (logic), NIGHT-010: the bed → sleep (saved) → the next morning in the day zoo with all day
// animals home; with the night zoo complete the fallen tree opens now (Q-078, `unlock_after =
// "night_1"`, Q-133), the moon door closes.
#[test]
fn night_003_sleep_until_the_next_morning() {
    let mut g = night_game(5);
    assert!(!g.level.is_barrier_open("barrier_ne_tree"));
    for a in NIGHT_1 {
        assert!(g.debug_send_home(a));
    }
    let ev = g.drain_events();
    assert!(ev.contains(&GameEvent::LevelComplete {
        level: "night_1".into()
    }));
    assert!(
        !ev.iter()
            .any(|e| matches!(e, GameEvent::BarrierOpened { .. })),
        "next morning"
    );
    at_bed(&mut g);
    let r = g.interact();
    assert_eq!(r, Some(zoo_core::Interaction::Sleep));
    let ev = g.drain_events();
    assert!(ev.contains(&GameEvent::SleepStarted));
    assert!(g.save_due(), "the game is saved (progress event)");
    g.mark_saved();
    let ev = run(&mut g, SLEEP_S + 0.1);
    assert!(ev.iter().any(|e| e.1 == GameEvent::Morning));
    assert!(ev.iter().any(|e| e.1
        == GameEvent::BarrierOpened {
            id: "barrier_ne_tree".into()
        }));
    assert!(g.save_due(), "autosave in the morning");
    assert!(g.level_unlocked("level_2"));
    assert!(!g.level_unlocked("night_1") && !door_walkable(&g));
    assert!(!g.player_in_night_zoo());
    for a in DAY_1 {
        assert_eq!(g.animal(a).unwrap().state, AnimalState::InEnclosure);
    }
    run(&mut g, MORNING_S + 0.1);
    assert_eq!(g.daytime.phase, Phase::Day);
    assert_eq!(g.daytime.light().night, 0.0);
}

// NIGHT-004 (logic): through the open moon door into night_1 (its missions in scope), and
// back to the day zoo at night.
#[test]
fn night_004_moon_door_into_the_night_zoo_and_back() {
    let mut g = night_game(6);
    let door = g.level.data.element("moon_door").unwrap().rect;
    // walk up to the door on the day side
    g.player.pos = Vec2::new(door.x as f32 + door.w as f32 + 0.6, door.z as f32 + 1.0);
    g.player.facing = Vec2::NEG_X;
    assert!(matches!(
        g.available_target(),
        Some(Target::MoonDoor { .. })
    ));
    let r = g.interact();
    assert_eq!(
        r,
        Some(zoo_core::Interaction::MoonDoor {
            into_night_zoo: true
        })
    );
    assert!(g.player_in_night_zoo());
    assert!(g.level.grid().is_walkable(cell_of(g.player.pos), false));
    // the night zoo's own missions are in scope
    let boards: Vec<&str> = g
        .interactables()
        .iter()
        .filter_map(|i| match i.target {
            Target::InfoBoard { animal } => Some(animal),
            _ => None,
        })
        .collect();
    for a in NIGHT_1 {
        assert!(boards.contains(&a), "{a}");
    }
    // walking (not only the button) works too: the entry cells are walkable
    let entry = g
        .level
        .data
        .entries
        .iter()
        .find(|e| e.barrier == "moon_door")
        .unwrap();
    assert!(entry
        .cells
        .cells()
        .all(|c| g.level.grid().is_walkable(c, false)));
    // back
    g.player.pos = Vec2::new(door.x as f32 - 0.6, door.z as f32 + 1.0);
    g.player.facing = Vec2::X;
    let r = g.interact();
    assert_eq!(
        r,
        Some(zoo_core::Interaction::MoonDoor {
            into_night_zoo: false
        })
    );
    assert!(!g.player_in_night_zoo());
    assert_eq!(g.daytime.phase, Phase::Night, "still night in the day zoo");
}

// NIGHT-006: eyes shine inside the lantern radius at night, not outside, never by day.
#[test]
fn night_006_eyes_shine_in_the_lantern_light() {
    let mut g = night_game(7);
    let owl = g.animal("owl").unwrap().pos;
    g.player.pos = owl + Vec2::new(LANTERN_RADIUS_M - 0.3, 0.0);
    assert!(g.eyes_shine(g.animal("owl").unwrap()));
    g.player.pos = owl + Vec2::new(LANTERN_RADIUS_M + 0.5, 0.0);
    assert!(!g.eyes_shine(g.animal("owl").unwrap()));
    g.player.pos = owl + Vec2::new(1.0, 0.0);
    g.debug_set_daytime("day");
    assert!(!g.eyes_shine(g.animal("owl").unwrap()), "never by day");
}

// NIGHT-007: every night animal has ≥ 3 hiding places, riddles for every reading level and
// language, and a food box with its food in the night zoo.
#[test]
fn night_007_night_animals_have_places_riddles_and_food() {
    let data = common::night1();
    let c = common::content();
    for a in NIGHT_1 {
        let places: Vec<_> = data.hiding_places_of(a).collect();
        assert!(places.len() >= 3, "{a}: {}", places.len());
        let info = zoo_core::animals::animal_info(a).unwrap();
        for f in info.foods {
            assert!(
                data.food_boxes.iter().any(|b| b.food == f.id()),
                "{a} {f:?}"
            );
        }
        for p in &places {
            for lang in Language::ALL {
                for level in ReadingLevel::ALL {
                    let key = riddle_key(a, &p.id, level);
                    assert!(c.text(lang, &key).is_some(), "{} {key}", lang.id());
                }
            }
        }
        for lang in Language::ALL {
            for key in [format!("animal-{a}"), format!("mission-{a}-home")] {
                assert!(c.text(lang, &key).is_some(), "{} {key}", lang.id());
            }
            for level in ReadingLevel::ALL {
                let key = zoo_core::content::facts_key(a, level);
                assert!(c.text(lang, &key).is_some(), "{} {key}", lang.id());
            }
        }
    }
    for f in Food::NIGHT {
        for lang in Language::ALL {
            assert!(c.text(lang, &f.label_key()).is_some(), "{f:?}");
        }
    }
}

// NIGHT-008: a save at night (day zoo or night zoo) restores the night, the area and the
// position; a save while sleeping wakes up the next morning.
#[test]
fn night_008_save_and_restore_at_night() {
    let mut g = night_game(8);
    let door = g.level.data.element("moon_door").unwrap().rect;
    g.player.pos = Vec2::new(door.x as f32 + door.w as f32 + 0.6, door.z as f32 + 1.0);
    g.player.facing = Vec2::NEG_X;
    g.interact();
    let pos = g.player.pos;
    let json = g.to_save().to_json();
    let r = Game::from_save_json(common::zoo_with_night(), &json).unwrap();
    assert_eq!(r.daytime.phase, Phase::Night);
    assert!(r.player_in_night_zoo());
    assert!(r.player.pos.distance(pos) < 1e-4);
    assert!(door_walkable(&r) && r.level_unlocked("night_1"));
    assert_eq!(r.to_save(), g.to_save());
    // sleeping → restored as the next morning
    let mut g = night_game(8);
    at_bed(&mut g);
    assert!(g.sleep());
    let json = g.to_save().to_json();
    let r = Game::from_save_json(common::zoo_with_night(), &json).unwrap();
    assert_eq!(r.daytime.phase, Phase::Day);
    assert!(
        !r.level_unlocked("night_1"),
        "the moon door is closed by day"
    );
    assert!(!r.player_in_night_zoo());
    // a day save (older saves have no daytime) is day
    let day = common::night_game(8).to_save();
    let mut v = serde_json::to_value(&day).unwrap();
    v.as_object_mut().unwrap().remove("daytime");
    let r = Game::from_save_json(common::zoo_with_night(), &v.to_string()).unwrap();
    assert_eq!(r.daytime.phase, Phase::Day);
}

// NIGHT-011, Q-079 / proposal Q-140: sleep any time — night-zoo progress is kept, a following night
// animal goes back to its place, level 2 stays closed while the night zoo is unfinished, and
// the bed works by day ("until the evening") so the next night — with the moon door open —
// comes when the child wants.
#[test]
fn q079_q140_night_zoo_every_night_until_complete() {
    let mut g = night_game(9);
    assert!(g.debug_send_home("owl"));
    let bat = g.animal_index("bat").unwrap();
    let place = g.animals[bat].hiding_place.clone();
    g.animals[bat].state = AnimalState::Following;
    at_bed(&mut g);
    assert!(g.sleep());
    run(&mut g, SLEEP_S + MORNING_S + 0.5);
    assert_eq!(g.daytime.phase, Phase::Day);
    assert!(
        !g.level_unlocked("level_2"),
        "the night zoo is not complete (Q-078)"
    );
    assert_eq!(g.animal("owl").unwrap().state, AnimalState::InEnclosure);
    let b = g.animal("bat").unwrap();
    assert_eq!(b.state, AnimalState::Escaped);
    assert_eq!(b.hiding_place, place);
    // by day the bed takes the child to the evening
    assert!(g.night_zoo_waiting());
    at_bed(&mut g);
    assert_eq!(g.available_target(), Some(Target::Bed));
    assert!(g.sleep());
    let ev = run(&mut g, SLEEP_S + DUSK_S);
    assert!(ev.iter().any(|e| e.1 == GameEvent::DuskStarted));
    assert!(ev.iter().any(|e| e.1 == GameEvent::NightFell));
    assert_eq!(g.daytime.phase, Phase::Night, "the next night");
    assert!(door_walkable(&g), "the moon door opens again");
    assert!(g.mission("owl").unwrap().complete);
    // finishing the night zoo, then sleeping: level 2 opens the next morning
    for a in ["hedgehog", "bat"] {
        assert!(g.debug_send_home(a));
    }
    assert!(!g.night_zoo_waiting());
    g.debug_next_morning();
    assert!(g.level_unlocked("level_2"));
}

// NIGHT-010: without the night level joined, the fallen tree follows its transition (level 1 → level 2)
// the next morning (the day-only zoo of the older tests).
#[test]
fn unlock_after_falls_back_to_the_transition() {
    let mut g = common::zoo_game(2);
    for a in DAY_1 {
        g.debug_send_home(a);
    }
    g.debug_next_morning();
    assert!(g.level.is_barrier_open("barrier_ne_tree"));
    let g = common::night_game(2);
    assert_eq!(g.level.barriers_unlocked_by("night_1"), ["barrier_ne_tree"]);
    assert!(g.level.barriers_unlocked_by("level_1").is_empty());
    assert!(g
        .level
        .exit_barriers_of("level_1")
        .contains(&"barrier_ne_tree".to_owned()));
    assert!(!g
        .level
        .exit_barriers_of("level_1")
        .contains(&"moon_door".to_owned()));
}

// NIGHT-015: day animals lie down at night; night animals wander more.
#[test]
fn day_animals_sleep_at_night() {
    let g = night_game(10);
    let z = g.animal("zebra").unwrap();
    assert_eq!(g.rest_clip(z), "sleep");
}

#[test]
fn debug_set_daytime_jumps() {
    let mut g = common::night_game(11);
    assert!(!g.debug_set_daytime("noon"));
    assert!(g.debug_set_daytime("night"));
    assert!(door_walkable(&g));
    assert!(g.debug_set_daytime("day"));
    assert!(!door_walkable(&g));
    assert_eq!(g.daytime.phase, Phase::Day);
}

// NIGHT-017: flying night animals perch / hang at their hiding place, fly `fly_height` above
// the ground while they follow, stand at home; eye_glow never while sleeping (proposal
// Q-146, answered).
#[test]
fn night_017_flyers_perch_hang_and_fly_and_sleeping_eyes_stay_dark() {
    use zoo_core::night::{eye_glow, flyer_pose};
    let anims = zoo_core::animals::AnimTable::from_toml_str(&common::read(
        "assets/models/animals/animal_anims.toml",
    ))
    .unwrap();
    let h = anims.fly_height("bat").expect("bat fly_height");
    assert!((h - 1.5).abs() < 1e-6);
    assert_eq!(anims.fly_height("owl"), Some(1.5));
    assert_eq!(anims.fly_height("hedgehog"), None);
    let p = flyer_pose(AnimalState::Escaped, Some(2.5), "hang", h);
    assert_eq!((p.rest, p.lift), ("hang", 2.5));
    let p = flyer_pose(AnimalState::Escaped, Some(6.0), "idle", h);
    assert_eq!((p.rest, p.lift), ("perch", 6.0));
    let p = flyer_pose(AnimalState::Following, Some(6.0), "idle", h);
    assert_eq!((p.rest, p.locomotion, p.lift), ("fly", "fly", 1.5));
    let p = flyer_pose(AnimalState::InEnclosure, None, "idle", h);
    assert_eq!((p.rest, p.lift), ("idle", 0.0));
    assert!(eye_glow(true, "idle", None));
    assert!(!eye_glow(false, "idle", None));
    assert!(!eye_glow(true, "sleep", None));
    assert!(!eye_glow(true, "idle", Some("sleep")));
    // every flyer clip exists in the models' clip table
    for a in ["bat", "owl"] {
        for c in ["perch", "fly", "sleep"] {
            assert!(anims.clip(a, c).is_some(), "{a}.{c}");
        }
    }
    assert!(anims.clip("bat", "hang").is_some());
}

// NIGHT-018 (Q-142 answered): the lantern's visible ground pool is as wide as the eyeshine
// radius (2.5 m).
#[test]
fn night_018_lantern_pool_matches_the_eyeshine_radius() {
    use zoo_core::night::{lantern_light_radius, LANTERN_LIGHT_Y_M};
    let r = lantern_light_radius();
    let pool = (r * r - LANTERN_LIGHT_Y_M * LANTERN_LIGHT_Y_M).sqrt();
    assert!((pool - LANTERN_RADIUS_M).abs() < 0.01, "pool {pool}");
    assert!((pool - 2.5).abs() < 0.05);
}

// NIGHT-016 (unit part; the key / touch button is the e2e test): the bed's stand point in
// the furnished bedroom is walkable, free of furniture colliders, reachable from the spawn,
// and standing there facing the bed makes the bed the interact target at night.
#[test]
fn night_016_bed_stand_is_free_and_reachable() {
    use zoo_core::collision::PLAYER_RADIUS_M;
    let mut g = night_game(1);
    let bed = g.bed().expect("a bed").0;
    let stand = g.bed_stand(bed).expect("a stand point");
    let cell = cell_of(stand);
    assert!(
        g.level.grid().is_passable(cell, false),
        "stand cell walkable"
    );
    assert!(
        !g.level.colliders().overlaps(stand, PLAYER_RADIUS_M),
        "no furniture on the stand point {stand}"
    );
    let spawn = g.level.data.spawn.cell();
    let reach = zoo_core::nav::flood_fill(g.level.grid(), spawn, false);
    assert!(
        reach[g.level.grid().index(cell).unwrap()],
        "reachable from the spawn"
    );
    g.player.pos = stand;
    g.player.facing = (bed - stand).normalize();
    assert_eq!(g.available_target(), Some(Target::Bed));
    // away from the bed: nothing bed-related
    g.player.pos = stand + Vec2::new(3.0, 0.0);
    assert_ne!(g.available_target(), Some(Target::Bed));
}

// LAYOUT-L2-018 (Q-141 answered; indoors since 2026-10-01, LAYOUT-047 in `beds_indoors.rs`):
// level 2 has its own bed `bed_l2` inside `zookeeper_house_2` on walkable cells, clear of
// hiding places and scenery; its stand cell is walkable, 1.0–1.5 m from the bed and reachable;
// with level 2 open the bed offered near level 2 is `bed_l2`.
#[test]
fn layout_l2_018_level_2_bed() {
    use zoo_core::collision::PLAYER_RADIUS_M;
    let data = common::zoo_with_night();
    let bed = data
        .items
        .iter()
        .find(|it| it.id == "bed_l2")
        .expect("bed_l2")
        .clone();
    assert_eq!(bed.kind, "bed");
    let k = data.part_index("level_2").unwrap();
    assert_eq!(bed.part, k);
    assert_eq!(bed.building.as_deref(), Some("zookeeper_house_2"));
    let mut g = zoo_core::Game::new(data.clone(), 3).unwrap();
    assert!(g.level.open_barrier("barrier_ne_tree"));
    assert!(g.level_unlocked("level_2"));
    // footprint 2 × 1 m on walkable cells outside hiding places and scenery
    for dx in [-0.75f32, 0.25, 0.75] {
        let c = cell_of(bed.pos() + Vec2::new(dx, 0.0));
        assert!(g.level.grid().is_walkable(c, false), "{c}");
        assert!(!data.hiding_places.iter().any(|h| h.rect.contains(c)));
        assert!(!data.scenery.iter().any(|s| s.rect.contains(c)));
    }
    // the stand point
    let spawn = data.parts[k].spawn.cell();
    let reach = zoo_core::nav::flood_fill(g.level.grid(), spawn, false);
    let stand_cell = glam::IVec2::from(bed.stand.expect("stand"));
    let stand = zoo_core::level::cell_center(stand_cell);
    let d = stand.distance(bed.pos());
    assert!((1.0..=1.5).contains(&d), "stand {d} m from the bed");
    assert!(
        reach[g.level.grid().index(stand_cell).unwrap()],
        "stand reachable"
    );
    assert!(!g.level.colliders().overlaps(stand, PLAYER_RADIUS_M));
    // near level 2: that bed is the one offered
    g.player.pos = zoo_core::level::cell_center(spawn);
    assert_eq!(g.bed_stand(g.player.pos), Some(stand));
    g.player.pos = stand;
    g.player.facing = (bed.pos() - stand).normalize();
    assert!(g.debug_set_daytime("night"));
    assert_eq!(g.available_target(), Some(Target::Bed));
}

/// NIGHT-033 (user report 2026-10-04): by day the bed can always be used and sleeping leads to
/// the evening — also when no night zoo waits; the 🧭 hint offers it by day only when sleeping
/// moves the game on (night zoo waiting or exits pending).
#[test]
fn night_033_sleeping_by_day_always_works() {
    let mut g = Game::new(common::zoo_with_night(), 5).unwrap();
    assert_eq!(g.daytime.phase, Phase::Day);
    assert!(!g.sleep_advances());
    at_bed(&mut g);
    assert!(g.bed_usable());
    assert_eq!(g.available_target(), Some(Target::Bed));
    let t = zoo_core::hints::HintTracker::default();
    assert!(zoo_core::hints::candidates(&g, &t)
        .iter()
        .all(|h| h.kind != zoo_core::hints::HintKind::Bed));
    assert!(g.sleep());
    run(&mut g, SLEEP_S + DUSK_S + 1.0);
    assert_eq!(g.daytime.phase, Phase::Night);
    // pending exits make the bed a by-day hint target
    let mut g = Game::new(common::zoo_with_night(), 5).unwrap();
    g.daytime.pending_exits.push("night_1".into());
    assert!(g.sleep_advances());
    assert!(zoo_core::hints::candidates(&g, &t)
        .iter()
        .any(|h| h.kind == zoo_core::hints::HintKind::Bed));
}

/// HINT-030: level 1 solved, the exits pending, by day → the target is the bed, never "all done".
#[test]
fn hint_030_solved_level_with_pending_exits_points_to_the_bed() {
    use zoo_core::hints::{candidates, what_next, HintKind, HintTracker};
    let mut g = Game::new(common::zoo_with_night(), 5).unwrap();
    for a in DAY_1 {
        assert!(g.debug_send_home(a));
    }
    run(&mut g, CELEBRATION_S + DUSK_S + 1.0);
    g.debug_set_daytime("day"); // the child is up again by day, the exits still pending
    g.daytime.pending_exits.push("night_1".into());
    let t = HintTracker::default();
    assert_eq!(what_next(&g, &t), None);
    assert_eq!(
        candidates(&g, &t).first().map(|h| h.kind),
        Some(HintKind::Bed)
    );
}

/// NIGHT-034: at night the baby of a day pair at home stands still (no route, no hop).
#[test]
fn night_034_babies_of_day_animals_sleep_at_night() {
    let id = "zebra";
    let g0 = common::night_game(12);
    let mut save = g0.to_save();
    save.babies = vec![id.to_string()];
    let mut g = Game::from_save(common::zoo_with_night(), &save).expect("restores");
    assert!(g.debug_send_home(id));
    for _ in 0..600 {
        g.update(DT, Vec2::ZERO);
    }
    assert!(g.debug_set_daytime("night"));
    for _ in 0..120 {
        g.update(DT, Vec2::ZERO);
    }
    let b = g.baby_states.get(id).expect("baby");
    assert!(b.route.is_empty());
    assert_eq!(b.play.hop, 0.0);
    assert_eq!(g.rest_clip(g.animal(id).unwrap()), "sleep");
    let p0 = b.pos;
    for _ in 0..300 {
        g.update(DT, Vec2::ZERO);
    }
    assert_eq!(g.baby_states.get(id).unwrap().pos, p0);
}
