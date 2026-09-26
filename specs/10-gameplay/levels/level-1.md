---
id: GAME-LEVEL-1
title: Level 1 — entrance, zebra, hippo, panda
aspect: gameplay
module: levels
status: draft
depends_on: [GAME-LAYOUT, GAME-RESCUE, CONT-MISSIONS, GAME-PLAYER]
test_prefix: LAYOUT-L1
updated: 2026-09-26
---

# Level 1 — entrance, zebra, hippo, panda

Owned by the `zoo-level-designer` agent. Layout data: `assets/levels/level-1.toml`
(level id `level_1`). Coordinate system, element types and general rules: GAME-LAYOUT.
Look of each area: ART-ENVIRONMENT and the mockup briefs in `art/environment/`.

## Goal

A small, quickly buildable first level that teaches the whole rescue loop (GAME-RESCUE)
three times:

| Mission | Enclosure | Food box | Candidate hiding places (one is picked per playthrough) | What the riddles rely on (CONT-MISSIONS) |
|---|---|---|---|---|
| `zebra` | `enc_zebra` (west) | Gras / grass | `loc_river` (north-east), `loc_meadow` (east, behind the bridge), `loc_sand` (north) | flowing water, bridge, ducks · tall grass, wildflowers, butterflies · dry yellow sand, no grass |
| `hippo` | `enc_hippo` (east) | Melonen / melons | `loc_pond` (west), `loc_mud` (north-west corner), `loc_shade` (west, at the zoo wall) | still water, water lilies, frogs · brown mud, wet, splashing · shade under big trees, zoo wall, dry grass |
| `panda` | `enc_panda` (north) | Bambus / bamboo | `loc_cave` (south-east), `loc_bamboo` (south-west corner), `loc_leaves` (north-east corner) | dark, cool, stone, echo · tall green bamboo thicket taller than the wall · red/yellow leaf pile, rake |

Plus the zoo entrance (spawn) and the food storage. Everything else of the zoo is closed off
by child-friendly barriers.

**Discovery (user decision 2026-09-26, GAME-RESCUE §1):** every animal has 3 candidate
hiding places; one per animal is picked per playthrough with the seeded RNG. Each animal's
candidates all lie on the far side of the map from its own enclosure (zebra west → east and
north; hippo east → west; panda north → south and north-east corners), so the ring design
below holds for every candidate. In level 1 **all 27 combinations** keep the chosen places
≥ 12 m apart (RESC-014), so the pick needs no retries here (see "Hiding places").

**Design idea: a ring around a hidden middle.** One ring path runs around a central block
(food storage + tall hedges + a dense tree grove). Every enclosure sits on the outside of
the ring, and its animal hides on the **opposite** side of the central block — so the
child cannot see the animal from its own enclosure and has to read the riddle. River and
pond lie on opposite sides of the ring and look clearly different (long, flowing, bridge,
ducks vs. round, still, lilies, frogs).

**Decided:** grass is walkable, slower than paths (Q-046, user 2026-09-26). Every walkable
cell has a `surface`: `path` (cells of `path` elements, incl. bridge, jetty, cave floor) or
`grass` (all other walkable cells, `.` on the map).

## Proposals used in this level (not yet decided)

| Topic | Proposal used here | Question |
|---|---|---|
| Food storage lock | Unlocked from the start, no `quest_key` in level 1. | Q-033 |
| Hiding places in layout data | New element type `hiding_place` (overlay rectangle, not solid, with `animal`, `animal_spot`, `features`); GAME-LAYOUT marks it as proposal. | Q-044 |
| Candidate hiding places and scenery | All 9 candidates are listed in `[[hiding_place]]` (with `wander_radius_m`, `wander_on`, `features`, `scenery`, `pose`); non-solid ground dressing (tall grass, sand, mud, tree shade, leaf pile) in `[[scenery]]`. The three `[[element]] type = "hiding_place"` entries stay as legacy mirrors until zoo-core reads `[[hiding_place]]`. | Q-080 |
| Growing bamboo | Growing bamboo stands only at `loc_bamboo`; the panda enclosure shows cut bamboo on a feeding rack instead of live clumps, so the `loc_bamboo` riddle cannot point at the own enclosure. | Q-081 |
| Picking hiding places | Uniform seeded pick per animal, re-drawn on a conflict (see "Hiding places"); no memory of earlier playthroughs. | Q-082 |
| Sight test data | Solid elements carry `blocks_view` (and `height_m` for the mockups). | Q-044 |
| Barrier unlock | `barrier_ne_tree` opens when all three level-1 animals are home; the other two barriers belong to later levels. | Q-022, Q-023 |
| Food boxes in level 1 | All 10 food boxes stand in the storage (natural distractors). | Q-047 |
| Food box positions (PoC) | The storage interior is not modelled yet: the 10 boxes stand in a row in front of the storage's south facade (box centres z = 10.66, x = −3.6 … 3.6 every 0.8 m, labels facing south), on the north row of `path_ring_s`. Order west → east: melons, hay, bananas, bamboo, **grass**, meat, leaves, fish food, berries, eucalyptus. Data: `[[food_box]]` in `level-1.toml`. | Q-065 |
| Walking speed | 1.75 m/s on paths; grass 0.56 × = 0.98 m/s (GAME-PLAYER §6, user decision 2026-09-26). | Q-024 |
| Panda spot and cave view | Panda lies near the cave mouth so its head is visible from the high camera (see "High-angle camera" below). | — (level design) |
| Hippo pool data | `hippo_pool` is an `[[enclosure_feature]]` (not an element) at x 11–18, z 15–21 with a west ramp; `enc_hippo.home_wander_on = ["grass", "water"]`; water and grass connect only over the ramp; the hippo picks a water target 7 of 10 times (see "Hippo enclosure pool"). | Q-085 |
| Tree areas | `grove_center` `dense` with a bush border; `trees_nw`, `trees_ne` `sparse` with explicit tree/bush positions (see "Woods"); wander areas of hiding places clipped to their `rect`. | Q-085 |
| Collision footprints, invisible walls | Footprint values and fixes of GAME-LAYOUT "Collision footprints"; exceptions for this level in "Woods" and "Collision and billboards". | Q-087 |
| Enclosure signs | The sign in front of each gate becomes a gate arch (panel bottom ≥ 2.1 m) so the player never passes through its panel. | Q-086 |

## Spawn and camera

- Spawn cell **(0, 2)** on `path_plaza`, 2 m inside the entrance gate.
- Facing **+Z (north)** in level coordinates (= world −Z, GAME-LAYOUT "Coordinate spaces", Q-056); the camera starts south of the player looking north — the
  high-angle ≈ 55° follow camera of GAME-PLAYER §2 (Q-049 answered), default distance.
- First view: the entrance gate below the player, the food storage straight ahead (8 m), the zebra enclosure fence on the
  left, the grey rock hill on the right (its cave mouth faces north, away from the spawn),
  the map board to the left of the plaza.

## Map

Scale **1 character = 1 m** (one grid cell). North (+Z) is up; row labels are `z`, the
x axis is labelled below. Generated from `assets/levels/level-1.toml`.

