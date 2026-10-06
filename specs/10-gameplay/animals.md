---
id: GAME-ANIMALS
title: Animals and enclosures
aspect: gameplay
module: animals
status: draft
depends_on: [GAME-WORLD]
test_prefix: ANIM
updated: 2026-10-03
---

# Animals and enclosures

## Goal

Data and states for every animal: what it eats, where it hides after escaping, and what its
enclosure needs. The mission flow itself is in GAME-RESCUE.

## Animal data (candidate — final list Q-002)

Proposal of 10 animals pending Q-002. Each hiding place needs a location riddle per reading level.

| Animal id | Correct food (box word) | Hiding place (start missions) | Enclosure items |
|---|---|---|---|
| `zebra` | Gras | `loc_river` | bushes, grass |
| `hippo` | Melonen | `loc_pond` | pool, stone (Q-004) |
| `panda` | Bambus | `loc_cave` | bamboo |
| `koala` | Eukalyptus | `loc_tallest_tree` | eucalyptus tree |
| `elephant` | Heu | `loc_mud_pool` | water, logs, bridge? (Q-005) |
| `goldfish` | Fischfutter | river/stream places of its level (bowl needed, GAME-RESCUE) | aquarium/pond, water plants |
| `monkey` | Bananen | `loc_pirate_ship` | climbing frame |
| `giraffe` | Blätter | `loc_playground` | tall feeding rack |
| `lion` | Fleisch | `loc_sun_rocks` | rocks |
| `snow_fox` | Beeren | `loc_ice_cream_kiosk` | shade, cool den |
| `hedgehog` | Käfer | night_1 places (GAME-LEVEL-NIGHT-1) | straw nest, log tunnel |
| `bat` | Obst | night_1 places | branches, ropes |
| `owl` | Käfer | night_1 places | perch poles, owl box |
| `snake` (user request 2026-10-03) | Fisch | `loc_stone_wall`, `loc_pumpkins`, `loc_rowing_boat` (night_2, GAME-LEVEL-NIGHT-2) | terrarium: sand, warm rock, branch, water dish |
| `chameleon` (perched) | Grillen | `loc_lanterns`, `loc_palm`, `loc_vine_arch` (night_2) | terrarium: tall leafy branches, hanging vine |
| `poison_dart_frog` | Fliegen | `loc_stepping_stones`, `loc_ferns`, `loc_rain_barrel` (night_2) | terrarium: big leaves, mossy log, shallow dish, mist |

Riddles and math per animal: CONT-MISSIONS (`specs/20-content/missions/start-missions.md`).

## Basic food and treats (user request 2026-10-03)

The column "Correct food" above is the **basic food**: it makes the escaped animal follow. Each species also has a **treat** (garden treat or another box food) that makes happy hearts **and the baby** at home; the same food can be basic for one species and a treat for another. Full table (16 species), rules and the info board lines: GAME-FEED "Basic food and treats". The three terrarium animals: snake (basic fish, treat eggs), chameleon (basic crickets, treat frozen insects), poison dart frog (basic flies, treat crickets). Their info boards (and every board) show both lines (ANIM-014).

## Animal states

```
escaped ──(shown correct food)──▶ following ──(enters own enclosure)──▶ in_enclosure
   ▲                                  │
   └──────────(never: animals do not escape again)
```

- **Active from the level start (user request 2026-09-30):** every animal of a level is already in `escaped` at its chosen hiding place, visible and simulated when the level starts (GAME-LAYOUT "Joining levels", LAYOUT-044); an animal is never created, revealed or placed later.
- `escaped`: at its hiding place, plays `idle`/`eat`/`drink`, reacts to the player by looking.
  **Wandering** (user decision 2026-09-26): from time to time (pause 6–15 s, random, seeded)
  the animal walks slowly (≈ 0.5 m/s, `walk` clip) to a new spot **within its hiding area**
  (radius ≤ 3 m around the hiding-place spot, walkable cells only, never onto paths the
  player needs to pass, never through props), then idles/eats/drinks again. It never
  leaves the hiding area, so the riddle always stays true. When the player comes within
  3 m, it stops and looks at her (so it is easy to reach and show the food).
  *Proposal (Q-097, M5a):* an escaped animal that is out of the player's reach (e.g. far out
  in the pond, > 2 m from anywhere she can stand) comes towards her when she is within 5 m,
  to the cell of its wander area nearest to her, then stops and looks at her.
  *Implementation (M5a):* `zoo_core::wander` — wander areas as in GAME-LEVEL-1 "Hiding
  places" (rect clip), 4-neighbour routes between cell centres, 0.5 m/s, pauses 6–15 s from
  the game RNG; targets prefer cells whose 8 neighbours belong to the same part of the area
  (an animal does not rest with its head through a fence or over the pool rim). Facing,
  pause and route are saved (ANIM-011).
  *Exception — perches (Q-094 confirmed 2026-10-01, implemented M5b):* an animal whose hiding place has
  `perch_height_m` sits up there and does not wander (ANIM-008 applies to ground hiding
  places only; GAME-RESCUE §12, RESC-026).
