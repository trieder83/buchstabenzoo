//! Overview map (GAME-MAP, Q-363): the whole zoo seen from above, built from the level data.
//! Pure data for the host to draw: level parts with their progress, the shapes (paths, water,
//! buildings, enclosures, barriers, trees), the player and a subtle "go there next" mark.
//! It never lists escaped animals or hiding places (MAP-005).

use glam::Vec2;
use serde_json::{json, Value};

use crate::game::Game;
use crate::hints::{candidates, HintKind, HintTracker};
use crate::level::{cell_of, ElementType, Rect};
use crate::AnimalState;

/// Progress of one level part on the map (MAP-010).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartState {
    /// Its entry barrier is still closed (greyed, 🔒).
    Locked,
    /// Unlocked, animals still missing.
    Open,
    /// Every animal of the level is home (✔).
    Solved,
}

impl PartState {
    pub fn id(self) -> &'static str {
        match self {
            PartState::Locked => "locked",
            PartState::Open => "open",
            PartState::Solved => "solved",
        }
    }
}

/// One level part: its rectangle, state and animal counts (a pair counts once).
#[derive(Debug, Clone, PartialEq)]
pub struct OverviewPart {
    pub id: String,
    pub rect: Rect,
    pub state: PartState,
    pub night: bool,
    pub total: u32,
    pub home: u32,
}

/// One enclosure: its animal icon and whether the whole group is home.
#[derive(Debug, Clone, PartialEq)]
pub struct OverviewEnclosure {
    pub id: String,
    pub animal: String,
    pub rect: Rect,
    pub part: usize,
    pub home: bool,
}

/// A drawn rectangle: `class` is `path`, `water`, `building`, `barrier`, `wall` or `trees`;
/// `kind` the element kind (entrance, food_storage, zookeeper_house, kiosk, ...).
#[derive(Debug, Clone, PartialEq)]
pub struct OverviewShape {
    pub class: &'static str,
    pub kind: String,
    pub id: String,
    pub rect: Rect,
    pub part: usize,
}

/// Where to go next: the level part, and a point only for places that are not an animal's
/// hiding area (board, storage, gate, bed, moon door, ...).
#[derive(Debug, Clone, PartialEq)]
pub struct NextMark {
    pub part: usize,
    pub kind: &'static str,
    pub pos: Option<Vec2>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Overview {
    pub bounds: Rect,
    pub parts: Vec<OverviewPart>,
    pub enclosures: Vec<OverviewEnclosure>,
    pub shapes: Vec<OverviewShape>,
    pub player: Vec2,
    pub facing: Vec2,
    pub player_part: Option<usize>,
    pub next: Option<NextMark>,
}

fn shape_class(ty: ElementType, kind: &str) -> Option<&'static str> {
    match ty {
        ElementType::Path => Some("path"),
        ElementType::Building => Some("building"),
        ElementType::Barrier => Some("barrier"),
        ElementType::Boundary => Some("wall"),
        ElementType::Landmark if crate::level::WATER_KINDS.contains(&kind) || kind == "pool" => {
            Some("water")
        }
        ElementType::Decoration | ElementType::Landmark
            if matches!(kind, "trees" | "tree_grove") =>
        {
            Some("trees")
        }
        _ => None,
    }
}

