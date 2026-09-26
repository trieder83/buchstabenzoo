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
| `path` | main path, side path, bridge | yes |
| `enclosure` | one per animal (see GAME-ANIMALS) | only via its gate when leading an animal |
| `building` | entrance, food storage, kiosk, toilet, zookeeper house | entrance only if interactive |
| `landmark` | pirate ship, fountain, map board | around it / on it if specified |
| `barrier` | road block, stones, fallen tree, construction fence, closed gate | no |
| `boundary` | outer zoo wall, hedge, water | no, never removed |
| `decoration` | trees, benches, bushes | no (collision) |

## Levels

One file per level in `specs/10-gameplay/levels/<level_id>.md` containing:
area list, enclosures and buildings (with grid rectangles), barriers with their unlock
condition, spawn point, and a top-down ASCII or SVG map.

| Level id | Area | Status |
|---|---|---|
| (to be defined by the agent after Q-023) | | |

## Behaviour

1. The player can never leave the union of walkable cells of all unlocked levels.
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

## Open questions

- Q-006, Q-017, Q-022, Q-023.
