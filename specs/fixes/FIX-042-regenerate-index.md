---
id: FIX-042
date: 2026-09-26
type: cleanup
specs: []
questions: []
---

## Problem

`specs/INDEX.md` was stale after M6: TECH-WATER and GAME-AMBIENT now `implemented`,
TECH-WATER depends on GAME-LAYOUT, new test cases (LAYOUT-026, AMB-009, FIX-040 rows),
Q-068 answered, new Q-107/108/121/122 references.

## Resolution

Regenerated from frontmatter (33 specs, 381 active test cases). Structure checks passed:
frontmatter complete, ids unique, files in their aspect folders, depends_on ids exist, no
dependency cycles, no duplicate test ids, every referenced `Q-###` exists, every test ID
used in `crates/`, `web/` and `tools/` exists in a spec.

## Changed files

- `specs/INDEX.md`
