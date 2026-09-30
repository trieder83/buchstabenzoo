---
id: GAME-LEVEL-NIGHT-1
title: Night level 1 — the moonlit forest garden (hedgehog, bat, owl)
aspect: gameplay
module: levels
status: draft
depends_on: [GAME-LAYOUT, GAME-NIGHT, GAME-RESCUE, CONT-MISSIONS, GAME-PLAYER, GAME-CAMERA-VIEWS, GAME-LEVEL-1]
test_prefix: LAYOUT-N1
updated: 2026-09-28
---

# Night level 1 — the moonlit forest garden (hedgehog, bat, owl)

Owned by the `zoo-level-designer` agent. Layout data: `assets/levels/night-1.toml` (level id
`night_1`). Coordinate system, element types and general rules: GAME-LAYOUT. Night rules:
GAME-NIGHT. Mood: `art/environment/env_night_overview/overview.png` (approved mood image — the
layout below is the real one; see "Mockups").

## Goal

The first **night zoo** (GAME-NIGHT rule 4): a small moonlit forest garden **behind the moon
door** in the west wall of level 1. It teaches the rescue loop at night three times (GAME-NIGHT
rule 5: lantern, shining eyes, night clues — moonlight, sounds, smells):

| Mission | Enclosure (indoor, night house) | Food box | Candidate hiding places (one per playthrough) | What the riddles rely on (CONT-MISSIONS "Night level 1") |
|---|---|---|---|---|
| `hedgehog` | `enc_n1_hedgehog` (warm red-orange light) | Käfer / beetles | `loc_brush_pile` (south-west corner), `loc_flowerpots` (west), `loc_mushrooms` (west) | heap of dry twigs in the hedge corner, rustling · clay pots with white night flowers, sweet smell · mushroom ring on moss under an old tree, smell of damp earth |
| `bat` | `enc_n1_bat` (soft blue light) | Obst / fruit | `loc_windmill` (south), `loc_fireflies` (south), `loc_hollow_tree` (south-east) | little windmill with turning sails, whirring · meadow with dancing fireflies, crooked tree · very thick old tree with a round knothole |
| `owl` | `enc_n1_owl` (soft blue light) | Käfer / beetles (Q-077) | `loc_moon_pond` (south-east), `loc_hilltop` (north-west), `loc_fir` (north-west corner) | moon mirrored in still water, reeds, a post · hill with a big stone, no trees, brightest moonlight · tall dark pointed fir with cones |

Plus the entry path from the moon door, a small **plaza** with string lights, a **map board**,
the **night food storage** (a food hut), the **night house** (enterable, three dim indoor
enclosures) and a **loop path** (north, west and south ring) around a dense old-tree grove.

**Design idea (as level 1): a ring around a hidden middle.** Everything the child needs first
(plaza, food hut, boards, night house) lies in the **east**, close to the moon door. The nine
hiding places lie in the **far west and south** of the loop, all ≥ 22 m from their own board
and gate, behind the dense groves `grove_n1_center` and `grove_n1_north`.

## Proposals used in this level (not yet decided)

