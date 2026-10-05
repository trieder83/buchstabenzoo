---
id: GAME-MAP
title: Zoo overview map
aspect: gameplay
module: map
status: draft
depends_on: [GAME-PLAYER, GAME-LAYOUT, GAME-RESCUE]
test_prefix: MAP
updated: 2026-10-04
---

# Zoo overview map

## Goal

The child can open a **big map of the whole zoo** to get the overview (user request
2026-10-04, Q-363): where the levels are, which are open and how far she is, where she
stands. The map never gives away where an animal hides, so the location riddles still have
to be read (MAP-005). This overview replaces the earlier explored-fog map for now (see
"Deferred: explored fog").

## Behaviour

1. **Open / close.** A large 🗺️ button (`#map-btn`, icon only, >= 64 px, no text) is the
   first row of the **settings menu**; tapping it closes the settings and opens the map
   full-screen (`#overview-map`). Close: the ✖ button (`#map-close`, >= 64 px), `Esc`, or `M`
   (desktop; `M` also opens it). While the map is open the game is paused: movement input,
   the interact key and the simulation do nothing (MAP-006).
2. **Data from the level data.** The map is built in `zoo-core` (`overview`), not from
   screenshots: the host only draws it (2D canvas) and adds the icons and labels.
3. **Drawn:**
   - the level parts (rectangle each), with the **level state**: *open* (colour) with its
     progress `home/total` and a ✔ when *solved* (every animal of the level home; pairs count
     once, RESC-028); *locked* (entry barrier closed): grey with 🔒; night levels with 🌙;
   - paths (plaza, main, side, jetty, bridge), water (pond, river, stream, fountain, pool),
     trees, buildings (entrance, food storage, zookeeper house, kiosk, ...), enclosures
     (outline + the animal icon, ✔ when the animal or pair is home), barriers (gates, road
     block, fallen tree) and the zoo wall;
   - the **player**: a marker with her facing direction, always visible and drawn on top.
4. **Next mark.** A subtle pulsing arrow shows the **level or place to go next** (the
   compass target, GAME-HINT): a point for a board, the food storage, a gate, the bed or the
   moon door; for an animal's hiding area **only the level** (the part's label), never the
   spot. Nothing is drawn when there is no target.
5. **What the map never shows:** escaped animals and hiding places, in any state. Animal
   icons appear only at their own enclosure.
6. **Text.** Icons first; the few labels (title, level names, "locked") come from Fluent keys
   (`map-*`, de + en) and follow the language at once.
7. **Fit.** The whole zoo is scaled to fit the viewport, aspect kept, on a 780x360 landscape
   phone and a 360x780 portrait phone; the ✖ button sits in a corner outside the map drawing. No scrolling or zooming is needed.
8. **Saving.** The overview keeps no state of its own (everything derives from the game
   state), so there is nothing to save.
9. **Map board.** The map board at the entrance (`M`, GAME-LEVEL-1) opens the same overview.

## Deferred: explored fog

The earlier idea (map shows only explored cells, Q-053 reveal radius, Q-054 found marker,
Q-055 entrance board) stays a possible later option. MAP-001…004, MAP-007 and MAP-008 are
`deferred`: no code, no test until the user asks for fog (Q-363).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| MAP-001 | (deferred, fog) Given a new game, then only the cells within the reveal radius of the spawn are explored. | deferred |
| MAP-002 | (deferred, fog) Given the player walks along a path, then every cell within the reveal radius of any visited position is explored. | deferred |
| MAP-003 | (deferred, fog) Given a cell was explored, when the player moves away, then it stays explored. | deferred |
| MAP-004 | (deferred, fog) Given a cell of a locked level within the reveal radius, then it stays unexplored. | deferred |
| MAP-005 | Given escaped animals, then the overview data contains no hiding place id and no position of an escaped animal; icons only at enclosures. | unit |
| MAP-006 | Given the map is open, when a movement key is held, then the player does not move and the game does not advance; after closing it moves again. | e2e |
| MAP-007 | (deferred, fog) Given a saved game is loaded, then the explored cells equal those at save time. | deferred |
| MAP-008 | (deferred, fog) Given the map is opened on a 1080x2340 viewport, then unexplored areas show fog only. | deferred |
| MAP-009 | Given an enclosure whose animal (pair) is home, then its overview entry has `home = true`, else false. | unit |
| MAP-010 | Given a new game with the night zoo, then level_1 is open 0/3, level_2 and level_3 locked, night_1 locked and a night level. | unit |
| MAP-011 | Given all animals of level 1 home, then level_1 is solved 3/3. | unit |
| MAP-012 | Given the entry barrier of level_2 opened, then level_2 is open with 4 animals, 0 home. | unit |
| MAP-013 | Given the player stands at the spawn, then the overview holds her position, facing and level part. | unit |
| MAP-014 | Given a game, then the overview JSON has no `animals` list and no hiding place id. | unit |
| MAP-015 | Given the next target is an animal's hiding area, then the next mark has the level only (no point); other targets have a point. | unit |
| MAP-016 | Given the overview, then it holds paths, water, buildings (entrance, food storage), barriers and the zoo wall, each inside a level part. | unit |
| MAP-017 | Given the settings menu is open, then a 🗺️ button `map-btn` (>= 64 px) is its first row; tapping it opens `#overview-map`, closes the settings and shows the player marker. | e2e |
| MAP-018 | Given the map is open, then locked levels show 🔒 and are grey, the open level shows its progress, a solved level shows ✔. | e2e |
| MAP-019 | Given the map is open, when ✖, `Esc` or `M` is used, then it closes; `M` opens it. | e2e |
| MAP-020 | Given 780x360 and 360x780 viewports, then the opened map and its close button fit completely inside the viewport and the close button is >= 64 px. | e2e |
| MAP-021 | Given the language is switched, then the map title follows (Fluent `map-title`). | e2e |

## Open questions

- Q-363 Overview map instead of the explored-fog map (this spec).
- Q-053 Reveal radius, Q-054 found marker, Q-055 entrance board (only for the deferred fog).
