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

| Mission | Enclosure | Food box | Hiding place | What the riddle relies on (CONT-MISSIONS) |
|---|---|---|---|---|
| `zebra` | `enc_zebra` (west) | Gras / grass | `loc_river` (north-east) | flowing water, bridge, ducks |
| `hippo` | `enc_hippo` (east) | Melonen / melons | `loc_pond` (west) | still water, water lilies, frogs |
| `panda` | `enc_panda` (north) | Bambus / bamboo | `loc_cave` (south-east) | dark, cool, stone, echo |

Plus the zoo entrance (spawn) and the food storage. Everything else of the zoo is closed off
by child-friendly barriers.

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
| Sight test data | Solid elements carry `blocks_view` (and `height_m` for the mockups). | Q-044 |
| Barrier unlock | `barrier_ne_tree` opens when all three level-1 animals are home; the other two barriers belong to later levels. | Q-022, Q-023 |
| Food boxes in level 1 | All 10 food boxes stand in the storage (natural distractors). | Q-047 |
| Walking speed | 1.4 m/s on paths; grass 0.7 × = 0.98 m/s (grass factor: GAME-PLAYER). | Q-024 |
| Panda spot and cave view | Panda lies near the cave mouth so its head is visible from the high camera (see "High-angle camera" below). | — (level design) |

## Spawn and camera

