---
id: ART-ENVIRONMENT
title: Environment — mockups and models
aspect: art
module: environment
status: draft
depends_on: [ART-PIPELINE, ART-DIRECTION, GAME-WORLD, CONT-MISSIONS]
test_prefix: AENV
updated: 2026-10-06
---

# Environment — mockups and models

**Contents:** Goal · Mockups required (before modelling) · Hiding places (must appear in a mockup) · Modular props (modelled once, reused) · Night art (GAME-NIGHT) · Behaviour · Test cases · Open questions


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
| `env_hippo` | Hippo enclosure | square tiled pool (no lilies/frogs — must not look like `loc_pond`), stones, wooden hut (cf. `art/props/kit_enclosure_buildings/sheet_zebra_hippo_v3.jpg`) |
| `env_panda` | Panda enclosure | cut bamboo on a feeding rack (no growing bamboo clumps — Q-081 answered, must not look like `loc_bamboo`), wooden platform and shelter — no stone cave (must not look like `loc_cave`) |
| `env_zebra` | Zebra enclosure | bushes, grass, leaves, stone-arch shelter — no water (riddle points to the river) |
| `env_koala` | Koala enclosure, level 2 (GAME-LEVEL-2, west) | eucalyptus trees of normal height, climbing trunk, small wooden shelter; the koala pair — **no** tree taller than the others, no blossoms, no tree house (riddle guards) |
| `env_elephant` | Elephant enclosure, level 2 (east) | tiled bathing pool `elephant_pool` with a ramp (like the hippo pool), hay rack, one boulder — **no** jet or coins in the water, no logs, no ball (riddle guards; Q-005 bridge not used) |
| `env_goldfish` | Goldfish pond enclosure, level 3 (east) | round pond `goldfish_pond` with a low stone rim, water plants, low wooden fence, flat stone step (the gate where the bowl is put down) — no waterfall, wheel or willow |
| `env_monkey` | Monkey enclosure, level 3 (north) | climbing frame of logs and ropes, hanging tyres, wooden monkey house, banana basket, hiding spots for the baby — no ship, carousel or trampoline |
| `env_giraffe` | Giraffe enclosure, level 2 (north) | tall feeding rack with leafy branches, giraffe house with a tall door — no tower, slide or train |
| `env_lion` | Lion enclosure, level 2 (south) | wooden sun deck with a straw roof, a big lying log, dry grass — **no flat rocks** (the rocks are the hiding place `loc_sun_rocks`), no stage, no chairs |
| `env_snow_fox` | Snow fox enclosure, level 3 (south) | wooden den with straw under two pine trees, light rocks, drinking bowl — no freezer, sprinkler or washing line |
| `env_food_storage` | Food storage | food boxes with labels (locked door only if Q-033 keeps the lock) |
| `env_pirate_ship` | Pirate ship (Q-017 proposal: climbing frame on the level-3 adventure playground) | covered by the mockup `loc_pirate_ship` below (no separate brief) |
| `style_frame` | Style frame (comic style, Q-010) | the zebra-enclosure scene in the comic style from the high game camera: `style_frame.png`; no `layout.md` (ART-PIPELINE §5) |
| `env_level1_overview` | Whole level 1 (GAME-LEVEL-1) | bird's-eye `overview.png` + orthographic `top_down.png` matching the level-1 ASCII map |
| `loc_river` | Zebra hiding place, level 1 | flowing river with rapids, wooden bridge, ducks, zebras drinking on the bank; fallen-tree barrier behind the bridge |
| `loc_pond` | Hippo hiding place, level 1 | round still pond, water lilies, frogs, reeds, wooden jetty, hippo with only eyes/ears above water in the west half near the north shore (FIX-056; the approved mockup shows it near the jetty — position only); no bridge, no ducks |
| `loc_cave` | Panda hiding place, level 1 | grey rock hill, dark cool cave mouth facing north, sleeping panda inside; "path under repair" barrier with zookeeper cart |
| `loc_meadow` | Zebra hiding place, level 1 (north-east, behind the bridge) | knee-high tall grass with red/yellow/white wildflowers and butterflies, big trees (`trees_ne`) behind, zebras grazing; the narrow trail and the river at the left edge |
| `loc_sand` | Zebra hiding place, level 1 (north, behind the panda enclosure) | dry yellow sand patch without grass, a few small rocks at the rim, zebras rolling/standing in the dust; panda enclosure fence at the bottom, north hedge at the top |
| `loc_mud` | Hippo hiding place, level 1 (north-west corner) | big brown mud puddle with glossy wet highlights, splashes and footprints, hippo half-sunk in the mud; trees behind, north hedge; must not look like water (no lilies, frogs, reeds, no blue) |
| `loc_shade` | Hippo hiding place, level 1 (west, at the zoo wall) | 2 m wide strip between the high zoo wall and big trees whose crowns overhang it, flat dark-green shade on dry grass, hippo dozing; no water, no mud |
| `loc_bamboo` | Panda hiding place, level 1 (south-west corner) | dense bamboo thicket (3 m) standing taller than the 2.5 m zoo wall behind it, panda sitting at its edge chewing bamboo; map board and plaza edge far right |
| `loc_leaves` | Panda hiding place, level 1 (very north-east corner between the hedges, FIX-056) | big raked heap of red, yellow and brown leaves at the edge of the trees, a rake leaning on a small one-seat bench, panda lying on its back in the leaves; sunny and colourful, not dark |
| `env_level2_overview` | Whole level 2 (GAME-LEVEL-2) | bird's-eye `overview.png` + orthographic `top_down.png` matching the level-2 ASCII map (also the GAME-MAP art) |
| `env_level3_overview` | Whole level 3 (GAME-LEVEL-3) | bird's-eye `overview.png` + orthographic `top_down.png` matching the level-3 ASCII map |
| `env_zookeeper_house` | Zookeeper house of level 3 (`zookeeper_house_3`) | closed house + roof cut-away with the big empty glass fish bowl on the table, bed, shelves; water tap with a small basin next to the door; food storage 3 next door |
| `env_zookeeper_house_1` | Zookeeper house of level 1 (`zookeeper_house_1`, west of the entrance plaza — GAME-LEVEL-1, GAME-NIGHT, GAME-CART) | closed house + roof cut-away: child-size bed with the blue star blanket, night table with bedside lamp, window, rug, toy chest, desk with one blank sheet (the "Math Fighter" note), key box with a 3-wheel lock outside by the door; plus a night view of the bedroom corner — no fish bowl, no bamboo at the house |
| `env_zookeeper_house_2` | Zookeeper house of level 2 (`zookeeper_house_2`, south-west corner on `path_l2_sw` — GAME-LEVEL-2, GAME-NIGHT; user request 2026-10-01: every bed indoors) | closed house + roof cut-away: same model and room as the level-1 house with the child-size bed, night table with bedside lamp, moon window, rug, toy chest; no desk, no key box |
| `env_night1_overview` | Whole night level 1 (GAME-LEVEL-NIGHT-1), layout-true (the approved `env_night_overview` is the mood image) | at night, camera yaw west: open moon door at the bottom, plaza with string lights, food hut, night house with three lit boards, lantern-lit loop around an old-tree grove, and all 9 night hiding places with their clues (twig heap, flowerpots with white flowers, mushroom ring, hill with a big stone, fir, windmill, firefly meadow, pond mirroring the moon, knothole tree); + orthographic `top_down.png` |
| `env_night2_overview` | Whole night level 2 / terrarium garden (GAME-LEVEL-NIGHT-2, proposal 2026-10-03), layout-true | night bird's-eye `overview.png` + `top_down.png` matching the night-2 ASCII map: plaza with string lights, food hut with fridge, terrarium house (glowing amber / violet / teal glass fronts), the nine riddle places (stone wall, pumpkins, upturned boat, lantern tree, palm, vine arch, stepping plates, fern glade, rain barrel), the lantern gate on the east edge. Mood: tropical, cosy, never dark-scary. **Brief not written yet** (art agent). |
| `env_terrarium_house` | Terrarium house `terrarium_house` of `night_2` | closed house + roof cut-away: warm-lit visitor hall, three glass-front terrariums (snake: warm rock + branch + heat lamp; chameleon: tall leafy branches + UV violet lamp; frog: big leaves + mossy log + mist), door on the street, boards outside; props from the modular list plus `terrarium_front`, `terrarium_frame`, `terrarium_lamp`, `terrarium_rock_warm`, `terrarium_branch`, `terrarium_leaf_big`, `terrarium_moss_log`, `terrarium_dish`, `mist_puff`. **Brief not written yet** (art agent; no concept images made here). |
| `env_garden` | Vegetable garden `garden_veg`, level 1 (north, between the panda enclosure and the river — GAME-LEVEL-1, GAME-GARDEN) | low picket fence with a small open gate, 2 m path, two carrot beds and two potato beds (clearly different from above), a picture stake sign per bed, empty wheelbarrow and watering can at the hedge, the girl pulling a carrot — dry soil (not `loc_mud`), no rake, butterflies, wildflowers or water spray (riddle guards) |
| `loc_treehouse` | Koala hiding place, level 2 (east wall, east of the music stage — FIX-056) | old oak with a wooden tree house (roof, round window) at 3.5 m and a rope ladder; the koala pair on the porch |
| `loc_tallest_tree` | Koala hiding place, level 2 (east) | a 12 m giant tree, twice as tall as all other trees, thick trunk; the koala pair at the very top |
| `loc_blossom_tree` | Koala hiding place, level 2 (north-east corner) | tree covered in pink blossoms, petals drifting onto the grass, bees; the koala pair in the crown |
| `loc_fountain` | Elephant hiding place, level 2 (south-west) | round stone basin with a water jet, coins on the bottom, the elephant drinking and showering |
| `loc_log_pile` | Elephant hiding place, level 2 (north-west) | neatly stacked thick tree trunks, sawdust on the grass, the elephant next to them |
| `loc_big_ball` | Elephant hiding place, level 2 (north) | giant red-and-white ball taller than the girl on the lawn, the elephant nudging it |
| `loc_lookout_tower` | Giraffe hiding place, level 2 (south-west) | wooden lookout tower with stairs and a roofed platform at 4 m, the giraffe's head level with the platform |
| `loc_train` | Giraffe hiding place, level 2 (south) | little zoo train (engine with chimney and bell, 2 open wagons) at a small station platform, the giraffe sniffing the chimney |
| `loc_playground` | Giraffe hiding place, level 2 (south-east corner) | slide and double swing (no sandpit), the giraffe towering over them |
| `loc_sun_rocks` | Lion hiding place, level 2 (north-west corner) | big flat light-grey rock slabs flush with the grass in full sun, no tree, no shade, the lion dozing |
| `loc_stage` | Lion hiding place, level 2 (north-east) | round wooden music stage with a pointed roof, drums and a xylophone, the lion roaring in front |
| `loc_deckchairs` | Lion hiding place, level 2 (north-east) | three striped deckchairs under a big sunshade, the lion sprawled next to them |
| `loc_pirate_ship` | Monkey hiding place, level 3 (south-east, adventure playground; = `env_pirate_ship`) | pirate-ship climbing frame on bark mulch: mast with crow's nest, white sail, black flag with a white paw print (no skull), treasure chest, rope ladder, **no slide**; the monkey in the crow's nest |
| `loc_carousel` | Monkey hiding place, level 3 (south-west) | small carousel with painted wooden horses under a striped round roof, the monkey riding backwards |
| `loc_trampoline` | Monkey hiding place, level 3 (south-west corner, by the end of the stream — FIX-056) | round ground-level trampoline (blue mat, red rim) flush with the lawn, the monkey mid-somersault |
| `loc_waterfall` | Goldfish hiding place, level 3 (north-west) | the stream falling 2.5 m from a rock ledge at the north wall, white foam pool, a goldfish flashing orange in the foam |
| `loc_water_wheel` | Goldfish hiding place, level 3 (west) | tiny wooden mill hut with a big wooden water wheel turning in the stream, glittering drops, the goldfish beside it |
| `loc_willow` | Goldfish hiding place, level 3 (south-west) | weeping willow whose long branches hang like a green curtain into the stream, the goldfish in the shade below |
| `loc_ice_cream_kiosk` | Snow fox hiding place, level 3 (north-east) | ice cream kiosk with a striped awning, a cone icon on the roof (no text), freezer chest with cold mist, the snow fox beside it |
| `loc_sprinkler` | Snow fox hiding place, level 3 (north-west) | lawn with a turning garden sprinkler, arcs of drops, a small rainbow, glossy wet grass, the snow fox enjoying the spray — must not look like the fountain |
| `loc_laundry` | Snow fox hiding place, level 3 (north-west, by the stream) | washing line with big white sheets and towels flapping, the white snow fox almost invisible between them |

