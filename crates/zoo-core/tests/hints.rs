//! GAME-HINT (HINT-001…006, HINT-008, HINT-010…012, HINT-014) and the night progress of
//! GAME-NIGHT (NIGHT-019, NIGHT-021 core part): the next-target hint and "what is still
//! missing before night falls".

mod common;

use glam::{IVec2, Vec2, Vec4};
use zoo_core::animals::AnimalState;
use zoo_core::daytime::{Phase, CELEBRATION_S, DUSK_S};
use zoo_core::game::{STALL_HELP_S, STALL_WALK_S};
use zoo_core::hints::{
    self, candidates, compass_badge, night_progress, HintKind, HintTracker, ProgressState,
    AREA_HINT_AFTER_S, AREA_MIN_DIAMETER_M, HINT_SHOW_S, IDLE_NUDGE_S, WIDE_AREA_MIN_DIAMETER_M,
};
use zoo_core::level::{cell_center, cell_of};
use zoo_core::rng::Pcg32;
use zoo_core::{nav, Food, Game, GameEvent};

const DT: f32 = 1.0 / 60.0;
const DAY_1: [&str; 3] = ["zebra", "hippo", "panda"];

/// The joined zoo with the night level, parsed once.
fn zoo_data() -> zoo_core::LevelData {
    static DATA: std::sync::OnceLock<zoo_core::LevelData> = std::sync::OnceLock::new();
    DATA.get_or_init(common::zoo_with_night).clone()
}

fn night_game(seed: u64) -> Game {
    Game::new(zoo_data(), seed).expect("zoo with night starts")
}

/// A game with its hint tracker, stepped together like the host does.
struct Sim {
    g: Game,
    t: HintTracker,
    events: Vec<GameEvent>,
}

impl Sim {
    fn new(g: Game) -> Self {
        Self {
            g,
            t: HintTracker::default(),
            events: Vec::new(),
        }
    }

    fn tick(&mut self, dir: Vec2) {
        self.g.update(DT, dir);
        for e in self.g.drain_events() {
            self.t.observe(&e);
            self.events.push(e);
        }
        self.t.update(&self.g, DT);
    }

    fn idle(&mut self, s: f32) {
        for _ in 0..(s / DT).round() as usize {
            self.tick(Vec2::ZERO);
        }
    }

    fn press(&mut self) -> hints::Hint {
        self.t.press(&self.g).expect("a hint target").clone()
    }

    /// Walks along the grid path to a cell with joystick input (real movement/collision).
    /// Ends at the cell centre, or — when a collider in the cell (a garden fence, a prop)
    /// keeps her from the centre — where she stops inside the target cell.
    fn walk_to(&mut self, target: IVec2, timeout: f32) {
        let mut t = 0.0;
        let mut path: Vec<IVec2> = Vec::new();
        let mut blocked = 0.0;
        while cell_of(self.g.player.pos) != target
            || self.g.player.pos.distance(cell_center(target)) > 0.05
        {
            let before = self.g.player.pos;
            let here = cell_of(self.g.player.pos);
            let next = if here == target {
                target
            } else {
                if !path.contains(&here) {
                    let leading = self.g.is_leading();
                    let grid = self.g.level.grid();
                    let start = if grid.is_walkable(here, leading) {
                        here
                    } else {
                        nav::neighbours(grid, here, leading)
                            .next()
                            .map(|(c, _)| c)
                            .unwrap_or(here)
                    };
                    path = nav::find_path(grid, start, target, leading).unwrap_or_else(|| {
                        panic!("no path to {target} from {}", self.g.player.pos)
                    });
                    if start != here {
                        path.insert(0, here);
                    }
                }
                let k = path.iter().position(|&c| c == here).unwrap();
                path[(k + 1).min(path.len() - 1)]
            };
            let dir = (cell_center(next) - self.g.player.pos).normalize_or_zero();
            self.tick(dir);
            t += DT;
            if self.g.player.pos.distance(before) < 1e-4 {
                blocked += DT;
                if blocked > if here == target { 0.5 } else { 3.0 } {
                    break;
                }
            } else {
                blocked = 0.0;
            }
            assert!(
                t < timeout,
                "walk to {target} timed out at {}",
                self.g.player.pos
            );
        }
    }

    fn face(&mut self, p: Vec2) {
        let to = p - self.g.player.pos;
        if to.length() > 1e-3 {
            self.g.player.facing = to.normalize();
        }
    }

    /// Walks up to a (wandering) escaped animal until it is within interaction range.
    /// Position of the escaped member of a group nearest to the player (else the first one).
    fn member_pos(&self, animal: &str) -> Vec2 {
        let p = self.g.player.pos;
        self.g
            .group(animal)
            .into_iter()
            .map(|i| &self.g.animals[i])
            .filter(|x| x.state == AnimalState::Escaped)
            .map(|x| x.pos)
            .min_by(|a, b| a.distance(p).total_cmp(&b.distance(p)))
            .unwrap_or_else(|| self.g.animal(animal).unwrap().pos)
    }

