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
    /// Loop detector (HINT-027): the last hint id, the game's hint signature then and how often
    /// that pair was shown in a row.
    run: (String, u64, u32),
}

impl Sim {
    fn new(g: Game) -> Self {
        Self {
            g,
            t: HintTracker::default(),
            events: Vec::new(),
            run: (String::new(), 0, 0),
        }
    }

    /// HINT-027 "no hint loops": the same hint shown more than 3 times in a row while nothing
    /// changed in the game is a bug. Exempt: the read-again fallback (priority 6) while the
    /// "what next" line says what is left (HINT-028).
    fn check_loop(&mut self, h: &hints::Hint) {
        let sig = self.g.hint_signature();
        if self.run.0 == h.id && self.run.1 == sig {
            self.run.2 += 1;
        } else {
            self.run = (h.id.clone(), sig, 1);
        }
        // (the dusk is a bounded wait for the night: the bed is right, sleeping starts at night)
        let waiting = self.g.daytime.phase == Phase::Dusk || self.g.daytime.dusk_in.is_some();
        let exempt = waiting
            || (h.priority == hints::PRIO_FALLBACK && hints::what_next(&self.g, &self.t).is_some());
        assert!(
            self.run.2 <= 3 || exempt,
            "hint loop: {} shown {} times without any change (phase {:?}, pos {})",
            h.id,
            self.run.2,
            self.g.daytime.phase,
            self.g.player.pos
        );
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
        // (a child that cannot get close from one bank tries another one: banks already tried
        // are skipped, GAME-HOUSE changed the random stream and exposed a pond out of reach
        // of the first bank)
        let mut tried: Vec<IVec2> = Vec::new();
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
                .filter(|&c| tried.iter().all(|t| (*t - c).abs().max_element() > 3))
                .min_by(|a, b| {
                    cell_center(*a)
                        .distance(s)
                        .total_cmp(&cell_center(*b).distance(s))
                })
                .unwrap_or_else(|| cell_of(self.g.player.pos));
            tried.push(cell);
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
        panic!(
            "could not reach the {animal} (player {}, group {:?})",
            self.g.player.pos,
            self.g
                .group(animal)
                .iter()
                .map(|&j| (
                    self.g.animals[j].state,
                    self.g.animals[j].pos,
                    self.g.animals[j].wander.route.len(),
                    self.g.animals[j].wander.pause_s
                ))
                .collect::<Vec<_>>()
        );
    }
}

/// One step of the child following the hint.
fn follow_one(s: &mut Sim) {
    // asleep (the bed was used): the child waits for the morning, no hint is asked meanwhile
    if matches!(s.g.daytime.phase, Phase::Sleeping | Phase::Morning) {
        s.idle(zoo_core::daytime::SLEEP_S + zoo_core::daytime::MORNING_S + 0.5);
    }
    let h = s.press();
    s.check_loop(&h);
    follow_hint(s, h);
}