## Hiding places (must appear in a mockup)

Every hiding place of CONT-MISSIONS must be visible in at least one mockup, showing the
details its location riddles mention. Which mockup covers which place is assigned by the
`zoo-level-designer` (Q-044). Level 1: `loc_river`, `loc_pond` and `loc_cave` each have their
own mockup (see table above), and so do the six further level-1 candidates `loc_meadow`, `loc_sand`,
`loc_mud`, `loc_shade`, `loc_bamboo`, `loc_leaves` (briefs in `art/environment/<id>/`). Levels 2
and 3: every one of the 21 candidates has its own mockup brief as well (table above,
GAME-LEVEL-2 / GAME-LEVEL-3). Every hiding place also needs a small **`kiga` picture** (the board
shows the picture of the chosen place next to its one word; picture id = hiding place id) —
cropped from the approved mockup.

| Hiding place | Details the riddles rely on |
|---|---|
| `loc_river` | flowing water, bridge, ducks |
| `loc_pond` | still water, water lilies, frogs |
| `loc_cave` | dark, cool stone cave (echo) |
| `loc_meadow` | tall grass, wildflowers, butterflies, big trees behind |
| `loc_sand` | dry yellow sand, no grass |
| `loc_mud` | brown wet mud, splashing (not water) |
| `loc_shade` | shade of big trees right at the zoo wall, dry grass |
| `loc_bamboo` | dense green bamboo taller than the zoo wall |
| `loc_leaves` | raked pile of red/yellow/brown leaves, rake, sunny |
| `loc_treehouse` | tree house with roof and window high up, rope ladder |
| `loc_tallest_tree` | clearly the tallest tree of the zoo (twice as tall as all others), thick trunk |
| `loc_blossom_tree` | pink blossoms, drifting petals, bees |
| `loc_fountain` | stone basin, water jet, coins |
| `loc_log_pile` | stacked tree trunks, sawdust |
| `loc_big_ball` | giant round red-and-white ball, bigger than a child |
| `loc_lookout_tower` | wooden tower, stairs, high platform with visitors |
| `loc_train` | little train: engine with chimney and bell, wagons, station |
| `loc_playground` | slide, swings (no sandpit) |
| `loc_sun_rocks` | big flat rocks in full sun, no shade |
| `loc_stage` | round stage with a pointed roof, drums, xylophone |
| `loc_deckchairs` | striped deckchairs, big sunshade |
| `loc_pirate_ship` | mast, sail, black flag, treasure chest (see `env_pirate_ship`) |
| `loc_carousel` | carousel, wooden horses, striped roof, turning |
| `loc_trampoline` | round blue springy mat in the lawn |
| `loc_waterfall` | water falling from a rock ledge, white foam |
| `loc_water_wheel` | wooden water wheel turning in the stream, little mill hut |
| `loc_willow` | weeping willow, branches hanging into the water |
| `loc_ice_cream_kiosk` | kiosk, freezer chest, cones |
| `loc_sprinkler` | turning sprinkler, cold drops, rainbow, wet grass |
| `loc_laundry` | white sheets and towels on a washing line |

