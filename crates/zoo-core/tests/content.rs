//! CONT-READING, CONT-L10N, CONT-MISSIONS tests on the Fluent files.

mod common;

use std::collections::BTreeSet;

use zoo_core::content::{contains_word, default_language, riddle_key, sentence_word_counts};
use zoo_core::{Food, Language, ReadingLevel};

/// Missions whose texts are in the Fluent files (PoC: zebra only, PROD-POC).
const MISSIONS: [(&str, &str); 1] = [("zebra", "loc_river")];

fn all_keys(lang: &str) -> BTreeSet<String> {
    // every .ftl file of the language directory
    let dir = common::repo_root().join("assets/i18n").join(lang);
    let mut keys = BTreeSet::new();
    for entry in std::fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|e| e == "ftl") {
            let src = std::fs::read_to_string(&path).unwrap();
            let l = Language::from_id(lang).unwrap();
            let c = zoo_core::Content::from_sources(&[(l, &src)]).unwrap();
            keys.extend(c.keys(l));
        }
    }
    keys
}

// L10N-001
#[test]
fn l10n_001_every_de_key_in_en() {
    let (de, en) = (all_keys("de"), all_keys("en"));
    let missing: Vec<_> = de.difference(&en).collect();
    assert!(missing.is_empty(), "missing in en: {missing:?}");
}

// L10N-002
#[test]
fn l10n_002_no_en_orphans() {
    let (de, en) = (all_keys("de"), all_keys("en"));
    let orphans: Vec<_> = en.difference(&de).collect();
    assert!(orphans.is_empty(), "en keys not in de: {orphans:?}");
}

// L10N-003, L10N-005
#[test]
fn l10n_003_005_default_language() {
    assert_eq!(default_language("fr"), Language::De);
    assert_eq!(default_language("fr-FR"), Language::De);
    assert_eq!(default_language("en"), Language::En);
    assert_eq!(default_language("en-US"), Language::En);
    assert_eq!(default_language("de-AT"), Language::De);
    assert_eq!(default_language(""), Language::De);
}

// READ-002
#[test]
fn read_002_klasse1_max_5_words_per_sentence() {
    let c = common::content();
    for lang in Language::ALL {
        for key in c.keys(lang).iter().filter(|k| k.ends_with("-klasse1")) {
            let text = c.text(lang, key).unwrap();
            for n in sentence_word_counts(&text) {
                assert!(
                    n <= 5,
                    "{} {key}: sentence with {n} words: {text}",
                    lang.id()
                );
            }
        }
    }
    // food labels are shown on klasse1 as well
    for lang in Language::ALL {
        for f in Food::ALL {
            let t = c.text(lang, &f.label_key()).unwrap();
            assert!(sentence_word_counts(&t).iter().all(|&n| n <= 5));
        }
    }
    assert_eq!(
        sentence_word_counts("Ich habe Durst. Ich suche fließendes Wasser."),
        vec![3, 4]
    );
}

// MISS-001 (zebra), RESC-003 (zebra)
#[test]
fn miss_001_resc_003_zebra_riddles_all_levels_and_languages() {
    let c = common::content();
    for (animal, place) in MISSIONS {
        for lang in Language::ALL {
            for level in ReadingLevel::ALL {
                let key = riddle_key(animal, place, level);
                let t = c.text(lang, &key);
                assert!(
                    t.is_some_and(|t| !t.trim().is_empty()),
                    "missing {key} in {}",
                    lang.id()
                );
            }
        }
    }
}

// Exact texts of CONT-MISSIONS §1 (spot check)
#[test]
fn zebra_texts_match_cont_missions() {
    let c = common::content();
    assert_eq!(
        c.text(Language::De, "mission-zebra-riddle-klasse1")
            .unwrap(),
        "Ich habe Durst. Ich suche fließendes Wasser."
    );
    assert_eq!(
        c.text(Language::En, "mission-zebra-riddle-klasse1")
            .unwrap(),
        "I am thirsty. I look for running water."
    );
    assert_eq!(c.text(Language::De, "food-grass").unwrap(), "Gras");
    assert_eq!(c.text(Language::En, "food-grass").unwrap(), "grass");
    // compare every zebra riddle with the table in the spec
    let md = common::read("specs/20-content/missions/start-missions.md");
    let section = md
        .split("## 1. Zebra")
        .nth(1)
        .unwrap()
        .split("\n## ")
        .next()
        .unwrap();
    for level in ["klasse1", "klasse2", "klasse3"] {
        let row = section
            .lines()
            .find(|l| l.starts_with(&format!("| {level} |")))
            .unwrap();
        let cols: Vec<&str> = row.split('|').map(str::trim).collect();
        let key = format!("mission-zebra-riddle-{level}");
        assert_eq!(c.text(Language::De, &key).unwrap(), cols[2], "de {level}");
        assert_eq!(c.text(Language::En, &key).unwrap(), cols[3], "en {level}");
    }
}

// MISS-004
#[test]
fn miss_004_food_words_distinct() {
    let c = common::content();
    for lang in Language::ALL {
        let words: BTreeSet<String> = Food::ALL
            .iter()
            .map(|f| {
                c.text(lang, &f.label_key())
                    .expect("food label")
                    .to_lowercase()
            })
            .collect();
        assert_eq!(words.len(), Food::ALL.len(), "{}", lang.id());
    }
}

// RESC-011 (zebra; matching rule = whole word, case-insensitive, proposal of Q-039)
#[test]
fn resc_011_riddles_do_not_name_the_place() {
    let c = common::content();
    for (animal, place) in MISSIONS {
        for lang in Language::ALL {
            let word = c
                .text(lang, &riddle_key(animal, place, ReadingLevel::Kiga))
                .unwrap();
            for level in [
                ReadingLevel::Klasse1,
                ReadingLevel::Klasse2,
                ReadingLevel::Klasse3,
            ] {
                let t = c.text(lang, &riddle_key(animal, place, level)).unwrap();
                assert!(
                    !contains_word(&t, &word),
                    "{animal} {} {}: contains {word}",
                    lang.id(),
                    level.id()
                );
            }
        }
    }
    assert!(contains_word("Der Fluss fließt.", "fluss"));
    assert!(!contains_word("Das Flusspferd", "Fluss"));
}
