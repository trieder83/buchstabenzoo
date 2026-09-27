//! Vegetable garden and treats (GAME-GARDEN): plant spots that are harvested into a basket
//! and regrow in three visible steps, and the treats the animals like (proposal Q-100).
//!
//! Pure state: the layout comes from `[[garden]]`, `[[garden_bed]]`, `[[plant_spot]]`
//! (proposal Q-102); [`crate::Game`] decides when the player may harvest / give a treat.

use serde::{Deserialize, Serialize};

use crate::level::LevelData;
use crate::rng::Pcg32;

/// Treats the basket holds at most (GAME-GARDEN §4, proposal Q-101).
pub const BASKET_CAPACITY: u32 = 6;
/// Seconds of play time from an empty spot to a ripe plant (GAME-GARDEN §3, proposal Q-101);
/// every third of it is one visible growth step (empty → sprout → young → ripe).
pub const REGROW_S: f32 = 180.0;

/// A treat from the garden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Treat {
    Carrot,
    Potato,
}

impl Treat {
    pub const ALL: [Treat; 2] = [Treat::Carrot, Treat::Potato];

    pub fn id(self) -> &'static str {
        match self {
            Treat::Carrot => "carrot",
            Treat::Potato => "potato",
        }
    }

    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "carrot" => Some(Treat::Carrot),
            "potato" => Some(Treat::Potato),
            _ => None,
        }
    }

    /// Fluent key of the treat's word (HUD, feedback).
    pub fn label_key(self) -> &'static str {
        match self {
            Treat::Carrot => "garden-carrot",
            Treat::Potato => "garden-potato",
        }
    }
}

/// Which treats an animal likes (GAME-GARDEN §5, **proposal Q-100**: carrots — zebra,
/// elephant, giraffe, hippo, monkey, panda; potatoes — elephant, hippo, panda; the others
/// none).
pub fn likes(animal: &str, treat: Treat) -> bool {
    match treat {
        Treat::Carrot => matches!(
            animal,
            "zebra" | "elephant" | "giraffe" | "hippo" | "monkey" | "panda"
        ),
        Treat::Potato => matches!(animal, "elephant" | "hippo" | "panda"),
    }
}

/// Growth stage of a plant spot (GAME-GARDEN §3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Stage {
    Empty,
    Sprout,
    Young,
    Ripe,
}

impl Stage {
    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "empty" => Some(Stage::Empty),
            "sprout" => Some(Stage::Sprout),
            "young" => Some(Stage::Young),
            "ripe" => Some(Stage::Ripe),
            _ => None,
        }
    }

    pub fn id(self) -> &'static str {
        match self {
            Stage::Empty => "empty",
            Stage::Sprout => "sprout",
            Stage::Young => "young",
            Stage::Ripe => "ripe",
        }
    }

    /// Play time since the harvest at which this stage is reached.
    fn start_s(self) -> f32 {
        match self {
            Stage::Empty => 0.0,
            Stage::Sprout => REGROW_S / 3.0,
            Stage::Young => REGROW_S * 2.0 / 3.0,
            Stage::Ripe => REGROW_S,
        }
    }

    fn at(grown_s: f32) -> Self {
        [Stage::Ripe, Stage::Young, Stage::Sprout]
            .into_iter()
            .find(|s| grown_s >= s.start_s() - 1e-3)
            .unwrap_or(Stage::Empty)
    }
}

/// Runtime state of one plant spot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlantState {
    pub id: String,
    /// What it yields.
    pub treat: Treat,
    /// Seconds grown since it was harvested (≥ [`REGROW_S`] = ripe).
    pub grown_s: f32,
}

impl PlantState {
    pub fn stage(&self) -> Stage {
        Stage::at(self.grown_s)
    }
}

/// Treats in the basket (a slot of its own, separate from the hands: GARD-007).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Basket {
    pub carrots: u32,
    pub potatoes: u32,
}

impl Basket {
    pub fn total(&self) -> u32 {
        self.carrots + self.potatoes
    }

