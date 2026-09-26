---
id: FIX-010
date: 2026-09-26
type: missing-tests
specs: [GAME-RESCUE, GAME-ANIMALS, GAME-FEED, CONT-L10N, CONT-MATH]
questions: []
---

## Problem

Behaviour rules without test cases: GAME-RESCUE §4 (mission start / following before the
board is read), GAME-ANIMALS info board shows the riddle of the chosen hiding place,
GAME-FEED §4 "boxes never run out", CONT-L10N §5 default = device language, CONT-MATH §4 task
text per reading level. RESC-006 tested a 5 m resume distance that the rule did not state.

## Resolution

Added RESC-012, RESC-013, ANIM-005, FEED-006, L10N-005, MATH-006. GAME-RESCUE §6 now states
the 5 m resume distance already fixed by RESC-006.

## Changed files

- `specs/10-gameplay/rescue-mission.md`, `specs/10-gameplay/animals.md`,
  `specs/10-gameplay/feeding.md`, `specs/20-content/localization.md`, `specs/20-content/math.md`
