//! Game state and the rescue mission flow (GAME-RESCUE, GAME-ANIMALS, GAME-FEED).
//!
//! Deterministic: all randomness comes from the seed, time only from `dt`.

use glam::{IVec2, Vec2};

use crate::animals::{animal_info, AnimalInfo, AnimalState};
use crate::content::{riddle_key, Language, ReadingLevel};
use crate::food::{Carry, Food, FoodBox, FoodLabel, FoodStorage};
use crate::level::{cell_center, cell_of, CellKind, ElementType, Level, LevelData, Surface};
use crate::nav;
use crate::player::{in_interaction_range, MoveParams, Player};
use crate::rng::Pcg32;

/// Follow behaviour (GAME-RESCUE §6).
#[derive(Debug, Clone, Copy)]
pub struct FollowParams {
    /// Distance kept behind the player ("a short distance", value not in the spec).
    pub keep_distance_m: f32,
    /// Following animals wait when the player is farther away than this.
    pub wait_beyond_m: f32,
    /// Waiting animals follow again when the player is back within this distance.
    pub resume_within_m: f32,
}

impl Default for FollowParams {
    fn default() -> Self {
        Self {
            keep_distance_m: 1.5,
            wait_beyond_m: 15.0,
            resume_within_m: 5.0,
        }
    }
}

/// Events for presentation (animation, audio, UI). Drained with [`Game::drain_events`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameEvent {
    MissionStarted {
        animal: String,
    },
    /// Player took food; `returned` is the food put back into its box (FEED-003).
    FoodTaken {
        food: Food,
        returned: Option<Food>,
    },
    NotInterested {
        animal: String,
    },
    StartedFollowing {
        animal: String,
    },
    Waiting {
        animal: String,
    },
    FollowingAgain {
        animal: String,
    },
    /// Animals stop at a wrong enclosure's gate (RESC-007).
    Refuse {
        animal: String,
        enclosure: String,
    },
    FoodConsumed {
        food: Food,
    },
    /// Animals entered their own enclosure: play `eat`, then `happy`; gate closes.
    InEnclosure {
        animal: String,
    },
    MissionComplete {
        animal: String,
    },
    AllAnimalsHome,
    BarrierOpened {
        id: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InteractError {
    UnknownTarget,
    OutOfRange,
}

/// One animal group (herd size Q-030: modelled as one unit).
#[derive(Debug, Clone)]
pub struct Animal {
    pub info: &'static AnimalInfo,
    pub state: AnimalState,
    pub pos: Vec2,
    /// Id of the hiding place chosen for this playthrough.
    pub hiding_place: String,
    /// Index of its enclosure element.
    pub enclosure: usize,
    /// `following` but waiting for the player (> 15 m, RESC-006).
    pub waiting: bool,
    /// Stopped at a wrong enclosure's gate (RESC-007).
    pub refusing: bool,
    path: Vec<IVec2>,
    path_target: Option<IVec2>,
}

impl Animal {
    pub fn id(&self) -> &'static str {
        self.info.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Mission {
    pub started: bool,
    pub complete: bool,
}

/// Player settings relevant for content.
#[derive(Debug, Clone, Copy)]
pub struct Settings {
    pub reading_level: ReadingLevel,
    pub language: Language,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            reading_level: ReadingLevel::Klasse1,
            language: Language::De,
        }
    }
}

/// Content of an info board (GAME-ANIMALS "Info board") as Fluent keys.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InfoBoard {
    pub animal: &'static str,
    pub riddle_key: String,
    /// `kiga`: a picture of the hiding place (its id) next to the one-word riddle.
    pub picture: Option<String>,
    /// The food word — the same key as the matching food box label (ANIM-003).
    pub food_key: String,
}

#[derive(Debug)]
pub enum GameError {
    UnknownAnimal(String),
    NoHidingPlace(String),
    NoAnimalSpot(String),
}

impl std::fmt::Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameError::UnknownAnimal(a) => write!(f, "unknown animal {a}"),
            GameError::NoHidingPlace(a) => write!(f, "no hiding place for {a} in the level"),
            GameError::NoAnimalSpot(h) => write!(f, "hiding place {h} has no animal_spot"),
        }
    }
}