| Topic | Proposal used here | Question |
|---|---|---|
| Night level data | `[level] time = "night"`; reachable only through the level-1 `moon_door` (open every night after level 1's nightfall, closed by day); `[[entry]] entry_n1_moon` | Q-133 |
| Night house | Enterable hall (`building`, `interior` + `door`, Q-092) with the three indoor enclosures (`indoor = true`) directly north of it, gates opening into the hall; `model_rect` = the whole house model (hall + enclosure wing); roof cut away while the player stands on a hall or door cell; **info boards outside** in front of the house (the hall cannot hold solid board cells, LAYOUT-003) | Q-134 |
| Night food | Own food hut `food_storage_n1` with 4 boxes: *Käfer* (hedgehog **and** owl), *Obst* (bat), distractors *Würmer*, *Nektar*; food ids `beetles`, `fruit`, `worms`, `nectar` | Q-135 |
| Riddle uniqueness | Night levels are their own scope for LAYOUT-024 scenery kinds (the night pond and hill do not clash with the level-1 pond and rock hill: day and night riddles are never active together); night riddles still avoid every day `kiga` word | Q-136 |
| Lights, items, props | `[[light]]` (lantern posts, string lights, wall/board lamps, indoor lights with `color`), furniture `[[prop]]`, interactables `[[item]]` | Q-137 |
| Telescope | `telescope_n1`: a toy star telescope halfway along the north ring — a resting point that keeps the walk to the north-west ≤ 10 s per leg; interactive (look at the moon) later? | Q-138 |
| Perched animals | bat hangs / owl sits at `perch_height_m` beside the spot (as Q-094); ground wander area is the fallback | Q-094 |
| Poses | bat `hang` (upside down), hedgehog snuffling | Q-043 |
| 22 m haze rule (FIX-056) | `tree_hollow_n1` and `loc_hollow_tree` moved 3 m south-east into the corner by the east hedge; the south places reach the plaza over `loc_moon_pond` → food boxes (see "Hiding places", "Walking distances"). | Q-145 |

## Spawn, entry and camera

- **Entry:** `[[entry]] entry_n1_moon`, cells (−25, 29…30) — edge-adjacent to the level-1
  barrier `moon_door` (x −24…−23, z 29…30). Walking west through the open moon door the player
  steps onto `path_n1_entry`. These are the only walkable border cells (LAYOUT-021).
- **Spawn** (only used when a save inside `night_1` has no position): cell (−28, 29) on the
  entry path, facing **−x (west)**, camera east of the player looking west (GAME-PLAYER §2).
- **First view** (camera looking west): the plaza under its string lights straight ahead with
  the food hut and its boxes in front of it, the map board on the right (north), the night house with
  its blue and red-orange portholes and the three lit boards at the upper right.

## Map

Scale **1 character = 1 m**. North (+z) is up, x axis below. Generated from
`assets/levels/night-1.toml` (scratch script, 2026-09-27).

```
   53 %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
   52 %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
   51 %%.............TTTTTTTTTTTTeeeeeeaaaaauuuuuuTT%%
   50 %%.............TTTTTTTTTTTTeeeeeeaaaaauuuuuuTT%%
   49 %%........YY...TTTTTTTTTTTTeeeeeeaaaaauuuuuuTT%%
   48 %%........YY9..TTTTTTTTTTTTeeeeeeaaaaauuuuuuTT%%
   47 %%.............TTTTTTTTTTTTeeeeeeaaaaauuuuuuTT%%
   46 %%.............TTTTTTTTTTTTeeeeeeaaaaauuuuuuTT%%
   45 %%..........==.TTTTTTTTTTTTeeeeeeaaaaauuuuuuTT%%
   44 %%.^^^......==.TTTTTTTTTTTTeeggeeaggaauugguuTT%%
   43 %%.^^^8.....==.TTTTTTTTTTTTNnnnnnnnnnnnnnnnNTT%%
   42 %%.^^^......==.TTTTTTTTTTTTNnnnnnnnnnnnnnnnNTT%%
   41 %%..........==.TTTTTTTTTTTTNnnnnnnnnnnnnnnnNTT%%
   40 %%..........==.............NnnnnnnnnnnnnnnnNTT%%
   39 %%..........==.............NNNNNNNNDNNNNNNNNTT%%
   38 %%..........==......t.........i..i===..i......%%
   37 %%......=============================.........%%
   36 %%......=============================.........%%
   35 %%......=============================.........%%
   34 %%......===......................=========....%%
   33 %%AA....===......................=========....%%
   32 %%AAmm..===..TTTTTTTTTTTTT.......=========M...%%
   31 %%..mm..===..TTTTTTTTTTTTT..FFFFF=========M...%%
   30 %%..3...===..TTTTTTTTTTTTT..FFFFF===============
   29 %%......===..TTTTTTTTTTTTT..FFFFD===========S===
   28 %%......===..TTTTTTTTTTTTT..FFFFFf========....%%
   27 %%......===..TTTTTTTTTTTTT..FFFFFf========....%%
   26 %%......===..TTTTTTTTTTTTT..FFFFFf========....%%
   25 %%......===..TTTTTTTTTTTTT.......=========.bb.%%
   24 %%......===..TTTTTTTTTTTTT.......=========....%%
   23 %%......===..TTTTTTTTTTTTT.......===..........%%
   22 %%P.....===..TTTTTTTTTTTTT.......===..........%%
   21 %%P.....===..TTTTTTTTTTTTT.......===..........%%
   20 %%..2...===..TTTTTTTTTTTTT.......===..........%%
   19 %%......===..TTTTTTTTTTTTT.......===..........%%
   18 %%......===..TTTTTTTTTTTTT.......===..........%%
   17 %%......===......................===........HH%%
   16 %%......===......................===........HH%%
   15 %%......=================================....6%%
   14 %%......=================================.....%%
   13 %%......=================================.....%%
   12 %%...................******............jj.....%%
   11 %%...................******..........~~~~~~...%%
   10 %%..1...............y5*****.......7..~~~~~~...%%
    9 %%,,,..........WW4...******..........~~~~~~...%%
    8 %%,,,..........WW....******..........~~~~~~...%%
    7 %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
    6 %%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
      |.......|.......|.......|.......|.......|.......
  x:  -72     -64     -56     -48     -40     -32     
```

| Char | Meaning | Char | Meaning |
|---|---|---|---|
| `%` | tall hedge (level edge, solid) | `=` | path (entry, plaza, loop, trails) |
| `S` | spawn | `M` | map board `map_board_n1` |
| `b` | bench | `t` | toy telescope `telescope_n1` |
| `F` / `D` | food hut `food_storage_n1` / door (also the night-house door) | `f` | night food boxes (props on the plaza edge, Q-181 answered: outside) |
| `N` / `n` | night house walls / hall (interior, walkable) | `g` | glass gate of an indoor enclosure |
| `e` `a` `u` | indoor enclosure hedgehog / bat / owl | `i` | info board (with board lamp) |
| `T` | dense old-tree grove (`grove_n1_center`, `grove_n1_north`, `trees_n1_ne`) | `.` | grass (walkable, slower) |
| `,` | brush pile `brush_pile_n1` (scenery, walkable) | `P` | potting bench with flowerpots |
| `A` | big old mossy tree `tree_old_n1` | `m` | mushroom ring `mushroom_ring_n1` (scenery) |
| `^` | small hill `hill_n1` | `Y` | fir tree `fir_n1` |
| `W` | windmill `windmill_n1` | `y` | crooked tree `tree_crooked_n1` |
| `*` | firefly meadow `firefly_meadow_n1` (scenery) | `~` / `j` | pond `pond_n1` / jetty `jetty_n1` |
| `H` | hollow tree `tree_hollow_n1` | `1` `2` `3` | hedgehog spots `loc_brush_pile`, `loc_flowerpots`, `loc_mushrooms` |
| `4` `5` `6` | bat spots `loc_windmill`, `loc_fireflies`, `loc_hollow_tree` | `7` `8` `9` | owl spots `loc_moon_pond`, `loc_hilltop`, `loc_fir` |

East of x −25 (not drawn): the level-1 zoo wall with the moon door at z 29…30.

## Elements

Grid rect = `x, z, w, d` in 1 m cells (south-west corner + size). Solid = every type except
`path`. The table mirrors `night-1.toml` (LAYOUT-005).

| Id | Type | Grid rect (x, z, w, d) | Notes |
|---|---|---|---|
| `hedge_n1_west` | decoration (hedge) | -72, 6, 2, 48 | Level edge, permanent. |
| `hedge_n1_south` | decoration (hedge) | -70, 6, 46, 2 | Level edge, permanent. |
| `hedge_n1_north` | decoration (hedge) | -70, 52, 46, 2 | Level edge, permanent (level 3 lies east of its east end, beyond the zoo wall). |
| `hedge_n1_east_s` | decoration (hedge) | -26, 8, 2, 21 | East edge in front of the level-1 zoo wall (wall_west_s), south of the moon door. |
| `hedge_n1_east_n` | decoration (hedge) | -26, 31, 2, 21 | East edge in front of the level-1 zoo wall (wall_west), north of the moon door. |
| `path_n1_entry` | path (main) | -30, 29, 6, 2 | From the moon door (level 1) west to the plaza; cells (-25, 29..30) are the level entry. |
| `path_n1_plaza` | path (plaza) | -39, 24, 9, 11 | Night plaza under string lights; food boxes and the food hut door on its west edge, path to the night house north, s_link south. |
| `map_board_n1` | landmark (map_board) | -30, 31, 1, 2 | Picture map of the night garden (silhouettes, no text), with a board lamp. |
| `bench_n1_plaza` | decoration (bench) | -29, 25, 2, 1 |  |
| `food_storage_n1` | building (food_hut) | -44, 26, 5, 6 | `door` (-40, 29), `interior` (-43, 27, 3, 4): enterable. Small wooden food hut with warmly lit windows (env_night_overview); the 4 night food boxes stand in front of its east facade, two more real labelled food boxes inside (Q-135, Q-181 answered, Q-194 answered 2026-09-29). |
| `path_n1_house` | path (main) | -38, 35, 3, 4 | From the plaza north to the night-house door (-37, 39). |
| `night_house` | building (night_house) | -45, 39, 17, 5 | `interior` (-44, 40, 15, 4), `door` (-37, 39), `model_rect` (-45, 39, 17, 13). Rounded night house with a grass roof, painted moon, blue and warm red-orange porthole windows (env_night_house, Q-116). Inside: dim blue and red-orange light, three glass-fronted indoor enclosures along the north side, each with a small glass door (gate) and an enclosure sign above it. |
| `enc_n1_hedgehog` | enclosure | -45, 44, 6, 8 | gate (-43, 44, 2, 1), `indoor = true`, `home_wander_on = ["grass"]`. Indoor enclosure, warm red-orange light: straw nest box, a hollow log tunnel, low ferns, earth floor. No twig heap, no mushrooms, no flower pots (riddle guards). |
| `enc_n1_bat` | enclosure | -39, 44, 5, 8 | gate (-38, 44, 2, 1), `indoor = true`, `home_wander_on = ["grass"]`. Indoor enclosure, soft blue light: bare climbing branches and ropes under the ceiling to hang from, a fruit bowl on a shelf. No hollow tree, no windmill, no fireflies (riddle guards). |
| `enc_n1_owl` | enclosure | -34, 44, 6, 8 | gate (-32, 44, 2, 1), `indoor = true`, `home_wander_on = ["grass"]`. Indoor enclosure, soft blue light: perch poles, a wooden owl box high on the wall, a painted moon on the back wall. No fir tree, no hill, no pond (riddle guards). |
| `board_n1_hedgehog` | decoration (info_board) | -42, 38, 1, 1 | board of `enc_n1_hedgehog`, `mount = "wall"` (Q-157). Outside the night house, south of the hall wall, below its enclosure; board lamp (Q-134 answered: boards outside, the hall has no room for solid boards). |
| `board_n1_bat` | decoration (info_board) | -39, 38, 1, 1 | board of `enc_n1_bat`, `mount = "wall"` (Q-157). Left of the door path, flat on the facade; board lamp. |
| `board_n1_owl` | decoration (info_board) | -33, 38, 1, 1 | board of `enc_n1_owl`, `mount = "wall"` (Q-157). Right of the door path, below the owl enclosure; board lamp. |
| `trees_n1_ne` | decoration (tree_grove) | -28, 39, 2, 13 | `density = "dense"`, bush border. Old round trees between the night house and the east hedge. |
| `telescope_n1` | decoration (telescope) | -52, 38, 1, 1 | Toy star telescope on a wooden stand, pointing at the moon — a resting point halfway along the north ring (keeps the boards -> north-west walk <= 10 s per leg). Not a riddle detail; interactive use (look at the moon) comes later as a small bonus (Q-138 answered). |
| `path_n1_ring_n` | path (main) | -64, 35, 26, 3 | From the night-house path west to the west ring; passes the boards. |
| `path_n1_ring_w` | path (main) | -64, 13, 3, 22 | West side of the loop; the hedgehog places and the hill lie west of it. |
| `path_n1_ring_s` | path (main) | -61, 13, 25, 3 | South side of the loop; windmill, firefly meadow and pond lie south of it. |
| `path_n1_s_link` | path (main) | -39, 16, 3, 8 | From the south ring north to the plaza. |
| `path_n1_nw` | path (side) | -60, 38, 2, 8 | Short trail from the north ring up to the fir tree (ends just south of loc_fir); the hill lies to its west. |
| `path_n1_se` | path (side) | -36, 13, 5, 3 | Short path east from the south ring to the pond jetty. |
| `jetty_n1` | path (jetty) | -33, 12, 2, 1 | Small wooden jetty deck on the pond's north shore (look-out point over the mirrored moon). The owl of loc_moon_pond sits on a wooden post at the pond's west shore, (-35.6, 10.5). |
| `grove_n1_center` | decoration (tree_grove) | -59, 18, 13, 15 | `density = "dense"`, bush border. The hidden middle: old round trees with big friendly crowns (no hollow, no fir, no mushrooms — riddle guards); bush border on all walkable sides. |
| `grove_n1_north` | decoration (tree_grove) | -57, 41, 12, 11 | `density = "dense"`, bush border. Old round trees north of the ring, between the fir and the night house (reaches the house wall, so no dead-end strip remains). |
| `windmill_n1` | landmark (windmill) | -57, 8, 2, 2 | Little wooden garden windmill (4 m) with four slowly turning sails and a small balcony; soft whirring sound. The only windmill in the zoo (loc_windmill). |
| `tree_crooked_n1` | decoration (tree_crooked) | -52, 10, 1, 1 | Small crooked tree with a low horizontal branch in the firefly meadow (the bat hangs from the branch, loc_fireflies). |
| `pond_n1` | landmark (pond) | -35, 8, 6, 4 | Small round still pond; the moon and stars are mirrored in it (night water shader). Reeds at the rim. No lilies, no frogs, no ducks, no bridge (riddle guards vs. level-1 loc_pond / loc_river, Q-136). |
| `tree_hollow_n1` | decoration (tree_hollow) | -28, 16, 2, 2 | Very thick old tree with a big round knot hole at 2.5 m (env_night_overview), at the east hedge in the south-east corner of the loop. The only hollow tree in the zoo (loc_hollow_tree). Moved 3 m south-east by FIX-056 (old position x −29…−28, z 19…20; 22 m haze rule). |
| `potting_bench_n1` | decoration (potting_bench) | -70, 21, 1, 2 | Old wooden potting bench against the west hedge with stacked clay flower pots below and white night-scented flowers (evening primrose) in pots on top (loc_flowerpots). |
| `tree_old_n1` | decoration (tree_old) | -70, 32, 2, 2 | Big old mossy tree by the west hedge; a ring of mushrooms on the moss at its foot (loc_mushrooms). |
| `hill_n1` | landmark (hill) | -69, 42, 3, 3 | Small round grassy hill (2 m) with one big round stone on top, no trees — the brightest moonlit spot of the garden (loc_hilltop). |
| `fir_n1` | decoration (fir_tree) | -62, 48, 2, 2 | The one tall dark pointed fir tree in the north-west corner, with cones (loc_fir). The only fir in the zoo. |

## Night house (GAME-NIGHT rule 4; Q-134 answered)

A rounded house with a grass roof, a painted moon and blue / warm red-orange porthole windows
(`env_night_house`). The child walks in through the door (−37, 39) into a **visitor hall**
(interior x −44…−30, z 40…43, 15 × 4 m, surface `path`); along the hall's north side are the
**glass fronts** of the three indoor enclosures, each with a small glass door (the enclosure
gate, 2 m) and an enclosure sign above it. The roof and the walls above 1 m of the whole house
(`model_rect` (−45, 39, 17, 13)) are cut away while the player stands on a hall or door cell
(GAME-PLAYER §2, PLAY-028).

| Enclosure | Rect | Gate (south side, into the hall) | Light (Q-116) | Inside (riddle guards) |
|---|---|---|---|---|
| `enc_n1_hedgehog` | (−45, 44, 6, 8) | (−43, 44, 2, 1) | warm red-orange `#E8735A` | straw nest box, hollow-log tunnel, ferns, earth floor — no twig heap, mushrooms or flowerpots |
| `enc_n1_bat` | (−39, 44, 5, 8) | (−38, 44, 2, 1) | soft blue `#5B7FE0` | bare branches and ropes under the ceiling, a fruit bowl — no hollow tree, windmill or fireflies |
| `enc_n1_owl` | (−34, 44, 6, 8) | (−32, 44, 2, 1) | soft blue `#5B7FE0` | perch poles, an owl box high on the wall, a painted moon — no fir, hill or pond |

- **Info boards outside** (Q-134 answered): `board_n1_hedgehog` (−42, 38), `board_n1_bat`
  (−39, 38), `board_n1_owl` (−33, 38) hang in a row on the south wall (`mount = "wall"`: a flat
  panel on the facade behind their cell, not solid — no pocket beside the door, Q-157,
  LAYOUT-038), each below its enclosure, facing south onto `path_n1_ring_n` / `path_n1_house`, each with a `board_lamp`. The
  hall has no room for solid board cells (a board inside the building rect would overlap the
  building, LAYOUT-003). The child reads the riddle outside and leads the animal in through the
  door to its glass gate.
- Indoor lights: `[[light]] kind = "indoor"` with `color` per enclosure (red-orange over the
  hedgehog, blue over bat and owl) and one soft blue hall light — dim but readable (NIGHT-005).
- Leading an animal home: through the door (−37, 39), along the hall to the gate; the hall is
  4 m wide, so a following animal fits next to the player.

## Night food storage (Q-135 answered; boxes outside: Q-181 answered)

`food_storage_n1` (x −44…−40, z 26…31, door (−40, 29) on the east facade) is a small wooden food
hut with warm windows. It is **enterable** like every building with a door (GAME-LAYOUT
"Enterable buildings"): `interior` (−43, 27, 3, 4) is walkable floor, the door opens as the child
comes near, the roof, the upper walls and the "Futter" board hide inside in the zoo view (kept in
first person). Its four `[[food_box]]`es stay **outside** (Q-181 answered 2026-09-28) on the
plaza's west row in front of the east facade (x −38.66, labels facing east, towards the spawn
camera), in one row south of the door: *Käfer* / *beetles* (z 26.0), *Obst* / *fruit* (26.65),
*Würmer* / *worms* (27.3), *Nektar* / *nectar* (27.95). The door (z 29…30) is enterable, so
nothing solid stands within 0.9 m beside its posts (LAYOUT-038) and the diagonal approaches from
3 m aside stay free (LAYOUT-034) — the Q-135 rows on both sides of the door (27.3 / 28.1 and
30.9 / 31.7) no longer fit. Inside (Q-194 answered 2026-09-29): two more real, labelled food
boxes (interact → label panel → take, same as outside) on the plank platform at the back (west)
wall: *Käfer* / *beetles* (−43.44, 28.2) and *Obst* / *fruit* (−43.44, 29.8) — the night zoo's
active foods, repeated; the model's sacks and crates stay in the corners of the wall band.
Hedgehog and owl both need *Käfer*: the child takes from the same box twice. *Würmer* and
*Nektar* are distractors now and foods of later night levels (GAME-NIGHT table: badger/kiwi,
slow loris).

