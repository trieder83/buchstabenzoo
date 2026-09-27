//! Nightfall and the night zoo in the game (GAME-NIGHT): moon door, bed, sleeping, the next
//! morning, eyeshine and the debug hooks. The time of day itself is [`crate::daytime`].

use glam::Vec2;

use crate::animals::AnimalState;
use crate::daytime::{Change, Phase};
use crate::game::{Animal, Game, GameEvent};
use crate::level::{cell_center, ElementType};
use crate::player::in_interaction_range;
use crate::wander::{self, Wander};

/// Radius of the player's lantern light (art plan: 2.5 m around the player; NIGHT-005/006).
pub const LANTERN_RADIUS_M: f32 = 2.5;
/// Height of the hand lantern's light centre above her feet (m).
pub const LANTERN_LIGHT_Y_M: f32 = 1.4;

/// Radius of the lantern's point-light sphere so that its hard-edged pool on the ground is
/// as wide as the eyeshine radius (Q-142 answered: the visible pool ≈ 2.5 m).
pub fn lantern_light_radius() -> f32 {
    (LANTERN_RADIUS_M * LANTERN_RADIUS_M + LANTERN_LIGHT_Y_M * LANTERN_LIGHT_Y_M).sqrt()
}
/// Barrier kind of the night-zoo gate (GAME-NIGHT rule 3).
pub const MOON_DOOR_KIND: &str = "moon_door";

impl Game {
    /// Ids of the moon doors: barriers of kind `moon_door`, or barriers whose transition
    /// leads into a night level.
    pub fn moon_doors(&self) -> Vec<String> {
        self.level
            .data
            .elements_of(ElementType::Barrier)
            .filter(|e| {
                e.kind.as_deref() == Some(MOON_DOOR_KIND)
                    || e.transition
                        .as_deref()
                        .and_then(|t| t.split("->").nth(1))
                        .and_then(|l| self.level.data.part_index(l))
                        .is_some_and(|k| self.level.data.is_night_part(k))
            })
            .map(|e| e.id.clone())
            .collect()
    }

    /// The beds (GAME-NIGHT rule 3, Q-096: the bed in either zookeeper house works):
    /// interaction points (level), from `[[item]] kind = "bed"`, else the bed of every
    /// unlocked enterable zookeeper house ([`crate::scene::bed_pose`]).
    pub fn beds(&self) -> Vec<Vec2> {
        let data = &self.level.data;
        let items: Vec<Vec2> = data
            .items
            .iter()
            .filter(|it| it.kind == "bed" && self.part_unlocked(it.part))
            .map(|it| it.pos())
            .collect();
        if !items.is_empty() {
            return items;
        }
        data.elements
            .iter()
            .filter(|e| self.part_unlocked(e.part))
            .filter_map(crate::scene::bed_pose)
            .map(|(c, _)| c)
            .collect()
    }

    /// Where the child stands to use the bed nearest to `from`: the bed item's `stand` cell
    /// (level data), else a walkable cell next to it (NIGHT-016).
    pub fn bed_stand(&self, from: Vec2) -> Option<Vec2> {
        let data = &self.level.data;
        let item = data
            .items
            .iter()
            .filter(|it| it.kind == "bed" && self.part_unlocked(it.part))
            .min_by(|a, b| a.pos().distance(from).total_cmp(&b.pos().distance(from)));
        if let Some(c) = item.and_then(|it| it.stand) {
            return Some(cell_center(glam::IVec2::new(c[0], c[1])));
        }
        let bed = self
            .beds()
            .into_iter()
            .min_by(|a, b| a.distance(from).total_cmp(&b.distance(from)))?;
        let grid = self.level.grid();
        crate::level::Rect::new(bed.x as i32 - 2, bed.y as i32 - 2, 5, 5)
            .cells()
            .filter(|&c| grid.is_passable(c, false))
            .map(cell_center)
            .filter(|q| q.distance(bed) <= 1.8)
            .min_by(|a, b| a.distance(bed).total_cmp(&b.distance(bed)))
    }

