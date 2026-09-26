---
id: GAME-ANIMALS
title: Animals and enclosures
aspect: gameplay
module: animals
status: draft
depends_on: [GAME-WORLD]
test_prefix: ANIM
updated: 2026-09-26
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
| `goldfish` | Fischfutter | `loc_fountain` | water plants |
| `monkey` | Bananen | `loc_pirate_ship` | climbing frame |
| `giraffe` | Blätter | `loc_playground` | tall feeding rack |
| `lion` | Fleisch | `loc_sun_rocks` | rocks |
| `snow_fox` | Beeren | `loc_ice_cream_kiosk` | shade, cool den |

Riddles and math per animal: CONT-MISSIONS (`specs/20-content/missions/start-missions.md`).

## Animal states

```
escaped ──(shown correct food)──▶ following ──(enters own enclosure)──▶ in_enclosure
   ▲                                  │
   └──────────(never: animals do not escape again)
```

- `escaped`: at its hiding place, plays `idle`/`eat`/`drink`, reacts to the player by looking.
- `following`: follows the player (GAME-RESCUE §6).
- `in_enclosure`: inside, plays idle/happy animations. Final state.

## Info board

Every enclosure has an **info board** (*Infotafel*) next to its sign. Its text (per reading
level and language, CONT-READING, CONT-L10N) contains:
1. the animal's name,
2. the **location riddle** for the hiding place chosen in this playthrough (GAME-RESCUE §2),
3. its **food**, using exactly the word printed on the matching food box label.

On `kiga` the board shows pictures (habitat, food) plus one word each; read-aloud on tap (Q-007).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| ANIM-001 | Given the animal data, then every animal has ≥ 1 correct food and ≥ 1 hiding place. | unit |
| ANIM-002 | Given an animal in `in_enclosure`, then no event can change its state. | unit |
| ANIM-003 | Given every info board text (all levels, all languages), then the food word equals the label text of a food box with that food. | unit |
| ANIM-004 | Given every hiding place in the data, then it references an existing location in the layout data (GAME-LAYOUT). | unit |
| ANIM-005 | Given seed S picks hiding place H for the zebras, then the zebra info board shows the location riddle for H (at the current reading level and language). | unit |

## Open questions

- Q-002 final list, Q-004 hippos, Q-005 elephant, Q-030 herd size, Q-036 goldfish transport.
- Q-043 Animation set per animal (hiding-place idles, reactions, koala/goldfish locomotion).
- Q-044 How hiding places are represented in the layout data.
