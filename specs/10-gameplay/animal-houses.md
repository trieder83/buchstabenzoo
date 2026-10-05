---
id: GAME-HOUSE
title: Animal houses (door-only entry)
aspect: gameplay
module: houses
status: draft
depends_on: [GAME-LAYOUT, GAME-ANIMALS, GAME-FAMILY, GAME-FEED, ART-ENVIRONMENT]
test_prefix: HOUSE
updated: 2026-10-04
---

# Animal houses (door-only entry)

**Contents:** Goal · Terms and rules · Data model · Which species get a house · Houses per level · Behaviour · Geometry and models · Never stuck · Test cases · Open questions

## Goal

User request 2026-10-04: "make sure the animals only enter their houses by the door; the
giraffe has a home high enough". Today the houses and shelters exist only as model names in
ART-ENVIRONMENT (`hut_wood`, `elephant_house`, `giraffe_house`, `monkey_house`,
`koala_shelter`, `panda_shelter`, `stone_arch_shelter`); the level data has only a reserved
placeholder (`hippo_hut`, `kind = "hut"`). This spec turns them into level data and wander
rules: an **animal house** (*Tierhaus*) is a small building inside an **enclosure** with
solid walls and **one doorway**. The animal walks in and out only over the door cells and
never through a wall. The child player never enters (she cannot enter an enclosure at all).

Glossary: `animal_house` (Tierhaus / animal house) — a building inside an enclosure that its
animals use as shelter; never "cage" (proposal for the glossary, Q-359). Not to be confused
with `building` elements (zookeeper house, storages: enterable by the child, GAME-LAYOUT
"Enterable buildings") nor with the `night_house` / `terrarium_house` (indoor enclosures).

## Terms and rules

1. **Footprint** `rect`: the whole house in grid cells, completely inside its enclosure's
   `rect`. It consists of three disjoint cell sets:
   - **interior** (`interior`, a rect inside the footprint): the room; animal-walkable.
   - **door cells** (`door`, a rect 1 cell deep and 2 or 3 cells wide on the footprint edge
     that faces the enclosure, `door_side`): the doorway; animal-walkable. It touches
     interior cells on its inner side and a free enclosure cell on its outer side (the
     **door front**).
   - **walls**: every other footprint cell (footprint − interior − door). Solid.
2. **Walls are solid for everyone.** For the player: the whole enclosure is solid already
   (GAME-LAYOUT "Collision footprints"), the footprint changes nothing there. For the animals:
   wall cells are **not part of the home wander area** (they are the same kind of cell as the
   old reserved `hut`), so no wander route, no wander target and no feed spot can lie on a
   wall; interior + door cells **are** part of the home wander area (class `Land`).
   Consequence: the interior is reachable only through the door cells — by construction, not
   by a special-case check. A wall is a full 1 m thick cell; the model's wall is thinner and
   sits on the footprint outline, never inside the interior or the door cells.
3. **Door-only entry/exit** also holds for the route code: wander routes are 4-neighbour
   routes over cell centres (`WanderArea::route`), so a route never cuts a wall corner; a
   route into the interior always contains at least one door cell.
