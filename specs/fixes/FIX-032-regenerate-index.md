---
id: FIX-032
date: 2026-09-26
type: cleanup
specs: []
questions: []
---

## Problem

`specs/INDEX.md` was stale: GAME-FAMILY, GAME-NIGHT, GAME-SAVE, GAME-LEVEL-2 and GAME-LEVEL-3
were missing; test counts and open questions (Q-069–Q-099) out of date.

## Resolution

Regenerated from frontmatter after M5a (29 specs, 313 active test cases). Structure checks
passed: frontmatter complete, ids unique, files in their aspect folders, depends_on ids exist,
test ids match `test_prefix`, no duplicate test ids, every referenced `Q-###` exists.

## Changed files

- `specs/INDEX.md`
