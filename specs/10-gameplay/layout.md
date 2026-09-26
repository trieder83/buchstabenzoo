---
id: GAME-LAYOUT
title: Zoo layout and level boundaries
aspect: gameplay
module: layout
status: draft
depends_on: [GAME-WORLD, ART-ENVIRONMENT, GAME-PLAYER]
test_prefix: LAYOUT
updated: 2026-09-26
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
| `building` | entrance, food storage, kiosk, toilet, zookeeper house | entrance only if interactive |
| `landmark` | pirate ship, fountain, map board | around it / on it if specified |
| `barrier` | road block, stones, fallen tree, construction fence, closed gate | no |
| `boundary` | outer zoo wall, hedge, water | no, never removed |
| `decoration` | trees, benches, bushes, tall hedges, info boards | no (collision) |
| `hiding_place` *(proposal, Q-044)* | `loc_*` of CONT-MISSIONS: an overlay rectangle with `animal`, `animal_spot` (cell where the animal waits) and `features` (riddle details it must show); may overlap a landmark, path or building | not solid itself — the cells keep the walkability of what lies underneath |

**Walkable ground (decided, Q-046, user 2026-09-26):** every cell of a level that is not
covered by a solid element is walkable. Cells have a `surface`: `path` (cells of `path`
elements, incl. bridges, jetties, cave floors) or `grass` (all other walkable cells). The
player walks on grass **slower** than on paths (speed factor in GAME-PLAYER, proposal
0.7 ×). Because grass is walkable, all border cells of a level must be solid.

**Proposals used by the level files (not yet decided):**
- *Sight blocking data (Q-044):* solid elements may set `blocks_view = true` (and `height_m`,
  used for mockups/greybox). Since the high-angle camera is decided (Q-049, GAME-PLAYER §2),
  "an animal's hiding place cannot be seen from its own enclosure" is checked as a **screen
  test** per level (hiding place off-screen while the player stands at its info board or
  enclosure gate, e.g. LAYOUT-L1-006), not as an eye-level line of sight.

## Levels

One file per level in `specs/10-gameplay/levels/level-<N>.md` (spec id `GAME-LEVEL-<N>`, level id
`level_<N>`, layout data `assets/levels/level-<N>.toml`) containing:
area list, enclosures and buildings (with grid rectangles), barriers with their unlock
condition, spawn point, and a top-down ASCII or SVG map.

| Level id | Area | Status |
|---|---|---|
| `level_1` | [levels/level-1.md](levels/level-1.md) (GAME-LEVEL-1): entrance, food storage, zebra, hippo and panda enclosures and their hiding places `loc_river`, `loc_pond`, `loc_cave` | draft — proposal pending Q-023 |
| later levels | to be defined after Q-023 | — |

## Behaviour

1. The player can never leave the union of walkable cells (path and grass) of all unlocked levels.
2. Each barrier belongs to exactly one level transition and has one unlock condition (Q-022).
3. When a barrier is removed, a short in-world animation plays (e.g. a zookeeper rolls the
   stone away); the removed area becomes walkable.
4. Every enclosure, building and landmark is reachable by path from the spawn point within
   its level.
5. Level and world coordinates are converted only by `world = (x, 0, −z)` (Q-056); models
   are never mirrored and are oriented by rotations about +Y only.
6. Every straight run of fence, hedge or wall is filled by 2 m segments plus, for odd
   lengths, one 1 m segment at the end; turns use corner pieces (Q-057).

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

## Open questions

- Q-006, Q-017, Q-022, Q-023.
- Q-056 answered: coordinate spaces (level x east / z north; world = (x, 0, −z)).
- Q-057 answered: 1 m segment variants and the fill rule. Q-059 band joins, Q-060 enclosure fence and band placement (proposals), Q-061 front direction of props (open).
- Q-044 `hiding_place` element type and `blocks_view` (proposal above). Q-046 walkable ground (answered). Q-049 high-angle camera (answered — sight test is a screen test; FOV axis Q-052).
