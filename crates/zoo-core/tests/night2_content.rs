//! Texts of the terrarium garden (GAME-LEVEL-NIGHT-2 "Riddles and facts"): every key in de and en,
//! the riddle rules (RESC-011 / MISS-007 / MISS-013 for the new places), READ-002 and the food words.

mod common;

use zoo_core::content::{contains_word, facts_key, riddle_key};
use zoo_core::{Food, Language, ReadingLevel};

const PLACES: [(&str, &str); 9] = [
    ("snake", "loc_stone_wall"),
    ("snake", "loc_pumpkins"),
    ("snake", "loc_rowing_boat"),
    ("chameleon", "loc_lanterns"),
    ("chameleon", "loc_palm"),
    ("chameleon", "loc_vine_arch"),
    ("poison_dart_frog", "loc_stepping_stones"),
    ("poison_dart_frog", "loc_ferns"),
    ("poison_dart_frog", "loc_rain_barrel"),
];

fn any_word(text: &str, label: &str) -> bool {
    label.split_whitespace().any(|w| contains_word(text, w))
}

// CONT-MISSIONS (night_2): a riddle for every candidate place, facts, names, the plural home text
// and the six new food words exist in every reading level and language; the data places match.
#[test]
fn night2_texts_exist_in_every_level_and_language() {
    let c = common::content();
    let data = common::night2();
    for lang in Language::ALL {
        for (animal, place) in PLACES {
            assert!(
                data.hiding_place(place).is_some_and(|h| h.animal == animal),
                "{place}"
            );
            for level in ReadingLevel::ALL {
                for key in [riddle_key(animal, place, level), facts_key(animal, level)] {
                    assert!(
                        c.text(lang, &key).is_some_and(|t| !t.trim().is_empty()),
                        "{} {key}",
                        lang.id()
                    );
                }
            }
        }
        for animal in ["snake", "chameleon", "poison_dart_frog"] {
            for key in [
                format!("animal-{animal}"),
                format!("animal-{animal}-more"),
                format!("mission-{animal}-home"),
            ] {
                assert!(
                    c.text(lang, &key).is_some_and(|t| !t.is_empty()),
                    "{} {key}",
                    lang.id()
                );
            }
        }
        for f in [
            "fish",
            "crickets",
            "flies",
            "eggs",
            "frozen_insects",
            "bone",
        ] {
            assert!(c.text(lang, &format!("food-{f}")).is_some(), "food-{f}");
        }
        for level in ReadingLevel::ALL {
            for key in [
                format!("board-treat-{}", level.id()),
                format!("welcome-level-night_2-{}", level.id()),
            ] {
                assert!(c.text(lang, &key).is_some(), "{} {key}", lang.id());
            }
        }
        for key in [
            "sign-terrarium-house",
            "map-level-night_2",
            "hint-night-gate",
        ] {
            assert!(c.text(lang, key).is_some(), "{} {key}", lang.id());
        }
    }
    // the info-board food words equal the box label words (ANIM-003)
    assert_eq!(Food::Fish.label_key(), "food-fish");
}

// RESC-011 / MISS-007 / MISS-013 (night_2): no klasse1..3 riddle contains the kiga word of its own
// place, of another candidate of the same animal, or of any other night_2 / night_1 place; the facts
// never name a place either (ANIM-007).
#[test]
fn night2_riddles_never_name_a_place() {
    let c = common::content();
    let night1_places = [
        "loc_brush_pile",
        "loc_flowerpots",
        "loc_mushrooms",
        "loc_windmill",
        "loc_fireflies",
        "loc_hollow_tree",
        "loc_moon_pond",
        "loc_hilltop",
        "loc_fir",
    ];
    let night1_animals = [
        "hedgehog", "hedgehog", "hedgehog", "bat", "bat", "bat", "owl", "owl", "owl",
    ];
    for lang in Language::ALL {
        let mut words: Vec<(String, String)> = PLACES
            .iter()
            .map(|(a, p)| {
                (
                    p.to_string(),
                    c.text(lang, &riddle_key(a, p, ReadingLevel::Kiga)).unwrap(),
                )
            })
            .collect();
        for (a, p) in night1_animals.iter().zip(night1_places) {
            words.push((
                p.to_owned(),
                c.text(lang, &riddle_key(a, p, ReadingLevel::Kiga)).unwrap(),
            ));
        }
        for (animal, place) in PLACES {
            for level in [
                ReadingLevel::Klasse1,
                ReadingLevel::Klasse2,
                ReadingLevel::Klasse3,
            ] {
                let t = c.text(lang, &riddle_key(animal, place, level)).unwrap();
                for (other, word) in &words {
                    assert!(
                        !any_word(&t, word),
                        "{} {animal} {place} {}: contains \"{word}\" (kiga word of {other}): {t}",
                        lang.id(),
                        level.id()
                    );
                }
                // the riddle names neither its animal's food nor a food box word
                for f in ["fish", "crickets", "flies", "eggs", "frozen_insects"] {
                    let fw = c.text(lang, &format!("food-{f}")).unwrap();
                    assert!(
                        !contains_word(&t, &fw),
                        "{} {place}: names the food {fw}",
                        lang.id()
                    );
                }
            }
        }
        for animal in ["snake", "chameleon", "poison_dart_frog"] {
            for level in ReadingLevel::ALL {
                let t = c.text(lang, &facts_key(animal, level)).unwrap();
                for (other, word) in &words {
                    if level != ReadingLevel::Kiga {
                        assert!(
                            !any_word(&t, word),
                            "{} facts {animal} {}: names {other}",
                            lang.id(),
                            level.id()
                        );
                    }
                }
            }
        }
    }
}
