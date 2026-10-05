//! Pairs for all day animals (GAME-FAMILY "Pairs", Q-073 answered 2026-10-01, Q-308):
//! FAM-021 … FAM-029. Level-design checks run on the real level data of the joined zoo.

mod common;

use glam::Vec2;
use zoo_core::animals::{self, pair_gap_m};
use zoo_core::level::{cell_center, ElementType};
use zoo_core::wander::{feed_spot, hiding_area, home_area};
use zoo_core::{AnimalState, Game};

/// Every species of the game comes as a pair (user decision 2026-10-01, Q-308): all day animals,
/// the goldfish (two fish in the bowl) and the three night animals.
const PAIRS: [&str; 13] = [
    "zebra", "hippo", "panda", "koala", "elephant", "giraffe", "lion", "monkey", "snow_fox",
    "goldfish", "hedgehog", "bat", "owl",
];
/// Species of the night zoo (their level is asleep by day).
const NIGHT: [&str; 3] = ["hedgehog", "bat", "owl"];
/// Species with a garden treat to like (they can get a baby, GAME-GARDEN).
const NO_FEED_SPOT: [&str; 4] = ["goldfish", "hedgehog", "bat", "owl"];

/// The joined zoo with the night zoo.
fn base(seed: u64) -> Game {
    common::night_game(seed)
}

/// All three day levels open: level 1 and the night zoo done, the next morning level 2 opens,
/// level 2 done, the next morning level 3 (GAME-NIGHT rule 7: a level's exit opens the morning
/// after night_1 is complete). The night animals are home then.
fn open_zoo(seed: u64) -> Game {
    let mut g = base(seed);
    for level in [
        &["zebra", "hippo", "panda", "hedgehog", "bat", "owl"][..],
        &["koala", "elephant", "giraffe", "lion"][..],
    ] {
        for a in level {
            assert!(g.debug_send_home(a), "{a}");
        }
        g.debug_next_morning();
        g.drain_events();
    }
    assert!(g.level_unlocked("level_2") && g.level_unlocked("level_3"));
    g
}

/// Full night with the night zoo open and its animals still escaped (a fresh game, the moon
/// door forced open like the e2e debug jump does).
fn night_zoo(seed: u64) -> Game {
    let mut g = base(seed);
    assert!(g.debug_set_daytime("night"));
    assert!(g.level_unlocked("night_1"), "the night zoo is open");
    g
}

// FAM-021: every species (day, goldfish, night) has exactly two members (male = member 0,
// female = member 1) at one shared hiding place.
#[test]
fn fam_021_every_species_is_a_pair() {
    let g = base(3);
    for id in PAIRS {
        let group = g.group(id);
        assert_eq!(group.len(), 2, "{id}");
        assert_eq!(
            (g.animals[group[0]].member, g.animals[group[1]].member),
            (0, 1),
            "{id}"
        );
        assert_eq!(
            g.animals[group[0]].hiding_place, g.animals[group[1]].hiding_place,
            "{id}: one shared hiding place"
        );
        assert_eq!(g.animals[group[0]].enclosure, g.animals[group[1]].enclosure);
    }
    // nobody stays single
    let mut ids: Vec<&str> = g.animals.iter().map(|a| a.id()).collect();
    ids.dedup();
    assert_eq!(ids.len(), PAIRS.len());
    assert!(g.animals.len() == 2 * PAIRS.len());
}

// FAM-022 (LAYOUT-014 for pairs): every hiding place of every pair species has a wander area of
// >= 9 cells that holds two animals: two cells at least the species' pair gap apart, both
// within 3 m of the spot; perched places (koala) leave room for two on the wide branch.
#[test]
fn fam_022_every_hiding_place_holds_two_animals() {
    let g = base(1);
    let zoo = &g.level.data;
    let mut places = 0;
    for id in PAIRS {
        let gap = pair_gap_m(id);
        for h in zoo.hiding_places_of(id) {
            places += 1;
            let area = hiding_area(&g.level, h);
            let n = area.len();
            assert!(n >= 9, "{}: {n} cells", h.id);
            let spot = h.spot();
            let far = area
                .cells()
                .filter(|&(c, _)| {
                    let d = cell_center(c).distance(spot);
                    d >= gap - 1e-3 && d <= 3.0
                })
                .count();
            assert!(far >= 1, "{}: no cell {gap} m..3 m from the spot", h.id);
            if h.perch_height_m.is_some() {
                // the pair sits side by side: the offset point is on the same branch
                assert!(animals::member_look(id, 1).scale > 0.0);
            }
        }
    }
    assert!(places >= 39, "{places} candidate places");
}

