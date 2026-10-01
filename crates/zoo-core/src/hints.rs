//! Next-target hint (GAME-HINT) and the night progress indicator (GAME-NIGHT "Night
//! progress"): pure, deterministic logic; the host only draws the 🧭 button, the bouncing
//! indicator, the edge arrow and the 🌙 progress HUD.
//!
//! * [`candidates`] lists every currently useful target, best first (priority, then the
//!   nearest). Targets that do not exist in the game yet (events, golf carts, the key box,
//!   the map board, ad boards) are skipped.
//! * [`HintTracker`] keeps what is shown (12 s, cycling through the top 3 on repeated
//!   presses), the riddle-fairness timers (rule 4: board first, the hiding-area edge after
//!   60 s) and the idle nudge (rule 6: the 🧭 button pulses after 90 s without a useful
//!   action).
//! * [`night_progress`] tells the child what is still missing before night falls.

use std::collections::{BTreeMap, HashSet};

use glam::{IVec2, Vec2, Vec4};

use crate::animals::AnimalState;
use crate::daytime::Phase;
use crate::food::Food;
use crate::game::{Animal, Game, GameEvent, Target};
use crate::level::{cell_center, cell_of};

/// A hint stays visible this long (rule 2).
pub const HINT_SHOW_S: f32 = 12.0;
/// Repeated presses within [`HINT_SHOW_S`] cycle through this many targets (rule 3).
pub const HINT_CYCLE: usize = 3;
/// Rule 4: after the board was read and this long without finding the animal, the hint
/// points at the edge of its hiding area (Q-127).
pub const AREA_HINT_AFTER_S: f32 = 60.0;
/// Rule 4: the hiding-area circle is at least this wide.
pub const AREA_MIN_DIAMETER_M: f32 = 6.0;
/// Rule 4a: right after the board was read the search area is this wide (at least), so the
/// hint answers at once but stays a riddle (Q-195).
pub const WIDE_AREA_MIN_DIAMETER_M: f32 = 12.0;
/// Rule 6: the 🧭 button pulses after this long without a useful action (Q-127).
pub const IDLE_NUDGE_S: f32 = 90.0;
/// A shown target counts as reached within this distance (rule 2: "until reached").
pub const REACHED_M: f32 = 2.5;
/// The shown target is re-evaluated this often while visible (s).
const REFRESH_S: f32 = 0.25;
/// The walking distance to the shown target is re-measured this often (s).
const WALK_REFRESH_S: f32 = 1.0;
/// One distance dot of the edge arrow per this many metres of walking (1…5 dots).
pub const DOT_M: f32 = 10.0;
pub const MAX_DOTS: u32 = 5;

/// What the child does at a hint target (the host shows it as a small icon, rule 2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HintKind {
    /// 📋 read an info board.
    Board,
    /// 📦 take food in the food storage.
    Food,
    /// 🐾 the hiding area of an animal (never the animal itself, rule 4).
    Animal,
    /// 🚪 lead the animal home through its gate.
    Gate,
    /// 📦 pick up a lying item the mission needs (GAME-FEED §13).
    PickUp,
    /// 🎋 cut bamboo at a ripe cut spot (GAME-FEED §16).
    Bamboo,
    /// 💧 fill the fish bowl.
    Water,
    /// 🥕 the vegetable garden.
    Garden,
    /// 🧺 a treat for an animal at home.
    Treat,
    /// 🛏 the bed.
    Bed,
    /// 🆘 the missing animal itself (NEVER STUCK: the stall help, or the safety net when a
    /// mission has no other step).
    Help,
    /// 🌙 the moon door.
    MoonDoor,
}

impl HintKind {
    /// Id for the host (icon).
    pub fn id(self) -> &'static str {
        match self {
            HintKind::Board => "board",
            HintKind::Food => "food",
            HintKind::Animal => "animal",
            HintKind::Gate => "gate",
            HintKind::PickUp => "pick_up",
            HintKind::Bamboo => "bamboo",
            HintKind::Water => "water",
            HintKind::Garden => "garden",
            HintKind::Treat => "treat",
            HintKind::Bed => "bed",
            HintKind::Help => "help",
            HintKind::MoonDoor => "moon_door",
        }
    }

    /// Fluent key of the next-step line shown with the hint (rule 4a, `hint-<step>`).
    pub fn step_key(self) -> &'static str {
        match self {
            HintKind::Board => "hint-read",
            HintKind::Food => "hint-food",
            HintKind::Animal => "hint-search",
            HintKind::Gate => "hint-home",
            HintKind::PickUp => "hint-pickup",
            HintKind::Bamboo => "hint-bamboo",
            HintKind::Water => "hint-water",
            HintKind::Garden => "hint-garden",
            HintKind::Treat => "hint-treat",
            HintKind::Bed => "hint-bed",
            HintKind::Help => "hint-help",
            HintKind::MoonDoor => "hint-moon",
        }
    }
}