impl std::error::Error for GameError {}

pub struct Game {
    pub level: Level,
    pub player: Player,
    pub carry: Carry,
    pub storage: FoodStorage,
    pub animals: Vec<Animal>,
    pub missions: Vec<Mission>,
    pub settings: Settings,
    pub move_params: MoveParams,
    pub follow_params: FollowParams,
    events: Vec<GameEvent>,
    /// Gate cell the player stood on in the last update (for once-per-entry events).
    last_gate: Option<usize>,
    all_home: bool,
}

impl Game {
    /// New game: every enclosure empty, every animal at a hiding place picked with the seeded
    /// RNG (GAME-RESCUE §1). One animal per enclosure in the level data.
    pub fn new(data: LevelData, seed: u64) -> Result<Self, GameError> {
        let mut rng = Pcg32::new(seed);
        let mut animals = Vec::new();
        for (i, enc) in data.elements.iter().enumerate() {
            if enc.ty != ElementType::Enclosure {
                continue;
            }
            let id = enc.animal.clone().unwrap_or_default();
            let info = animal_info(&id).ok_or_else(|| GameError::UnknownAnimal(id.clone()))?;
            let places: Vec<_> = data
                .elements_of(ElementType::HidingPlace)
                .filter(|h| h.animal.as_deref() == Some(info.id))
                .collect();
            if places.is_empty() {
                return Err(GameError::NoHidingPlace(id));
            }
            let place = places[rng.below(places.len() as u32) as usize];
            let spot = place
                .animal_spot_cell()
                .ok_or_else(|| GameError::NoAnimalSpot(place.id.clone()))?;
            animals.push(Animal {
                info,
                state: AnimalState::Escaped,
                pos: cell_center(spot),
                hiding_place: place.id.clone(),
                enclosure: i,
                waiting: false,
                refusing: false,
                path: Vec::new(),
                path_target: None,
            });
        }
        let move_params = MoveParams::default();
        let facing = match data.spawn.facing.as_str() {
            "-z" => Vec2::NEG_Y,
            "+x" => Vec2::X,
            "-x" => Vec2::NEG_X,
            _ => Vec2::Y,
        };
        let player = Player::new(cell_center(data.spawn.cell()), facing, &move_params);
        let missions = vec![Mission::default(); animals.len()];
        Ok(Self {
            level: Level::new(data),
            player,
            carry: Carry::default(),
            storage: FoodStorage::default(),
            animals,
            missions,
            settings: Settings::default(),
            move_params,
            follow_params: FollowParams::default(),
            events: Vec::new(),
            last_gate: None,
            all_home: false,
        })
    }

    pub fn drain_events(&mut self) -> Vec<GameEvent> {
        std::mem::take(&mut self.events)
    }

    pub fn animal_index(&self, id: &str) -> Option<usize> {
        self.animals.iter().position(|a| a.id() == id)
    }

    pub fn animal(&self, id: &str) -> Option<&Animal> {
        self.animals.iter().find(|a| a.id() == id)
    }

    pub fn mission(&self, animal: &str) -> Option<Mission> {
        self.animal_index(animal).map(|i| self.missions[i])
    }

    /// Whether the player currently leads an animal group (gates passable).
    pub fn is_leading(&self) -> bool {
        self.animals
            .iter()
            .any(|a| a.state == AnimalState::Following)
    }