/// Builds the overview model. `hints` gives the next target (the compass logic).
pub fn overview(g: &Game, hints: &HintTracker) -> Overview {
    let data = &g.level.data;
    let mut parts: Vec<OverviewPart> = Vec::new();
    for (k, p) in data.parts.iter().enumerate() {
        let ids = g.part_animal_ids(k);
        let home = ids
            .iter()
            .filter(|id| {
                g.group(id)
                    .iter()
                    .all(|&i| g.animals[i].state == AnimalState::InEnclosure)
            })
            .count() as u32;
        let total = ids.len() as u32;
        let state = if !g.part_unlocked(k) {
            PartState::Locked
        } else if total > 0 && home == total {
            PartState::Solved
        } else {
            PartState::Open
        };
        parts.push(OverviewPart {
            id: p.id.clone(),
            rect: p.bounds,
            state,
            night: p.night,
            total,
            home,
        });
    }
    let mut enclosures = Vec::new();
    let mut shapes = Vec::new();
    for e in &data.elements {
        let kind = e.kind.as_deref().unwrap_or("");
        if e.ty == ElementType::Enclosure {
            if e.indoor {
                continue;
            }
            let Some(animal) = e.animal.as_deref() else {
                continue;
            };
            let group = g.group(animal);
            let home = !group.is_empty()
                && group
                    .iter()
                    .all(|&i| g.animals[i].state == AnimalState::InEnclosure);
            enclosures.push(OverviewEnclosure {
                id: e.id.clone(),
                animal: animal.to_owned(),
                rect: e.rect,
                part: e.part,
                home,
            });
        } else if let Some(class) = shape_class(e.ty, kind) {
            shapes.push(OverviewShape {
                class,
                kind: kind.to_owned(),
                id: e.id.clone(),
                rect: e.rect,
                part: e.part,
            });
        }
    }
    let player = g.player.pos;
    let player_part = data.part_at(cell_of(player));
    // the next target: the best hint candidate, never the animal's hiding place
    let next = candidates(g, hints).first().and_then(|h| {
        let part = data.part_at(cell_of(h.pos))?;
        let hidden = matches!(h.kind, HintKind::Animal | HintKind::Help);
        Some(NextMark {
            part,
            kind: h.kind.id(),
            pos: (!hidden).then_some(h.pos),
        })
    });
    let b = data.level.bounds;
    let bounds = parts.iter().fold(None::<Rect>, |acc, p| {
        Some(match acc {
            None => p.rect,
            Some(a) => {
                let (x0, z0) = (a.x.min(p.rect.x), a.z.min(p.rect.z));
                let x1 = (a.x + a.w).max(p.rect.x + p.rect.w);
                let z1 = (a.z + a.d).max(p.rect.z + p.rect.d);
                Rect {
                    x: x0,
                    z: z0,
                    w: x1 - x0,
                    d: z1 - z0,
                }
            }
        })
    });
    Overview {
        bounds: bounds.unwrap_or(b),
        parts,
        enclosures,
        shapes,
        player,
        facing: g.player.facing,
        player_part,
        next,
    }
}

fn rect_json(r: Rect) -> Value {
    json!([r.x, r.z, r.w, r.d])
}

impl Overview {
    /// JSON for the host: `{"bounds", "parts": [{id, rect, state, night, total, home}],
    /// "enclosures": [{id, animal, rect, part, home}], "shapes": [{class, kind, rect, part}],
    /// "player": {x, z, fx, fz, part}, "next": null | {part, kind, x, z}}` (rect = `[x, z, w, d]`).
    pub fn to_json(&self) -> String {
        let parts: Vec<Value> = self
            .parts
            .iter()
            .map(|p| {
                json!({"id": p.id, "rect": rect_json(p.rect), "state": p.state.id(),
                    "night": p.night, "total": p.total, "home": p.home})
            })
            .collect();
        let enclosures: Vec<Value> = self
            .enclosures
            .iter()
            .map(|e| {
                json!({"id": e.id, "animal": e.animal, "rect": rect_json(e.rect),
                    "part": e.part, "home": e.home})
            })
            .collect();
        let shapes: Vec<Value> = self
            .shapes
            .iter()
            .map(|s| json!({"class": s.class, "kind": s.kind, "rect": rect_json(s.rect), "part": s.part}))
            .collect();
        let next = self.next.as_ref().map(|n| {
            json!({"part": n.part, "kind": n.kind,
                "x": n.pos.map(|p| p.x), "z": n.pos.map(|p| p.y)})
        });
        json!({
            "bounds": rect_json(self.bounds),
            "parts": parts,
            "enclosures": enclosures,
            "shapes": shapes,
            "player": {"x": self.player.x, "z": self.player.y,
                "fx": self.facing.x, "fz": self.facing.y, "part": self.player_part},
            "next": next,
        })
        .to_string()
    }
}