/// The child walks to the hint `h` and does what it says.
fn follow_hint(s: &mut Sim, h: hints::Hint) {
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
            // the line says "show it its food": the child fetches the right food first
            let food = s.g.animal(a).unwrap().info.foods[0];
            if s.g
                .carry
                .food()
                .is_none_or(|f| !s.g.animal(a).unwrap().info.eats(f))
            {
                s.g.carry.take(&zoo_core::FoodBox { food });
            }
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
            if s.g.needs_container(a) {
                // the carried fish bowl cannot walk the gate cell with this scripted player
                s.g.debug_send_home(a);
                s.idle(0.5);
                return;
            }
            let gate = s.g.level.data.elements[s.g.animal(a).unwrap().enclosure]
                .gate
                .unwrap()
                .cells()
                .next()
                .unwrap();
            s.walk_to(gate, 30.0);
            s.idle(0.5);
        }
        HintKind::NightGate => {
            // through the open lantern gate: to the cell 3 m beyond it
            s.walk_to(cell_of(h.stand), 150.0);
            let beyond = h.pos + (h.pos - h.stand).normalize_or_zero() * 3.0;
            s.walk_to(cell_of(beyond), 60.0);
            s.idle(0.2);
        }
        HintKind::Note => {
            // CART-029 / HINT-032: read the note on the desk (the panel opens by itself)
            s.walk_to(cell_of(h.stand), 150.0);
            s.face(h.pos);
            s.idle(0.6);
            assert!(s.g.note_read, "the note was read");
            s.g.close_panel();
        }
        HintKind::KeyBox => {
            // the child typed what the note says; every odd game first mistypes three times
            // (the next hint is then the note again: HINT-032) and reads the note once more
            s.walk_to(cell_of(h.stand), 150.0);
            s.face(h.pos);
            s.idle(0.1);
            assert_eq!(
                s.g.available_target(),
                Some(zoo_core::game::Target::KeyBox {
                    id: "key_box_l1".into()
                }),
                "standing at the key box"
            );
            let answer = s.g.cart_task().answer;
            if s.g.key_box_tries == 0 && s.g.hint_signature().is_multiple_of(3) {
                let wrong = answer % 999 + 1;
                for _ in 0..3 {
                    assert!(matches!(
                        s.g.enter_code(wrong),
                        zoo_core::cart_key::CodeResult::Wrong { .. }
                    ));
                }
                s.idle(0.1);
            } else {
                assert_eq!(
                    s.g.enter_code(answer),
                    zoo_core::cart_key::CodeResult::Right
                );
                s.idle(0.1);
            }
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
            if h.kind == HintKind::Bed
                && (s.g.daytime.dusk_in.is_some() || s.g.daytime.phase == Phase::Dusk)
            {
                // the child waits at the bed until night falls
                s.idle(CELEBRATION_S + DUSK_S + 0.5);
            }
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

// HINT-017 / HINT-026: the treat hint points at the animal's fence (outside) or at the animal
// (inside) only while the species' baby is still possible and the child holds its SPECIAL gift
// (a liked garden treat; for a species without a liked treat its own food). Hearts are no task.
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
    assert!(
        !has(&g),
        "the zebra's own food only gives hearts: no task (HINT-026)"
    );
}

/// The zoo with every day animal home and the day running (the night zoo not done).
fn day_all_home(seed: u64, with_night: bool) -> Sim {
    let mut s = Sim::new(night_game(seed));
    let ids: Vec<&'static str> = s.g.animals.iter().map(|a| a.id()).collect();
    for id in ids {
        let part = s.g.animals[s.g.animal_index(id).unwrap()].part;
        if with_night || !s.g.level.data.is_night_part(part) {
            s.g.debug_send_home(id);
        }
    }
    s.g.drain_events();
    let _ = s.g.debug_set_daytime("day");
    s.idle(0.5);
    s
}

// HINT-026 (reproduces the user report 2026-10-04): the monkeys were fed and have their baby,
// the child still holds food and apples — the treat hint must be gone, not offered again.
#[test]
fn hint_026_treat_hint_disappears_after_feeding_and_once_the_baby_exists() {
    let mut s = day_all_home(3, false);
    let has = |s: &Sim| {
        candidates(&s.g, &s.t)
            .iter()
            .any(|h| h.kind == HintKind::Treat && h.animal == Some("monkey"))
    };
    // own food only: hearts, no task (the old stateless hint offered it for ever)
    let own = s.g.animal("monkey").unwrap().info.foods[0];
    s.g.carry.take(&zoo_core::FoodBox { food: own });
    assert!(!has(&s), "carrying its food is no task");
    s.g.carry.consume();
    // an apple in the basket: the baby is the task
    s.g.garden.basket.apples = 2;
    assert!(has(&s), "apple for the monkey pair without a baby");
    let h = candidates(&s.g, &s.t)
        .into_iter()
        .find(|h| h.id == "treat:monkey")
        .unwrap();
    s.walk_to(cell_of(h.stand), 200.0);
    s.face(h.pos);
    assert_eq!(
        s.g.give_treat("monkey", zoo_core::garden::Treat::Apple),
        Some(true)
    );
    assert!(s.g.babies.iter().any(|b| b == "monkey"), "the baby is born");
    assert!(!has(&s), "baby exists: never again");
    assert!(s.g.gift_cooldown_s("monkey") > 170.0);
    // the second apple does not bring the hint back
    assert!(!has(&s));
    // cooldown (without a baby, e.g. an old state): no treat hint for 180 s, then it is due again
    s.g.babies.clear();
    s.g.baby_states.clear();
    assert!(!has(&s), "just fed: cooldown");
    s.idle(179.0);
    assert!(!has(&s), "still cooling down at 179 s");
    s.idle(2.0);
    assert!(has(&s), "after 180 s a baby is possible again");
}