- Spawn cell **(0, 2)** on `path_plaza`, 2 m inside the entrance gate.
- Facing **+Z (north)**; the camera starts south of the player looking north — the
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
  45 ##.............===................~~~.........%%
  44 ##.............===................~~~.........%%
  43 ##.............===................~~~.........%%
  42 ##.............===................~~~..TTTTTT.%%
  41 ##..TTTTTTTT...===pppppppppppp....~~~..TTTTTT.%%
  40 ##..TTTTTTTT...===pppppppppppp....~~~..TTTTTT.%%
  39 ##..TTTTTTTT...===pppppppppppp....~~~..TTTTTT.%%
  38 ##..TTTTTTTT...===pppppppppppp....~~~..TTTTTT.%%
  37 ##..TTTTTTTT...===pppppppppppp....~~~..TTTTTT.%%
  36 ##..TTTTTTTT...===pppppppppppp....~~~..TTTTTT.%%
  35 ##..TTTTTTTT...===pppppppppppp....~~~..TTTTTT.%%
  34 ##..TTTTTTTT...===pppppppppppp....~~~..TTTTTT.%%
  33 ##..TTTTTTTT...===pppppppppppp....~~~.........%%
  32 ##..TTTTTTTT...===pppppggppppp..1.~~~.........%%
  31 ##.............===................~~~.........%%
  30 ##.............===..i...........==HHH=========XX
  29 ##..............==================HHH=========XX
  28 ##..............==================HHH=========XX
  27 ##...oooooooo...================..~~~.........%%
  26 ##...oooooooobb.===TTTTTTTTTT===..~~~~~~~~~~~~~~
  25 ##...oooooooo...===TTTTTTTTTT===..~~~~~~~~~~~~~~
  24 ##...oooooooo...===TTTTTTTTTT===..~~~~~~~~~~~~~~
  23 ##...oooooooojjj===TTTTTTTTTT===..............%%
  22 ##...oooooo2ojjj===TTTTTTTTTT===%hhhhhhhhhhh..%%
  21 ##...oooooooo...===TTTTTTTTTT===%hhhhhhhhhhh..%%
  20 ##...oooooooo...===TTTTTTTTTT===%hhhhhhhhhhh..%%
  19 ##..............===TTTTTTTTTT===%hhhhhhhhhhh..%%
  18 ##..zzzzzzzzzzz.===TTTTTTTTTT===.hhhhhhhhhhh..%%
  17 ##..zzzzzzzzzzz.===TTTTTTTTTT===ihhhhhhhhhhh..%%
  16 ##..zzzzzzzzzzz.===%FFFFFFFF%===.ghhhhhhhhhh..%%
  15 ##..zzzzzzzzzzz.===%FFFFFFFF%===.ghhhhhhhhhh..%%
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
   2 ##.................=====S====....^^^^^^^^^^^^^%%
   1 ##.................==========....^^^^^^^^^^^^^%%
   0 ##.................==========....^^^^^^^^^^^^^%%
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
| `^` | rock hill (`landmark`) | `T` | trees / tree grove (`decoration`, blocks view) |
| `F` / `D` | food storage / its door | `M` | map board (`landmark`) |
| `z` `h` `p` | enclosure zebra / hippo / panda | `g` | enclosure gate (with enclosure sign) |
| `i` | info board | `b` | bench |
| `S` | spawn | `1` `2` `3` | animal spot of `loc_river` (zebra), `loc_pond` (hippo), `loc_cave` (panda) |
| `.` | grass (walkable, slower — Q-046) | | |

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
| `map_board` | landmark (map_board) | -7, 3, 1, 2 | Picture map of level 1 (silhouettes, no text). |
| `bench_plaza` | decoration (bench) | 6, 4, 2, 1 | Bench. |
| `path_ring_s` | path | -8, 8, 16, 3 | Ring path, south side; food storage door. |
| `path_ring_w` | path | -8, 11, 3, 16 | Ring path, west side; zebra gate, pond jetty. |
| `path_ring_e` | path | 5, 11, 3, 16 | Ring path, east side; hippo gate. |
| `path_ring_n` | path | -8, 27, 16, 3 | Ring path, north side; panda gate, bridge. |
| `food_storage` | building (food_storage) | -4, 11, 8, 6 | Food storage, door at cell (0, 11) on the south facade. Unlocked (proposal Q-033). |
| `hedge_center_w` | decoration (hedge) | -5, 11, 1, 6 | Tall hedge beside the food storage (sight blocker). |
| `hedge_center_e` | decoration (hedge) | 4, 11, 1, 6 | Tall hedge beside the food storage (sight blocker). |
| `grove_center` | decoration (tree_grove) | -5, 17, 10, 10 | Dense grove of tall trees inside the ring (main sight blocker). |
| `enc_zebra` | enclosure | -20, 7, 11, 12 | Zebra enclosure; gate (-10, 12, 1, 2) on the east fence; stone-arch shelter, bushes, grass, **no water**. |
| `board_zebra` | decoration (info_board) | -9, 14, 1, 1 | Info board of `enc_zebra`, next to the gate. |
| `pond_water` | landmark (pond) | -19, 20, 8, 8 | Still pond: water lilies, frogs, reeds. |
| `jetty_pond` | path (jetty) | -11, 22, 3, 2 | Wooden jetty from the ring path to the pond edge. |
| `bench_pond` | decoration (bench) | -11, 26, 2, 1 | Bench on the pond shore. |
| `loc_pond` | hiding_place *(proposal)* | -19, 19, 11, 10 | Hiding place of `hippo`; animal spot (-13, 22) in the water, 2 m from the jetty tip. |
| `enc_panda` | enclosure | -6, 32, 12, 10 | Panda enclosure; gate (-1, 32, 2, 1) on the south fence; bamboo, wooden platform and shelter, **no stone/cave**. |
| `board_panda` | decoration (info_board) | -4, 30, 1, 1 | Info board of `enc_panda`, next to the gate. |
| `path_north` | path | -9, 30, 3, 16 | Side path to `barrier_north_gate`. |
| `trees_nw` | decoration (trees) | -20, 32, 8, 10 | Tree group (decoration, sight/space filler). |
| `path_bridge_w` | path | 8, 28, 2, 3 | Short path from the ring to the bridge. |
| `bridge_river` | path (bridge) | 10, 28, 3, 3 | Wooden bridge over the river. |
| `river_n` | landmark (river) | 10, 31, 3, 17 | River, flowing south from under the north hedge; rapids, ducks. |
| `river_mid` | landmark (river) | 10, 27, 3, 1 | River under the bridge (south side). |
| `river_e` | landmark (river) | 10, 24, 14, 3 | River bend flowing east, leaves through a grate under the east hedge. |
| `path_ne` | path | 13, 28, 9, 3 | Path behind the bridge to `barrier_ne_tree`. |
| `trees_ne` | decoration (trees) | 15, 34, 6, 9 | Tree group east of the river. |
| `loc_river` | hiding_place *(proposal)* | 6, 31, 4, 4 | Hiding place of `zebra`; animal spot (8, 32) on the west bank next to the bridge. |
| `enc_hippo` | enclosure | 9, 11, 11, 12 | Hippo enclosure; gate (9, 15, 1, 2) on the west fence; square tiled pool, wooden hut, stones. |
| `hedge_hippo_nw` | decoration (hedge) | 8, 19, 1, 4 | Tall hedge left of the hippo gate (sight blocker for the cave). |
| `hedge_hippo_sw` | decoration (hedge) | 8, 11, 1, 4 | Tall hedge right of the hippo gate (sight blocker for the cave). |
| `board_hippo` | decoration (info_board) | 8, 17, 1, 1 | Info board of `enc_hippo`, next to the gate. |
| `rock_hill_w` | landmark (rock_hill) | 9, 0, 1, 8 | Rock hill, west flank of the cave mouth. |
| `rock_hill_back` | landmark (rock_hill) | 10, 0, 3, 5 | Rock hill, back wall of the cave. |
| `rock_hill_e` | landmark (rock_hill) | 13, 0, 9, 8 | Rock hill, main mass east of the cave. |
| `path_cave_floor` | path (cave) | 10, 5, 3, 3 | Cave floor under the rock roof (walkable, dark). |
| `path_cave` | path | 8, 8, 14, 3 | Service path along the rock hill; cave mouth; ends at `barrier_east_repair`. |
| `loc_cave` | hiding_place *(proposal)* | 10, 5, 3, 3 | Hiding place of `panda`; animal spot (10, 5) in the back corner on the west side of the cave. |

