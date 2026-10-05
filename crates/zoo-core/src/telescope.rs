//! The telescope (GAME-TELESCOPE, TELE-001…006): the eight planets of the solar system as a
//! fixed data table (kind, order, look, Fluent keys). Pure data - the host draws the discs
//! and shows the info box; nothing here changes game state.

use crate::content::ReadingLevel;

/// What kind of planet it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanetKind {
    /// Rocky planet (Mercury, Venus, Earth, Mars).
    Rocky,
    /// Gas giant (Jupiter, Saturn).
    Gas,
    /// Ice giant (Uranus, Neptune).
    Ice,
}

impl PlanetKind {
    pub fn id(self) -> &'static str {
        match self {
            PlanetKind::Rocky => "rocky",
            PlanetKind::Gas => "gas",
            PlanetKind::Ice => "ice",
        }
    }

    /// Fluent key of the type label (`telescope-kind-rocky`, ...).
    pub fn label_key(self) -> String {
        format!("telescope-kind-{}", self.id())
    }
}

/// One planet: how it is drawn and which texts belong to it.
#[derive(Debug, Clone, PartialEq)]
pub struct Planet {
    pub id: &'static str,
    pub kind: PlanetKind,
    /// Position from the Sun, 1..=8.
    pub order: u8,
    /// Relative display size (not to scale, only a hint: giants larger), 0.6..=1.0.
    pub size: f32,
    /// Main and second colour (`#RRGGBB`) of the disc.
    pub colors: [&'static str; 2],
    /// Has a ring (only Saturn).
    pub ring: bool,
    /// Number of horizontal bands (gas giants), 0 = none.
    pub bands: u8,
    /// Number of craters (Mercury) / clouds (Venus, Earth) drawn as spots.
    pub spots: u8,
}

impl Planet {
    /// Fluent key of the planet name.
    pub fn name_key(&self) -> String {
        format!("telescope-{}-name", self.id)
    }

    /// Fluent key of the one sentence at a reading level.
    pub fn text_key(&self, level: ReadingLevel) -> String {
        format!("telescope-{}-{}", self.id, level.id())
    }
}

/// The eight planets in order from the Sun.
pub const PLANETS: [Planet; 8] = [
    Planet {
        id: "mercury",
        kind: PlanetKind::Rocky,
        order: 1,
        size: 0.6,
        colors: ["#9c9a96", "#6f6d6a"],
        ring: false,
        bands: 0,
        spots: 4,
    },
    Planet {
        id: "venus",
        kind: PlanetKind::Rocky,
        order: 2,
        size: 0.72,
        colors: ["#f0d27a", "#d9a94a"],
        ring: false,
        bands: 0,
        spots: 3,
    },
    Planet {
        id: "earth",
        kind: PlanetKind::Rocky,
        order: 3,
        size: 0.74,
        colors: ["#3f86d6", "#4fae5c"],
        ring: false,
        bands: 0,
        spots: 3,
    },
    Planet {
        id: "mars",
        kind: PlanetKind::Rocky,
        order: 4,
        size: 0.66,
        colors: ["#d8553a", "#a63b27"],
        ring: false,
        bands: 0,
        spots: 2,
    },
    Planet {
        id: "jupiter",
        kind: PlanetKind::Gas,
        order: 5,
        size: 1.0,
        colors: ["#e3a869", "#f4e4cf"],
        ring: false,
        bands: 5,
        spots: 1,
    },
    Planet {
        id: "saturn",
        kind: PlanetKind::Gas,
        order: 6,
        size: 0.9,
        colors: ["#e8cc8a", "#c9a45e"],
        ring: true,
        bands: 3,
        spots: 0,
    },
    Planet {
        id: "uranus",
        kind: PlanetKind::Ice,
        order: 7,
        size: 0.84,
        colors: ["#a9e6ec", "#80c9d4"],
        ring: false,
        bands: 0,
        spots: 0,
    },
    Planet {
        id: "neptune",
        kind: PlanetKind::Ice,
        order: 8,
        size: 0.84,
        colors: ["#2f55c9", "#1f3a96"],
        ring: false,
        bands: 2,
        spots: 0,
    },
];

/// The planet with an id.
pub fn planet(id: &str) -> Option<&'static Planet> {
    PLANETS.iter().find(|p| p.id == id)
}

/// The table as JSON for the host (`telescope_json`): per planet the look and the Fluent
/// keys (the host asks `t(key)` for the texts; the sentence key per reading level).
pub fn to_json() -> String {
    let mut out = String::from("{\"planets\":[");
    for (i, p) in PLANETS.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        let texts: Vec<String> = ReadingLevel::ALL
            .iter()
            .map(|l| format!("\"{}\":\"{}\"", l.id(), p.text_key(*l)))
            .collect();
        out.push_str(&format!(
            "{{\"id\":\"{}\",\"kind\":\"{}\",\"order\":{},\"size\":{},\"colors\":[\"{}\",\"{}\"],\"ring\":{},\"bands\":{},\"spots\":{},\"name\":\"{}\",\"kind_key\":\"{}\",\"texts\":{{{}}}}}",
            p.id,
            p.kind.id(),
            p.order,
            p.size,
            p.colors[0],
            p.colors[1],
            p.ring,
            p.bands,
            p.spots,
            p.name_key(),
            p.kind.label_key(),
            texts.join(",")
        ));
    }
    out.push_str("]}");
    out
}