    /// The first bed (level 1's) and its readable side (none).
    pub fn bed(&self) -> Option<(Vec2, Option<Vec2>)> {
        self.beds().first().map(|p| (*p, None))
    }

    /// The bed nearest to the player.
    fn nearest_bed(&self) -> Option<Vec2> {
        let p = self.player.pos;
        self.beds()
            .into_iter()
            .min_by(|a, b| a.distance(p).total_cmp(&b.distance(p)))
    }

    /// Whether the animal lives in a night level (GAME-NIGHT rule 6).
    pub fn is_night_animal(&self, a: &Animal) -> bool {
        self.level.data.is_night_part(a.part)
    }

    /// Whether the player stands in a night level.
    pub fn player_in_night_zoo(&self) -> bool {
        self.level
            .data
            .part_at(crate::level::cell_of(self.player.pos))
            .is_some_and(|k| self.level.data.is_night_part(k))
    }

    /// NIGHT-006: an animal's eyes shine when the lantern light reaches it (night light on,
    /// within [`LANTERN_RADIUS_M`] of the player); never by day.
    pub fn eyes_shine(&self, a: &Animal) -> bool {
        self.daytime.is_dark()
            && self.in_scope(a)
            && a.state != AnimalState::InEnclosure
            && a.pos.distance(self.player.pos) <= LANTERN_RADIUS_M
    }

    /// A night level can be visited but is not complete (its moon door's level had its
    /// nightfall): the bed also works by day and the child sleeps until the evening
    /// (proposal Q-140 — no dead end when the child slept before the night zoo was done).
    pub fn night_zoo_waiting(&self) -> bool {
        let data = &self.level.data;
        self.moon_doors().iter().any(|id| {
            let Some(e) = data.element(id) else {
                return false;
            };
            let after = e.unlock_after.clone().or_else(|| {
                e.transition
                    .as_deref()
                    .and_then(|t| t.split("->").next())
                    .map(str::to_owned)
            });
            let open_nights = after.is_none_or(|l| self.daytime.nightfalls.contains(&l));
            let night_part = data
                .entries
                .iter()
                .find(|en| en.barrier == *id)
                .and_then(|en| {
                    data.parts
                        .iter()
                        .position(|p| p.entries.iter().any(|x| x.id == en.id))
                });
            let incomplete = night_part.is_some_and(|k| {
                self.animals
                    .iter()
                    .zip(&self.missions)
                    .any(|(a, m)| a.part == k && !m.complete)
            });
            open_nights && incomplete
        })
    }

    /// Whether the bed can be used now: at night (NIGHT-003), and by day while a night zoo
    /// is waiting (Q-140).
    pub fn bed_usable(&self) -> bool {
        self.daytime.is_night()
            || (self.daytime.phase == Phase::Day
                && self.daytime.dusk_in.is_none()
                && self.night_zoo_waiting())
    }

    /// Goes to bed (within reach; at night → the next morning, NIGHT-003; by day with a
    /// night zoo waiting → the evening, Q-140). Returns whether sleep started.
    pub fn sleep(&mut self) -> bool {
        let Some(bed) = self.nearest_bed() else {
            return false;
        };
        if !in_interaction_range(bed.distance(self.player.pos)) || !self.bed_usable() {
            return false;
        }
        let ok = if self.daytime.is_night() {
            self.daytime.sleep()
        } else {
            self.daytime.sleep_until_evening()
        };
        if ok {
            self.events.push(GameEvent::SleepStarted);
        }
        ok
    }

