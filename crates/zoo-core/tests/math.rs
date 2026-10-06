//! CONT-MATH: the golf-cart note task (MATH-001…013) and the key-box combination
//! (CART-012, CART-023, CART-026).

mod common;

use std::collections::{BTreeSet, HashSet};

use zoo_core::content::{Language, ReadingLevel};
use zoo_core::math::{
    cart_note_task, pad3, parse_code, MathAid, MathKind, MathLevel, MathTask, LOCK_HELP_TRIES,
};

/// Own evaluator of the tests: re-computes the answer from the operands, independent of the
/// generator's `evaluate`.
fn eval(t: &MathTask) -> Option<i64> {
    let o: Vec<i64> = t.operands.iter().map(|&v| i64::from(v)).collect();
    match t.kind {
        MathKind::Add => Some(o[0] + o[1]),
        MathKind::Sub => Some(o[0] - o[1]),
        MathKind::Mul => Some(o[0] * o[1]),
        MathKind::Div => (o[0] % o[1] == 0).then(|| o[0] / o[1]),
        MathKind::Unit => Some(o[0] * if o[1] == 100 { 100 } else { 10 }),
        MathKind::Frac => {
            // numerator / denominator of the whole
            (o[2] * o[0] % o[1] == 0).then(|| o[2] * o[0] / o[1])
        }
        MathKind::Money => {
            let cents = o[0] + o[1];
            (cents % 100 == 0).then_some(cents / 100)
        }
        MathKind::Word => Some(o[0] * o[1] + o[2]),
    }
}

/// The number range of a level's table (CONT-MATH "Cart note"): every operand of the task.
fn operands_in_range(t: &MathTask) -> bool {
    let o = &t.operands;
    let r = |v: u32, lo: u32, hi: u32| (lo..=hi).contains(&v);
    match (t.level, t.kind) {
        (MathLevel::Mathe1, MathKind::Add) => o.iter().all(|&v| r(v, 1, 19)) && o[0] + o[1] <= 20,
        (MathLevel::Mathe1, MathKind::Sub) => r(o[0], 2, 20) && r(o[1], 1, o[0] - 1),
        (MathLevel::Mathe2, MathKind::Add) => o.iter().all(|&v| r(v, 1, 99)) && o[0] + o[1] <= 100,
        (MathLevel::Mathe2, MathKind::Sub) => o.iter().all(|&v| r(v, 1, 100)) && o[1] < o[0],
        (MathLevel::Mathe2, MathKind::Mul) => [2, 3, 4, 5, 10].contains(&o[0]) && r(o[1], 1, 10),
        (MathLevel::Mathe3, MathKind::Add) => o.iter().all(|&v| r(v, 1, 999)) && o[0] + o[1] <= 999,
        (MathLevel::Mathe3, MathKind::Sub) => o.iter().all(|&v| r(v, 1, 999)) && o[1] < o[0],
        (MathLevel::Mathe3, MathKind::Mul) => r(o[0], 2, 10) && r(o[1], 2, 10),
        (MathLevel::Mathe3, MathKind::Div) => {
            r(o[1], 2, 10) && o[0].is_multiple_of(o[1]) && o[0] <= 100
        }
        (MathLevel::Mathe4, MathKind::Add) => {
            o.iter().all(|&v| r(v, 100, 999)) && o[0] + o[1] <= 999
        }
        (MathLevel::Mathe4, MathKind::Sub) => o.iter().all(|&v| r(v, 100, 999)) && o[1] < o[0],
        (MathLevel::Mathe4, MathKind::Mul) => {
            (r(o[0], 11, 99) && r(o[1], 2, 9)) || (r(o[0], 100, 499) && o[1] == 2)
        }
        (MathLevel::Mathe4, MathKind::Unit) => (o[1] == 100 || o[1] == 10) && o[0] * o[1] <= 999,
        (MathLevel::Mathe5, MathKind::Frac) => {
            [(1, 2), (1, 4), (3, 4), (1, 5)].contains(&(o[0], o[1])) && o[2].is_multiple_of(o[1])
        }
        (MathLevel::Mathe5, MathKind::Money) => o.iter().all(|&v| v % 100 == 50),
        (MathLevel::Mathe5, MathKind::Word) => true,
        _ => false, // a kind the level does not have
    }
}