// HINT-027: the repeat guard. An optional hint reached twice while nothing changed is dropped
// until the game state changes; the follow-the-hints fuzz (loop detector in `follow_one`) never
// shows one hint more than 3 times in a row without a change, over many seeds and 400 steps.
#[test]
fn hint_027_repeat_guard_drops_a_repeated_optional_hint() {
    let mut s = day_all_home(5, true);
    s.g.garden.basket.apples = 1; // an apple for a baby that is possible
    let first = s.press();
    assert_eq!(first.priority, hints::PRIO_OPTIONAL, "{first:?}");
    let id = first.id.clone();
    for round in 0..2 {
        assert!(
            candidates(&s.g, &s.t).iter().any(|h| h.id == id),
            "round {round}: the hint is there"
        );
        s.t.hide();
        s.g.player.pos = first.stand;
        s.t.press(&s.g);
        s.idle(0.5); // reached: counted
        s.t.hide();
        // (the child does nothing with it: no change of the game)
    }
    assert!(
        !candidates(&s.g, &s.t).iter().any(|h| h.id == id),
        "reached twice without a change: dropped"
    );
    // a change of the game state (a new item) brings it back
    s.g.garden.basket.oranges = 1;
    assert!(candidates(&s.g, &s.t).iter().any(|h| h.id == id));
}

#[test]
fn hint_027_no_loops_when_following_the_hints_for_400_steps() {
    // from the end state (all animals home, food in the hands, treats in the basket) over
    // several seeds, then from messy states through the whole day (loop detector: follow_one)
    for seed in [1u64, 2, 3] {
        let mut s = day_all_home(seed, true);
        let own = s.g.animal("monkey").unwrap().info.foods[0];
        s.g.carry.take(&zoo_core::FoodBox { food: own });
        s.g.garden.basket.carrots = 2;
        for i in 0..400 {
            s.t.hide();
            if candidates(&s.g, &s.t).is_empty() {
                panic!(
                    "step {i}: no candidates, phase {:?} pos {}",
                    s.g.daytime.phase, s.g.player.pos
                );
            }
            follow_one(&mut s);
        }
    }
    for seed in 0..6u64 {
        let mut s = messy_state(seed);
        let _ = s.g.debug_set_daytime("day");
        s.g.player.pos = s.g.level.data.spawn.cell().as_vec2() + Vec2::splat(0.5);
        for _ in 0..400 {
            s.t.hide();
            follow_one(&mut s);
        }
    }
}