    /// Goes through an open moon door to its other side. Returns `Some(into_night_zoo)`.
    pub fn go_through_moon_door(&mut self, id: &str) -> Option<bool> {
        if !self.daytime.is_night() || !self.level.is_barrier_open(id) {
            return None;
        }
        let e = self.level.data.element(id)?;
        let r = e.rect;
        let door = Vec2::new(r.x as f32 + r.w as f32 / 2.0, r.z as f32 + r.d as f32 / 2.0);
        if !in_interaction_range(e.rect.distance_to(self.player.pos)) {
            return None;
        }
        // the night side: the entry cells of the night level behind this door
        let entry = self
            .level
            .data
            .entries
            .iter()
            .find(|en| en.barrier == id)
            .map(|en| {
                let c = en.cells;
                Vec2::new(c.x as f32 + c.w as f32 / 2.0, c.z as f32 + c.d as f32 / 2.0)
            })?;
        let into = !self.player_in_night_zoo();
        let away = (entry - door).normalize_or_zero();
        let depth = (entry - door).length().max(1.0);
        let to = if into {
            entry + away * 1.0
        } else {
            door - away * (depth + 1.0)
        };
        let to = crate::save::nearest_walkable(self, to, false);
        self.player.pos = to;
        self.player.y = self.level.ground_height(to);
        self.player.facing = if into { away } else { -away };
        // the following group comes along
        let behind = to - self.player.facing * self.follow_params.keep_distance_m;
        let behind = crate::save::nearest_walkable(self, behind, true);
        for a in &mut self.animals {
            if a.state == AnimalState::Following {
                a.pos = behind;
                a.path.clear();
                a.path_target = None;
            }
        }
        self.events.push(GameEvent::MoonDoor {
            into_night_zoo: into,
        });
        Some(into)
    }

    pub(crate) fn update_daytime(&mut self, dt: f32) {
        let mut changes = Vec::new();
        self.daytime.update(dt, &mut changes);
        for c in changes {
            match c {
                Change::DuskStarted => self.events.push(GameEvent::DuskStarted),
                Change::NightFell => self.night_falls(),
                Change::Woke => self.wake(true),
                Change::DayStarted => self.events.push(GameEvent::DayStarted),
            }
        }
    }

    /// Night: the moon doors open (NIGHT-002; every night once their `unlock_after` level —
    /// default: the level they belong to — had its nightfall, Q-079, Q-133). `force` opens
    /// them regardless (debug).
    fn night_falls_with(&mut self, force: bool) {
        for id in self.moon_doors() {
            let e = self.level.data.element(&id).expect("moon door element");
            let after = e.unlock_after.clone().or_else(|| {
                e.transition
                    .as_deref()
                    .and_then(|t| t.split("->").next())
                    .map(str::to_owned)
            });
            let unlocked = after.is_none_or(|l| self.daytime.nightfalls.contains(&l));
            if (force || unlocked) && self.level.open_barrier(&id) {
                self.events.push(GameEvent::BarrierOpened { id });
            }
        }
        self.events.push(GameEvent::NightFell);
    }

    fn night_falls(&mut self) {
        self.night_falls_with(false);
    }

    /// The next morning (NIGHT-003): the exits of the levels completed before the night open
    /// (Q-091), the moon doors close, the player wakes at the bed, night animals that were
    /// following go back to their hiding places (progress kept, Q-079); autosave.
    pub(crate) fn wake(&mut self, at_bed: bool) {
        for level in self.daytime.take_pending_exits() {
            self.open_exits(&level);
        }
        let in_night_zoo = self.player_in_night_zoo();
        self.close_moon_doors();
        if at_bed || in_night_zoo {
            match self.nearest_bed() {
                Some(bed) => {
                    self.player.pos = crate::save::nearest_walkable(self, bed, false);
                }
                None => {
                    self.player.pos = cell_center(self.level.data.parts[0].spawn.cell());
                }
            }
            self.player.y = self.level.ground_height(self.player.pos);
        }
        self.events.push(GameEvent::Morning);
    }

    fn close_moon_doors(&mut self) {
        for id in self.moon_doors() {
            self.level.close_barrier(&id);
        }
        for i in 0..self.animals.len() {
            if !self.is_night_animal(&self.animals[i]) {
                continue;
            }
            if self.animals[i].state == AnimalState::Following {
                let place = self.animals[i].hiding_place.clone();
                let spot = self
                    .level
                    .data
                    .hiding_place(&place)
                    .map_or(self.animals[i].pos, |h| h.spot());
                let pause = wander::draw_pause(&mut self.rng);
                let a = &mut self.animals[i];
                a.state = AnimalState::Escaped;
                a.pos = spot;
                a.waiting = false;
                a.path.clear();
                a.path_target = None;
                a.wander = Wander {
                    pause_s: pause,
                    route: Vec::new(),
                };
            }
        }
        self.refresh_areas();
    }