    fn reach_animal(&mut self, animal: &str) {
        for _ in 0..8 {
            let s = self.member_pos(animal);
            let grid = self.g.level.grid();
            let cell = self
                .g
                .level
                .data
                .level
                .bounds
                .cells()
                .chain(zoo_core::Rect::new(s.x as i32 - 6, s.y as i32 - 6, 13, 13).cells())
                .filter(|&c| grid.is_passable(c, false))
                .min_by(|a, b| {
                    cell_center(*a)
                        .distance(s)
                        .total_cmp(&cell_center(*b).distance(s))
                })
                .unwrap();
            self.walk_to(cell, 150.0);
            for _ in 0..120 {
                let to = self.member_pos(animal) - self.g.player.pos;
                if to.length() < 1.7 {
                    break;
                }
                self.tick(to.normalize());
            }
            if self.member_pos(animal).distance(self.g.player.pos) <= 2.0 {
                return;
            }
            self.idle(1.0);
            if self.member_pos(animal).distance(self.g.player.pos) <= 2.0 {
                return;
            }
        }
        panic!("could not reach the {animal}");
    }
}

/// One step of the child following the hint.
fn follow_one(s: &mut Sim) {
    let h = s.press();
    let animal = h.animal;
    match h.kind {
        HintKind::Board => {
            let a = animal.expect("board of a mission");
            let started = s.g.mission(a).unwrap().started;
            let right =
                s.g.carry
                    .food()
                    .is_some_and(|f| s.g.animal(a).unwrap().info.eats(f));
            if started && right {
                // the child read the riddle: go and find the animal, show the food
                s.reach_animal(a);
                s.face(s.member_pos(a));
                s.g.show_food(a).expect("in range");
                s.idle(0.2);
            } else {
                s.walk_to(cell_of(h.stand), 150.0);
                s.face(h.pos);
                s.idle(0.6); // the panel opens by itself and starts the mission
                assert!(s.g.mission(a).unwrap().started, "board of {a} read");
                s.g.close_panel();
            }
        }
        HintKind::Food => {
            // the board said which food: read the box labels, take that one
            let a = animal.expect("food for a mission");
            let food = s.g.animal(a).unwrap().info.foods[0];
            let &(_, pos, facing) =
                s.g.food_boxes
                    .iter()
                    .filter(|b| b.0 == food)
                    .min_by(|x, y| x.1.distance(h.pos).total_cmp(&y.1.distance(h.pos)))
                    .expect("a box with the food");
            s.walk_to(cell_of(pos + facing * 1.1), 150.0);
            s.g.take_food(food).expect("at the box");
            s.g.close_panel();
            s.idle(0.1);
        }
        HintKind::Animal => {
            let a = animal.expect("area of a mission");
            s.walk_to(cell_of(h.stand), 150.0);
            s.reach_animal(a);
            s.face(s.g.animal(a).unwrap().pos);
            s.g.show_food(a).expect("in range");
            s.idle(0.2);
        }
        HintKind::Help => {
            // the animal itself: walk up, show the food it likes (the board's food)
            let a = animal.expect("help for a mission");
            s.walk_to(cell_of(h.stand), 150.0);
            s.reach_animal(a);
            s.face(s.g.animal(a).unwrap().pos);
            let _ = s.g.show_food(a);
            s.idle(0.2);
        }
        HintKind::Gate => {
            let a = animal.expect("gate of a mission");
            s.walk_to(cell_of(h.stand), 150.0);
            s.idle(2.0);
            let gate = s.g.level.data.elements[s.g.animal(a).unwrap().enclosure]
                .gate
                .unwrap()
                .cells()
                .next()
                .unwrap();
            s.walk_to(gate, 30.0);
            s.idle(0.5);
        }
        HintKind::PickUp
        | HintKind::Bamboo
        | HintKind::Water
        | HintKind::Bed
        | HintKind::MoonDoor
        | HintKind::Garden
        | HintKind::Potato
        | HintKind::Apple
        | HintKind::Orange
        | HintKind::Treat => {
            s.walk_to(cell_of(h.stand), 150.0);
            s.face(h.pos);
            s.idle(0.1);
            s.g.interact();
            s.idle(0.2);
        }
    }
}

/// Plays like a child who only follows the 🧭 hint (and reads: the board tells the food and
/// where the animal hides). Returns after `until` holds or panics after `max_steps` hints.
fn follow_hints(s: &mut Sim, max_steps: usize, until: impl Fn(&Sim) -> bool) {
    for step in 0..max_steps {
        if until(s) {
            return;
        }
        follow_one(s);
        assert!(
            step + 1 < max_steps,
            "the hints did not lead there in {max_steps} steps"
        );
    }
    assert!(until(s), "the hints did not lead there");
}

