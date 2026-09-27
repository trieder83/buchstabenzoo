---
id: FIX-060
date: 2026-09-27
type: missing-tests
specs: [GAME-LAYOUT]
questions: [Q-150]
---

## Problem

The new GAME-LAYOUT rule "Doors are never blocked" (user request 2026-09-27) had no test
case. LAYOUT-031 covers only the gate/door models, and LAYOUT-L2-018 covers only `bed_l2`.

## Resolution

Added LAYOUT-032. For every gate/door opening, no prop, board, lamp, item or furniture
footprint may lie in the opening or in the ≥ 1 m walkway in front of it. A sign that belongs
to a door must be beside it or above the opening. The only exception is the food-box rows in
front of the non-enterable food storage doors, until Q-150 is decided.

## Changed files

- `specs/10-gameplay/layout.md`