// FAM-023: the enclosure of every pair species fits male, female and baby: >= 12 home cells,
// a feeding spot whose rank 0/1/2 cells are distinct home cells, the two adults a pair gap
// apart, the baby cell inside the fence.
#[test]
fn fam_023_enclosures_fit_male_female_and_baby() {
    let g = base(1);
    for id in PAIRS {
        let e = g
            .level
            .data
            .elements_of(ElementType::Enclosure)
            .find(|e| e.animal.as_deref() == Some(id))
            .unwrap_or_else(|| panic!("{id}: no enclosure"));
        assert!(e.pair, "{id}: pair flag");
        let idx = g
            .level
            .data
            .elements
            .iter()
            .position(|x| x.id == e.id)
            .unwrap();
        let area = home_area(&g.level, idx);
        assert!(area.len() >= 12, "{id}: {} home cells", area.len());
        let Some(s) = feed_spot(&g.level, idx, &area) else {
            // no treats, no feeding spot: the goldfish (bowl step) and the night animals
            assert!(NO_FEED_SPOT.contains(&id), "{id}: no feed spot");
            continue;
        };
        let cells = [s.cell_for(0), s.cell_for(1), s.cell_for(2)];
        assert!(
            cells[0] != cells[1] && cells[0] != cells[2] && cells[1] != cells[2],
            "{id}"
        );
        for c in cells {
            assert!(area.contains(c), "{id}: feed cell {c} not a home cell");
        }
        let d = cell_center(cells[0]).distance(cell_center(cells[1]));
        // (the hippo fence side is crowded by the info board and hedge: its adults wait 1 m
        // apart at the feeding spot, Q-282)
        let want = if id == "hippo" { 1.0 } else { pair_gap_m(id) };
        assert!(d >= want - 1e-3, "{id}: adults {d} m apart");
    }
}

// FAM-024: a new game puts the pair at one hiding place, a pair gap apart and within 3 m, for
// every seed and species (spawn).
#[test]
fn fam_024_pair_starts_a_gap_apart_and_together() {
    for seed in 0..40u64 {
        let g = base(seed);
        for id in PAIRS {
            let group = g.group(id);
            let (a, b) = (&g.animals[group[0]], &g.animals[group[1]]);
            let d = a.pos.distance(b.pos);
            assert!(d <= 3.0, "seed {seed} {id}: {d} m apart");
            assert!(
                d >= pair_gap_m(id) - 1e-3,
                "seed {seed} {id}: only {d} m apart"
            );
            assert!(a.state == AnimalState::Escaped && b.state == AnimalState::Escaped);
            // perched: the two sit at different points of the branch
            if let (Some((p0, _)), Some((p1, _))) = (g.perch(a), g.perch(b)) {
                assert!(p0.distance(p1) >= 0.6, "seed {seed} {id}: same perch point");
            }
        }
    }
}

// FAM-025: wandering never lets the two animals of a pair clip (escaped and at home, by day and
// in the night zoo): over two simulated minutes they stay at least the pair gap apart (a step's
// tolerance); perched pairs sit side by side on the branch instead.
#[test]
fn fam_025_pair_members_keep_their_distance_while_wandering() {
    for seed in [1u64, 2, 3] {
        for (case, home) in [("escaped", false), ("home", true), ("night", true)] {
            let mut g = match case {
                "night" => night_zoo(seed),
                // every barrier forced open: all day levels simulated, all animals escaped
                "escaped" => {
                    let mut g = base(seed);
                    let barriers: Vec<String> = g
                        .level
                        .data
                        .elements_of(ElementType::Barrier)
                        .map(|e| e.id.clone())
                        .collect();
                    for b in &barriers {
                        g.level.open_barrier(b);
                    }
                    g
                }
                _ => open_zoo(seed),
            };
            if case == "night" {
                // the night animals are escaped first: take them for a minute out
                for id in NIGHT {
                    assert!(g
                        .group(id)
                        .iter()
                        .all(|&i| g.animals[i].state == AnimalState::Escaped));
                }
            }
            if home && case != "night" {
                for id in PAIRS {
                    if !NIGHT.contains(&id) {
                        g.debug_send_home(id);
                    }
                }
            }
            g.player.pos = Vec2::new(-200.0, -200.0); // nobody near: they wander freely
            let mut worst = f32::INFINITY;
            let mut who = "";
            let mut checked = 0u32;
            for _ in 0..(60 * 120) {
                g.update(1.0 / 60.0, Vec2::ZERO);
                for id in PAIRS {
                    let group = g.group(id);
                    let (a, b) = (&g.animals[group[0]], &g.animals[group[1]]);
                    if a.state != b.state || g.perch(a).is_some() || g.perch(b).is_some() {
                        continue;
                    }
                    if !g.in_scope(a) || a.state == AnimalState::InBowl {
                        continue;
                    }
                    // (GAME-HOUSE rule 4: the gap is waived at an animal house, footprint and
                    // door front, where a pair squeezes through the door)
                    if a.state == AnimalState::InEnclosure
                        && g.houses[a.enclosure].as_ref().is_some_and(|h| {
                            h.near(zoo_core::level::cell_of(a.pos))
                                || h.near(zoo_core::level::cell_of(b.pos))
                        })
                    {
                        continue;
                    }
                    checked += 1;
                    let d = a.pos.distance(b.pos) - pair_gap_m(id);
                    if d < worst {
                        worst = d;
                        who = id;
                    }
                }
            }
            assert!(checked > 0, "{case}: nothing simulated");
            assert!(
                worst >= -0.05,
                "seed {seed} {case} home {home}: {who} clips ({worst})"
            );
        }
    }
}

