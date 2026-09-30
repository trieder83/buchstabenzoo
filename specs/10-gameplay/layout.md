---
id: GAME-LAYOUT
title: Zoo layout and level boundaries
aspect: gameplay
module: layout
status: draft
depends_on: [GAME-WORLD, ART-ENVIRONMENT, GAME-PLAYER]
test_prefix: LAYOUT
updated: 2026-09-30
---

# Zoo layout and level boundaries

Owned by the `zoo-level-designer` agent. Describes **where** everything is; the look of each
area is in ART-ENVIRONMENT, the rules in GAME-WORLD.

## Goal

A zoo map small enough to build and test, split into levels. Each level opens a bounded
part of the zoo; **barriers** (road blocks, stones, fallen trees, construction fences,
closed gates) keep the player inside the current scope in a way that makes sense in the
world.

## Coordinate system

- Grid of 1 m cells, origin at the zoo entrance gate, +X east, +Z north (**level
  coordinates**, see "Coordinate spaces").
- Every element has an id, a grid rectangle (`x, z, width, depth`) and a level.

## Coordinate spaces (decided, Q-056, user 2026-09-26)

- **Level coordinates** `(x, z)`: the layout data, the ASCII maps (north up) and all game
  logic in `zoo-core` (grid, navigation, player, animals) use x = east, z = north, in metres.
- **World space** `(X, Y, Z)`: the render/glTF space — right-handed, Y-up, as glTF defines it.
  Level north is world **−Z**, level east is world **+X**.
- The **single** conversion is `world = (x, 0, −z)` (and back `level = (X, −Z)`), implemented
  once in `zoo-core` (`zoo_core::coords::level_to_world` / `world_to_level`). The renderer and
  the placement of models in a level use only this function; no other code flips an axis.
- **Models are never mirrored** (no negative scale). A model's local axes are world axes:
  its north is local −Z (in the Blender scripts: Blender +Y), its east is local +X. A model
  is oriented only by a rotation about +Y. A positive angle turns counter-clockwise seen from
  above (north → west); a clockwise quarter turn (north → east, as in tile rotations) is −90°.
- **Front of a model:** characters and animals keep the glTF front = local **+Z** (ART-RIG
  §1), i.e. at yaw 0 they face level **south** (towards the default camera); facing level
  north (as at the spawn) is yaw 180°. The front direction of props with a readable or
  visible side (signs, boards, food box labels, gate details) is not yet specified (Q-061).
- With the default camera looking north (forward = world −Z, up = +Y), level east is on the
  right of the screen and level north is up the screen — the game view matches the ASCII
  map orientation.

## Modular edges: fences, hedges, walls (decided, Q-057, user 2026-09-26)

Straight edges are built from the modular pieces of ART-ENVIRONMENT (`fence_wood`, `hedge`,
`zoo_wall` in **2 m** and **1 m** variants, plus corner pieces and `gate_wood`). A **run** is
one straight line of pieces, measured in whole metres along x or z.

- **Fill rule:** a run of length L m is filled from its start (west end for runs along x,
  south end for runs along z) with ⌊L / 2⌋ **2 m segments**; if L is odd, **one 1 m segment
  goes at the end** (the east/north end). Chosen because it is the simplest rule — every
  integer length is filled exactly, with at most one 1 m piece per run
  (`zoo_core::level::segment_run`).
- **Corners** use the corner pieces (L pieces with 1 m arms): where a run turns by 90°, the
  corner piece sits on the intersection point and each arm takes the first/last metre of the
  two runs, so a straight run between two corners is L − 2 m long.
- **Enclosure fences** *(proposal, level design, Q-060)*: the fence runs along the outline of the
  enclosure rectangle (cell edges), with a `fence_wood_corner` at each of the four corners.
  The gate cells lie on one side; the gate opening is exactly **2 m** (one `gate_wood` leaf,
  rotated open in-game) and splits that side into two runs. The gate may not overlap a
  corner arm.
- **Hedge and wall bands** *(proposal, level design, Q-060)*: `hedge` and `zoo_wall` elements are
  1–2 cells deep while the models are ~1 m (hedge) and 0.6–0.8 m (wall) thick. They are
  placed as **one row on the centre line of the band** (e.g. `hedge_east_b` = x 22–24 → row
  on x = 23), over the full band length. The run direction is parallel to the level border
  the element touches; an element touching no border or two borders (level corner) runs
  along its longer side. Cells stay solid over the full band; only the look is one row.
  Joins where two bands (or a band and a barrier) meet end to end or at a level corner are
  open (Q-059).

## Element types

| Type | Examples | Walkable |
|---|---|---|
| `path` | main path, side path, bridge, jetty, cave floor | yes, `surface = path` (full speed) |
| `enclosure` | one per animal (see GAME-ANIMALS) | only via its gate when leading an animal |
| `building` | entrance, food storage, kiosk, toilet, zookeeper house | entrance only if interactive; *proposal Q-092:* a building with an `interior` rect and a `door` cell is **enterable** — interior and door cells are walkable (surface `path`, floor), the rest stays solid (e.g. `zookeeper_house_3`, GAME-LEVEL-3). **Every building with a `door` is enterable** (user request 2026-09-28, LAYOUT-041) — see "Enterable buildings" |
| `landmark` | pirate ship, fountain, map board, stream, waterfall, stage, zoo train | around it / on it if specified; water landmarks (`pond`, `river`, `stream`, `fountain`) are never walkable |
| `barrier` | road block, stones, fallen tree, construction fence, closed gate, **moon door** (kind `moon_door`, see "Moon door and night levels") | no (while closed) |
| `boundary` | outer zoo wall, hedge, water | no, never removed |
| `decoration` | trees, benches, bushes, tall hedges, info boards | no (collision) — except `sparse` tree areas: walkable between the trunks (see "Forests") |
| `hiding_place` *(data decided, Q-080 answered: a `[[hiding_place]]` list entry, no longer an `[[element]]`)* | `loc_*` of CONT-MISSIONS: an overlay rectangle with `animal`, `animal_spot` (cell where the animal starts and is found), `wander_radius_m` (≤ 3 m), `wander_on` (`grass` / `water` / `cave`; for `water` optionally `water_kinds`, e.g. `["stream"]`), optional `perch_height_m` (animal sits up in a tree / on a ship, proposal Q-094), `features` (riddle details it must show), `scenery` (ids providing the features) and `pose`; listed in a level's `[[hiding_place]]` list (≥ 3 candidates per animal, see "Hiding places" below); may overlap a landmark, path or building | not solid itself — the cells keep the walkability of what lies underneath |

**Flowing water** *(decided, Q-066 answered 2026-09-26)*: every `landmark` of kind `river` or
`stream` has `flow = "N" | "E" | "S" | "W"` (level coordinates, direction the water flows);
other elements have no `flow`. The pieces of one river — its river / stream elements and the
`bridge` path elements aligned with them (a bridge inherits the flow of the river piece it
continues) — must chain along the flow: the cells just beyond a piece's downstream edge are
covered by exactly one next piece of the same width, either straight on (same direction,
same centre line) or a 90° bend whose square is the first `width` cells of the next piece;
flow never reverses, two rivers never merge, and every river has exactly one source
(upstream end). The water field of TECH-WATER is baked from this chain (`zoo_core::water::
river_paths`); broken data is rejected (LAYOUT-026).

**Scenery** *(decided, Q-080 answered)*: non-solid ground dressing that a riddle relies on (tall
grass, sand, mud, tree shade, leaf pile) is listed in a level's `[[scenery]]` list with `id`,
`kind`, `rect`, `hiding_place`, `props`. Like food boxes it is not an element: its cells stay
walkable with surface `grass`, it never overlaps a solid element or a path, and each kind a
riddle relies on exists only once per level. Solid dressing (e.g. a bamboo thicket) is a
normal `decoration` element. A **bamboo forest** is such a `decoration` of kind `bamboo`; with
`harvestable = true` bamboo is cut at its `[[cut_spot]]` entries (`id`, `forest`, `pos` = foot
of the stalk at the forest edge, `stand` = walkable point ≤ 1.5 m in front; GAME-FEED §14).