/// Priorities of rule 3 (lower = better). Events (1) do not exist yet.
pub const PRIO_EVENT: u8 = 1;
pub const PRIO_MISSION: u8 = 2;
pub const PRIO_UNSTARTED: u8 = 3;
pub const PRIO_OPTIONAL: u8 = 4;
/// Rule 3.5: the bed and the moon door. They only exist as targets when they are the way
/// forward (dusk, night, or by day while the night zoo waits), so they rank before the
/// optional activities (proposal Q-189).
pub const PRIO_NIGHT: u8 = 3;
/// Fallback so the game is never stuck (HINT-008): read a board again.
pub const PRIO_FALLBACK: u8 = 6;

/// One hint target.
#[derive(Debug, Clone, PartialEq)]
pub struct Hint {
    /// Stable identity (`board:zebra`, `storage:0`, `area:zebra`, `gate:enc_zebra`, …).
    pub id: String,
    pub kind: HintKind,
    pub priority: u8,
    /// Where the indicator stands (level coordinates).
    pub pos: Vec2,
    /// Height of the indicator above the ground (m).
    pub height: f32,
    /// A walkable point from where the child uses the target (scripted player, tests).
    pub stand: Vec2,
    /// The mission (animal) the hint belongs to.
    pub animal: Option<&'static str>,
    /// Stands for a target on the other side of the moon door (ranks after the targets on
    /// this side of the same priority).
    pub via_door: bool,
    /// Ranks after the other targets of the same priority (the re-read board next to the
    /// search area, rule 4a).
    pub after_area: bool,
}

fn hint(id: String, kind: HintKind, priority: u8, pos: Vec2, stand: Vec2) -> Hint {
    let height = match kind {
        HintKind::Food => 3.2,
        HintKind::MoonDoor => 3.6,
        HintKind::Board | HintKind::Gate => 2.6,
        HintKind::Bamboo => 3.2,
        _ => 2.2,
    };
    Hint {
        id,
        kind,
        priority,
        pos,
        height,
        stand,
        animal: None,
        via_door: false,
        after_area: false,
    }
}

/// Walkable point near `p`, on the side of `from` (the scripted player stands there).
fn stand_near(g: &Game, p: Vec2, from: Vec2, ahead_m: f32) -> Vec2 {
    let dir = (from - p).normalize_or(Vec2::NEG_Y);
    crate::save::nearest_walkable(g, p + dir * ahead_m, false)
}

/// Whether a level point lies in a night level (the night zoo).
fn in_night_zone(g: &Game, p: Vec2) -> bool {
    g.level
        .data
        .part_at(cell_of(p))
        .is_some_and(|k| g.level.data.is_night_part(k))
}

/// Circle of an animal's hiding area (rule 4): centre (the place's spot) and radius — at
/// least [`AREA_MIN_DIAMETER_M`] wide (`wide`: [`WIDE_AREA_MIN_DIAMETER_M`], rule 4a) and 1 m
/// wider than where the animal wanders, so the edge is never the animal's position.
pub fn hiding_circle(g: &Game, a: &Animal, wide: bool) -> Option<(Vec2, f32)> {
    let place = g.level.data.hiding_place(&a.hiding_place)?;
    let min_d = if wide {
        WIDE_AREA_MIN_DIAMETER_M
    } else {
        AREA_MIN_DIAMETER_M
    };
    let r = (min_d / 2.0).max(place.wander_radius_m) + 1.0;
    Some((cell_center(place.spot_cell()), r))
}

/// The point of the hiding-area circle nearest to the player.
fn circle_edge(center: Vec2, r: f32, from: Vec2) -> Vec2 {
    center + (from - center).normalize_or(Vec2::NEG_Y) * r
}