// HINT-028: nothing but optional things left -> the compass says what is left in plain words
// (key present, badge agrees); everything home = the celebration, never a dead end.
#[test]
fn hint_028_only_optional_left_the_what_next_text() {
    // all animals home, night zoo not done: the bed by day leads on (Q-140), no "what next" needed
    let s = day_all_home(3, false);
    assert_eq!(hints::what_next(&s.g, &s.t), None);
    let first = candidates(&s.g, &s.t).remove(0);
    assert_eq!(first.kind, HintKind::Bed);
    assert_eq!(
        hints::step_key_for(&s.g, &s.t, &first),
        "hint-bed-night-zoo"
    );
    // everything home including the night zoo: celebration text, badge agrees, no monkey hint
    let mut s = day_all_home(3, true);
    s.g.garden.basket.carrots = 2;
    assert_eq!(hints::what_next(&s.g, &s.t), Some("all_done"));
    assert_eq!(compass_badge(&s.g, &s.t).map(|b| b.0), Some("all_done"));
    let h = s.press();
    assert_eq!(hints::step_key_for(&s.g, &s.t, &h), "next-all_done");
    // the texts exist in every reading level and language
    for lang in ["de", "en"] {
        let ftl = common::ftl_text(lang);
        for k in ["hint-bed-night-zoo", "next-all_done", "next-explore"] {
            assert!(ftl.contains(&format!("{k} =")), "{lang} {k}");
        }
        for n in ["all_done", "explore"] {
            for l in ["kiga", "klasse1", "klasse2", "klasse3"] {
                let k = format!("night-progress-info-next-{n}-{l} =");
                assert!(ftl.contains(&k), "{lang} {k}");
            }
        }
    }
}