## Hiding places (candidates)

Data: `[[hiding_place]]` in `night-1.toml` (fields as in level 1, Q-080). Riddle keys
`mission-<animal>-riddle-<id>-<reading_level>` (CONT-MISSIONS "Night level 1",
`assets/i18n/{de,en}/night.ftl`). Wander areas (radius 3 m, on grass, clipped to the rect,
4-connected) computed with a scratch script, 2026-09-27.

| Id | Animal | Area rect (x, z, w, d) | Animal spot | Perch | Wander cells | Features (riddle details) | Scenery | Spot → own board | Fastest walk own board → spot | Nearest wander cell ↔ own board/gate standing point (≥ 22 m, Q-110, FIX-056) |
|---|---|---|---|---|---|---|---|---|---|---|
| `loc_brush_pile` | hedgehog | -70, 8, 6, 5 | (-68, 10) | — | 26 | brush_pile, twigs, rustling, hedge_corner | `brush_pile_n1`, `hedge_n1_west`, `hedge_n1_south` | 38.2 m | 25.3 s | 33.2 m |
| `loc_flowerpots` | hedgehog | -70, 18, 6, 6 | (-68, 20) | — | 25 | flower_pots, potting_bench, night_flowers, sweet_smell | `potting_bench_n1` | 31.6 m | 19.7 s | 26.6 m |
| `loc_mushrooms` | hedgehog | -70, 27, 6, 7 | (-68, 30) | — | 26 | mushrooms, moss, earthy_smell, old_tree | `mushroom_ring_n1`, `tree_old_n1` | 27.2 m | 14.5 s | 22.1 m |
| `loc_windmill` | bat | -59, 8, 6, 5 | (-55, 9) | 2.5 m (under the sails' platform) | 13 | windmill, turning_sails, whirring | `windmill_n1` | 33.1 m | 21.3 s | 28.3 m |
| `loc_fireflies` | bat | -52, 8, 7, 5 | (-51, 10) | 2.0 m (branch of the crooked tree) | 20 | fireflies, little_lights, crooked_tree, low_grass | `firefly_meadow_n1`, `tree_crooked_n1` | 30.5 m | 18.3 s | 25.6 m |
| `loc_hollow_tree` | bat | -31, 12, 5, 4 | (-27, 15) | 2.5 m (at the knothole) | 11 | hollow_tree, knot_hole, thick_old_trunk | `tree_hollow_n1` | 25.9 m | 16.9 s | 22.5 m |
| `loc_moon_pond` | owl | -40, 8, 5, 5 | (-38, 10) | 1.5 m (post at the west shore, (−35.6, 10.5)) | 25 | pond, moon_reflection, reeds, wooden_post | `pond_n1`, `jetty_n1` | 28.4 m | 15.1 s | 24.1 m |
| `loc_hilltop` | owl | -70, 39, 7, 8 | (-66, 43) | 2.4 m (stone on the hilltop) | 21 | hill, big_stone, moonlight, no_trees | `hill_n1` | 33.4 m | 20.1 s | 29.2 m |
| `loc_fir` | owl | -63, 46, 6, 6 | (-60, 48) | 6.0 m (top of the fir) | 22 | fir_tree, pointed_top, cones, needles | `fir_n1` | 28.8 m | 18.5 s | 24.4 m |

**Standing points** (CAMV-008): walkable cell centres ≤ 2.5 m from the own info board plus the
hall cells in front of the own gate. The nearest case is `loc_mushrooms` (22.1 m); the level
minimum is 22.1 m. **FIX-056 (22 m rule):** `loc_hollow_tree` was 17.7 m from the bat board, so
the hollow tree moved 3 m south-east into the corner by the east hedge (tree (−28, 16, 2, 2),
spot (−27, 15) right in front of its knothole, rect (−31, 12, 5, 4) south of it; was tree
(−29, 19), spot (−30, 19), rect (−33, 16, 7, 4); 15 → 11 cells; nearest cell now 22.5 m). The **zoo-view screen test** (portrait
1080×2340, 45° steps, 10/14/20 m, points at 0.5 m, 1.0 m and perch + 1 m) finds no wander cell
on screen from any standing point (scratch check; LAYOUT-N1-006 is the real test).

**Spread (RESC-014).** Straight distances between spots of different animals; **all 27
combinations** keep the chosen spots ≥ 12 m apart (closest: `loc_hollow_tree` – `loc_moon_pond`
12.1 m):

| | `loc_windmill` | `loc_fireflies` | `loc_hollow_tree` | `loc_moon_pond` | `loc_hilltop` | `loc_fir` |
|---|---|---|---|---|---|---|
| `loc_brush_pile` | 13.0 | 17.0 | 41.3 | 30.0 | 33.1 | 38.8 |
| `loc_flowerpots` | 17.0 | 19.7 | 41.3 | 31.6 | 23.1 | 29.1 |
| `loc_mushrooms` | 24.7 | 26.2 | 43.7 | 36.1 | 13.2 | 19.7 |
| `loc_windmill` | — | — | — | 17.0 | 35.7 | 39.3 |
| `loc_fireflies` | — | — | — | 13.0 | 36.2 | 39.1 |
| `loc_hollow_tree` | — | — | — | 12.1 | 48.0 | 46.7 |

**Picking rule:** as level 1 (Q-082): uniform seeded pick per animal, own seeded RNG of the level
(`seed ^ k·φ`, GAME-LAYOUT "Implementation"); the first draw is always valid here.

## Hiding places — riddle details (night clues)

| Place | What the child sees / hears / smells | Riddle guards |
|---|---|---|
| `loc_brush_pile` | A big heap of dry brown twigs in the **corner where the west and south hedges meet**; it **rustles** when the hedgehog moves (audio). | Only brush pile in the zoo; not a leaf pile (no red/yellow leaves, no rake — level-1 `loc_leaves`). |
| `loc_flowerpots` | An old **potting bench** at the west hedge, stacked **clay pots** on and under it, white night-scented flowers (evening primrose) that **smell sweet** at night (a soft scent-swirl effect near them). | No other flower pots; not the level-1 garden (no beds), no blossoms on trees (level-2 `loc_blossom_tree`). |
| `loc_mushrooms` | A **ring of round brown/cream mushrooms** on a green **moss** patch at the foot of a big old mossy tree; smell of damp earth. | Only mushrooms in the zoo; friendly shapes (no red-with-white-dots). |
| `loc_windmill` | A little wooden **windmill** (4 m) whose four **sails turn** slowly and **whir**; the bat hangs under its balcony. | Only windmill; not the level-3 water wheel (turns in the wind, no water). |
| `loc_fireflies` | A low meadow full of **fireflies** (Q-115: the only riddle place with fireflies; no fireflies anywhere else in `night_1`), a small crooked tree whose low branch holds the bat. | Low grass (not the level-1 tall-grass meadow), no butterflies, no flowers. |
| `loc_hollow_tree` | The **thickest, oldest tree** of the garden with a big round **knothole** at 2.5 m; the bat hangs right in front of it. | Only hollow tree (the grove trees and `tree_old_n1` have no holes); not the level-2 tallest tree / tree house. |
| `loc_moon_pond` | A small **still pond** in which the **moon and stars are mirrored** (night water shader), reeds at the rim, a short jetty; the owl sits on a **wooden post** at the west shore. | No lilies, frogs, ducks or bridge (level-1 `loc_pond`, `loc_river`); the only water of `night_1`. |
| `loc_hilltop` | A small round **grassy hill** (2 m) with one **big round stone** on top and **no trees** — the brightest moonlit spot (an extra soft moonlight pool). | Only hill of `night_1`; grassy, not grey rock (level-1 rock hill / level-2 sun rocks). |
| `loc_fir` | The one tall **dark pointed fir** (9 m) with brown **cones**; the owl sits on the very top. | Only fir in the zoo; the grove trees are round deciduous trees. |

Riddles of one animal never share their key detail: hedgehog = sound (rustling twigs) vs. smell
(sweet night flowers) vs. smell/touch (mushrooms, moss); bat = turning sails vs. dancing lights
vs. round hole in a thick trunk; owl = moon **in the water** vs. moonlight **on a bare hill** vs.
the **top of a pointed tree**.

## Barriers

`night_1` has **no barriers of its own**; it is sealed by hedges on all sides. Its only entry is
the level-1 barrier `moon_door` (GAME-LEVEL-1 "Moon door"): open at night after level 1's
nightfall, closed by day. A later night level (`night_2`, Q-023) would join behind a new barrier
of this level (e.g. a closed garden gate in `hedge_n1_north`), not designed yet.

## Walking distances

Fastest walk (8-neighbour, 1.93 m/s on paths, 1.45 m/s on grass; scratch estimates of
2026-09-27 — the LAYOUT-N1-005 test prints the exact values):

| From → to (neighbours) | Fastest time |
|---|---|
| moon door (entry cells) → food boxes | 7.0 s |
| moon door → map board | 2.1 s |
| food boxes → bat board | 3.1 s |
| bat board → hedgehog board / owl board / night-house door | 0.5 s / 2.8 s / 0.5 s |
| night-house door → hedgehog gate / owl gate (hall) | 4.3 s / 4.3 s |
| hedgehog board → telescope | 4.1 s |
| telescope → `loc_fir` | 7.9 s |
| `loc_fir` → `loc_hilltop` | 5.3 s |
| `loc_mushrooms` → `loc_flowerpots` | 6.1 s |
| `loc_flowerpots` → `loc_brush_pile` | 6.1 s |
| `loc_windmill` → `loc_fireflies` | 1.0 s |
| `loc_fireflies` → `loc_moon_pond` | 7.9 s |
| `loc_moon_pond` → `loc_hollow_tree` | 6.9 s |
| `loc_moon_pond` → food boxes | 7.2 s |

Every place has a neighbour ≤ 10 s. Around the loop three legs are close to the limit (for
information; each end has a nearer neighbour): telescope → `loc_hilltop` 9.5 s, `loc_hilltop` →
`loc_mushrooms` 9.7 s, `loc_brush_pile` → `loc_windmill` 9.6 s. Since FIX-056 the south
places join the plaza over `loc_moon_pond` → food boxes (7.2 s); `loc_hollow_tree` → food boxes
is not a neighbour pair. Board → own spot: 14–25 s (the places are far on
purpose, as in level 1).

## Night lights (GAME-NIGHT rule 1, Q-118, Q-114)

`[[light]]` in `night-1.toml`: 12 lantern posts along the entry path, the loop and the trails
(~10 m apart, 0.25 m inside the path edge — never in a hiding-place rect or scenery), two
string lights in an X over the plaza (the night zoo's "entrance plaza"), wall lamps at the
night-house door and the food hut, board lamps on the three info boards and the map board, and
the indoor lights of the night house (`indoor`, `color`). The hiding places have **no lantern
of their own**: the child's hand lantern (radius 2.5 m, NIGHT-006) finds the shining eyes; the
fireflies, the moon reflection and the hilltop moonlight are the places' own soft lights.

## Night riddles and the haze (Q-126, CAMV-008)

The close views keep the day haze at night (Q-126 answered; fog end 20.8 m since the user raised
the view distance by 30 %, 2026-09-27); every wander cell is ≥ 22 m from the own standing points
(table above, FIX-056), so shining eyes are never visible through the haze from the board or
gate. LAYOUT-N1-006 checks this 22 m margin for `night_1` (CAMV-008 covers levels 1–3).

## Behaviour

1. `night_1` contains exactly the elements of the table above; `night-1.toml` mirrors it
   (LAYOUT-005).
2. Its bounds (−72, 6, 48, 48) do not overlap any other level; every border cell is solid except
   the entry cells (−25, 29…30), which are path cells edge-adjacent to `moon_door` (LAYOUT-021).
3. With the moon door open, every enclosure, building, landmark, board, food box and hiding
   place of `night_1` is reachable from the entry; with it closed, no cell of `night_1` is
   reachable from level 1.
4. No two solid elements overlap; no path cell lies under a solid element.
5. The night-house hall and door are walkable (surface `path`); each indoor enclosure's gate
   opens onto a hall cell; the three boards stand outside the house.
6. Every hiding place: wander area ≥ 9 cells with a cell ≥ 2 m from the spot, inside its rect,
   no solid or path cell; places of different animals never overlap; all 27 combinations keep
   the spots ≥ 12 m apart.
7. Every wander cell (0.5 m, animal height, perch + 1 m) is off-screen in the zoo view from its
   own board/gate standing points and ≥ 22 m from them (CAMV-008 with the Q-110 margin; 22 m
   since the fog end grew to 20.8 m, FIX-056).
8. Each place's `features` exist only there within the night levels (Q-136), the fireflies
   exist only at `loc_fireflies` (Q-115), and every scenery id a place names exists.
9. Neighbouring points of interest are ≤ 10 s apart ("Walking distances").
10. The night food storage has one box each of `beetles`, `fruit`, `worms`, `nectar`, with a
    reachable standing cell in front of each.
11. Lantern posts and string-light ends stand on walkable cells outside hiding-place rects and
    scenery; every board has a board lamp.

## Mockups

The approved mood image `env_night_overview/overview.png` is not layout-true (it shows outdoor
enclosures and the moon door at the bottom). Next step (ART-PIPELINE): a greybox of this layout
→ layout-true overview + player views, brief `art/environment/env_night_overview/layout.md`
(camera and areas). The night house interior follows `env_night_house` (cut-away).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| LAYOUT-N1-001 | Given `level-1.toml` + `night-1.toml` joined with the moon door open, then every enclosure, building, landmark, info board, food box and hiding place of `night_1` has a walkable cell reachable from the level-1 spawn next to it or inside it. | unit |
| LAYOUT-N1-002 | Given the joined levels with the moon door closed (day), then no cell of `night_1` is reachable from the level-1 spawn; every border cell of `night_1` except (−25, 29) and (−25, 30) is solid. | unit |
| LAYOUT-N1-003 | Given `night-1.toml`, then no two solid elements share a cell and no path cell lies under a solid element; its bounds are disjoint from levels 1–3. | unit |
| LAYOUT-N1-004 | Given this spec's element table and `night-1.toml`, then both list the same ids, types and rectangles (LAYOUT-005). | unit |
| LAYOUT-N1-005 | Given `night-1.toml` with 1.93 m/s on paths and 1.45 m/s on grass, then each pair of the "Walking distances" table is ≤ 10 s. | unit |
| LAYOUT-N1-006 | Given missions `hedgehog`, `bat`, `owl`, every candidate, the player on every walkable cell ≤ 2.5 m from the own board or in front of the own gate, and the zoo camera at every 45° rotation and 10, 14, 20 m on 1080×2340, then no wander cell centre (0.5 m, 1.0 m, perch + 1 m) is on screen, and every one is ≥ 22 m (planar, cell centres) from the standing points (CAMV-008, Q-110; 17 m before FIX-056). | unit |
| LAYOUT-N1-007 | Given the hiding places, then each has ≥ 9 wander cells (counts of the table: 26, 25, 26, 13, 20, 15, 25, 21, 22), a cell ≥ 2 m from the spot, no solid or path cell, lies inside its rect; places of different animals do not overlap; all 27 combinations keep spots ≥ 12 m apart. | unit |
| LAYOUT-N1-008 | Given the hiding places, then their `features` contain the CONT-MISSIONS details, every `scenery` id exists, each scenery kind and each riddle element kind (`windmill`, `tree_hollow`, `fir_tree`, `hill`, `pond`, `potting_bench`, `tree_crooked`) occurs once in the night levels, and `firefly` props appear only in `firefly_meadow_n1`. | unit |
| LAYOUT-N1-009 | Given `night_house`, then its interior and door are walkable with surface `path`, each indoor enclosure (`indoor = true`) has its gate edge-adjacent to an interior cell, and its `model_rect` contains the hall and all three enclosures. | unit |
| LAYOUT-N1-010 | Given the `[[food_box]]` list, then the foods it has are exactly `beetles`, `fruit`, `worms`, `nectar`; 2–6 more boxes stand inside the hut and may repeat a food; every box has a walkable cell centre within 2 m in front of it reachable from the entry. | unit |
| LAYOUT-N1-011 | Given the `[[light]]` list, then every post / string end is on a walkable cell outside hiding-place rects and scenery, every board and the map board has exactly one `board_lamp`, every `attach` id exists, and the night house has an `indoor` light per enclosure with the Q-116 colours. | unit |
| LAYOUT-N1-012 | Given the player walks through the open moon door from level 1, then she stands on `path_n1_entry` in `night_1` and the night missions are in scope; walking back east returns her to `path_moon` (NIGHT-004). | e2e |
| LAYOUT-N1-013 | Given the layout-true night mockups (after approval), then a reviewer can name each place's night clue without text and confirms nothing is scary (NIGHT-009). | manual |

## Open questions

- Q-145 answered 2026-09-27: the layout changes of FIX-056 are accepted (22 m haze rule: moved/clipped hiding places, moved board, bench and trail, new walking neighbours).
- Q-133 answered 2026-09-27: night level data (`time = "night"`, moon door `opens_at` / `unlock_after`), open every night — also after the night zoo is done.
- Q-134 answered 2026-09-27: night house structure (hall + indoor enclosures + `model_rect`, boards outside).
- Q-135 answered 2026-09-27: night food storage and food ids (`beetles`, `fruit`, `worms`, `nectar`; hedgehog and owl share beetles).
- Q-136 answered 2026-09-27: night levels are their own riddle scope.
- Q-137 answered 2026-09-27: `[[light]]`, `[[item]]`, `[[prop]]` data shape.
- Q-138 answered 2026-09-27: telescope is decoration now, interactive later as a small bonus.
- Q-142 answered 2026-09-27: eyeshine 2.5 m, visible lantern ground pool 2.5 m (GAME-NIGHT, NIGHT-018).
- Q-094 perch heights, Q-043 poses (`hang`), Q-126 (answered) night haze.
- Q-147 string-light spans on the plaza; Q-149 `loc_hilltop` perch height 2.4 m vs. the `rock_hill` model (1.95 m); Q-151 "Futter" board of the `food_hut` above its eaves.
- Q-154 `kit_landmarks` (windmill, trees, `rock_hill`, …) has no manifest entry / ART listing yet.
- Q-173 scope of the Q-157 wall-gap rule (nothing solid 0.1–0.6 m in front of a wall/fence near an opening).
- Q-181 answered 2026-09-28: the night food boxes stay outside (one row south of the door), two more real, labelled food boxes inside (Q-194 answered 2026-09-29).