## Modular props (modelled once, reused)

`fence_wood`, `fence_stone`, `path_tile`, `enclosure_sign`, `food_box`, `bench`, `tree`,
`bush`, `grass_tuft`, `rock`, `water_tile`, `key`, `info_board`.

Added for level 1 (proposal, zoo-level-designer — review together with the level-1
mockups): `hedge` (tall, 3 m), `water_river_*` tiles of `kit_water` (river; the flow is drawn by the water shader, TECH-WATER —
formerly `water_tile_flowing`), `bridge_wood`, `jetty_wood`, `lily_pad`, `reed` (`duck`, `frog`: removed from the kit — animated ambient animals since M6, ART-ANIMALS "Ambient animals", Q-122 answered), `bamboo`, `map_board`, `gate_wood`,
`road_block`, `repair_sign` (blank, shovel icon), `zookeeper_cart`, `traffic_cone`,
`fallen_tree`, `flower_bed`.

Golf cart props (GAME-CART, 2026-10-06): `golf_cart` (drivable, **red body, white roof with red stripes**; not the static `zookeeper_cart` repair prop), `parking_sign` (post with a P and a cart icon), `key_box` / `key_box_open`, `cart_key`.

Added for the level-1 candidate hiding places (proposal, zoo-level-designer, 2026-09-26 —
only where no existing prop fits): `wildflowers` (small tuft with coloured flower heads,
≤ 80 tris, scattered in tall grass — `flower_bed` is a formal bed and does not fit a meadow),
`butterfly` (≤ 20 tris, bobbing like the ducks; M6: a built-in 12-triangle two-wing mesh in the renderer, GAME-AMBIENT), `mud_tile` (1 m ground tile of glossy brown
mud with an edge variant, for `kit_ground`), `shade_decal` (flat dark-green ground decal for
tree shade — Q-080 answered: a flat ground decal, not a renderer feature), `leaf_pile` (lumpy heap of red/yellow/brown
leaves, ≤ 300 tris) and `rake` (≤ 60 tris). Reused: `grass_tuft` scaled ×3 for tall grass,
`sand_tile` (kit_ground) for the sand patch, `bamboo` for the thicket, `zoo_wall` (kit_fences), `rock`, `tree_round`.

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

