---
id: FIX-019
date: 2026-09-26
type: cleanup
specs: []
questions: []
---

## Problem

`specs/INDEX.md` was outdated: GAME-LEVEL-1 missing, test counts and open questions stale
(Q-010, Q-046, Q-049 answered; Q-050–Q-052 new).

## Resolution

Regenerated from frontmatter (21 specs, 137 active test cases; retired rows not counted).
Structure checks passed: frontmatter complete, ids unique, depends_on ids exist, no
dependency cycles, test ids match `test_prefix`, no duplicate test ids, every referenced
`Q-###` exists.

## Changed files

- `specs/INDEX.md`