    /// Debug / e2e: jumps to a time of day (`day`, `dusk`, `night`, `sleeping`, `morning`).
    /// `night` opens the moon doors; `morning` / `day` apply the morning (pending exits
    /// open, moon doors close, the player wakes at the bed if she was in the night zoo).
    /// Returns false for an unknown id.
    pub fn debug_set_daytime(&mut self, id: &str) -> bool {
        let Some(phase) = Phase::from_id(id) else {
            return false;
        };
        let was = self.daytime.phase;
        match phase {
            Phase::Night | Phase::Sleeping => {
                self.daytime.force(Phase::Night);
                if was != Phase::Night {
                    self.night_falls_with(true);
                }
                if phase == Phase::Sleeping {
                    self.daytime.force(Phase::Sleeping);
                    self.events.push(GameEvent::SleepStarted);
                }
            }
            Phase::Dusk => self.daytime.force(Phase::Dusk),
            Phase::Morning | Phase::Day => {
                let dark = matches!(was, Phase::Night | Phase::Sleeping | Phase::Dusk);
                self.daytime.force(Phase::Morning);
                if dark || !self.daytime.pending_exits.is_empty() {
                    // she wakes at the bed after sleeping; else she stays (unless she was
                    // in the night zoo, which closes)
                    self.wake(was == Phase::Sleeping);
                }
                if phase == Phase::Day {
                    self.daytime.force(Phase::Day);
                    self.events.push(GameEvent::DayStarted);
                }
            }
        }
        true
    }

    /// Tests / e2e: runs the rest of the night quickly — night falls (if it has not), the
    /// child sleeps and wakes up the next morning (full day light). Returns the events.
    pub fn debug_next_morning(&mut self) -> Vec<GameEvent> {
        if self.daytime.phase != Phase::Night && self.daytime.phase != Phase::Sleeping {
            self.debug_set_daytime("night");
        }
        self.debug_set_daytime("sleeping");
        self.debug_set_daytime("day");
        self.drain_events()
    }
}

/// How a flying night animal (bat, owl: `fly_height` in `animal_anims.toml`) is posed
/// (ART-ANIMALS "Night animals", GAME-NIGHT, NIGHT-017): perched at its hiding place it plays
/// `perch` (the bat its data pose, `hang` below the grip point); following the child it
/// flies (`fly`, origin raised by `fly_height`); at home and elsewhere it stands (`idle`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FlyerPose {
    pub rest: &'static str,
    pub locomotion: &'static str,
    /// Height of the model origin above the ground (m).
    pub lift: f32,
}

/// Pose of a flyer: `perch_m` = the perch height while it sits at its hiding place, `pose`
/// = the hiding place's data pose (`hang` for the bat).
pub fn flyer_pose(
    state: AnimalState,
    perch_m: Option<f32>,
    pose: &str,
    fly_height: f32,
) -> FlyerPose {
    match (state, perch_m) {
        (AnimalState::Following, _) => FlyerPose {
            rest: "fly",
            locomotion: "fly",
            lift: fly_height,
        },
        (AnimalState::Escaped, Some(h)) => FlyerPose {
            rest: if pose == "hang" { "hang" } else { "perch" },
            locomotion: "fly",
            lift: h,
        },
        _ => FlyerPose {
            rest: "idle",
            locomotion: "walk",
            lift: 0.0,
        },
    }
}

/// Whether an animal's `eye_glow` slot shines (NIGHT-006): inside the lantern light, and
/// not while it sleeps (no eyelids — Q-146 answered 2026-09-27: `eye_glow` is skipped
/// while `sleep` plays).
pub fn eye_glow(shine: bool, rest_clip: &str, action: Option<&str>) -> bool {
    shine && rest_clip != "sleep" && action != Some("sleep")
}