Added for levels 2 and 3 (proposal, zoo-level-designer, 2026-09-26 — only where no existing
prop fits): `petal_decal` (pink petals on grass), `flat_rock_slab` (flush grey slab, 3 sizes),
`bark_mulch_tile` (1 m ground tile, `kit_ground`), `trampoline_ground` (flush round mat),
`sprinkler` (turning lawn sprinkler, animated spray with a rainbow band in the renderer),
`water_tap` (tap with a small basin, on a wall), `fish_bowl` (big glass bowl, empty / filled /
with fish — carried with `socket_carry`), `washing_line` (two posts, line, sheets swaying),
`bee` (≤ 20 tris, like `butterfly`), `construction_fence` (striped panel, 2 m and 1 m) and
`digger` (small toy-like digger) for `barrier_l2_construction`, `stream` tiles (reuse the
`water_river_*` tiles, 3 m wide). Reused: `pool_tiled` (elephant pool), `food_storage_building`
(food storages 2 and 3), `map_board`, `hedge`, `zoo_wall`, `tree_round`, `bush`, `rock`, `bench`.

Unique (non-modular) models needed for levels 2 and 3: `treehouse_oak`, `tree_giant` (12 m),
`tree_blossom`, `fountain_stone`, `log_pile`, `play_ball`, `lookout_tower`, `zoo_train`
(engine + 2 wagons + track piece + platform), `slide` + `swings` (built 2026-10-03 as `playground_slide` / `playground_swings`), `music_stage`,
`deckchairs_sunshade`, `koala_shelter`, `elephant_house`, `giraffe_house` + `giraffe_feeding_rack`,
`lion_sun_deck`, `waterfall_ledge`, `mill_hut_wheel` (animated wheel), `willow`, `pirate_ship`,
`carousel` (turning; built 2026-10-03 as `carousel` with a `rotor` node), `ice_cream_kiosk` (built 2026-10-03, freezer chest included, `kit_landmarks_play`), `zookeeper_house` (closed + cut-away,
concept in `kit_buildings`), `pond_stone_rim`, `monkey_climbing_frame`, `snow_fox_den`, `eucalyptus_tree`, `hay_rack`,
`monkey_house`, `pine_tree`.