```
  47 ##%%%%%%%%%%%%%XXX%%%%%%%%%%%%%%%%~~~%%%%%%%%%%%
  46 ##%%%%%%%%%%%%%XXX%%%%%%%%%%%%%%%%~~~%%%%%%%%%%%
  45 ##..mmmmmmm....===.....sssssss....~~~.........%%
  44 ##..mmm6mmm....===.....sss5sss....~~~..ll9l...%%
  43 ##..mmmmmmm....===.....sssssss....~~~..llll...%%
  42 ##..mmmmmmm....===.....sssssss....~~~==::::::.%%
  41 ##..::*::*::...===pppppppppppp....~~~==:t::t:.%%
  40 ##..::::::::...===pppppppppppp....~~~==::::::.%%
  39 ##,,::::::::...===pppppppppppp....~~~==::::::.%%
  38 ##,,:t::t::t...===pppppppppppp....~~~==::t::t.%%
  37 ##,,::::::::...===pppppppppppp....~~~==::::::.%%
  36 ##7,::t::t::...===pppppppppppp....~~~==::::::.%%
  35 ##,,::::::::...===pppppppppppp....~~~==:t::t:.%%
  34 ##,,::::::::...===pppppppppppp....~~~==:::::*.%%
  33 ##,,:t::t::t...===pppppppppppp....~~~=="""""""%%
  32 ##..::::::::...===pppppggppppp..1.~~~==""4""""%%
  31 ##.............===................~~~=="""""""%%
  30 ##.............===..i...........==HHH=========XX
  29 ##..............==================HHH=========XX
  28 ##..............==================HHH=========XX
  27 ##...oooooooo...================..~~~.........%%
  26 ##...oooooooobb.===TTTTTTTTTT===..~~~~~~~~~~~~~~
  25 ##...oooooooo...===TTTTTTTTTT===..~~~~~~~~~~~~~~
  24 ##...oooooooo...===TTTTTTTTTT===..~~~~~~~~~~~~~~
  23 ##...oooooooojjj===TTTTTTTTTT===..............%%
  22 ##...oooooo2ojjj===TTTTTTTTTT===%hhhhhhhhhhh..%%
  21 ##...oooooooo...===TTTTTTTTTT===%hhuuuuuuuuh..%%
  20 ##...oooooooo...===TTTTTTTTTT===%hhuuuuuuuuh..%%
  19 ##..............===TTTTTTTTTT===%hhuuuuuuuuh..%%
  18 ##..zzzzzzzzzzz.===TTTTTTTTTT===.hhruuuuuuuh..%%
  17 ##..zzzzzzzzzzz.===TTTTTTTTTT===ihhruuuuuuuh..%%
  16 ##..zzzzzzzzzzz.===%FFFFFFFF%===.ghruuuuuuuh..%%
  15 ##..zzzzzzzzzzz.===%FFFFFFFF%===.ghuuuuuuuuh..%%
  14 ##..zzzzzzzzzzzi===%FFFFFFFF%===%hhhhhhhhhhh..%%
  13 ##..zzzzzzzzzzg.===%FFFFFFFF%===%hhhhhhhhhhh..%%
  12 ##..zzzzzzzzzzg.===%FFFFFFFF%===%hhhhhhhhhhh..%%
  11 ##..zzzzzzzzzzz.===%FFFFDFFF%===%hhhhhhhhhhh..%%
  10 ##..zzzzzzzzzzz.==============================XX
   9 ##..zzzzzzzzzzz.==============================XX
   8 ##..zzzzzzzzzzz.==============================XX
   7 ##..zzzzzzzzzzz....==========....^ccc^^^^^^^^^%%
   6 ##.................==========....^ccc^^^^^^^^^%%
   5 ##.................==========....^3cc^^^^^^^^^%%
   4 ##...............M.==========.bb.^^^^^^^^^^^^^%%
   3 ##...............M.==========....^^^^^^^^^^^^^%%
   2 ##YYYY8............=====S====....^^^^^^^^^^^^^%%
   1 ##YYYY.............==========....^^^^^^^^^^^^^%%
   0 ##YYYY.............==========....^^^^^^^^^^^^^%%
  -1 #####################EEEEEE#####################
  -2 #####################EEEEEE#####################
     |...|...|...|...|...|...|...|...|...|...|...|...|
 x:  -24     -16     -8      0       8       16      24
```

| Char | Meaning | Char | Meaning |
|---|---|---|---|
| `#` | outer zoo wall (`boundary`) | `E` | entrance gate (`building`) |
| `%` | tall hedge (`decoration`, solid, blocks view) | `X` | barrier |
| `=` | path | `H` | bridge (path) |
| `j` | wooden jetty (path) | `c` | cave floor (path, under the rock roof) |
| `~` | river — flowing water (`landmark`) | `o` | pond — still water (`landmark`) |
| `^` | rock hill (`landmark`) | `T` | dense tree grove `grove_center` (`decoration`, `dense`, solid) |
| `:` | open wood `trees_nw` / `trees_ne` (`sparse`: walkable grass between the trees) | `t` `*` | cell of a tree trunk / bush in an open wood (only the trunk/bush collider is solid) |
| `u` | hippo pool water `hippo_pool` (inside `enc_hippo`, not walkable for the player) | `r` | hippo pool entry ramp |
| `F` / `D` | food storage / its door | `M` | map board (`landmark`) |
| `z` `h` `p` | enclosure zebra / hippo / panda | `g` | enclosure gate (with enclosure sign) |
| `i` | info board | `b` | bench |
| `S` | spawn | `Y` | bamboo thicket `bamboo_sw` (`decoration`, solid) |
| `1` `4` `5` | animal spot of zebra candidates `loc_river`, `loc_meadow`, `loc_sand` | `2` `6` `7` | animal spot of hippo candidates `loc_pond`, `loc_mud`, `loc_shade` |
| `3` `8` `9` | animal spot of panda candidates `loc_cave`, `loc_bamboo`, `loc_leaves` | | |
| `"` | tall grass with wildflowers (`[[scenery]]` `tall_grass_ne`, walkable) | `s` | sand patch (`sand_n`, walkable) |
| `m` | mud puddle (`mud_nw`, walkable) | `,` | tree shade at the zoo wall (`shade_w`, walkable) |
| `l` | leaf pile (`leaf_pile_ne`, walkable) | | |
| `.` | grass (walkable, slower — Q-046) | | |

Scenery cells (`"` `s` `m` `,` `l`) keep surface `grass`; they are drawn where no spot digit
is. Each hiding place's wander area (all cells within 3 m of its spot on its `wander_on`
surface) is listed in "Hiding places"; for the scenery places it lies inside the scenery
patch, for `loc_river` on the grass west of the river, for `loc_pond` in the water, for
`loc_cave` on the cave floor.

## Elements

Grid rect = `x, z, w, d` in 1 m cells (south-west corner + size), as in GAME-LAYOUT.
Solid = every type except `path` and `hiding_place`.