/// Every currently useful target, best first: rule 3 priorities, the nearest first within
/// a priority; the same id only once. Targets on the other side of the moon door are
/// replaced by the (open) moon door; unreachable ones are dropped. Never empty while the
/// zoo has an info board in scope, a bed or a moon door (HINT-008).
pub fn candidates(g: &Game, t: &HintTracker) -> Vec<Hint> {
    let p = g.player.pos;
    let mut out: Vec<Hint> = Vec::new();
    let its = g.interactables();
    let data = &g.level.data;

    // --- boards (their interaction point and readable side)
    let board = |animal: &str| {
        its.iter().find_map(|it| match it.target {
            Target::InfoBoard { animal: a } if a == animal => {
                let dir = it.readable.unwrap_or(Vec2::NEG_Y);
                let stand = crate::save::nearest_walkable(g, it.point + dir * 1.0, false);
                Some((it.point, stand))
            }
            _ => None,
        })
    };

    // --- the nearest unlocked food storage: its door (not the right box — reading the
    // labels stays the child's job, GAME-FEED §16, Q-187)
    let storage = || -> Option<Hint> {
        let mut parts: BTreeMap<usize, (Vec2, Vec2, u32)> = BTreeMap::new();
        for b in &data.food_boxes {
            if !g.part_unlocked(b.part) {
                continue;
            }
            let e = parts.entry(b.part).or_insert((Vec2::ZERO, Vec2::ZERO, 0));
            e.0 += b.pos();
            e.1 += b.facing();
            e.2 += 1;
        }
        parts
            .into_iter()
            .map(|(k, (sum, facing, n))| {
                // the storage's door (the gap in the box row, Q-150) — with the labelled boxes
                // outside in a row (Q-181 answered) the row's centre could be the right box
                // itself; without a storage building the centre of the boxes
                let door = data
                    .elements
                    .iter()
                    .filter(|e| e.part == k)
                    .filter(|e| matches!(e.kind.as_deref(), Some("food_storage" | "food_hut")))
                    .find_map(|e| Some((e.door_cell()?, crate::scene::door_facade(e)?)));
                let (c, f) = match door {
                    Some((d, dir)) => (cell_center(d), dir.offset().as_vec2()),
                    None => (
                        sum / n as f32,
                        (facing / n as f32).normalize_or(Vec2::NEG_Y),
                    ),
                };
                let stand = crate::save::nearest_walkable(g, c + f * 1.1, false);
                hint(
                    format!("storage:{k}"),
                    HintKind::Food,
                    PRIO_MISSION,
                    c,
                    stand,
                )
            })
            .min_by(|a, b| a.pos.distance(p).total_cmp(&b.pos.distance(p)))
    };

    // --- missions (rule 3.2 / 3.3)
    let carried = g.carry.food();
    let leading: Option<&'static str> = g
        .animals
        .iter()
        .find(|a| a.state == AnimalState::Following || a.state == AnimalState::InBowl)
        .map(|a| a.id());
    // the mission whose food is in the hands is the current one
    let focus: Option<&'static str> = leading.or_else(|| {
        g.animals
            .iter()
            .filter(|a| a.state == AnimalState::Escaped && g.in_scope(a))
            .filter(|a| g.mission(a.id()).is_some_and(|m| m.started && !m.complete))
            .find(|a| carried.is_some_and(|f| a.info.eats(f)))
            .map(|a| a.id())
    });
    let mut seen: HashSet<&'static str> = HashSet::new();
    for a in &g.animals {
        let id = a.id();
        if !g.in_scope(a) || !seen.insert(id) {
            continue;
        }
        let Some(m) = g.mission(id) else { continue };
        if m.complete {
            continue;
        }
        let prio = if focus.is_none() || focus == Some(id) {
            PRIO_MISSION
        } else {
            PRIO_UNSTARTED
        };
        let group: Vec<&Animal> = g.animals.iter().filter(|x| x.id() == id).collect();
        let moving = group
            .iter()
            .any(|x| matches!(x.state, AnimalState::Following | AnimalState::InBowl));
        if moving {
            // a partner left far behind (RESC-006): fetch it first, the pair enters together
            if let Some(w) = group
                .iter()
                .find(|x| x.state == AnimalState::Following && x.waiting)
            {
                let mut h = hint(
                    format!("partner:{id}"),
                    HintKind::Animal,
                    PRIO_MISSION,
                    w.pos,
                    stand_near(g, w.pos, p, 3.0),
                );
                h.animal = Some(id);
                out.push(h);
                continue;
            }
            // animal following (or in the carried bowl) → its gate
            let enc = &data.elements[a.enclosure];
            if let Some(gr) = enc.gate {
                let c = Vec2::new(
                    gr.x as f32 + gr.w as f32 / 2.0,
                    gr.z as f32 + gr.d as f32 / 2.0,
                );
                let r = enc.rect;
                let outside = |q: Vec2| {
                    let cell = cell_of(q);
                    !r.contains(cell) && g.level.grid().is_walkable(cell, false)
                };
                let stand = crate::level::Rect::new(c.x as i32 - 3, c.y as i32 - 3, 7, 7)
                    .cells()
                    .map(cell_center)
                    .filter(|&q| outside(q) && q.distance(c) >= 0.9)
                    .min_by(|x, y| x.distance(c).total_cmp(&y.distance(c)))
                    .unwrap_or(c);
                let mut h = hint(
                    format!("gate:{}", enc.id),
                    HintKind::Gate,
                    PRIO_MISSION,
                    c,
                    stand,
                );
                h.animal = Some(id);
                out.push(h);
            }
            continue;
        }
        if !m.started {
            if let Some((pt, stand)) = board(id) {
                let mut h = hint(
                    format!("board:{id}"),
                    HintKind::Board,
                    PRIO_UNSTARTED,
                    pt,
                    stand,
                );
                h.animal = Some(id);
                out.push(h);
            }
            continue;
        }
        let mut steps: Vec<Hint> = Vec::new();
        // the goldfish needs the bowl, filled with water (GAME-RESCUE "goldfish bowl")
        let mut container_ready = true;
        if g.needs_container(id) {
            if let Some(b) = &g.bowl {
                if !b.carried {
                    container_ready = false;
                    let stand = stand_near(g, b.pos, p, 0.9);
                    steps.push(hint(
                        format!("item:{}", b.id),
                        HintKind::PickUp,
                        prio,
                        b.pos,
                        stand,
                    ));
                } else if !b.water {
                    container_ready = false;
                    if let Some((src, q)) = g.water_point() {
                        let stand = stand_near(g, q, p, 0.9);
                        steps.push(hint(
                            format!("water:{src}"),
                            HintKind::Water,
                            prio,
                            q,
                            stand,
                        ));
                    }
                }
            }
        }
        let right = carried.is_some_and(|f| a.info.eats(f));
        if !right {
            // food missing → the storage, a lying right food, ripe bamboo (GAME-FEED §13/§16)
            if let Some(mut s) = storage() {
                s.priority = prio;
                steps.push(s);
            }
            for f in &g.lying.foods {
                if a.info.eats(f.food) && its.iter().any(|it| it.target == lying(f.uid, f.food)) {
                    let stand = stand_near(g, f.pos, p, 0.9);
                    steps.push(hint(
                        format!("lying:{}", f.uid),
                        HintKind::PickUp,
                        prio,
                        f.pos,
                        stand,
                    ));
                }
            }
            if a.info.eats(Food::Bamboo) {
                for c in &data.cut_spots {
                    let ripe = its
                        .iter()
                        .any(|it| it.target == Target::CutSpot { spot: c.id.clone() });
                    if ripe {
                        steps.push(hint(
                            format!("cut:{}", c.id),
                            HintKind::Bamboo,
                            prio,
                            c.pos(),
                            Vec2::from(c.stand),
                        ));
                    }
                }
            }
        } else if container_ready {
            // right food: the search area at once — wide first, the exact circle after 60 s
            // (rule 4a, Q-195); the board (read the riddle again) stays the second choice
            let wide = t.search_s(id) < AREA_HINT_AFTER_S;
            if let Some((c, r)) = hiding_circle(g, a, wide) {
                if p.distance(c) > r {
                    let edge = circle_edge(c, r, p);
                    let stand = crate::save::nearest_walkable(g, edge, false);
                    steps.push(hint(
                        format!("area:{id}"),
                        HintKind::Animal,
                        prio,
                        edge,
                        stand,
                    ));
                }
            }
            if let Some((pt, stand)) = board(id) {
                let mut b = hint(format!("board:{id}"), HintKind::Board, prio, pt, stand);
                b.after_area = true;
                steps.push(b);
            }
        }
        for mut h in steps {
            h.animal = Some(id);
            out.push(h);
        }
    }

    // --- never stuck (rule 3.1b): an open mission always has a hint of priority <= 3; after
    // a long stall (STALL_HELP_S) the hint points at the missing animal itself
    let mut helped: HashSet<&'static str> = HashSet::new();
    for a in &g.animals {
        let id = a.id();
        if !g.in_scope(a) || !helped.insert(id) || g.mission(id).is_none_or(|m| m.complete) {
            continue;
        }
        let stalled = g.stall_s() >= crate::game::STALL_HELP_S;
        let has = out
            .iter()
            .any(|h| h.animal == Some(id) && h.priority <= PRIO_UNSTARTED);
        if has && !stalled {
            continue;
        }
        // the member that is not home (the escaped one first)
        let Some(m) = g
            .animals
            .iter()
            .filter(|x| x.id() == id && x.state != AnimalState::InEnclosure)
            .min_by_key(|x| x.state != AnimalState::Escaped)
        else {
            continue;
        };
        let mut h = hint(
            format!("help:{id}"),
            HintKind::Help,
            if stalled { PRIO_EVENT } else { PRIO_MISSION },
            m.pos,
            stand_near(g, m.pos, p, 2.0),
        );
        h.animal = Some(id);
        out.push(h);
    }

    // --- optional activities (rule 3.4): ripe plants if the basket has room, treats
    let room = g.garden.basket.total() < crate::garden::BASKET_CAPACITY;
    for it in &its {
        match &it.target {
            Target::Plant { spot } if room => {
                let stand = stand_near(g, it.point, p, 0.9);
                out.push(hint(
                    format!("plant:{spot}"),
                    HintKind::Garden,
                    PRIO_OPTIONAL,
                    it.point,
                    stand,
                ));
            }
            _ => {}
        }
    }
    if leading.is_none() {
        let mut seen: HashSet<&'static str> = HashSet::new();
        for a in &g.animals {
            if a.state != AnimalState::InEnclosure
                || !g.in_scope(a)
                || !g.gift_liked(a.id())
                || !seen.insert(a.id())
            {
                continue;
            }
            // the member of the group nearest to the player: the child walks up to it (inside
            // the enclosure) or to its fence, where it waits (GAME-GARDEN §6)
            let Some(it) = its
                .iter()
                .find(|it| it.target == Target::Treat { animal: a.id() })
            else {
                continue;
            };
            let r = data.elements[a.enclosure].rect;
            let min = Vec2::new(r.x as f32, r.z as f32);
            let max = min + Vec2::new(r.w as f32, r.d as f32);
            let fence = p.clamp(min, max);
            let (point, stand) = if fence == p {
                (it.point, stand_near(g, it.point, p, 0.9))
            } else if let Some(s) = g.feed_spot(a.enclosure) {
                // the feeding spot next to the gate: there the animals wait (GARD-010)
                let c = (cell_center(s.cells[0]) + cell_center(s.cells[1])) / 2.0;
                let spot = c - Vec2::new(s.inward.x as f32, s.inward.y as f32) * 0.5;
                (spot, crate::save::nearest_walkable(g, s.stand, false))
            } else {
                (fence, stand_near(g, fence, p, 0.9))
            };
            let mut h = hint(
                format!("treat:{}", a.id()),
                HintKind::Treat,
                PRIO_OPTIONAL,
                point,
                stand,
            );
            h.animal = Some(a.id());
            out.push(h);
        }
    }

    // --- night (rule 3.5): the bed, the moon door; at dusk the bed is next
    let dusk = g.daytime.phase == Phase::Dusk || g.daytime.dusk_in.is_some();
    if g.bed_usable() || dusk {
        if let Some(stand) = g.bed_stand(p) {
            let bed = g
                .beds()
                .into_iter()
                .min_by(|a, b| a.distance(p).total_cmp(&b.distance(p)))
                .unwrap_or(stand);
            out.push(hint("bed".into(), HintKind::Bed, PRIO_NIGHT, bed, stand));
        }
    }
    // (from the night zoo the door back is only a way to a target over there)
    let doors = open_moon_doors(g);
    let here_night = in_night_zone(g, p);
    if g.daytime.is_night() && !here_night {
        out.extend(doors.iter().cloned());
    }

    // --- the other side of the moon door: go through the door first
    let mut fixed: Vec<Hint> = Vec::with_capacity(out.len());
    for h in out {
        if h.kind == HintKind::MoonDoor || in_night_zone(g, h.pos) == here_night {
            fixed.push(h);
        } else if let Some(d) = doors
            .iter()
            .min_by(|a, b| a.pos.distance(p).total_cmp(&b.pos.distance(p)))
        {
            let mut d = d.clone();
            d.priority = h.priority;
            d.animal = h.animal;
            d.via_door = true;
            fixed.push(d);
        }
    }
    let mut out = fixed;

    // --- never stuck (HINT-008): read the nearest board again, else the bed / moon door
    if out.is_empty() {
        let fallback = its
            .iter()
            .filter_map(|it| match it.target {
                Target::InfoBoard { animal } => Some((animal, it)),
                _ => None,
            })
            .filter(|(_, it)| in_night_zone(g, it.point) == here_night)
            .min_by(|a, b| a.1.point.distance(p).total_cmp(&b.1.point.distance(p)));
        if let Some((animal, it)) = fallback {
            let dir = it.readable.unwrap_or(Vec2::NEG_Y);
            let stand = crate::save::nearest_walkable(g, it.point + dir, false);
            let mut h = hint(
                format!("board:{animal}"),
                HintKind::Board,
                PRIO_FALLBACK,
                it.point,
                stand,
            );
            h.animal = Some(animal);
            out.push(h);
        } else if let Some(stand) = g.bed_stand(p) {
            let bed = g.bed().map_or(stand, |b| b.0);
            out.push(hint("bed".into(), HintKind::Bed, PRIO_FALLBACK, bed, stand));
        } else if let Some(d) = doors.first() {
            let mut d = d.clone();
            d.priority = PRIO_FALLBACK;
            out.push(d);
        }
    }

    out.sort_by(|a, b| {
        a.priority
            .cmp(&b.priority)
            .then(a.via_door.cmp(&b.via_door))
            .then(a.after_area.cmp(&b.after_area))
            .then(a.pos.distance(p).total_cmp(&b.pos.distance(p)))
            .then(a.id.cmp(&b.id))
    });
    let mut ids: HashSet<String> = HashSet::new();
    out.retain(|h| ids.insert(h.id.clone()));
    out
}