Added for the vegetable garden (proposal, zoo-level-designer, 2026-09-26 — GAME-GARDEN §9, one concept sheet first): `garden_fence` (low picket fence 0.8 m, 2 m + 1 m pieces), `garden_gate` (1 m leaf, used as a pair), `garden_bed` (raised wooden frame 0.8 × 2.9 m with dry soil, modular 1 × 3 m), `carrot_plant` and `potato_plant` (3 growth stages + empty soil), `garden_sign` (small stake sign, picture of the vegetable; word rendered by the game), `wheelbarrow`, `watering_can`. Reused: `path_tile`, `hedge`.

Unique (non-modular) models needed for level 1: `entrance_arch`, `food_storage_building`,
`stone_arch_shelter` (zebra), `hut_wood` and `pool_tiled` (hippo), `panda_platform` and
`panda_shelter`, `rock_hill_cave`, `river_grate`.

**Animal houses (GAME-HOUSE, 2026-10-04):** `stone_arch_shelter`, `hut_wood`, `panda_shelter`, `koala_shelter`, `elephant_house`, `giraffe_house`, `monkey_house` are built to the footprint, interior and **doorway** of the `animal_house` data (walls on the footprint outline, doorway exactly the `door` cells wide, no door leaf, clear height `door_height_m` — giraffe house >= 5.0 m, ridge about 7 m; footprints and doors: GAME-HOUSE "Houses per level"). Briefs must state doorway width and height; a placeholder box with a door cut-out is used until the model exists.