fn day_done(s: &Sim) -> bool {
    DAY_1.iter().all(|a| s.g.mission(a).unwrap().complete)
}

// HINT-001
#[test]
fn hint_001_new_game_nearest_unread_board() {
    for seed in [1, 17] {
        let mut s = Sim::new(night_game(seed));
        let h = s.press();
        assert_eq!(h.kind, HintKind::Board);
        let p = s.g.player.pos;
        // the nearest of the three level-1 boards
        let d = DAY_1
            .iter()
            .map(|a| {
                candidates(&s.g, &s.t)
                    .into_iter()
                    .find(|c| c.id == format!("board:{a}"))
                    .unwrap()
                    .pos
                    .distance(p)
            })
            .fold(f32::MAX, f32::min);
        assert!((h.pos.distance(p) - d).abs() < 1e-3, "{h:?}");
        assert!(!s.g.mission(h.animal.unwrap()).unwrap().started);
    }
}

fn read_board(s: &mut Sim, animal: &str) {
    let h = candidates(&s.g, &s.t)
        .into_iter()
        .find(|c| c.id == format!("board:{animal}"))
        .expect("board hint");
    s.walk_to(cell_of(h.stand), 150.0);
    s.face(h.pos);
    s.idle(0.6);
    assert!(s.g.mission(animal).unwrap().started);
    s.g.close_panel();
}

// HINT-002
#[test]
fn hint_002_board_read_no_food_then_food_storage() {
    let mut s = Sim::new(night_game(17));
    read_board(&mut s, "zebra");
    let h = s.press();
    assert_eq!(h.kind, HintKind::Food, "{h:?}");
    assert_eq!(h.animal, Some("zebra"));
    // at the level-1 storage: its door (the gap in the box row, Q-187), not the grass box
    let grass =
        s.g.food_boxes
            .iter()
            .find(|b| b.0 == Food::Grass)
            .unwrap()
            .1;
    assert!(h.pos.distance(grass) > 0.5, "never the right box itself");
    assert!(s.g.food_boxes.iter().any(|b| b.1.distance(h.pos) < 4.0));
}

// HINT-003
#[test]
fn hint_003_wide_area_first_then_exact_circle_after_60_s() {
    let mut s = Sim::new(night_game(17));
    read_board(&mut s, "zebra");
    let &(_, pos, facing) = s.g.food_boxes.iter().find(|b| b.0 == Food::Grass).unwrap();
    s.walk_to(cell_of(pos + facing * 1.1), 60.0);
    s.g.take_food(Food::Grass).unwrap();
    assert!(s.t.search_s("zebra") < AREA_HINT_AFTER_S);
    // HINT-015 (rule 4a): no waiting time — the wide search area at once, the board second
    let h = s.press();
    assert_eq!(
        (h.kind, h.id.as_str()),
        (HintKind::Animal, "area:zebra"),
        "{h:?}"
    );
    let a = s.g.animal("zebra").unwrap();
    let (c, r_wide) = hints::hiding_circle(&s.g, a, true).unwrap();
    assert!(2.0 * r_wide >= WIDE_AREA_MIN_DIAMETER_M);
    assert!(
        (h.pos.distance(c) - r_wide).abs() < 1e-3,
        "edge of the wide circle"
    );
    assert!(h.pos.distance(a.pos) >= 0.9, "never the animal's position");
    assert_eq!(h.kind.step_key(), "hint-search");
    let h = s.press();
    assert_eq!((h.kind, h.id.as_str()), (HintKind::Board, "board:zebra"));
    assert_eq!(h.kind.step_key(), "hint-read");
    s.t.hide();
    s.idle(AREA_HINT_AFTER_S + 0.5 - s.t.search_s("zebra"));
    assert!(s.t.search_s("zebra") >= AREA_HINT_AFTER_S);
    let h = s.press();
    assert_eq!(h.kind, HintKind::Animal, "{h:?}");
    let a = s.g.animal("zebra").unwrap();
    let (c, r) = hints::hiding_circle(&s.g, a, false).unwrap();
    assert!(2.0 * r >= AREA_MIN_DIAMETER_M);
    assert!(r < r_wide, "the exact circle is smaller than the wide one");
    assert!(
        (h.pos.distance(c) - r).abs() < 1e-3,
        "on the edge of the circle"
    );
    assert!(h.pos.distance(a.pos) >= 0.9, "never the animal's position");
    // never the animal's position, for any time and any wander step
    for _ in 0..20 {
        s.idle(3.0);
        for c in candidates(&s.g, &s.t) {
            assert!(
                c.pos.distance(s.g.animal("zebra").unwrap().pos) >= 0.9
                    || c.kind != HintKind::Animal,
                "{c:?}"
            );
        }
    }
}

