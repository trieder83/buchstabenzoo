---
id: FIX-027
date: 2026-09-26
type: cleanup
specs: []
questions: []
---

## Problem

`specs/INDEX.md` was stale after PoC M4 and the water spec: TECH-WATER missing, ART-ANIMALS
depends_on (ART-RIG) outdated, test counts (GAME-PLAYER, GAME-FEED, GAME-LEVEL-1,
ART-ENVIRONMENT, ART-ANIMALS …) and open questions (Q-018, Q-066, Q-067 answered; Q-064,
Q-065, Q-068 new) out of date.

## Resolution

Regenerated from frontmatter (24 specs, 197 active test cases). Structure checks passed:
frontmatter complete with `updated`, ids unique, files in their aspect folders, depends_on
ids exist, test ids match `test_prefix`, no duplicate test ids, every referenced `Q-###`
exists.

## Changed files

- `specs/INDEX.md`