Concept sheets (2026-09-26, in review — `art/props/`): `kit_buildings` (`entrance_arch`,
`zookeeper_house` closed + roof-removed cutaway), `food_storage_building` (closed + cutaway),
`kit_enclosure_buildings` (`stone_arch_shelter`, `hut_wood` + cutaway, `pool_tiled`,
`panda_shelter`, `panda_platform`, `bamboo_feeding_rack`), `rock_hill_cave` (closed +
cutaway), `kit_furniture` (`bench`, `feeding_trough` — dropped 2026-10-01, no trough, Q-248). New ids proposed there, not yet placed in `level-1.toml`: `zookeeper_house`,
`bamboo_feeding_rack`.

## Night art (GAME-NIGHT)

Plan and per-group decisions: `art/night/README.md` (proposal 2026-09-26). Night is a
**renderer mode** (GAME-NIGHT §10) — existing models get **no night copies**; the renderer
tints them with blue moonlight and adds lamp point lights and emissive areas.

- **(a) no new art (tinted):** ground and paths, fences/hedges/walls, enclosure signs, food
  boxes, nature, barriers, player and animals' bodies, UI panels (look as by day, NIGHT-005).
- **(b) emissive parts / night state on existing models:** `info_board` and `map_board` get a
  `board_lamp` (GAME-NIGHT rule 5); buildings (`zookeeper_house`, `food_storage_building`,
  `entrance_arch`, `hut_wood`, shelters) get lit windows (`*_glow` material slots) and a
  `wall_lamp`; water shows reflected stars and lantern reflections (shader); every animal has
  an `eye_glow` slot for eyeshine (NIGHT-006, never red) and a `sleep` pose; ducks sleep,
  butterflies/bees hide.