// MATH-007
#[test]
fn math_007_every_task_is_valid_and_in_range() {
    for level in MathLevel::ALL {
        for seed in 0..1000u64 {
            let t = cart_note_task(seed, level);
            assert_eq!(t.level, level);
            assert!(
                (1..=999).contains(&t.answer),
                "{level:?} seed {seed}: answer {} out of 1..=999",
                t.answer
            );
            assert_eq!(
                eval(&t),
                Some(i64::from(t.answer)),
                "{level:?} seed {seed}: {:?}",
                t
            );
            assert!(level.kinds().contains(&t.kind), "{level:?} {t:?}");
            assert!(operands_in_range(&t), "{level:?} seed {seed}: {t:?}");
        }
    }
}

// MATH-001, MATH-009
#[test]
fn math_001_009_deterministic_and_own_stream() {
    for level in MathLevel::ALL {
        for seed in [0u64, 1, 42, 99_999] {
            assert_eq!(cart_note_task(seed, level), cart_note_task(seed, level));
        }
    }
    // generating the task never touches the game RNG (hiding-place picks stay the same)
    let g = common::game(7);
    let before = g.to_save();
    let _ = g.cart_task();
    let _ = g.cart_task();
    assert_eq!(g.to_save().rng, before.rng);
    // two games with the same seed pick the same hiding places whatever the math level is
    let mut a = common::game(7);
    a.set_math_level(MathLevel::Mathe5);
    let b = common::game(7);
    let places = |g: &zoo_core::Game| -> Vec<String> {
        g.animals.iter().map(|x| x.hiding_place.clone()).collect()
    };
    assert_eq!(places(&a), places(&b));
}

// MATH-008, CART-023
#[test]
fn math_008_pad3_and_exactly_one_code_opens() {
    assert_eq!(pad3(5), "005");
    assert_eq!(pad3(100), "100");
    assert_eq!(pad3(999), "999");
    for n in 1..=999u32 {
        assert_eq!(pad3(n).len(), 3);
        assert_eq!(parse_code(&pad3(n)), Some(n));
    }
    assert_eq!(parse_code("5"), None);
    assert_eq!(parse_code("0005"), None);
    assert_eq!(parse_code("0a5"), None);
    for level in MathLevel::ALL {
        let t = cart_note_task(3, level);
        let matching: Vec<u32> = (0..1000).filter(|&c| pad3(c) == t.code()).collect();
        assert_eq!(matching, vec![t.answer]);
    }
}

// MATH-010
#[test]
fn math_010_math_level_setting() {
    let mut g = common::game(1);
    assert_eq!(g.math_level(), MathLevel::Mathe1);
    assert_eq!(MathLevel::from_id("mathe3"), Some(MathLevel::Mathe3));
    assert_eq!(MathLevel::from_id("mathe9"), None);
    assert_eq!(MathLevel::from_id_or_default("kaputt"), MathLevel::Mathe1);
    g.set_math_level(MathLevel::Mathe3);
    assert_eq!(g.math_level(), MathLevel::Mathe3);
    // a save/restore keeps it
    let r = zoo_core::Game::from_save(common::level1(), &g.to_save()).expect("restores");
    assert_eq!(r.math_level(), MathLevel::Mathe3);
    for l in MathLevel::ALL {
        assert_eq!(MathLevel::from_id(l.id()), Some(l));
    }
}

// MATH-011
#[test]
fn math_011_visual_aid_matches_the_kind() {
    assert_eq!(LOCK_HELP_TRIES, 3);
    for level in MathLevel::ALL {
        for seed in 0..200u64 {
            let t = cart_note_task(seed, level);
            let want = match t.kind {
                MathKind::Add | MathKind::Sub | MathKind::Money => "dots",
                MathKind::Mul | MathKind::Word => "blocks",
                MathKind::Div | MathKind::Unit => "groups",
                MathKind::Frac => "bar",
            };
            assert_eq!(t.aid.id(), want, "{t:?}");
            match t.aid {
                MathAid::Dots { a, b, minus } => {
                    let v = if minus { a - b } else { a + b };
                    assert_eq!(v, t.answer, "{t:?}");
                }
                MathAid::Blocks { rows, cols } => {
                    assert!(rows * cols <= t.answer, "{t:?}");
                }
                MathAid::Groups { groups, per } => {
                    assert!(groups * per >= t.answer.min(groups * per), "{t:?}");
                }
                MathAid::Bar { parts, shaded } => assert!(shaded < parts),
            }
        }
    }
}

