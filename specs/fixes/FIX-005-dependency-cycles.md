---
id: FIX-005
date: 2026-09-26
type: contradiction
specs: [PROD-VISION, GAME-ANIMALS, CONT-MATH, GAME-QUESTS, ART-ENVIRONMENT]
questions: []
---

## Problem

`depends_on` had three cycles: PROD-VISION ↔ GAME-RESCUE, GAME-ANIMALS ↔ GAME-RESCUE,
CONT-MATH ↔ GAME-RESCUE.

## Resolution

Direction rule applied: the vision depends on nothing; data/level specs (GAME-ANIMALS,
CONT-MATH, CONT-READING) do not depend on the flow spec (GAME-RESCUE); flow and mission
content depend on them. Removed GAME-RESCUE from PROD-VISION, GAME-ANIMALS and CONT-MATH;
CONT-MATH now depends on CONT-READING (task texts follow the reading level). Added missing
real dependencies: GAME-QUESTS → GAME-RESCUE, ART-ENVIRONMENT → CONT-MISSIONS. Checked by
script: all ids exist, no cycles.

## Changed files

- `specs/00-product/vision.md`, `specs/10-gameplay/animals.md`, `specs/20-content/math.md`,
  `specs/10-gameplay/quests/overview.md`, `specs/30-art/environment.md`