    pub fn count(&self, t: Treat) -> u32 {
        match t {
            Treat::Carrot => self.carrots,
            Treat::Potato => self.potatoes,
        }
    }

    fn slot(&mut self, t: Treat) -> &mut u32 {
        match t {
            Treat::Carrot => &mut self.carrots,
            Treat::Potato => &mut self.potatoes,
        }
    }

    /// Removes one treat; false if there is none.
    pub fn take(&mut self, t: Treat) -> bool {
        let s = self.slot(t);
        if *s == 0 {
            return false;
        }
        *s -= 1;
        true
    }
}

/// Why a harvest did not happen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HarvestError {
    UnknownSpot,
    NotRipe,
    /// The basket already holds [`BASKET_CAPACITY`] treats (GARD-003).
    BasketFull,
}

/// Gardens of the zoo: plant states and the basket (saved, GARD-008).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Garden {
    pub plants: Vec<PlantState>,
    pub basket: Basket,
}

impl Garden {
    /// A new game: every plant spot at its `start_stage`, an empty basket.
    pub fn new(data: &LevelData) -> Self {
        let plants = data
            .plant_spots
            .iter()
            .filter_map(|s| {
                let treat = Treat::from_id(&s.kind)?;
                let stage = Stage::from_id(&s.start_stage).unwrap_or(Stage::Ripe);
                Some(PlantState {
                    id: s.id.clone(),
                    treat,
                    grown_s: stage.start_s(),
                })
            })
            .collect();
        Self {
            plants,
            basket: Basket::default(),
        }
    }

    pub fn plant(&self, id: &str) -> Option<&PlantState> {
        self.plants.iter().find(|p| p.id == id)
    }

    /// Advances the regrowth by `dt` seconds of play time (GARD-004).
    pub fn update(&mut self, dt: f32) {
        for p in &mut self.plants {
            if p.grown_s < REGROW_S {
                p.grown_s = (p.grown_s + dt).min(REGROW_S);
            }
        }
    }

    /// Harvests a ripe plant into the basket: 1 carrot, or 2–3 potatoes (seeded, GARD-002);
    /// what does not fit stays in the soil (the basket is filled up to its capacity). The
    /// spot becomes empty soil and regrows. Returns the treat and how many went in.
    pub fn harvest(&mut self, id: &str, rng: &mut Pcg32) -> Result<(Treat, u32), HarvestError> {
        let space = BASKET_CAPACITY.saturating_sub(self.basket.total());
        let p = self
            .plants
            .iter_mut()
            .find(|p| p.id == id)
            .ok_or(HarvestError::UnknownSpot)?;
        if p.stage() != Stage::Ripe {
            return Err(HarvestError::NotRipe);
        }
        if space == 0 {
            return Err(HarvestError::BasketFull);
        }
        let n = match p.treat {
            Treat::Carrot => 1,
            Treat::Potato => 2 + rng.next_u32() % 2,
        }
        .min(space);
        p.grown_s = 0.0;
        let treat = p.treat;
        *self.basket.slot(treat) += n;
        Ok((treat, n))
    }