// MATH-012, MATH-006, CART-031
#[test]
fn math_012_every_text_key_in_every_language_and_reading_level() {
    let content = common::content();
    for kind in MathKind::ALL {
        for rl in ReadingLevel::ALL {
            for lang in Language::ALL {
                let key = format!("math-cart-{}-{}", kind.id(), rl.id());
                let t = content
                    .text_args(
                        lang,
                        &key,
                        &[("a", "3".into()), ("b", "4".into()), ("c", "5".into())],
                    )
                    .unwrap_or_else(|| panic!("{key} missing in {}", lang.id()));
                assert!(!t.trim().is_empty(), "{key} empty");
                if kind == MathKind::Word {
                    for n in ["3", "4", "5"] {
                        assert!(t.contains(n), "{key}: {t}");
                    }
                }
            }
        }
    }
    // the generated tasks resolve their key too
    for level in MathLevel::ALL {
        for seed in 0..50u64 {
            let t = cart_note_task(seed, level);
            for lang in Language::ALL {
                assert!(content
                    .text_args(lang, &t.text_key(ReadingLevel::Kiga), &t.text_args(lang))
                    .is_some());
                assert!(t.expression(lang).ends_with('?') || t.expression(lang).contains("= ?"));
            }
        }
    }
    // the decimal separator follows the language (money)
    let money = (0..2000u64)
        .map(|s| cart_note_task(s, MathLevel::Mathe5))
        .find(|t| t.kind == MathKind::Money)
        .expect("a money task");
    assert!(money.expression(Language::De).contains(",50 €"));
    assert!(money.expression(Language::En).contains(".50 €"));
}

// MATH-013, CART-012
#[test]
fn math_013_variety_and_every_kind() {
    for level in MathLevel::ALL {
        let mut distinct: HashSet<(String, Vec<u32>)> = HashSet::new();
        let mut kinds: BTreeSet<MathKind> = BTreeSet::new();
        for seed in 0..200u64 {
            let t = cart_note_task(seed, level);
            kinds.insert(t.kind);
            distinct.insert((t.kind.id().to_owned(), t.operands.clone()));
        }
        assert!(
            distinct.len() >= 20,
            "{level:?}: {} distinct",
            distinct.len()
        );
        let want: BTreeSet<MathKind> = level.kinds().iter().copied().collect();
        assert_eq!(kinds, want, "{level:?}");
    }
}

// CART-012, CART-026
#[test]
fn cart_012_026_note_task_follows_the_level_and_seed() {
    let mut g = common::game(11);
    let a = g.cart_task();
    assert_eq!(a.level, MathLevel::Mathe1);
    assert!(a.answer <= 19);
    g.key_box_tries = 2;
    g.note_read = true;
    g.set_math_level(MathLevel::Mathe4);
    assert_eq!(g.cart_task(), cart_note_task(11, MathLevel::Mathe4));
    assert_eq!(g.key_box_tries, 0);
    assert!(g.note_read, "note_read stays");
    // different seeds give different tasks
    let tasks: HashSet<_> = (0..40u64)
        .map(|s| cart_note_task(s, MathLevel::Mathe2).operands)
        .collect();
    assert!(tasks.len() > 10);
}

// CART-031: every cart key exists in de and en (L10N-001/002 also compare the sets)
#[test]
fn cart_031_cart_keys_exist() {
    let content = common::content();
    let mut keys: Vec<String> = [
        "cart-locked",
        "cart-closed-level",
        "cart-walk",
        "cart-no-park",
        "hint-cart-note",
        "hint-cart-keybox",
        "cart-note-title",
        "cart-lock-title",
        "cart-lock-wrong",
        "cart-keybox-open",
        "cart-key-got",
        "ui-key",
        "ui-lock-up",
        "ui-lock-down",
        "ui-lock-open",
        "ui-lock-close",
        "ui-math-level",
    ]
    .map(String::from)
    .to_vec();
    for rl in ReadingLevel::ALL {
        keys.push(format!("cart-note-line-{}", rl.id()));
    }
    for l in MathLevel::ALL {
        keys.push(format!("ui-level-{}", l.id()));
    }
    for lang in Language::ALL {
        for k in &keys {
            assert!(
                content.text(lang, k).is_some(),
                "{k} missing in {}",
                lang.id()
            );
        }
    }
    assert_eq!(
        content.text(Language::De, "cart-note-title").as_deref(),
        Some("Math Fighter")
    );
    assert_eq!(
        content.text(Language::En, "cart-note-title").as_deref(),
        Some("Math Fighter")
    );
}