| Id | Type | Grid rect (x, z, w, d) | Notes |
|---|---|---|---|
| `wall_south_w` | boundary (zoo_wall) | -24, -2, 21, 2 | Outer zoo wall (stone blocks + hedge top), permanent. |
| `entrance_gate` | building (entrance) | -3, -2, 6, 2 | Entrance arch with blank sign board; turnstiles behind the player are closed. |
| `wall_south_e` | boundary (zoo_wall) | 3, -2, 21, 2 | Outer zoo wall, permanent. |
| `wall_west` | boundary (zoo_wall) | -24, 0, 2, 48 | Outer zoo wall, permanent. |
| `hedge_north_a` | decoration (hedge) | -22, 46, 13, 2 | Tall hedge, level edge, permanent. |
| `barrier_north_gate` | barrier (closed_gate) | -9, 46, 3, 2 | Closed wooden gate in the north hedge (later level). |
| `hedge_north_b` | decoration (hedge) | -6, 46, 16, 2 | Tall hedge, level edge, permanent. |
| `hedge_north_c` | decoration (hedge) | 13, 46, 9, 2 | Tall hedge, level edge, permanent. |
| `hedge_east_a` | decoration (hedge) | 22, 0, 2, 8 | Tall hedge behind the rock hill, permanent. |
| `barrier_east_repair` | barrier (road_block) | 22, 8, 2, 3 | Path under repair (later level). |
| `hedge_east_b` | decoration (hedge) | 22, 11, 2, 13 | Tall hedge, level edge, permanent. |
| `hedge_east_c` | decoration (hedge) | 22, 27, 2, 1 | Tall hedge between river and fallen tree. |
| `barrier_ne_tree` | barrier (fallen_tree) | 22, 28, 2, 3 | Fallen tree behind the bridge (→ level 2). |
| `hedge_east_d` | decoration (hedge) | 22, 31, 2, 17 | Tall hedge, level edge, permanent. |
| `path_plaza` | path (plaza) | -5, 0, 10, 8 | Entrance plaza; spawn. |
| `bamboo_sw` | decoration (bamboo) | -22, 0, 4, 3 | Dense bamboo thicket (3 m, taller than the 2.5 m zoo wall) in the south-west corner; the only growing bamboo outside the panda enclosure (`loc_bamboo`, Q-081). |
| `map_board` | landmark (map_board) | -7, 3, 1, 2 | Picture map of level 1 (silhouettes, no text). |
| `bench_plaza` | decoration (bench) | 6, 4, 2, 1 | Bench. |
| `path_ring_s` | path | -8, 8, 16, 3 | Ring path, south side; food storage door. |
| `path_ring_w` | path | -8, 11, 3, 16 | Ring path, west side; zebra gate, pond jetty. |
| `path_ring_e` | path | 5, 11, 3, 16 | Ring path, east side; hippo gate. |
| `path_ring_n` | path | -8, 27, 16, 3 | Ring path, north side; panda gate, bridge. |
| `food_storage` | building (food_storage) | -4, 11, 8, 6 | Food storage, door at cell (0, 11) on the south facade. Unlocked (proposal Q-033). |
| `hedge_center_w` | decoration (hedge) | -5, 11, 1, 6 | Tall hedge beside the food storage (sight blocker). |
| `hedge_center_e` | decoration (hedge) | 4, 11, 1, 6 | Tall hedge beside the food storage (sight blocker). |
| `grove_center` | decoration (tree_grove) | -5, 17, 10, 10 | Dense grove of tall trees inside the ring (main sight blocker). `density = "dense"`: solid, never entered (LAYOUT-016); bush border on all four walkable sides (`edge = "bushes"`, proposal Q-085); no canopy within 1.5 m of the north edge so it does not hide the player at `board_panda` (QA F11). |
| `enc_zebra` | enclosure | -20, 7, 11, 12 | Zebra enclosure; gate (-10, 12, 1, 2) on the east fence; stone-arch shelter, bushes, grass, **no water**. |
| `board_zebra` | decoration (info_board) | -9, 14, 1, 1 | Info board of `enc_zebra`, next to the gate. |
| `pond_water` | landmark (pond) | -19, 20, 8, 8 | Still pond: water lilies, frogs, reeds. |
| `jetty_pond` | path (jetty) | -11, 22, 3, 2 | Wooden jetty from the ring path to the pond edge. |
| `bench_pond` | decoration (bench) | -11, 26, 2, 1 | Bench on the pond shore. |
| `loc_pond` | hiding_place *(proposal)* | -19, 19, 11, 10 | Hiding place of `hippo`; animal spot (-13, 22) in the water, 2 m from the jetty tip. Legacy mirror of `[[hiding_place]]` `loc_pond` (Q-080). |
| `enc_panda` | enclosure | -6, 32, 12, 10 | Panda enclosure; gate (-1, 32, 2, 1) on the south fence; cut bamboo on a feeding rack (proposal Q-081 — no growing bamboo clumps), wooden platform and shelter, **no stone/cave**. |
| `board_panda` | decoration (info_board) | -4, 30, 1, 1 | Info board of `enc_panda`, next to the gate. |
| `path_north` | path | -9, 30, 3, 16 | Side path to `barrier_north_gate`. |
| `trees_nw` | decoration (trees) | -20, 32, 8, 10 | Open wood, `density = "sparse"`: walkable between 8 `tree_round` and 2 bushes (positions in "Woods"); its west row overhangs `shade_w` (`loc_shade`). Not solid as an element. |
| `path_bridge_w` | path | 8, 28, 2, 3 | Short path from the ring to the bridge. |
| `bridge_river` | path (bridge) | 10, 28, 3, 3 | Wooden bridge over the river. |
| `river_n` | landmark (river) | 10, 31, 3, 17 | River, flowing south from under the north hedge; rapids, ducks. |
| `river_mid` | landmark (river) | 10, 27, 3, 1 | River under the bridge (south side). |
| `river_e` | landmark (river) | 10, 24, 14, 3 | River bend flowing east, leaves through a grate under the east hedge. |
| `path_ne` | path | 13, 28, 9, 3 | Path behind the bridge to `barrier_ne_tree`. |
| `trees_ne` | decoration (trees) | 15, 34, 6, 9 | Open wood east of the river, `density = "sparse"`: walkable between 6 `tree_round` and 1 bush (positions in "Woods"); `loc_meadow` lies south of it, `loc_leaves` north of it (rake leans on the tree at (16.0, 41.3)). Not solid as an element. |
| `path_ne_trail` | path (side) | 13, 31, 2, 12 | Narrow trail from `path_ne` north between the river and `trees_ne` to the leaf pile (`loc_leaves`). |
| `loc_river` | hiding_place *(proposal)* | 6, 30, 4, 6 | Hiding place of `zebra`; animal spot (8, 32) on the west bank next to the bridge. Rect = bounding box of its wander area. Legacy mirror of `[[hiding_place]]` `loc_river` (Q-080). |
| `enc_hippo` | enclosure | 9, 11, 11, 12 | Hippo enclosure; gate (9, 15, 1, 2) on the west fence; square tiled pool `hippo_pool` (x 11–18, z 15–21, `[[enclosure_feature]]`, see "Hippo enclosure pool"), wooden hut (area x 15–18, z 11–14 reserved), edge stones; `home_wander_on = ["grass", "water"]`. |
| `hedge_hippo_nw` | decoration (hedge) | 8, 19, 1, 4 | Tall hedge left of the hippo gate (sight blocker for the cave). |
| `hedge_hippo_sw` | decoration (hedge) | 8, 11, 1, 4 | Tall hedge right of the hippo gate (sight blocker for the cave). |
| `board_hippo` | decoration (info_board) | 8, 17, 1, 1 | Info board of `enc_hippo`, next to the gate. |
| `rock_hill_w` | landmark (rock_hill) | 9, 0, 1, 8 | Rock hill, west flank of the cave mouth. |
| `rock_hill_back` | landmark (rock_hill) | 10, 0, 3, 5 | Rock hill, back wall of the cave. |
| `rock_hill_e` | landmark (rock_hill) | 13, 0, 9, 8 | Rock hill, main mass east of the cave. |
| `path_cave_floor` | path (cave) | 10, 5, 3, 3 | Cave floor under the rock roof (walkable, dark). |
| `path_cave` | path | 8, 8, 14, 3 | Service path along the rock hill; cave mouth; ends at `barrier_east_repair`. |
| `loc_cave` | hiding_place *(proposal)* | 10, 5, 3, 3 | Hiding place of `panda`; animal spot (10, 5) in the back corner on the west side of the cave. Legacy mirror of `[[hiding_place]]` `loc_cave` (Q-080). |

