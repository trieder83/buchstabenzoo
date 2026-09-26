---
id: FIX-033
date: 2026-09-26
type: contradiction
specs: [GAME-LEVEL-1, GAME-FAMILY, PROD-POC, GAME-RESCUE, GAME-ANIMALS]
questions: []
---

## Problem

After M5b some statements disagreed with the M5b changes: GAME-LEVEL-1 still named the host
key `zoo.picks.level-1` (now `zoo.picks`, GAME-SAVE §8); GAME-FAMILY's M5a status said the
pair logic is not implemented and FAM-001…006 stay open, while the M5b section implements
rules 1–2 behind `pair = true` (plus a typo "the second one wander cell away"); PROD-POC gave
≈ 56 draw calls inside level 2/3, GAME-LAYOUT ≈ 55–60; RESC-014 still said "every animal of
level 1" although picks are now per level; GAME-ANIMALS said every escaped animal wanders on
the ground (ANIM-008) without the perch exception of GAME-RESCUE §12.

## Resolution

Host key updated in GAME-LEVEL-1; M5a paragraph of GAME-FAMILY marked superseded for rules 1–2
(FAM-003…006 still open) and typo fixed; PROD-POC aligned to ≈ 55–60; RESC-014 now covers
every level; GAME-ANIMALS got a cross-reference that perched animals do not wander (proposal
Q-094, RESC-026).

## Changed files

- `specs/10-gameplay/levels/level-1.md` (hiding-place picking rule only)
- `specs/10-gameplay/families.md`
- `specs/00-product/poc.md`
- `specs/10-gameplay/rescue-mission.md`
- `specs/10-gameplay/animals.md`
