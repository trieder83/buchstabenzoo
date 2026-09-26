---
id: FIX-029
date: 2026-09-26
type: contradiction
specs: [GAME-LEVEL-2, GAME-LAYOUT, GAME-ANIMALS, GAME-RESCUE, GAME-PLAYER, ART-ENVIRONMENT]
questions: []
---

## Problem

GAME-LEVEL-2 `food_storage_2` cited "proposal Q-085" (woods / hippo pool) instead of Q-089
(food storage per level). The new questions Q-097 (affects GAME-RESCUE, GAME-PLAYER §5),
Q-098 and Q-099 (affect GAME-LAYOUT, ART-ENVIRONMENT) were not referenced by all affected
specs. GAME-ANIMALS listed ANIM-012 before ANIM-011.

## Resolution

Q-085 → Q-089 in GAME-LEVEL-2; Q-097 added to the open-question lists of GAME-ANIMALS,
GAME-RESCUE and GAME-PLAYER; Q-098/Q-099 added to GAME-LAYOUT and ART-ENVIRONMENT; ANIM test
rows put in order.

## Changed files

- `specs/10-gameplay/levels/level-2.md`
- `specs/10-gameplay/layout.md`
- `specs/10-gameplay/animals.md`
- `specs/10-gameplay/rescue-mission.md`
- `specs/10-gameplay/player.md`
- `specs/30-art/environment.md`
