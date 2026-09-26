---
id: ART-ENVIRONMENT
title: Environment — mockups and models
aspect: art
module: environment
status: draft
depends_on: [ART-PIPELINE, ART-DIRECTION, GAME-WORLD, CONT-MISSIONS]
test_prefix: AENV
updated: 2026-09-26
---

# Environment — mockups and models

## Goal

Every area of the zoo has approved mockups before it is built, and is assembled from
reusable modular props.

## Mockups required (before modelling)

| Asset id | Area | Must show |
|---|---|---|
| `env_entrance` | Zoo entrance / start | gate, map board, first visitors |
| `env_enclosure_row` | Enclosure paths | paths, fences, enclosure signs, benches |
| `env_hippo` | Hippo enclosure | pool, stones, wooden hut (cf. `art/reference/ref-enclosure-buildings.jpg`) |
| `env_panda` | Panda enclosure | bamboo |
| `env_zebra` | Zebra enclosure | bushes, grass, leaves |
| `env_koala` | Koala enclosure | eucalyptus trees |
| `env_elephant` | Elephant enclosure | water, hay, logs, bridge (Q-005) |
| `env_goldfish` | Goldfish pond / aquarium | water plants |
| `env_monkey` | Monkey enclosure | climbing frame, hiding spots for the baby |
| `env_giraffe` | Giraffe enclosure | tall feeding rack |
| `env_lion` | Lion enclosure | rocks |
| `env_snow_fox` | Snow fox enclosure | shade, cool den |
| `env_food_storage` | Food storage | food boxes with labels (locked door only if Q-033 keeps the lock) |
| `env_pirate_ship` | Pirate ship (playground?) | monkey hiding place `loc_pirate_ship`: mast, sail, skull flag, treasure chest; key hiding spot if Q-033 keeps `quest_key` |

## Hiding places (must appear in a mockup)

Every hiding place of CONT-MISSIONS must be visible in at least one mockup, showing the
details its location riddles mention. Which mockup covers which place is assigned by the
`zoo-level-designer` (Q-044).

| Hiding place | Details the riddles rely on |
|---|---|
| `loc_river` | flowing water, bridge, ducks |
| `loc_pond` | still water, water lilies, frogs |
| `loc_cave` | dark, cool stone cave (echo) |
| `loc_tallest_tree` | clearly the tallest tree, entrance gate visible from the top |
| `loc_mud_pool` | brown mud, splashing |
| `loc_fountain` | stone basin, water jet, coins |
| `loc_pirate_ship` | see `env_pirate_ship` |
| `loc_playground` | slide, swings, sandpit, trees |
| `loc_sun_rocks` | big flat rocks in full sun |
| `loc_ice_cream_kiosk` | kiosk, freezer chest, cones |

## Modular props (modelled once, reused)

`fence_wood`, `fence_stone`, `path_tile`, `enclosure_sign`, `food_box`, `bench`, `tree`,
`bush`, `grass_tuft`, `rock`, `water_tile`, `key`, `info_board`.

## Behaviour

1. Each mockup's `layout.md` lists which modular props it uses; new props are added to the
   list above before modelling.
2. Enclosure signs and food box labels are **not** baked into textures — text is rendered
   by the game from i18n keys so they switch language (CONT-L10N).
3. Areas are sized so that walking from one enclosure to the next takes ≤ 10 s.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| AENV-001 | Given each area in the table, then an approved mockup exists (see APIPE-003). | asset |
| AENV-002 | Given each `layout.md`, then every prop it names exists in the modular props list. | asset |
| AENV-003 | Given the language is switched from `de` to `en`, then all enclosure signs show the English name without reloading the level. | e2e |
| AENV-004 | Given every hiding place id in CONT-MISSIONS, then at least one approved mockup's `layout.md` lists it. | asset |

## Open questions

- Q-006 One open world vs. separate areas.
- Q-017 Is the pirate ship a playground in the zoo, or a separate location?
- Q-033 Food storage locked? Q-044 Hiding places in layout and mockups.
