//! GAME-TELESCOPE: the planet table, its Fluent texts and the telescope interactable
//! (TELE-001…006).

mod common;

use glam::Vec2;
use zoo_core::game::Target;
use zoo_core::telescope::{planet, PlanetKind, PLANETS};
use zoo_core::{Content, Game, Interaction, Language, ReadingLevel};

fn content() -> Content {
    let mut sources: Vec<(Language, String)> = Vec::new();
    for (lang, id) in [(Language::De, "de"), (Language::En, "en")] {
        let dir = common::repo_root().join("assets/i18n").join(id);
        for e in std::fs::read_dir(dir).unwrap() {
            let p = e.unwrap().path();
            if p.extension().is_some_and(|x| x == "ftl") {
                sources.push((lang, std::fs::read_to_string(p).unwrap()));
            }
        }
    }
    let refs: Vec<(Language, &str)> = sources.iter().map(|(l, s)| (*l, s.as_str())).collect();
    Content::from_sources(&refs).unwrap()
}

// TELE-001
#[test]
fn tele_001_eight_planets_in_order() {
    let ids: Vec<_> = PLANETS.iter().map(|p| p.id).collect();
    assert_eq!(
        ids,
        ["mercury", "venus", "earth", "mars", "jupiter", "saturn", "uranus", "neptune"]
    );
    for (i, p) in PLANETS.iter().enumerate() {
        assert_eq!(p.order as usize, i + 1);
        assert!((0.6..=1.0).contains(&p.size));
    }
    assert!(planet("saturn").is_some() && planet("pluto").is_none());
}

// TELE-002
#[test]
fn tele_002_kinds() {
    let kind = |id| planet(id).unwrap().kind;
    for id in ["mercury", "venus", "earth", "mars"] {
        assert_eq!(kind(id), PlanetKind::Rocky, "{id}");
    }
    for id in ["jupiter", "saturn"] {
        assert_eq!(kind(id), PlanetKind::Gas, "{id}");
    }
    for id in ["uranus", "neptune"] {
        assert_eq!(kind(id), PlanetKind::Ice, "{id}");
    }
}

// TELE-003, TELE-004
#[test]
fn tele_003_004_texts_at_every_level_in_de_and_en() {
    let c = content();
    for lang in [Language::De, Language::En] {
        for kind in [PlanetKind::Rocky, PlanetKind::Gas, PlanetKind::Ice] {
            assert!(
                c.text(lang, &kind.label_key()).is_some(),
                "{kind:?} {lang:?}"
            );
        }
        for p in &PLANETS {
            let name = c.text(lang, &p.name_key()).unwrap_or_default();
            assert!(!name.is_empty(), "{} name {lang:?}", p.id);
            for level in ReadingLevel::ALL {
                let t = c
                    .text(lang, &p.text_key(level))
                    .unwrap_or_else(|| panic!("{} {level:?} {lang:?}", p.id));
                assert!(!t.is_empty() && t.chars().count() <= 250, "{}: {t}", p.id);
                if level == ReadingLevel::Kiga {
                    assert!(t.split_whitespace().count() <= 6, "kiga too long: {t}");
                }
            }
        }
        for k in ["telescope-title", "telescope-close", "telescope-hint"] {
            assert!(c.text(lang, k).is_some(), "{k} {lang:?}");
        }
    }
    // only Saturn has a ring
    let rings: Vec<_> = PLANETS.iter().filter(|p| p.ring).map(|p| p.id).collect();
    assert_eq!(rings, ["saturn"]);
}

fn stand_at_telescope(g: &mut Game) {
    let r = g.level.data.element("telescope_n1").unwrap().rect;
    let c = Vec2::new(r.x as f32 + 0.5, r.z as f32 + 0.5);
    let grid = g.level.grid();
    let stand = zoo_core::Rect::new(r.x - 3, r.z - 3, 7, 7)
        .cells()
        .filter(|&cell| grid.is_walkable(cell, false))
        .map(zoo_core::level::cell_center)
        .filter(|q| (0.8..=1.8).contains(&q.distance(c)))
        .min_by(|a, b| a.distance(c).total_cmp(&b.distance(c)))
        .expect("a place to stand at the telescope");
    g.player.pos = stand;
    g.player.facing = (c - stand).normalize_or(Vec2::Y);
}

// TELE-005
#[test]
fn tele_005_only_at_night() {
    let mut g = common::night_game(11);
    for a in ["zebra", "hippo", "panda"] {
        assert!(g.debug_send_home(a));
    }
    g.drain_events();
    stand_at_telescope(&mut g);
    assert!(!g.daytime.is_night());
    assert!(!matches!(
        g.available_target(),
        Some(Target::Telescope { .. })
    ));
    assert!(g.debug_set_daytime("night"));
    assert!(g.level_unlocked("night_1"));
    stand_at_telescope(&mut g);
    assert_eq!(
        g.available_target(),
        Some(Target::Telescope {
            id: "telescope_n1".into()
        })
    );
    assert_eq!(g.available_target().unwrap().kind(), "telescope");
    assert_eq!(
        g.interact(),
        Some(Interaction::Telescope {
            id: "telescope_n1".into()
        })
    );
}

// TELE-006
#[test]
fn tele_006_json_lists_the_planets() {
    let j = zoo_core::telescope::to_json();
    assert_eq!(j.matches("\"id\":").count(), 8);
    assert!(j.contains("\"id\":\"saturn\",\"kind\":\"gas\",\"order\":6"));
    assert!(j.contains("\"ring\":true"));
    assert!(j.contains("\"klasse2\":\"telescope-saturn-klasse2\""));
}