fn lying(uid: u32, food: Food) -> Target {
    Target::LyingFood { uid, food }
}

/// The open moon doors as hints (the side of the player).
fn open_moon_doors(g: &Game) -> Vec<Hint> {
    let p = g.player.pos;
    g.moon_doors()
        .into_iter()
        .filter(|id| g.level.is_barrier_open(id))
        .filter_map(|id| {
            let r = g.level.data.element(&id)?.rect;
            let c = Vec2::new(r.x as f32 + r.w as f32 / 2.0, r.z as f32 + r.d as f32 / 2.0);
            let stand = stand_near(g, c, p, 1.2);
            Some(hint(
                format!("moon_door:{id}"),
                HintKind::MoonDoor,
                PRIO_NIGHT,
                c,
                stand,
            ))
        })
        .collect()
}

/// Whether an event is a useful action of the child (rule 6): any interaction result, not
/// the animals' own reactions.
pub fn is_useful(e: &GameEvent) -> bool {
    e.is_progress()
        || matches!(
            e,
            GameEvent::PanelOpened { .. }
                | GameEvent::NotInterested { .. }
                | GameEvent::Refuse { .. }
                | GameEvent::NeedsContainer { .. }
                | GameEvent::ContainerEmpty { .. }
                | GameEvent::TreatRefused { .. }
                | GameEvent::FoodRefused { .. }
                | GameEvent::BasketFull
        )
}