## Hiding places (candidates)

Discovery (user decision 2026-09-26, GAME-RESCUE §1, GAME-ANIMALS "Animal states"): 3
candidates per animal. Data: `[[hiding_place]]` in `level-1.toml` (fields explained there;
proposal Q-080). Riddle keys: `mission-<animal>-riddle-<id>-<reading_level>` (CONT-MISSIONS).
The `kiga` board shows the picture of the chosen place (`loc_*` picture id).

| Id | Animal | Area rect (x, z, w, d) | Animal spot | Wander on | Wander cells | Features (riddle details) | Scenery | Spot → own info board | Own board → spot (fastest walk) |
|---|---|---|---|---|---|---|---|---|---|
| `loc_river` | zebra | 6, 30, 4, 6 | (8, 32) | grass | 19 | flowing_water, bridge, ducks | `river_n`, `bridge_river` | 24.8 m | 16.6 s |
| `loc_meadow` | zebra | 15, 31, 7, 3 | (17, 32) | grass | 16 | tall_grass, wildflowers, butterflies, big_trees_behind | `tall_grass_ne`, `trees_ne` | 31.6 m | 21.4 s |
| `loc_sand` | zebra | -1, 42, 7, 4 | (2, 44) | grass | 22 | sand, dry, yellow_ground, no_grass | `sand_n` | 32.0 m | 23.9 s |
| `loc_pond` | hippo | -19, 19, 11, 10 | (-13, 22) | water | 22 | still_water, water_lilies, frogs | `pond_water`, `jetty_pond` | 21.6 m | 16.8 s |
| `loc_mud` | hippo | -20, 42, 7, 4 | (-17, 44) | grass | 22 | mud, brown_ground, wet, splashing | `mud_nw` | 36.8 m | 27.3 s |
| `loc_shade` | hippo | -22, 33, 2, 7 | (-22, 36) | grass | 12 | shade, big_trees, zoo_wall, dry_grass | `shade_w`, `trees_nw`, `wall_west` | 35.5 m | 30.6 s |
| `loc_cave` | panda | 10, 5, 3, 3 | (10, 5) | cave | 9 | dark, cool, stone, echo | `path_cave_floor`, `rock_hill_back` | 28.7 m | 17.7 s |
| `loc_bamboo` | panda | -22, 0, 8, 6 | (-18, 2) | grass | 21 | bamboo_thicket, green_stalks, taller_than_wall, rattling | `bamboo_sw`, `wall_west`, `wall_south_w` | 31.3 m | 23.9 s |
| `loc_leaves` | panda | 14, 43, 7, 3 | (17, 44) | grass | 17 | leaf_pile, red_yellow_leaves, rake, under_trees | `leaf_pile_ne`, `trees_ne` | 25.2 m | 18.4 s |

"Spot → own info board" = straight distance between the spot and the info-board cell
centres; walking times with 1.75 m/s on paths and 0.98 m/s on grass (as "Walking distances").

**Wander area** (GAME-ANIMALS "Animal states"; *proposal, level design*): the cells whose
centre is within `wander_radius_m` (3 m) of the spot centre, whose surface matches
`wander_on`, that lie inside the place's `rect` (*proposal Q-085* — needed since the woods
next to `loc_shade`, `loc_mud`, `loc_meadow` and `loc_leaves` became walkable; the clipped
areas equal the cell counts in the table above), and that are 4-connected to the spot
through such cells. `grass` = walkable
cell that is not a `path` cell (scenery cells count as grass); `water` = cell of
`pond_water` (the hippo swims); `cave` = `path` cell of kind `cave` (dead end the player
never needs to pass). An escaped animal only walks between cells of its wander area, so it
never stands on a path the player needs, never enters a solid element and never leaves the
place its riddle describes. Every wander area has ≥ 9 cells and a cell ≥ 2 m from the spot.

**Spread (RESC-014).** Straight distances between spots of different animals (minimum
12.0 m, `loc_meadow` – `loc_leaves`); every one of the 3 × 3 × 3 = 27 combinations has all
three chosen spots ≥ 12 m apart:

| | `loc_pond` | `loc_mud` | `loc_shade` | `loc_cave` | `loc_bamboo` | `loc_leaves` |
|---|---|---|---|---|---|---|
| `loc_river` | 23.3 | 27.7 | 30.3 | 27.1 | 39.7 | 15.0 |
| `loc_meadow` | 31.6 | 36.1 | 39.2 | 27.9 | 46.1 | 12.0 |
| `loc_sand` | 26.6 | 19.0 | 25.3 | 39.8 | 46.5 | 15.0 |
| `loc_cave` | 28.6 | 47.4 | 44.6 | — | — | — |
| `loc_bamboo` | 20.6 | 42.0 | 34.2 | — | — | — |
| `loc_leaves` | 37.2 | 34.0 | 39.8 | — | — | — |

**Picking rule** (*proposal, Q-082*): with the seeded RNG, pick one candidate per animal
uniformly (animals in the order of their enclosures in the level data); if two chosen spots
are < 12 m apart, draw the whole set again (at most 64 draws), then fall back to the first
valid combination in data order. The chosen place ids are saved (GAME-SAVE). In level 1 the
first draw is always valid.

**Nearest neighbour of each new place** (fastest walk, ≤ 10 s — "Walking distances" rule):
`loc_meadow` → bridge 2.5 s; `loc_sand` → `barrier_north_gate` (end of `path_north`) 7.2 s;
`loc_mud` → `barrier_north_gate` 6.2 s; `loc_shade` → `loc_mud` 7.4 s; `loc_bamboo` →
`map_board` 8.6 s; `loc_leaves` → `loc_meadow` 8.5 s (via `path_ne_trail`).

**Sight test.** For every candidate, no cell of its wander area (0.5 m above ground) is on
screen while the player stands next to its own info board or enclosure gate, for every
camera rotation and zoom (LAYOUT-L1-006; checked with the 1080×2340 portrait viewport and,
for information, also with 2340×1080 landscape: all 6 new candidates are off-screen in both;
`loc_pond` has one wander cell on screen in landscape from one gate-side cell).

## Hiding places — riddle details and sight lines

