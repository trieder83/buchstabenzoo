---
id: FIX-014
date: 2026-09-26
type: gap
specs: []
questions: []
---

## Problem

`specs/INDEX.md` did not exist.

## Resolution

Generated from frontmatter (20 specs, tables per aspect, test and open-question counts,
status summary). Structure checks run at the same time: all frontmatter complete, ids
unique, files in their aspect folders, test ids match `test_prefix`, no duplicate test ids,
every `Q-###` reference exists.

## Changed files

- added `specs/INDEX.md`