// HINT-029: the full flow level 3 -> dusk -> bed -> morning with hints only; babies and treats
// given on the way; the 🧭 never points at the monkeys again once they have their baby.
#[test]
fn hint_029_level_3_done_then_dusk_bed_morning_with_hints_only() {
    let mut s = Sim::new(night_game(3));
    let ids: Vec<&'static str> = s.g.animals.iter().map(|a| a.id()).collect();
    for id in ids {
        let part = s.g.animals[s.g.animal_index(id).unwrap()].part;
        let name = s.g.level.data.parts[part].id.clone();
        if name == "level_1" || name == "level_2" {
            s.g.debug_send_home(id);
        }
    }
    s.g.drain_events();
    let _ = s.g.debug_set_daytime("day");
    // level 3: follow the hints until its animals are home; the monkeys get their baby on the way
    let l3 = |s: &Sim| {
        s.g.animals
            .iter()
            .zip(&s.g.missions)
            .filter(|(a, _)| s.g.level.data.parts[a.part].id == "level_3")
            .all(|(_, m)| m.complete)
    };
    follow_hints(&mut s, 120, l3);
    s.g.garden.basket.apples = 2;
    let c = candidates(&s.g, &s.t);
    assert!(
        c.iter().any(|h| h.id == "treat:monkey"),
        "an apple: the baby is the task"
    );
    assert!(s
        .g
        .give_treat("monkey", zoo_core::garden::Treat::Apple)
        .unwrap());
    s.g.carry.take(&zoo_core::FoodBox {
        food: s.g.animal("monkey").unwrap().info.foods[0],
    });
    assert!(s
        .events
        .iter()
        .any(|e| matches!(e, GameEvent::LevelComplete { level } if level == "level_3")));
    // celebration, dusk: from now on every press is the bed, never the monkeys or the garden
    for _ in 0..4 {
        s.t.hide();
        let h = s.press();
        assert_eq!(h.kind, HintKind::Bed, "{h:?}");
        assert!(candidates(&s.g, &s.t)
            .iter()
            .all(|h| h.id != "treat:monkey"));
        s.idle(CELEBRATION_S / 4.0);
    }
    s.idle(DUSK_S + 0.5);
    assert_eq!(s.g.daytime.phase, Phase::Night);
    let mut bed = None;
    for _ in 0..4 {
        let h = s.press();
        if h.kind == HintKind::Bed {
            bed = Some(h);
            break;
        }
    }
    let bed = bed.expect("the bed is among the top hints at night");
    s.walk_to(cell_of(bed.stand), 300.0);
    s.face(bed.pos);
    s.idle(0.1);
    assert_eq!(s.g.interact(), Some(zoo_core::Interaction::Sleep));
    s.idle(zoo_core::daytime::SLEEP_S + 0.5);
    assert!(s.events.contains(&GameEvent::Morning));
    s.idle(zoo_core::daytime::MORNING_S + 0.5);
    // the new day: the night zoo waits -> the bed again (never the monkeys), with its line
    s.t.hide();
    let h = s.press();
    assert_eq!(h.kind, HintKind::Bed, "{h:?}");
    assert_eq!(hints::step_key_for(&s.g, &s.t, &h), "hint-bed-night-zoo");
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
    // GAME-HOUSE (HOUSE-016): animals at home rest inside their animal house in about half of
    // the states; they must come out for the child and never be a hint target
    for i in 0..g.animals.len() {
        let a = &g.animals[i];
        let inside = a.state == AnimalState::InEnclosure
            && r.next_u32().is_multiple_of(2)
            && g.houses[a.enclosure].is_some();
        if inside {
            let h = g.houses[a.enclosure].clone().unwrap();
            let cells = h.interior_cells();
            let c = cells[(a.member as usize) % cells.len()];
            let a = &mut g.animals[i];
            a.pos = cell_center(c);
            a.wander.route.clear();
            a.wander.rest_s = 15.0;
        }
    }
    // (a restore recomputes the wander areas, so the animals inside really rest)
    let save = g.to_save();
    let p = g.player.pos;
    *g = Game::from_save(zoo_data(), &save).unwrap();
    g.player.pos = p;
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

// ------------------------------------------------------------------ the terrarium garden (night_2)

/// The whole game with the terrarium garden, parsed once.
fn zoo2_data() -> zoo_core::LevelData {
    static DATA: std::sync::OnceLock<zoo_core::LevelData> = std::sync::OnceLock::new();
    DATA.get_or_init(common::zoo_with_night2).clone()
}

const GARDEN: [&str; 3] = ["snake", "chameleon", "poison_dart_frog"];

fn garden_done(s: &Sim) -> bool {
    GARDEN.iter().all(|a| s.g.mission(a).unwrap().complete)
}

/// A night in which night_1 is complete (the garden gate is open) and the child stands in night_1.
fn garden_sim(seed: u64) -> Sim {
    let mut g = Game::new(zoo2_data(), seed).expect("zoo with night_2");
    // level 1 is done (the night only falls after it)
    for a in DAY_1 {
        assert!(g.debug_send_home(a));
    }
    assert!(g.debug_set_daytime("night"));
    for a in ["hedgehog", "bat", "owl"] {
        assert!(g.debug_send_home(a));
    }
    g.drain_events();
    g.player.pos = cell_center(IVec2::new(-30, 29));
    Sim::new(g)
}

// HINT-025 / NIGHT-031 (NEVER STUCK, night_2): a child who only follows the 🧭 hint, from messy
// states of the garden (split pairs, a wrong food or a treat in the hands, babies, a save/restore in
// the middle), always brings all three pairs home, and then the hint leads to the bed and the morning.
#[test]
fn hint_025_never_stuck_following_the_hints_through_the_garden() {
    for seed in 0..6u64 {
        let mut s = garden_sim(seed);
        let mut r = Pcg32::new(seed + 7);
        // messy start: one pair split, a baby somewhere, a wrong food or a treat in the hands
        let snake = s.g.group("snake");
        s.g.animals[snake[0]].state = AnimalState::InEnclosure;
        s.g.missions[snake[0]].started = true;
        if seed % 2 == 0 {
            let mut save = s.g.to_save();
            save.babies = vec!["chameleon".to_owned()];
            let p = s.g.player.pos;
            s.g = Game::from_save(zoo2_data(), &save).unwrap();
            s.g.player.pos = p;
        }
        let noise = [Food::Grass, Food::Eggs, Food::Fish, Food::Flies, Food::Bone];
        let mut steps = 0;
        while !garden_done(&s) {
            steps += 1;
            assert!(
                steps < 90,
                "seed {seed}: the garden is not done after 90 hint steps"
            );
            if r.next_u32().is_multiple_of(4) {
                s.g.carry.take(&zoo_core::FoodBox {
                    food: noise[(r.next_u32() % 5) as usize],
                });
            }
            s.t.hide();
            follow_one(&mut s);
            if r.next_u32().is_multiple_of(5) {
                let save = s.g.to_save();
                let p = s.g.player.pos;
                s.g = Game::from_save(zoo2_data(), &save).unwrap();
                s.g.player.pos = p;
            }
        }
        // all home: the hint leads to the bed and the morning comes
        let mut tail = 0;
        while s.g.daytime.phase != Phase::Day {
            tail += 1;
            assert!(
                tail < 12,
                "seed {seed}: no morning after {tail} steps ({:?})",
                s.g.daytime.phase
            );
            s.t.hide();
            follow_one(&mut s);
        }
    }
}

// NIGHT-031 (core): the gate is the first hint in night_1 while night_2 waits; its priority is <= 3
// and following it brings the child into the garden, where the hint is a board of the garden.
#[test]
fn night_031_gate_hint_leads_into_the_garden() {
    let mut s = garden_sim(2);
    let h = s.press();
    assert_eq!(h.kind, HintKind::NightGate, "{h:?}");
    assert!(h.priority <= hints::PRIO_UNSTARTED);
    s.t.hide();
    follow_one(&mut s);
    let here = s.g.level.data.part_at(cell_of(s.g.player.pos));
    assert_eq!(
        here,
        s.g.level.data.part_index("night_2"),
        "{:?}",
        s.g.player.pos
    );
    s.t.hide();
    let h = s.press();
    assert!(
        h.kind == HintKind::Board && h.animal.is_some_and(|a| GARDEN.contains(&a)),
        "{h:?}"
    );
}

// ---------------------------------------------------------------------------------------------
// Cart key hints (GAME-CART rule 16, GAME-HINT "Cart key hints"): HINT-032, HINT-033, CART-029

fn cart_hint(s: &Sim) -> Option<hints::Hint> {
    candidates(&s.g, &s.t)
        .into_iter()
        .find(|h| matches!(h.kind, HintKind::Note | HintKind::KeyBox))
}

/// The child takes the cart hint (the other optional hints rank before it): press until it is
/// the shown one, then follow it.
fn follow_cart(s: &mut Sim) {
    let h = cart_hint(s).expect("a cart hint");
    s.t.hide();
    s.check_loop(&h);
    follow_hint(s, h);
}

/// Every animal home, every pair has its baby: no garden / treat hint, only the cart hints.
fn quiet_day(seed: u64) -> Sim {
    let mut s = day_all_home(seed, true);
    let ids: Vec<String> = s.g.animals.iter().map(|a| a.id().to_owned()).collect();
    for id in ids {
        if !s.g.babies.contains(&id) {
            s.g.babies.push(id);
        }
    }
    s
}

fn at_box(s: &mut Sim) {
    let it =
        s.g.level
            .data
            .items
            .iter()
            .find(|i| i.id == "key_box_l1")
            .unwrap();
    let (pos, stand) = (it.pos(), it.stand.unwrap());
    s.g.player.pos = cell_center(IVec2::from(stand));
    s.g.player.facing = (pos - s.g.player.pos).normalize();
}

// HINT-032: the stages note -> key box -> (3 wrong codes) note -> key box -> none
#[test]
fn hint_032_cart_key_stages() {
    let mut s = quiet_day(3);
    let h = cart_hint(&s).expect("the note hint");
    assert_eq!((h.kind, h.id.as_str()), (HintKind::Note, "cart-note"));
    assert_eq!(h.priority, hints::PRIO_OPTIONAL);
    assert_eq!(h.kind.id(), "note");
    assert_eq!(h.kind.step_key(), "hint-cart-note");
    let note =
        s.g.level
            .data
            .items
            .iter()
            .find(|i| i.kind == "note_math_fighter")
            .unwrap();
    assert!(h.pos.distance(note.pos()) < 1e-3);
    // stage 1 -> 2: reading the note
    let sig = s.g.hint_signature();
    follow_cart(&mut s);
    assert!(s.g.note_read);
    assert_ne!(s.g.hint_signature(), sig, "the stage changes the signature");
    let h = cart_hint(&s).expect("the key box hint");
    assert_eq!((h.kind, h.id.as_str()), (HintKind::KeyBox, "cart-keybox"));
    assert_eq!(h.kind.id(), "keybox");
    assert_eq!(h.kind.step_key(), "hint-cart-keybox");
    // 3 wrong codes -> the note again; each wrong code is a state change
    at_box(&mut s);
    let wrong = s.g.cart_task().answer % 999 + 1;
    let mut sigs = vec![s.g.hint_signature()];
    for i in 1..=3 {
        s.g.enter_code(wrong);
        sigs.push(s.g.hint_signature());
        let kind = cart_hint(&s).unwrap().kind;
        assert_eq!(
            kind,
            if i < 3 {
                HintKind::KeyBox
            } else {
                HintKind::Note
            },
            "after {i}"
        );
    }
    sigs.dedup();
    assert_eq!(sigs.len(), 4, "every wrong code changes the signature");
    follow_cart(&mut s); // reads the note: tries = 0
    assert_eq!(s.g.key_box_tries, 0);
    assert_eq!(cart_hint(&s).unwrap().kind, HintKind::KeyBox);
    // the right code: no cart hint ever again
    at_box(&mut s);
    let answer = s.g.cart_task().answer;
    s.g.enter_code(answer);
    assert!(s.g.has_cart_key);
    assert!(cart_hint(&s).is_none());
    s.idle(200.0);
    assert!(cart_hint(&s).is_none(), "never after the key was taken");
}

// HINT-032: never by night, never in front of a mission step, never without the items
#[test]
fn hint_032_cart_hint_ranks_behind_everything_and_not_at_night() {
    let s = Sim::new(night_game(5));
    let list = candidates(&s.g, &s.t);
    assert!(list[0].priority <= hints::PRIO_UNSTARTED, "{:?}", list[0]);
    let pos = list
        .iter()
        .position(|h| h.kind == HintKind::Note)
        .expect("listed");
    assert!(list[..pos]
        .iter()
        .all(|h| h.priority <= hints::PRIO_OPTIONAL));
    assert!(list[pos..]
        .iter()
        .all(|h| h.priority >= hints::PRIO_OPTIONAL));
    // dusk and night: carts do not exist, no cart hint
    let mut s = day_all_home(3, true);
    assert!(cart_hint(&s).is_some());
    let _ = s.g.debug_set_daytime("night");
    assert!(cart_hint(&s).is_none());
}

// HINT-033: the cart hints never block "what next" / all_done and never loop
#[test]
fn hint_033_no_block_no_loop() {
    let mut s = quiet_day(3);
    assert!(cart_hint(&s).is_some());
    assert_eq!(hints::what_next(&s.g, &s.t), Some("all_done"));
    assert_eq!(compass_badge(&s.g, &s.t).map(|b| b.0), Some("all_done"));
    // the repeat guard: reached twice without a change -> dropped, the read-again fallback
    // (priority 6) remains, so the 🧭 still answers
    let first = s.press();
    assert_eq!(first.id, "cart-note");
    for _ in 0..2 {
        s.t.hide();
        s.g.player.pos = first.stand;
        // (looking away: the child does not read the note)
        s.g.player.facing = (first.stand - first.pos).normalize();
        s.t.press(&s.g);
        s.idle(0.5);
        s.t.hide();
    }
    assert!(!s.g.note_read);
    assert!(
        cart_hint(&s).is_none(),
        "reached twice without a change: dropped"
    );
    let h = s.press();
    assert_eq!(h.priority, hints::PRIO_FALLBACK, "{h:?}");
    // a change of the game state brings it back
    s.g.note_read = true;
    assert_eq!(cart_hint(&s).unwrap().kind, HintKind::KeyBox);
}

/// A trip to the golf cart of level 1 (CART-029): tap it (locked feedback + hint marker before
/// the key), or board, drive a little and get out again.
fn cart_trip(s: &mut Sim, rng: &mut Pcg32) {
    let stand = s.g.carts[0].stand;
    s.g.player.pos = cell_center(stand);
    s.g.player.facing = (s.g.carts[0].pos - s.g.player.pos).normalize();
    if s.g.seated.is_some() {
        return;
    }
    let before = s.g.hint_signature();
    match s.g.interact() {
        Some(zoo_core::game::Interaction::CartLocked { .. }) => {
            assert!(!s.g.has_cart_key);
            let shown = s.t.show_cart_key(&s.g).map(|h| h.kind);
            assert!(
                matches!(shown, Some(HintKind::Note | HintKind::KeyBox)),
                "the marker points at the next key step: {shown:?}"
            );
            assert!((s.t.time_left() - 12.0).abs() < 1e-3);
        }
        Some(zoo_core::game::Interaction::CartBoarded { .. }) => {
            for k in 0..60 {
                let a = (k / 15) as f32 * 1.7 + rng.below(6) as f32;
                s.g.update(1.0 / 30.0, glam::Vec2::new(a.cos(), a.sin()));
            }
            for k in 0..400 {
                if s.g.leave_cart() == zoo_core::cart::LeaveResult::Left {
                    break;
                }
                let a = (k / 40) as f32 * 1.3;
                s.g.update(1.0 / 30.0, glam::Vec2::new(a.cos(), a.sin()));
            }
            assert!(s.g.seated.is_none(), "she can always get out");
        }
        other => panic!("cart tap: {other:?}"),
    }
    // cart actions are no progress of the hint signature (no hint loop through them)
    assert_eq!(before, s.g.hint_signature());
}

// CART-029: the follow-the-hints fuzz with cart actions never loops and reaches the key; once
// with only the cart hints left (quiet day), once with the garden / treat hints around
#[test]
fn cart_029_fuzz_follow_hints_with_cart_actions() {
    for (seed, quiet) in [
        (1u64, true),
        (2, true),
        (3, true),
        (4, true),
        (1, false),
        (2, false),
        (5, false),
    ] {
        let mut s = if quiet {
            quiet_day(seed)
        } else {
            day_all_home(seed, true)
        };
        let mut rng = Pcg32::new(seed * 77);
        let mut steps = 0;
        while !s.g.has_cart_key {
            steps += 1;
            assert!(
                steps < 150,
                "seed {seed} quiet {quiet}: no key after {steps} steps"
            );
            match rng.below(7) {
                // wrong codes, level changes, save + restore (the tracker is not saved: a new
                // session starts without repeat-guard memory, so the loop detector restarts)
                0 => {
                    at_box(&mut s);
                    let wrong = s.g.cart_task().answer % 999 + 1;
                    s.g.enter_code(wrong);
                }
                1 => {
                    let l = zoo_core::math::MathLevel::ALL[rng.below(5) as usize];
                    s.g.set_math_level(l);
                }
                2 => {
                    let json = s.g.to_save().to_json();
                    s.g = Game::from_save_json(zoo_data(), &json).expect("restores");
                    s.t = HintTracker::default();
                    s.run = (String::new(), 0, 0);
                }
                3 => cart_trip(&mut s, &mut rng),
                _ => {}
            }
            s.t.hide();
            // never stuck: a hint always exists
            assert!(!candidates(&s.g, &s.t).is_empty());
            // the child follows the best hint; now and then she goes for the cart hint
            if cart_hint(&s).is_some() && rng.below(3) == 0 {
                follow_cart(&mut s);
            } else {
                follow_one(&mut s);
            }
        }
        assert!(cart_hint(&s).is_none());
        // afterwards the hints go on without loops, 40 more steps (the carts are usable now)
        for k in 0..40 {
            s.t.hide();
            if k % 7 == 0 {
                cart_trip(&mut s, &mut rng);
            }
            follow_one(&mut s);
        }
    }
}
