---
id: FIX-036
date: 2026-09-26
type: cleanup
specs: []
questions: []
---

## Problem

`specs/INDEX.md` was stale after M5b and the garden/ambient work: GAME-AMBIENT and
GAME-GARDEN were missing, test counts and open questions (Q-102…Q-106) out of date.

## Resolution

Regenerated from frontmatter (31 specs, 344 active test cases). Structure checks passed:
frontmatter complete, ids unique, files in their aspect folders, depends_on ids exist, no
duplicate test ids, every referenced `Q-###` exists, every test ID used in `crates/` and
`web/` exists in a spec.

## Changed files

- `specs/INDEX.md`