/// What the hint shows and remembers (host side state, not saved: a reload starts fresh).
#[derive(Debug, Clone, Default)]
pub struct HintTracker {
    shown: Option<Hint>,
    left_s: f32,
    /// Ids of the top targets at the first press (cycled by further presses, rule 3).
    cycle: Vec<String>,
    index: usize,
    refresh_s: f32,
    walk_s: f32,
    walk_m: f32,
    /// Seconds since the board of an animal was read, while it is still hidden (rule 4).
    searching: BTreeMap<&'static str, f32>,
    idle_s: f32,
    visited: HashSet<IVec2>,
    pulses: u32,
}

impl HintTracker {
    /// The hint button (🧭, `H`, or tapping the 🌙 progress): shows the best target, or —
    /// pressed again while a hint is shown — the next of the top [`HINT_CYCLE`].
    pub fn press(&mut self, g: &Game) -> Option<&Hint> {
        self.idle_s = 0.0;
        let list = candidates(g, self);
        if list.is_empty() {
            self.hide();
            return None;
        }
        let pick = if self.shown.is_some() && !self.cycle.is_empty() {
            // the next id of the remembered cycle that is still a candidate
            let n = self.cycle.len();
            (1..=n).map(|k| (self.index + k) % n).find_map(|i| {
                list.iter()
                    .find(|h| h.id == self.cycle[i])
                    .map(|h| (i, h.clone()))
            })
        } else {
            None
        };
        let (index, h) = match pick {
            Some(x) => x,
            None => {
                self.cycle = list.iter().take(HINT_CYCLE).map(|h| h.id.clone()).collect();
                (0, list[0].clone())
            }
        };
        self.index = index;
        self.left_s = HINT_SHOW_S;
        self.refresh_s = REFRESH_S;
        self.walk_s = 0.0;
        self.walk_m = walking_distance(g, h.pos);
        self.shown = Some(h);
        self.shown.as_ref()
    }