| Hiding place | Riddle details and how the layout provides them | Elements between enclosure and hiding place (`blocks_view`, informative — the decisive check is the screen test LAYOUT-L1-006) |
|---|---|---|
| `loc_river` (zebra) | **Flowing:** `river_n` → `river_e`, visible current, small rapids with stones and foam, gurgling sound. **Bridge:** `bridge_river` 1–3 m south of the animal spot. **Ducks:** 3 ducks swimming near the bridge. | `grove_center`, `hedge_center_w` (zebra enclosure is on the west side of the ring). |
| `loc_pond` (hippo) | **Still:** mirror-flat water, no current, no sound of rushing water. **Water lilies:** pads with pink/white flowers over most of the surface. **Frogs:** 2–3 frogs on stones and lily pads, croaking. Hippo in the water, only eyes and ears showing (`klasse3` riddle). Round shape, reeds, jetty — **no bridge, no ducks** (clearly different from the river). | `grove_center`, `hedge_center_e`, `hedge_hippo_nw` (hippo enclosure is on the east side). |
| `loc_cave` (panda) | **Dark:** 3 m deep cave under a rock roof, mouth facing north. **Cool:** blue-grey shadow colours, damp stone. **Stone:** walls and floor of grey rock blocks. **Echo:** reverb zone on `path_cave_floor` (audio). Panda asleep in the back corner on the west side (back-right as seen from the mouth). | `grove_center`, `hedge_hippo_nw`, `hedge_hippo_sw`, rock walls (panda enclosure is north of the ring). |
| `loc_meadow` (zebra) | **Tall grass:** `tall_grass_ne`, knee-high tufts (`grass_tuft` ×3 scale) reaching the zebra's belly — the only tall grass in level 1. **Wildflowers:** red, yellow and white flower heads between the tufts. **Butterflies:** 3–4 butterflies fluttering over the grass. **Big trees behind:** `trees_ne` directly north of the meadow. Zebras graze (`eat`). Reached over the bridge (east of the river). | `grove_center`, `hedge_center_w`, river (zebra enclosure is west, the meadow is the far north-east). |
| `loc_sand` (zebra) | **Sand, yellow and dry:** `sand_n`, a 7 × 4 m patch of `sand_tile` north of the panda enclosure, a few small rocks at the rim — the only sand in level 1. **No grass:** no tufts on the sand. Zebras roll/stand in the dust (`pose` proposal, Q-043). | `grove_center`, `enc_panda` (open fence, far distance does the work: 31 m). |
| `loc_mud` (hippo) | **Mud, brown, wet:** `mud_nw`, a 7 × 4 m brown puddle with glossy wet highlights, splashes and footprints at the rim — the only mud in level 1. **Not water:** no lilies, frogs, reeds or ducks, brown not blue (must not look like the pond). Hippo half-sunk in the mud. | `grove_center`, `trees_nw` (hippo enclosure is east; 37 m). |
| `loc_shade` (hippo) | **Shade under big trees:** `shade_w`, the 2 m corridor between `wall_west` and `trees_nw`; the crowns overhang it; flat dark-green shade decal on the ground. **Zoo wall:** the high stone wall right behind the hippo — the only place where big trees stand at the wall. **Dry grass:** no water, no mud. Hippo lies dozing (`sleep`). | `grove_center`, `trees_nw` (35 m). |
| `loc_bamboo` (panda) | **Bamboo thicket:** `bamboo_sw`, dense clumps of the `bamboo` prop filling the south-west corner, 3 m tall — **taller than the 2.5 m zoo wall** behind it. **Rattling:** stalks sway and clack in the wind (audio). Panda sits at the thicket's edge chewing (`eat`). Growing bamboo only here (Q-081). | `grove_center`, `enc_zebra` (panda enclosure is north; 31 m). |
| `loc_leaves` (panda) | **Leaf pile:** `leaf_pile_ne`, a big raked heap of red, yellow and brown leaves at the north edge of `trees_ne`, a **rake** leaning on a tree — the only leaf pile in level 1. **Sunlit and colourful** (not dark: must not look like the cave). Panda lies on its back in the leaves; leaves fly when it moves. | `trees_ne`, river (25 m). |

Riddle guards (so the riddle points to exactly one place):
- No other water in level 1 — in particular **no water trough in the zebra enclosure**. The
  only exception is `hippo_pool` inside the hippo's own enclosure: square, light blue/white
  tiled rim and ramp, clear water, **no lilies, frogs, reeds or ducks**, so it never matches
  the `loc_pond` riddle (lilies + frogs); the `kiga` picture of `loc_pond` must show lilies
  and frogs (risk noted in Q-085).
- No other stone shelter that could pass as a cave — the panda enclosure uses wood only; the
  zebra's stone arch is open on both sides and lies inside a fenced enclosure.
- The only bridge in level 1 is `bridge_river`.
- Scenery kinds exist once each: tall grass and butterflies only at `loc_meadow`, sand only
  at `loc_sand`, mud only at `loc_mud`, a leaf pile only at `loc_leaves`, growing bamboo only
  at `loc_bamboo` (the panda enclosure shows cut bamboo on a rack — Q-081), big trees right
  at the zoo wall only at `loc_shade`.
- Riddles of one animal never share their key detail: zebra = thirst/flowing water vs.
  hunger/tall grass vs. itchy fur/yellow dry ground; hippo = still water with lilies vs.
  brown wet ground vs. dry shade at the wall; panda = dark cool stone vs. tall green stalks
  vs. colourful leaves.

## Barriers

| Id | Kind | Cells (x, z, w, d) | Unlock condition — **proposal (Q-022)** | In-world explanation |
|---|---|---|---|---|
| `barrier_ne_tree` | fallen tree | 22, 28, 2, 3 | Level 1 → level 2: all three level-1 missions complete (`zebra`, `hippo`, `panda` in `in_enclosure`). | A storm knocked the tree over. When all three animals are home, a zookeeper saws it up and rolls the logs aside (GAME-LAYOUT §3). |
| `barrier_north_gate` | closed gate | -9, 46, 3, 2 | Later level (to be defined with Q-023). | Wooden gate with a padlock and a "closed" icon sign (no text). |
| `barrier_east_repair` | road block | 22, 8, 2, 3 | Later level (to be defined with Q-023). | "Path under repair": striped road block, blank sign with a shovel icon, zookeeper cart with traffic cones. |

The west and south sides are the permanent outer zoo wall. North and east are sealed by
permanent tall hedges and the three barriers; the river leaves the level through a grate
under the east hedge and enters from under the north hedge (water is not walkable).

## Walking distances

Fastest walking time over walkable cells (8-neighbour, diagonal = √2), with **1.75 m/s on
`path` cells and 0.98 m/s on `grass`** (0.56 ×, GAME-PLAYER §6, user decision 2026-09-26;
before: 1.4 m/s path, 0.7 ×), i.e. the child mostly uses
the paths. Measured from the spawn cell, the cell in front of the storage door, the cells
next to an info board, and the cells within interaction range (2 m, GAME-PLAYER) of an
animal spot. Shortest distance (any surface) given for reference.

| From → to (neighbours along the ring) | Shortest distance | Fastest time |
|---|---|---|
| spawn → food storage door | 8.0 m | 4.6 s |
| food storage → zebra info board | 10.2 m | 6.2 s |
| zebra info board → pond (hippo) | 7.8 m | 5.9 s |
| pond → panda info board | 9.9 m | 6.0 s |
| panda info board → river (zebra) | 10.4 m | 7.1 s |
| river → hippo info board | 13.4 m | 7.7 s |
| hippo info board → cave (panda) | 11.2 m | 6.7 s |
| cave → food storage door | 10.1 m | 6.8 s |
| hippo info board → food storage door | 12.2 m | 7.3 s |
| spawn → map board | 6.4 m | 3.9 s |