// HINT-004
#[test]
fn hint_004_following_then_gate() {
    let mut s = Sim::new(night_game(17));
    read_board(&mut s, "zebra");
    let &(_, pos, facing) = s.g.food_boxes.iter().find(|b| b.0 == Food::Grass).unwrap();
    s.walk_to(cell_of(pos + facing * 1.1), 60.0);
    s.g.take_food(Food::Grass).unwrap();
    s.reach_animal("zebra");
    s.g.show_food("zebra").unwrap();
    assert_eq!(s.g.animal("zebra").unwrap().state, AnimalState::Following);
    let h = s.press();
    assert_eq!((h.kind, h.id.as_str()), (HintKind::Gate, "gate:enc_zebra"));
    let gate = s.g.level.data.element("enc_zebra").unwrap().gate.unwrap();
    assert!(gate.distance_to(h.pos) < 1.0);
}

// HINT-005: GAME-EVENTS does not exist in the code yet — events are skipped, so no target
// has the event priority; the priority order itself puts events first.
#[test]
fn hint_005_event_priority_first_and_skipped_while_missing() {
    const { assert!(hints::PRIO_EVENT < hints::PRIO_MISSION) };
    let s = Sim::new(night_game(3));
    assert!(candidates(&s.g, &s.t)
        .iter()
        .all(|c| c.priority > hints::PRIO_EVENT));
}

// HINT-006
#[test]
fn hint_006_three_presses_three_targets() {
    let mut s = Sim::new(night_game(5));
    let a = s.press().id;
    s.idle(1.0);
    let b = s.press().id;
    s.idle(1.0);
    let c = s.press().id;
    assert!(a != b && b != c && a != c, "{a} {b} {c}");
    // the fourth press starts the cycle again
    assert_eq!(s.press().id, a);
    // shown for 12 s, then gone
    s.idle(HINT_SHOW_S + 0.1);
    assert!(s.t.shown().is_none());
}

/// A random reachable state: missions partly done, food in the hands or lying, the time of
/// day, the player on a walkable cell.
fn random_state(seed: u64) -> Sim {
    let mut r = Pcg32::new(seed);
    let mut s = Sim::new(night_game(seed));
    let g = &mut s.g;
    let ids: Vec<&str> = {
        let mut v: Vec<&str> = g
            .animals
            .iter()
            .filter(|a| g.in_scope(a))
            .map(|a| a.id())
            .collect();
        v.dedup();
        v
    };
    for id in &ids {
        match r.next_u32() % 4 {
            0 => {}
            1 => {
                for j in g.group(id) {
                    g.missions[j].started = true;
                }
            }
            2 => {
                g.debug_send_home(id);
            }
            _ => {
                for j in g.group(id) {
                    g.missions[j].started = true;
                }
            }
        }
    }
    let foods = [Food::Grass, Food::Melons, Food::Bamboo, Food::Hay];
    if r.next_u32().is_multiple_of(2) {
        let f = foods[(r.next_u32() % 4) as usize];
        g.carry.take(&zoo_core::FoodBox { food: f });
    }
    match r.next_u32() % 5 {
        0 => {
            let _ = g.debug_set_daytime("night");
        }
        1 => {
            let _ = g.debug_set_daytime("dusk");
        }
        _ => {}
    }
    g.drain_events();
    let grid = g.level.grid();
    let cells: Vec<IVec2> = g
        .level
        .data
        .level
        .bounds
        .cells()
        .filter(|&c| grid.is_walkable(c, false))
        .collect();
    let c = cells[(r.next_u32() as usize) % cells.len()];
    g.player.pos = cell_center(c);
    s
}

// HINT-008
#[test]
fn hint_008_never_stuck_fuzz_1000_states() {
    for seed in 0..1000u64 {
        let mut s = random_state(seed);
        let h = s.t.press(&s.g).cloned();
        assert!(h.is_some(), "no hint in state {seed}");
        let h = h.unwrap();
        assert!(h.pos.is_finite() && h.stand.is_finite(), "{seed}: {h:?}");
        // riddles stay fair: never an escaped animal's position
        for a in
            s.g.animals
                .iter()
                .filter(|a| a.state == AnimalState::Escaped)
        {
            assert!(
                h.kind != HintKind::Animal || h.pos.distance(a.pos) >= 0.9,
                "{seed}: {h:?}"
            );
        }
    }
}