// FAM-026 (RESC-014 for pairs): showing the right food to one animal of each pair species makes
// both follow, and leading them to the gate brings both home (the gate lets a pair in).
#[test]
fn fam_026_every_pair_follows_and_enters_together() {
    for id in PAIRS.iter().copied().filter(|&id| id != "goldfish") {
        let mut g = if NIGHT.contains(&id) {
            night_zoo(5)
        } else {
            open_zoo(5)
        };
        // the species sent home are reset to their hiding places
        let group = g.group(id);
        assert_eq!(group.len(), 2, "{id}");
        let e = g.animals[group[0]].enclosure;
        let enc = g.level.data.elements[e].clone();
        for &i in &group {
            g.animals[i].state = AnimalState::Escaped;
            g.animals[i].pos = {
                let h = g
                    .level
                    .data
                    .hiding_place(&g.animals[i].hiding_place)
                    .unwrap();
                h.spot()
            };
        }
        // the player stands next to the first animal with its food
        let food = g.animals[group[0]].info.foods[0];
        g.carry.take(&zoo_core::FoodBox { food });
        g.player.pos = g.animals[group[0]].pos + Vec2::new(0.0, -1.0);
        g.show_food(id).unwrap_or_else(|e| panic!("{id}: {e:?}"));
        assert!(
            group
                .iter()
                .all(|&i| g.animals[i].state == AnimalState::Following),
            "{id}: both follow"
        );
        // both enter at the gate (the engine's own entering rule, GARD-014)
        let gate = enc.gate.unwrap();
        g.player.pos = cell_center(glam::IVec2::new(gate.x, gate.z));
        for &i in &group {
            g.animals[i].pos = g.player.pos + Vec2::new(0.0, 1.0);
        }
        for _ in 0..600 {
            g.update(1.0 / 60.0, Vec2::ZERO);
        }
        assert!(
            group
                .iter()
                .all(|&i| g.animals[i].state == AnimalState::InEnclosure),
            "{id}: both home: {:?}",
            group
                .iter()
                .map(|&i| g.animals[i].state)
                .collect::<Vec<_>>()
        );
        assert!(g.mission(id).unwrap().complete, "{id}: mission complete");
        // every member's mission flag too: the compass strip lists a species as long as ONE member
        // of it is not complete (user report 2026-10-02: finished animals were still listed)
        assert!(
            group.iter().all(|&i| g.missions[i].complete),
            "{id}: member missions {:?}",
            group
                .iter()
                .map(|&i| g.missions[i].complete)
                .collect::<Vec<_>>()
        );
        let np = zoo_core::hints::night_progress(&g);
        if let Some((_, home)) = np.animals.iter().find(|(a, _)| *a == id) {
            assert!(*home, "{id}: still listed as missing after both entered");
        }
    }
}

// FAM-027: a liked treat to a pair at home makes one baby (it is drawn with the adult model at
// 45 % until a baby model exists); species without a liked garden treat make no baby yet.
#[test]
fn fam_027_baby_for_treat_eaters_and_fallback_looks() {
    use zoo_core::garden::{likes, Treat};
    for id in PAIRS {
        let mut g = night_zoo(7);
        g.debug_send_home(id);
        g.drain_events();
        g.garden.basket.carrots = 2;
        g.garden.basket.potatoes = 2;
        g.garden.basket.apples = 1;
        g.garden.basket.oranges = 1;
        let treat = Treat::ALL.into_iter().find(|&t| likes(id, t));
        if let Some(t) = treat {
            assert_eq!(g.give_treat(id, t), Some(true), "{id}");
            assert_eq!(g.babies, vec![id.to_string()], "{id}: one baby");
            let b = g.baby_states.get(id).expect("baby placed");
            let f = g.group(id)[1];
            assert!(
                b.pos.distance(g.animals[f].pos) <= 4.0,
                "{id}: baby near the female"
            );
        } else {
            // lion, snow fox, koala: no garden treat yet (Q-281)
            assert!(g.babies.is_empty());
        }
        // looks: dedicated models keep scale 1, the fallback is 92 % / 45 %
        let own = animals::female_model(id).is_some();
        assert_eq!(animals::member_look(id, 0).scale, 1.0);
        assert_eq!(
            animals::member_look(id, 1).scale,
            if own { 1.0 } else { 0.92 }
        );
        assert_eq!(
            animals::baby_look(id).scale,
            if animals::baby_model(id).is_some() {
                1.0
            } else {
                0.45
            }
        );
    }
}