**Gardens** *(proposal, level design — Q-102; first used by `garden_veg`, GAME-LEVEL-1,
GAME-GARDEN)*: a fenced vegetable garden is listed in `[[garden]]` (`id`, `rect`, `gate`,
`gate_side`, `fence_runs` on cell-edge lines with `fence_inset_m`, `edges_closed_by`,
`animals_enter`, `props`) with its beds in `[[garden_bed]]` (`rect`, `plant`, sign position,
facing and Fluent key) and its harvestable plants in `[[plant_spot]]` (`id`, `bed`, `kind`,
`pos`, `start_stage`, `stand`). Like scenery it is **not an element**: its cells stay walkable
(its inner path is a normal `path` element, kind `garden`); the fence, beds, signs and tools
are prop colliders that lie inside the garden `rect`, and each fence run also blocks the
cell-edge crossings on its line for grid navigation and flood fills. The gate opens by itself
when the player comes near; animals never path into a garden.

**Hiding places** *(user decision 2026-09-26, GAME-RESCUE §1; data decided, Q-080 answered)*: every
animal of a level has ≥ 3 candidate hiding places spread over the level; one per animal is
picked per playthrough with the seeded RNG. Level-design rules for every candidate: its
**wander area** (cells within `wander_radius_m` of the spot on its `wander_on` surface,
4-connected to the spot; never a path the player needs, never a solid cell) has ≥ 9 cells;
the wander area is off-screen from the animal's own info board and gate (screen test);
places and wander areas of different animals never overlap; for every candidate there is a
combination (one place per animal) with all chosen spots ≥ 12 m apart; each riddle detail
exists only at its own place.

**Walkable ground (decided, Q-046, user 2026-09-26):** every cell of a level that is not
covered by a solid element is walkable. Cells have a `surface`: `path` (cells of `path`
elements, incl. bridges, jetties, cave floors) or `grass` (all other walkable cells). The
player walks on grass **slower** than on paths (GAME-PLAYER §6: 1.93 m/s on paths,
1.45 m/s on grass since 2026-09-29, Q-197). Because grass is walkable, all border cells of a level must be solid.

**Missions in scope** (Q-069 answered 2026-09-26): `[level] missions = ["zebra", …]` lists
the missions of a level that are playable; only their animals, info boards and gates are
interactable. Without the field every enclosure's animal is in scope.

**Proposals used by the level files (not yet decided):**
- *Sight blocking data (Q-044):* solid elements may set `blocks_view = true` (and `height_m`,
  used for mockups/greybox). Since the high-angle camera is decided (Q-049, GAME-PLAYER §2),
  "an animal's hiding place cannot be seen from its own enclosure" is checked as a **screen
  test** per level (hiding place off-screen while the player stands at its info board or
  enclosure gate, e.g. LAYOUT-L1-006), not as an eye-level line of sight. In the close
  camera views (look-around, first person) the distance haze does this job: every wander
  cell ≥ **22 m** from the same standing points (fog end 20.8 m + 1.2 m margin — raised from 17 m when the close-view fog grew by 30 %, user decision 2026-09-27; GAME-CAMERA-VIEWS 5,
  CAMV-008; Q-110 answered 2026-09-27). Distances are planar, between cell centres; the
  standing points are the walkable cells ≤ 2.5 m from the own board and the walkable cells
  around the own gate (incl. its diagonal corners). All four level files keep it since
  FIX-056 and again since Q-171 (`board_zebra` south of its gate, 2026-09-28; level minima:
  `level_1` 22.0 m, `level_2` 22.0 m, `level_3` 22.2 m, `night_1` 22.1 m) — tested for
  levels 1–3 by CAMV-008 (`camv_008_level_data_keeps_the_22_m_margin`) and for `night_1` by
  LAYOUT-N1-006; tools, in this order: clip the wander area (rect or a smaller `wander_radius_m`)
  on the side facing the board, move the spot inside its place, move the board along its
  enclosure, move the whole place (FIX-056 moves accepted, Q-145 answered).

## Joining levels (proposal, level design — Q-088)

The zoo is **one continuous map** split into levels:
- Every level file uses the **same level coordinates** (origin = centre of the zoo entrance
  gate, GAME-LAYOUT "Coordinate system"); nothing is translated when a level is loaded.
- The `bounds` of different levels never overlap. A later level lies directly behind a
  barrier of an earlier level: the cells of the new level that are edge-adjacent to that
  barrier's cells are listed in the new level's **`[[entry]]`** table (`id`, `cells` rect,
  `from_level`, `barrier`). Entry cells are path cells; they are the **only** walkable
  border cells of a level (all other border cells are solid, as for level 1).
- The engine loads **all unlocked levels** into one grid (union of their cells) and opens
  every barrier whose unlock condition is met (GAME-LAYOUT Behaviour 3); the player can walk
  back into earlier levels. Locked levels are not loaded (or loaded but unreachable — their
  entry barriers are closed). Saves keep one position in level coordinates (GAME-SAVE).
- A barrier belongs to the earlier level's file and names its transition
  (`transition = "level_1->level_2"`); a level may have several entries (level 3: from level 2
  and, proposal Q-090, through the level-1 north gate).
- Riddle details are unique **zoo-wide** among all levels that can be unlocked together
  (Q-083): a landmark or scenery kind a riddle relies on exists once in the joined map.
- Current plan (proposals): level 1 at x −24…23, z −2…47; level 2 east of it (x 24…75,
  z 12…61) behind `barrier_ne_tree`; level 3 north of level 1 (x −24…23, z 48…93) behind
  `barrier_l2_construction` (and `barrier_north_gate`); **night level 1** (`night_1`) west of
  level 1 (x −72…−25, z 6…53) behind the level-1 **moon door** (x −24…−23, z 29…30). The area
  east of level 1 south of level 2 (behind `barrier_east_repair`) is free for a later level
  (Q-023).

**All animals active from the level start (user request 2026-09-30).** When a level starts —
level 1 at a new game, a later level at the moment it unlocks (its entry barrier opens), a
night level when its door opens — **every animal of that level is already placed and active
somewhere on that level**: state `escaped` (GAME-ANIMALS), visible, simulated (idle /
wandering, perched animals sit and face the player, the goldfish swims), standing on a
walkable cell inside the chosen hiding place's wander area, and interactable as soon as its
mission is in scope. Nothing spawns late (not when the player comes near, not when the board is
read, not after a timer). The hiding places are chosen **once at game start** from the seed
(RESC-014, RESC-016), also for the locked levels, so an unlocked level never has to
"place" animals in front of the player. A pair (`pair = true`) is two animals at the same
place, both active. Every level file therefore needs: an enclosure and ≥ 3 `[[hiding_place]]`
candidates for **every** animal of `[level] missions` (or of every enclosure when the field is
absent), and no animal without a mission entry. Locked levels keep their animals hidden and
asleep (LAYOUT-025) — whether they should already be active from the very start of the game is Q-201.