- **(c) night-only new assets:** `lantern_post`, `string_lights`, `hand_lantern`,
  `board_lamp`, `wall_lamp`, `firefly`, `sky_moon` / `sky_stars` and the `moon_door`
  (closed / opening / open) — `art/props/kit_night`; `bed`, `night_table`, `bedside_lamp`,
  `window_moon`, `rug_round`, `toy_chest` — `art/props/kit_bedroom`; `night_house` —
  `art/environment/env_night_house`; night level mood — `art/environment/env_night_overview`;
  night animals — `art/animals/*`; choice icons `icon_sleep`, `icon_moon_door` (UI).
- **Emissive convention:** glowing parts are separate `*_glow` material slots (pale cream by
  day, emissive at night); each lamp model has a `light` empty (position, radius) for its
  point light. Night colours (proposal): lamp glow `#FFD66B`, light pools `#FFC46E`, windows
  `#FFC857`, moon `#FFF4C9`, eyeshine `#E6F7A0`, darkest night shadow not below `#2B3566`.
- **Prompts:** night briefs copy the STYLE block verbatim and add the "NIGHT LIGHTING"
  paragraph of `art/night/README.md` right after it (Q-113).
- Open: Q-113 (night style block), Q-114 (light-pool fallback), Q-115 (fireflies), Q-116
  (night-house red light), Q-117 (hand lantern), Q-118 (number/placement of lanterns).

## Behaviour

1. Each mockup's `layout.md` lists which modular props it uses; new props are added to the
   list above before modelling.
2. Enclosure signs and food box labels are **not** baked into textures — text is rendered
   by the game from i18n keys so they switch language (CONT-L10N).
3. Areas are sized so that walking from one enclosure to the next takes ≤ 10 s.
4. Straight edge pieces (`fence_wood`, `hedge`, `zoo_wall`) exist as 2 m and 1 m variants
   of the same look; ground tiles are 1 m (one per cell). Models use the world axes of
   GAME-LAYOUT (north = −Z) and are never mirrored (Q-056, Q-057).
6. **Enclosure signs show the animal** (user request 2026-09-26, answers Q-064 in part):
   the big enclosure sign shows a large, solid dark animal **silhouette** of its animal on
   the cream panel, clearly readable from outside the enclosure (from the path and from the
   55° game camera at every zoom), at every reading level. The silhouette is a flat decal
   (one image per animal, e.g. `assets/textures/signs/silhouette_<animal>.png`, drawn in the
   comic style) applied by the renderer to the sign panel; no text on the sign for now.
