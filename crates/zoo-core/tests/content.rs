//! CONT-READING, CONT-L10N, CONT-MISSIONS tests on the Fluent files.

mod common;

use std::collections::BTreeSet;

use zoo_core::content::{contains_word, default_language, riddle_key, sentence_word_counts};
use zoo_core::{Food, Language, ReadingLevel};

/// Candidate hiding places of the level-1 animals (GAME-LEVEL-1 "Hiding places",
/// CONT-MISSIONS §1–§3): (animal, hiding place).
const MISSIONS: [(&str, &str); 9] = [
    ("zebra", "loc_river"),
    ("zebra", "loc_meadow"),
    ("zebra", "loc_sand"),
    ("hippo", "loc_pond"),
    ("hippo", "loc_mud"),
    ("hippo", "loc_shade"),
    ("panda", "loc_cave"),
    ("panda", "loc_bamboo"),
    ("panda", "loc_leaves"),
];

/// The level-1 animals (the playable missions of level 1).
const ANIMALS: [&str; 3] = ["zebra", "hippo", "panda"];

/// Candidate hiding places of one animal (from `MISSIONS`).
fn candidates(animal: &str) -> impl Iterator<Item = &'static str> + '_ {
    MISSIONS
        .iter()
        .filter(move |(a, _)| *a == animal)
        .map(|(_, p)| *p)
}

/// Whole-word check of a (possibly multi-word) label: true if the text contains any word of
/// the label (Q-083 answered: two-word `kiga` labels are checked word by word).
fn contains_any_word(text: &str, label: &str) -> bool {
    label.split_whitespace().any(|w| contains_word(text, w))
}

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
    // L10N-005 (user decision 2026-09-26): German regardless of the browser language
    assert_eq!(default_language("en"), Language::De);
    assert_eq!(default_language("en-US"), Language::De);
    assert_eq!(default_language("de-AT"), Language::De);
    assert_eq!(default_language(""), Language::De);
}

// READ-002
#[test]
fn read_002_klasse1_max_5_words_per_sentence() {
    let c = common::content();
    for lang in Language::ALL {
        for key in c.keys(lang).iter().filter(|k| k.ends_with("-klasse1")) {
            // (math word problems have number arguments)
            let args = [
                ("a", "3".to_owned()),
                ("b", "4".to_owned()),
                ("c", "5".to_owned()),
            ];
            let text = c.text_args(lang, key, &args).unwrap();
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

// MISS-001, RESC-003, MISS-008 (level 1: all 9 candidate hiding places)
#[test]
fn miss_001_resc_003_miss_008_level1_riddles_all_levels_and_languages() {
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
        c.text(Language::De, "mission-zebra-riddle-loc_river-klasse1")
            .unwrap(),
        "Ich habe Durst. Ich suche fließendes Wasser."
    );
    assert_eq!(
        c.text(Language::En, "mission-zebra-riddle-loc_river-klasse1")
            .unwrap(),
        "I am thirsty. I look for running water."
    );
    // the legacy PoC keys without a hiding place are gone (CONT-MISSIONS Behaviour 1)
    for lang in Language::ALL {
        assert!(c.text(lang, "mission-zebra-riddle-klasse1").is_none());
    }
    assert_eq!(c.text(Language::De, "food-grass").unwrap(), "Gras");
    assert_eq!(c.text(Language::En, "food-grass").unwrap(), "grass");
    // compare the zebra loc_river riddle with the first riddle table of §1 in the spec
    let md = common::read("specs/20-content/missions/start-missions.md");
    let section = md
        .split("## 1. Zebra")
        .nth(1)
        .unwrap()
        .split("\n## ")
        .next()
        .unwrap();
    let river = section
        .split("Riddle — `loc_river`")
        .nth(1)
        .expect("loc_river table")
        .split("Riddle — ")
        .next()
        .unwrap();
    for level in ["klasse1", "klasse2", "klasse3"] {
        let row = river
            .lines()
            .find(|l| l.starts_with(&format!("| {level} |")))
            .unwrap();
        let cols: Vec<&str> = row.split('|').map(str::trim).collect();
        let key = format!("mission-zebra-riddle-loc_river-{level}");
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

// RESC-011 (level 1; matching rule = whole word, case-insensitive, Q-039; two-word labels
// word by word, Q-083)
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
                    !contains_any_word(&t, &word),
                    "{animal} {} {}: contains {word}",
                    lang.id(),
                    level.id()
                );
            }
        }
    }
    assert!(contains_word("Der Fluss fließt.", "fluss"));
    assert!(!contains_word("Das Flusspferd", "Fluss"));
    assert!(contains_any_word("A big pile of leaves.", "leaf pile"));
    assert!(!contains_any_word("Red leaves.", "leaf pile"));
}