**Implementation (M5b, 2026-09-26 — proposals Q-088…Q-094 implemented data-driven, still
open for confirmation):**
- `LevelData::join` joins `level-1.toml`, `level-2.toml`, `level-3.toml` into one grid (id
  `zoo`); cells of the joined bounding box that belong to no level are out of bounds. All
  three levels are always loaded; a locked level is sealed by its closed entry barriers,
  its animals are hidden and not simulated (asleep), its boards and animals are not
  interactable. **Missions in scope** = the union of the unlocked levels' missions (`[level]
  missions`, else every enclosure animal of the level). A level is unlocked when one of its
  `[[entry]]` barriers is open.
- **Barrier opening — the next morning (GAME-NIGHT, 2026-09-27; replaces the temporary rule
  of Q-091):** when the last mission of a level completes, the level is queued; after the
  night (the child sleeps in the bed) every barrier the level unlocks opens — `unlock_after =
  "<level>"` when that level is joined (e.g. the fallen tree after `night_1`, Q-078), else the
  exit barriers of the level (`transition = "<level>-><next>"`) — **and every entry barrier of
  the next level** (for level 3 also the level-1 `barrier_north_gate`, proposal Q-090). Moon
  doors are not exits (see "Moon door and night levels"). Opened barriers never close; their
  models disappear.
- **Discovery per level:** one pick per animal with the rule of GAME-RESCUE §1 (spread
  ≥ 12 m within the level); level 1 uses the main seeded RNG (unchanged from M5a), later
  levels a seeded RNG of their own (`seed ^ k·φ`).
- **Rendering (QA F12):** static batches are split into render regions — one per level, one
  per barrier (hidden when open), one per enterable-building roof — culled against the view
  frustum and the visible ground area. Measured (1280×720, max zoom): level-1 spawn 58 draw
  calls / 3.1 k instances (M5a-like), inside level 2 or 3 ≈ 55–60 / 2.8 k, at a level border
  2 levels (≈ 91 / 6.4 k), worst case the level-3 spawn where all three levels meet
  (≈ 119 / 9.2 k).
- Enterable buildings (Q-092): `interior` + `door` cells are walkable floor (surface
  `path`); the roof and the walls above 1 m are hidden while the player stands on an
  interior or door cell (PLAY-028). Since 2026-09-28 every building with a door is
  enterable (see "Enterable buildings").
- LAYOUT-024 treats background kinds (water bodies shared by one animal's places, the
  level-1 tree areas, walls, hedges and the three-part level-1 rock hill) as non-details.

## Level design rules — how a level is played and how the player is guided (user request 2026-09-30)

Binding rules for every level file (`levels/level-<N>.md`, `assets/levels/*.toml`). They pull
together what GAME-RESCUE, GAME-HINT, GAME-NIGHT and this spec define; a level that breaks
one is not done.

**A. How a level is played (the loop of one level)**

1. **Start:** the level is entered (level 1: new game at the entrance gate with the intro,
   GAME-RESCUE; later levels: walk through the opened barrier). All animals of the level are
   already active on it (rule 2 of "Joining levels", LAYOUT-044).
2. **Mission per animal**, in any order (the level is not linear): read the **info board**
   (location riddle) → take the right **food** from the food storage by reading the box labels →
   walk to where the riddle points → show the food → the animal follows → lead it through its
   **gate** into its enclosure (GAME-RESCUE). Every animal has ≥ 3 hiding places, chosen by seed.
3. **Optional side activities** never block a mission: garden and treats, golf carts, map,
   events (GAME-GARDEN, GAME-CART, GAME-EVENTS).
4. **Level complete** when every mission of the level is home: celebration, then dusk →
   the bed (sleep → next morning) or the moon door to the night zoo (GAME-NIGHT). The next
   level's barrier opens **the next morning**, never at the moment of the last animal.
5. **Free play** afterwards: the child may walk back through earlier levels; nothing is lost.

**B. How the player is guided (no text needed to navigate, no time pressure)**

6. **Every level needs a first step within sight of its start:** the spawn (or entry cell) has
   an info board or the food storage within 15 m, on screen in the zoo view. Level 1: welcome
   board / intro at the gate, first info board ≤ 15 m.
7. **The riddle is the guide, the world confirms it:** every hiding place has ≥ 2 riddle
   details visible from the walk there (landmarks, scenery, `features`), each unique in the
   joined map (Q-083); a child who understands the riddle beats guessing (LAYOUT-L1-006/007).
8. **Streets lead:** every board, food storage, gate, garden gate, bed and door is on or at the
   end of a street (path cells, full speed); streets run under gates and barriers (LAYOUT-040)
   and form loops, not dead ends; grass is slower (GAME-PLAYER §6) and is where the hiding places lie.
9. **Landmarks and signs orient:** every enclosure has a readable sign and a visible gate;
   the food storage is recognisable from the road; barriers are explained by their look (fallen
   tree, construction fence), never by an invisible wall (LAYOUT-019).
10. **Barriers limit scope, not hope:** while a barrier is closed the child sees why (and
    that "something comes later"); no closed area holds a mission of the current level
    (LAYOUT-002).
11. **The 🧭 hint always works** (GAME-HINT rule 4a): from the first second it shows the next
    step (read / take food / search / lead home / bed / moon door) as icon + line; the level
    data must therefore give every step a hint target: board, storage door, hiding-area
    circle, gate, bed, door (HINT-008/015). A level without a reachable target for any of these
    states is invalid.
12. **Wrong choices are free:** wrong food → "not interested", wrong gate → the animals stop
    and refuse, no penalty, no fail state, nothing can get lost or stuck (items are boxes
    that never run out, dropped items can be picked up again — GAME-FEED).
13. **Reading level scales, the layout does not:** the same map serves `kiga` to `klasse3`;
    only the texts, riddles (per reading level) and label difficulty change (CONT-READING);
    `kiga` is guided by pictures and read-aloud.
14. **Sizes:** first board reachable ≤ 15 s of walking from the start; the longest walk
    between two mission steps ≤ 60 s on streets; a level takes about 10–20 minutes for a
    child of the target reading level (proposal — Q-202).

Tests: LAYOUT-046 (guidance checks below) and the per-level tests of rules 6, 7, 8, 10.

## Enterable buildings (user request 2026-09-28)

**Every building with a door is enterable** once its level is unlocked ("make sure we can
enter all unlocked doors, like from the Futterhaus"): a `building` element with a `door` cell
always has an `interior` rect (LAYOUT-041). Today: `zookeeper_house_1`, `zookeeper_house_3`,
`night_house`, the food storages `food_storage` (interior (−3, 12, 6, 4)), `food_storage_2`
((40, 28, 4, 6)), `food_storage_3` ((0, 62, 6, 4)) and the food hut `food_storage_n1`
((−43, 27, 3, 4)). Buildings without a door (entrance arch, kiosk) stay solid blocks.
- **Floor:** interior and door cells are walkable (surface `path`); the ground height inside is
  the floor (PLAY-035). The remaining cells of the building rect — the 1 m **wall band** between
  the outer rect and the interior — stay solid; built-ins, furniture and food boxes may stand
  in it against the inner wall faces (they are reached from the interior). Where a model's
  walls are thinner than the band (food storage, food hut: 0.2 m walls on the rect edge), a
  low plank platform (0.15 m, part of the model) fills the band so the walkable floor ends
  visibly at the interior edge (LAYOUT-019, no invisible walls); the stock boxes stand on it.
- **Door:** the door cell touches an interior cell and a walkable cell outside; every interior
  cell is reachable from the door; the `door_wood` model opens while the player is within
  1.6 m of the door cell (`Game::opening_open`, "Gates and doors") and closes behind her.
- **Roof:** in the zoo view the roof, the walls above 1 m and a name board over the door (the
  "Futter" board of a food storage, the night-house board) are hidden while the player stands
  on an interior or door cell (PLAY-028); in first person the roof and its ceiling stay
  (CAMV-022).
- **Inside is solid where it looks solid:** furniture and stock boxes have colliders or stand in
  the solid wall band (GAME-PLAYER 9); nothing stands in the door walkway (LAYOUT-032).
- **Food storages** (Q-181 answered 2026-09-28): the labelled food boxes stay **outside** in a
  row in front of the door facade (GAME-FEED §7), the door gap wide enough for an enterable
  door (≥ 0.9 m beside each post, LAYOUT-038; approaches from 3 m aside free, LAYOUT-034).
  Inside (Q-194 answered 2026-09-29), a few more **real, labelled** food boxes stand on the
  plank platform along the walls, taken exactly like the outside ones (foods may repeat the
  outside row, FEED-008/FEED-028); geometry: FEED-027. The `klasse3` distractor boxes go
  inside once they exist (Q-032). The "Futter" board stays above the storage door (bottom
  2.3 m).

## Moon door and night levels (level design, Q-133 answered; GAME-NIGHT rules 3, 4, 7)

- A **night level** (`[level] time = "night"`, id `night_<N>`) is a normal level file in the
  shared coordinates. It is reachable only through a **moon door**: a barrier of kind
  `moon_door` in an earlier day level (`transition = "<day level>->night_<N>"`), listed as the
  night level's `[[entry]]` barrier.
- **Special opening rule:** a moon door is **not** an exit barrier of the Q-091 rule. It has
  `unlock_after = "<day level>"` and `opens_at = "night"`: it opens at every nightfall once
  that level's nightfall has happened (all its missions complete) and **closes again in the
  morning**; its model stays (the leaves swing; open = walkable). Before the first nightfall
  it is always closed (NIGHT-002).
- The day barrier to the next day level follows GAME-NIGHT rule 7 / Q-078: `unlock_after =
  "night_<N>"`, `opens_at = "morning"` (opens the morning after the night level is complete).
- Night levels are their own **riddle scope** (Q-136 answered): scenery kinds and riddle
  landmarks are unique among the night levels (LAYOUT-024 per scope); night riddles still
  avoid every `kiga` word of the day levels (CONT-MISSIONS MISS-013).
- **Enterable house with indoor enclosures** (first used by `night_house`, Q-134 answered):
  the building's `interior` is the visitor hall; `enclosure` elements with `indoor = true`
  lie directly next to it with their gate edge-adjacent to an interior cell; `model_rect` is
  the footprint of the whole house model (roof cut away while the player is on a hall/door
  cell). Info boards stand outside (a solid board cell inside the building rect would overlap
  it, LAYOUT-003).

## Night lights, interactables and furniture (Q-118 answered; data shape Q-137 answered)

- `[[light]]` (night-only props, hidden by day): `id`, `kind` = `lantern_post` |
  `string_lights` | `wall_lamp` | `board_lamp` | `indoor`, `pos` [x, z] (level metres),
  `facing` (+x/−x/+z/−z: direction of the lamp arm / lit side), optional `to` (second post of
  string lights), `attach` (building for `wall_lamp` / `indoor`, board for `board_lamp`),
  `radius_m`, `color` (`#RRGGBB`, indoor lights).
- **Placement rules (Q-118):** lantern posts along the **main paths ≈ every 10 m** and one
  **beside every enclosure gate** (it lights the enclosure sign); string lights **only over the
  entrance plaza** of a level (level 1; in a night level its entry plaza); a `wall_lamp` at
  every building door; a `board_lamp` on every info board and map board. Posts stand **0.25 m
  inside a path edge** (or on grass beside a gate), never inside a hiding-place `rect`, scenery,
  a garden or on a gate's leading cells; the gate lantern stands in front of the inner end of
  the enclosure sign (0.3 m past the sign's end nearest the gate, touching its front, ≥ 0.6 m
  off the fence — no slot, LAYOUT-039, Q-173); collider C(0, 0, 0.12) only while visible. Budget:
  point lights for the player lantern and the nearest ≤ 8 lamps, decals for the rest (Q-114).
- `[[item]]` (interactables the game logic uses; already used for the fish bowl, Q-093):
  `kind = "bed"` (sleep, NIGHT-003), `note_math_fighter` (GAME-CART 13), `key_box` (GAME-CART
  12, 14); fields `pos`, `building`, `facing`, `stand` (walkable cell the child uses it from).
- Info board elements may set `mount = "wall"` (Q-157, LAYOUT-038): a flat board on the building
  facade behind the board cell instead of a standing board — the cell stays walkable, no
  footprint, the board lamp clips above the panel. Used where a standing board would form a
  pocket beside a door (the three `night_1` boards on the night-house facade).
- `[[prop]]` (furniture inside buildings, not grid elements): `id`, `model`, `pos`, `facing`,
  `size_m` [w, d] (collider box until the model exists; [0, 0] = no collider), `building`.
- `[[event_spot]]` (GAME-EVENTS burglar event, Q-139 answered): `burglar_entry` (ladder on the
  inside of the wall, `pos`, `facing`), `burglar_target` (`target` building whose food box is
  taken), `burglar_hideout` (`rect` of walkable grass where they are caught; never a hiding
  place, scenery, garden or path). One set per day level.

## Levels

One file per level in `specs/10-gameplay/levels/level-<N>.md` (spec id `GAME-LEVEL-<N>`, level id
`level_<N>`, layout data `assets/levels/level-<N>.toml`) containing:
area list, enclosures and buildings (with grid rectangles), barriers with their unlock
condition, spawn point, and a top-down ASCII or SVG map.

| Level id | Area | Status |
|---|---|---|
| `level_1` | [levels/level-1.md](levels/level-1.md) (GAME-LEVEL-1): entrance, food storage, zookeeper house `zookeeper_house_1` (bed, cart key box), moon door to `night_1`, zebra, hippo and panda enclosures and their 9 candidate hiding places (`loc_river`, `loc_meadow`, `loc_sand`; `loc_pond`, `loc_mud`, `loc_shade`; `loc_cave`, `loc_bamboo`, `loc_leaves`); hippo pool `hippo_pool`; dense central grove, sparse woods `trees_nw` / `trees_ne`; vegetable garden `garden_veg` in the back (GAME-GARDEN, Q-102) | draft — proposal pending Q-023 |
| `level_2` | [levels/level-2.md](levels/level-2.md) (GAME-LEVEL-2): behind `barrier_ne_tree` (east); food storage 2, koala (pair), elephant, giraffe and lion enclosures, elephant pool, and their 12 candidate hiding places (`loc_treehouse`, `loc_tallest_tree`, `loc_blossom_tree`; `loc_fountain`, `loc_log_pile`, `loc_big_ball`; `loc_lookout_tower`, `loc_train`, `loc_playground`; `loc_sun_rocks`, `loc_stage`, `loc_deckchairs`); exit `barrier_l2_construction` | draft — proposal (Q-088, Q-089, Q-091, Q-094, Q-095) |
| `level_3` | [levels/level-3.md](levels/level-3.md) (GAME-LEVEL-3): behind `barrier_l2_construction` (north of level 1, second entry through `barrier_north_gate`, Q-090); zookeeper house with the fish bowl and a tap, food storage 3, stream with waterfall, monkey, goldfish (pond) and snow fox enclosures, adventure playground with the pirate ship, and their 9 candidate hiding places (`loc_pirate_ship`, `loc_carousel`, `loc_trampoline`; `loc_waterfall`, `loc_water_wheel`, `loc_willow`; `loc_ice_cream_kiosk`, `loc_sprinkler`, `loc_laundry`) | draft — proposal (Q-017, Q-088…Q-095) |
| `night_1` | [levels/night-1.md](levels/night-1.md) (GAME-LEVEL-NIGHT-1): the night zoo west of level 1 behind the level-1 `moon_door` (open at night); plaza with string lights, night food hut, night house with the indoor enclosures of hedgehog, bat and owl, loop path around dense old-tree groves, and 9 candidate hiding places (`loc_brush_pile`, `loc_flowerpots`, `loc_mushrooms`; `loc_windmill`, `loc_fireflies`, `loc_hollow_tree`; `loc_moon_pond`, `loc_hilltop`, `loc_fir`) | draft (Q-133…Q-138 answered) |
| `level_4` | *planned (Q-174 answered 2026-09-28), not specified yet:* the only known content is the **bear enclosure** (bear pair, needed for the honey part of the bee event, GAME-EVENTS); its place in the map, entry, other enclosures and hiding places are open (Q-175) | planned |
| later levels | further night levels (`night_2`…) and day areas after Q-023 | — |

## Behaviour

1. The player can never leave the union of walkable cells (path and grass) of all unlocked levels.
2. Each barrier belongs to exactly one level transition and has one unlock condition (Q-022).
3. When a barrier is removed, a short in-world animation plays (e.g. a zookeeper rolls the
   stone away); the removed area becomes walkable. Nothing that could look like a riddle
   detail stays behind (e.g. the logs of `barrier_ne_tree` are carted away, not piled up —
   the only log pile is the elephant's `loc_log_pile`, GAME-LEVEL-2).
4. Every enclosure, building and landmark is reachable by path from the spawn point within
   its level.
5. Level and world coordinates are converted only by `world = (x, 0, −z)` (Q-056); models
   are never mirrored and are oriented by rotations about +Y only.
6. Every straight run of fence, hedge or wall is filled by 2 m segments plus, for odd
   lengths, one 1 m segment at the end; turns use corner pieces (Q-057).
7. *(Proposal Q-088)* All levels share one coordinate system; unlocked levels are joined
   into one grid through their `[[entry]]` cells (see "Joining levels").


## Forests: dense vs. walkable (user decision 2026-09-26)

Tree areas have a **density**:
- `dense` (e.g. the hidden grove in the middle of level 1, forest edges along the zoo
  wall): solid as a whole — blocks walking and view (as today).
- `sparse` (open woods, tree groups in meadows): **walkable between the trees**. Only each
  **trunk** is solid (circle, r ≈ 0.35 m; bushes r ≈ 0.6 m); trees stand ≥ 1.8 m apart
  (trunk to trunk) so the player (r 0.3 m) and following animals fit through. Canopies do not
  block walking; when a canopy hides the player the occluder fade applies (GAME-PLAYER §2).
Every other prop keeps its own collision shape (GAME-PLAYER §7) — in particular **info
boards, enclosure signs and the map board are always solid**, so the player can never walk
into or through a billboard.

**Level data for tree areas** *(proposal, level design — Q-085)*:
- Every `decoration` of kind `trees` / `tree_grove` has `density = "dense" | "sparse"`.
- `dense`: all cells of the rectangle are solid (as before). Its walkable sides get a
  visible border (`edge = "bushes"`: `bush` props every ≈ 1.3 m, centred 0.6 m inside the
  edge), so the solid edge is never an invisible wall under the canopies (LAYOUT-019).
- `sparse`: the element is **not solid** — its cells are walkable with surface `grass`.
  It lists every tree and bush explicitly: `trees = [{ pos = [x, z], model = "tree_round" |
  "bush" }, …]` (level coordinates of the trunk centre). Rules: every collider (footprints
  of "Collision footprints" below: `tree_round` r 0.45, `bush` r 0.70) lies fully inside the
  rectangle; the clear gap between two obstacles is ≥ 1.8 m (a bush touching a trunk forms
  one obstacle with it); the player can cross the area west–east and south–north. Sparse
  areas never overlap a path, scenery or another solid element, like any element.
- Hiding places next to sparse woods: a place's wander area is additionally clipped to the
  place's `rect`, so opening a wood never enlarges a wander area (LAYOUT-014 unchanged).

## Enclosure features and wandering at home (proposal, level design — Q-085)

GAME-ANIMALS: an animal `in_enclosure` wanders slowly inside its enclosure. Where it may
walk is level data:
- Enclosure elements carry `home_wander_on` — a list of surfaces, default `["grass"]`.
  `grass` = cells of the enclosure rectangle that are not gate cells, not covered by an
  enclosure feature (except its ramp) or by a reserved building area, and whose centre is
  not blocked by a prop footprint; `water` = the cells of the enclosure's `pool` feature.
- `[[enclosure_feature]]` (like `[[food_box]]` and `[[scenery]]` **not** an element; the
  enclosure cells stay solid for the player, so LAYOUT-003 is unaffected): `id`, `enclosure`,
  `kind = "pool"` (or `kind = "hut"`: a reserved building area, not part of the home wander
  area — *proposal Q-098*, first used by `hippo_hut`), `rect` (inside the enclosure rectangle), `water = "still"`, `ramp` (cells
  of the entry ramp, part of `rect`), `ramp_side`, `edge_stones` (decoration rocks just
  outside the rim, solid for the animal), `model`.
- Water and grass of the home wander area connect **only through ramp cells** (the rim is
  0.4 m high), so the animal visibly walks in and out over the ramp.
- *Proposal:* an animal with `water` in `home_wander_on` picks a water cell for 7 of 10 new
  wander targets (hippos spend most time in the water; seeded RNG, ANIM-011).

## Collision footprints (proposal, level design — Q-087)

Measured from the exported `.glb` files: the cross-section of each mesh between 0.05 m and
1.4 m height (player height 1.20 m + margin) in model space (x = model right, z = model
front = world +Z at yaw 0), compared with `zoo_core::collision::footprint`. Proposed
values cover the cross-section within 0.02 m and extend at most 0.1 m beyond it (LAYOUT-018).
`B(x, z, hx, hz)` = box centre and half extents, `C(x, z, r)` = circle.

| Model | Mesh cross-section 0.05–1.4 m (x; z) | Current footprint | Proposed footprint | Issue |
|---|---|---|---|---|
| `info_board` | −0.50…+0.50; −0.37…+0.25 | B(0, −0.05, 0.50, 0.25) | B(0, −0.06, 0.51, 0.32) | panel overhangs the box by 0.07 m (back) / 0.05 m (front); cell is solid, so not reachable today (F13) |
| `enclosure_sign` | −1.18…+1.18; −0.14…+0.35 — posts at x ±1.09, **panel between the posts from 0.99 m** up to 1.8 m | **B(0, +0.10, 1.19, 0.26)** (the whole sign) | — | decided 2026-09-27 (Q-086 (b)): the sign stands **beside** its gate (LAYOUT-033), so the whole sign is solid and no longer a billboard over the gate |
| `map_board` | −1.07…+1.07; −0.14…+0.22 — panel from 0.73 m | two posts C(±1.0, 0, 0.14) | B(0, +0.04, 1.08, 0.19) | panel between the posts is not solid; today only the solid `map_board` cells stop the player (LAYOUT-017 must not rely on cells) |
| `tree_round` | trunk ±0.37 (0.4–1.4 m); root flare r 0.72 at 0–0.2 m | C(0, 0, 0.35) | C(0, 0, 0.45) | trunk barely covered; the feet may overlap the flat roots (sparse woods) |
| `tree_grove` | trunk ±0.26 below 0.9 m; branches from 0.9 m to −0.77…+0.65 at 1.4 m | C(0, 0, 0.35) | keep (only used in `dense` areas) — C(0, 0, 0.75) if ever used in a sparse area | low branches at head height |
| `tree_eucalyptus` | trunk ±0.13 | C(0, 0, 0.35) | C(0, 0, 0.20) | footprint 0.22 m larger than the trunk (invisible wall); not placed in level 1 |
| `bush` | −0.67…+0.67; −0.55…+0.64 | C(0, 0, 0.55) | C(0, +0.05, 0.67) | 0.12 m of bush not solid (F13) |
| `rock` | −0.61…+0.86; −0.46…+0.52 (off-centre) | C(0, 0, 0.50) | C(−0.14, +0.03, 0.49) + C(+0.37, +0.03, 0.49) | 0.36 m of rock not solid on +x (F13) |
| `bamboo` | stalks −0.31…+0.45; −0.44…+0.33 (below 0.9 m); leaves to −0.46…+0.67; −0.87…+0.72 | C(0, 0, 0.60) | ~~C(+0.10, −0.07, 0.80)~~ **B(+0.105, −0.075, 0.575, 0.80)** (M5a: the circle reached 0.24 m beyond the mesh, LAYOUT-018) | leaves 0.27 m outside; only in solid cells today |
| `road_block` | −1.05…+1.05; −0.41…+0.41 (feet) | B(0, 0, 1.05, 0.20) | B(0, 0, 1.05, 0.42) | feet 0.21 m outside the box |
| `repair_sign` | −0.37…+0.37; −0.11…+0.10 (panel 0.6–1.5 m) | C(0, 0, 0.15) | B(0, 0, 0.38, 0.12) | billboard-like panel 0.22 m wider than the post circle |
| `zookeeper_cart` | −0.94…+1.38; −0.56…+0.56 | B(0, 0, 1.15, 0.56) | B(+0.22, 0, 1.17, 0.57) | handle end 0.23 m outside, 0.21 m invisible at the other end (F13) |
| `gate_zoo_closed` | −1.56…+1.56; −0.36…+0.36 | none | B(0, 0, 1.56, 0.36) (M5a: its pillars reach 0.11 m past the band edge once the leaf stands 0.25 m behind it) | thin leaf between two deep pillars |
| `fallen_tree` | −1.25…+1.17; −2.45…+1.81 | none (barrier cells only) | B(−0.04, −0.32, 1.22, 2.14), removed with the barrier | at `barrier_ne_tree` (placed at the rect centre, yaw 0) the trunk reaches x 21.75 — 0.25 m into the walkable column x = 21 of `path_ne`; the player walks into it |
| `food_box`, `food_box_stack`, `traffic_cone`, `bridge_wood` | — | — | keep | match within 0.02 m |

**Invisible walls (QA F7, F8)** — grid-solid cell edges facing a walkable cell with no
visible geometry (0.05–1.4 m) within 0.3 m, measured on level 1 with the current scene
assembly. Proposed fixes *(Q-087)*:

| Where | Gap between solid edge and visible model | Proposed fix |
|---|---|---|
| `zoo_wall` bands (`wall_west`, `wall_south_w/e`) | 0.70 m | draw every hedge/wall band on its **walkable-side row** (piece centre 0.5 m from the walkable edge) instead of the band centre line (changes the Q-060 (b) proposal); the outer row stays solid and hidden behind it |
| `hedge` bands (`hedge_north_*`, `hedge_east_*`) | 0.48–0.50 m | same |
| `barrier_north_gate` (`gate_zoo_closed`, 0.6 m deep in a 2 m band) | 0.70 m | place the barrier model on the walkable-side row of its rectangle — *M5a:* the leaf is thin, so the gate stands with its leaf 0.25 m behind the edge |
| `barrier_east_repair` (`road_block`, faces west) | 0.60 m | same (road block centre 0.45 m from the west edge x = 22) — *M5a:* its bar is thin, so the centre stands 0.30 m behind the edge (feet 0.12 m on the path, solid) |
| `entrance_gate` (arch open over cells x −2…1, z −1) | whole cell | closed turnstile/gate placeholder in the arch (QA F8) |
| `grove_center` edges along the ring path | up to the whole cell (trees on a 2.5 m grid, trunks r 0.26 at walking height) | `edge = "bushes"` border (see "Forests") |
| `trees_nw`, `trees_ne` | up to the whole cell | now `sparse` — only trunks and bushes are solid |
| info boards (front), `map_board` (back strip) | 0.26 m / 0.36 m | acceptable (≤ 0.4 m, the board fills its cell's width); no change |


## Gates and doors (user request 2026-09-27)

Every opening in a fence, wall, hedge or building that the player or an animal passes
through has a **real gate or door model** — never an empty gap or a placeholder:
enclosure gates (`gate_wood`, 2 m), the garden gate (`garden_gate`, two leaves), the night
house enclosure doors (`glass_door`), building doors (`door_wood`: zookeeper house, food
storage, food hut, night house), the entrance turnstiles (`turnstile`) under the entrance
arch, barrier gates (`gate_zoo_closed`) and the moon door (`moon_door`). Gates and doors
**open visibly**: enclosure gates swing open when the player leads animals through (and
close behind them), the garden gate opens when the player is within 2 m, building doors open
while the player passes, the moon door opens at nightfall. Closed gates are solid; open gates
are walkable only as the rules allow (enclosure gates only while leading animals — GAME-RESCUE).

**Gates between the levels** (user request 2026-09-27): every level transition — each
`[[entry]]` of a level and the barrier it names (`barrier_ne_tree` level 1 → 2,
`barrier_l2_construction` level 2 → 3, `barrier_north_gate` level 1 → 3, and every later
entry) — has a **real zoo gate** (`gate_zoo`, big double gate with two leaves, the
`gate_zoo_closed` look when shut) set into the level's boundary hedge/wall line across the
entry path, never just a gap. While the next level is locked the gate is **closed and
solid** and the story barrier (fallen tree, construction fence, road block) stands in front
of it on the old level's side; when the level unlocks (next morning, GAME-NIGHT) the barrier
is cleared and the gate **swings open visibly** and stays open (walkable, both directions).
`barrier_north_gate` *is* such a gate (no extra barrier). `barrier_east_repair` (road block
to a later level) gets its gate too once that level exists. The gate's leaves open outwards
into the new level and its posts join the hedge/wall on both sides without a gap.

**The street continues under the gate** (user request 2026-09-28, LAYOUT-040): at every level
transition the path is continuous — path cells under the barrier cells and under the gate,
joining the old level's path to the new level's `[[entry]]` path without a grass gap, a curb
(edging stone across the path) or a seam; the story barrier (fallen tree, construction fence,
closed gate) stands **on** the street, and after unlocking the child walks on path all the way.
Data: the old level's path element runs on over the barrier cells (`path_ne` to x 23,
`path_north` to z 47, `path_l2_nw` from x 24); a path cell under a barrier is the one allowed
"path under a solid element" (LAYOUT-003 and the per-level variants). **The moon door too**
(Q-182 answered 2026-09-28): `path_moon` runs on under the door cells (x −24…−23) to
`night_1`'s `path_n1_entry`, so the child walks on path through the open door.

**Enclosure signs stand beside the gate** (user decision 2026-09-27, Q-086 (b)): the
`enclosure_sign` never stands in front of or over a gate. It stands outside the fence along
it, on the path side, with ≥ 0.9 m between the sign and the gate post (nothing solid within 0.9 m beside a post, Q-157, LAYOUT-038), flush in front of the fence (back of its footprint 0.09 m off the fence line — no slot behind it, LAYOUT-039); its whole footprint
(B(0, 0.10, 1.19, 0.26), solid) lies on walkable cells outside the gate opening and the
≥ 1 m walkway in front of it (LAYOUT-032), clear of the info board and the food boxes, with
a free place ≥ 1 m in front of it to look at it. The engine tries the side away from the
enclosure's info board first (`scene::enclosure_sign_spot`). Indoor enclosures of the night
house: a silhouette board above the glass door. Level 1: `board_hippo` and
`hedge_hippo_nw` moved to make room at the hippo gate (GAME-LEVEL-1). The 22 m hiding-place
rule and CAMV-008 are unchanged (tests green).

**Doors are never blocked** (user request 2026-09-27): no prop, board, lamp or furniture
stands in a door or gate opening or in the ≥ 1 m walkway in front of it (the player's body
width); a sign that belongs to a door goes beside or above it (indoor enclosures of the
night house: a silhouette board above the glass door, bottom 2.3 m). This holds for
food storage doors (since 2026-09-28 every door is enterable): the food-box row in front of
a food storage leaves a free gap ≥ 1.2 m wide in front of the door, the whole door opening
inside it (Q-150, user 2026-09-27), and — the door being enterable — nothing solid within
0.9 m beside its posts (LAYOUT-038; Q-181 answered: boxes outside); the stock boxes inside
keep the door walkway inside free the same way (Q-194). Also
nothing without a collider (lamp posts, taps, items) stands there, and no lamp post in the
2.5 m leading lane straight in front of an opening.

**No wall gaps near a door or gate** (user decision 2026-09-28, Q-173): within **3 m** of
every door and gate opening (enclosure gates, glass doors, building doors, the garden gate,
level gates, the moon door) nothing solid stands **0.1–0.6 m** in front of a wall, fence,
hedge or facade — a solid thing there is either **flush** (gap < 0.1 m) or leaves **≥ 0.6 m**
(the player's body fits through). A gap in between is a slot a child walking along the wall
towards the opening can get wedged in. "Solid" = every prop collider (signs, taps, garden
signs, furniture, …) and the lantern posts at night (LAYOUT-035); "wall" = every cell that is
not walkable (gate cells count as open) and the thin garden-fence runs; the opening's own
gate/door parts do not count. Fixes of 2026-09-28: enclosure signs flush on the fence (0.19 m
slot before), the gate lanterns in front of the sign's inner end (they stood 0.2–0.4 m in
front of the fence between gate post and sign), `l2_lantern_ring_w_n` 0.45 m farther east
(0.44 m from the corner of `board_koala`), the level-1 garden signs ≥ 0.6 m off the panda
fence and the garden fence (LAYOUT-039).

*Implementation (2026-09-27):* `zoo_core::scene::LevelScene::openings` lists every gate /
door model with its opening and model widths; `Game::opening_open` decides: building doors
of **enterable** buildings — since 2026-09-28 every building with a door — while the player
is within 1.6 m of the door cell (a door of a building without an interior would stay shut), enclosure gates (`gate_wood`, indoor `glass_door`) while
the player leads animals (or carries one in its container) within 3 m — they stay open
1.5 s behind the animals —, the garden gate within 2 m (proposal Q-102), the moon door with
its barrier. The presentation eases the open amount (≈ 0.5 s open, 0.8 s close):
`door_wood` / `gate_wood` swing as a whole by 90° (inwards / into the enclosure), the
leaves of `garden_gate`, `glass_door` and `moon_door` open to their back. Collision is
unchanged (door cells walkable, gate cells by the leading rule, the garden fence edges and
the garden-bed / sign / tool footprints solid). Building models: `zookeeper_house`,
`food_storage` (also `food_storage_2`, turned −90°, and `food_storage_3`), `food_hut`,
`night_house`, `entrance_arch` replace the procedural boxes where footprint and door cell
fit; `zookeeper_house_3` (7 × 6 m) stays procedural (with a `door_wood` in its gap) until a
model fits. Entrance turnstiles: three `turnstile` units 0.2 m nearer the plaza than the
README row (glTF z −0.7) so no invisible wall remains (LAYOUT-019).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| LAYOUT-001 | Given level N's layout data, then every enclosure/building/landmark is reachable from the spawn by walkable cells (flood fill). | unit |
| LAYOUT-002 | Given level N is active, then no walkable cell of a locked level is reachable (barriers seal the scope). | unit |
| LAYOUT-003 | Given all layout data, then no two solid elements overlap. | unit |
| LAYOUT-004 | Given a barrier's unlock condition is met, then its cells become walkable and LAYOUT-001 holds for the next level. | unit |
| LAYOUT-005 | Given every level spec, then its elements match the layout data file (ids and rectangles). | unit |
| LAYOUT-006 | Given a level's layout data, then every walkable cell covered by a `path` element has surface `path` and every other walkable cell has surface `grass` (Q-046). | unit |
| LAYOUT-007 | Given any level point (x, z), when converted with `level_to_world` and back with `world_to_level`, then the result equals (x, z), and `level_to_world(x, z) = (x, 0, −z)` (Q-056). | unit |
| LAYOUT-008 | Given a camera in world space looking north (forward = −Z, up = +Y), then its right vector (forward × up) is `level_to_world` of level east (+x), and level north maps to forward. | unit |
| LAYOUT-009 | Given `level-1.toml` and the default camera of GAME-PLAYER §2 (south of the spawn looking north, pitch 55°, 14 m, 35° vertical FOV), then an element east of the spawn (`bench_plaza`) projects to the right of the spawn on screen, one west of it (`map_board`) to the left, and one north (`food_storage`) above it. | unit |
| LAYOUT-010 | Given a ring of points that is clockwise seen from above in the level map (north up, east right), when converted to world space and viewed from above (camera looking down −Y with north up), then the ring is still clockwise (no mirroring). | unit |
| LAYOUT-011 | Given level north in world space, when rotated about +Y by −90°, then it equals level east (a clockwise quarter turn seen from above); +90° gives level west. | unit |
| LAYOUT-012 | Given a run of length L for every L in 1..=60 (incl. the level-1 lengths 3, 9, 13, 17, 21 and even lengths), when split by `segment_run`, then the segments are contiguous from offset 0, their lengths sum to L, all are 2 m except exactly one 1 m segment at the end when L is odd. | unit |
| LAYOUT-013 | Given `level-1.toml`, then every `hedge` / `zoo_wall` band has a unique run direction and centre line and is filled exactly by `segment_run`, and every enclosure has a 2 m gate on one side, not overlapping a corner arm, and every fence run between corners and gate is filled exactly. | unit |
| LAYOUT-014 | Given every level's layout data, then every animal with an enclosure in that level has ≥ 3 `[[hiding_place]]` candidates, each with a wander area of ≥ 9 cells, candidates of different animals never overlap (rects and wander areas), and every candidate is part of at least one pick with all chosen spots ≥ 12 m apart (level 1: LAYOUT-L1-014/015). | unit |
| LAYOUT-015 | Given a `sparse` tree area, then the player can walk from one side to the other between the trees, never overlaps a trunk collider, and trunks are ≥ 1.8 m apart. | unit |
| LAYOUT-016 | Given a `dense` tree area, then no cell inside it is reachable for the player. | unit |
| LAYOUT-017 | Given every info board, enclosure sign and map board of the level, then the player cannot overlap its collider from any direction (billboards are solid), and no walkable position lets the player circle overlap the panel's cross-section below 1.4 m (the 1.20 m player never passes through or under a panel lower than 2.1 m — Q-086). | unit |
| LAYOUT-018 | Given every prop model with a footprint and its `.glb`, then the footprint covers the mesh cross-section between 0.05 m and 1.4 m height within 0.02 m and extends at most 0.1 m beyond it (table "Collision footprints"; QA F13). | unit |
| LAYOUT-019 | Given a level's layout data and scene assembly, then every edge between a grid-solid cell and a walkable cell has visible model geometry (0.05–1.4 m) within 0.3 m of the edge on the solid side (no invisible walls; QA F7/F8; exceptions only listed in the level spec). | unit |
| LAYOUT-020 | Given an enclosure with `home_wander_on`, then its home wander area (definition in "Enclosure features and wandering at home") has ≥ 9 cells per listed surface, contains no gate cell and no cell adjacent to a gate cell on the `water` surface, and water and grass cells connect only through ramp cells. | unit |
| LAYOUT-021 | Given all level files, then their `bounds` are pairwise disjoint, every `[[entry]]` cell is a path cell on the level border that is edge-adjacent to a cell of the named barrier in the named earlier level, and every other border cell of every level is solid (Q-088). | unit |
| LAYOUT-022 | Given levels 1…N joined and exactly the barriers of completed transitions open, then the flood fill from the level-1 spawn reaches every walkable cell of the unlocked levels that LAYOUT-001 requires and no cell of a locked level; opening the next transition's barrier(s) makes the next level's spawn reachable. | unit |
| LAYOUT-023 | Given a `building` with `interior` and `door` (Q-092) — since 2026-09-28 every building with a door (LAYOUT-041) — then its interior and door cells are walkable with surface `path`, its other cells are solid, and the door cell is edge-adjacent to a walkable cell outside the building. | unit |
| LAYOUT-024 | Given all levels joined, then every `[[scenery]]` kind and every element kind named as riddle scenery of a hiding place (e.g. `fountain`, `waterfall`, `treehouse`, `pirate_ship`) occurs only at that one hiding place in the joined map (zoo-wide riddle uniqueness, Q-083). | unit |
| LAYOUT-025 | Given the joined zoo with level N locked, then the animals of level N are hidden and not simulated (positions unchanged after 120 s), its info boards, animals and gates are not interactable, and the missions in scope are exactly the union of the unlocked levels' missions (Q-088 proposal, RESC-025). | unit |
| LAYOUT-026 | Given all level files, then every `river` / `stream` element has a `flow` of N/E/S/W and no other element has one; the river pieces (incl. aligned bridges) chain along the flow with constant width and 90° bends at the upstream end of the next piece (level 1: `river_n` → `bridge_river` → `river_mid` → `river_e`, one left bend; level 3: `stream_l3`); a missing or invalid `flow`, a reversed flow or a bend at the wrong end is rejected. (Q-066) | unit |
| LAYOUT-027 | Given every level's `[[light]]` list, then every `lantern_post` and string-light end stands on a walkable cell outside every hiding-place `rect`, scenery rect and garden and ≥ 0.7 m from every food box; every info board and map board has exactly one `board_lamp` attached; every outdoor enclosure gate has a lantern post within 3 m of the gate centre (indoor enclosures: an `indoor` light attached to them); every `attach` id exists (Q-118, Q-137). | unit |
| LAYOUT-028 | Given a moon door (`kind = "moon_door"`), then it is closed by day and before its `unlock_after` level's nightfall, open at night afterwards (every night), closed again the next morning, never removed; and it is not opened by the Q-091 exit-barrier rule (Q-133). | unit |
| LAYOUT-029 | Given a building with indoor enclosures (`indoor = true`), then each indoor enclosure's gate is edge-adjacent to an `interior` cell of the building, the building's `model_rect` contains the building rect and the indoor enclosures, and none of the enclosure's boards lies inside the building rect (Q-134). | unit |
| LAYOUT-030 | Given every day level's `[[event_spot]]` list, then it has one `burglar_entry`, one `burglar_target` naming an existing building and one `burglar_hideout` of walkable grass reachable from the target, overlapping no hiding-place rect, scenery, garden or path (Q-139). | unit |
| LAYOUT-031 | Given every gate/door opening in the joined zoo (enclosure gates, garden gate, building doors, night-house doors, entrance, barrier gates, moon door), then a gate/door model is placed that exactly fills the opening (no gap > 5 cm, no overlap with posts), and it opens/closes per the rules. | unit |
| LAYOUT-032 | Given every gate/door opening in the joined zoo (LAYOUT-031), then no prop, board, lamp, item or furniture footprint lies in the opening or in the ≥ 1 m walkway in front of it, and a sign that belongs to a door is beside it or above it (bottom above the opening height); also for the food storage doors: the food-box row leaves a free gap ≥ 1.2 m around the door (Q-150; boxes outside, Q-181 answered) and the stock boxes inside stand clear of the door walkway (Q-194). This also holds for things without a collider (food boxes, items, furniture props, water taps, night lamp posts and string-light posts), and no lamp post stands in the 2.5 m leading lane straight in front of an opening (QA 2026-09-27). | unit |
| LAYOUT-033 | Given every outdoor enclosure gate of the joined zoo (levels 1–3, `night_1`), then its `enclosure_sign` stands beside the gate — ≥ 0.9 m from the gate post along the fence (centre ≥ 3.09 m from the gate centre; Q-157, LAYOUT-038), ≤ 5 m away — and the gate's 1 m walkway has no collider; the sign's footprint lies on walkable cells clear of the info board and the food boxes (user decision 2026-09-27, Q-086 (b)). | unit |
| LAYOUT-034 | Given every opening the player may pass (doors of enterable buildings, the garden gate, enclosure gates and glass doors while leading), when she walks straight at it from 3 m in front, from 2.5 m out and 2.5 m aside, and from 1.5 m out and 3 m aside (both sides), then she reaches the opening without getting stuck (≥ 5 cm progress per second) — nothing beside an opening forms a pocket (QA 2026-09-27; no exceptions since Q-157 was answered). | unit |
| LAYOUT-035 | Given a lantern post of any level, then it has the collider C(0, 0, 0.12) while it is visible (at night) and none by day (the `[[light]]` placement rules, Q-118/Q-137). | unit |
| LAYOUT-036 | Given every `[[entry]]` in the joined zoo, then a `gate_zoo` model stands across its entry path in the boundary line with its posts touching the hedge/wall on both sides (no gap); while the level is locked the gate is closed and solid, after unlocking it is open, walkable both ways and drawn open; the story barrier is gone. | unit |
| LAYOUT-037 | Given the review screenshots of each level transition before and after unlocking, then a closed gate (behind its barrier) and later an open gate are visible. | e2e |
| LAYOUT-038 | Given every door, gate and level gate, then no solid item stands within 0.9 m beside its posts so that a player walking at the opening at 45° can get caught in a corner (no pockets, Q-157). | unit |
| LAYOUT-039 | Given every door and gate opening of the joined zoo with `night_1` (level gates and the moon door included) and the lantern posts solid (night), then no prop collider or lantern post within 3 m of the opening stands 0.1–0.6 m in front of a wall, fence, hedge or facade (non-walkable cell or thin garden fence) — it is flush (< 0.1 m) or ≥ 0.6 m away (Q-173). | unit |
| LAYOUT-040 | Given every level transition of the joined zoo (each `[[entry]]`, the moon door included since Q-182) and each of its entry cells, then the cells in a straight line from 2 m inside the new level through the entry cell and every barrier cell to the first cell of the old level are path cells (the barrier stands on the street), walkable path once the barrier is open, drawn as path tiles with no edging stone across the line (no grass gap, curb or seam). | unit |
| LAYOUT-041 | Given every `building` with a `door` in the joined zoo with `night_1`, then it has an `interior` (enterable); its door cell touches an interior cell and a walkable cell outside; every interior cell is reachable from the door over interior cells; its `door_wood` is an enterable building door that is open while the player is at the door and shut when she is ≥ 3 m away; its roof hides inside (building model or roof region); every stock box inside is solid (Q-194). | unit |
| LAYOUT-042 | Given the level-1 → level-2 gate opened (fallen tree cleared), when the player stands on `path_ne` west of it looking east, then the open gate is on screen and path pixels run continuously from her feet through the gate into level 2 (review screenshot `level-gate-street.png`). | e2e |
| LAYOUT-043 | Given the player walks from the ring through the door into the level-1 food storage, then the door opens, she stands on the floor inside, the roof and the "Futter" board are hidden in the zoo view and drawn again in first person (review screenshots `storage-inside-zoo.png`, `storage-inside-fp.png`, the stock boxes along the walls), and she walks back out through the door and takes the grass from its box in the row outside (Q-181 answered). | e2e |
| LAYOUT-044 | Given each level file (`level-1`, `level-2`, `level-3`, `night-1`) and any seed, when the level starts (new game / barrier opened / moon door opened), then for every animal of the level (every `[level] missions` entry, else every enclosure animal; both members of a pair) exactly one chosen hiding place exists, the animal is in state `escaped`, visible, not asleep, its position is a walkable cell inside that place's wander area, and after 10 s of simulation it has reacted (animation state ≠ frozen, ANIM-008 limits hold); no animal is missing or spawned later (user request 2026-09-30). | unit |
| LAYOUT-045 | Given the level data, then every enclosure animal of a level is listed in that level's `missions` (or the field is absent) and has ≥ 3 `[[hiding_place]]` candidates, so no animal of an in-scope enclosure is left without a place. | unit |
| LAYOUT-046 | Given each level file, then (rule 6) an info board or the food storage lies within 15 m of the spawn / entry cell and on screen in the zoo view; (rule 8) every board, storage door, enclosure gate, garden gate, bed and door has a `path` cell within 2 m and is connected to the spawn over `path` cells only; (rule 10) no mission element of the level lies behind a closed barrier; (rule 11) from every seeded game state (new, board read, food carried, following, level done, dusk, night) the hint has a reachable target; (rule 14) the walking distance from the spawn to the first board is ≤ 15 s and between any two mission steps ≤ 60 s on streets. | unit |

## Open questions

- Q-201 answered 2026-09-30: locked levels stay asleep until opened. Q-202 answered 2026-09-30: pacing numbers stand.

- Q-006, Q-017, Q-022, Q-023.
- Answered 2026-09-27 (as recommended): Q-133 moon door / night level data and opening rule, Q-134 night house with indoor enclosures, Q-135 night food storage, Q-136 riddle scope of night levels, Q-137 `[[light]]` / `[[item]]` / `[[prop]]` data shape, Q-138 telescope, Q-139 burglar event spots. Q-118 (answered) lantern placement, Q-110 (answered) 17 m fog margin.
- Q-104 draw-call budget for the joined zoo, Q-105 background kinds exempt from LAYOUT-024.
- Q-102 garden data (`[[garden]]`, `[[garden_bed]]`, `[[plant_spot]]`, fence edges, self-opening gate, animals never enter).
- Q-088 joining levels (one continuous map, `[[entry]]`), Q-089 food storage per level, Q-090 level-1 north gate as second level-3 entry, Q-091 (answered 2026-09-27) barrier opens the next morning, Q-092 enterable buildings (`interior`, `door`), Q-093 fish bowl / water-source data, Q-094 `perch_height_m`, Q-095 new hiding places of levels 2–3.
- Q-056 answered: coordinate spaces (level x east / z north; world = (x, 0, −z)).
- Q-057 answered: 1 m segment variants and the fill rule. Q-059 band joins, Q-060 enclosure fence and band placement (proposals), Q-061 front direction of props (open).
- Q-085 tree-area data (`density`, `trees`, `edge`), `[[enclosure_feature]]`, `home_wander_on`, wander areas clipped to `rect`. Q-086 answered 2026-09-27: (b) enclosure sign beside the gate (LAYOUT-033; distance ≥ 0.9 m since Q-157). Q-087 collision footprint values and invisible-wall fixes (band row on the walkable side). Q-098 `kind = "hut"` enclosure feature. Q-099 remaining invisible walls (`map_board` back, fallen tree).
- Q-044 `hiding_place` element type and `blocks_view` (proposal above). Q-080 (answered) `[[hiding_place]]` / `[[scenery]]` lists, wander area data. Q-069 (answered) `[level] missions`. Q-046 walkable ground (answered). Q-049 high-angle camera (answered — sight test is a screen test; FOV axis Q-052).
- Q-150 answered 2026-09-27: the food-box rows leave a gap ≥ 1.2 m in front of the storage doors (no exception to LAYOUT-032 any more). Q-157 answered 2026-09-27: no pocket beside a door or gate (zebra board, level-3 tap and night-house boards moved; LAYOUT-034, LAYOUT-038); wall-gap part: Q-173 answered 2026-09-28 (LAYOUT-039, "No wall gaps near a door or gate"). Q-171 answered 2026-09-28: `board_zebra` south of its gate, 22 m margin tested for levels 1–3 (CAMV-008). Q-174 answered: bears in a new level 4; Q-175 what else level 4 contains. Q-154 `door_wood`, `glass_door`, `turnstile` not yet in an ART spec.
- Q-181 answered 2026-09-28: labelled food boxes outside, a few more boxes inside; Q-194 answered 2026-09-29: the inside boxes are real, labelled food boxes too (foods may repeat the outside row). Q-182 answered 2026-09-28: the street runs under the moon door too (LAYOUT-040).
