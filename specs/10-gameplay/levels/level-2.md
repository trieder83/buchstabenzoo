---
id: GAME-LEVEL-2
title: Level 2 — koala, elephant, giraffe, lion (behind the fallen tree)
aspect: gameplay
module: levels
status: draft
depends_on: [GAME-LAYOUT, GAME-LEVEL-1, GAME-RESCUE, GAME-FAMILY, GAME-NIGHT, CONT-MISSIONS, GAME-PLAYER]
test_prefix: LAYOUT-L2
updated: 2026-09-28
---

# Level 2 — koala, elephant, giraffe, lion (behind the fallen tree)

Owned by the `zoo-level-designer` agent. Layout data: `assets/levels/level-2.toml`
(level id `level_2`). Coordinate system, element types, "Joining levels" and general rules:
GAME-LAYOUT. Look of each area: ART-ENVIRONMENT and the mockup briefs in `art/environment/`.

## Goal

The second day level (day and night levels alternate, GAME-NIGHT / Q-078): after level 1 is
complete, night falls; **the next morning** the zookeeper has sawn up the fallen tree
`barrier_ne_tree` (GAME-LEVEL-1) and the path behind the level-1 bridge leads into a new,
bigger part of the zoo with **four** rescue missions:

| Mission | Enclosure | Food box | Candidate hiding places (one is picked per playthrough) | What the riddles rely on (CONT-MISSIONS) |
|---|---|---|---|---|
| `koala` (**pair**, GAME-FAMILY) | `enc_koala` (west) | Eukalyptus / eucalyptus | `loc_treehouse` (east, between the stage and the east wall — FIX-056), `loc_tallest_tree` (east), `loc_blossom_tree` (north-east corner) | wooden house with window and rope ladder high up · the tallest tree of the zoo · pink blossoms, drifting petals, bees |
| `elephant` | `enc_elephant` (east) | Heu / hay | `loc_fountain` (south-west), `loc_log_pile` (north-west), `loc_big_ball` (north) | water jet, stone basin, coins · stacked tree trunks, sawdust · giant round red-and-white toy |
| `giraffe` | `enc_giraffe` (north) | Blätter / leaves | `loc_lookout_tower` (south-west), `loc_train` (south), `loc_playground` (south-east corner) | wooden tower with stairs, visitors high up · little train with a bell · slide and swings |
| `lion` | `enc_lion` (south) | Fleisch / meat | `loc_sun_rocks` (north-west corner), `loc_stage` (north-east), `loc_deckchairs` (north-east) | big flat warm rocks in full sun · round music stage with drums · striped deckchairs under a sunshade |

Plus the level entry (spawn), a picture map board and **food storage 2** (all 10 food boxes,
proposal Q-089). The next level (level 3) waits behind a construction fence.

**Design idea: the level-1 ring, one size bigger.** A ring path (18 × 18 m) runs around a
central block (food storage 2 + a dense grove). One enclosure sits on each side of the ring;
every animal's three candidate places lie in the **opposite** corners of the level, far
from its own board (22–38 m), so the child reads the riddle instead of spotting the animal.
Side paths lead from the ring into the four corners, where the landmarks of the hiding
places stand (tree house, fountain, tower, train, playground, giant tree, stage, deckchairs,
blossom tree, ball, log pile, flat rocks). Nothing of level 2 needs the level-1 food storage.

**Discovery** (GAME-RESCUE §1): 3 candidates per animal, 3⁴ = 81 combinations, of which
**48** keep all four chosen spots ≥ 12 m apart; every candidate is part of ≥ 9 valid
combinations (picking rule Q-082: re-draw until valid).

## Proposals used in this level (not yet decided)