// HINT-010
#[test]
fn hint_010_idle_nudge_after_90_s_and_useful_actions_restart_it() {
    let mut s = Sim::new(night_game(2));
    s.idle(IDLE_NUDGE_S - 1.0);
    assert_eq!(s.t.pulses(), 0);
    // a useful action restarts the 90 s: a new cell explored
    let p = s.g.player.pos;
    for _ in 0..40 {
        s.tick(Vec2::X);
    }
    assert!(cell_of(s.g.player.pos) != cell_of(p));
    s.idle(IDLE_NUDGE_S - 1.0);
    assert_eq!(s.t.pulses(), 0, "exploring restarted the idle time");
    // an interaction restarts it too
    s.t.observe(&GameEvent::MissionStarted {
        animal: "zebra".into(),
    });
    s.idle(IDLE_NUDGE_S - 1.0);
    assert_eq!(s.t.pulses(), 0);
    s.idle(1.5);
    assert_eq!(s.t.pulses(), 1, "pulses once");
    assert!(s.t.shown().is_none(), "no popup, no hint opens by itself");
    // animals' own reactions are no useful action
    assert!(!hints::is_useful(&GameEvent::Waiting {
        animal: "zebra".into()
    }));
}

// HINT-011, FEED-024, FEED-025: lying right food and (after the panda board) ripe bamboo are mission targets.
#[test]
fn hint_011_lying_food_and_bamboo_cut_spots() {
    let mut s = Sim::new(night_game(17));
    // before the panda board is read, no cut spot is offered
    assert!(candidates(&s.g, &s.t)
        .iter()
        .all(|c| c.kind != HintKind::Bamboo));
    read_board(&mut s, "panda");
    let c = candidates(&s.g, &s.t);
    assert!(c
        .iter()
        .any(|h| h.kind == HintKind::Food && h.animal == Some("panda")));
    let cuts: Vec<_> = c.iter().filter(|h| h.kind == HintKind::Bamboo).collect();
    assert_eq!(
        cuts.len(),
        s.g.level.data.cut_spots.len(),
        "every ripe cut spot"
    );
    assert!(cuts.iter().all(|h| h.priority == hints::PRIO_MISSION));
    // take bamboo, put it down: the lying bamboo is a "pick up" target
    let &(_, pos, facing) = s.g.food_boxes.iter().find(|b| b.0 == Food::Bamboo).unwrap();
    s.walk_to(cell_of(pos + facing * 1.1), 60.0);
    s.g.take_food(Food::Bamboo).unwrap();
    s.walk_to(cell_of(pos + facing * 4.0), 60.0);
    assert!(s.g.put_down().is_ok());
    let c = candidates(&s.g, &s.t);
    let lying = c
        .iter()
        .find(|h| h.kind == HintKind::PickUp)
        .expect("pick up");
    assert!(lying.pos.distance(s.g.lying.foods[0].pos) < 1e-3);
    assert_eq!(lying.animal, Some("panda"));
    // a lying wrong food is no target
    s.g.lying.foods[0].food = Food::Hay;
    assert!(candidates(&s.g, &s.t)
        .iter()
        .all(|h| h.kind != HintKind::PickUp));
}

// HINT-012: at night the bed and the moon door; targets behind the moon door → the door.
#[test]
fn hint_012_night_bed_and_moon_door() {
    let mut s = Sim::new(night_game(4));
    for a in DAY_1 {
        s.g.debug_send_home(a);
    }
    s.idle(CELEBRATION_S + 1.0);
    // dusk: the bed is next (night is coming)
    assert_eq!(s.press().kind, HintKind::Bed);
    s.t.hide();
    s.idle(DUSK_S + 0.5);
    assert_eq!(s.g.daytime.phase, Phase::Night);
    let c = candidates(&s.g, &s.t);
    // the night boards are behind the moon door: the door and the bed are the top two
    let top: Vec<HintKind> = c.iter().take(2).map(|h| h.kind).collect();
    assert!(
        top.contains(&HintKind::MoonDoor) && top.contains(&HintKind::Bed),
        "{c:?}"
    );
    assert!(c.iter().all(|h| h.kind != HintKind::Board), "{c:?}");
    // show the moon door (one of the top two, press cycles) and follow it
    let mut door = s.press();
    if door.kind != HintKind::MoonDoor {
        door = s.press();
    }
    assert_eq!(door.kind, HintKind::MoonDoor, "{c:?}");
    s.walk_to(cell_of(door.stand), 150.0);
    s.face(door.pos);
    s.idle(0.3);
    // reached (≤ REACHED_M) — or timed out on a long walk: in both cases no door hint is
    // left on screen, whatever the walking time (street under the door, Q-182)
    assert!(
        s.t.shown().is_none_or(|h| h.kind != HintKind::MoonDoor),
        "{:?}",
        s.t.shown()
    );
    // through the door: the night boards (a fresh press, not the cycle of the day zoo)
    assert!(s.g.interact().is_some());
    assert!(s.g.player_in_night_zoo());
    s.t.hide();
    let h = s.press();
    assert_eq!(h.kind, HintKind::Board, "{h:?}");
    assert!(["hedgehog", "bat", "owl"].contains(&h.animal.unwrap()));
}

