---
id: FIX-053
date: 2026-09-27
type: missing-tests
specs: [GAME-NIGHT, GAME-AMBIENT, GAME-LEVEL-2, GAME-LEVEL-3, GAME-LEVEL-NIGHT-1, ART-ANIMALS]
questions: []
---

## Problem

GAME-NIGHT rule 5 / engine section (day animals lie down and do not wander at night, night
animals wander twice as much) had no test case; the eyeshine radius (2.5 m, used by
GAME-LEVEL-NIGHT-1 and `LANTERN_RADIUS_M`) was not stated in GAME-NIGHT; GAME-NIGHT did not
list Q-091 as answered; GAME-AMBIENT was changed (rule 9) without bumping `updated`.

## Resolution

Added **NIGHT-015**; stated the 2.5 m eyeshine radius in the engine section; listed Q-091
(answered) and the new Q-141…Q-143 in GAME-NIGHT and the affected specs; bumped GAME-AMBIENT
`updated`.

## Changed files

- `specs/10-gameplay/night.md`, `specs/10-gameplay/ambient.md`,
  `specs/10-gameplay/levels/level-2.md`, `specs/10-gameplay/levels/level-3.md`,
  `specs/10-gameplay/levels/night-1.md`, `specs/30-art/animals.md`, `specs/open-questions.md`
