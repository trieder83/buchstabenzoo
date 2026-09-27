---
id: FIX-063
date: 2026-09-27
type: cleanup
specs: []
questions: []
---

## Problem

`INDEX.md` was out of date:
- New tests: NIGHT-016…018, ARCH-006…008, LAYOUT-032, LAYOUT-L2-018 and others since FIX-055.
- Answered questions: Q-128, Q-132 and others, plus Q-141, Q-142, Q-144 and Q-146.
- New questions: Q-147…Q-154.

## Resolution

Regenerated `INDEX.md` from frontmatter:
- 37 specs: draft 35, implemented 2.
- 462 active test cases.
- Questions: answered 61, open 84, partly answered 7, proposed 2.

No frontmatter errors, duplicate spec ids, duplicate test ids, prefix mismatches, unknown
dependencies, unknown Q-### references or dependency cycles.

## Changed files

- `specs/INDEX.md`
