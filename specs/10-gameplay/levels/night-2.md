---
id: GAME-LEVEL-NIGHT-2
title: Night level 2 — the terrarium garden (snake, chameleon, poison dart frog)
aspect: gameplay
module: levels
status: draft
depends_on: [GAME-LAYOUT, GAME-NIGHT, GAME-RESCUE, GAME-FAMILY, GAME-FEED, GAME-LEVEL-NIGHT-1]
test_prefix: LAYOUT-N2
updated: 2026-10-04
---

# Night level 2 — the terrarium garden (snake, chameleon, poison dart frog)

**Contents:** Goal · Why a second night level (numbers) · Design assumptions · Spawn, entry, camera · Map · Elements · Terrarium house · Night food storage · Hiding places · Riddle details · Riddles and facts (de / en) · Barrier · Walking and pacing · Lights · Never stuck · Behaviour · Mockups and art · Test cases · Open questions

Owned by the `zoo-level-designer` agent. **Implemented 2026-10-04** (user requests 2026-10-03 "a terrarium in a
building of the night level, with snake, chameleon and a colourful poison dart frog" and 2026-10-04 "also implement
the terrarium building in the night level": Q-330…Q-339 and Q-350…Q-352 answered "implement as recommended").
`assets/levels/night-2.toml` mirrors the element table below (LAYOUT-N2-004 checks both, the map is generated from it).
**Simplifications of the first implementation:** the terrarium house has the plan of the night house (same 17 × 13 m
`model_rect`, cases 8 m deep instead of 5) and is drawn with the `night_house` model as a stand-in; the riddle
scenery are coloured placeholder boxes; the basic-food / treat rule is implemented for the three new species only (the
older species keep the FAM-008/009 rule until the 16-species master table of GAME-FEED is built); the fridge prop, the
silhouettes for the signs and the real models are listed in "Mockups and art". Level id `night_2`. Night rules: GAME-NIGHT. Frame: the plaza, hut,
house and loop of `night_1`, **shifted 48 m west** (same relative geometry, so the 22 m haze rule, the 12 m spread and the
walking numbers of night_1 carry over); all riddle places, scenery and the house contents are new.

The night house of `night_1` holds hedgehog, bat and owl: this level is the first with terrariums.

## Goal

A tropical-looking **terrarium garden** behind a lantern gate in the west hedge of `night_1`, opened
once the first night zoo is done. It teaches nothing new in the loop (board → food → search →
gate, GAME-RESCUE) but brings three **exotic animals** and the new **two-role food model** (basic food /
treat, GAME-FEED "Basic food and treats") to the night zoo:

| Mission (species, each a male + female pair) | Terrarium (indoor enclosure, glass case) | Basic food (box) | Treat (baby, GAME-FAMILY) | Candidate hiding places | What the riddles rely on |
|---|---|---|---|---|---|
| `snake` Schlange / snake | `enc_n2_snake` (warm amber light) | `fish` Fisch / fish | `eggs` Eier / eggs | `loc_stone_wall` (south-west corner), `loc_pumpkins` (west), `loc_rowing_boat` (west) | row of warm stacked stones · big round orange pumpkins · blue wooden boat upside down with two oars |
| `chameleon` Chamäleon / chameleon (perched) | `enc_n2_chameleon` (violet UV-style light) | `crickets` Grillen / crickets | `frozen_insects` Frostinsekten / frozen insects | `loc_lanterns` (south), `loc_palm` (south), `loc_vine_arch` (south-east) | tree hung with colourful paper lanterns · palm with fan leaves and nuts, no branches · wooden arch with creeper curtains and pink flowers |
| `poison_dart_frog` Pfeilgiftfrosch / poison dart frog | `enc_n2_frog` (moist teal light) | `flies` Fliegen / flies | `crickets` Grillen / crickets | `loc_stepping_stones` (south), `loc_ferns` (north-west), `loc_rain_barrel` (north-west corner) | round smooth plates over wet moss, squelching · giant ferns, mist · wooden barrel under a dripping eave |

Babies: `snake_hatchling` (de Schlangenbaby / en baby snake), `chameleon_baby` (Chamäleonbaby / baby chameleon),
`frog_froglet` (Fröschlein / froglet; the tadpole is a fact on the board, not a model). Model ids follow the
approved-in-review concept briefs `art/animals/{snake,chameleon,poison_dart_frog}_family`.

## Why a second night level (decision with numbers)

`night_1` cannot hold the three new species (three pairs, nine hiding places, a second set of boards):

- 48 × 48 = 2304 cells; walkable non-path cells outside every existing hiding-place rect: **591**.
- Each new hiding place needs its spot **≥ 12 m** from the nine existing spots (RESC-014), its wander cells **≥ 22 m** from
  its own board and gate (Q-110, FIX-056) and ≥ 9 wander cells. Scratch check (2026-10-03, terrarium house in
  the only free block, board at (−33, 24)): cells that are ≥ 12 m from all nine spots **and** ≥ 25 m from the new board:
  **1**. A greedy search for nine mutually ≥ 12 m spots finds **one**. A new house in the other free block (−46…−40, 17…25) gives **0**.
- Putting the house in `night_1` and the hiding places in a second level is impossible: an animal hides in its own level.
- Six species would also make a night of 20+ minutes with a 12-icon world (the compass strip lists the species of the
  *current* level only, at most 6 icons: night_1 = 3, night_2 = 3, so the strip limit is never an issue either way).

→ **`night_2` is a new night level** (own spec, own data, own riddle scope shared with `night_1`, Q-136); `night_1` keeps hedgehog, bat and owl unchanged, and its *only* edit is the gate and a short path (see "Barrier").
Open for the user: Q-330.

## Design assumptions

| Topic | Proposal used here | Question |
|---|---|---|
| Level data | `[level] id = "night_2"`, `time = "night"`, bounds (−120, 6, 48, 48) = x −120…−73, z 6…53, `missions = ["snake", "chameleon", "poison_dart_frog"]`; `[[entry]] entry_n2_garden` cells (−73, 25…26) next to the night_1 barrier `barrier_n1_garden` | Q-330, Q-331 |
| Unlock | `barrier_n1_garden`: `unlock_after = "night_1"`, `opens_at = "night"` (open as soon as the three night_1 animals are home, stays open every later night) | Q-331 |
| Optional | night_2 never blocks sleeping, the morning, or level 2 (`barrier_ne_tree` keeps `unlock_after = "night_1"`) | Q-332 |
| Terrarium | enclosure with `indoor = true` plus the new flag `terrarium = true` (glass front, case frame, heat/UV lamp); gate = `glass_door` exactly like the night house. The flag is data (`Element::terrarium`); the case look comes with the real house model | Q-333 |
| Foods | two-role model GAME-FEED "Basic food and treats": new foods `fish`, `crickets`, `flies`, `eggs`, `frozen_insects`, `bone` (6; total 20) | Q-334 |
| Riddle scope | kinds unique across `night_1` ∪ `night_2`; no `kiga` word of a day place or a night_1 place (MISS-013) | Q-136 |
| Sizes | snake 1.2 m long (coiled 0.35 m high), chameleon 0.5 m with crest and curled tail (concept brief 0.35 m: comic scale, like Q-143), frog 0.4 m (comic, real ≈ 4 cm) | Q-335 |

## Spawn, entry and camera

- **Entry:** cells (−73, 25…26), edge-adjacent to `barrier_n1_garden` (night_1 cells x −72…−71, z 25…26); the only walkable border cells (LAYOUT-021).
- **Spawn** (save without a position): cell (−76, 25) on `path_n2_entry`, facing **−x (west)**, camera east of the player looking west.
- **First view:** the plaza under its string lights, the food hut with three boxes in front of it, the map board right (north), the terrarium house at the upper right with its amber, violet and teal glow behind the glass fronts.

## Map

Scale **1 character = 1 m**, north (+z) up, x axis below (generated from the element list, scratch script 2026-10-03).

```
   53 %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
   52 %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
   51 %%.............TTTTTTTTTTTTsssssscccccrrrrrrTT%%
   50 %%.............TTTTTTTTTTTTsssssscccccrrrrrrTT%%
   49 %%........RR...TTTTTTTTTTTTsssssscccccrrrrrrTT%%
   48 %%........RR...TTTTTTTTTTTTsssssscccccrrrrrrTT%%
   47 %%.............TTTTTTTTTTTTsssssscccccrrrrrrTT%%
   46 %%.............TTTTTTTTTTTTsssssscccccrrrrrrTT%%
   45 %%.............TTTTTTTTTTTTsssssscccccrrrrrrTT%%
   44 %%..nnn........TTTTTTTTTTTTssggsscggccrrggrrTT%%
   43 %%..nnn........TTTTTTTTTTTTNhhhhhhhhhhhhhhhNTT%%
   42 %%..nnn........TTTTTTTTTTTTNhhhhhhhhhhhhhhhNTT%%
   41 %%.............TTTTTTTTTTTTNhhhhhhhhhhhhhhhNTT%%
   40 %%.........................NhhhhhhhhhhhhhhhNTT%%
   39 %%.........................NNNNNNNNDNNNNNNNNTT%%
   38 %%..................b.........i..i===..i......%%
   37 %%......=============================.........%%
   36 %%......=============================.........%%
   35 %%......=============================.........%%
   34 %%......===......................=========....%%
   33 %%......===......................=========....%%
   32 %%BBB...===..TTTTTTTTTTTTT.......=========....%%
   31 %%BBB...===..TTTTTTTTTTTTT..FFFFF=========....%%
   30 %%......===..TTTTTTTTTTTTT..FfffF=========....%%
   29 %%......===..TTTTTTTTTTTTT..FfffD=========....%%
   28 %%......===..TTTTTTTTTTTTT..FfffF=========Mbb.%%
   27 %%......===..TTTTTTTTTTTTT..FfffF=========M...%%
   26 %%......===..TTTTTTTTTTTTT..FFFFF===============
   25 %%......===..TTTTTTTTTTTTT.......===============
   24 %%......===..TTTTTTTTTTTTT.......=========....%%
   23 %%......===..TTTTTTTTTTTTT.......===..........%%
   22 %%......===..TTTTTTTTTTTTT.......===..........%%
   21 %%kkkk..===..TTTTTTTTTTTTT.......===..........%%
   20 %%kkkk..===..TTTTTTTTTTTTT.......===..........%%
   19 %%kkkk..===..TTTTTTTTTTTTT.......===..........%%
   18 %%......===..TTTTTTTTTTTTT.......===..........%%
   17 %%......===......................===........VV%%
   16 %%......===......................===........VV%%
   15 %%......============================..........%%
   14 %%......============================..........%%
   13 %%......============================..........%%
   12 %%............................................%%
   11 %%WWWWW.......................................%%
   10 %%..................P...........zzzz..........%%
    9 %%.............LL...............zzzz..........%%
    8 %%.............LL.............................%%
    7 %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
    6 %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
      |.......|.......|.......|.......|.......|.......
  x:  -120    -112    -104    -96     -88     -80     
```

| Char | Meaning | Char | Meaning |
|---|---|---|---|
| `%` | tall hedge (level edge) | `=` | path (entry, plaza, loop, links) |
| `M` / `b` | map board / bench | `F` `f` `D` | food hut `food_storage_n2` walls / floor / door (the plaza is east of it) |
| `N` `h` `D` | terrarium house walls / visitor hall / door (z 39, x −85) | `s` `c` `r` | terrarium snake / chameleon / frog (indoor enclosures, z 44-51; `g` = glass gate) |
| `i` | info board (board lamp) | `T` | dense old-tree grove / `trees_n2_ne` |
| `W` | low stone wall (`loc_stone_wall`) | `k` | pumpkin patch (`loc_pumpkins`, scenery, walkable) |
| `B` | upturned rowing boat (`loc_rowing_boat`) | `L` | lantern tree (`loc_lanterns`) |
| `P` | palm tree (`loc_palm`) | `V` | wooden vine arch (`loc_vine_arch`) |
| `z` | stepping stones on wet moss (`loc_stepping_stones`, scenery, walkable) | `n` | fern glade (`loc_ferns`, scenery, walkable) |
| `R` | rain barrel under a little eave (`loc_rain_barrel`) | `.` | grass (walkable, slower) |

In the rows 44–51 the letters `s`, `c`, `r` are the three terrariums; the `f` inside the hut (rows 27–30) is the hut floor (`F` = hut walls); `h` is the visitor hall. The hedge cells at x −74…−73 are the east border (generated from `night-2.toml`).
East of x −73 (not drawn): the night_1 west hedge with the gate at z 25…26.

## Elements

Grid rect = `x, z, w, d`. Solid = every type except `path`. The table mirrors the future `night-2.toml` (LAYOUT-005). Scratch check: no cell shared by two elements.

| Id | Type | Grid rect (x, z, w, d) | Notes |
|---|---|---|---|
| `hedge_n2_west` | decoration (hedge) | −120, 6, 2, 48 | Level edge, permanent. |
| `hedge_n2_south` | decoration (hedge) | −118, 6, 46, 2 | Level edge, permanent. |
| `hedge_n2_north` | decoration (hedge) | −118, 52, 46, 2 | Level edge, permanent. |
| `hedge_n2_east_s` | decoration (hedge) | −74, 8, 2, 17 | East edge south of the gate (behind it the night_1 west hedge). |
| `hedge_n2_east_n` | decoration (hedge) | −74, 27, 2, 25 | East edge north of the gate. |
| `path_n2_entry` | path (main) | −78, 25, 6, 2 | From the gate west to the plaza; cells (-73, 25..26) are the level entry. |
| `path_n2_plaza` | path (plaza) | −87, 24, 9, 11 | Plaza under string lights; food boxes and the hut door on its west edge, path to the terrarium house north, s_link south. |
| `map_board_n2` | landmark (map_board) | −78, 27, 1, 2 | Picture map of the terrarium garden (silhouettes of snake, chameleon and frog, no text), with a board lamp. |
| `bench_n2_plaza` | decoration (bench) | −77, 28, 2, 1 |  |
| `food_storage_n2` | building (food_hut) | −92, 26, 5, 6 | `door` (−88, 29); `interior` (−91, 27, 3, 4). Small wooden food hut (copy of food_storage_n1), enterable: `interior` (-91, 27, 3, 4), door (-88, 29) on the east facade; basic foods in front of its east facade, the two treat boxes inside at its west wall (Q-336). A fridge prop for the frozen insects needs a model (not yet made). |
| `path_n2_house` | path (main) | −86, 35, 3, 4 | From the plaza north to the terrarium-house door (-85, 39). |
| `terrarium_house` | building (terrarium_house) | −93, 39, 17, 5 | `door` (−85, 39); `interior` (−92, 40, 15, 4); `model_rect` (−93, 39, 17, 13). Terrarium house: the same plan as the night house (hall + three glass-fronted terrariums along the north side) with a warm interior. Stand-in model: the night_house model until a conservatory model exists (kit_buildings spec `terrarium_house`). |
| `enc_n2_snake` | enclosure | −93, 44, 6, 8 | gate (−91, 44, 2, 1); `indoor = true`; `terrarium = true`; `pair = true`; `home_wander_on = ["grass"]`. Indoor terrarium behind a glass front, warm amber heat lamp: sandy floor, a warm flat rock, a branch to coil on, a water dish. No riddle detail of any hiding place inside. |
| `enc_n2_chameleon` | enclosure | −87, 44, 5, 8 | gate (−86, 44, 2, 1); `indoor = true`; `terrarium = true`; `pair = true`; `home_wander_on = ["grass"]`. Indoor terrarium behind a glass front, violet UV-style light: tall leafy branches, a hanging vine, a leaf dish. No riddle detail of any hiding place inside. |
| `enc_n2_frog` | enclosure | −82, 44, 6, 8 | gate (−80, 44, 2, 1); `indoor = true`; `terrarium = true`; `pair = true`; `home_wander_on = ["grass"]`. Indoor terrarium behind a glass front, moist teal light: large leaves, a mossy log, a shallow dish, fine mist. No riddle detail of any hiding place inside. |
| `board_n2_snake` | decoration (info_board) | −90, 38, 1, 1 | `mount = "wall"`; board of `enc_n2_snake`; board lamp. Outside the terrarium house on its south wall; board lamp. |
| `board_n2_chameleon` | decoration (info_board) | −87, 38, 1, 1 | `mount = "wall"`; board of `enc_n2_chameleon`; board lamp. Outside the terrarium house on its south wall; board lamp. |
| `board_n2_frog` | decoration (info_board) | −81, 38, 1, 1 | `mount = "wall"`; board of `enc_n2_frog`; board lamp. Outside the terrarium house on its south wall; board lamp. |
| `trees_n2_ne` | decoration (tree_grove) | −76, 39, 2, 13 | `density = "dense"`. Old round trees between the house and the east hedge. |
| `bench_n2_ring` | decoration (bench) | −100, 38, 1, 1 | Resting point on the north ring (there is no telescope in night_2, Q-367). |
| `path_n2_ring_n` | path (main) | −112, 35, 26, 3 | North ring from the house path west; passes the boards. |
| `path_n2_ring_w` | path (main) | −112, 13, 3, 22 | West side of the loop. |
| `path_n2_ring_s` | path (main) | −109, 13, 25, 3 | South side of the loop (x -109..-85). |
| `path_n2_s_link` | path (main) | −87, 16, 3, 8 | From the south ring north to the plaza. |
| `grove_n2_center` | decoration (tree_grove) | −107, 18, 13, 15 | `density = "dense"`. The hidden middle: old round trees (no palm, no lanterns). |
| `grove_n2_north` | decoration (tree_grove) | −105, 41, 12, 11 | `density = "dense"`. Old round trees north of the ring, up to the house wall. |
| `stone_wall_n2` | decoration (stone_wall) | −118, 11, 5, 1 | Low dry wall of flat grey stacked stones with moss in the gaps (loc_stone_wall); the only stone wall in the night zoo. |
| `rowing_boat_n2` | decoration (rowing_boat) | −118, 31, 3, 2 | Old wooden rowing boat, blue, upside down on the grass, two oars leaning on it (loc_rowing_boat). |
| `lantern_tree_n2` | decoration (lantern_tree) | −105, 8, 2, 2 | Tree with a spreading low crown hung with 20 colourful paper lanterns (loc_lanterns). |
| `palm_tree_n2` | decoration (palm_tree) | −100, 10, 1, 1 | Slim palm (4 m), no branches, fan leaves, coconuts (loc_palm). |
| `vine_arch_n2` | decoration (vine_arch) | −76, 16, 2, 2 | Wooden gate arch overgrown with green creepers, pink flowers, a crossbar (loc_vine_arch). |
| `rain_barrel_n2` | decoration (rain_barrel) | −110, 48, 2, 2 | Big wooden barrel with iron rings under a small eave and drain pipe; drips (loc_rain_barrel). |

## Scenery (non-solid `[[scenery]]` entries of the hiding places)

Walkable ground dressing a riddle relies on; not elements (LAYOUT-N2-004 checks them against `[[scenery]]`).

| Id | Type | Grid rect (x, z, w, d) | Notes |
|---|---|---|---|
| `pumpkin_patch_n2` | scenery (pumpkin_patch) | −118, 19, 4, 3 | place `loc_pumpkins`. Seven big round orange pumpkins on creeping vines with huge rough leaves; walkable. The only pumpkins in the zoo. |
| `stepping_stones_n2` | scenery (stepping_stones) | −88, 9, 4, 2 | place `loc_stepping_stones`. Round smooth plates over a wet moss patch, small trickle; walkable. No river, pond or bridge (riddle guards). |
| `fern_glade_n2` | scenery (fern_glade) | −116, 42, 3, 3 | place `loc_ferns`. Head-high giant ferns around an old stump, light mist; walkable. No mushrooms, no moss ring (riddle guards). |

## Terrarium house

An enterable house like `night_house` (hall + indoor enclosures, boards outside) but a **terrarium house**: low, rounded, a glass-roofed
conservatory look with plants climbing the window frames; **warm, lit interior** (visitor hall light warm white `#FFE9B0`, dim but readable, NIGHT-005),
no scary darkness. The door (−85, 39) stands at the north end of `path_n2_house` (a street, GAME-LAYOUT rule 8).

```
 z 51  ssssss ccccc rrrrrr      terrariums (indoor enclosures z 44-51, glass fronts at z 44)
 z 44  ss gg ss c gg cc rr gg rr    gg = glass door / gate (2 m)
 z 43  hall  x -92..-78 (15 x 4 m, surface path)   ← visitor hall
 z 40  hall
 z 39  wall .... .... D(-85) .... ....            ← south wall, door at x -85
 z 38  i(-90)   i(-87)   path  i(-81)             ← boards outside
```

| Terrarium | Rect | Gate (hall side) | Light | Inside (riddle guards: nothing of any riddle place) |
|---|---|---|---|---|
| `enc_n2_snake` | (−93, 44, 6, 8) | (−91, 44, 2, 1) | warm amber `#F2A93B` (heat lamp) | sandy floor, a warm flat rock, a branch to coil on, a small water dish — no stones stacked as a wall, no pumpkins, no boat |
| `enc_n2_chameleon` | (−87, 44, 5, 8) | (−86, 44, 2, 1) | violet UV-style `#9B6BE0` | tall leafy branches and a hanging vine from the ceiling, a leaf dish — no lanterns, no palm, no arch |
| `enc_n2_frog` | (−82, 44, 6, 8) | (−80, 44, 2, 1) | moist teal `#3CC7A0` | large leaves, a mossy log, a shallow dish, fine mist — no ferns, no barrel, no stepping plates |

- **Terrarium case (props, Q-333):** the case is the enclosure rect behind a **glass front** along z 44: a translucent pale-blue pane (alpha 0.25,
  no refraction; bright white frame lines and a few "shine" streaks; style = the existing `glass_door` model, drawn with the same blended pass, outlines only on the frame). Proposed models: `terrarium_front` (glass front wall segment, 1 m modules, ≤ 40 tris), `terrarium_frame` (case posts + lintel + lamp rail), `terrarium_lamp` (heat / UV lamp, emissive), `terrarium_rock_warm`, `terrarium_branch`, `terrarium_leaf_big`, `terrarium_moss_log`, `terrarium_dish`, `mist_puff` (billboard sprite). Their gates are the existing `glass_door`; no new door mechanism.
- **How animals enter:** exactly like hedgehog, bat and owl: the player leads the following pair through the door (−85, 39), along the hall (4 m wide), the gate opens while she leads animals within 3 m (`Game::opening_open`, 1.5 s behind them) and the animals step onto their home cells. The snake slithers (0.9 m/s home wander ≈ 0.5), the frog hops, the chameleon walks (quadruped rig).
- **Feed spot / stand cells (GAME-GARDEN 6a):** the child stands in the hall in front of the glass (hall cells z 43): `feed_spot` derived per case on the **gate side** (2 cells inside the glass on the gate edge: snake (−91, 44, 2, 1)…, derived when absent). Giving works through the glass at ≤ 2 m; the child never enters a terrarium. Pair gap (centre to centre): snake 1.0 m, chameleon 0.7 m (perch pair offset), frog 0.5 m; every case has ≥ 12 home cells (48, 40, 48 cells).
- **Collision / walkway:** hall interior and door are walkable path cells; the 4 m hall leaves ≥ 2 m free beside a following animal; nothing solid within 0.9 m beside the door posts (LAYOUT-038), diagonal approaches from 3 m stay free (LAYOUT-034), boards are wall-mounted and non-solid (Q-157, LAYOUT-032/041).
- **Info boards outside** (as Q-134): each facing south onto `path_n2_ring_n` / `path_n2_house`, with a board lamp; the panel shows riddle, **basic food and treat lines** (GAME-FEED "Info board"), facts and the pair note.
- **Indoor lights** `[[light]] kind = "indoor"`: three case lights (colours above) and one warm hall light.
- **Glass and perf:** 3 cases × ≈ 8 blended quads, one extra blended draw call; no new shader (PERF note below).

## Night food storage

`food_storage_n2` (−92, 26, 5, 6), door (−88, 29) east, interior (−91, 27, 3, 4): a copy of `food_storage_n1` shifted by −48 in x; a small **fridge** prop (`fridge`, GAME-ECON model, snowflake sticker) at the back wall is planned and waits for its model (the frozen-insects box stands inside at the back wall without it). Boxes (GAME-FEED §7, Q-181/Q-194 rules):

- **Outside**, plaza west row x −86.66, labels east, z 26.0 / 26.65 / 27.3 / 27.95: *Fisch / fish* (`fish`), *Grillen / crickets* (`crickets`), *Fliegen / flies* (`flies`) and the distractor *Käfer / beetles* (`beetles`). These are the **basic foods** of the level (and the crickets are also the frog's treat).
- **Inside** on the plank platform at the back wall (x −91.44, z 28.2 and 29.8) and at the fridge: *Eier / eggs* (`eggs`, snake treat), *Frostinsekten / frozen insects* (`frozen_insects`, chameleon treat, stands at the fridge). Treat-only foods stand **inside** (Q-336): the child looks for the treat after reading "Leckerli" on the board; the hut door and the hint lead there.
- Inside crates (unlabelled stock) as night_1: eggs ×2, crickets ×2 optional.

## Hiding places

`[[hiding_place]]` fields as night_1. Wander area: radius 3 m, grass (scenery cells walkable), clipped to the rect, 4-connected from the spot (scratch script 2026-10-03). Standing points: walkable cell centres ≤ 2.5 m from the own board plus the hall cells in front of the own gate.

| Id | Animal | Area rect | Spot | Perch | Wander cells | Min distance wander cell ↔ own standing points |
|---|---|---|---|---|---|---|
| `loc_stone_wall` | snake | (−118, 8, 6, 5) | (−116, 9) | — | 16 | 34.7 m |
| `loc_pumpkins` | snake | (−118, 18, 6, 6) | (−116, 20) | — | 27 | 26.6 m |
| `loc_rowing_boat` | snake | (−118, 27, 6, 7) | (−116, 30) | — | 21 | 22.1 m |
| `loc_lanterns` | chameleon | (−107, 8, 6, 5) | (−103, 9) | 1.8 m (low lantern branch) | 13 | 28.3 m |
| `loc_palm` | chameleon | (−100, 8, 7, 5) | (−99, 10) | 2.2 m (lowest leaf stalk) | 20 | 25.6 m |
| `loc_vine_arch` | chameleon | (−79, 12, 5, 4) | (−75, 15) | 2.0 m (crossbar of the arch) | 11 | 22.5 m |
| `loc_stepping_stones` | poison dart frog | (−88, 8, 5, 5) | (−86, 10) | — | 25 | 24.1 m |
| `loc_ferns` | poison dart frog | (−118, 39, 7, 8) | (−114, 43) | — | 28 | 29.2 m |
| `loc_rain_barrel` | poison dart frog | (−111, 46, 6, 6) | (−108, 48) | — | 22 | 24.4 m |

All nine have ≥ 9 cells (≥ 11), a cell ≥ 2 m from the spot, no solid or path cell, are reachable from the entry over walkable cells and keep **≥ 22 m** (min 22.1). Spots of different animals are **≥ 12.1 m** apart (LAYOUT-N2-007; closest `loc_vine_arch`–`loc_stepping_stones`); rects of different animals do not overlap. The perched chameleon pair sits 0.7 m apart on a 1.5 m wide branch (GAME-FAMILY rule 12); the lantern tree's low branch, the palm's lowest stalk and the arch's crossbar are modelled 1.5 m wide.
**Picking rule:** as night_1 (seeded uniform pick per animal, own RNG).

## Riddle details (night clues)

| Place | What the child sees / hears | Riddle guards |
|---|---|---|
| `loc_stone_wall` | a long low wall of flat grey stones with moss in the gaps, still warm; the snake lies along its foot in the corner | only stone wall; not the level-1 rock hill / level-2 sun rocks (grey stacked pieces, no cave, no big boulder) |
| `loc_pumpkins` | big round orange pumpkins, creeping vines with huge rough leaves | only pumpkins; no vegetable beds (level-1 garden) |
| `loc_rowing_boat` | a blue wooden boat upside down on the grass, two oars leaning on it | no water, no sails or flag (level-1 pirate ship is a ship) |
| `loc_lanterns` | a tree full of colourful paper lanterns glowing red, yellow, green, blue | not the string lights / lantern posts (those are plain warm lamps) |
| `loc_palm` | a slim palm with a rough trunk, no branches, a fan of big leaves, brown coconuts | only palm; no bananas (monkey food) |
| `loc_vine_arch` | a wooden arch with curtains of green creepers and pink flowers, a crossbar | no blossoms on a tree (level-2 koala), no flowerpots (night_1) |
| `loc_stepping_stones` | round smooth plates in a row over wet green moss, soft squelching | no river, pond, bridge or waterfall (day and night_1 water) |
| `loc_ferns` | head-high giant ferns around an old stump, mist on the ground | no mushrooms, moss ring or big old tree (night_1 loc_mushrooms) |
| `loc_rain_barrel` | a wooden barrel with iron rings, a small eave with a drain pipe dripping *tipp, tipp* | only barrel; no fountain / well (level 2) |

One animal's riddles never share their key detail: snake = warm stacked stones vs. round orange fruit vs. a blue upturned boat; chameleon = coloured lights vs. fan leaves and nuts vs. flowering creeper curtains; frog = squelchy plates vs. giant ferns and mist vs. drips and a barrel. No riddle contains a `kiga` word of any day place or of its own/any night place (MISS-013).

## Riddles and facts (texts in `assets/i18n/{de,en}/terrarium.ftl`)

Key pattern as night_1 (the file `terrarium.ftl` also holds `board-treat-<level>`, `welcome-level-night_2-<level>`, `map-level-night_2`, `hint-night-gate`): `mission-<animal>-riddle-<place>-<reading_level>`, `mission-<animal>-facts-<reading_level>`, `animal-<animal>` (+ `-more`), `mission-<animal>-home` (plural), `food-<id>`, `sign-terrarium-house`. `klasse1` sentences have ≤ 5 words, `kiga` is one word shown next to a picture.

Names: `animal-snake` Schlange / snake · `animal-chameleon` Chamäleon / chameleon · `animal-poison_dart_frog` Pfeilgiftfrosch / poison dart frog (short `kiga` word "Frosch" / "frog") · `sign-terrarium-house` Terrarienhaus / terrarium house · homes: "Super! Die Schlangen sind wieder zu Hause." / "Great! The snakes are home again." (chameleons, frogs likewise).

### `snake` — facts

| Level | de | en |
|---|---|---|
| kiga | Schuppen | scales |
| klasse1 | Ich habe keine Beine. Ich schlängele mich. Ich mag Wärme. | I have no legs. I wiggle along. I like warmth. |
| klasse2 | Schlangen haben keine Beine und kriechen auf dem Bauch. Ihre Haut ist trocken und glatt, nicht nass. Wenn sie gewachsen sind, streifen sie die alte Haut ab. Unsere Schlange ist ganz freundlich. | Snakes have no legs and glide on their belly. Their skin is dry and smooth, not slimy. When they have grown, they slip out of their old skin. Our snake is very friendly. |
| klasse3 | Schlangen haben keine Beine und bewegen sich mit kräftigen Muskeln am ganzen Körper vorwärts. Sie sind wechselwarm: Erst wenn sie sich an einem warmen Platz aufgewärmt haben, werden sie munter. Mit der Zunge „riechen“ sie ihre Umgebung. Wenn sie gewachsen sind, streifen sie ihre alte Haut ab wie einen zu engen Pullover. Unsere Kornnatter ist völlig harmlos. | Snakes have no legs and move along using strong muscles all over their body. They are cold-blooded: only when they have warmed up in a warm spot do they wake up. They "smell" with their tongue. When they have grown, they slip out of their old skin like a jumper that is too tight. Our corn snake is completely harmless. |

### `snake` — riddles

| Place | Level | de | en |
|---|---|---|---|
| `loc_stone_wall` | kiga | Mauer | wall |
| | klasse1 | Ich bin lang und glatt. Es ist warm hier. Ich liege zwischen Steinen. | I am long and smooth. It is warm here. I lie between stones. |
| | klasse2 | Die Schlange liegt am Fuß einer langen, niedrigen Reihe aus aufgestapelten Steinen. Die Steine sind vom Tag noch warm, und zwischen ihnen wächst weiches Moos. | The snake lies at the foot of a long, low row of stacked stones. The stones are still warm from the day, and soft moss grows between them. |
| | klasse3 | Schlangen lieben Wärme, und die holen sie sich aus der Sonne. Unsere Schlange liegt dort, wo flache graue Steine zu einer langen, niedrigen Reihe aufgestapelt sind. Auch jetzt in der Nacht sind sie noch ein bisschen warm, und in den Fugen wächst weiches grünes Moos. Ganz hinten in der Ecke, bei der Hecke, hört die Reihe auf. | Snakes love warmth, and they get it from the sun. Our snake lies where flat grey stones are stacked in a long, low row. Even now at night they are still a little warm, and soft green moss grows in the gaps. The row ends right at the back, in the corner by the hedge. |
| `loc_pumpkins` | kiga | Kürbisse | pumpkins |
| | klasse1 | Hier liegen dicke Kugeln. Sie sind orange. Die Ranken sind rau. | Here lie fat round things. They are orange. The vines are rough. |
| | klasse2 | Die Schlange hat sich zwischen großen, runden, orangen Früchten versteckt, die auf dem Boden liegen. Darüber breiten sich riesige, raue Blätter aus. | The snake has hidden between big, round, orange fruits lying on the ground. Huge, rough leaves spread above them. |
| | klasse3 | In einer Ecke des Gartens hat sich etwas Langes zusammengerollt. Rund um die Schlange liegen dicke, glatte Früchte in leuchtendem Orange, manche so groß wie ein Kinderkopf. Lange Ranken mit riesigen, rauen Blättern kriechen über den Boden. Man kann kaum sehen, wo die Früchte aufhören und die Schlange anfängt. | Something long has curled up in one corner of the garden. All around the snake lie fat, smooth fruits in bright orange, some as big as a child's head. Long vines with huge, rough leaves creep over the ground. It is hard to see where the fruits end and the snake begins. |
| `loc_rowing_boat` | kiga | Boot | boat |
| | klasse1 | Ich liege unter Holz. Es ist blau. Daneben lehnen zwei Ruder. | I lie under wood. It is blue. Two oars lean next to it. |
| | klasse2 | Die Schlange ist unter etwas Großem aus Holz verschwunden. Es ist blau gestrichen und liegt umgedreht im Gras. Zwei lange Ruder lehnen daneben. | The snake has slipped under something big made of wood. It is painted blue and lies upside down in the grass. Two long oars lean beside it. |
| | klasse3 | Früher sind zwei Kinder damit gerudert, heute liegt es umgedreht im Gras und dient als Dach. Die blaue Farbe blättert schon ab, und neben dem runden Bauch lehnen zwei lange Ruder. Darunter ist es trocken und warm, genau richtig für eine Schlange. | Once two children rowed in it; now it lies upside down in the grass and works as a roof. The blue paint is already peeling, and two long oars lean beside its round belly. It is dry and warm underneath, just right for a snake. |

### `chameleon` — facts

| Level | de | en |
|---|---|---|
| kiga | Farben | colours |
| klasse1 | Ich ändere meine Farbe. Meine Augen drehen sich. Mein Schwanz rollt sich ein. | I change my colour. My eyes turn around. My tail curls up. |
| klasse2 | Chamäleons können ihre Farbe ändern, wenn sie sich freuen oder ärgern oder wenn es kühler wird. Ihre Augen drehen sich unabhängig voneinander, so sehen sie in zwei Richtungen gleichzeitig. Mit dem langen Schwanz halten sie sich fest. | Chameleons can change their colour when they are happy or cross or when it gets cooler. Their eyes turn independently, so they look in two directions at once. They hold on tight with their long tail. |
| klasse3 | Chamäleons wechseln ihre Farbe nicht nur, um sich zu verstecken, sondern auch, wenn sie sich freuen, ärgern oder wenn es wärmer oder kälter wird. Jedes Auge dreht sich allein, so schaut das Tier gleichzeitig nach vorn und nach hinten. Mit der langen, klebrigen Zunge schnappt es Grillen aus der Luft, schneller als man blinzeln kann. Der Schwanz kann sich einrollen und hält das Tier fest. | Chameleons change colour not only to hide, but also when they are happy, cross, or when it gets warmer or colder. Each eye turns on its own, so the animal looks forward and backward at the same time. With its long, sticky tongue it snaps crickets out of the air, faster than you can blink. Its tail can curl up and holds the animal tight. |

### `chameleon` — riddles

| Place | Level | de | en |
|---|---|---|---|
| `loc_lanterns` | kiga | Lampions | lanterns |
| | klasse1 | Ich sitze hoch oben. Um mich leuchtet es bunt. Rot, gelb und blau. | I sit up high. Colourful lights glow around me. Red, yellow and blue. |
| | klasse2 | Das Chamäleon sitzt auf einem Ast in einem Baum voller Papierlaternen. Sie leuchten rot, gelb, grün und blau und schaukeln leise. | The chameleon sits on a branch in a tree full of paper lanterns. They glow red, yellow, green and blue and sway gently. |
| | klasse3 | In diesem Baum wachsen keine Früchte, sondern Lichter: Viele runde Papierlaternen hängen an dünnen Schnüren in den Zweigen. Sie leuchten rot, gelb, grün und blau und schaukeln leise hin und her. Auf einem Ast mitten zwischen ihnen sitzt das Chamäleon, und weil es seine Farbe wechseln kann, ist es kaum zu sehen. | No fruit grows in this tree, only lights: many round paper lanterns hang on thin strings among the twigs. They glow red, yellow, green and blue and sway gently to and fro. The chameleon sits on a branch right in the middle of them, and because it can change its colour, it is hard to spot. |
| `loc_palm` | kiga | Palme | palm |
| | klasse1 | Mein Baum hat keine Äste. Oben wachsen große Fächer. Daran hängen braune Nüsse. | My tree has no branches. Big fans grow at the top. Brown nuts hang from them. |
| | klasse2 | Das Chamäleon hängt an einem schlanken Baum ohne Äste. Der Stamm ist rau, und ganz oben wachsen riesige Blätter wie große Fächer. Darunter hängen braune Nüsse. | The chameleon clings to a slim tree without branches. The trunk is rough, and right at the top grow huge leaves like big fans. Brown nuts hang below them. |
| | klasse3 | Dieser Baum sieht anders aus als alle anderen im Garten: Sein Stamm ist schlank und rau und hat keinen einzigen Zweig. Ganz oben breiten sich große gefiederte Blätter aus wie ein riesiger Fächer. Dazwischen hängen dicke braune Kokosnüsse. Das Chamäleon hält sich mit seinem Schwanz an einem der untersten Blattstiele fest. | This tree looks different from all the others in the garden: its trunk is slim and rough and has not a single twig. Right at the top, big feathery leaves spread out like a giant fan. Thick brown coconuts hang between them. The chameleon holds on with its tail to one of the lowest leaf stalks. |
| `loc_vine_arch` | kiga | Rankbogen | arch |
| | klasse1 | Hier hängen grüne Vorhänge. Rosa Blumen blühen. Oben liegt ein Balken. | Green curtains hang here. Pink flowers bloom. A beam lies across the top. |
| | klasse2 | Das Chamäleon sitzt auf dem Balken eines Torbogens aus Holz. Grüne Ranken mit rosa Blumen wachsen daran und hängen herab wie Vorhänge. | The chameleon sits on the beam of a wooden gateway. Green creepers with pink flowers grow on it and hang down like curtains. |
| | klasse3 | Wer durch diesen Torbogen aus Holz gehen will, muss erst die Vorhänge aus grünen Ranken zur Seite schieben. Zarte rosa Blumen wachsen daran und ein dicker Balken liegt oben quer darüber. Dort sitzt das Chamäleon, ganz still und fast so grün wie die Ranken. | Anyone who wants to walk through this wooden gateway must first push aside curtains of green creepers. Delicate pink flowers grow on them, and a thick beam lies across the top. The chameleon sits there, very still and almost as green as the creepers. |

### `poison_dart_frog` — facts (the word "giftig" appears only here, as a fact, and never frightening)

| Level | de | en |
|---|---|---|
| kiga | bunt | colourful |
| klasse1 | Ich bin ganz bunt. Ich bin sehr klein. Ich hüpfe gern. | I am very colourful. I am very small. I like to hop. |
| klasse2 | Pfeilgiftfrösche sind winzig, aber sehr bunt. Ihre leuchtenden Farben sagen: Lass mich in Ruhe! Sie leben im warmen, feuchten Regenwald. Hier im Zoo sind sie harmlos und fressen kleine Fliegen. | Poison dart frogs are tiny but very colourful. Their bright colours say: Leave me alone! They live in the warm, wet rainforest. Here in the zoo they are harmless and eat little flies. |
| klasse3 | Pfeilgiftfrösche leben im Regenwald in Südamerika. Ihre kräftigen Farben sind eine Warnung: In der Wildnis sind manche giftig, deshalb lassen Feinde sie in Ruhe. Das Gift stammt aus ihrem Futter im Regenwald, bei uns im Zoo sind die Frösche deshalb ungefährlich. Aus den Eiern schlüpfen Kaulquappen, die das Männchen auf dem Rücken zum Wasser trägt. | Poison dart frogs live in the rainforest of South America. Their strong colours are a warning: in the wild some are poisonous, so enemies leave them alone. The poison comes from their food in the rainforest, so here in the zoo the frogs are harmless. Tadpoles hatch from the eggs, and the father carries them on his back to the water. |

### `poison_dart_frog` — riddles

| Place | Level | de | en |
|---|---|---|---|
| `loc_stepping_stones` | kiga | Trittsteine | stepping stones |
| | klasse1 | Der Boden ist weich. Alles ist nass. Es quatscht leise. | The ground is soft. Everything is wet. It squelches softly. |
| | klasse2 | Der Frosch sitzt auf einer Reihe runder, glatter Platten, die über nasses, grünes Moos führen. Wer darüberläuft, hört es leise quatschen. | The frog sits on a row of round, smooth plates leading across wet green moss. Anyone who steps near hears a soft squelch. |
| | klasse3 | Hier ist der Boden immer nass und weich wie ein Schwamm. Wer trockene Füße behalten will, hüpft über eine Reihe runder, glatter Platten, die wie kleine Inseln im grünen Moos liegen. Auf einer davon sitzt der kleine Frosch, ganz still, mit glänzender Haut. | The ground here is always wet and soft as a sponge. Anyone who wants dry feet hops across a row of round, smooth plates that lie like little islands in the green moss. On one of them sits the little frog, very still, with shiny skin. |
| `loc_ferns` | kiga | Farne | ferns |
| | klasse1 | Hier ist es feucht. Riesige Wedel wachsen. Alles ist grün. | It is damp here. Giant fronds grow. Everything is green. |
| | klasse2 | Der Frosch sitzt unter riesigen grünen Wedeln, die sich wie Schirme über ihn biegen. Es ist feucht, und über dem Boden hängt feiner Nebel. | The frog sits under giant green fronds that bend over it like umbrellas. It is damp, and a fine mist hangs above the ground. |
| | klasse3 | Neben einem alten Baumstumpf wächst ein ganzer Wald aus riesigen, gefiederten Wedeln. Sie sind so hoch wie ein Kind und biegen sich wie grüne Schirme. Es ist kühl und feucht, und über dem Boden hängt ein feiner Nebel. Dazwischen leuchtet etwas Buntes. | Next to an old tree stump grows a whole forest of giant, feathery fronds. They are as tall as a child and bend over like green umbrellas. It is cool and damp, and a fine mist hangs over the ground. Something colourful glows in between. |
| `loc_rain_barrel` | kiga | Regentonne | rain barrel |
| | klasse1 | Es tropft: tipp, tipp, tipp. Neben mir steht ein Fass. Es ist aus Holz. | It drips: tip, tip, tip. A barrel stands next to me. It is made of wood. |
| | klasse2 | Der Frosch sitzt neben einem großen Fass aus Holz. Von einem kleinen Dach tropft es hinein: tipp, tipp, tipp. | The frog sits next to a big wooden barrel. Water drips into it from a little roof: tip, tip, tip. |
| | klasse3 | Unter einem kleinen Vordach steht ein großes, rundes Fass aus Holz mit dunklen Eisenringen. Aus dem Rohr tropft es gleichmäßig hinein: tipp, tipp, tipp. Am Rand lehnt ein großes Blatt, und genau darunter sitzt der Frosch und hört zu. | Under a small eave stands a big round wooden barrel with dark iron rings. Water drips in steadily from the pipe: tip, tip, tip. A big leaf leans against the rim, and right underneath it sits the frog, listening. |

*English riddle wording differs slightly from the table where an English `kiga` word would otherwise occur in a riddle (RESC-011 / MISS-013, `night2_riddles_never_name_a_place`): `stones` → `rocks` (snake), `lanterns` → `paper lamps`, `barrel` → `tub`, `arch` / `fan` sentences shortened to ≤ 5 words (READ-002); the German texts are as in the tables.*

Info-board treat line (implemented as a prefix line before the facts: `board-treat-<level>` + the treat's food word; the basic food keeps its existing line): de "Leckerli: Eier", en "Treat: eggs"; the planned combined wording of GAME-FEED "Info board": de "Grundfutter: Fisch · Leckerli: Eier" (snake), "Grundfutter: Grillen · Leckerli: Frostinsekten" (chameleon), "Grundfutter: Fliegen · Leckerli: Grillen" (frog); en "Basic food: fish · Treat: eggs", "Basic food: crickets · Treat: frozen insects", "Basic food: flies · Treat: crickets".

## Barrier (edit of night_1)

| Id | Kind | Cells | Unlock | In-world explanation |
|---|---|---|---|---|
| `barrier_n1_garden` | `closed_gate` (a wooden garden gate in the west hedge of night_1, with a lantern, a moon sign and a small padlock; stand-in model: the `gate_zoo` level gate at the entry, Q-331) | night_1 (−72, 25, 2, 2) | `unlock_after = "night_1"`, `opens_at = "night"` | "Terrarium garden — opens when the first night animals are home": the sign shows moon + three small silhouettes (snake, chameleon, frog) + a lock; the padlock springs open and the lantern turns green when night_1 is complete. Never invisible, never a wall. |

Edits in `night-1.toml` (spec rows in night-1.md, LAYOUT-N1-014): `hedge_n1_west` shortened to (−72, 6, 2, 19) and the new `hedge_n1_west_n` (−72, 27, 2, 27) (the old id keeps the south part: `loc_brush_pile` names it as scenery); new `path_n1_gate` (path, main) (−72, 25, 8, 2: the street runs under the gate cells, LAYOUT-040) from `path_n1_ring_w` west through the free band between `loc_flowerpots` (z 18…23) and `loc_mushrooms` (z 27…33); one lantern `n1_lantern_gate` at (−66.5, 25.25) (inside the path edge) post at (−66, 24.7). The band z 24…26 holds no wander cell, so LAYOUT-N1-006/007/008 stay valid.

## Walking and pacing

Estimates (8-neighbour, 1.93 m/s paths, 1.45 m/s grass; LAYOUT-N2-005 prints exact values):

| From → to | ≈ time |
|---|---|
| entry → food boxes | 7.5 s |
| entry → map board | 2 s |
| entry → first board (`board_n2_frog`, standing cell (−81, 37)) | 9–10 s (≤ 15 s) |
| food boxes → boards / house door | 8 s / 7 s |
| house door → gates (hall) | 4 s |
| `loc_stone_wall` → `loc_pumpkins` → `loc_rowing_boat` | 7 s, 6 s |
| `loc_rowing_boat` → `loc_ferns` → `loc_rain_barrel` | 8 s, 5.5 s |
| `loc_lanterns` → `loc_palm` | 2.5 s |
| `loc_palm` → `loc_stepping_stones` → `loc_vine_arch` | 7 s, 7 s |
| `loc_stepping_stones` → food boxes | 9 s |

Every place has a neighbour ≤ 10 s; board → own spot 15–30 s (places far on purpose). **Time budget:** three animals × (board + food + search + lead home) ≈ 8–12 min plus the wait for pairs and the optional treats: inside the **10–20 min per level** rule (Q-202). `night_1` ≈ 8–12 min; the whole night (both levels) ≈ 20–25 min but split by design: **night_2 is optional and can wait for another night** (progress saved, the moon door and the gate stay open). Walk back to a bed: night_2 plaza → level-1 bed ≈ 100–110 m ≈ 60 s on paths (the 🛏 hint offers the nearest bed; Q-337 asks for a bed next to the terrarium house — recommendation: no).

**Unlock flow:** night_1 complete → celebration → `barrier_n1_garden` opens (lantern turns green, hint 🚪) → the child may walk through (moon-door style), do the three missions, then sleep. Day progression is unchanged: morning after the first completed night opens level 2 (Q-078/Q-091).

## Lights

`[[light]]`: 10 lantern posts along entry, loop and links (≥ 0.25 m inside the path edge, outside hiding rects and scenery), two string lights over the plaza, wall lamps at the house door and hut door, board lamps ×4 (three boards + map board), indoor lights ×4 (hall + three terrariums, colours above), the lantern tree's own 20 lamps (emissive props, no lights), one lantern at the gate (`n1_lantern_gate` in night-1.toml). The hiding places have no lantern of their own (the player's hand lantern finds the shining eyes; eyeshine is real for chameleon and frog).

## Never stuck (binding)

Applies GAME-LAYOUT "Level design rules" and GAME-HINT; **the child cannot get stuck in night_2 — provided the rows below are implemented and tested.**

1. **Hint targets.** From any reachable state with an incomplete night_2 mission the 🧭 has a target of priority ≤ 3: board → basic food box → search area / gate → (never the treat as the only target). At night with night_1 complete and night_2 incomplete and the gate open, the hint candidates are the gate 🚪 (priority 3) and the bed 🛏; the gate comes first while the player is in night_1 or night_2 (NIGHT-031). With the gate still closed (night_1 incomplete) the hint stays on night_1's missions (priority ≤ 3, as today). Treats (garden, box) are priority > 3 and never the only hint (HINT-024).
2. **Findable.** Every hiding place reachable over walkable cells (LAYOUT-N2-001/007); frogs 0.4 m and snakes lying low are visible through their shining eyes within 2.5 m and by the 🧭 arrow; the `loc_*` rect has no cell hidden behind solid scenery (4-connected wander area from the spot). Perched chameleons are reachable at the pair offset.
3. **Pair/group states.** `snake`, `chameleon`, `poison_dart_frog` are species with two members: one member home / one waiting, a member following while the other waits, a split pair after save/restore (HINT-021 pattern), a baby in the case. Code must look up groups by species, never "the first animal of the species" (`Game::animal(id)`, `animal_index` bug class) — checklist item for the implementer; the species ids are new strings, so every hard-coded species match (animals.rs, hints.rs, night.rs, scene.rs `PLANT_KINDS`-style tables, sound cue tables, compass icons) is extended in the same change.
4. **Safety net.** After 120 s without mission progress the hint points straight at the missing animal; after a further 60 s it comes towards the player (HINT-020), for every species including these.
5. **Fuzz.** The seeded fuzz test (HINT-021) is extended: after night_1 is done it opens the gate, plays night_2 (wrong food, treat before basic food, wrong gate, split pairs, save/restore mid-way, babies) and must reach "all animals of night_2 home" and then dusk → bed (HINT-025). It also covers night_1 with the new gate in the way.
6. **Treat and baby are optional** (HINT-024, FAM-031): refusing a food, a wrong treat, or never giving one blocks nothing.
7. **New mechanic report:** new states = a fed-with-treat baby in a glass case, a treat box inside the hut; both are covered by the rows above. The child **cannot** get stuck: night_2 only adds optional work (sleep is always available), every step has a ≤ 3 hint.

## Behaviour

1. `night_2` contains exactly the elements of the table; `night-2.toml` mirrors it (LAYOUT-005).
2. Bounds (−120, 6, 48, 48) overlap no other level; every border cell is solid except the entry cells (−73, 25…26).
3. With the gate open every enclosure, building, board, food box and hiding place is reachable from the entry; with it closed no cell of `night_2` is reachable from `night_1`.
4. No two solid elements overlap; no path cell lies under a solid element.
5. The terrarium house hall and door are walkable (surface `path`); each terrarium's gate edge-adjacent to a hall cell; boards outside.
6. Every hiding place has ≥ 9 wander cells, a cell ≥ 2 m from the spot, no solid/path cell, inside its rect; places of different animals do not overlap; spots ≥ 12 m apart.
7. Every wander cell is off-screen in the zoo view from its own standing points and ≥ 22 m from them (CAMV-008).
8. Scenery kinds / riddle element kinds (`stone_wall`, `pumpkin_patch`, `rowing_boat`, `lantern_tree`, `palm_tree`, `vine_arch`, `stepping_stones`, `fern_glade`, `rain_barrel`) occur once in `night_1` ∪ `night_2`.
9. Neighbouring points of interest ≤ 10 s; first board ≤ 15 s from the entry; steps ≤ 60 s apart (LAYOUT-046).
10. The storage holds one outside box each of `fish`, `crickets`, `flies`, `beetles` and one inside box each of `eggs`, `frozen_insects`; every box has a reachable standing cell.
11. Lights and boards as night_1.

## Mockups and art

Brief to write (art agent, not touched here): `art/environment/env_terrarium_house/brief.md` — `overview.png`: top-down cut-away of the house with three lit glass terrariums (amber / violet / teal) along the north wall, hall, door on the street, plaza with hut; `player_view.png`: the child in the hall looking at the three glass fronts with the animals visible behind the glass; props from the modular list plus the new terrarium models; mood of `env_night_house` but **warm and cosy**, tropical plants, nothing scary. Also needed: concept sheets for the three families (in review: `art/animals/{snake,chameleon,poison_dart_frog}_family`), a greybox of this layout, riddle-place mockups for the nine places (briefs after approval).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| LAYOUT-N2-001 | Given the joined levels with `barrier_n1_garden` open, then every enclosure, building, landmark, board, food box and hiding place of `night_2` has a reachable walkable cell next to / inside it from the night_1 spawn. | unit |
| LAYOUT-N2-002 | Given the barrier closed, then no cell of `night_2` is reachable; every border cell of `night_2` except (−73, 25…26) is solid; the night_1 border cells (−72…−71, 25…26) are the barrier. | unit |
| LAYOUT-N2-003 | Given `night-2.toml`, no two solid elements share a cell, no path under a solid, bounds disjoint from all levels. | unit |
| LAYOUT-N2-004 | Given this element table and `night-2.toml`, same ids, types, rects (LAYOUT-005). | unit |
| LAYOUT-N2-005 | Given 1.93 / 1.45 m/s, each pair of "Walking and pacing" is ≤ 10 s, the entry → first board ≤ 15 s, no mission step > 60 s apart. | unit |
| LAYOUT-N2-006 | Given missions snake, chameleon, poison_dart_frog, every candidate, the player on every cell ≤ 2.5 m from the own board or in front of the own gate, zoo camera at 45° steps and 10/14/20 m, then no wander cell is on screen and every one is ≥ 22 m from the standing points. | unit |
| LAYOUT-N2-007 | Given the hiding places, then ≥ 9 wander cells each (16, 27, 21, 13, 20, 11, 25, 28, 22), a cell ≥ 2 m from the spot, no solid/path cell, inside the rect; different animals' places do not overlap; all different-animal spot pairs ≥ 12 m. | unit |
| LAYOUT-N2-008 | Given the hiding places, features and scenery ids exist, each scenery / riddle kind occurs once in the night levels, MISS-013 holds for the new riddles. | unit |
| LAYOUT-N2-009 | Given `terrarium_house`, interior and door walkable `path`, each indoor terrarium has its gate edge-adjacent to an interior cell, `model_rect` contains hall and cases, `terrarium = true` on the three enclosures. | unit |
| LAYOUT-N2-010 | Given the food boxes of night_2: outside `fish`, `crickets`, `flies`, `beetles`; inside `eggs`, `frozen_insects`; each with a reachable standing cell within 2 m. | unit |
| LAYOUT-N2-011 | Given the lights, same checks as LAYOUT-N1-011 (posts, board lamps, indoor lights with the colours above). | unit |
| LAYOUT-N2-015 | Given the new foods, animals, places and the lantern-gate hint, then the host has an icon for each (no reading needed). | vitest |
| NIGHT-N2-001 | Given the zoo with night_2, then the texts of every riddle (9 places x 4 reading levels), the facts, names, homes, six food words, the treat line, welcome, map and gate-hint texts exist in de and en; no klasse1..3 riddle contains the `kiga` word of any night place or a food word (RESC-011 / MISS-007 / MISS-013). | unit |
| NIGHT-N2-002 | Given the garden: the pairs start together a gap apart (40 seeds), places and terrariums fit two adults and a baby, the basic food makes both follow and enter, a treat or wrong food does not (FAM-021..026, FEED-37). | unit |
| NIGHT-N2-003 | Given a pair at home, then the basic food gives hearts and no baby, the treat exactly one baby (once, saved), a wrong food is refused; the info board names the treat (FEED-38, FAM-031 for the three species). | unit |
| LAYOUT-N2-012 | Given night_1 complete, when the player walks west through `path_n1_gate`, then she stands on `path_n2_entry`; the night_2 missions are in scope and the compass strip shows 3 icons (snake, chameleon, frog). | e2e |
| LAYOUT-N2-013 | Given the glass terrarium fronts in the zoo and first-person views, then the animals are visible through the glass and the gate opens when a pair is led (door/gate rows LAYOUT-034). | e2e |
| LAYOUT-N2-014 | Given the layout-true mockups, a reviewer names each place's clue without text; nothing is scary (NIGHT-009); the poison dart frog is shown friendly. | manual |
| LAYOUT-N1-014 | Given `night-1.toml` with `path_n1_gate` and `barrier_n1_garden`, then LAYOUT-N1-002/003/006/007 stay green (the gate band holds no wander cell) and the barrier cells are the only non-solid-when-open border cells besides the entry. | unit |

Further rows: NIGHT-030..032 (night.md), HINT-024/025 (hints.md), FAM-031/032 (families.md), FEED-036..044 (feeding.md), FAM/ANIM rows for the species.

**Implemented tests (2026-10-04):** `crates/zoo-core/tests/night2_layout.rs` (LAYOUT-N2-001…011, LAYOUT-N1-014), `night2_game.rs` (NIGHT-030, NIGHT-031 compass, NIGHT-032, NIGHT-N2-002/003, HINT-025 states), `night2_content.rs` (NIGHT-N2-001), `hints.rs` (HINT-025 / NIGHT-031 following the hints through the garden with messy states and save/restore; HINT-027 loop detector inside), `web/src/ui.test.ts` (LAYOUT-N2-015), e2e `web/tests/e2e/night2.spec.ts` (LAYOUT-N2-012/013). **Not covered yet:** FEED-036 / 039 / 040 / 041 / 042 / 044 and FAM-032 for the 13 older species (the basic-food / treat rule is built for the three new species only; FEED-043 is covered by the 20 pictograms).

## Open questions

- Q-330…Q-339, Q-350…Q-352 answered 2026-10-04 ("implement as recommended"); see open-questions.md.
- Open for the user: the 16-species treat rule for the older species (Q-350 table), the glass-case look and the real house model, the fridge prop and the nine scenery models (see "Mockups and art"), sign silhouettes for snake / chameleon / frog.
