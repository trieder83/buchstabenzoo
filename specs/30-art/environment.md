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

### Built kits (scripted, `tools/blender/props/`)

Axes and orientation follow GAME-LAYOUT "Coordinate spaces" (Q-056): model east = +X,
model north = −Z (Blender +Y), origin on the ground; models are oriented by rotation about
+Y only, never mirrored.

| Kit | Asset ids | Size / placement |
|---|---|---|
| `kit_ground` | `path_tile_straight`, `path_tile_curve`, `path_tile_t`, `path_tile_cross`, `path_tile_end`, `plaza_tile`, `grass_tile`, `sand_tile`, `path_edge` | **1 m × 1 m tiles, one per grid cell** (the level grid is 1 m; level-1 paths are 3 cells wide). The path tile of a cell is chosen from which of its **4 neighbours** (N, E, S, W) are path cells, and rotated about +Y in 90° steps (mapping in `kit_ground.py`). `path_edge` is a 1 m strip. |
| `kit_fences` | `fence_wood`, `fence_wood_1m`, `fence_wood_corner`, `fence_wood_end`, `gate_wood`, `hedge`, `hedge_1m`, `hedge_corner`, `zoo_wall`, `zoo_wall_1m`, `zoo_wall_corner` | Straight pieces **2 m and 1 m** long (Q-057); corner pieces are L pieces with 1 m arms; fence 1.1 m high, hedge 3 m high × ~1 m thick, zoo wall 2.5 m high × 0.6 m (0.8 m cap). **One** `gate_wood` (the leaf, origin on the hinge axis), opened in-game by rotating it about +Y — there are no separate closed/open models. |

**Filling edges (Q-057):** straight runs of fence, hedge and wall are filled by the rule of
GAME-LAYOUT "Modular edges": 2 m segments from the run start, plus one 1 m segment at the
end when the length is odd; corner pieces where a run turns. Hedge/wall bands in the level
data are 1–2 cells deep; the models are placed as one row on the band's centre line
(proposal; joins Q-059).

Unique (non-modular) models needed for level 1: `entrance_arch`, `food_storage_building`,
`stone_arch_shelter` (zebra), `hut_wood` and `pool_tiled` (hippo), `panda_platform` and
`panda_shelter`, `rock_hill_cave`, `river_grate`.

## Behaviour

1. Each mockup's `layout.md` lists which modular props it uses; new props are added to the
   list above before modelling.
2. Enclosure signs and food box labels are **not** baked into textures — text is rendered
   by the game from i18n keys so they switch language (CONT-L10N).
3. Areas are sized so that walking from one enclosure to the next takes ≤ 10 s.
4. Straight edge pieces (`fence_wood`, `hedge`, `zoo_wall`) exist as 2 m and 1 m variants
   of the same look; ground tiles are 1 m (one per cell). Models use the world axes of
   GAME-LAYOUT (north = −Z) and are never mirrored (Q-056, Q-057).
5. **Living water** (user request 2026-09-26): all water is animated so the zoo feels
   lively — always in the comic style (flat colour bands, hard edges, bold shapes; no
   realistic reflections, refraction or noisy normal maps).
   - **River (flowing):** clearly visible flow along the river direction — scrolling
     light streaks/foam bands, small white foam at banks, bridge posts and rocks, gentle
     bobbing of ducks. Flow speed reads as "moving water" (≈ 0.5–1 m/s visual speed).
   - **Pond (still, but alive):** slow gentle ripples/rings and soft shimmer bands, lily
     pads and the frog bob slightly; no directional flow. The river must stay clearly
     "flowing" and the pond clearly "still" — this difference is a riddle clue (zebra vs.
     hippo, CONT-MISSIONS).
   - **Shoreline:** a thin animated foam/wave line where water meets the bank.
   - Animation is done in the renderer (shader time + UV/vertex motion, TECH-ARCH), not by
     animated meshes; it loops seamlessly, is independent of frame rate, and costs no
     extra draw calls per tile. The same motion must be visible from the 55° game camera at
     every zoom (10–20 m).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| AENV-001 | Given each area in the table (not `style_frame` and `env_level1_overview`, which are single images without a player view), then an approved mockup exists (see APIPE-003). | asset |
| AENV-002 | Given each `layout.md`, then every prop it names exists in the modular props list. | asset |
| AENV-003 | Given the language is switched from `de` to `en`, then all enclosure signs show the English name without reloading the level. | e2e |
| AENV-004 | Given every hiding place id in CONT-MISSIONS, then at least one approved mockup's `layout.md` lists it. | asset |
| AENV-005 | Given the exported `kit_fences` models, then `fence_wood`, `hedge` and `zoo_wall` each exist as a 2 m and a 1 m (`_1m`) straight piece with the same height and thickness, and `tools/blender/check_glb.py` passes for all of them (sizes, budget, Y-up, origin on the ground). | asset |
| AENV-006 | Given the exported `kit_ground` tiles, then every tile is 1 m × 1 m (one per grid cell) and `tools/blender/check_glb.py` passes. | asset |
| AENV-007 | Given the river and the pond on screen, when two screenshots are taken 0.5 s apart, then pixels of both water areas differ (water is animated), and the river's dominant motion vector points along the river direction while the pond has no dominant direction. | e2e |
| AENV-008 | Given the water shader, then the animation is a function of time only (same image for the same time regardless of frame rate) and loops seamlessly. | unit |
| AENV-009 | Given the style frame and an in-game screenshot of river and pond, then reviewers confirm the water motion looks cartoon-like (flat bands, hard edges) and river vs. pond read as flowing vs. still. | manual |
| AENV-010 | Given level 1 on a mid-range phone, then enabling water animation costs ≤ 1 ms GPU time per frame and no extra draw calls. | manual |

## Open questions

- Q-006 One open world vs. separate areas.
- Q-017 Is the pirate ship a playground in the zoo, or a separate location?
- Q-033 Food storage locked? Q-044 Hiding places in layout and mockups.
- Q-049 answered: high-angle game camera (GAME-PLAYER §2). Q-048 screen orientation. Q-052 FOV axis.
- Q-056 answered: axes (model north = −Z, never mirrored). Q-057 answered: 1 m segment variants, fill rule. Q-059 band joins, Q-060 fence/band placement, Q-061 front direction of props (open).