    /// Restores saved plant states (by id; unknown ids are ignored, new spots keep their
    /// start stage) and the basket (clamped to the capacity).
    pub fn restore(&mut self, saved: &Garden) {
        for s in &saved.plants {
            if let Some(p) = self.plants.iter_mut().find(|p| p.id == s.id) {
                if s.grown_s.is_finite() {
                    p.grown_s = s.grown_s.clamp(0.0, REGROW_S);
                }
            }
        }
        let b = saved.basket;
        self.basket.carrots = b.carrots.min(BASKET_CAPACITY);
        self.basket.potatoes = b.potatoes.min(BASKET_CAPACITY - self.basket.carrots);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn level1() -> LevelData {
        let s = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/levels/level-1.toml"
        ))
        .unwrap();
        LevelData::from_toml_str(&s).unwrap()
    }

    #[test]
    fn gard_001_harvest_a_ripe_carrot() {
        let mut g = Garden::new(&level1());
        let mut rng = Pcg32::new(1);
        assert_eq!(g.plant("carrot_w1").unwrap().stage(), Stage::Ripe);
        assert_eq!(g.harvest("carrot_w1", &mut rng), Ok((Treat::Carrot, 1)));
        assert_eq!(g.basket.carrots, 1);
        assert_eq!(g.plant("carrot_w1").unwrap().stage(), Stage::Empty);
        assert_eq!(
            g.harvest("carrot_w1", &mut rng),
            Err(HarvestError::NotRipe),
            "empty soil cannot be harvested"
        );
    }

    #[test]
    fn gard_002_potatoes_two_or_three_seeded() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..20 {
            let mut g = Garden::new(&level1());
            let mut rng = Pcg32::new(seed);
            let (t, n) = g.harvest("potato_w1", &mut rng).unwrap();
            assert_eq!(t, Treat::Potato);
            assert!((2..=3).contains(&n), "{n}");
            assert_eq!(g.basket.potatoes, n);
            // same seed, same result
            let mut g2 = Garden::new(&level1());
            assert_eq!(g2.harvest("potato_w1", &mut Pcg32::new(seed)).unwrap().1, n);
            seen.insert(n);
        }
        assert_eq!(seen.len(), 2, "both 2 and 3 occur");
    }

    #[test]
    fn gard_003_full_basket_refuses_and_the_plant_stays() {
        let mut g = Garden::new(&level1());
        let mut rng = Pcg32::new(3);
        for id in [
            "carrot_w1",
            "carrot_w2",
            "carrot_w3",
            "carrot_e1",
            "carrot_e2",
            "carrot_e3",
        ] {
            g.harvest(id, &mut rng).unwrap();
        }
        assert_eq!(g.basket.total(), BASKET_CAPACITY);
        assert_eq!(
            g.harvest("potato_w1", &mut rng),
            Err(HarvestError::BasketFull)
        );
        assert_eq!(g.plant("potato_w1").unwrap().stage(), Stage::Ripe);
        assert_eq!(g.basket.total(), BASKET_CAPACITY);
    }

    #[test]
    fn gard_004_regrows_in_three_visible_steps_within_3_minutes() {
        let mut g = Garden::new(&level1());
        g.harvest("carrot_e2", &mut Pcg32::new(1)).unwrap();
        let mut stages = vec![g.plant("carrot_e2").unwrap().stage()];
        for _ in 0..(180 * 10) {
            g.update(0.1);
            let s = g.plant("carrot_e2").unwrap().stage();
            if stages.last() != Some(&s) {
                stages.push(s);
            }
        }
        assert_eq!(
            stages,
            [Stage::Empty, Stage::Sprout, Stage::Young, Stage::Ripe]
        );
        // ripe exactly after 3 minutes, not before
        let mut g = Garden::new(&level1());
        g.harvest("carrot_e2", &mut Pcg32::new(1)).unwrap();
        g.update(179.0);
        assert_eq!(g.plant("carrot_e2").unwrap().stage(), Stage::Young);
        g.update(1.0);
        assert_eq!(g.plant("carrot_e2").unwrap().stage(), Stage::Ripe);
    }

    #[test]
    fn gard_008_state_round_trips() {
        let mut g = Garden::new(&level1());
        let mut rng = Pcg32::new(5);
        g.harvest("carrot_w1", &mut rng).unwrap();
        g.harvest("potato_e2", &mut rng).unwrap();
        g.update(70.0);
        let json = serde_json::to_string(&g).unwrap();
        let back: Garden = serde_json::from_str(&json).unwrap();
        let mut fresh = Garden::new(&level1());
        fresh.restore(&back);
        assert_eq!(fresh, g);
        assert_eq!(fresh.plant("carrot_w1").unwrap().stage(), Stage::Sprout);
    }

    #[test]
    fn treats_per_animal_follow_the_proposal() {
        assert!(likes("zebra", Treat::Carrot));
        assert!(!likes("zebra", Treat::Potato));
        assert!(likes("hippo", Treat::Potato));
        assert!(!likes("koala", Treat::Carrot));
        assert!(!likes("lion", Treat::Potato));
    }
}
