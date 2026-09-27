---
id: FIX-049
date: 2026-09-27
type: contradiction
specs: [GAME-LAYOUT]
questions: []
---

## Problem

GAME-LAYOUT got `GAME-NIGHT` in `depends_on`, while GAME-NIGHT depends on GAME-LAYOUT: two
cycles (GAME-LAYOUT ↔ GAME-NIGHT and GAME-LAYOUT → GAME-NIGHT → GAME-SAVE → GAME-MAP →
GAME-LAYOUT).

## Resolution

Direction rule of FIX-005: data/layout specs do not depend on flow specs. Removed `GAME-NIGHT`
from GAME-LAYOUT's `depends_on`; its "Moon door and night levels" section keeps the text
cross-references. Checked by script: no cycles, all ids exist.

## Changed files

- `specs/10-gameplay/layout.md`