    /// Hides the hint.
    pub fn hide(&mut self) {
        self.shown = None;
        self.left_s = 0.0;
        self.cycle.clear();
        self.index = 0;
    }

    /// The target shown now.
    pub fn shown(&self) -> Option<&Hint> {
        self.shown.as_ref()
    }

    /// Seconds the hint is still shown.
    pub fn time_left(&self) -> f32 {
        self.left_s
    }

    /// Walking distance to the shown target (m, measured about once a second).
    pub fn walk_m(&self) -> f32 {
        self.walk_m
    }

    /// Distance dots of the edge arrow (1…[`MAX_DOTS`], one per [`DOT_M`] of walking).
    pub fn dots(&self) -> u32 {
        ((self.walk_m / DOT_M).ceil() as u32).clamp(1, MAX_DOTS)
    }

    /// How long the child has been looking for an animal since reading its board (rule 4).
    pub fn search_s(&self, animal: &str) -> f32 {
        self.searching.get(animal).copied().unwrap_or(0.0)
    }

    /// How often the 🧭 button pulsed (rule 6); the host plays the pulse when it grows.
    pub fn pulses(&self) -> u32 {
        self.pulses
    }

    /// Seconds without a useful action.
    pub fn idle_s(&self) -> f32 {
        self.idle_s
    }