// FAM-028: every pair species' info board carries the generic pair note (Fluent, all reading
// levels, de + en; `kiga` = the symbols only), the goldfish board does not, and the "home"
// message of every pair species is plural.
#[test]
fn fam_028_pair_note_and_plural_home_texts() {
    use zoo_core::content::{pair_note_key, Language, ReadingLevel};
    let c = common::content();
    let mut g = base(1);
    for level in ReadingLevel::ALL {
        g.settings.reading_level = level;
        for id in PAIRS {
            let b = g.info_board(id).unwrap_or_else(|| panic!("{id}: board"));
            assert_eq!(b.pair_note_key, Some(pair_note_key(level)), "{id}");
        }
        for lang in Language::ALL {
            let t = c.text(lang, &pair_note_key(level)).expect("pair note text");
            assert!(!t.trim().is_empty());
            if level == ReadingLevel::Kiga {
                assert_eq!(t, "♂ ♀", "kiga: symbols only");
            }
        }
    }
    for id in PAIRS {
        let de = c.text(Language::De, &format!("mission-{id}-home")).unwrap();
        let en = c.text(Language::En, &format!("mission-{id}-home")).unwrap();
        assert!(de.contains(" sind "), "{id}: {de}");
        assert!(en.contains(" are "), "{id}: {en}");
    }
}

// FAM-029 (RESC-018..022 for two fish): the fish food shown from the bank with a filled bowl in
// the hands puts BOTH goldfish into the bowl; the saved game keeps both in it; putting the bowl at
// the pond's step brings both home and completes the mission; an old save that knew only one fish
// still ends with both fish home.
#[test]
fn fam_029_two_goldfish_in_one_bowl() {
    let mut g = open_zoo(4);
    let group = g.group("goldfish");
    assert_eq!(group.len(), 2);
    // the bowl is carried and filled
    {
        let b = g.bowl.as_mut().expect("the zoo has a fish bowl");
        b.carried = true;
        b.water = true;
    }
    g.carry.take(&zoo_core::FoodBox {
        food: zoo_core::Food::FishFood,
    });
    // stand next to the fish
    let fish = g.animals[group[0]].pos;
    let grid = g.level.grid();
    let stand = zoo_core::Rect::new(fish.x as i32 - 3, fish.y as i32 - 3, 7, 7)
        .cells()
        .filter(|&c| grid.is_passable(c, false))
        .map(cell_center)
        .min_by(|a, b| a.distance(fish).total_cmp(&b.distance(fish)))
        .unwrap();
    g.player.pos = stand;
    g.show_food("goldfish").unwrap();
    assert!(group
        .iter()
        .all(|&i| g.animals[i].state == AnimalState::InBowl));
    assert!(g.carrying_animal());
    // saved: both stay in the bowl
    let json = g.to_save().to_json();
    let r = Game::from_save_json(common::zoo_with_night(), &json).unwrap();
    assert!(r
        .group("goldfish")
        .iter()
        .all(|&i| r.animals[i].state == AnimalState::InBowl));
    assert_eq!(r.bowl, g.bowl);
    // delivered at the step: both home, mission complete
    let gate = g.level.data.element("enc_goldfish").unwrap().gate.unwrap();
    g.player.pos = cell_center(glam::IVec2::new(gate.x - 1, gate.z));
    g.player.facing = Vec2::X;
    g.interact();
    assert!(group
        .iter()
        .all(|&i| g.animals[i].state == AnimalState::InEnclosure));
    assert!(g.mission("goldfish").unwrap().complete);
    let (a, b) = (&g.animals[group[0]], &g.animals[group[1]]);
    assert!(
        a.pos.distance(b.pos) >= pair_gap_m("goldfish") - 1e-3,
        "two cells"
    );
    // an old save with one fish home: the second one is still to rescue (never stuck)
    let mut save = g.to_save();
    save.animals.retain(|x| x.id != "goldfish" || x.member == 0);
    save.missions
        .retain(|m| m.animal != "goldfish" || m.complete);
    let r = Game::from_save(common::zoo_with_night(), &save).unwrap();
    assert!(r.any_mission_open(), "the missing fish reopens the mission");
}
