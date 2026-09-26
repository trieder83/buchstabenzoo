---
id: FIX-023
date: 2026-09-26
type: cleanup
specs: []
questions: []
---

## Problem

`specs/INDEX.md` was outdated: PROD-POC and GAME-MAP were missing, test counts
(GAME-LAYOUT, ART-ENVIRONMENT) and open questions (Q-052, Q-056, Q-057 answered;
Q-053–Q-055, Q-059–Q-061 new) were stale.

## Resolution

Regenerated from frontmatter (23 specs, 161 active test cases). Structure checks passed:
frontmatter complete, ids unique, files in their aspect folders, depends_on ids exist, no
dependency cycles, test ids match `test_prefix`, no duplicate test ids, every referenced
`Q-###` exists.

## Changed files

- `specs/INDEX.md`