// MISS-007 (level 1): no klasse1–klasse3 riddle of one candidate contains the kiga word of
// another candidate of the same animal (whole word, each word of a two-word label)
#[test]
fn miss_007_riddle_never_names_another_candidate() {
    let c = common::content();
    for animal in ANIMALS {
        for lang in Language::ALL {
            for place in candidates(animal) {
                for other in candidates(animal).filter(|o| *o != place) {
                    let word = c
                        .text(lang, &riddle_key(animal, other, ReadingLevel::Kiga))
                        .unwrap();
                    for level in [
                        ReadingLevel::Klasse1,
                        ReadingLevel::Klasse2,
                        ReadingLevel::Klasse3,
                    ] {
                        let t = c.text(lang, &riddle_key(animal, place, level)).unwrap();
                        assert!(
                            !contains_any_word(&t, &word),
                            "{animal} {place} {} {}: contains {word} (kiga word of {other})",
                            lang.id(),
                            level.id()
                        );
                    }
                }
            }
        }
    }
}

/// Words that name or point at a hiding place (ANIM-007): the `kiga` riddle word (as
/// RESC-011) plus the place's visible features (GAME-LEVEL-1 `features`, "Place words" in
/// CONT-MISSIONS §1–§3). Food words (*Gras*, *Bambus*, ...) are never listed.
fn place_words(place: &str, lang: Language) -> &'static [&'static str] {
    use Language::{De, En};
    match (place, lang) {
        ("loc_river", De) => &["fluss", "flüsse", "bach", "brücke", "enten", "wasser"],
        ("loc_river", En) => &["river", "rivers", "stream", "bridge", "ducks", "water"],
        ("loc_meadow", De) => &["wiese", "blumen", "schmetterlinge"],
        ("loc_meadow", En) => &["meadow", "flowers", "butterflies"],
        ("loc_sand", De) => &["sand", "staubbad"],
        ("loc_sand", En) => &["sand", "dust"],
        ("loc_pond", De) => &["teich", "seerosen", "frösche", "wasser"],
        ("loc_pond", En) => &["pond", "water", "lilies", "frogs"],
        ("loc_mud", De) => &["matsch", "pfütze", "schlamm"],
        ("loc_mud", En) => &["mud", "puddle"],
        ("loc_shade", De) => &["schatten", "mauer", "bäume"],
        ("loc_shade", En) => &["shade", "wall", "trees"],
        ("loc_cave", De) => &["höhle", "stein", "echo", "dunkel", "kühl"],
        ("loc_cave", En) => &["cave", "stone", "echo", "dark", "cool"],
        ("loc_bamboo", De) => &["bambuswald", "dickicht", "stangen"],
        ("loc_bamboo", En) => &["forest", "thicket", "stalks"],
        ("loc_leaves", De) => &["laubhaufen", "laub", "haufen", "blätter"],
        ("loc_leaves", En) => &["leaf", "pile", "heap", "leaves"],
        _ => &[],
    }
}

// ANIM-006
#[test]
fn anim_006_info_board_shows_facts_riddle_and_food() {
    let c = common::content();
    let mut g = common::game(1);
    for animal in ANIMALS {
        for lang in Language::ALL {
            for level in ReadingLevel::ALL {
                g.settings.reading_level = level;
                let b = g.info_board(animal).unwrap();
                assert_eq!(
                    b.facts_key,
                    format!("mission-{animal}-facts-{}", level.id())
                );
                assert_eq!(b.name_key, format!("animal-{animal}"));
                let facts = c.text(lang, &b.facts_key).expect("facts text");
                let riddle = c.text(lang, &b.riddle_key).expect("riddle text");
                let food = c.text(lang, &b.food_key).expect("food word");
                let name = c.text(lang, &b.name_key).expect("animal name");
                let more = c.text(lang, &b.more_key).expect("more-about heading");
                assert!(more.contains(&name) || more.to_lowercase().contains(&name.to_lowercase()));
                for t in [&facts, &riddle, &food, &name] {
                    assert!(!t.trim().is_empty());
                }
                assert_ne!(facts, riddle, "{} {}", lang.id(), level.id());
                // length rules per reading level (GAME-ANIMALS "Info board" item 4)
                let n = sentence_word_counts(&facts);
                match level {
                    ReadingLevel::Kiga => {
                        assert_eq!(n, vec![1], "kiga: one word: {facts}");
                        assert!(
                            !facts.contains(['.', '!', '?']),
                            "kiga: no sentence: {facts}"
                        );
                    }
                    ReadingLevel::Klasse1 => assert_eq!(n.len(), 3, "klasse1: {facts}"),
                    ReadingLevel::Klasse2 => {
                        assert!((3..=4).contains(&n.len()), "klasse2: {facts}")
                    }
                    ReadingLevel::Klasse3 => {
                        assert!((4..=6).contains(&n.len()), "klasse3: {facts}")
                    }
                }
            }
        }
    }
}