4. **Door width and height:** width 2 cells (a 1 × 2 or 2 × 1 rect; 3 is allowed), so that two animals of a pair can pass and
   the animal model does not touch the posts; **door height** `door_height_m` ≥ the
   game size of the tallest animal of the species + 0.5 m (ANIM sizes, ART-ANIMALS "Game
   sizes"): zebra 2.23 → ≥ 2.8, hippo 1.7 → ≥ 2.2, panda 1.1 → ≥ 1.6, koala 0.9 → ≥ 1.4,
   elephant 3.0 → ≥ 3.5, **giraffe 4.5 → ≥ 5.0**, monkey 1.1 → ≥ 1.6. The roof eave
   `roof_height_m` ≥ `door_height_m` + 0.4; the model must show the opening at least that
   high so the animal model never pokes through the lintel.
5. **No door leaf.** The doorway of an animal house is an open archway (nothing to open or
   shut), so animals never wait at a door and LAYOUT-031 (gate/door models) does not apply.
6. Houses never block the **enclosure gate** (≥ 1 free row/column between the gate cells and
   any wall: Manhattan distance gate cell to wall cell ≥ 2), the **free area** (the home
   wander area without the house stays 4-connected and keeps ≥ 55 % of its former cells) or
   the **pool** (no overlap with a pool, its ramp or its rim cells; hippo/elephant water
   cell counts are unchanged).

## Data model

An animal house is an `[[enclosure_feature]]` with `kind = "animal_house"` (not an `element`:
like the pool its cells are inside the enclosure rect and stay solid for the player, so the
LAYOUT-003 overlap rule is unaffected; it **replaces** the placeholder `kind = "hut"`, which
becomes a deprecated alias that the loader still reads until the data is migrated — Q-359).
Fields (extending LAYOUT "Enclosure features"):

| field | meaning |
|---|---|
| `id` | `<species>_house` / `_shelter` / `_hut`, unique |
| `enclosure` | id of the enclosing `enclosure` element |
| `kind` | `"animal_house"` |
| `rect` | footprint `[x, z, w, d]` |
| `interior` | `[x, z, w, d]` inside the footprint, ≥ 2 × 2 cells and ≥ 2 cells per animal of the pair + 2 (see Behaviour: capacity) |
| `door` | `[x, z, w, d]`, `w`/`d` = 2 × 1 or 1 × 2, on the footprint edge named by `door_side` |
| `door_side` | `"-x"`, `"+x"`, `"-z"`, `"+z"`: the direction in which the door faces (towards the enclosure interior; `+z` = north) |
| `door_height_m` | clear height of the doorway (rule 4) |
| `roof_height_m` | eave height; the ridge may be higher (model) |
| `model` | `stone_arch_shelter`, `hut_wood`, `panda_shelter`, `koala_shelter`, `elephant_house`, `giraffe_house`, `monkey_house` |
| `rest` | optional, default `true`: animals may rest inside (Behaviour) |
| `notes` | free text |

Example (giraffe, level 2):

```toml
[[enclosure_feature]]
id = "giraffe_house"
enclosure = "enc_giraffe"
kind = "animal_house"
model = "giraffe_house"
rect = [41, 50, 6, 6]          # x 41-46, z 50-55: footprint, back wall on the north fence
interior = [42, 51, 4, 4]      # x 42-45, z 51-54
door = [43, 50, 2, 1]          # x 43-44 on the south edge (z 50); door front z 49
door_side = "-z"
door_height_m = 5.2            # giraffe 4.5 m + 0.5 m margin (rule 4), user request 2026-10-04
roof_height_m = 5.6
```

`home_area` change (for the implementer): for a cell in the footprint: interior or door →
`Land` (if `rest` is true, else skipped like a wall — then the house is decoration only);
any other footprint cell → skipped (wall). `LevelData` loader checks rule 1 and 6 at load.

## Which species get a house

Decided (minimal, for a calm zoo; the six enclosures with the largest free area plus the
panda): every enclosure of levels 1–3 whose art spec already names a shelter gets one.

| species | enclosure | model | door height | reason |
|---|---|---|---|---|
| zebra | `enc_zebra` (L1) | `stone_arch_shelter` | ≥ 2.8 | art spec `env_zebra` |
| hippo | `enc_hippo` (L1) | `hut_wood` | ≥ 2.2 | existing reserved hut, `env_hippo` |
| panda | `enc_panda` (L1) | `panda_shelter` | ≥ 1.6 | art spec `env_panda` (wooden shelter) |
| koala | `enc_koala` (L2) | `koala_shelter` | ≥ 1.4 | art spec `env_koala` |
| elephant | `enc_elephant` (L2) | `elephant_house` | ≥ 3.5 | art spec model list |
| giraffe | `enc_giraffe` (L2) | `giraffe_house` | **≥ 5.0** | art spec `env_giraffe` "giraffe house with a tall door" |
| monkey | `enc_monkey` (L3) | `monkey_house` | ≥ 1.6 | art spec `env_monkey` |

No house (decided): `enc_lion` (sun deck instead), `enc_snow_fox` (open), `enc_goldfish`
(pond only), the indoor enclosures of `night_1`/`night_2` (they are inside a house already).
Lion den and fox den are open: Q-360.

## Houses per level

Coordinates are grid cells (`x, z`; `+z` = north; a rect covers `x…x+w-1`, `z…z+d-1`; the
fence stands on the enclosure rect's edge cells). All positions were checked against every
`element`, `enclosure_feature`, `[[prop]]`, `[[scenery]]`, `[[hiding_place]]`, `[[light]]`
and `[[food_box]]` of the level files on 2026-10-04: inside the enclosure rects there is
nothing but the pools (hippo, elephant) and their stones; all lamps, boards, signs and
hiding places lie outside. Checked by script (flood fill): every house interior is reachable
from the cell inside the gate and the remaining free area stays connected.

| id | enclosure (rect) | footprint `rect` | `interior` | `door`, `door_side` | door front | door/roof h (m) | notes |
|---|---|---|---|---|---|---|---|
| `zebra_shelter` | `enc_zebra` (−20,7,11,12), gate (−10,12,1,2) east | −20,15,5,4 (x −20…−16, z 15…18, NW corner) | −19,16,3,2 | −19,15,2,1 `-z` | (−19…−18, 14) | 2.8 / 3.2 | stone arch, open archway; 6 interior cells; gate 8 cells away |
| `panda_shelter` | `enc_panda` (−6,32,12,10), gate (−1,32,2,1) south | −6,37,5,5 (x −6…−2, z 37…41, NW corner) | −5,38,3,3 | −2,38,1,2 `+x` | (−1, 38…39) | 1.8 / 2.2 | wooden shelter; the climbing platform and the bamboo rack (decoration) stay east of x −1 |
| `hippo_hut` | `enc_hippo` (9,11,11,12), gate (9,15,1,2) west | **14,11,5,4** (x 14…18, z 11…14, south fence; was reserved 15,11,4,4) | 15,12,3,2 | 14,12,1,2 `-x` | (13, 12…13) | 2.3 / 2.7 | not facing the pool (the rim is 0.4 m high and water and grass connect only over the ramp); east strip x 19 stays free and connected; edge stone (13.6, 14.3) moves to (12.6, 14.4) |
| `koala_shelter` | `enc_koala` (27,33,8,10), gate (34,36,1,2) east | 27,39,4,4 (x 27…30, z 39…42, NW corner) | 28,40,2,2 | 30,40,1,2 `+x` | (31, 40…41) | 1.5 / 1.9 | small wooden shelter; eucalyptus trunks (decoration) east/south of it |
| `elephant_house` | `enc_elephant` (55,28,12,13), gate (55,33,1,2) west | 55,28,4,4 (x 55…58, z 28…31, SW corner) | 56,29,2,2 | 58,29,1,2 `+x` | (59, 29…30) | 3.6 / 4.0 | the pool takes the middle (x 59…65, z 31…38), so the house stands in the free south-west corner; the door front is the south strip (z 28…30); one free row (z 32) between the house and the gate cells |
| `giraffe_house` | `enc_giraffe` (40,44,12,12), gate (45,44,2,1) south | 41,50,6,6 (x 41…46, z 50…55, back wall on the north fence) | 42,51,4,4 | 43,50,2,1 `-z` | (43…44, 49) | **5.2 / 5.6** (ridge ≈ 7) | tall door for the 4.5 m giraffe; the tall feeding rack (decoration) stands east of x 47; 16 interior cells |
| `monkey_house` | `enc_monkey` (−4,82,12,10), gate (1,82,2,1) south | −4,87,5,5 (x −4…0, z 87…91, NW corner) | −3,88,3,3 | 0,88,1,2 `+x` | (1, 88…89) | 1.8 / 2.2 | the climbing frame and banana basket (decoration) stay east of x 1; the burglar hideout (8…10, 87…91) is outside |

Free home-area cells afterwards (grass + ramp + interior/door): zebra 113, panda 102, hippo 59
(grass incl. interior/door; plus 56 pool cells incl. the 3 ramp cells), koala 68, elephant 86 (plus 56 water), giraffe 124, monkey 104 (measured by HOUSE-006; prop-blocked cells like bushes and stones are already excluded).

ASCII of the giraffe enclosure (1 char = 1 m; `#` fence, `W` house wall, `d` door cell, `.`
interior, ` ` grass, `G` gate; north up):

```
z55 ############
z54 #W....W    #     x40 fence, x41..46 house (walls W, interior x42..45)
z53 #W....W    #
z52 #W....W    #
z51 #W....W    #
z50 #WWddWW    #     door cells d = x43..44
z49 #          #     door front
z48 #          #
z47 #          #
z46 #          #
z45 #          #
z44 #####GG#####     gate x45..46
```

(Schematic; the exact cell data are the table above.)

## Behaviour

An animal `in_enclosure` (GAME-ANIMALS) picks wander targets in its home area as before
(ANIM-010/011); the house adds two behaviours, deterministic and seeded (ANIM-011):

1. **Resting in the house (proposal Q-361).** Of 10 new wander targets, `HOUSE_TARGETS_OF_10 = 2`
   are an interior cell of the house (species without a house: none; for species with a pool,
   hippo and elephant, the 7 of 10 water targets are kept and the 2 house targets replace land
   targets, ANIM-012 "most time in the pool" stays valid). It walks over the door
   cells to the target, stays a **rest** of 8–20 s (`draw_pause`-like, saved with the animal,
   `rest_s`), then picks the next target outside or inside as usual. It never stands on a
   door cell for longer than the normal pause.
2. **Night/dusk (proposal Q-361).** When the level's nightfall begins (GAME-NIGHT), every
   home animal with a house walks into it and rests there until morning; a pair rests in
   different interior cells They hurry in at `GIFT_SPEED` (1 m/s) so that every animal is
   inside within 60 s (HOUSE-010); an animal outside while the child stands within
   `NOTICE_PLAYER_M` of the door front or the gate stays out until she leaves (no in-and-out
   flapping). No new rule for the day zoo otherwise.
3. **The child comes (never hide an animal she needs).** A resting animal ends its rest and
   walks out when the player is within `NOTICE_PLAYER_M` (5 m) of the door front or of the gate,
   or when care feeding / the baby challenge (GAME-FEED, GAME-FAMILY) needs it at the feed
   spot. The feed spot (`wander::feed_spot`) is always a cell outside the house.
4. **Capacity.** The interior has ≥ 2 cells per animal of the pair (4 for a pair: male +
   female (+ baby: the baby uses the door front or an interior cell; rest is skipped when no
   free interior cell is left, the animal just wanders). Two animals never share a cell. The
   pair gap (GAME-FAMILY, FAM-025) is waived on the footprint and the ring of cells around it
   (`House::near`: walls, interior, door, door front): a pair of elephants cannot keep 2 m apart
   in a 2 × 2 room and a pair must get through the 2-wide door. FAM-025 exempts the same cells.
5. **Escaped animals never enter a house.** Hiding places and their wander areas lie outside
   enclosures; a led (`following`) animal reaches the home area only over the gate cells and
   `home_entry`, then behaves as in 1. No animal is led into the house by the player.
6. **Visibility.** The roof stays drawn (it is the model's silhouette); while any animal rests
   inside and the player is within 8 m of the door, the roof/front wall is cut away like the
   enterable buildings (`PLAY-028` style, proposal Q-361) so the child sees the animal.
   Animals inside never stand in the door cells for the roof logic.

## Geometry and models

- Footprint sizes are for the logical cells; the model is built to the footprint (walls on
  the outline) with the doorway exactly `door` wide and `door_height_m` high. The giraffe
  house is the tallest building of the zoo (ridge ≈ 7 m); the 55° camera shadow of a 7 m roof
  covers ≈ 5 m north of it (outside the north fence: the hiding place `loc_big_ball` x 51…56 is
  east of the footprint and stays visible).
- Until the models exist the renderer draws a **placeholder box with a door cut-out**
  (walls from the footprint, the door cells open, height `roof_height_m`) — never a full box,
  so the logic and the visuals agree from the first build.
- Art: the models are already listed (`art/props/kit_enclosure_buildings` for the zebra,
  hippo, panda; `elephant_house`, `giraffe_house`, `koala_shelter`, `monkey_house` unique
  models of levels 2/3). Their `concept_approved` gate (ART-PIPELINE) applies; ART-ENVIRONMENT
  lists door width/height as brief requirements.

## Never stuck

- An animal resting in a house is **home** (counted by enclosure rect, not by cell): missions,
  the 🌙 compass strip, hints and the safety net (GAME-HINT) are unaffected. No hint targets
  an animal inside a house.
- Care feeding and the baby challenge: the animal leaves the house when the child arrives
  (Behaviour 3), so it can always be fed at the gate; a child can never be blocked by a
  resting animal.
- Walls never block the gate, the free area or a pool (rule 6, HOUSE-001/006).
- Save/restore with an animal inside a house restores position, route and `rest_s` (HOUSE-015).
- **Fuzz test:** the seeded random-action fuzz of the never-stuck rules (GAME-HINT) is extended
  in the same change with houses: animals rest inside, treats/baby/wrong food/save-restore
  while an animal is inside; it must still reach "all animals home" and dusk → bed (HOUSE-016).
  Result of this design review: **the child cannot get stuck** by the houses — nothing a
  mission needs is inside a house and every inside animal comes out on demand.

## Test cases

Unit tests live in `crates/zoo-core/tests/houses.rs` (new, with the data of levels 1–3).

| ID | Test case | Type |
|---|---|---|
| HOUSE-001 | Given every `[[enclosure_feature]]` with `kind = "animal_house"` of levels 1–3, then its footprint lies completely inside its enclosure rect; `interior` and `door` lie inside the footprint and are disjoint; the door is 2 or 3 cells wide, 1 deep, on the footprint edge named by `door_side`, touches an interior cell on the inner side and a free enclosure cell (the door front) on the outer side; the footprint overlaps no gate cell, pool, ramp, edge stone, other feature, prop or hiding place; the Manhattan distance from every gate cell to every wall cell is ≥ 2. | unit |
| HOUSE-002 | Given the species table of this spec, then exactly the seven listed enclosures have one animal house each (`zebra_shelter`, `panda_shelter`, `hippo_hut`, `koala_shelter`, `elephant_house`, `giraffe_house`, `monkey_house`) with the listed footprint, interior, door, side and model; `enc_lion`, `enc_snow_fox`, `enc_goldfish` have none (spec and data agree, like LAYOUT-005). | unit |
| HOUSE-003 | Given every animal house, then `door_height_m` ≥ the game size of its species + 0.5 m and `roof_height_m` ≥ `door_height_m` + 0.4; for the giraffe `door_height_m` ≥ 5.0 m and the doorway of the `giraffe_house` model (once built) is ≥ 5.0 m high and ≥ 2 m wide. | unit |
| HOUSE-004 | Given `wander::home_area` of every enclosure with a house, then no wall cell is in the area, every interior and door cell is in the area with class `Land`, no cell of the area lies outside the enclosure rect, and the area contains the cell inside the gate. | unit |
| HOUSE-005 | Given the home area of every enclosure with a house, when the BFS starts at `home_entry`, then every interior cell is reached **and** the BFS over the same area with the door cells removed does not reach any interior cell (door-only entry); every area cell is reached (no pockets); the number of walls + interior + door cells equals the footprint area. | unit |
| HOUSE-006 | Given the home area with and without the house, then the area outside the house stays 4-connected, keeps ≥ 55 % of its cells, and the water and ramp cell counts of the hippo (56 water) and elephant (56 water) pools are unchanged; the hippo's grass and water still connect only over the ramp cells. | unit |
| HOUSE-007 | Given every ordered pair of cells of each such area, then `WanderArea::route` steps over 4-neighbour cells only, no segment between consecutive cell centres intersects a wall cell, and every route from an outside cell to an interior cell contains at least one door cell. | unit |
| HOUSE-008 | Given each housed species at home over 600 s with fixed seeds (as ANIM-012), then its position is always on a cell of its home area (never on a wall cell, never off the area), it rested inside the house at least once, it left again, and every transition between outside and inside crossed a door cell. | unit |
| HOUSE-009 | Given an animal resting in its house and the player at 4 m from the gate or the door front, then within 2 s of simulated time it starts a route out of the house; given care feeding / the baby challenge (FEED, FAM) with the animal inside, then the feed spot is a cell outside the house and the animal reaches it. | unit |
| HOUSE-010 | Given the nightfall of a level, then every home animal with a house is inside it after ≤ 60 s of simulated time a pair on two different interior cells, and no animal without a house is affected (proposal Q-361). | unit |
| HOUSE-011 | Given every hiding place and event spot of levels 1–3, then none of its wander cells lies in an animal-house footprint, and no escaped or following animal ever stands on a house cell in the seeded mission playthroughs. | unit |
| HOUSE-012 | Given the player walking at every wall and at the door of each house from all four sides over 20 s, then she never leaves the walkable cells (the enclosure stays solid, the house adds no gap) and LAYOUT-034 (gate approach) still passes for all enclosure gates. | unit |
| HOUSE-013 | Given the animal houses, then their doorway has no door leaf and the gate/door model test LAYOUT-031 lists no animal-house door; the doorway front cells carry no prop or scenery (LAYOUT-032 style, ≥ 1 m free). | unit |
| HOUSE-014 | Given an animal resting in a house and the player within 8 m of the door, then the roof/front wall is cut away (`House::cutaway`), otherwise the roof is drawn (proposal Q-361). | unit |
| HOUSE-015 | Given a save made while an animal rests inside a house (position, route, `rest_s`) and restored, then the animal continues identically for 60 s (extends ANIM-011) and old saves without `rest_s` load (`#[serde(default)]`). | unit |
| HOUSE-016 | Given the never-stuck fuzz test (many seeded random legal action sequences incl. pairs, split pairs, treats, babies, wrong food, wrong gates, save/restore) on levels 1–3 with houses, then it always reaches "all animals home" and dusk → bed, and from every reached state the 🧭 hint has a target of priority ≤ 3 while a mission is incomplete. | unit |
| HOUSE-017 | Given the safety net (GAME-HINT, stall ≥ 120 s), then an animal inside a house is not counted as missing and no hint or call-in targets a house cell. | unit |
| HOUSE-018 | Given the real game in the browser (e2e), then over 90 s of fast-forwarded home time each housed animal of the level is sampled via `window.__zoo` : never on a wall cell of the footprint, at least once inside, and the giraffe's head (4.5 m) never above `door_height_m` while in a door cell. | e2e |
| HOUSE-019 | Given the review shots of every housed enclosure (`UPDATE_SHOTS=1`, camera views), then the house with its doorway, the door front and the animal inside/at the door are visible; the giraffe house shows a doorway visibly taller than the giraffe. | e2e |
| HOUSE-020 | Given the placeholder boxes (until the models exist), then each has the door cut-out of its `door` cells and the height `roof_height_m` (never a closed box) and the placeholder is replaced when the model is loaded. | unit |

## Open questions

- **Q-359** data shape and name (`[[enclosure_feature]] kind = "animal_house"` instead of an `element`; replaces `hut`; glossary term `animal_house`) — recommendation: as specified.
- **Q-360** which species get a house (decided: seven; lion den / fox den open) — recommendation: none for lion, fox and goldfish for now.
- **Q-361** behaviour defaults (2 of 10 targets, 8–20 s rest, dusk: all go in, roof cut-away within 8 m) — recommendation: as specified, roof cut-away is the only visual cost.
- **Q-362** the per-level positions and footprint changes (hippo hut grows to 5 × 4 and one edge stone moves; elephant house is small, 2 × 2 interior, because the pool fills the middle) — recommendation: accept; if the elephant house should be bigger the pool is trimmed on the west side.