All neighbour pairs are ≤ 10 s (longest: river → hippo info board 7.7 s, was 9.6 s). Not
neighbours (for information): panda info board → food storage 14.8 s (was 18.4 s); zebra
board → panda board 9.5 s (was 11.7 s — now also under 10 s); hippo board → zebra board
13.5 s (was 16.5 s). The
panda mission is therefore the longest round trip — intentionally the third mission a child
will usually do.

## High-angle camera (Q-049 answered)

The game uses the high-angle follow camera of GAME-PLAYER §2 (pitch ≈ 55°, default ≈ 14 m,
zoom 10–20 m, rotation in 45° steps; user decision 2026-09-26). Effects on this level:

1. **Hiding places vs. own enclosure.** From above, the central grove and the hedges no
   longer block the view the way they do at eye level. The design works through
   **distance**: info board → own hiding place is 24.8 m (zebra → river), 21.6 m
   (hippo → pond), 28.7 m (panda → cave), and 25–37 m for the six further candidates
   ("Hiding places"). Rule 7 / LAYOUT-L1-006 is therefore a screen test, applied to every
   candidate and its whole wander area. Whether it holds depends on the FOV axis (Q-052).
2. **Animals seen while walking.** From above, the child sees more of the surroundings while
   walking the ring (e.g. the hippo in the pond when walking to the zebra board). This is
   allowed (GAME-RESCUE §4) but makes guessing easier; playtest before changing anything.
3. **Cave.** The rock roof over `path_cave_floor` is cut away while the player is in or at
   the cave mouth (GAME-PLAYER §2). *Proposal (level design):* the panda lies near the
   mouth (head visible from above); the "dark" riddle detail comes from lighting (dark
   blue-grey floor, shadow) rather than from an enclosed interior.
4. **Food storage.** The roof is cut away while the player is inside (GAME-PLAYER §2).
5. **Signs and info boards** are tilted back towards the camera; riddle texts and food box
   labels are read in the close-up text panel (ART-DIRECTION §4, GAME-PLAYER §4, ADIR-001/003).
6. **Hedges.** 3 m hedges still frame areas and seal the level from above; their role as
   sight blockers (`blocks_view`) matters less.

## Behaviour

1. Level 1 contains exactly the elements of the table above; `assets/levels/level-1.toml`
   mirrors it (GAME-LAYOUT LAYOUT-005).
2. The player spawns at cell (0, 2) facing north (level +z = world −Z, Q-056).
3. Every enclosure, building, landmark, barrier and hiding place (every `[[hiding_place]]`
   candidate) is reachable from the spawn over walkable cells (hedges and walls need not be).
4. Every border cell of the level bounds `(-24, -2, 48, 50)` is solid; with all barriers
   closed no walkable cell outside the bounds can be reached.
5. No two solid elements overlap, and no path cell lies under a solid element.
6. Walking time between neighbouring points of interest (table above) is ≤ 10 s with
   1.75 m/s on paths and 0.98 m/s on grass.
7. For each level-1 mission and **each of its candidate hiding places**, while the player
   stands at its info board or enclosure gate, every cell of the place's wander area (the
   animal spot included) is **off-screen** for every allowed camera rotation (45° steps)
   and zoom (10–20 m) of GAME-PLAYER §2 — the animal cannot be seen from its own enclosure
   wherever it hides and wanders (Q-049; FOV definition Q-052).
8. Each hiding place's `features` contain all riddle details of CONT-MISSIONS for it; the
   river has a bridge within 4 m of the animal spot; no bridge and no river cell lies
   within the pond hiding place; level 1 contains no other water and no other bridge; each
   scenery kind that a riddle relies on exists only at its own hiding place.
9. Each animal spot (of every candidate) is within interaction range (2 m) of at least one
   walkable cell.
10. When the unlock condition of `barrier_ne_tree` is met, its cells become walkable; the
    other two barriers stay closed.
11. The food storage is open from the start (proposal Q-033).
12. The food boxes (`[[food_box]]`, GAME-FEED §7) are props, not layout elements: they do not
    occupy grid cells (LAYOUT-L1-003/004 unaffected) but are solid for the player
    (GAME-PLAYER §7); at least 2 m of `path_ring_s` stays free in front of them.
13. Every candidate hiding place has a **wander area** (definition in "Hiding places") of
    ≥ 9 cells with a cell ≥ 2 m from the spot; it contains no solid cell and no `path` cell
    except cave floor, lies inside the place's `rect`, and wander areas and `rect`s of
    different animals never overlap (GAME-ANIMALS ANIM-008).
14. For every candidate there is a combination of one candidate per animal that contains it
    and keeps all chosen spots ≥ 12 m apart (level 1: all 27 combinations) — RESC-014.
15. `[[scenery]]` entries are not solid and change no cell: they overlap no solid element and
    no path, and lie inside their hiding place's `rect`.
16. Each candidate hiding place is ≤ 10 s (fastest walk) from a neighbouring point of
    interest (list in "Hiding places").