    /// A useful action happened (rule 6): the 90 s start again.
    pub fn useful(&mut self) {
        self.idle_s = 0.0;
    }

    /// Looks at a game event (useful actions restart the idle nudge).
    pub fn observe(&mut self, e: &GameEvent) {
        if is_useful(e) {
            self.useful();
        }
    }

    /// Advances the timers: the search timers of rule 4, the idle nudge of rule 6 (a new
    /// cell explored is useful) and the shown hint (12 s, reached, or no longer useful).
    pub fn update(&mut self, g: &Game, dt: f32) {
        for a in &g.animals {
            let searching = a.state == AnimalState::Escaped
                && g.in_scope(a)
                && g.mission(a.id()).is_some_and(|m| m.started && !m.complete);
            if searching {
                if a.member == 0 {
                    *self.searching.entry(a.id()).or_insert(0.0) += dt;
                }
            } else {
                self.searching.remove(a.id());
            }
        }
        if self.visited.insert(cell_of(g.player.pos)) {
            self.idle_s = 0.0;
        }
        let asleep = matches!(g.daytime.phase, Phase::Sleeping | Phase::Morning);
        if self.shown.is_none() && !asleep && g.panel.open.is_none() {
            self.idle_s += dt;
            if self.idle_s >= IDLE_NUDGE_S {
                self.idle_s = 0.0;
                self.pulses += 1;
            }
        }
        if self.shown.is_none() {
            return;
        }
        self.left_s -= dt;
        if self.left_s <= 0.0 || asleep {
            self.hide();
            return;
        }
        self.refresh_s -= dt;
        if self.refresh_s <= 0.0 {
            self.refresh_s = REFRESH_S;
            let id = self
                .shown
                .as_ref()
                .map(|h| h.id.clone())
                .unwrap_or_default();
            match candidates(g, self).into_iter().find(|h| h.id == id) {
                Some(h) => self.shown = Some(h),
                None => {
                    self.hide(); // done (board read, food taken, …)
                    return;
                }
            }
        }
        let reached = self
            .shown
            .as_ref()
            .is_some_and(|h| h.pos.distance(g.player.pos) <= REACHED_M);
        if reached {
            self.hide();
            return;
        }
        self.walk_s += dt;
        if self.walk_s >= WALK_REFRESH_S {
            self.walk_s = 0.0;
            if let Some(h) = &self.shown {
                self.walk_m = walking_distance(g, h.pos);
            }
        }
    }
}

/// Walking distance from the player to a point (grid path; the straight line if there is
/// no path).
pub fn walking_distance(g: &Game, to: Vec2) -> f32 {
    let grid = g.level.grid();
    let from = cell_of(crate::save::nearest_walkable(g, g.player.pos, true));
    let goal = cell_of(crate::save::nearest_walkable(g, to, true));
    let straight = g.player.pos.distance(to);
    match crate::nav::find_path(grid, from, goal, true) {
        Some(path) if path.len() > 1 => {
            let len: f32 = path
                .windows(2)
                .map(|w| cell_center(w[0]).distance(cell_center(w[1])))
                .sum();
            len.max(straight)
        }
        _ => straight,
    }
}

/// Where the hint indicator goes on the screen (rule 2): above the target when it is on
/// screen, else an edge arrow at the screen border pointing to it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScreenPlace {
    pub on_screen: bool,
    /// CSS px, y down.
    pub x: f32,
    pub y: f32,
    /// Direction of the edge arrow (degrees, screen space, 0 = right, 90 = down).
    pub angle_deg: f32,
}

/// Places the indicator from the clip-space position of the target (`vp * (p, 1)`) on a
/// `w`×`h` px screen; off-screen (or behind the camera) the arrow sits `margin` px inside
/// the border in the target's direction.
pub fn screen_place(clip: Vec4, w: f32, h: f32, margin: f32) -> ScreenPlace {
    let c = Vec2::new(w / 2.0, h / 2.0);
    if clip.w > 1e-4 {
        let ndc = Vec2::new(clip.x, clip.y) / clip.w;
        let s = Vec2::new((ndc.x * 0.5 + 0.5) * w, (0.5 - ndc.y * 0.5) * h);
        let inside = s.x >= margin && s.x <= w - margin && s.y >= margin && s.y <= h - margin;
        if inside {
            return ScreenPlace {
                on_screen: true,
                x: s.x,
                y: s.y,
                angle_deg: 90.0,
            };
        }
        return edge(c, s - c, w, h, margin);
    }
    // behind the camera: keep its side, point down (towards the viewer; screen y is down)
    let side = clip.x / clip.w.abs().max(1e-4);
    edge(c, Vec2::new(side, 1.0), w, h, margin)
}