## Hiding places — riddle details and sight lines

| Hiding place | Riddle details and how the layout provides them | Elements between enclosure and hiding place (`blocks_view`, informative — the decisive check is the screen test LAYOUT-L1-006) |
|---|---|---|
| `loc_river` (zebra) | **Flowing:** `river_n` → `river_e`, visible current, small rapids with stones and foam, gurgling sound. **Bridge:** `bridge_river` 1–3 m south of the animal spot. **Ducks:** 3 ducks swimming near the bridge. | `grove_center`, `hedge_center_w` (zebra enclosure is on the west side of the ring). |
| `loc_pond` (hippo) | **Still:** mirror-flat water, no current, no sound of rushing water. **Water lilies:** pads with pink/white flowers over most of the surface. **Frogs:** 2–3 frogs on stones and lily pads, croaking. Hippo in the water, only eyes and ears showing (`klasse3` riddle). Round shape, reeds, jetty — **no bridge, no ducks** (clearly different from the river). | `grove_center`, `hedge_center_e`, `hedge_hippo_nw` (hippo enclosure is on the east side). |
| `loc_cave` (panda) | **Dark:** 3 m deep cave under a rock roof, mouth facing north. **Cool:** blue-grey shadow colours, damp stone. **Stone:** walls and floor of grey rock blocks. **Echo:** reverb zone on `path_cave_floor` (audio). Panda asleep in the back corner on the west side (back-right as seen from the mouth). | `grove_center`, `hedge_hippo_nw`, `hedge_hippo_sw`, rock walls (panda enclosure is north of the ring). |

Riddle guards (so the riddle points to exactly one place):
- No other water in level 1 — in particular **no water trough in the zebra enclosure**; the
  hippo pool is square with a tiled edge and has no lilies or frogs.
- No other stone shelter that could pass as a cave — the panda enclosure uses wood only; the
  zebra's stone arch is open on both sides and lies inside a fenced enclosure.
- The only bridge in level 1 is `bridge_river`.

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

Fastest walking time over walkable cells (8-neighbour, diagonal = √2), with **1.4 m/s on
`path` cells and 0.98 m/s on `grass`** (0.7 ×, Q-046 / Q-024), i.e. the child mostly uses
the paths. Measured from the spawn cell, the cell in front of the storage door, the cells
next to an info board, and the cells within interaction range (2 m, GAME-PLAYER) of an
animal spot. Shortest distance (any surface) given for reference.

| From → to (neighbours along the ring) | Shortest distance | Fastest time |
|---|---|---|
| spawn → food storage door | 8.0 m | 5.7 s |
| food storage → zebra info board | 10.2 m | 7.5 s |
| zebra info board → pond (hippo) | 7.8 m | 7.2 s |
| pond → panda info board | 9.9 m | 7.5 s |
| panda info board → river (zebra) | 10.4 m | 8.7 s |
| river → hippo info board | 13.4 m | 9.6 s |
| hippo info board → cave (panda) | 11.2 m | 8.2 s |
| cave → food storage door | 10.1 m | 8.3 s |
| hippo info board → food storage door | 12.2 m | 9.0 s |
| spawn → map board | 6.4 m | 4.7 s |

All neighbour pairs are ≤ 10 s. Not neighbours (for information): panda info board → food
storage 18.4 s; zebra board → panda board 11.7 s; hippo board → zebra board 16.5 s. The
panda mission is therefore the longest round trip — intentionally the third mission a child
will usually do.

## High-angle camera (Q-049 answered)

The game uses the high-angle follow camera of GAME-PLAYER §2 (pitch ≈ 55°, default ≈ 14 m,
zoom 10–20 m, rotation in 45° steps; user decision 2026-09-26). Effects on this level:

