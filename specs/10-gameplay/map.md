---
id: GAME-MAP
title: Zoo map with explored areas
aspect: gameplay
module: map
status: draft
depends_on: [GAME-PLAYER, GAME-LAYOUT, GAME-RESCUE]
test_prefix: MAP
updated: 2026-09-26
---

# Zoo map with explored areas

## Goal

The child can open a map of the zoo to orient themselves — but the map only shows what
they have **already explored** (user decision 2026-09-26). Unexplored parts stay hidden,
so exploring stays exciting and the riddles still have to be solved by reading.

## Behaviour

1. **Open / close.** A large map button (icon, no text) is always visible in the HUD;
   tapping it (desktop: `M`) opens the map full-screen and pauses player movement. Tapping
   the close button or the map button again returns to the game.
2. **Explored cells.** The level grid (GAME-LAYOUT, 1 m cells) has an `explored` flag per
   cell. A cell becomes explored when it is within the **reveal radius** of the player
   (proposal: 10 m, Q-053). Explored cells never become unexplored again.
3. **Fog.** Unexplored cells are drawn as a soft, friendly "cloud/paper" fog in the comic
   style — no hint of what lies beneath (no outlines, no icons, no colours of the terrain).
4. **Content shown on explored cells:** the top-down map art of the level (the approved
   `top_down.png` of `env_level<N>_overview`, drawn in the comic style) plus icons:
   - the player (always shown, with viewing direction),
   - enclosures with the animal silhouette and their state: empty or animal home,
   - the food storage, the entrance and landmarks,
   - barriers of the current level (e.g. fallen tree icon).
5. **What the map never shows:** escaped animals and hiding places, until the player has
   found the animal (then a small "found here" marker is allowed — Q-054). The map must not
   make the location riddle unnecessary.
6. **Locked levels.** Cells beyond barriers of locked levels stay fog, even if they are
   within the reveal radius (the barrier blocks the reveal).
7. **Text on the map.** No text required; icons only. Optional labels (enclosure names) use
   the current language and reading level (CONT-L10N, CONT-READING).
8. **Saving.** The explored flags are part of the save state (TECH-ARCH `zoo-core`) and
   survive restarts.
9. **Map board.** The map board at the entrance (`M` in GAME-LEVEL-1) opens the same map;
   whether it reveals the level layout at once is open (Q-055; recommendation: no — it only
   shows what is explored, plus the entrance area).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| MAP-001 | Given a new game, then only the cells within the reveal radius of the spawn are explored. | unit |
| MAP-002 | Given the player walks along a path, then every cell within the reveal radius of any visited position is explored. | unit |
| MAP-003 | Given a cell was explored, when the player moves away, then it stays explored. | unit |
| MAP-004 | Given a cell of a locked level within the reveal radius, then it stays unexplored. | unit |
| MAP-005 | Given an escaped animal on an explored cell that the player has not found yet, then the map shows no icon for it. | unit |
| MAP-006 | Given the map is open, then player movement input is ignored. | unit |
| MAP-007 | Given a saved game is loaded, then the explored cells equal those at save time. | unit |
| MAP-008 | Given the map is opened on a 1080×2340 viewport, then the whole explored area and the player icon are visible and unexplored areas show fog only. | e2e |
| MAP-009 | Given an enclosure whose animal is home, then its map icon shows the "home" state. | unit |

## Open questions

- Q-053 Reveal radius (fixed 10 m, or what was on screen?).
- Q-054 Show a marker where a found animal was?
- Q-055 Does the entrance map board reveal the level layout?