17. `enc_hippo` contains `hippo_pool` (43 % of its inner cells, see "Hippo enclosure
    pool"); the hippo wanders on grass and pool water when home.
18. `grove_center` is `dense` (solid, never entered); `trees_nw` and `trees_ne` are
    `sparse`: their cells are walkable grass and only the listed trunks and bushes are solid
    ("Woods").
19. Info boards, enclosure signs and the map board are solid from every side; the player
    never passes through or under a sign panel (Q-086).


## Hippo enclosure pool (user decision 2026-09-26)

The hippo enclosure (`enc_hippo`) contains a **pool** (`hippo_pool`, the tiled pool of the
approved `kit_enclosure_buildings` sheet): still water covering about half of the
enclosure's inner area, with a shallow entry ramp and a few stones at the edge. The hippo
wanders in and out of the pool when home (GAME-ANIMALS wandering; `wander_on` includes the
pool water) and spends most of its time in the water with only eyes, ears and back
showing. The pool is inside the fence, so it is never reachable for the player; it must be
visible from the path in front of the gate. Its water uses the pond look (still, TECH-WATER).

**Placement** *(level design; data shape proposal Q-085)* — `[[enclosure_feature]]`
`hippo_pool` in `level-1.toml`:

| Field | Value | Why |
|---|---|---|
| `rect` | 11, 15, 8, 7 (x 11–18, z 15–21) | 56 of the 130 inner cells (enclosure 11 × 12 = 132 minus the 2 gate cells) = **43 %** |
| distance to the gate | 2 cells (the grass column x = 10 and the gate column x = 9 lie between the gate and the rim) | not adjacent to the gate cells (9, 15), (9, 16); the arriving hippo steps onto grass first |
| margins | 1 m grass east (x = 19) and north (z = 22) between rim and fence; 4 m grass south (z 11–14) with the hut area x 15–18 | rim never touches the fence; room for `hut_wood` (≈ 4 × 3.5 m, placement to be confirmed with the hut model) |
| `ramp` | 11, 16, 1, 3 on the **west** side (`ramp_side = "-x"`) | faces the gate and the path; the child sees the hippo walk in and out |
| `edge_stones` | (10.4, 21.6), (11.4, 14.4), (13.6, 14.3) — `rock` at scale 0.5 on the grass at the north-west corner and the south rim | decoration from the concept; kept out of the 1 m strips east and north of the pool so the home wander area stays connected |
| water | `still`, pond water shader, no plants or animals | riddle guard (not the pond) |

**Home wander area** (GAME-LAYOUT "Enclosure features and wandering at home"): 55 grass
cells (enclosure minus gate cells, pool, hut area and the three stone cells) + 56 water
cells, all connected; grass and water connect only through the 3 ramp cells. *Proposal:* 7 of
10 wander targets are water cells.

**Visible from the path in front of the gate** (player on (8, 15) or (8, 16), high-angle
camera of GAME-PLAYER §2): portrait 1080×2340 at 14 m with the camera turned to look east
(towards the gate): 25–33 % of the pool cells incl. ≥ 2 ramp cells on screen (the narrow
portrait width shows the near, west part of the pool); at 20 m 57–71 %. Landscape 2340×1080
at the default rotation (looking north), 14 m: 98–100 %. In portrait with the camera looking
north the pool is beside the screen — intrinsic to the narrow portrait view, not a layout
problem.

**For the implementer (zoo-core):** read `[[enclosure_feature]]` and `home_wander_on`; place
`pool_tiled` (placeholder: water tiles in the pond look inside a tiled-rim box) at the rect,
the ramp on its west side and the edge stones; replace the current hippo
`enclosure_dressing` (its rock at (16.7, 14.6) lies in the hut area, the rock at
(11.75, 20.0) and the bush at (17.8, 20.6) lie in the pool).

## Woods (user decision 2026-09-26: dense vs. walkable)

| Tree area | Density | Why |
|---|---|---|
| `grove_center` | `dense` | must stay solid: the hidden middle of the ring (LAYOUT-016); bush border so its edge along the ring path is not an invisible wall |
| `trees_nw` | `sparse` | open wood west of `path_north`; the child can walk in; nothing behind it that must stay sealed (zoo wall 2 m west, mud patch north) |
| `trees_ne` | `sparse` | open wood between meadow and leaf pile; walking through it links `loc_meadow` and `loc_leaves` besides `path_ne_trail` |

`trees_nw` stands 2 m away from the zoo wall (the shade corridor `shade_w` lies between), so
it is not a "forest edge along the zoo wall" in the sense of the user decision; it could be
made `dense` without breaking any rule (Q-085).

Tree and bush positions (trunk centres, level coordinates; `tree_round` collider r 0.45,
`bush` r 0.70 — GAME-LAYOUT "Collision footprints"):

| Area | Trees (`tree_round`) | Bushes | Smallest clear gap | Crossing (player r 0.3) |
|---|---|---|---|---|
| `trees_nw` | (−18.9, 33.2), (−15.9, 33.0), (−12.9, 33.3), (−17.4, 36.2), (−14.3, 36.2), (−19.0, 38.7), (−15.9, 38.7), (−12.5, 38.7) | (−17.45, 41.3), (−14.1, 41.3) | 1.88 m | west–east and south–north ok |
| `trees_ne` | (16.0, 35.2), (19.4, 35.2), (17.7, 38.2), (20.5, 38.2), (16.0, 41.3), (19.3, 41.3) | (20.3, 34.9), touching the trunk at (19.4, 35.2) (one obstacle) | 1.90 m | west–east and south–north ok |

Checks with the woods open (scratch analysis of `level-1.toml`, 2026-09-26):
- **Reachability / sealing (LAYOUT-L1-001/002):** 1 173 reachable cells (was 1 074); every
  free cell of both woods is reachable; every border cell is still solid; the woods touch no
  barrier and no border, so no shortcut past `barrier_north_gate`, `barrier_ne_tree` or
  `barrier_east_repair` opens.
- **Hiding places:** with the rect clip, all wander areas keep their cell counts (19, 16, 22,
  22, 22, 12, 9, 21, 17); without it `loc_meadow`, `loc_shade`, `loc_leaves` would grow
  into the woods (21, 17, 23 cells) and leave their riddle area. No collider lies inside a
  hiding-place `rect` or on scenery. LAYOUT-L1-006 is a screen test of wander cells, so the
  thinner woods change nothing there; `trees_nw`/`trees_ne` get `blocks_view = false`
  (canopies only, the occluder fade applies).
- **Walking times (LAYOUT-L1-005/018):** opening cells can only shorten walks; all
  listed times stay ≤ 10 s.

## Collision and billboards (GAME-LAYOUT "Collision footprints", Q-086, Q-087)

Findings for level 1 (scene assembly of 2026-09-26 + exported `.glb` files):
- **Enclosure signs (billboard violation):** each `enclosure_sign` stands 0.35 m outside its
  gate across the whole 2 m opening; only its two posts are solid and its panel starts at
  0.99 m, so the 1.20 m player walks **through** the panel (zebra: from the grass column
  x = −9 or on the way to the gate). A full box would block the gate. Proposal (Q-086): make
  it a **gate arch** — posts beside the opening, panel bottom ≥ 2.1 m; beside the gate there
  is no room at the hippo gate (hedges `hedge_hippo_sw/nw` and `board_hippo` fill x = 8).
- **Map board:** only its posts are solid; it is safe today only because its two cells are
  solid. Proposed box footprint in GAME-LAYOUT.
- **Info boards:** solid cell + box; the panel overhangs the box by ≤ 0.07 m (proposed box
  in GAME-LAYOUT).
- **Fallen tree** (`barrier_ne_tree`): the model reaches x 21.75, 0.25 m into the walkable
  column x = 21 of `path_ne`; give it a footprint (GAME-LAYOUT) or place it 0.3 m further east.
- **Invisible walls:** wall and hedge bands 0.5–0.7 m (F7), both barriers 0.6–0.7 m, the
  open entrance arch (F8), the edges of `grove_center`; accepted: info board fronts
  (0.26 m) and the back strip of `map_board` (0.36 m). Fixes in GAME-LAYOUT.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| LAYOUT-L1-001 | Given `level-1.toml`, then every enclosure, building, landmark, barrier and hiding place has a walkable cell reachable from the spawn next to it or inside it (flood fill). | unit |
| LAYOUT-L1-002 | Given `level-1.toml` with all barriers closed, then every border cell of the bounds is solid and the flood fill from the spawn never leaves the bounds. | unit |
| LAYOUT-L1-003 | Given `level-1.toml`, then no two solid elements share a cell and no `path` cell is covered by a solid element. | unit |
| LAYOUT-L1-004 | Given this spec's element table and `level-1.toml`, then both list the same ids, types and rectangles (instance of LAYOUT-005). | unit |
| LAYOUT-L1-005 | Given `level-1.toml` with path speed 1.75 m/s and grass speed 0.98 m/s, then each neighbour pair of the walking-distance table has a fastest walking time ≤ 10 s. | unit |
| LAYOUT-L1-006 | Given missions `zebra`, `hippo`, `panda`, **every candidate hiding place of the mission** (`[[hiding_place]]`), the player on each walkable cell next to the info board or the enclosure gate, and the camera of GAME-PLAYER §2 at every 45° rotation and at 10, 14 and 20 m distance on a 1080×2340 viewport, then the animal spot and every cell centre of its wander area (0.5 m above ground) lie outside the view frustum (depends on Q-052). | unit |
| LAYOUT-L1-007 | Given the hiding places of level 1, then their `features` contain the CONT-MISSIONS details (river: flowing_water, bridge, ducks; pond: still_water, water_lilies, frogs; cave: dark, cool, stone, echo; meadow: tall_grass, wildflowers, butterflies; sand: sand, dry, yellow_ground; mud: mud, brown_ground, wet; shade: shade, big_trees, zoo_wall; bamboo: bamboo_thicket, green_stalks, taller_than_wall; leaves: leaf_pile, red_yellow_leaves, rake), a `bridge` path lies within 4 m of the river animal spot, no bridge/river cell lies inside `loc_pond`, and every `scenery` id a hiding place names exists (`[[scenery]]` or `[[element]]`). | unit |
| LAYOUT-L1-008 | Given `level-1.toml`, then the only water elements are `river_*` and `pond_water`, the only other water is the `hippo_pool` enclosure feature inside `enc_hippo` (no lily/frog/reed/duck props on it), and the only `bridge` is `bridge_river`. | unit |
| LAYOUT-L1-009 | Given each level-1 hiding place (every `[[hiding_place]]` candidate), then at least one walkable cell centre is within 2 m of its animal spot. | unit |
| LAYOUT-L1-010 | Given missions `zebra`, `hippo`, `panda` complete, then `barrier_ne_tree` cells are walkable, `barrier_north_gate` and `barrier_east_repair` are still solid (depends on Q-022). | unit |
| LAYOUT-L1-011 | Given level 1 starts on a 1080×2340 viewport, then the player stands on the plaza facing north with the camera at maximum zoom-out (20 m) and the food storage is on screen. | e2e |
| LAYOUT-L1-012 | Given the approved mockups `loc_river` and `loc_pond`, then a reviewer can tell river and pond apart without text (flow + bridge + ducks vs. still + lilies + frogs), and `env_zebra` shows no water, `env_panda` no stone cave. | manual |
| LAYOUT-L1-013 | Given `level-1.toml`, then it has 10 `food_box` entries (one per food) in front of the food storage, and a walkable cell centre within 2 m in front of each box is reachable from the spawn. | unit |
| LAYOUT-L1-014 | Given each `[[hiding_place]]` of level 1, when its wander area is computed (cells within `wander_radius_m` = 3 m of the spot centre, surface = `wander_on`, 4-connected to the spot), then it contains the spot, has ≥ 9 cells and a cell ≥ 2 m from the spot, contains no solid cell and no `path` cell except kind `cave`, lies inside the place's `rect`, and wander areas and `rect`s of places of different animals are disjoint. | unit |
| LAYOUT-L1-015 | Given the `[[hiding_place]]` list of level 1, then every animal with an enclosure has ≥ 3 candidates, and for every candidate there is a combination (one candidate per animal) that contains it and has all spots pairwise ≥ 12 m apart; in level 1 all 27 combinations do (RESC-014). | unit |
| LAYOUT-L1-016 | Given the `[[scenery]]` list, then no scenery rect overlaps a solid element or a `path` cell, each lies inside the `rect` of its `hiding_place`, and each scenery `kind` occurs once in level 1. | unit |
| LAYOUT-L1-017 | Given the legacy `[[element]] type = "hiding_place"` entries (while they exist, Q-080), then each has a `[[hiding_place]]` with the same id, animal, rect, animal spot and features. | unit |
| LAYOUT-L1-018 | Given `level-1.toml` with path speed 1.75 m/s and grass speed 0.98 m/s, then `loc_meadow`, `loc_sand`, `loc_mud`, `loc_shade`, `loc_bamboo` and `loc_leaves` (cells within 2 m of the spot) are each ≤ 10 s fastest walk from the neighbour listed in "Hiding places". | unit |
| LAYOUT-L1-019 | Given the approved mockups of `loc_meadow`, `loc_sand`, `loc_mud`, `loc_shade`, `loc_bamboo`, `loc_leaves`, then a reviewer can name each place's riddle details without text, `loc_mud` does not look like water, `loc_leaves` does not look dark (not like the cave), and `env_panda` shows no growing bamboo (Q-081). | manual |
| LAYOUT-L1-020 | Given the level data, then `enc_hippo` contains a `hippo_pool` water area of 35–60 % of its inner cells (enclosure cells minus gate cells; level 1: 56 / 130 = 43 %), fully inside the fence and not adjacent (8-neighbourhood) to the gate cells, with its `ramp` inside the pool rect; `enc_hippo.home_wander_on` contains `water`, and the home wander area (GAME-LAYOUT LAYOUT-020) includes the pool cells and is connected from the grass cell next to the gate. | unit |
| LAYOUT-L1-021 | Given the player on (8, 15) or (8, 16) in front of the hippo gate, the camera of GAME-PLAYER §2 at 14 m, then with the camera looking east on a 1080×2340 viewport ≥ 25 % of the pool cell centres (water height 0) incl. ≥ 2 ramp cells are on screen, and looking north on a 2340×1080 viewport ≥ 90 % are. | unit |
| LAYOUT-L1-022 | Given `trees_nw` and `trees_ne` (`sparse`), then every listed tree/bush collider (r 0.45 / 0.70) lies inside the element rect and outside every path, scenery and hiding-place `rect`; the clear gap between obstacles is ≥ 1.8 m (touching colliders count as one obstacle); the player circle can cross each wood west–east and south–north; every cell centre of the woods not covered by a collider is reachable from the spawn (instance of LAYOUT-015). | unit |
| LAYOUT-L1-023 | Given `grove_center` (`dense`), then none of its cells is reachable (LAYOUT-016) and each of its walkable sides has a bush border (LAYOUT-019). | unit |
| LAYOUT-L1-024 | Given the wander areas of all 9 hiding places computed with the rect clip, then they equal the cell counts of the "Hiding places" table (19, 16, 22, 22, 22, 12, 9, 21, 17) and contain no tree or bush collider. | unit |
| LAYOUT-L1-025 | Given the level-1 scene, then the player circle can reach no position that overlaps the cross-section below 1.4 m of any `info_board`, `enclosure_sign` or `map_board` mesh (LAYOUT-017), and the only invisible-wall edges wider than 0.3 m (LAYOUT-019) are the accepted ones listed in "Collision and billboards" (none once Q-087 is implemented). | unit |

## Open questions

- Q-022 barrier unlock conditions, Q-023 number of levels and their areas.
- Q-085 data shape for woods, hippo pool and home wandering (`density`, `trees`, `edge`, `[[enclosure_feature]]`, `home_wander_on`, wander clip to `rect`, 7/10 water targets). Q-086 enclosure sign as a gate arch. Q-087 collision footprint values and invisible-wall fixes.
- Q-033 food storage lock (level 1 assumes unlocked).
- Q-044 `hiding_place` element type and `blocks_view` data. Q-080 `[[hiding_place]]` / `[[scenery]]` lists and `wander_on`. Q-081 growing bamboo only at `loc_bamboo`. Q-082 picking rule for hiding places. Q-083 two-word `kiga` place words and food-word clashes. Q-043 poses at the new places.
- Q-046 answered: grass walkable, slower than paths.
- Q-049 answered: high-angle game camera (GAME-PLAYER §2; section above). Q-052 answered: 35° vertical FOV; level 1 starts at maximum zoom-out (LAYOUT-L1-011).
- Q-047 which food boxes stand in the storage in level 1.
- Q-024 walking speed (answered by GAME-PLAYER §6: 1.75 m/s path, 0.98 m/s grass).
- Q-056 answered (axes: level x east / z north, world = (x, 0, −z)). Q-057 answered (fences, hedges, walls: 2 m + 1 m segments, bands as one row on the centre line — GAME-LAYOUT "Modular edges", LAYOUT-013). Q-059 band joins, Q-060 fence/band placement (proposals).