1. **Hiding places vs. own enclosure.** From above, the central grove and the hedges no
   longer block the view the way they do at eye level. The design works through
   **distance**: info board → own hiding place is 24.8 m (zebra → river), 21.6 m
   (hippo → pond), 28.7 m (panda → cave). Rule 7 / LAYOUT-L1-006 is therefore a screen
   test. Whether it holds depends on the FOV axis (Q-052).
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
2. The player spawns at cell (0, 2) facing +Z.
3. Every enclosure, building, landmark, barrier and hiding place is reachable from the
   spawn over walkable cells (hedges and walls need not be).
4. Every border cell of the level bounds `(-24, -2, 48, 50)` is solid; with all barriers
   closed no walkable cell outside the bounds can be reached.
5. No two solid elements overlap, and no path cell lies under a solid element.
6. Walking time between neighbouring points of interest (table above) is ≤ 10 s with
   1.4 m/s on paths and 0.98 m/s on grass.
7. For each level-1 mission, while the player stands at its info board or enclosure gate,
   the animal spot of its hiding place is **off-screen** for every allowed camera rotation
   (45° steps) and zoom (10–20 m) of GAME-PLAYER §2 — the animal cannot be seen from its
   own enclosure (Q-049; FOV definition Q-052).
8. Each hiding place's `features` contain all riddle details of CONT-MISSIONS for it; the
   river has a bridge within 4 m of the animal spot; no bridge and no river cell lies
   within the pond hiding place; level 1 contains no other water and no other bridge.
9. Each animal spot is within interaction range (2 m) of at least one walkable cell.
10. When the unlock condition of `barrier_ne_tree` is met, its cells become walkable; the
    other two barriers stay closed.
11. The food storage is open from the start (proposal Q-033).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| LAYOUT-L1-001 | Given `level-1.toml`, then every enclosure, building, landmark, barrier and hiding place has a walkable cell reachable from the spawn next to it or inside it (flood fill). | unit |
| LAYOUT-L1-002 | Given `level-1.toml` with all barriers closed, then every border cell of the bounds is solid and the flood fill from the spawn never leaves the bounds. | unit |
| LAYOUT-L1-003 | Given `level-1.toml`, then no two solid elements share a cell and no `path` cell is covered by a solid element. | unit |
| LAYOUT-L1-004 | Given this spec's element table and `level-1.toml`, then both list the same ids, types and rectangles (instance of LAYOUT-005). | unit |
| LAYOUT-L1-005 | Given `level-1.toml` with path speed 1.4 m/s and grass speed 0.98 m/s, then each neighbour pair of the walking-distance table has a fastest walking time ≤ 10 s. | unit |
| LAYOUT-L1-006 | Given missions `zebra`, `hippo`, `panda`, the player on each walkable cell next to the info board or the enclosure gate, and the camera of GAME-PLAYER §2 at every 45° rotation and at 10, 14 and 20 m distance on a 1080×2340 viewport, then the animal spot (0.5 m above ground) lies outside the view frustum (depends on Q-052). | unit |
| LAYOUT-L1-007 | Given the hiding places of level 1, then their `features` contain the CONT-MISSIONS details (river: flowing_water, bridge, ducks; pond: still_water, water_lilies, frogs; cave: dark, cool, stone, echo), a `bridge` path lies within 4 m of the river animal spot, and no bridge/river cell lies inside `loc_pond`. | unit |
| LAYOUT-L1-008 | Given `level-1.toml`, then the only water elements are `river_*` and `pond_water` and the only `bridge` is `bridge_river`. | unit |
| LAYOUT-L1-009 | Given each level-1 hiding place, then at least one walkable cell centre is within 2 m of its animal spot. | unit |
| LAYOUT-L1-010 | Given missions `zebra`, `hippo`, `panda` complete, then `barrier_ne_tree` cells are walkable, `barrier_north_gate` and `barrier_east_repair` are still solid (depends on Q-022). | unit |
| LAYOUT-L1-011 | Given level 1 starts on a 1080×2340 viewport, then the player stands on the plaza facing north and the food storage is on screen. | e2e |
| LAYOUT-L1-012 | Given the approved mockups `loc_river` and `loc_pond`, then a reviewer can tell river and pond apart without text (flow + bridge + ducks vs. still + lilies + frogs), and `env_zebra` shows no water, `env_panda` no stone cave. | manual |

## Open questions

- Q-022 barrier unlock conditions, Q-023 number of levels and their areas.
- Q-033 food storage lock (level 1 assumes unlocked).
- Q-044 `hiding_place` element type and `blocks_view` data.
- Q-046 answered: grass walkable, slower than paths.
- Q-049 answered: high-angle game camera (GAME-PLAYER §2; section above). Q-052 FOV axis (LAYOUT-L1-006, LAYOUT-L1-011).
- Q-047 which food boxes stand in the storage in level 1.
- Q-024 walking speed (distance limit assumes 1.4 m/s).