| Topic | Proposal used here | Question |
|---|---|---|
| Joining levels | One continuous map: level 2 uses the level-1 coordinate system, its bounds touch level 1 at the fallen tree; `[[entry]]` lists the level cells behind the barrier. The engine joins all unlocked levels into one grid (GAME-LAYOUT "Joining levels"). | Q-088 |
| When the barrier opens | `barrier_ne_tree` opens on the **morning after** level 1 is complete (after night 1 / the night zoo, GAME-NIGHT); `barrier_l2_construction` likewise on the morning after level 2 is complete. | Q-022, Q-091 |
| Own food storage | `food_storage_2` with all 10 boxes (same order as level 1) at the level entry; the level-1 storage stays usable. | Q-089, Q-047 |
| Koalas up in the tree | At the three koala places the pair sits **up in the tree / tree house** (`perch_height_m` 3.0–9.0 m) and does not wander on the ground; the player interacts from the ground within 2 m of the spot; shown eucalyptus, the pair climbs down and follows. The ground wander area stays in the data as fallback. | Q-094, Q-043 |
| Hiding places and scenery | `[[hiding_place]]` / `[[scenery]]` as in level 1. | Q-080 |
| New hiding places | The seven single places of the old CONT-MISSIONS draft are replaced by 3 candidates each; `loc_mud_pool` is dropped (clash with the hippo's `loc_mud`, Q-083 c), `loc_fountain` moves from the goldfish to the elephant, `loc_playground` keeps the giraffe with the `kiga` word *Spielplatz* (Q-039). | Q-095 |
| Enclosure features | `elephant_pool` as `[[enclosure_feature]]` (same data shape as `hippo_pool`). | Q-085 |
| Tree areas | Centre grove `dense` (solid, blocks view). | Q-085 |
| 22 m haze rule (FIX-056) | `board_elephant` moved south of its gate (54, 31); `loc_big_ball`, `loc_fountain`, `loc_lookout_tower`, `loc_stage` clipped on the side facing their board; `loc_treehouse` and its oak `treehouse_e` moved from the south-west corner to the east wall (70, 44) (see "Hiding places"). | Q-145 |

## Spawn and camera

- Spawn cell **(27, 29)** on `path_l2_entry`, 3 m behind the opened fallen tree, facing **+x
  (east)**; camera behind the player looking east (GAME-PLAYER §2). Used when a save starts
  or continues in level 2 and after the morning cut-scene (the zookeeper waves, the logs are
  carried away on the cart, the path is free).
- First view: the entry path straight ahead with the food storage 2 at its end (9 m), the
  map board and the koala enclosure on the left, the fountain on the right.
- Walking back west through the opened tree leads to the level-1 bridge (the whole zoo stays
  walkable, GAME-LAYOUT "Joining levels").

## Map

Scale **1 character = 1 m** (one grid cell). North (+Z) is up; row labels are `z`, the x axis
is labelled below. Generated from `assets/levels/level-2.toml`. The fallen tree
`barrier_ne_tree` (level 1) lies directly west of x 24, z 28–30.

```
   61 %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%##
   60 %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%##
   59 %%................................................##
   58 %%..sssssss...............oo.6............,,,,....##
   57 %%..sssasss...............oo......CCCC....,,,,....##
   56 %%..sssssss.......................CCCC....,,,,BB..##
   55 %%..sssssss.....jjjjjjjjjjjj........c.....,,,3BB..##
   54 XX=============.jjjjjjjjjjjj..............,,,,....##
   53 XX=============.jjjjjjjjjjjj..............,,,,....##
   52 XX=============.jjjjjjjjjjjj..............,,,,....##
   51 %%..........===.jjjjjjjjjjjj===============.......##
   50 %%..........===.jjjjjjjjjjjj===============.......##
   49 %%..........===.jjjjjjjjjjjj===============.......##
   48 %%...5......===.jjjjjjjjjjjj===...................##
   47 %%..........===.jjjjjjjjjjjj===...................##
   46 %%.LLLL.....===.jjjjjjjjjjjj===...QQQQ.........HHH##
   45 %%.LLLL.....===.jjjjjjjjjjjj===..bQQQQ.........HHH##
   44 %%..........===.jjjjjggjjjjj===...QQQQ........1HHH##
   43 %%..........===.............===...QQQQ............##
   42 %%.kkkkkkkk.===....i........===...................##
   41 %%.kkkkkkkk.==================....................##
   40 %%.kkkkkkkk.==================.eeeeeeeeeeee.......##
   39 %%.kkkkkkkki==================.eeeeeeeeeeee.......##
   38 %%.kkkkkkkk.===TTTTTTTTTTTT===.eeeeeeeeeeee.......##
   37 %%.kkkkkkkg.===TTTTTTTTTTTT===.eeeeeeeeeeee.......##
   36 %%.kkkkkkkg.===TTTTTTTTTTTT===.eeeeeeeeeeee.......##
   35 %%.kkkkkkkk.===TTTTTTTTTTTT===.eeeeeeeeeeee.......##
   34 %%.kkkkkkkk.===FFFFFFTTTTTT===.geeeeeeeeeee.......##
   33 %%.kkkkkkkk.===FFFFFFTTTTTT===.geeeeeeeeeee.......##
   32 %%......M...===FFFFFFTTTTTT===.eeeeeeeeeeee..2GG..##
   31 %%......M...===FFFFFFTTTTTT===ieeeeeeeeeeee...GG..##
   30 ===============DFFFFFTTTTTT===.eeeeeeeeeeee===....##
   29 ===S===========FFFFFFTTTTTT===.eeeeeeeeeeee===....##
   28 ===============FFFFFFTTTTTT===.eeeeeeeeeeee===....##
   27 %%.......======FFFFFFTTTTTT===.............===....##
   26 %%.OOO...=======YY============================....##
   25 %%.OOO4..========y============================....##
   24 %%.OOO...=====================================....##
   23 %%.......===........i.........==..................##
   22 %%.......===..................==..................##
   21 %%.......===.....lllllgglllll.==..................##
   20 %%.......===.....llllllllllll.==..................##
   19 %%.......===.....llllllllllll.==..................##
   18 %%.......===.....llllllllllll.==..................##
   17 %%.......===WWW7.llllllllllll.==..........PP......##
   16 %%.......===WWW..llllllllllll....8........PP9PPPP.##
   15 %%..........WWW..llllllllllll.ZZZZZZZZ....PP.PPPP.##
   14 %%...............llllllllllll.ZZZZZZZZ............##
   13 %%##################################################
   12 %%##################################################
      |.......|.......|.......|.......|.......|.......|...
 x:   24      32      40      48      56      64      72
```

| Char | Meaning | Char | Meaning |
|---|---|---|---|
| `#` | outer zoo wall (`boundary`) | `%` | tall hedge (level edge, solid) |
| `X` | barrier `barrier_l2_construction` (→ level 3) | `=` | path |
| `F` / `D` | food storage 2 / its door | `T` | dense grove (`decoration`, solid) |
| `k` `e` `l` `j` | enclosure koala / elephant / lion / giraffe | `g` | enclosure gate (with enclosure sign) |
| `i` | info board | `M` | map board |
| `H` | tree house (`treehouse_e`) | `G` | giant tree (`tree_giant_e`) |
| `B` | blossom tree (`tree_blossom_ne`) | `O` | fountain (`fountain_sw`) |
| `L` | log pile (`log_pile_nw`) | `o` | giant play ball (`ball_n`) |
| `W` | lookout tower (`tower_sw`) | `Z` | zoo train at its station (`train_se`) |
| `P` | playground slide and swings | `Q` | music stage (`stage_ne`) |
| `C` | deckchairs and sunshade (`deckchairs_ne`) | `,` | pink petal carpet (`petals_ne`, walkable) |
| `s` | flat rocks (`flat_rocks_nw`, walkable) | `S` | spawn |
| `1` `2` `3` | animal spot koala: `loc_treehouse`, `loc_tallest_tree`, `loc_blossom_tree` | `4` `5` `6` | animal spot elephant: `loc_fountain`, `loc_log_pile`, `loc_big_ball` |
| `7` `8` `9` | animal spot giraffe: `loc_lookout_tower`, `loc_train`, `loc_playground` | `a` `b` `c` | animal spot lion: `loc_sun_rocks`, `loc_stage`, `loc_deckchairs` |
| `.` | grass (walkable, slower — Q-046) | `Y` / `y` | bed `bed_l2` (`[[item]]`, solid) / its stand cell (Q-141) |

## Elements

Grid rect = `x, z, w, d` in 1 m cells (south-west corner + size), as in GAME-LAYOUT. Solid =
every type except `path` and `hiding_place`. The level has no legacy `[[element]] type =
"hiding_place"` mirrors; its hiding places exist only in `[[hiding_place]]` (Q-080).

| Id | Type | Grid rect (x, z, w, d) | Notes |
|---|---|---|---|
| `hedge_l2_w_a` | decoration (hedge) | 24, 12, 2, 16 | Level edge south of the entry (behind it: level-1 hedge_east_b / river grate). |
| `hedge_l2_w_b` | decoration (hedge) | 24, 31, 2, 21 |  |
| `barrier_l2_construction` | barrier (construction_fence) | 24, 52, 2, 3 | Construction fence of the new adventure playground (level 3): striped fence panels, a sign with a digger/shovel icon, a small digger and a zookeeper. Next morning: the fence is gone, a ribbon and balloons mark the opening. |
| `hedge_l2_w_c` | decoration (hedge) | 24, 55, 2, 7 |  |
| `wall_l2_south` | boundary (zoo_wall) | 26, 12, 50, 2 | Outer zoo wall (south-east part of the zoo), permanent. |
| `wall_l2_east` | boundary (zoo_wall) | 74, 14, 2, 48 | Outer zoo wall, permanent. |
| `hedge_l2_north` | decoration (hedge) | 26, 60, 48, 2 | Level edge, permanent. |
| `path_l2_entry` | path (main) | 24, 28, 12, 3 | From the opened fallen tree (barrier_ne_tree, level 1) east to the ring. First cells = level entry. |
| `path_l2_ring_s` | path (main) | 36, 24, 18, 3 |  |
| `path_l2_ring_w` | path (main) | 36, 27, 3, 12 |  |
| `path_l2_ring_e` | path (main) | 51, 27, 3, 12 |  |
| `path_l2_ring_n` | path (main) | 36, 39, 18, 3 |  |
| `path_l2_sw` | path (side) | 33, 16, 3, 12 | Side path south from the entry path to the playground and the fountain. |
| `path_l2_se` | path (side) | 54, 24, 16, 3 | Side path east from the ring to the zoo train station and the lookout tower. |
| `path_l2_ne` | path (side) | 52, 42, 3, 10 | Side path north from the ring past the giraffe enclosure to the music stage. |
| `path_l2_ne_e` | path (side) | 55, 49, 12, 3 | Side path east to the deckchairs and the blossom tree. |
| `path_l2_station` | path (side) | 54, 17, 2, 7 | Narrow platform path from path_l2_se down to the zoo train station. |
| `path_l2_e` | path (side) | 67, 27, 3, 4 | Short path from path_l2_se north to the giant tree east of the elephant enclosure. |
| `path_l2_north` | path (side) | 36, 42, 3, 10 | Side path north from the ring to the construction fence (level 3). |
| `path_l2_nw` | path (side) | 24, 52, 15, 3 | Leads west to barrier_l2_construction and runs on under it (x 24…25) to the level-3 gate and `path_l3_entry`: the construction fence stands on the street (LAYOUT-040). |
| `food_storage_2` | building (food_storage) | 39, 27, 6, 8 | door at cell (39, 30); enterable, walkable interior (40, 28, 4, 6). Second food storage (proposal Q-089): all 10 food boxes, row in front of the west facade facing the arriving child (Q-181 answered); 6 more real labelled food boxes inside (Q-194 answered 2026-09-29). |
| `grove_l2_center` | decoration (tree_grove) | 45, 27, 6, 12 | density `dense`.  |
| `trees_l2_center_n` | decoration (trees) | 39, 35, 6, 4 | density `dense`.  |
| `enc_koala` | enclosure | 27, 33, 8, 10 | gate (34, 36, 1, 2). Two eucalyptus trees (medium height, grey-green leaves), climbing trunk with forks, small wooden shelter, feeding trough (GAME-FAMILY). No tree taller than the others, no blossoms, no tree house (riddle guards). |
| `board_koala` | decoration (info_board) | 35, 39, 1, 1 | info board of `enc_koala`.  |
| `enc_elephant` | enclosure | 55, 28, 12, 13 | gate (55, 33, 1, 2). Elephant pool (still water, stones, a shallow ramp — like hippo_pool), hay rack, sand-coloured ground patch, one big boulder. No fountain, no logs, no ball (riddle guards). |
| `board_elephant` | decoration (info_board) | 54, 31, 1, 1 | info board of `enc_elephant`, just south of its gate cells; moved from 54, 36 by FIX-056 so that `loc_big_ball` stays ≥ 22 m from its standing points. |
| `enc_lion` | enclosure | 41, 14, 12, 8 | gate (46, 21, 2, 1). Wooden sun deck with a straw roof, a big lying log, dry grass. NO flat rocks, no stage, no chairs (riddle guards). |
| `board_lion` | decoration (info_board) | 44, 23, 1, 1 | info board of `enc_lion`.  |
| `enc_giraffe` | enclosure | 40, 44, 12, 12 | gate (45, 44, 2, 1). Tall feeding rack with leafy branches (4 m), giraffe house with a tall door. No tower, no slide, no train (riddle guards). |
| `board_giraffe` | decoration (info_board) | 43, 42, 1, 1 | info board of `enc_giraffe`.  |
| `map_board_l2` | landmark (map_board) | 32, 31, 1, 2 | Picture map at the level entry (opens the map, GAME-MAP). |
| `treehouse_e` | decoration (treehouse) | 71, 44, 3, 3 | Old oak with a wooden tree house (roof, window, rope ladder) at the east wall between the music stage and the wall. The only tree house in the zoo (loc_treehouse). Moved from the south-west corner (28, 15) and renamed from `treehouse_sw` by FIX-056 (22 m haze rule). |
| `tree_giant_e` | decoration (giant_tree) | 70, 31, 2, 2 | The tallest tree of the whole zoo (12 m, twice as tall as every other tree; levels 1-3 have no tree above 7 m). loc_tallest_tree. |
| `tree_blossom_ne` | decoration (blossom_tree) | 70, 55, 2, 2 | Tree full of pink blossoms; pink petals on the ground (petals_ne). The only blossoming tree in the zoo (loc_blossom_tree). |
| `fountain_sw` | landmark (fountain) | 27, 24, 3, 3 | Round stone basin with a water jet in the middle, coins shining on the bottom. The only fountain in the zoo (loc_fountain). |
| `log_pile_nw` | decoration (log_pile) | 27, 45, 4, 2 | Stacked logs of the fallen tree (barrier_ne_tree), sawdust around. The only log pile in the zoo (loc_log_pile). |
| `ball_n` | decoration (play_ball) | 50, 57, 2, 2 | Giant red-and-white play ball (1.8 m) for the elephants (enrichment toy). loc_big_ball. |
| `tower_sw` | landmark (lookout_tower) | 36, 15, 3, 3 | Wooden lookout tower for visitors: stairs, platform at 4 m with a railing and a small roof. loc_lookout_tower. |
| `train_se` | landmark (zoo_train) | 54, 14, 8, 2 | Little zoo train (locomotive with chimney and bell + 2 open wagons) standing on a short piece of track at a small station platform along the south wall. loc_train. |
| `playground_se_slide` | decoration (slide) | 66, 15, 2, 3 | Playground slide (1.8 m). loc_playground. |
| `playground_se_swings` | decoration (swings) | 69, 15, 4, 2 | Double swing. loc_playground. |
| `stage_ne` | landmark (stage) | 58, 43, 4, 4 | Round wooden music stage (0.5 m high) with a pointed roof, drums and a xylophone on it. loc_stage. |
| `deckchairs_ne` | decoration (deckchairs) | 58, 56, 4, 2 | Three striped deckchairs and a big sunshade. loc_deckchairs. |

## Level entry and exit

| Id | Cells (x, z, w, d) | Joins | Opens when |
|---|---|---|---|
| `entry_l2_west` (`[[entry]]`) | 24, 28, 1, 3 | level-1 `barrier_ne_tree` (22, 28, 2, 3) — the level-1 `path_ne` continues as `path_l2_entry` | `barrier_ne_tree` opens (GAME-LEVEL-1; morning after level 1, Q-091) |
| `barrier_l2_construction` (exit) | 24, 52, 2, 3 | level-3 `entry_l3_east` (23, 52, 1, 3) | missions `koala`, `elephant`, `giraffe`, `lion` complete, then the next morning (Q-022, Q-091) |

## Hiding places (candidates)

Data: `[[hiding_place]]` in `level-2.toml` (fields as in level 1, Q-080; `perch_height_m`
proposal Q-094). Riddle keys `mission-<animal>-riddle-<id>-<reading_level>`
(CONT-MISSIONS §4–§9). Every `rect` is the bounding box of the place's wander area and its
scenery (generated).

| Id | Animal | Area rect (x, z, w, d) | Animal spot | Wander on | Wander cells | Perch | Features (riddle details) | Scenery | Spot → own info board |
|---|---|---|---|---|---|---|---|---|---|
| `loc_treehouse` | koala | 67, 42, 7, 6 | (70, 44) | grass | 21 | 3.5 m | tree_house, rope_ladder, wooden_roof, window | `treehouse_e` | 35.4 m |
| `loc_tallest_tree` | koala | 67, 31, 5, 5 | (69, 32) | grass | 17 | 9.0 m | tallest_tree, thick_trunk, crown_above_all_trees | `tree_giant_e` | 34.7 m |
| `loc_blossom_tree` | koala | 66, 52, 6, 7 | (69, 55) | grass | 24 | 3.0 m | pink_blossoms, falling_petals, bees | `tree_blossom_ne`, `petals_ne` | 37.6 m |
| `loc_fountain` | elephant | 28, 22, 3, 6 | (30, 25) | grass | 10 | — | water_jet, stone_basin, coins, splashing | `fountain_sw` | 24.7 m |
| `loc_log_pile` | elephant | 26, 46, 7, 6 | (29, 48) | grass | 24 | — | stacked_logs, bark, sawdust | `log_pile_nw` | 30.2 m |
| `loc_big_ball` | elephant | 51, 57, 6, 3 | (53, 58) | grass | 14 | — | giant_ball, red_white, round | `ball_n` | 27.0 m |
| `loc_lookout_tower` | giraffe | 37, 14, 4, 5 | (39, 17) | grass | 11 | — | wooden_tower, stairs, high_platform, visitors_look_down | `tower_sw` | 25.3 m |
| `loc_train` | giraffe | 54, 16, 7, 4 | (57, 16) | grass | 16 | — | train, locomotive, wagons, rails, bell | `train_se` | 29.5 m |
| `loc_playground` | giraffe | 66, 14, 5, 6 | (68, 16) | grass | 16 | — | slide, swings | `playground_se_slide`, `playground_se_swings` | 36.1 m |
| `loc_sun_rocks` | lion | 28, 55, 7, 5 | (31, 57) | grass | 27 | — | flat_rocks, warm, full_sun, no_shade | `flat_rocks_nw` | 36.4 m |
| `loc_stage` | lion | 55, 45, 5, 4 | (57, 45) | grass | 12 | — | stage, pointed_roof, drums, xylophone | `stage_ne` | 25.6 m |
| `loc_deckchairs` | lion | 57, 52, 7, 6 | (60, 55) | grass | 20 | — | striped_deckchairs, sunshade | `deckchairs_ne` | 35.8 m |

**Wander area** as in GAME-LEVEL-1 ("Hiding places"): cells within 3 m of the spot, on the
`wander_on` surface, 4-connected to the spot, never a path cell. All 12 have ≥ 10 cells and a
cell ≥ 2 m from the spot; wander areas and rects of different animals are disjoint.

**Haze rule (22 m, FIX-056).** Every wander cell and spot is ≥ 22 m (planar) from every standing
point of its own animal (walkable cells ≤ 2.5 m from the own info board, cells around the own
gate; GAME-LAYOUT "Sight", CAMV-008, close-view fog end 20.8 m). Changes of 2026-09-27:
`board_elephant` moved from (54, 36) to (54, 31) (south of the gate, facing west as before);
`loc_big_ball` clipped to z 57–59 (rect was 51, 55, 6, 5; 20 → 14 cells); `loc_fountain`
clipped to x 28–30 (rect was 28, 22, 5, 6; 20 → 10 cells — the new board stands 2 m further
south); `loc_lookout_tower` clipped to z 14–18 (was 37, 14, 4, 7; 16 → 11 cells); `loc_stage`
clipped to z 45–48 (was 55, 42, 5, 7; 19 → 12 cells); `loc_treehouse` moved with its oak from
the south-west corner (rect 26, 14, 4, 5, spot (27, 15); only 4 cells there were ≥ 22 m from
the koala gate) to the east wall between the stage and the wall (rect 67, 42, 7, 6 — clear of
the burglar hideout z 37–41 —, spot (70, 44); 11 → 21 cells). Smallest distances now:
`loc_big_ball` 22.0 m, `loc_fountain` 22.2 m, `loc_lookout_tower` 22.1 m, `loc_stage` 22.4 m,
the others ≥ 24.7 m.

**Spread (RESC-014).** Straight distances between spots of different animals (m); 57 of 81
combinations have all four spots pairwise ≥ 12 m apart (48 before FIX-056), every candidate is in ≥ 12 of them:

| | `loc_treehouse` | `loc_tallest_tree` | `loc_blossom_tree` | `loc_fountain` | `loc_log_pile` | `loc_big_ball` | `loc_lookout_tower` | `loc_train` | `loc_playground` | `loc_sun_rocks` | `loc_stage` | `loc_deckchairs` |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `loc_treehouse` | — | — | — | 44.3 | 41.2 | 22.0 | 41.1 | 30.9 | 28.1 | 41.1 | 13.0 | 14.9 |
| `loc_tallest_tree` | — | — | — | 39.6 | 43.1 | 30.5 | 33.5 | 20.0 | 16.0 | 45.5 | 17.7 | 24.7 |
| `loc_blossom_tree` | — | — | — | 49.2 | 40.6 | 16.3 | 48.4 | 40.8 | 39.0 | 38.1 | 15.6 | 9.0 |
| `loc_fountain` | 44.3 | 39.6 | 49.2 | — | — | — | 12.0 | 28.5 | 39.1 | 32.0 | 33.6 | 42.4 |
| `loc_log_pile` | 41.2 | 43.1 | 40.6 | — | — | — | 32.6 | 42.5 | 50.4 | 9.2 | 28.2 | 31.8 |
| `loc_big_ball` | 22.0 | 30.5 | 16.3 | — | — | — | 43.3 | 42.2 | 44.6 | 22.0 | 13.6 | 7.6 |
| `loc_lookout_tower` | 41.1 | 33.5 | 48.4 | 12.0 | 32.6 | 43.3 | — | — | — | 40.8 | 33.3 | 43.4 |
| `loc_train` | 30.9 | 20.0 | 40.8 | 28.5 | 42.5 | 42.2 | — | — | — | 48.5 | 29.0 | 39.1 |
| `loc_playground` | 28.1 | 16.0 | 39.0 | 39.1 | 50.4 | 44.6 | — | — | — | 55.2 | 31.0 | 39.8 |
| `loc_sun_rocks` | 41.1 | 45.5 | 38.1 | 32.0 | 9.2 | 22.0 | 40.8 | 48.5 | 55.2 | — | — | — |
| `loc_stage` | 13.0 | 17.7 | 15.6 | 33.6 | 28.2 | 13.6 | 33.3 | 29.0 | 31.0 | — | — | — |
| `loc_deckchairs` | 14.9 | 24.7 | 9.0 | 42.4 | 31.8 | 7.6 | 43.4 | 39.1 | 39.8 | — | — | — |

**Sight test (LAYOUT-L2-006).** For every candidate, no cell of its wander area is on screen
while the player stands on any walkable cell next to its own info board or gate, for all
8 camera rotations and zoom 10 / 14 / 20 m, 35° vertical FOV, 1080×2340 portrait — tested at
0.5 m **and at the animal's height** (koala 0.9 m, elephant 3.0 m, giraffe 4.5 m, lion 1.5 m)
and, for perches, at perch height + 1 m (koala up to 10 m). All 12 pass; for information all
12 are also off-screen in 2340×1080 landscape (the ball and the tree house were moved for this).

**Nearest neighbour of each place** (fastest walk, 1.93 m/s path / 1.45 m/s grass, ≤ 10 s):
`loc_treehouse` → loc_blossom_tree 7.6 s (FIX-056; was → loc_lookout_tower from the old south-west corner); `loc_fountain` → spawn 2.3 s; `loc_lookout_tower` → board_lion 7.0 s; `loc_train` → board_lion 9.3 s; `loc_playground` → loc_tallest_tree 9.2 s; `loc_tallest_tree` → loc_playground 9.2 s; `loc_blossom_tree` → loc_deckchairs 5.1 s; `loc_stage` → board_giraffe 7.4 s; `loc_deckchairs` → loc_stage 5.4 s; `loc_big_ball` → loc_deckchairs 4.9 s; `loc_sun_rocks` → construction 3.2 s; `loc_log_pile` → construction 3.0 s.

## Hiding places — riddle details and guards

| Hiding place | Riddle details and how the layout provides them |
|---|---|
| `loc_treehouse` (koala) | Old oak `treehouse_e` (east wall, north of the elephant enclosure) with a wooden **tree house** (roof, round window) at 3.5 m and a **rope ladder**; the koala pair sits on its little porch. |
| `loc_tallest_tree` (koala) | `tree_giant_e`, **12 m** — twice as tall as any other tree in levels 1–3 (all others ≤ 7 m, riddle guard); trunk 2 m thick; the pair clings to the top branches at ≈ 9 m. East of the elephant enclosure, reached by `path_l2_e`. |
| `loc_blossom_tree` (koala) | `tree_blossom_ne` covered in **pink blossoms**, **petals** drifting down onto `petals_ne`, 3–4 **bees** around the crown. The only pink tree of the zoo. |
| `loc_fountain` (elephant) | `fountain_sw`: round **stone basin** with a **water jet** in the middle and **coins** glinting on the bottom; the elephant drinks and showers with its trunk. The only fountain; no jet in the elephant pool. |
| `loc_log_pile` (elephant) | `log_pile_nw`: **thick tree trunks stacked** in a neat pile (the sawn-up fallen tree of level 1) and **sawdust** on the grass. Riddle guard: removing `barrier_ne_tree` leaves **no logs** at level 1 (the zookeeper carts them away; GAME-LEVEL-1 barrier animation must not leave a pile — to be confirmed with the level-1 owner). |
| `loc_big_ball` (elephant) | `ball_n`: a **giant red-and-white ball** (1.8 m, taller than the girl) on the lawn north of the giraffe enclosure; the elephant nudges it. |
| `loc_lookout_tower` (giraffe) | `tower_sw`: wooden **tower** with **stairs** and a roofed **platform** at 4 m; the giraffe's head is level with the platform (giraffe 4.5 m). |
| `loc_train` (giraffe) | `train_se`: little **zoo train** (engine with chimney and **bell** + 2 open wagons) on a short track at a **station** platform (`path_l2_station`). |
| `loc_playground` (giraffe) | `playground_se_slide` + `playground_se_swings`: **slide** (1.8 m) and double **swing**; the giraffe towers over them. No sandpit (Q-083: sand belongs to the zebra's `loc_sand`). The only slide and swings of the zoo (the level-3 pirate ship has **no slide**). |
| `loc_sun_rocks` (lion) | `flat_rocks_nw`: big **flat, light-grey slabs** flush with the grass in the sunniest corner, **no tree, no shade**. The lion enclosure has **no rocks** (wooden sun deck instead) so the riddle cannot point home; must not look like the level-1 rock hill (tall, cave). |
| `loc_stage` (lion) | `stage_ne`: round wooden **music stage** (0.5 m high) with a **pointed roof**, **drums** and a xylophone on it; the lion lies in front and roars. |
| `loc_deckchairs` (lion) | `deckchairs_ne`: three **striped deckchairs** under a big **sunshade**; the lion sprawls next to them. The only deckchairs/sunshade in levels 1–2. |

Riddle guards (a riddle points to exactly one place in the **whole zoo**, Q-083):
- Only one fountain, one tree house, one giant tree, one blossoming tree, one ball, one log
  pile, one tower, one train, one slide/swing set, one stage, one set of deckchairs, one
  patch of flat rocks in the joined levels 1–2 (and 1–3).
- Enclosure guards: `enc_koala` has eucalyptus trees of normal height, no blossoms, no house;
  `enc_elephant` has a pool without jet or coins, no logs, no ball; `enc_giraffe` has a tall
  feeding rack, no tower, no slide, no train; `enc_lion` has a wooden sun deck and a lying
  log, **no flat rocks**, no stage, no chairs.
- Riddles of one animal never share their key detail (koala: house up high / highest tree /
  pink flowers; elephant: water jet and coins / stacked wood / big round toy; giraffe: people
  up high on a tower / engine and bell / slide and swings; lion: warm stones / music and drums
  / striped chairs and sunshade) — MISS-007 checks the place words.

## Barriers

| Id | Kind | Cells (x, z, w, d) | Unlock condition — **proposal (Q-022, Q-091)** | In-world explanation |
|---|---|---|---|---|
| `barrier_l2_construction` | construction fence | 24, 52, 2, 3 | Level 2 → level 3: missions `koala`, `elephant`, `giraffe`, `lion` complete (all in `in_enclosure`), level 2's night has passed and the next morning has started (Q-141 a: until a `night_2` exists). | Striped construction fence panels with a sign showing a digger/shovel icon (no text), a small yellow digger and a zookeeper behind it: "the new adventure playground is being built". On the morning it opens, the fence is gone and a ribbon with balloons marks the opening. |

The west edge towards level 1 is a tall hedge except the three entry cells; south and east
are the outer zoo wall; north is a tall hedge (outside the zoo's planned area). All border
cells are solid except `entry_l2_west`.

## Walking distances

Fastest walking time (8-neighbour, no corner cutting, **1.93 m/s on paths, 1.45 m/s on
grass** — GAME-PLAYER §6), measured from the spawn, the cell in front of the storage door
(38, 30), the cells next to each info board, the map board and the barrier, and the cells
within 2 m of each animal spot.

| From → to | Fastest time |
|---|---|
| spawn → map_board_l2 | 2.8 s |
| spawn → storage_door | 5.9 s |
| storage_door → board_koala | 5.1 s |
| storage_door → board_lion | 5.6 s |
| board_lion → board_elephant | 7.1 s |
| board_elephant → board_giraffe | 9.1 s |
| board_giraffe → board_koala | 4.1 s |
| board_koala → loc_log_pile | 8.9 s |

All neighbour pairs are ≤ 10 s (longest: elephant board → giraffe board 9.1 s, since the
elephant board stands south of its gate — FIX-056). Each hiding place is ≤ 10 s from its neighbour (list above).

## Food storage 2 (proposal Q-089)

`food_storage_2` (39, 27, 6, 8) stands at the end of the entry path, door (39, 30) on the
west facade. It is **enterable** like every building with a door (GAME-LAYOUT): `interior`
(40, 28, 4, 6) is walkable floor. The 10 food boxes (`[[food_box]]`, same order as level 1:
melons, hay, bananas, bamboo, grass, meat, leaves, fish food, berries, eucalyptus from south to
north) stay **outside** (Q-181 answered 2026-09-28) in a row in front of the west facade (box
centres x = 38.66, labels facing west) on the east row of `path_l2_ring_w`; 2 m of the ring stay
free. The door (z 30…31) keeps a free gap with ≥ 0.9 m beside each post (Q-150, LAYOUT-038,
since the door is enterable): bamboo z 26.6, melons 27.4, hay 28.2, bananas 28.9, gap (box edges
29.21 … 31.74), meat 32.05, leaves 32.77, fish food 33.49, berries 34.21, eucalyptus 34.93,
grass 35.65. **Inside** (Q-194 answered 2026-09-29): 6 more real, labelled food boxes (interact
→ label panel → take, same as outside) on the plank platform of the wall band, labels facing
the free floor — the level's four animals' foods, repeated: back (east) wall hay (44.44, 32.48),
leaves (44.44, 33.12), eucalyptus (44.44, 29.08), meat (44.44, 29.72); north wall hay (42.0,
34.44); south wall leaves (41.0, 27.56).

## Bed of level 2 (Q-141 answered, option b)

After level 2's last mission night falls (GAME-NIGHT); the 🛏 icon offers the **nearest
unlocked bed** (GAME-NIGHT rule 3), so a child in level 2 does not have to walk back to
`zookeeper_house_1` (x −14…−9). Level 2 gets its own bed as an `[[item]]` (not an element —
same data shape as level-1 `bed_l1`; items are not in the element table / LAYOUT-005):

| Id | Kind | Centre (x, z) | Footprint (x, z) | Facing | Stand cell | Notes |
|---|---|---|---|---|---|---|
| `bed_l2` | bed (`kit_bedroom` `bed` model, 2.0 × 1.0 m) | 42.0, 26.5 | 41.0–43.0, 26.0–27.0 (cells (41, 26), (42, 26)) | `-z` (used from its long south side, headboard east, yaw 0) | (42, 25), centre (42.5, 25.5), 1.1 m in front | Outdoors against the south wall of `food_storage_2`, on the north row of `path_l2_ring_s`. |

Why here: the storage is surrounded by paths and solid trees (no free grass beside it), and
the south facade has no door. The bed leans against the windowless storage wall (sheltered
on one side, visible from the ring), 3 m from the storage's south-west corner and the food
boxes; `path_l2_ring_s` keeps a 2 m free lane south of it (z 24–25). It is no path junction
(ring corners are x 36–38 and 51–53), blocks no door or gate (storage door (39, 30) on the
west facade, lion gate (46–47, 21), `board_lion` (44, 23) keeps its reading cells), and lies
in no hiding-place rect, wander area, scenery or lantern position. No riddle scenery: a bed
is not a feature of any level-2 hiding place.

**Proposal (Q-152):** give the bed a small shelter so it reads as "the zookeeper's night
camp" and not as furniture left on the path — e.g. a striped canvas awning / lean-to roof
fixed to the storage's south wall above the bed (≈ 2.4 × 1.4 m, eaves at ≈ 2.0 m) and a
lantern on a hook. No model id exists for it yet; until decided the bed stands without a
roof.

## Elephant pool (user decision 2026-09-26: pools where animals need water)

`elephant_pool` (`[[enclosure_feature]]`, same shape as `hippo_pool`): 7 × 8 m tiled pool in
the east part of `enc_elephant` (59, 31, 7, 8), a shallow ramp on its west side (59, 33, 1, 3)
facing the gate, 39 % of the enclosure cells; the elephant wanders in and out of it when home
(`home_wander_on = ["grass", "water"]`). No jet and no coins (must not look like the fountain).

## High-angle camera

- Tall animals and perches are tested at their height (see "Sight test"): the giraffe's head
  (4.5 m) and the koalas at the top of the giant tree (≈ 9–10 m) project much farther up the
  screen than their feet; every place is still off-screen from its own board.
- The giant tree (12 m), the tower (5 m) and the stage roof fade when they are between the
  camera and the player (occluder fade, GAME-PLAYER §2).
- The map board and info boards are solid billboards (GAME-LAYOUT "Forests", LAYOUT-017).

## Behaviour

1. Level 2 contains exactly the elements of the table above; `assets/levels/level-2.toml`
   mirrors it (LAYOUT-005).
2. Level 2 uses the level coordinates of level 1; its bounds `(24, 12, 52, 50)` do not
   overlap level 1; its only walkable border cells are the `[[entry]]` cells, each
   edge-adjacent to a cell of `barrier_ne_tree` (GAME-LAYOUT "Joining levels").
3. Every enclosure, building, landmark, barrier and hiding place of level 2 is reachable from
   the spawn over walkable cells; with `barrier_l2_construction` closed no cell of level 3
   is reachable, and while `barrier_ne_tree` is closed no cell of level 2 is reachable from
   level 1.
4. No two solid elements overlap, and no path cell lies under a solid element.
5. Neighbouring points of interest (table above) and each hiding place's neighbour are ≤ 10 s
   apart.
6. For each mission and each candidate hiding place, while the player stands next to its info
   board or gate, the wander area (at 0.5 m and at the animal's height, perches at perch
   height + 1 m) is off-screen for every camera rotation and zoom (portrait 1080×2340).
7. Each hiding place's `features` contain its riddle details; each landmark kind a riddle
   relies on exists once in the joined levels 1–2; the enclosures contain none of their own
   animal's hiding-place features.
8. Each animal spot is within 2 m of a walkable cell.
9. Every candidate has a wander area of ≥ 9 cells inside its rect; wander areas and rects of
   different animals are disjoint; every candidate is part of a combination with all four
   spots ≥ 12 m apart.
10. The koala pair (GAME-FAMILY) starts together at the chosen koala place; both animals use
    the same perch/wander area.
11. The 10 food boxes of food storage 2 are props (not elements), each with a walkable
    standing cell within 2 m in front, reachable from the spawn.
12. `barrier_l2_construction` opens only under its unlock condition; `barrier_ne_tree` never
    closes again.

## Night lights and burglar event (GAME-NIGHT, GAME-EVENTS; Q-118, Q-137, Q-139 answered)

`[[light]]` in `level-2.toml` (night-only): 10 lantern posts along the entry path and the ring
(≈ 10 m, 0.25 m inside the path edge: (28.5, 28.25) (moved out of the level gate lane, LAYOUT-036), (35.0, 30.75), (37.0, 24.25), (48.5, 24.25),
(53.0, 24.25), (36.25, 33.0), (36.7, 38.5), (51.25, 28.0), (51.25, 38.0), (47.0, 39.25)), one
beside every gate (koala (35.75, 34.8), elephant (54.25, 36.2), lion (49.2, 22.75), giraffe
(48.2, 43.25) — in front of the enclosure sign's inner end, ≥ 0.6 m off the fence, moved
2026-09-28 for LAYOUT-039 / Q-173; the ring post (36.7, 38.5) moved 0.45 m east, away from the
corner of `board_koala`), a wall lamp at the `food_storage_2` door (38.95, 31.3) and board lamps on the four
info boards and `map_board_l2`. No string lights (Q-118: only the level-1 entrance plaza). No post
stands in a hiding-place rect or scenery.

`[[event_spot]]`: burglars climb in over a ladder on the inside of `wall_l2_east` at (73.7, 39.5),
take a food box in front of `food_storage_2` and hide on the grass strip between the elephant
enclosure and the east wall (`l2_burglar_hideout` (71, 37, 3, 5)) — no hiding place, scenery or
path; note texts `event-burglar-note-level_2-<reading_level>` (CONT-MISSIONS).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| LAYOUT-L2-001 | Given `level-1.toml` and `level-2.toml` joined with `barrier_ne_tree` open, then every enclosure, building, landmark, barrier and hiding place of level 2 has a walkable cell reachable from the level-2 spawn next to or inside it (flood fill), and the level-1 spawn reaches the level-2 spawn. | unit |
| LAYOUT-L2-002 | Given `level-2.toml`, then every border cell of its bounds is solid except the `[[entry]]` cells, which are path cells edge-adjacent to a cell of their barrier in `level-1.toml`; the bounds of levels 1, 2 and 3 are pairwise disjoint. | unit |
| LAYOUT-L2-003 | Given `level-2.toml`, then no two solid elements share a cell and no path cell is covered by a solid element other than a level-transition barrier (the street runs on under `barrier_l2_construction`, LAYOUT-040). | unit |
| LAYOUT-L2-004 | Given this spec's element table and `level-2.toml`, then both list the same ids, types and rectangles (instance of LAYOUT-005). | unit |
| LAYOUT-L2-005 | Given the joined levels 1–2 with path speed 1.93 m/s and grass 1.45 m/s, then each pair of the walking-distance table and each hiding place → neighbour pair is ≤ 10 s. | unit |
| LAYOUT-L2-006 | Given missions `koala`, `elephant`, `giraffe`, `lion`, every candidate hiding place, the player on each walkable cell next to the info board or gate, and the camera of GAME-PLAYER §2 at every 45° rotation and 10/14/20 m on 1080×2340, then every wander cell centre at 0.5 m, at the animal's height (koala 0.9, elephant 3.0, giraffe 4.5, lion 1.5 m) and, if `perch_height_m` is set, at perch height + 1 m lies outside the view frustum. | unit |
| LAYOUT-L2-007 | Given the hiding places of level 2, then their `features` contain the CONT-MISSIONS details (tree house: tree_house, rope_ladder; tallest tree: tallest_tree; blossom tree: pink_blossoms, falling_petals; fountain: water_jet, stone_basin, coins; log pile: stacked_logs, sawdust; ball: giant_ball; tower: wooden_tower, stairs, high_platform; train: train, bell; playground: slide, swings; sun rocks: flat_rocks, full_sun; stage: stage, drums; deckchairs: striped_deckchairs, sunshade) and every `scenery` id exists. | unit |
| LAYOUT-L2-008 | Given the joined levels 1–2, then each of the element kinds `fountain`, `treehouse`, `giant_tree`, `blossom_tree`, `play_ball`, `log_pile`, `lookout_tower`, `zoo_train`, `slide`, `swings`, `stage`, `deckchairs` and the scenery kinds `flat_rocks`, `petal_carpet` occur exactly once; no tree element other than `tree_giant_e` has `height_m` > 7; `enc_lion` notes contain no rocks. | unit |
| LAYOUT-L2-009 | Given each level-2 hiding place, then at least one walkable cell centre is within 2 m of its animal spot. | unit |
| LAYOUT-L2-010 | Given missions `koala`, `elephant`, `giraffe`, `lion` complete but the next morning not yet started, then `barrier_l2_construction` is solid; after the morning starts it is walkable and the level-3 spawn is reachable (Q-091). | unit |
| LAYOUT-L2-011 | Given `level-2.toml`, then it has 10 `food_box` entries (one per food) in front of `food_storage_2` plus 2–6 more inside that may repeat a food, each with a walkable standing cell within 2 m reachable from the spawn. | unit |
| LAYOUT-L2-012 | Given the level-2 spawn on a 1080×2340 viewport at maximum zoom-out, then the player faces east and `food_storage_2` is on screen. | e2e |
| LAYOUT-L2-013 | Given the `[[hiding_place]]` list of level 2, then every animal with an enclosure has ≥ 3 candidates, each wander area (as LAYOUT-L1-014) has ≥ 9 cells inside its rect, places of different animals are disjoint, and every candidate is in a combination with all spots pairwise ≥ 12 m (57 of 81 combinations). | unit |
| LAYOUT-L2-014 | Given the `[[scenery]]` list of level 2, then no scenery rect overlaps a solid element or a path cell, each lies inside its hiding place's rect, and each scenery kind occurs once in the joined levels 1–3. | unit |
| LAYOUT-L2-015 | Given a new game seeded with S that reaches level 2, then both koalas start at the same chosen koala place and both follow when one is shown eucalyptus (GAME-FAMILY FAM-001/002 in level 2). | unit |
| LAYOUT-L2-016 | Given `enc_elephant`, then `elephant_pool` covers 35–60 % of its cells, is fully inside the enclosure and not adjacent to the gate cells, and the elephant's home wander area includes pool cells. | unit |
| LAYOUT-L2-017 | Given the approved mockups of the 12 level-2 places, then a reviewer can name each place's riddle details without text, the lion enclosure shows no flat rocks, the elephant pool no jet or coins, the giant tree is clearly twice as tall as all other trees. | manual |
| LAYOUT-L2-018 | Given `level-2.toml`, then it has one `[[item]] kind = "bed"` (`bed_l2`) whose 2 × 1 m footprint lies on walkable cells of level 2 outside every hiding-place rect, scenery rect, door/gate cell and the 1 m in front of every door/gate, the level is still fully reachable from the spawn with the bed solid, and its `stand` cell is walkable, 1.0–1.5 m from the bed centre and reachable from the spawn; at level 2's nightfall the bed offered is `bed_l2` (Q-141). | unit |

## Implementation status (M5b, 2026-09-26)

- Playable in the joined zoo (GAME-LAYOUT "Joining levels"): the four missions follow the
  core loop; the koala is **one** animal (`pair` off until `koala_female.glb` exists —
  GAME-FAMILY; FAM-001/002 and LAYOUT-L2-015 are unit-tested with the flag on). Koalas sit
  at their `perch_height_m` (Q-094) and climb down when shown eucalyptus.
- **Barrier timing (Q-091 answered, 2026-09-27):** `barrier_l2_construction` (and the level-1
  `barrier_north_gate`, Q-090) open **the next morning** after the last level-2 mission
  (GAME-NIGHT "Implementation", NIGHT-010); LAYOUT-L2-010 is tested with this rule.
- Unit tests: LAYOUT-L2-001…011, 013, 014, 015 (FAM), 016 in
  `crates/zoo-core/tests/levels23.rs` / `zoo_game.rs`; the walking and sight tables are
  reproduced within 0.5 s / exactly. e2e: the giraffe mission (seed 4, `m5b.spec.ts`);
  LAYOUT-L2-012 (spawn view) is covered only by the entry screenshot.
- **Placeholders** (coloured boxes with the element's height inside its solid cells — the
  cells are the collider, wander areas and paths stay free): `fountain_sw` (basin, water,
  jet, coins), `treehouse_e` (oak, porch at 3.5 m, house, ladder), `tree_giant_e` (12 m trunk,
  two crown blocks leaving the 9 m branch free), `tree_blossom_ne` (pink crown, bees),
  `log_pile_nw`, `ball_n`, `tower_sw`, `train_se`, `playground_se_slide`,
  `playground_se_swings`, `stage_ne`, `deckchairs_ne`, `barrier_l2_construction` (striped
  panels, digger, sign), the giraffe rack and house, the lion sun deck and log, the
  elephant hay rack; scenery `flat_rocks_nw` and `petals_ne` as flat tiles; the
  `elephant_pool` rim via the `pool_tiled` fallback. Real kit props used: hedges, zoo
  walls, fences, gates, signs (with silhouettes), info/map boards, food boxes, groves,
  `tree_eucalyptus` (koala enclosure), `rock`, `bush`.

## Open questions

- Q-145 answered 2026-09-27: the layout changes of FIX-056 are accepted (22 m haze rule: moved/clipped hiding places, moved board, bench and trail, new walking neighbours).
- Q-137 `[[light]]` data shape, Q-139 burglar event spots (both answered 2026-09-27).

- Q-022 barrier unlock timing (Q-091 answered: the next morning); Q-141 (answered) night level between
  level 2 and level 3, and the bed after level 2; Q-088 joining levels; Q-089 own food
  storage per level; Q-094 koalas up in the tree; Q-095 the new hiding places and
  `kiga` words; Q-085 tree density and enclosure features; Q-080 hiding-place data;
  Q-082 picking rule; Q-043 poses (`climb` at perches, `sleep`, `drink`).
- Q-096 where the child sleeps after level 2 (zookeeper house of level 3 is still closed) —
  answered by Q-141 (b): `bed_l2` at the food storage; Q-152 shelter over `bed_l2`.
- Q-150 answered: the food-box row of `food_storage_2` leaves a gap ≥ 1.2 m in front of the door.
- Q-173 scope of the Q-157 wall-gap rule (nothing solid 0.1–0.6 m in front of a wall/fence near an opening).
- Q-181 answered 2026-09-28: the food boxes stay outside, a few more inside; Q-194 answered 2026-09-29: the inside boxes are real, labelled food boxes too (foods may repeat the outside row).