// HINT-007 helper (screen placement, the e2e checks the drawing).
#[test]
fn hint_007_screen_place_edge_arrow() {
    let (w, h) = (400.0, 800.0);
    let on = hints::screen_place(Vec4::new(0.2, 0.1, 0.5, 1.0), w, h, 40.0);
    assert!(on.on_screen && (on.x - 240.0).abs() < 1e-3 && (on.y - 360.0).abs() < 1e-3);
    let right = hints::screen_place(Vec4::new(3.0, 0.0, 0.5, 1.0), w, h, 40.0);
    assert!(!right.on_screen && (right.x - 360.0).abs() < 1e-3 && right.angle_deg.abs() < 1e-3);
    let behind = hints::screen_place(Vec4::new(-0.5, 0.3, 0.5, -1.0), w, h, 40.0);
    assert!(!behind.on_screen && behind.y > h / 2.0 && behind.x < w / 2.0);
    for p in [right, behind] {
        assert!(p.x >= 40.0 - 1e-3 && p.x <= w - 40.0 + 1e-3);
        assert!(p.y >= 40.0 - 1e-3 && p.y <= h - 40.0 + 1e-3);
    }
}

// NIGHT-019: the compass strip data.
#[test]
fn night_019_progress_until_nightfall() {
    let mut s = Sim::new(night_game(6));
    let p = night_progress(&s.g);
    assert_eq!(p.state, ProgressState::Missing);
    assert_eq!(p.level, "level_1");
    let mut ids: Vec<&str> = p.animals.iter().map(|(a, _)| *a).collect();
    ids.sort();
    assert_eq!(
        ids,
        ["hippo", "panda", "zebra"],
        "one icon per species of the level"
    );
    assert!(p.animals.iter().all(|(_, home)| !*home));
    s.g.debug_send_home("hippo");
    let p = night_progress(&s.g);
    assert_eq!(p.state, ProgressState::Missing);
    assert_eq!(p.animals.iter().filter(|(_, home)| *home).count(), 1);
    assert!(p.animals.contains(&("hippo", true)));
    s.g.debug_send_home("zebra");
    s.g.debug_send_home("panda");
    // all home: night is coming (celebration, dusk)
    assert_eq!(night_progress(&s.g).state, ProgressState::NightComing);
    assert!(night_progress(&s.g).animals.iter().all(|(_, h)| *h));
    s.idle(CELEBRATION_S + 1.0);
    assert_eq!(s.g.daytime.phase, Phase::Dusk);
    assert_eq!(night_progress(&s.g).state, ProgressState::NightComing);
    s.idle(DUSK_S);
    assert_eq!(s.g.daytime.phase, Phase::Night);
    // night: the night zoo's animals
    let p = night_progress(&s.g);
    assert_eq!(
        (p.state, p.level.as_str()),
        (ProgressState::Night, "night_1")
    );
    assert_eq!(p.animals.len(), 3);
    for a in ["hedgehog", "bat", "owl"] {
        s.g.debug_send_home(a);
    }
    assert_eq!(night_progress(&s.g).state, ProgressState::Sleep);
    // sleeping / morning: hidden; the next day: level 2's animals
    s.g.drain_events();
    s.g.debug_next_morning();
    s.idle(4.0);
    assert_eq!(s.g.daytime.phase, Phase::Day);
    let p = night_progress(&s.g);
    assert_eq!(
        (p.state, p.level.as_str()),
        (ProgressState::Missing, "level_2")
    );
    assert!(p.animals.iter().all(|(_, h)| !*h));
}

// NIGHT-019: slept before the night zoo was done (Q-140) → the compass says "sleep".
#[test]
fn night_019_progress_sleep_by_day_while_the_night_zoo_waits() {
    let mut s = Sim::new(night_game(6));
    for a in DAY_1 {
        s.g.debug_send_home(a);
    }
    s.idle(CELEBRATION_S + DUSK_S + 1.0);
    s.g.debug_next_morning();
    s.idle(4.0);
    assert_eq!(s.g.daytime.phase, Phase::Day);
    assert!(s.g.night_zoo_waiting());
    let p = night_progress(&s.g);
    assert_eq!(
        (p.state, p.level.as_str()),
        (ProgressState::Sleep, "night_1")
    );
    assert_eq!(s.press().kind, HintKind::Bed, "the hint points at the bed");
}

