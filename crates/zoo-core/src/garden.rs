//! Vegetable garden, fruit garden and treats (GAME-GARDEN): plant spots that are harvested
//! into a basket and regrow in three visible steps, and the treats the animals like (proposal
//! Q-100, fruit Q-321).
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

/// A treat from the garden: vegetables (level 1) and fruit (level 3, Q-320).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Treat {
    Carrot,
    Potato,
    Apple,
    Orange,
}

impl Treat {
    pub const ALL: [Treat; 4] = [Treat::Carrot, Treat::Potato, Treat::Apple, Treat::Orange];

    pub fn id(self) -> &'static str {
        match self {
            Treat::Carrot => "carrot",
            Treat::Potato => "potato",
            Treat::Apple => "apple",
            Treat::Orange => "orange",
        }
    }

    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "carrot" => Some(Treat::Carrot),
            "potato" => Some(Treat::Potato),
            "apple" => Some(Treat::Apple),
            "orange" => Some(Treat::Orange),
            _ => None,
        }
    }

    /// Fluent key of the treat's word (HUD, feedback).
    pub fn label_key(self) -> &'static str {
        match self {
            Treat::Carrot => "garden-carrot",
            Treat::Potato => "garden-potato",
            Treat::Apple => "garden-apple",
            Treat::Orange => "garden-orange",
        }
    }

    /// Fruit comes from trees (`<id>_tree_<stage>` models), vegetables from beds
    /// (`<id>_plant_<stage>`).
    pub fn is_fruit(self) -> bool {
        matches!(self, Treat::Apple | Treat::Orange)
    }

    /// Treats a harvest of this plant gives (carrot 1, potato 2–3 seeded, fruit 1).
    fn yield_n(self, rng: &mut Pcg32) -> u32 {
        match self {
            Treat::Carrot | Treat::Apple | Treat::Orange => 1,
            Treat::Potato => 2 + rng.next_u32() % 2,
        }
    }
}