7. **"Futter" sign at the food storage** (user request 2026-09-26): a wooden sign board on
   the storage wall directly above the row of food boxes shows the word from Fluent key
   `sign-food-storage` (de **Futter**, en *Food*) in large bold comic lettering, readable
   from the game camera. Text is rendered by the game (never baked into a model), so it
   switches with the language.
   *PoC implementation notes (M4b):* silhouettes are generated by
   `tools/textures/sign_silhouettes.py` (source of truth; 384 × 256 PNG, zebra with stripe
   cut-outs so it does not read as a horse) and drawn as a 1.02 × 0.68 m decal on the
   `sign_panel` face (the other animals' silhouettes are derived from their concept side
   views by the same script); an enclosure whose silhouette file is missing keeps a blank
   panel. The "Futter" board is a 3.4 × 1.2 × 0.08 m wooden board (placeholder box)
   on the south facade, bottom **2.3 m** (above the 2.1 m door opening of the `food_storage`
   model, in its gable — README_night open point 2, user request 2026-09-27), centred over
   the box row; its text is a generic
   **text texture**: zoo-core lists text decals (Fluent key + pixel size), the host draws the
   current-language string into an offscreen 2D canvas (system bold rounded sans, dark on
   cream with an outline) and hands the RGBA bytes to Rust, which uploads them; it is redrawn
   on every language change. Later signs reuse the same path.
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
   - *Implemented (M6):* TECH-WATER (water field + water shader, obstacle foam, bobbing) and
     GAME-AMBIENT (animated ducks, ducklings, frogs, butterflies); review shots
     `art/environment/poc/screenshot_poc_water_{river,pond,stream}.png` and
     `river_loop.gif` (catalog item `water_ingame`).

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
| AENV-011 | Given the level with the zebra enclosure, then its enclosure sign panel shows the zebra silhouette decal, facing the path (visible in a screenshot from the default camera on the path in front of the gate). | e2e |
| AENV-012 | Given the food storage, then a sign above the food boxes shows the `sign-food-storage` text of the current language ("Futter" in `de`), readable from the default camera (letter height ≥ 3 % of the viewport height). | e2e |
| AENV-013 | Given the exported `kit_landmarks_play` models `ice_cream_kiosk`, `carousel`, `playground_slide` and `playground_swings`, then each loads, has ≤ 600 triangles, stands on y = 0, fits its level rect (kiosk 4 × 3, carousel 4 × 4, slide 2 × 3, swings 4 × 2; slide ≤ 1.9 m, swings ≤ 2.3 m high) and the carousel has a `rotor` node with its pivot at the centre; `check_glb.py` passes. | asset |
| AENV-014 | Given the zoo scene, then the kinds `carousel`, `kiosk` (building, counter-style, not enterable), `slide` and `swings` are drawn by `carousel`, `ice_cream_kiosk`, `playground_slide` and `playground_swings` at their rect centres (front south, one each) with a placeholder fallback, and their rect cells stay solid (collision unchanged, LAYOUT-017). | unit |
| AENV-015 | Given level 3 unlocked, when the player looks at the kiosk, the carousel and the slide + swings, then the models are on screen without console errors and two screenshots 2 s of game time apart show a turned carousel (`rotor` spins). | e2e |
| AENV-016 | Given the exported `kit_landmarks_l2` models `zoo_train` (≤ 2200 triangles, ≤ 2.5 m, fits 8 × 2, nodes `wheel_e0..e2`, `wheel_w1a/w1b/w2a/w2b`, `smoke`) and `blossom_tree` (≤ 1800 triangles, ≤ 6.4 m, nodes `petals`, `bees`), then each loads and stands on y = 0; and the zoo scene draws them for `train_se` / `tree_blossom_ne` at the rect centre with a placeholder fallback (AENV-014 list); `check_glb.py` passes. | asset |

## Open questions

- Q-006 One open world vs. separate areas.
- Q-017 Is the pirate ship a playground in the zoo, or a separate location?
- Q-033 Food storage locked? Q-044 Hiding places in layout and mockups. Q-080 (answered) scenery data, Q-081 (answered) bamboo in the panda enclosure. Q-098 `hut_wood` area of the hippo enclosure (`kind = "hut"`). Q-099 remaining invisible walls (`map_board`, fallen tree).
- Q-049 answered: high-angle game camera (GAME-PLAYER §2). Q-048 screen orientation. Q-052 FOV axis.
- Q-056 answered: axes (model north = −Z, never mirrored). Q-057 answered: 1 m segment variants, fill rule. Q-059 band joins, Q-060 fence/band placement, Q-061 front direction of props (open).
- Q-147 `string_lights` stretched to spans ≤ 6 m; Q-148 answered (entrance arch board: de *Buchstaben Zoo*, en *Letter Zoo*); Q-149 `rock_hill` height vs. `loc_hilltop` perch; Q-151 `food_hut` "Futter" board; Q-152 shelter prop over `bed_l2`.
- Q-153 `food_storage` (model) vs. `food_storage_building` (id here), kind `buildings`. Q-154 models not listed here yet (`door_wood`, `glass_door`, `turnstile`, `string_post`, `food_hut`, bedroom items, `kit_landmarks`, garden items).