// HINT-014 / NIGHT-021 (core): a child who only follows the hints brings every level-1
// animal home and night falls — nothing blocks nightfall (several seeds = every hiding
// place of level 1 at least once).
#[test]
fn hint_014_night_021_following_the_hints_leads_to_the_night() {
    let mut places: std::collections::BTreeSet<String> = Default::default();
    for seed in [17u64, 1, 2, 3, 4, 5, 8, 11] {
        let mut s = Sim::new(night_game(seed));
        for a in DAY_1 {
            places.insert(s.g.animal(a).unwrap().hiding_place.clone());
        }
        follow_hints(&mut s, 60, day_done);
        assert!(s.events.contains(&GameEvent::LevelComplete {
            level: "level_1".into()
        }));
        s.idle(CELEBRATION_S + DUSK_S + 0.5);
        assert_eq!(s.g.daytime.phase, Phase::Night, "seed {seed}");
        assert!(s.g.level.is_barrier_open("moon_door"), "seed {seed}");
        // the bed is one of the next hints; sleeping brings the morning
        let mut bed = None;
        for _ in 0..3 {
            let h = s.press();
            if h.kind == HintKind::Bed {
                bed = Some(h);
                break;
            }
        }
        let bed = bed.expect("the bed is among the top hints at night");
        s.walk_to(cell_of(bed.stand), 150.0);
        s.face(bed.pos);
        s.idle(0.1);
        assert_eq!(
            s.g.interact(),
            Some(zoo_core::Interaction::Sleep),
            "seed {seed}"
        );
        s.idle(zoo_core::daytime::SLEEP_S + 0.5);
        assert!(s.events.contains(&GameEvent::Morning), "seed {seed}");
    }
    // every level-1 hiding place was played at least once
    assert_eq!(places.len(), 9, "{places:?}");
}

// HINT-017: the treat hint points at the animal's fence (outside) or at the animal (inside)
// only while the child holds something the animal likes; carried own food counts too.
#[test]
fn hint_017_treat_hint_follows_what_is_liked() {
    let mut g = common::game(1);
    assert!(g.debug_send_home("zebra"));
    let has = |g: &Game| {
        candidates(g, &HintTracker::default())
            .iter()
            .any(|h| h.kind == HintKind::Treat)
    };
    assert!(!has(&g), "nothing to give: no hint");
    g.garden.basket.carrots = 1;
    assert!(has(&g), "a liked carrot");
    g.garden.basket.carrots = 0;
    g.garden.basket.potatoes = 1; // zebras do not like potatoes
    assert!(!has(&g), "a disliked treat: no hint");
    let own = g.animal("zebra").unwrap().info.foods[0];
    g.carry.take(&zoo_core::FoodBox { food: own });
    assert!(has(&g), "the zebra's own carried food");
}

/// A random state with split pairs and disagreeing members (NEVER STUCK).
fn messy_state(seed: u64) -> Sim {
    let mut s = random_state(seed);
    let mut r = Pcg32::new(seed ^ 0x5eed);
    let g = &mut s.g;
    // every pair species of the zoo (Q-308: all day animals come as pairs), also split pairs
    let mut species: Vec<&'static str> = g.animals.iter().map(|a| a.id()).collect();
    species.dedup();
    species.sort_unstable();
    species.dedup();
    for id in species {
        let group = g.group(id);
        if group.len() < 2 || g.mission(id).is_some_and(|m| m.complete) {
            continue;
        }
        match r.next_u32() % 4 {
            0 => {
                // member 0 home, the partner out; the mission flags disagree
                g.animals[group[0]].state = AnimalState::InEnclosure;
                g.missions[group[0]].started = true;
                g.missions[group[0]].complete = r.next_u32().is_multiple_of(2);
            }
            1 => {
                g.animals[group[1]].state = AnimalState::InEnclosure;
                g.missions[group[1]].complete = r.next_u32().is_multiple_of(2);
            }
            2 => {
                // an old save knew one zebra only
                g.animals[group[0]].state = AnimalState::InEnclosure;
                g.missions[group[0]].started = true;
                g.missions[group[0]].complete = true;
                let mut save = g.to_save();
                save.animals.retain(|a| a.id != id || a.member == 0);
                save.missions.retain(|m| m.animal != id || m.complete);
                let p = g.player.pos;
                *g = Game::from_save(zoo_data(), &save).unwrap();
                g.player.pos = p;
            }
            _ => {}
        }
    }
    if r.next_u32().is_multiple_of(2) {
        // babies in every pair (FAM-011: wherever the female is, the baby is with her; it is
        // never a dead end)
        let mut save = g.to_save();
        let mut ids: Vec<String> = g.animals.iter().map(|a| a.id().to_string()).collect();
        ids.sort();
        ids.dedup();
        save.babies = ids;
        let p = g.player.pos;
        *g = Game::from_save(zoo_data(), &save).unwrap();
        g.player.pos = p;
    }
    g.drain_events();
    s
}

