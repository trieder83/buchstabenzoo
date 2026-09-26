---
id: GAME-LAYOUT
title: Zoo layout and level boundaries
aspect: gameplay
module: layout
status: draft
depends_on: [GAME-WORLD, ART-ENVIRONMENT]
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

- Grid of 1 m cells, origin at the zoo entrance gate, +X east, +Z north (Y-up in 3D).
- Every element has an id, a grid rectangle (`x, z, width, depth`) and a level.

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

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| LAYOUT-001 | Given level N's layout data, then every enclosure/building/landmark is reachable from the spawn by walkable cells (flood fill). | unit |
| LAYOUT-002 | Given level N is active, then no walkable cell of a locked level is reachable (barriers seal the scope). | unit |
| LAYOUT-003 | Given all layout data, then no two solid elements overlap. | unit |
| LAYOUT-004 | Given a barrier's unlock condition is met, then its cells become walkable and LAYOUT-001 holds for the next level. | unit |
| LAYOUT-005 | Given every level spec, then its elements match the layout data file (ids and rectangles). | unit |
| LAYOUT-006 | Given a level's layout data, then every walkable cell covered by a `path` element has surface `path` and every other walkable cell has surface `grass` (Q-046). | unit |

## Open questions

- Q-006, Q-017, Q-022, Q-023.
- Q-044 `hiding_place` element type and `blocks_view` (proposal above). Q-046 walkable ground (answered). Q-049 high-angle camera (answered — sight test is a screen test; FOV axis Q-052).