fn edge(c: Vec2, d: Vec2, w: f32, h: f32, margin: f32) -> ScreenPlace {
    let d = d.normalize_or(Vec2::Y);
    let hx = (w / 2.0 - margin).max(1.0);
    let hy = (h / 2.0 - margin).max(1.0);
    let k = (hx / d.x.abs().max(1e-6)).min(hy / d.y.abs().max(1e-6));
    let q = c + d * k;
    ScreenPlace {
        on_screen: false,
        x: q.x,
        y: q.y,
        angle_deg: d.y.atan2(d.x).to_degrees(),
    }
}

/// State of the 🌙 night progress indicator (GAME-NIGHT "Night progress").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressState {
    /// Not shown (sleeping, morning, nothing left to do).
    Hidden,
    /// Day play: animals of the current day level are still missing.
    Missing,
    /// All animals of the day level are home: night is coming (celebration, dusk).
    NightComing,
    /// Night with an unfinished night level: its animals.
    Night,
    /// Nothing is missing: go to bed (at night, or by day while the night zoo waits, Q-140).
    Sleep,
}

impl ProgressState {
    pub fn id(self) -> &'static str {
        match self {
            ProgressState::Hidden => "hidden",
            ProgressState::Missing => "missing",
            ProgressState::NightComing => "night_coming",
            ProgressState::Night => "night",
            ProgressState::Sleep => "sleep",
        }
    }
}

/// What the 🌙 indicator shows: the animals (one per species) of a level and whether each
/// is home.
#[derive(Debug, Clone, PartialEq)]
pub struct NightProgress {
    pub state: ProgressState,
    /// The level the animals belong to (empty when hidden).
    pub level: String,
    pub animals: Vec<(&'static str, bool)>,
}

fn level_animals(g: &Game, k: usize) -> Vec<(&'static str, bool)> {
    let mut out: Vec<(&'static str, bool)> = Vec::new();
    for (a, m) in g.animals.iter().zip(&g.missions) {
        if a.part != k {
            continue;
        }
        match out.iter_mut().find(|(id, _)| *id == a.id()) {
            Some(e) => e.1 &= m.complete,
            None => out.push((a.id(), m.complete)),
        }
    }
    out
}

/// The night progress of the current level (GAME-NIGHT "Night progress", NIGHT-019/020).
pub fn night_progress(g: &Game) -> NightProgress {
    let data = &g.level.data;
    let hidden = NightProgress {
        state: ProgressState::Hidden,
        level: String::new(),
        animals: Vec::new(),
    };
    let with = |state, k: usize| NightProgress {
        state,
        level: data.parts[k].id.clone(),
        animals: level_animals(g, k),
    };
    let done = |k: usize| level_animals(g, k).iter().all(|(_, home)| *home);
    let has_animals = |k: usize| g.animals.iter().any(|a| a.part == k);
    match g.daytime.phase {
        Phase::Sleeping | Phase::Morning => return hidden,
        Phase::Dusk => {
            // the level whose nightfall this is
            let k = g
                .daytime
                .nightfalls
                .last()
                .and_then(|l| data.part_index(l))
                .unwrap_or(0);
            return with(ProgressState::NightComing, k);
        }
        Phase::Night => {
            let night = (0..data.parts.len()).find(|&k| {
                data.is_night_part(k) && g.part_unlocked(k) && has_animals(k) && !done(k)
            });
            if let Some(k) = night {
                return with(ProgressState::Night, k);
            }
            let k = (0..data.parts.len())
                .rev()
                .find(|&k| g.part_unlocked(k) && has_animals(k))
                .unwrap_or(0);
            return with(ProgressState::Sleep, k);
        }
        Phase::Day => {}
    }
    if g.daytime.dusk_in.is_some() {
        let k = g
            .daytime
            .nightfalls
            .last()
            .and_then(|l| data.part_index(l))
            .unwrap_or(0);
        return with(ProgressState::NightComing, k);
    }
    // the first unlocked day level with animals still missing
    let day = (0..data.parts.len())
        .find(|&k| !data.is_night_part(k) && g.part_unlocked(k) && has_animals(k) && !done(k));
    if let Some(k) = day {
        return with(ProgressState::Missing, k);
    }
    if g.night_zoo_waiting() {
        // the night zoo waits: sleep until the evening (Q-140)
        let k = (0..data.parts.len())
            .find(|&k| data.is_night_part(k) && has_animals(k) && !done(k))
            .unwrap_or(0);
        return with(ProgressState::Sleep, k);
    }
    hidden
}