// HINT-019 (NEVER STUCK a): while a mission of the zoo is open, a hint of priority <= 3 exists —
// the optional garden is never the only way forward.
#[test]
fn hint_019_open_mission_always_has_a_mission_level_hint() {
    for seed in 0..1500u64 {
        let s = messy_state(seed);
        let day = s.g.daytime.phase == Phase::Day && s.g.daytime.dusk_in.is_none();
        // (by day the child is never inside the night zoo: its door is closed)
        let in_night =
            s.g.level
                .data
                .part_at(cell_of(s.g.player.pos))
                .is_some_and(|k| s.g.level.data.is_night_part(k));
        if !day || in_night || !s.g.any_mission_open() {
            continue;
        }
        let c = candidates(&s.g, &s.t);
        assert!(
            c.first()
                .is_some_and(|h| h.priority <= hints::PRIO_UNSTARTED),
            "seed {seed}: first hint {:?} {:?}",
            c.first().map(|h| (&h.id, h.priority)),
            s.g.animals
                .iter()
                .zip(&s.g.missions)
                .map(|(a, m)| (a.id(), a.state, a.part, *m))
                .collect::<Vec<_>>()
        );
    }
}

// HINT-020 (NEVER STUCK c): no mission progress for 120 s -> the hint points at the missing
// animal itself; after 180 s the escaped animals walk to the player; progress ends it.
#[test]
fn hint_020_stall_help_points_at_the_animal_and_it_comes_to_the_player() {
    let mut s = Sim::new(night_game(4));
    s.idle(STALL_HELP_S - 5.0);
    assert!(s.press().kind != HintKind::Help);
    s.t.hide();
    s.idle(10.0);
    let h = s.press();
    assert_eq!(h.kind, HintKind::Help, "{h:?}");
    let a = h.animal.unwrap();
    assert!(
        h.pos.distance(s.g.animal(a).unwrap().pos) < 0.01,
        "exact position"
    );
    // 60 s later they walk: within another 120 s every escaped animal of a day mission is near
    s.idle(STALL_WALK_S - STALL_HELP_S + 120.0);
    let p = s.g.player.pos;
    for an in
        s.g.animals
            .iter()
            .filter(|x| s.g.in_scope(x) && x.state == AnimalState::Escaped)
    {
        assert!(
            an.pos.distance(p) < 8.0,
            "{} is {} m away",
            an.id(),
            an.pos.distance(p)
        );
    }
    // taking the right food and showing it is progress again: the stall restarts
    let id = s.g.animals[0].id();
    let food = s.g.animal(id).unwrap().info.foods[0];
    s.g.carry.take(&zoo_core::FoodBox { food });
    s.idle(0.1);
    assert!(s.g.stall_s() < 1.0);
}

// HINT-021 (NEVER STUCK): following only the hints always brings every day animal home, also
// from split pairs, old one-zebra saves and with a save/restore in the middle.
#[test]
fn hint_021_never_stuck_follow_hints_from_messy_states_with_save_restore() {
    for seed in 0..40u64 {
        let mut s = messy_state(seed);
        // a fuzz state may be night: wake it up to a plain day
        let _ = s.g.debug_set_daytime("day");
        s.g.player.pos = s.g.level.data.spawn.cell().as_vec2() + Vec2::splat(0.5);
        let mut r = Pcg32::new(seed + 99);
        let mut steps = 0;
        while !day_done(&s) {
            steps += 1;
            assert!(steps < 60, "seed {seed}: not home after 60 hint steps");
            s.t.hide();
            follow_one(&mut s);
            if r.next_u32().is_multiple_of(3) {
                // save and restore in the middle
                let save = s.g.to_save();
                let p = s.g.player.pos;
                s.g = Game::from_save(zoo_data(), &save).unwrap();
                s.g.player.pos = p;
            }
        }
    }
}

// NIGHT-028: the compass badge = the kind of the best candidate, without showing a hint.
#[test]
fn night_028_compass_badge_is_the_next_task() {
    let mut s = Sim::new(night_game(6));
    let first = candidates(&s.g, &s.t).first().map(|h| h.kind.id());
    let b = compass_badge(&s.g, &s.t).expect("a badge in a new game");
    assert_eq!(Some(b.0), first);
    assert_eq!(b.0, "board");
    assert!(s.t.shown().is_none(), "computing the badge shows no hint");
    for a in DAY_1 {
        s.g.debug_send_home(a);
    }
    assert_eq!(compass_badge(&s.g, &s.t).map(|b| b.0), Some("night_coming"));
    s.idle(CELEBRATION_S + 1.0);
    assert_eq!(s.g.daytime.phase, Phase::Dusk);
    assert_eq!(compass_badge(&s.g, &s.t).map(|b| b.0), Some("bed"));
    s.idle(DUSK_S);
    assert_eq!(s.g.daytime.phase, Phase::Night);
    let k = compass_badge(&s.g, &s.t).map(|b| b.0).unwrap();
    assert!(k == "moon_door" || k == "bed", "night badge {k}");
    assert!(s.g.daytime.sleep());
    assert_eq!(compass_badge(&s.g, &s.t), None, "sleeping: no badge");
}
