---
id: GAME-LEVEL-3
title: Level 3 — monkey, goldfish, snow fox (adventure playground and stream)
aspect: gameplay
module: levels
status: draft
depends_on: [GAME-LAYOUT, GAME-LEVEL-1, GAME-LEVEL-2, GAME-RESCUE, GAME-NIGHT, CONT-MISSIONS, GAME-PLAYER]
test_prefix: LAYOUT-L3
updated: 2026-09-28
---

# Level 3 — monkey, goldfish, snow fox (adventure playground and stream)

Owned by the `zoo-level-designer` agent. Layout data: `assets/levels/level-3.toml`
(level id `level_3`). Coordinate system, element types, "Joining levels" and general rules:
GAME-LAYOUT. Look of each area: ART-ENVIRONMENT and the mockup briefs in `art/environment/`.

## Goal

The third day level: on the morning after level 2 is complete (after level 2's night — Q-141 a, until a `night_2` exists), the
construction fence `barrier_l2_construction` (GAME-LEVEL-2) is gone and the new northern part
of the zoo opens — an **adventure playground** with the pirate ship, a **stream** with a
waterfall, the **zookeeper house** and three rescue missions. With level 3 all ten day
animals of the artwork are in the game.

| Mission | Enclosure | Food box | Candidate hiding places (one is picked per playthrough) | What the riddles rely on (CONT-MISSIONS) |
|---|---|---|---|---|
| `monkey` | `enc_monkey` (north) | Bananen / bananas | `loc_pirate_ship` (south-east, adventure playground), `loc_carousel` (south-west), `loc_trampoline` (south-west corner, by the stream — FIX-056) | mast, sail, black flag, treasure chest · wooden horses, turning, music · springy blue mat in the lawn |
| `goldfish` | `enc_goldfish` (east, pond) | Fischfutter / fish food | `loc_waterfall` (north-west), `loc_water_wheel` (west), `loc_willow` (south-west) — all in the stream | water falling from rocks, white foam, loud · wooden wheel turning and clattering, little hut · long branches hanging into the water, shady and calm |
| `snow_fox` | `enc_snow_fox` (south) | Beeren / berries | `loc_ice_cream_kiosk` (north-east), `loc_sprinkler` (north-west), `loc_laundry` (north-west, by the stream) | cold air from a chest, waffles/cones · turning sprinkler, cold drops, rainbow, wet grass · white sheets on a washing line (white on white) |

Plus: **zookeeper house** `zookeeper_house_3` (enterable; the **fish bowl** stands on its
table, a **water tap** at its door), **food storage 3** (all 10 food boxes, proposal
Q-089), a map board at the entry, and a second entry path to the level-1 north gate
(proposal Q-090).

**Goldfish mission** (GAME-RESCUE "goldfish bowl"): read the board (it says a bowl is
needed, `mission-goldfish-bowl-hint-<reading_level>`) → take the bowl in the zookeeper house (5.7 s
from the storage) → fill it at the tap next to the door or at any stream bank → take fish
food from storage 3 (pocket, Q-084) → find the goldfish in the stream where the riddle
points → feed it from the bank → it jumps into the bowl → carry the bowl to `enc_goldfish`
and put it down on the stone step (its gate) → the fish jumps into its pond.

**Design idea.** The level-1 ring once more: a ring path around the zookeeper house, food
storage 3 and dense trees; enclosures on the north (monkey), east (goldfish) and south (snow
fox) side; the **west side is the stream**, so all three goldfish places lie 31–34 m from
the goldfish board on the opposite side. Monkey places are in the south and west, snow-fox
places in the north. 3³ = 27 combinations, **10** keep all spots ≥ 12 m apart; every
candidate is in ≥ 1 (picking rule Q-082).

## Proposals used in this level (not yet decided)

| Topic | Proposal used here | Question |
|---|---|---|
| Joining levels | Level 3 uses the level-1 coordinate system (north of level 1, west of level 2); `[[entry]]` cells behind `barrier_l2_construction` (level 2) and behind the level-1 `barrier_north_gate`. | Q-088 |
| Second entry | The level-1 `barrier_north_gate` opens **together with** `barrier_l2_construction`, so level 3 is also reached straight from the level-1 ring (shortcut home to the entrance). Needs a change of GAME-LEVEL-1 (LAYOUT-L1-010, unlock of `barrier_north_gate`). Until decided, `path_l3_gate` ends at the closed gate (sealed). | Q-090 |
| When the barrier opens | Morning after level 2 is complete. | Q-022, Q-091 |
| Own food storage | `food_storage_3` with all 10 boxes. | Q-089 |
| Enterable building | `zookeeper_house_3` has an `interior` rect and a `door` cell: those cells are walkable (surface `path`, floor), the rest of the building is solid; roof cut-away inside (GAME-PLAYER §2). | Q-092, Q-065 |
| Fish bowl and water sources | `[[item]] fish_bowl` (position on the table in the house) and `[[water_source]]` (`tap_l3` + every bank cell of `stream_l3`); water sources of other unlocked levels (level-1 river and pond, level-2 fountain) also fill the bowl. | Q-093, Q-084 |
| Pirate ship | A pirate-ship **climbing frame on the adventure playground** inside the zoo (south-east of level 3), not at a lake; no key on it (Q-033 open: the storage is unlocked). The monkey sits in the crow's nest (`perch_height_m` 4 m). | Q-017, Q-094 |
| Goldfish home | `enc_goldfish` is a pond enclosure (`[[enclosure_feature]] goldfish_pond`); its "gate" is a flat stone step where the bowl is put down (the player never enters). | Q-093 |
| Hiding places and scenery | As level 1 (`[[hiding_place]]` with `wander_on = "water"` and `water_kinds = ["stream"]` for the goldfish). | Q-080 |
| 22 m haze rule (FIX-056) | `loc_pirate_ship` wander radius 2.5 m; `loc_laundry` clipped on the side facing the snow-fox board; `loc_trampoline` with `trampoline_w` moved to the south-west corner by the stream (see "Hiding places"). | Q-145 |

## Spawn and camera

- Spawn cell **(20, 53)** on `path_l3_entry`, 3 m behind the opened construction fence,
  facing **−x (west)**; used when a save starts or continues in level 3 and after the
  morning cut-scene (ribbon and balloons at the opening, the zookeeper waves).
- First view: the pirate ship on its bark-mulch playground on the right, the map board on
  the right edge, the path west towards the ring and the food storage.

## Map

Scale **1 character = 1 m**. North (+Z) up; row labels `z`, x axis below. Generated from
`assets/levels/level-3.toml`. Level 1 lies south of z 48 (its `barrier_north_gate` at
x −9…−7, z 46–47), level 2 east of x 23 (its `barrier_l2_construction` at x 24–25, z 52–54).

```
   93 ################################################
   92 ################################################
   91 ##^^^^^.............yyyyyyyyyyyy..............%%
   90 ##^^^^^....rrrrrrr..yyyyyyyyyyyy..............%%
   89 ##^^^^^....rrrrrrr..yyyyyyyyyyyy..............%%
   88 ##^^^^^....rrrrrrr..yyyyyyyyyyyy..............%%
   87 ##~~~......rrr8rrr..yyyyyyyyyyyy..............%%
   86 ##~~~......rrrrrrr..yyyyyyyyyyyy..............%%
   85 ##~4~......rrrrrrr..yyyyyyyyyyyy.......KKKK...%%
   84 ##~~~......rrrrrrr..yyyyyyyyyyyy.......KKKK...%%
   83 ##~~~...............yyyyyyyyyyyy.......KKKK...%%
   82 ##~~~wwwww..........yyyyyggyyyyy..............%%
   81 ##~~~....................................7....%%
   80 ##~~~..9..............i.......................%%
   79 ##~~~.....==================================..%%
   78 ##~~~.....==================================..%%
   77 ##~~~.....==================================..%%
   76 ##~~~.....===TTTTTTTTTTTTTTTTTT===............%%
   75 ##~~~.....===TTTTTTTTTTTTTTTTTT===............%%
   74 ##~~~========TTTTTTTTTTTTTTTTTT===............%%
   73 ##~~~========TTTTTTTTTTTTTTTTTT===............%%
   72 ##~~~.mmm.===TTTTTTTTTTTTTTTTTT===.fffffffffff%%
   71 ##~5~.mmm.===TTTTTTTTTTTTTTTTTT===.fffffffffff%%
   70 ##~~~.mmm.===TTTTTTTTTTTTTTTTTT===ifffffffffff%%
   69 ##~~~.....===TTTTTTTTTTTTTTTTTT===.fffffffffff%%
   68 ##~~~.....===TTTTTTTTTTTTTTTTTT===.gffffffffff%%
   67 ##~~~.....===TTTTTTTTTTTTTTTTTT===.gffffffffff%%
   66 ##~~~.....===ZZZZZZZTTTFFFFFFFF===.fffffffffff%%
   65 ##~~~.....===Z_____ZTTTFFFFFFFF===.fffffffffff%%
   64 ##~~~.....===Z_U___ZTTTFFFFFFFF===.fffffffffff%%
   63 ##~~~.....===Z_____ZTTTFFFFFFFF===.fffffffffff%%
   62 ##~~~.....===Z_____ZTTTFFFFFFFF===............%%
   61 ##~~~========ZZZDZZZTTTFFFFDFFF===............%%
   60 ##~~~=============================.bbb........%%
   59 ##~~~.....========================.bbbPPPPPPP.%%
   58 ##~~~.YYY.========================.bb1PPPPPPP.%%
   57 ##~6~.YYY......===...i.........===.bbbPPPPPPP.%%
   56 ##~~~.YYY......===.xxxxggxxxx..===.bbbPPPPPPP.%%
   55 ##~~~...CCCC...===.xxxxxxxxxx..===.bbb....MM..%%
   54 ##~~~...CCCC2..===.xxxxxxxxxx..=================
   53 ##~~~...CCCC...===.xxxxxxxxxx..=============S===
   52 ##~~~tttCCCC...===.xxxxxxxxxx..=================
   51 ##...t3t.......===.xxxxxxxxxx.................%%
   50 ##...ttt.......===.xxxxxxxxxx.................%%
   49 ##%%%%%%%%%%%%%===%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
   48 ##%%%%%%%%%%%%%===%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
      |.......|.......|.......|.......|.......|.......
 x:   -24     -16     -8      0       8       16
```

| Char | Meaning | Char | Meaning |
|---|---|---|---|
| `#` | outer zoo wall (`boundary`) | `%` | tall hedge (level edge) |
| `=` | path | `~` | stream (`stream_l3`, water, flows south) |
| `^` | rock ledge with the waterfall (`waterfall_rocks`) | `m` | mill hut (water wheel at x −20, z 70–72) |
| `Y` | weeping willow (`tree_willow`) | `Z` / `_` / `D` / `U` | zookeeper house walls / walkable floor / doors / fish bowl on the table |
| `F` | food storage 3 | `T` | dense trees (solid) |
| `f` `y` `x` | enclosure goldfish (pond) / monkey / snow fox | `g` | gate (goldfish: stone step for the bowl) |
| `i` | info board | `M` | map board |
| `P` | pirate ship climbing frame | `b` | bark mulch (`bark_mulch_se`, walkable) |
| `C` | carousel | `t` | ground trampoline (`trampoline_w`, walkable) |
| `K` | ice cream kiosk (freezer chest in front) | `r` | wet lawn with the sprinkler (`sprinkler_lawn`, walkable) |
| `w` | washing line with white sheets | `S` | spawn |
| `1` `2` `3` | animal spot monkey: `loc_pirate_ship`, `loc_carousel`, `loc_trampoline` | `4` `5` `6` | animal spot goldfish (in the stream): `loc_waterfall`, `loc_water_wheel`, `loc_willow` |
| `7` `8` `9` | animal spot snow fox: `loc_ice_cream_kiosk`, `loc_sprinkler`, `loc_laundry` | `.` | grass (walkable) |

The water tap `tap_l3` is a prop at (−5.5, 60.9) flush on the south facade, 1.5 m right of the house door (moved 2026-09-27: no pocket beside the door, Q-157, LAYOUT-038).

## Elements

Grid rect = `x, z, w, d` (south-west corner + size). Solid = every type except `path` and
`hiding_place`, **except** the `interior` and `door` cells of `zookeeper_house_3`
(proposal Q-092).

| Id | Type | Grid rect (x, z, w, d) | Notes |
|---|---|---|---|
| `wall_l3_west` | boundary (zoo_wall) | -24, 48, 2, 46 | Outer zoo wall (continues level-1 wall_west); the stream leaves through a grate under it at z 52-54. |
| `wall_l3_north` | boundary (zoo_wall) | -22, 92, 46, 2 | Outer zoo wall, north edge of the zoo, permanent. |
| `hedge_l3_south_a` | decoration (hedge) | -22, 48, 13, 2 | Level edge; behind it lies level-1 hedge_north_a. |
| `path_l3_gate` | path (side) | -9, 48, 3, 2 | Path behind the level-1 barrier_north_gate (second level entry entry_l3_south, proposal Q-new): the gate opens together with level 3, a shortcut to the level-1 ring and entrance. |
| `hedge_l3_south_b` | decoration (hedge) | -6, 48, 28, 2 |  |
| `hedge_l3_east_a` | decoration (hedge) | 22, 48, 2, 4 |  |
| `hedge_l3_east_b` | decoration (hedge) | 22, 55, 2, 37 | Level edge towards level 2 (behind it: level-2 hedge_l2_w_c and outside area). |
| `waterfall_rocks` | landmark (waterfall) | -22, 88, 5, 4 | Rock ledge at the north wall; the stream springs from it and falls 2.5 m in a white foaming waterfall into the stream below (loc_waterfall). The only waterfall in the zoo. |
| `stream_l3` | landmark (stream) | -22, 52, 3, 36 | Narrow clear stream, flowing south from the waterfall; leaves west through a grate under wall_l3_west at z 52-54. Pebbles on the bottom, small reeds. No bridge, no ducks, no lilies (riddle guards). |
| `mill_hut` | decoration (mill_hut) | -18, 70, 3, 3 | Tiny wooden mill hut; its big wooden water wheel (radius 1.2 m, 0.5 m wide, 8 paddles) turns in the stream centred at (−20, 71.5): the axle runs from the hut wall 0.8 m above the water surface, the lower paddles dip 0.4 m (⅓ of the radius) under water, 67.5°/s (3 turns per 16 s water loop, ≈ 60°/s) towards the flow, foam where the paddles enter and leave the water — not solid. The only water wheel in the zoo (loc_water_wheel). |
| `tree_willow` | decoration (willow) | -18, 56, 3, 3 | Weeping willow on the east bank; its long hanging branches reach over the bank and dip into the water (canopy not solid, occluder fade). The only willow in the zoo (loc_willow). |
| `path_l3_entry` | path (main) | 7, 52, 17, 3 | From the opened construction fence (level 2) west to the ring; first cells = level entry. |
| `path_l3_link` | path (main) | 7, 55, 3, 3 |  |
| `path_l3_ring_s` | path (main) | -14, 58, 24, 3 |  |
| `path_l3_ring_w` | path (main) | -14, 61, 3, 16 |  |
| `path_l3_ring_e` | path (main) | 7, 61, 3, 16 |  |
| `path_l3_ring_n` | path (main) | -14, 77, 24, 3 |  |
| `path_l3_south` | path (side) | -9, 50, 3, 8 | Side path from the ring south to the hedge behind the level-1 north gate (entry_l3_south if Q-new is accepted). |
| `path_l3_bank` | path (side) | -19, 60, 5, 2 | Short path from the ring to the stream bank at the willow. |
| `path_l3_mill` | path (side) | -19, 73, 5, 2 | Short path from the ring to the stream bank just north of the mill hut. |
| `path_l3_ne` | path (side) | 10, 77, 10, 3 | Side path east from the ring towards the ice cream kiosk and the trampoline. |
| `zookeeper_house_3` | building (zookeeper_house) | -11, 61, 7, 6 | door at cell (-8, 61); walkable interior (-10, 62, 5, 4). Enterable zookeeper house (roof cut-away inside, GAME-PLAYER §2): shelves, a table, a bed, and the big empty glass bowl (fish_bowl) on the table. Water tap on the outside wall next to the door. |
| `food_storage_3` | building (food_storage) | -1, 61, 8, 6 | door at cell (3, 61); enterable, walkable interior (0, 62, 6, 4). Third food storage (proposal Q-089): all 10 food boxes outside in a row in front of the south facade (z 60.66; Q-181 answered): meat x −1.65, melons −0.93, hay −0.21, bananas 0.51, bamboo 1.23, grass 1.95, gap (box edges 2.26 … 4.74: ≥ 0.9 m beside each door post, Q-150, LAYOUT-038), fish food 5.05, berries 5.8, eucalyptus 6.6, leaves 7.4. Inside (Q-194 answered 2026-09-29): 6 more real, labelled food boxes (foods of the level's animals, repeated) on the wall band's plank platform — back (north) wall z 66.44 at x 0.88 (fish food), 1.52 (bananas), 4.28 (berries), 4.92 (fish food); west wall (−0.44, 64.0, bananas); east wall (6.44, 63.2, berries). |
| `trees_l3_center` | decoration (tree_grove) | -4, 61, 3, 16 | density `dense`.  |
| `trees_l3_center_e` | decoration (tree_grove) | -1, 67, 8, 10 | density `dense`.  |
| `enc_goldfish` | enclosure | 11, 63, 11, 10 | gate (11, 67, 1, 2). Goldfish pond enclosure: round pond with a low stone rim, water plants, a low wooden fence; the "gate" is a flat stone step where the bowl is put down to let the fish in (GAME-RESCUE goldfish bowl step 6). No waterfall, no wheel, no willow (riddle guards). |
| `board_goldfish` | decoration (info_board) | 10, 70, 1, 1 | info board of `enc_goldfish`.  |
| `enc_monkey` | enclosure | -4, 82, 12, 10 | gate (1, 82, 2, 1). Climbing frame of logs and ropes, hanging tyres, a wooden monkey house, a banana basket. No ship, no carousel, no trampoline (riddle guards). |
| `board_monkey` | decoration (info_board) | -2, 80, 1, 1 | info board of `enc_monkey`.  |
| `enc_snow_fox` | enclosure | -5, 50, 10, 7 | gate (-1, 56, 2, 1). Shady enclosure: wooden den with a straw bed under two pine trees, light rocks, a shallow drinking bowl. No freezer, no sprinkler, no washing line (riddle guards). |
| `board_snow_fox` | decoration (info_board) | -3, 57, 1, 1 | info board of `enc_snow_fox`.  |
| `map_board_l3` | landmark (map_board) | 18, 55, 2, 1 | Picture map at the level entry. |
| `pirate_ship` | landmark (pirate_ship) | 14, 56, 7, 4 | Pirate ship climbing frame (playground landmark, Q-017 proposal): hull on bark mulch, mast with a crow's nest, white sail, black pirate flag with a big white paw print (friendly, no skull), treasure chest on deck, rope ladder. No slide (riddle guard vs. the level-2 playground). loc_pirate_ship. |
| `carousel_sw` | landmark (carousel) | -16, 52, 4, 4 | Small carousel with wooden horses and a striped round roof, slowly turning, music box sound. loc_carousel. |
| `ice_cream_kiosk` | building (kiosk) | 15, 83, 4, 3 | Ice cream kiosk with a striped awning, a big ice-cream-cone icon on the roof (no text), a freezer chest with a glass lid in front (cold mist) and a cone stand. loc_ice_cream_kiosk. |
| `trees_l3_garden` | decoration (tree_grove) | -11, 67, 7, 10 | density `dense`.  |
| `laundry_line` | decoration (washing_line) | -19, 82, 5, 1 | Washing line of the zookeepers between two posts on the lawn by the stream with big white sheets and towels flapping in the wind, a peg bag and a laundry basket. loc_laundry. |

## Level entries

| Id | Cells (x, z, w, d) | Joins | Opens when |
|---|---|---|---|
| `entry_l3_east` (`[[entry]]`) | 23, 52, 1, 3 | level-2 `barrier_l2_construction` (24, 52, 2, 3) | Morning after level 2 is complete (GAME-LEVEL-2, Q-091). |
| `entry_l3_south` (`[[entry]]`, proposal Q-090) | −9, 48, 3, 1 | level-1 `barrier_north_gate` (−9, 46, 3, 2) — level-1 `path_north` continues as `path_l3_gate` / `path_l3_south` | Together with `entry_l3_east` (needs the GAME-LEVEL-1 change). |

Level 3 has **no exit barrier**: all ten day animals are then in the game. The next area
(further night levels, or the space east of level 1 behind `barrier_east_repair`) is not
planned yet (Q-023).

## Hiding places (candidates)

Data: `[[hiding_place]]` in `level-3.toml`. Riddle keys
`mission-<animal>-riddle-<id>-<reading_level>` (CONT-MISSIONS §7, §6, §10). Rect = bounding
box of the wander area and scenery.

| Id | Animal | Area rect (x, z, w, d) | Animal spot | Wander on | Wander cells | Perch | Features (riddle details) | Scenery | Spot → own info board |
|---|---|---|---|---|---|---|---|---|---|
| `loc_pirate_ship` | monkey | 10, 55, 6, 7 | (13, 58) | grass | 14 | 4.0 m | ship, mast, sail, flag, treasure_chest, rope_ladder | `pirate_ship`, `bark_mulch_se` | 26.6 m |
| `loc_carousel` | monkey | -14, 51, 5, 7 | (-12, 54) | grass | 19 | — | carousel, wooden_horses, music, turning_roof | `carousel_sw` | 27.9 m |
| `loc_trampoline` | monkey | -22, 50, 6, 4 | (-18, 51) | grass | 15 | — | trampoline, jumping, springy_mat | `trampoline_w` | 33.1 m |
| `loc_waterfall` | goldfish | -22, 82, 3, 6 | (-21, 85) | water | 16 | — | falling_water, foam, splashing, rock_ledge | `waterfall_rocks`, `stream_l3` | 34.4 m |
| `loc_water_wheel` | goldfish | -22, 68, 3, 7 | (-21, 71) | water | 17 | — | water_wheel, turning, clattering, wooden_hut | `mill_hut`, `stream_l3` | 31.0 m |
| `loc_willow` | goldfish | -22, 54, 3, 7 | (-21, 57) | water | 17 | — | weeping_willow, hanging_branches, shade_on_water | `tree_willow`, `stream_l3` | 33.6 m |
| `loc_ice_cream_kiosk` | snow_fox | 14, 80, 7, 4 | (17, 81) | grass | 18 | — | freezer_chest, cold_air, cones, scoops | `ice_cream_kiosk` | 31.2 m |
| `loc_sprinkler` | snow_fox | -13, 84, 7, 7 | (-10, 87) | grass | 29 | — | sprinkler, cold_drops, rainbow, wet_grass | `sprinkler_lawn` | 30.8 m |
| `loc_laundry` | snow_fox | -19, 77, 4, 5 | (-17, 80) | grass | 17 | — | white_sheets, washing_line, white_camouflage, wind | `laundry_line` | 26.9 m |

- **Goldfish wander area** = stream cells (`water_kinds = ["stream"]`) within 3 m of the spot,
  16–17 cells; the fish never leaves the stream. Its spot is 2 m from the bank column
  x = −19, so the player feeds it from the bank (interaction range 2 m, LAYOUT-L3-009).
- **Monkey at the pirate ship** sits in the crow's nest (`perch_height_m` 4 m, proposal
  Q-094); the ground wander area on the bark mulch is the fallback. Its `wander_radius_m` is
  **2.5 m** (the only place with less than 3 m): the diagonal cell (11, 60) was 21.6 m from the
  monkey board's standing cell (−1, 78) (FIX-056).
- **Haze rule (22 m, FIX-056).** Every wander cell and spot is ≥ 22 m (planar) from every
  standing point of its own animal (walkable cells ≤ 2.5 m from the own info board, cells
  around the own gate; GAME-LAYOUT "Sight", CAMV-008, fog end 20.8 m). Changes of 2026-09-27:
  `loc_pirate_ship` radius 3 → 2.5 m (20 → 14 cells); `loc_laundry` clipped to x −19…−16 (rect
  was −19, 77, 6, 5; 22 → 17 cells; cell (−15, 78) was 21.95 m from the snow-fox board);
  `loc_trampoline` moved with `trampoline_w` from the west lawn (rect −19, 62, 5, 6, spot
  (−17, 64) — 17.0–22 m from the monkey board, boxed in by stream, path and trees) to the
  south-west corner by the stream (rect −22, 50, 6, 4, spot (−18, 51); 26 → 15 cells).
  Smallest distances now: `loc_pirate_ship` 22.2 m, `loc_laundry` 22.2 m, `loc_carousel`
  22.8 m, the others ≥ 25.7 m.

**Spread (RESC-014)**, straight distances between spots of different animals (m); 12 of 27
combinations are valid (10 before FIX-056):

| | `loc_pirate_ship` | `loc_carousel` | `loc_trampoline` | `loc_waterfall` | `loc_water_wheel` | `loc_willow` | `loc_ice_cream_kiosk` | `loc_sprinkler` | `loc_laundry` |
|---|---|---|---|---|---|---|---|---|---|
| `loc_pirate_ship` | — | — | — | 43.4 | 36.4 | 34.0 | 23.3 | 37.0 | 37.2 |
| `loc_carousel` | — | — | — | 32.3 | 19.2 | 9.5 | 39.6 | 33.1 | 26.5 |
| `loc_trampoline` | — | — | — | 34.1 | 20.2 | 6.7 | 46.1 | 36.9 | 29.0 |
| `loc_waterfall` | 43.4 | 32.3 | 34.1 | — | — | — | 38.2 | 11.2 | 6.4 |
| `loc_water_wheel` | 36.4 | 19.2 | 20.2 | — | — | — | 39.3 | 19.4 | 9.8 |
| `loc_willow` | 34.0 | 9.5 | 6.7 | — | — | — | 44.9 | 32.0 | 23.3 |
| `loc_ice_cream_kiosk` | 23.3 | 39.6 | 46.1 | 38.2 | 39.3 | 44.9 | — | — | — |
| `loc_sprinkler` | 37.0 | 33.1 | 36.9 | 11.2 | 19.4 | 32.0 | — | — | — |
| `loc_laundry` | 37.2 | 26.5 | 29.0 | 6.4 | 9.8 | 23.3 | — | — | — |

Valid combinations (monkey, goldfish, snow fox): (`loc_pirate_ship`, `loc_waterfall`, `loc_ice_cream_kiosk`); (`loc_pirate_ship`, `loc_water_wheel`, `loc_ice_cream_kiosk`); (`loc_pirate_ship`, `loc_water_wheel`, `loc_sprinkler`); (`loc_pirate_ship`, `loc_willow`, `loc_ice_cream_kiosk`); (`loc_pirate_ship`, `loc_willow`, `loc_sprinkler`); (`loc_pirate_ship`, `loc_willow`, `loc_laundry`); (`loc_carousel`, `loc_waterfall`, `loc_ice_cream_kiosk`); (`loc_carousel`, `loc_water_wheel`, `loc_ice_cream_kiosk`); (`loc_carousel`, `loc_water_wheel`, `loc_sprinkler`); (`loc_trampoline`, `loc_waterfall`, `loc_ice_cream_kiosk`); (`loc_trampoline`, `loc_water_wheel`, `loc_ice_cream_kiosk`); (`loc_trampoline`, `loc_water_wheel`, `loc_sprinkler`).

**Sight test (LAYOUT-L3-006)**, same method as level 2 (0.5 m, animal height — monkey 1.1 m,
goldfish 0.3 m, snow fox 0.9 m —, perch + 1 m): all 9 candidates are off-screen in portrait
from every cell next to their board or gate, and for information also in landscape (the
trampoline and the washing line were moved west for this; the trampoline moved again to the
south-west corner for the 22 m haze rule, FIX-056).

**Nearest neighbour of each place** (≤ 10 s): `loc_pirate_ship` → spawn 5.4 s; `loc_carousel` → entry_s 3.7 s; `loc_trampoline` → loc_carousel 5.1 s (FIX-056; was → house_door from the west lawn); `loc_waterfall` → loc_sprinkler 8.0 s; `loc_water_wheel` → loc_laundry 6.7 s; `loc_willow` → house_door 8.3 s; `loc_ice_cream_kiosk` → board_monkey 9.8 s; `loc_sprinkler` → board_monkey 9.3 s; `loc_laundry` → board_monkey 7.3 s.

## Hiding places — riddle details and guards

| Hiding place | Riddle details and how the layout provides them |
|---|---|
| `loc_pirate_ship` (monkey) | `pirate_ship`: hull on bark mulch, **mast** with a crow's nest, white **sail**, **black flag** (white paw print, no skull), **treasure chest** on deck, rope ladder. The monkey sits in the crow's nest. **No slide** (the only slide is on the level-2 playground). |
| `loc_carousel` (monkey) | `carousel_sw`: small **carousel** with painted **wooden horses** under a striped round roof, slowly **turning**, music-box **music**. |
| `loc_trampoline` (monkey) | `trampoline_w`: round **ground-level trampoline** (blue mat, red rim) flush with the lawn in the south-west corner between the stream, the weeping willow and the carousel (FIX-056); the monkey bounces and somersaults. |
| `loc_waterfall` (goldfish) | `waterfall_rocks`: the stream **falls** 2.5 m from a rock ledge at the north wall, **white foam**, loud rushing sound; the fish swims in the foam pool below. Different from the level-1 river (only small rapids, bridge, ducks). |
| `loc_water_wheel` (goldfish) | `mill_hut` + its big **wooden water wheel** turning in the stream (x −20, z 70–72), **clattering** sound, drops glittering. The only water wheel. **The wheel runs in the water** (user request 2026-09-27): its axle stands on the bank side, the lower paddles dip below the stream's water surface (≈ ⅓ of the radius under water), the wheel turns continuously in the flow direction (south, ≈ 1 turn per 6 s, day and night), and the water shows foam/splash where the paddles enter and leave it (TECH-WATER obstacle foam + drop particles). |
| `loc_willow` (goldfish) | `tree_willow`: **weeping willow** whose long thin branches hang down like a green curtain **into the water**; shady and still underneath. The only willow. |
| `loc_ice_cream_kiosk` (snow fox) | `ice_cream_kiosk`: striped awning, cone icon on the roof, a **freezer chest** with a glass lid and **cold mist**, a cone stand; the fox sits next to the chest. |
| `loc_sprinkler` (snow fox) | `sprinkler_lawn`: a **garden sprinkler** turning in the middle of a lawn, arcs of **cold drops**, a small **rainbow**, glossy **wet grass**. Must not look like the level-2 fountain (no basin, no coins). |
| `loc_laundry` (snow fox) | `laundry_line`: a **washing line** with big **white sheets** and towels flapping in the wind on the lawn by the stream; the white fox is nearly invisible between them. |

Riddle guards (zoo-wide, Q-083):
- Only one waterfall, water wheel, willow, pirate ship, carousel, trampoline, kiosk,
  sprinkler and washing line in levels 1–3. The level-1 river has no waterfall; the stream
  has **no bridge, no ducks, no lilies, no frogs** (level-1 details).
- Enclosure guards: `enc_monkey` (climbing frame, ropes, tyres) has no ship, carousel or
  trampoline; `enc_goldfish` (pond) has no waterfall, wheel or willow; `enc_snow_fox`
  (wooden den under pines) has no freezer, sprinkler or washing line.
- The fish bowl, the tap and the stream banks are **not** riddle details (the riddles
  describe where the fish is, not how to carry it).

## Water sources and the fish bowl (proposal Q-093)

| Id | Kind | Where | Notes |
|---|---|---|---|
| `fish_bowl` (`[[item]]`) | carryable item | (−8.5, 64.5), table inside `zookeeper_house_3` | Big empty glass bowl; interact → carried with both hands (`socket_carry`, GAME-RESCUE). 2.1 s from the house door. |
| `tap_l3` (`[[water_source]]`) | tap | (−5.5, 60.9), flush on the south facade right of the door (Q-157) | Interact while carrying the empty bowl → filled. |
| `bank_stream_l3` (`[[water_source]]`) | bank | every walkable cell edge-adjacent to `stream_l3` | Interact at the bank → filled. |

Water sources of the other unlocked levels (level-1 `river_*` and `pond_water` banks,
level-2 `fountain_sw`) fill the bowl too (proposal Q-093). The goldfish home
`enc_goldfish` contains `goldfish_pond` (`[[enclosure_feature]]`, 13, 64, 8, 8), fenced — it
is not a water source for the player.

## Barriers

Level 3 owns no barrier. The border is the outer zoo wall (west, north), tall hedges (east
towards level 2, south towards level 1) and the two entries. The stream leaves the zoo
through a grate under `wall_l3_west` at z 52–54 (water is not walkable).

## Walking distances

Fastest walking time, 1.93 m/s paths / 0.98 m/s grass (GAME-PLAYER §6); `house_door` = cell
(−8, 60) in front of the house door, `bowl` = interior cell next to the table (−8, 64),
`storage_door` = (3, 60), `entry_s` = the cells behind the level-1 north gate.

| From → to | Fastest time |
|---|---|
| spawn → map_board_l3 | 0.7 s |
| spawn → loc_pirate_ship | 5.4 s |
| loc_pirate_ship → storage_door | 5.3 s |
| storage_door → house_door | 5.7 s |
| house_door → bowl | 2.1 s |
| storage_door → board_snow_fox | 3.5 s |
| board_snow_fox → house_door | 3.0 s |
| storage_door → board_goldfish | 7.7 s |
| board_goldfish → board_monkey | 9.1 s |
| house_door → entry_s | 6.2 s |
| board_snow_fox → entry_s | 7.0 s |

All neighbour pairs are ≤ 10 s. The goldfish round trip board → house (bowl) → tap →
storage (fish food) → stream → goldfish gate (carrying at 0.9 ×) takes about 60 s (willow), 66 s (water wheel) or 80 s (waterfall) of pure walking — the longest mission of the zoo, intentionally the last one.

## Behaviour

1. Level 3 contains exactly the elements of the table above; `assets/levels/level-3.toml`
   mirrors it (LAYOUT-005).
2. Its bounds `(−24, 48, 48, 46)` do not overlap levels 1 and 2; its only walkable border
   cells are the `[[entry]]` cells.
3. Every enclosure, building, landmark and hiding place is reachable from the spawn; with
   `barrier_l2_construction` and `barrier_north_gate` closed no level-3 cell is reachable
   from levels 1–2.
4. No two solid elements overlap; no path cell lies under a solid element; the interior and
   door cells of `zookeeper_house_3` are walkable.
5. Neighbouring points of interest and each hiding place's neighbour are ≤ 10 s apart.
6. The sight test holds for every candidate (as GAME-LEVEL-2 Behaviour 6).
7. Each landmark or scenery kind a riddle relies on exists once in the joined levels 1–3.
8. Each goldfish spot is a stream cell within 2 m of a walkable bank cell; the goldfish
   wander area contains only stream cells.
9. Wander areas: ≥ 9 cells, inside the rect, disjoint between animals; every candidate is
   in a valid combination (spots pairwise ≥ 12 m).
10. The fish bowl stands on a walkable interior cell of the house (reachable), the tap has a
    walkable cell within 1.5 m, and `enc_goldfish`'s gate step has a walkable cell in front.
11. The 10 food boxes of food storage 3 are props with a walkable standing cell within 2 m.

## Night lights and burglar event (GAME-NIGHT, GAME-EVENTS; Q-118, Q-137, Q-139 answered)

`[[light]]` in `level-3.toml` (night-only): 9 lantern posts along the entry path and the ring
((18.0, 52.25), (8.0, 52.25), (−13.0, 58.25), (−13.75, 72.0), (7.25, 62.0), (7.25, 72.0),
(−10.5, 77.25), (−3.0, 77.25), (7.0, 79.75); 0.25 m inside the path edge — the north-west post
was moved east so it stays outside `loc_laundry`), one beside every gate (goldfish (10.25, 65.8),
monkey (4.2, 81.25), snow fox (2.2, 57.75) — in front of the enclosure sign's inner end, ≥ 0.6 m
off the fence, moved 2026-09-28 for LAYOUT-039 / Q-173), wall lamps at the doors of `zookeeper_house_3`
(−8.3, 60.95, left of the door — the tap is on the right) and `food_storage_3` (4.3, 60.95), and
board lamps on the three info boards and `map_board_l3`. No string lights (Q-118).

`[[event_spot]]`: burglars climb in over a ladder on the inside of `wall_l3_north` at (9.5, 91.7),
take a food box in front of `food_storage_3` (never the fish bowl — Q-139 answered) and hide in the
grass corner between the monkey enclosure and the north wall (`l3_burglar_hideout` (8, 87, 3, 5));
note texts `event-burglar-note-level_3-<reading_level>`.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| LAYOUT-L3-001 | Given levels 1–3 joined with `barrier_ne_tree` and `barrier_l2_construction` open, then every enclosure, building, landmark and hiding place of level 3 is reachable from the level-3 spawn and from the level-1 spawn. | unit |
| LAYOUT-L3-002 | Given `level-3.toml`, then every border cell is solid except the `[[entry]]` cells, each a path cell edge-adjacent to its barrier in the earlier level; with both barriers closed the flood fill from the level-1 spawn reaches no level-3 cell. | unit |
| LAYOUT-L3-003 | Given `level-3.toml`, then no two solid elements share a cell, no path cell is under a solid element (level-transition barriers excepted, LAYOUT-040), and the `interior` + `door` cells of `zookeeper_house_3` and `food_storage_3` are walkable with surface `path` while its other cells are solid. | unit |
| LAYOUT-L3-004 | Given this spec's element table and `level-3.toml`, then both list the same ids, types and rectangles (LAYOUT-005). | unit |
| LAYOUT-L3-005 | Given the joined levels with 1.93 / 0.98 m/s, then every pair of the walking table and each hiding place → neighbour is ≤ 10 s. | unit |
| LAYOUT-L3-006 | Given missions `monkey`, `goldfish`, `snow_fox`, every candidate, the player next to the board or gate, all 8 rotations and 10/14/20 m on 1080×2340, then every wander cell centre (0.5 m, animal height, perch + 1 m) is outside the view frustum. | unit |
| LAYOUT-L3-007 | Given the hiding places of level 3, then their `features` contain the CONT-MISSIONS details (pirate ship: ship, mast, sail, flag, treasure_chest; carousel: carousel, wooden_horses; trampoline: trampoline; waterfall: falling_water, foam; water wheel: water_wheel, clattering; willow: weeping_willow, hanging_branches; kiosk: freezer_chest, cold_air, cones; sprinkler: sprinkler, cold_drops, rainbow; laundry: white_sheets, washing_line) and every `scenery` id exists. | unit |
| LAYOUT-L3-008 | Given levels 1–3 joined, then the kinds `waterfall`, `mill_hut`, `willow`, `pirate_ship`, `carousel`, `kiosk`, `washing_line` and the scenery kinds `trampoline`, `wet_lawn`, `bark_mulch` occur exactly once; `slide`/`swings` occur only in level 2; `stream_l3` has no bridge or jetty element next to it. | unit |
| LAYOUT-L3-009 | Given each goldfish candidate, then its spot is a `stream_l3` cell, its wander area contains only stream cells, and a walkable bank cell centre lies within 2 m of the spot; for every other candidate a walkable cell centre is within 2 m. | unit |
| LAYOUT-L3-010 | Given `level-3.toml`, then `fish_bowl` lies on a walkable interior cell of `zookeeper_house_3` reachable from the spawn, `tap_l3` has a walkable cell centre within 1.5 m, every `stream_l3` bank cell is a water source, and a walkable cell is edge-adjacent to the gate of `enc_goldfish`. | unit |
| LAYOUT-L3-011 | Given `level-3.toml`, then it has 10 `food_box` entries in front of `food_storage_3` (one per food) plus 2–6 more inside that may repeat a food, each with a reachable walkable standing cell within 2 m. | unit |
| LAYOUT-L3-012 | Given the `[[hiding_place]]` list of level 3, then every animal with an enclosure has ≥ 3 candidates, wander areas ≥ 9 cells inside their rect, disjoint between animals, and every candidate is in a combination with all spots ≥ 12 m apart (12 of 27). | unit |
| LAYOUT-L3-013 | Given the `[[scenery]]` list, then no scenery rect overlaps a solid element or path cell and each lies inside its hiding place's rect. | unit |
| LAYOUT-L3-014 | Given the goldfish mission in level 3 (seeded), when the player reads the board, takes the bowl, fills it at `tap_l3`, takes fish food, feeds the fish from the bank and puts the bowl on the goldfish gate step, then the mission completes (RESC-018…021 on this level). | e2e |
| LAYOUT-L3-015 | Given `enc_goldfish`, then `goldfish_pond` lies fully inside it, is not adjacent to the gate cells and the goldfish's home wander area contains only pond cells. | unit |
| LAYOUT-L3-016 | Given the approved mockups of the 9 level-3 places, then a reviewer can name the riddle details without text; the stream shows no bridge or ducks; the sprinkler does not look like a fountain; the pirate ship has no slide and no skull. | manual |
| LAYOUT-L3-017 | Given the `mill_hut` water wheel, then its lowest paddles lie below the `stream_l3` water surface (≥ 0.25 × radius under water) and inside stream cells, the wheel angle advances continuously over time in the flow direction (≈ 60°/s) by day and night, the wheel is not solid for the player, and a foam obstacle lies where the paddles meet the water. | unit |
| LAYOUT-L3-018 | Given the level-3 review screenshot of `loc_water_wheel`, then the wheel visibly dips into the stream and two frames 0.5 s apart show a different wheel angle. | e2e |

## Implementation status (M5b, 2026-09-26)

- Playable in the joined zoo; entered from level 2 through the construction fence **and**
  from the level-1 ring through `barrier_north_gate` (both open the morning after level 2
  is complete, Q-091 answered; Q-090 as proposed). No exit barrier.
- Goldfish: the bowl flow of GAME-RESCUE (tap `tap_l3`, every bank of `stream_l3` and of the
  other unlocked water bodies fills it; fish food in the pocket; 0.9 × speed; stone step =
  `enc_goldfish` gate) — LAYOUT-L3-014 e2e (seed 4, willow). The monkey sits in the crow's
  nest at 4 m (Q-094) and climbs down at 0.72 m/s (`climb`).
- `zookeeper_house_3` is enterable (Q-092): floor cells walkable, roof + upper walls hidden
  inside (PLAY-028/029); a table (`table_wood` placeholder with a collider), shelves and a
  bed (boxes) inside.
- Unit tests: LAYOUT-L3-001…013, 015 in `levels23.rs`; RESC-018…023 in `zoo_game.rs`.
- **Placeholders:** `waterfall_rocks` (ledge, rocks = kit `rock`, falling water and foam),
  `mill_hut` + water wheel (turning in the water, not solid, LAYOUT-L3-017/018: a procedural placeholder wheel with a spinning `wheel` part until `mill_hut_wheel` has an approved concept — `loc_water_wheel` `concept_approved = false`), `tree_willow` (trunk, crown,
  hanging branches), `pirate_ship` (hull, deck, mast, crow's nest at the perch point, sail,
  black flag with a white paw, treasure chest, rope ladder), `carousel_sw`,
  `ice_cream_kiosk` (small building, striped awning, cone icon, freezer with mist),
  `laundry_line`, the monkey climbing frame and house, the snow-fox den; scenery
  `bark_mulch_se`, `trampoline_w`, `sprinkler_lawn` (wet lawn, sprinkler, drops, rainbow)
  as flat boxes; `goldfish_pond` rim (fallback of `pond_stone_rim`), `water_tap`,
  `table_wood`, the fish bowl (renderer mesh). The stream uses the river water tiles
  (flow streaks north–south) with kit reeds on its banks.

## Open questions

- Q-145 answered 2026-09-27: the layout changes of FIX-056 are accepted (22 m haze rule: moved/clipped hiding places, moved board, bench and trail, new walking neighbours).
- Q-137 `[[light]]` data shape, Q-139 burglar event spots (both answered 2026-09-27). The beds of GAME-NIGHT are `bed_l1` in the level-1 `zookeeper_house_1` (Q-096) and `bed_l2` at level 2's food storage (Q-141 b); the bed in `zookeeper_house_3` stays decoration (no `[[item]] kind = "bed"`).

- Q-088 joining levels; Q-090 second entry through the level-1 north gate;
  Q-091 unlock timing (answered: the next morning); Q-141 (answered) night level before level 3; Q-089 own food storage; Q-092 enterable
  buildings; Q-093 fish bowl and water-source data (with Q-084); Q-094 animals up
  in a perch; Q-095 new hiding places; Q-017 pirate ship location (proposal here);
  Q-033 key on the pirate ship (not used); Q-080, Q-082, Q-043.
- Q-096 (answered by Q-141) the bed after level 2 is `bed_l2` in level 2, not in `zookeeper_house_3`.
- Q-150 answered: the food-box row of `food_storage_3` leaves a gap ≥ 1.2 m in front of the door (meat moved from x 3.4 to −1.4, leaves from 4.2 to 7.4). Q-157 answered (no pocket beside a door: `tap_l3` moved flush on the facade to (−5.5, 60.9), LAYOUT-038).
- Q-173 scope of the Q-157 wall-gap rule (nothing solid 0.1–0.6 m in front of a wall/fence near an opening).
- Q-181 answered 2026-09-28: the food boxes stay outside, a few more inside; Q-194 answered 2026-09-29: the inside boxes are real, labelled food boxes too (foods may repeat the outside row).