    /// Animals inside the given enclosure element.
    pub fn enclosure_occupants(&self, enclosure_id: &str) -> Vec<&'static str> {
        self.animals
            .iter()
            .filter(|a| {
                a.state == AnimalState::InEnclosure
                    && self.level.data.elements[a.enclosure].id == enclosure_id
            })
            .map(|a| a.id())
            .collect()
    }

    /// Info board of an animal's enclosure at the current reading level (ANIM-005).
    pub fn info_board(&self, animal: &str) -> Option<InfoBoard> {
        let a = self.animal(animal)?;
        let level = self.settings.reading_level;
        Some(InfoBoard {
            animal: a.id(),
            riddle_key: riddle_key(a.id(), &a.hiding_place, level),
            picture: (level == ReadingLevel::Kiga).then(|| a.hiding_place.clone()),
            food_key: a.info.foods[0].label_key(),
        })
    }

    /// Distance from the player to the info board of an animal's enclosure.
    fn board_distance(&self, animal: &str) -> Option<f32> {
        let a = self.animal(animal)?;
        let enc_id = &self.level.data.elements[a.enclosure].id;
        self.level
            .data
            .elements
            .iter()
            .filter(|e| {
                e.kind.as_deref() == Some("info_board") && e.enclosure.as_deref() == Some(enc_id)
            })
            .map(|e| e.rect.distance_to(self.player.pos))
            .reduce(f32::min)
    }

    /// The player reads the info board of an animal's enclosure (must be within 2 m). Starts
    /// the mission on first read (RESC-012).
    pub fn read_info_board(&mut self, animal: &str) -> Result<InfoBoard, InteractError> {
        let d = self
            .board_distance(animal)
            .ok_or(InteractError::UnknownTarget)?;
        if !in_interaction_range(d) {
            return Err(InteractError::OutOfRange);
        }
        let i = self
            .animal_index(animal)
            .ok_or(InteractError::UnknownTarget)?;
        if !self.missions[i].started {
            self.missions[i].started = true;
            self.events.push(GameEvent::MissionStarted {
                animal: animal.to_owned(),
            });
        }
        self.info_board(animal).ok_or(InteractError::UnknownTarget)
    }

    /// Distance from the player to the food storage door (boxes are taken at the door; the
    /// storage interior is not in the layout data yet).
    pub fn storage_distance(&self) -> Option<f32> {
        self.level
            .data
            .elements
            .iter()
            .filter(|e| e.kind.as_deref() == Some("food_storage"))
            .filter_map(|e| e.door_cell())
            .map(|d| crate::level::Rect::new(d.x, d.y, 1, 1).distance_to(self.player.pos))
            .reduce(f32::min)
    }

    /// Food box labels at the current reading level.
    pub fn food_labels(&self) -> Vec<(Food, FoodLabel)> {
        self.storage
            .boxes
            .iter()
            .map(|b| (b.food, b.label(self.settings.reading_level)))
            .collect()
    }

    /// Takes food from a box in the storage (GAME-FEED §3).
    pub fn take_food(&mut self, food: Food) -> Result<(), InteractError> {
        let d = self
            .storage_distance()
            .ok_or(InteractError::UnknownTarget)?;
        if !in_interaction_range(d) {
            return Err(InteractError::OutOfRange);
        }
        let b: FoodBox = *self
            .storage
            .boxes
            .iter()
            .find(|b| b.food == food)
            .ok_or(InteractError::UnknownTarget)?;
        let returned = self.carry.take(&b);
        self.events.push(GameEvent::FoodTaken { food, returned });
        Ok(())
    }

    /// Shows the carried food to an animal group within 2 m (GAME-RESCUE §5, GAME-FEED §4).
    pub fn show_food(&mut self, animal: &str) -> Result<(), InteractError> {
        let i = self
            .animal_index(animal)
            .ok_or(InteractError::UnknownTarget)?;
        if !in_interaction_range(self.animals[i].pos.distance(self.player.pos)) {
            return Err(InteractError::OutOfRange);
        }
        if self.animals[i].state != AnimalState::Escaped {
            return Ok(()); // following or home: nothing changes (ANIM-002)
        }
        // Only one group follows at a time (§5); a second group is not interested (Q-041 proposal).
        let other_following = self.is_leading();
        let right_food = self
            .carry
            .food()
            .is_some_and(|f| self.animals[i].info.eats(f));
        let id = animal.to_owned();
        if right_food && !other_following {
            let a = &mut self.animals[i];
            a.state = AnimalState::Following;
            a.waiting = false;
            a.path.clear();
            a.path_target = None;
            self.events.push(GameEvent::StartedFollowing { animal: id });
        } else {
            self.events.push(GameEvent::NotInterested { animal: id });
        }
        Ok(())
    }

    /// Advances the simulation by `dt` seconds with the joystick `input` (world `(x, z)`).
    pub fn update(&mut self, dt: f32, input: Vec2) {
        let leading = self.is_leading();
        self.player
            .step(self.level.grid(), &self.move_params, input, dt, leading);
        self.check_gate();
        self.update_followers(dt);
    }

    fn check_gate(&mut self) {
        let gate = match self.level.grid().kind(cell_of(self.player.pos)) {
            CellKind::Gate(enc) => Some(enc),
            _ => None,
        };
        let entered = gate.is_some() && gate != self.last_gate;
        self.last_gate = gate;
        for a in &mut self.animals {
            a.refusing = false;
        }
        let Some(enc) = gate else { return };
        let enc_id = self.level.data.elements[enc].id.clone();
        let leader = self
            .animals
            .iter()
            .position(|a| a.state == AnimalState::Following && !a.waiting);
        let Some(i) = leader else { return };
        if self.animals[i].enclosure == enc {
            self.enter_enclosure(i);
        } else {
            self.animals[i].refusing = true;
            if entered {
                self.events.push(GameEvent::Refuse {
                    animal: self.animals[i].id().to_owned(),
                    enclosure: enc_id,
                });
            }
        }
    }

    /// GAME-RESCUE §8 / §9.
    fn enter_enclosure(&mut self, i: usize) {
        let enc = &self.level.data.elements[self.animals[i].enclosure];
        let a = &mut self.animals[i];
        a.state = AnimalState::InEnclosure;
        a.waiting = false;
        a.pos = cell_center(IVec2::new(
            enc.rect.x + enc.rect.w / 2,
            enc.rect.z + enc.rect.d / 2,
        ));
        let id = a.id().to_owned();
        if let Some(food) = self.carry.consume() {
            self.events.push(GameEvent::FoodConsumed { food });
        }
        self.events
            .push(GameEvent::InEnclosure { animal: id.clone() });
        self.missions[i].complete = true;
        self.events.push(GameEvent::MissionComplete { animal: id });
        if !self.all_home && self.missions.iter().all(|m| m.complete) {
            self.all_home = true;
            self.events.push(GameEvent::AllAnimalsHome);
            // Proposal Q-022: exit barriers open when all animals of the level are home.
            for b in self.level.exit_barriers() {
                if self.level.open_barrier(&b) {
                    self.events.push(GameEvent::BarrierOpened { id: b });
                }
            }
        }
    }

    fn update_followers(&mut self, dt: f32) {
        let p = self.player.pos;
        let player_cell = cell_of(p);
        let fp = self.follow_params;
        for a in &mut self.animals {
            if a.state != AnimalState::Following {
                continue;
            }
            let dist = a.pos.distance(p);
            if !a.waiting && dist > fp.wait_beyond_m {
                a.waiting = true;
                self.events.push(GameEvent::Waiting {
                    animal: a.id().to_owned(),
                });
            } else if a.waiting && dist <= fp.resume_within_m {
                a.waiting = false;
                a.path_target = None;
                self.events.push(GameEvent::FollowingAgain {
                    animal: a.id().to_owned(),
                });
            }
            if a.waiting || a.refusing || dist <= fp.keep_distance_m {
                continue;
            }
            let grid = self.level.grid();
            let speed = match grid.surface(cell_of(a.pos)) {
                Some(Surface::Path) => self.move_params.speed_on(Surface::Path),
                _ => self.move_params.speed_on(Surface::Grass),
            };
            let here = cell_of(a.pos);
            if a.path_target != Some(player_cell) || !a.path.contains(&here) {
                a.path = nav::find_path(grid, here, player_cell, true).unwrap_or_default();
                a.path_target = Some(player_cell);
            }
            // Next waypoint: the cell after the current one, or the player when adjacent / no path.
            let waypoint = a
                .path
                .iter()
                .position(|&c| c == here)
                .and_then(|k| a.path.get(k + 1))
                .filter(|&&c| c != player_cell)
                .map(|&c| cell_center(c))
                .unwrap_or(p);
            let to = waypoint - a.pos;
            let step = (speed * dt).min(to.length()).min(dist - fp.keep_distance_m);
            if step > 0.0 {
                a.pos += to.normalize_or_zero() * step;
            }
        }
    }
}
