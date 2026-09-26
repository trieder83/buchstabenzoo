---
id: FIX-038
date: 2026-09-26
type: naming
specs: [PROD-GLOSSARY]
questions: []
---

## Problem

The "† German term is a proposal" footnote sat in the middle of the glossary table again (the
FIX-022 regression): every row after it (nightfall … golf_cart) no longer rendered as a table.
`water_field` said "baked by the renderer" (it is baked by the zoo-core level assembly, one
field for the joined zoo). The M6 terms `flow`, `scenery` and `ambient_animal` were used in
specs but missing from the glossary.

## Resolution

Moved the footnote below the table, corrected `water_field`, added `flow`, `scenery` and
`ambient_animal` (German terms marked † as proposals, Q-045).

## Changed files

- `specs/glossary.md`
