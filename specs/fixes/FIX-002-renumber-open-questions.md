---
id: FIX-002
date: 2026-09-26
type: naming
specs: [PROD-VISION, GAME-RESCUE, GAME-ANIMALS, GAME-FEED, CONT-MATH, CONT-MISSIONS]
questions: []
---

## Problem

The main session and the `character-artist` agent both used Q-024 … Q-029 on the same day
for different questions (gameplay/content vs. rig/characters). Numbers must never be reused.

## Resolution

The rig/character questions kept Q-024 … Q-029 (referenced by ART-RIG, ART-CHARACTERS). The
gameplay/content questions (herd size, end of game, distractors, food storage lock, math,
reading grades 4–5, goldfish bucket, riddle review) were renumbered to Q-030 … Q-037 and
all references updated. Verified: every `Q-###` referenced in `specs/` exists in
`open-questions.md` and means the same thing there.

## Changed files

- `specs/open-questions.md`, `specs/00-product/vision.md`, `specs/10-gameplay/rescue-mission.md`,
  `specs/10-gameplay/animals.md`, `specs/10-gameplay/feeding.md`, `specs/20-content/math.md`,
  `specs/20-content/missions/start-missions.md`