- `following`: follows the player (GAME-RESCUE §6).
- `in_enclosure`: inside, plays idle/happy animations and **wanders slowly inside its
  enclosure** the same way (pause 6–15 s, ≈ 0.5 m/s, stays inside the fence, avoids the
  gate cells) — the zoo looks alive. Final state. *Where* it may walk is level data: the
  enclosure's `home_wander_on` surfaces (`grass`, `water` = its pool) and its
  `[[enclosure_feature]]` pool with the entry ramp (GAME-LAYOUT "Enclosure features and
  wandering at home", Q-085 confirmed 2026-10-01). The hippo wanders in and out of `hippo_pool` over the
  ramp and spends most of its time in the water (GAME-LEVEL-1 "Hippo enclosure pool").

## Info board

Every enclosure has an **info board** (*Infotafel*) next to its sign. Its text (per reading
level and language, CONT-READING, CONT-L10N) contains:
1. the animal's name,
2. the **location riddle** for the hiding place chosen in this playthrough (GAME-RESCUE §2),
3. its **food**, using exactly the word printed on the matching food box label,
4. **more about the animal** (*Steckbrief*, user request 2026-09-26): 2–4 short, true,
   child-friendly facts per reading level (e.g. what it looks like, where it comes from,
   something surprising), shown in the info board panel **after** the riddle and the food
   word, **below** it as its own block that takes the remaining panel height and scrolls inside itself (never beside it, also in landscape; PLAY-038) (the riddle is the core of the game and must always be visible first — QA finding
   F2, 2026-09-26; panel fit on phones: Q-070). Facts must
   never reveal the current hiding place (same rule as RESC-011: no place word). Length per
   reading level: `kiga` 1 fact (one word + picture), `klasse1` 3 sentences of ≤ 5 words,
   `klasse2` 3–4 sentences, `klasse3` 4–6 sentences. Texts per animal in CONT-MISSIONS,
   keys `mission-<animal>-facts-<reading_level>`.

On `kiga` the board shows pictures (habitat, food) plus one word each; read-aloud on tap (Q-007).

## Pairs (GAME-FAMILY, Q-308 / Q-280, 2026-10-01)

Every species of the game lives as a **male + female pair** (two animals, member 0 / 1, one
shared hiding place, one mission per species that completes when **both** are home): the table
above lists species, not individual animals. Rules for room, spacing and look: GAME-FAMILY
"Pairs for every species". The goldfish pair is carried in one bowl; the night animals are pairs
too.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| ANIM-001 | Given the animal data, then every animal has ≥ 1 correct food and ≥ 1 hiding place. | unit |
| ANIM-002 | Given an animal in `in_enclosure`, then no event can change its state. | unit |
| ANIM-003 | Given every info board text (all levels, all languages), then the food word equals the label text of a food box with that food. | unit |
| ANIM-004 | Given every hiding place in the data, then it references an existing location in the layout data (GAME-LAYOUT). | unit |
| ANIM-005 | Given seed S picks hiding place H for the zebras, then the zebra info board shows the location riddle for H (at the current reading level and language). | unit |
| ANIM-006 | Given the zebra info board at every reading level and language, then the panel shows facts text (`mission-zebra-facts-<reading_level>`) in addition to riddle and food word. | unit |
| ANIM-007 | Given every facts text, then it contains no place word of the animal's hiding places (as RESC-011) and `klasse1` sentences have ≤ 5 words. | unit |
| ANIM-008 | Given an escaped animal over 120 s of simulated time, then it moved at least twice, never farther than 3 m from its hiding-place spot, never onto a non-walkable cell or into a prop, and its speed never exceeded 0.6 m/s. | unit |
| ANIM-009 | Given the player within 3 m of an escaped animal, then it stops wandering and faces the player. | unit |
| ANIM-010 | Given an animal in its enclosure over 120 s, then it wandered inside the enclosure only and never stood on a gate cell. | unit |
| ANIM-011 | Given the same seed and inputs, then wandering is identical (deterministic); after save/restore it continues identically (GAME-SAVE). | unit |
| ANIM-012 | Given the hippo `in_enclosure` in level 1 over 600 s (seeded), then it stood only on cells of its home wander area (`home_wander_on = ["grass", "water"]`), crossed between grass and pool only over ramp cells, and spent more than half of the time on pool cells (Q-085, confirmed 2026-10-01). | unit |
| ANIM-013 | Given a level that has just started (new game, barrier opened), then every one of its animals is `escaped`, visible and simulated from the first frame; no animal appears later (LAYOUT-044). | unit |
| ANIM-014 | Given every species at every reading level and language, then the info board data has the basic food(s) and ≥ 1 treat (GAME-FEED "Info board"); the words equal the label words of the matching boxes or garden words; the facts of `poison_dart_frog` may say "giftig" only as a fact and nothing on any panel is frightening. | unit |
| ANIM-015 | Given the new species `snake`, `chameleon`, `poison_dart_frog` (pairs), then each has foods, treat, hiding places in night_2, a pair gap and wander rules like every species; the chameleon is a perched species (`perch_height_m`, 0.7 m pair offset) and does not wander at its hiding place. | unit |

## Open questions

- Q-002 final list, Q-004 hippos, Q-005 elephant, Q-030 herd size; Q-036 answered (goldfish bowl, GAME-RESCUE).
- Q-043 Animation set per animal (hiding-place idles, reactions, koala/goldfish locomotion).
- Q-044 How hiding places are represented in the layout data. Q-085 `home_wander_on` and enclosure pools (data shape). Q-097 out-of-reach escaped animal comes towards the player (proposal).
- Q-174 bears for the honey event (Q-131 answered): level and species spec.