// ANIM-007 (whole-word, case-insensitive match as RESC-011 / Q-039)
#[test]
fn anim_007_facts_never_name_the_place() {
    let c = common::content();
    let g = common::game(1);
    for animal in ANIMALS {
        // the animal's own food word stays allowed (e.g. *bamboo* in *bamboo forest*)
        let food_key = g.info_board(animal).unwrap().food_key;
        for lang in Language::ALL {
            let food = c.text(lang, &food_key).unwrap().to_lowercase();
            // place words of ALL candidates: the facts are shown whichever place was picked
            let mut words: Vec<String> = Vec::new();
            for place in candidates(animal) {
                let pw = place_words(place, lang);
                assert!(!pw.is_empty(), "no place words for {place}");
                words.extend(pw.iter().map(|w| w.to_string()));
                let kiga = c
                    .text(lang, &riddle_key(animal, place, ReadingLevel::Kiga))
                    .unwrap()
                    .to_lowercase();
                words.extend(
                    kiga.split_whitespace()
                        .filter(|w| *w != food)
                        .map(str::to_string),
                );
            }
            for level in ReadingLevel::ALL {
                let key = zoo_core::content::facts_key(animal, level);
                let t = c.text(lang, &key).unwrap();
                for w in &words {
                    assert!(
                        !contains_word(&t, w),
                        "{key} ({}) contains the place word {w}: {t}",
                        lang.id()
                    );
                }
                if level == ReadingLevel::Klasse1 {
                    assert!(sentence_word_counts(&t).iter().all(|&n| n <= 5), "{t}");
                }
            }
        }
    }
    // the check really catches a place word
    assert!(contains_word("Zebras trinken am Fluss.", "fluss"));
    assert!(contains_word("They drink WATER.", "water"));
}

/// Compares the facts table of a CONT-MISSIONS section (e.g. "## 1. Zebra") with the Fluent
/// files.
fn assert_facts_match_spec(heading: &str, animal: &str) {
    let c = common::content();
    let md = common::read("specs/20-content/missions/start-missions.md");
    let section = md
        .split(heading)
        .nth(1)
        .unwrap_or_else(|| panic!("section {heading}"))
        .split("\n## ")
        .next()
        .unwrap();
    let facts = section
        .split("**Facts**")
        .nth(1)
        .unwrap_or_else(|| panic!("facts table in {heading}"));
    for level in ["klasse1", "klasse2", "klasse3"] {
        let row = facts
            .lines()
            .find(|l| l.starts_with(&format!("| {level} |")))
            .unwrap();
        let cols: Vec<&str> = row.split('|').map(str::trim).collect();
        let key = format!("mission-{animal}-facts-{level}");
        assert_eq!(
            c.text(Language::De, &key).unwrap(),
            cols[2],
            "{animal} de {level}"
        );
        assert_eq!(
            c.text(Language::En, &key).unwrap(),
            cols[3],
            "{animal} en {level}"
        );
    }
    let kiga = facts.lines().find(|l| l.starts_with("| kiga |")).unwrap();
    for lang in Language::ALL {
        let word = c
            .text(lang, &format!("mission-{animal}-facts-kiga"))
            .unwrap();
        assert!(
            kiga.contains(&format!("**{word}**")),
            "{animal} kiga {word}"
        );
    }
}

// CONT-MISSIONS zebra facts table == Fluent files
#[test]
fn zebra_facts_match_cont_missions() {
    assert_facts_match_spec("## 1. Zebra", "zebra");
}

// CONT-MISSIONS hippo and panda facts tables == Fluent files
#[test]
fn hippo_panda_facts_match_cont_missions() {
    assert_facts_match_spec("## 2. Hippo", "hippo");
    assert_facts_match_spec("## 3. Panda", "panda");
}
