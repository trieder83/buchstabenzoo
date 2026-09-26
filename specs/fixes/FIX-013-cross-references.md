---
id: FIX-013
date: 2026-09-26
type: gap
specs: [TECH-ARCH, GAME-PLAYER, GAME-FEED, ART-RIG, CONT-L10N, CONT-READING, CONT-MATH, GAME-ANIMALS, GAME-RESCUE]
questions: []
---

## Problem

- TECH-ARCH's `zoo-core` responsibilities missed rescue missions, math levels/tasks and the
  animation clip state/events that ART-RIG §4.8 assigns to `zoo-core`.
- Several specs were affected by open questions without listing them (e.g. GAME-PLAYER
  PLAY-005 needs Q-024; CONT-L10N had no "Open questions" section).
- Q-025 (carrying) still assumed small animals are carried in general, contradicting the
  Q-003 answer (animals follow).

## Resolution

- TECH-ARCH crate table extended.
- Open-question references added to GAME-PLAYER, GAME-FEED, ART-RIG, CONT-L10N,
  CONT-READING, CONT-MATH, GAME-ANIMALS, GAME-RESCUE.
- Q-025 annotated: only food, key, monkey baby and possibly the goldfish bucket are carried.
  Q-034 annotated with the "food portions vs. one food at a time" conflict.

## Changed files

- `specs/40-tech/architecture.md`, `specs/10-gameplay/*.md`, `specs/30-art/character-rig-and-animation.md`,
  `specs/20-content/*.md`, `specs/open-questions.md`