/// Which treats an animal likes (GAME-GARDEN §5, **proposal Q-100 / Q-321**): carrots —
/// zebra, elephant, giraffe, hippo, monkey, panda; potatoes — elephant, hippo, panda; apples
/// — monkey, elephant, giraffe, zebra, panda; oranges — monkey, elephant, giraffe; the others
/// none.
pub fn likes(animal: &str, treat: Treat) -> bool {
    match treat {
        Treat::Carrot => matches!(
            animal,
            "zebra" | "elephant" | "giraffe" | "hippo" | "monkey" | "panda"
        ),
        Treat::Potato => matches!(animal, "elephant" | "hippo" | "panda"),
        Treat::Apple => matches!(
            animal,
            "monkey" | "elephant" | "giraffe" | "zebra" | "panda"
        ),
        Treat::Orange => matches!(animal, "monkey" | "elephant" | "giraffe"),
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

/// Treats in the basket (a slot of its own, separate from the hands: GARD-007). Saves from
/// before the fruit garden hold only `carrots` and `potatoes` (GARD-017).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Basket {
    pub carrots: u32,
    pub potatoes: u32,
    #[serde(default)]
    pub apples: u32,
    #[serde(default)]
    pub oranges: u32,
}

impl Basket {
    pub fn total(&self) -> u32 {
        self.carrots + self.potatoes + self.apples + self.oranges
    }

    pub fn count(&self, t: Treat) -> u32 {
        match t {
            Treat::Carrot => self.carrots,
            Treat::Potato => self.potatoes,
            Treat::Apple => self.apples,
            Treat::Orange => self.oranges,
        }
    }

    fn slot(&mut self, t: Treat) -> &mut u32 {
        match t {
            Treat::Carrot => &mut self.carrots,
            Treat::Potato => &mut self.potatoes,
            Treat::Apple => &mut self.apples,
            Treat::Orange => &mut self.oranges,
        }
    }

    /// Puts up to `n` treats in, as many as fit under [`BASKET_CAPACITY`]; returns how many
    /// went in.
    pub fn add(&mut self, t: Treat, n: u32) -> u32 {
        let n = n.min(BASKET_CAPACITY.saturating_sub(self.total()));
        *self.slot(t) += n;
        n
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

    /// Harvests a ripe plant into the basket: 1 carrot / apple / orange, or 2–3 potatoes
    /// (seeded, GARD-002, GARD-016);
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
        let treat = p.treat;
        let n = treat.yield_n(rng).min(space);
        p.grown_s = 0.0;
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
        self.basket = Basket::default();
        for t in Treat::ALL {
            self.basket.add(t, saved.basket.count(t));
        }
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

    fn level3() -> LevelData {
        let s = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../assets/levels/level-3.toml"
        ))
        .unwrap();
        LevelData::from_toml_str(&s).unwrap()
    }

    // GARD-015
    #[test]
    fn gard_015_four_treats_ids_and_likes_table() {
        assert_eq!(Treat::ALL.len(), 4);
        for t in Treat::ALL {
            assert_eq!(Treat::from_id(t.id()), Some(t));
            assert_eq!(t.label_key(), format!("garden-{}", t.id()));
        }
        assert!(Treat::Apple.is_fruit() && Treat::Orange.is_fruit());
        assert!(!Treat::Carrot.is_fruit() && !Treat::Potato.is_fruit());
        // the monkey: apples and oranges (and carrots), not potatoes
        assert!(likes("monkey", Treat::Apple));
        assert!(likes("monkey", Treat::Orange));
        assert!(likes("monkey", Treat::Carrot));
        assert!(!likes("monkey", Treat::Potato));
        // the full table of GAME-GARDEN 5
        let table: [(&str, [bool; 4]); 11] = [
            ("zebra", [true, false, true, false]),
            ("elephant", [true, true, true, true]),
            ("giraffe", [true, false, true, true]),
            ("hippo", [true, true, false, false]),
            ("monkey", [true, false, true, true]),
            ("panda", [true, true, true, false]),
            ("koala", [false; 4]),
            ("lion", [false; 4]),
            ("snow_fox", [false; 4]),
            ("goldfish", [false; 4]),
            ("owl", [false; 4]),
        ];
        for (animal, want) in table {
            for (t, w) in Treat::ALL.into_iter().zip(want) {
                assert_eq!(likes(animal, t), w, "{animal} {t:?}");
            }
        }
    }

    // GARD-016
    #[test]
    fn gard_016_fruit_yields_one_and_shares_the_capacity() {
        let mut g = Garden::new(&level3());
        let mut rng = Pcg32::new(1);
        for (spot, treat) in [("apple_1", Treat::Apple), ("orange_1", Treat::Orange)] {
            assert_eq!(g.plant(spot).unwrap().stage(), Stage::Ripe, "{spot}");
            assert_eq!(g.harvest(spot, &mut rng), Ok((treat, 1)));
            assert_eq!(g.plant(spot).unwrap().stage(), Stage::Empty);
            assert_eq!(g.basket.count(treat), 1);
        }
        // capacity 6 in total over all kinds: 2 fruit + 3 carrots = 5, one more fits, then full
        g.basket.carrots = 3;
        assert_eq!(g.harvest("apple_2", &mut rng), Ok((Treat::Apple, 1)));
        assert_eq!(g.basket.total(), BASKET_CAPACITY);
        assert_eq!(
            g.harvest("orange_2", &mut rng),
            Err(HarvestError::BasketFull)
        );
        assert_eq!(g.plant("orange_2").unwrap().stage(), Stage::Ripe);
        // regrows in 3 minutes like the vegetables
        g.update(REGROW_S);
        assert_eq!(g.plant("apple_1").unwrap().stage(), Stage::Ripe);
    }

    // GARD-017
    #[test]
    fn gard_017_basket_saves_per_kind_and_old_saves_load() {
        let mut g = Garden::new(&level3());
        g.basket = Basket {
            carrots: 1,
            potatoes: 1,
            apples: 2,
            oranges: 1,
        };
        let json = serde_json::to_string(&g).unwrap();
        let back: Garden = serde_json::from_str(&json).unwrap();
        let mut fresh = Garden::new(&level3());
        fresh.restore(&back);
        assert_eq!(fresh.basket, g.basket);
        // an old save: only carrots and potatoes in the basket
        let old = r#"{"plants":[],"basket":{"carrots":2,"potatoes":3}}"#;
        let back: Garden = serde_json::from_str(old).unwrap();
        assert_eq!((back.basket.apples, back.basket.oranges), (0, 0));
        let mut fresh = Garden::new(&level3());
        fresh.restore(&back);
        assert_eq!((fresh.basket.carrots, fresh.basket.potatoes), (2, 3));
        // too many treats are clamped to the capacity
        let big = r#"{"plants":[],"basket":{"carrots":4,"potatoes":4,"apples":4,"oranges":4}}"#;
        let back: Garden = serde_json::from_str(big).unwrap();
        let mut fresh = Garden::new(&level3());
        fresh.restore(&back);
        assert_eq!(fresh.basket.total(), BASKET_CAPACITY);
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
