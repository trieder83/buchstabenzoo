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

Camera for all mockups (high-angle game camera decided 2026-09-26, Q-049): view like a
zoo-park simulation game — `overview.png` at ≈ 60–65° pitch with the whole area in frame,
`player_view.png` as the in-game follow camera of GAME-PLAYER §2 (≈ 55° pitch, default
≈ 14 m from the player, narrow FOV; ART-PIPELINE §4). Enclosure signs and info boards are tilted back towards the camera.

| Asset id | Area | Must show |
|---|---|---|
| `env_entrance` | Zoo entrance / start | gate, map board, first visitors |
| `env_enclosure_row` | Enclosure paths | paths, fences, enclosure signs, benches |
| `env_hippo` | Hippo enclosure | square tiled pool (no lilies/frogs — must not look like `loc_pond`), stones, wooden hut (cf. `art/reference/ref-enclosure-buildings.jpg`) |
| `env_panda` | Panda enclosure | bamboo, wooden platform and shelter — no stone cave (must not look like `loc_cave`) |
| `env_zebra` | Zebra enclosure | bushes, grass, leaves, stone-arch shelter — no water (riddle points to the river) |
| `env_koala` | Koala enclosure | eucalyptus trees |
| `env_elephant` | Elephant enclosure | water, hay, logs, bridge (Q-005) |
| `env_goldfish` | Goldfish pond / aquarium | water plants |
| `env_monkey` | Monkey enclosure | climbing frame, hiding spots for the baby |
| `env_giraffe` | Giraffe enclosure | tall feeding rack |
| `env_lion` | Lion enclosure | rocks |
| `env_snow_fox` | Snow fox enclosure | shade, cool den |
| `env_food_storage` | Food storage | food boxes with labels (locked door only if Q-033 keeps the lock) |
| `env_pirate_ship` | Pirate ship (playground?) | monkey hiding place `loc_pirate_ship`: mast, sail, skull flag, treasure chest; key hiding spot if Q-033 keeps `quest_key` |
| `style_frame` | Style frame (comic style, Q-010) | the zebra-enclosure scene in the comic style from the high game camera: `style_frame.png`; no `layout.md` (ART-PIPELINE §5) |
| `env_level1_overview` | Whole level 1 (GAME-LEVEL-1) | bird's-eye `overview.png` + orthographic `top_down.png` matching the level-1 ASCII map |
| `loc_river` | Zebra hiding place, level 1 | flowing river with rapids, wooden bridge, ducks, zebras drinking on the bank; fallen-tree barrier behind the bridge |
| `loc_pond` | Hippo hiding place, level 1 | round still pond, water lilies, frogs, reeds, wooden jetty, hippo with only eyes/ears above water; no bridge, no ducks |
| `loc_cave` | Panda hiding place, level 1 | grey rock hill, dark cool cave mouth facing north, sleeping panda inside; "path under repair" barrier with zookeeper cart |

## Hiding places (must appear in a mockup)

Every hiding place of CONT-MISSIONS must be visible in at least one mockup, showing the
details its location riddles mention. Which mockup covers which place is assigned by the
`zoo-level-designer` (Q-044). Level 1: `loc_river`, `loc_pond` and `loc_cave` each have their
own mockup (see table above); the others are assigned with their level.

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

Added for level 1 (proposal, zoo-level-designer — review together with the level-1
mockups): `hedge` (tall, 3 m), `water_tile_flowing` (river, moving texture), `bridge_wood`,
`jetty_wood`, `lily_pad`, `reed`, `duck`, `frog`, `bamboo`, `map_board`, `gate_wood`,
`road_block`, `repair_sign` (blank, shovel icon), `zookeeper_cart`, `traffic_cone`,
`fallen_tree`, `flower_bed`.

Unique (non-modular) models needed for level 1: `entrance_arch`, `food_storage_building`,
`stone_arch_shelter` (zebra), `hut_wood` and `pool_tiled` (hippo), `panda_platform` and
`panda_shelter`, `rock_hill_cave`, `river_grate`.

## Behaviour

1. Each mockup's `layout.md` lists which modular props it uses; new props are added to the
   list above before modelling.
2. Enclosure signs and food box labels are **not** baked into textures — text is rendered
   by the game from i18n keys so they switch language (CONT-L10N).
3. Areas are sized so that walking from one enclosure to the next takes ≤ 10 s.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| AENV-001 | Given each area in the table (not `style_frame` and `env_level1_overview`, which are single images without a player view), then an approved mockup exists (see APIPE-003). | asset |
| AENV-002 | Given each `layout.md`, then every prop it names exists in the modular props list. | asset |
| AENV-003 | Given the language is switched from `de` to `en`, then all enclosure signs show the English name without reloading the level. | e2e |
| AENV-004 | Given every hiding place id in CONT-MISSIONS, then at least one approved mockup's `layout.md` lists it. | asset |

## Open questions

- Q-006 One open world vs. separate areas.
- Q-017 Is the pirate ship a playground in the zoo, or a separate location?
- Q-033 Food storage locked? Q-044 Hiding places in layout and mockups.
- Q-049 answered: high-angle game camera (GAME-PLAYER §2). Q-048 screen orientation. Q-052 FOV axis.
