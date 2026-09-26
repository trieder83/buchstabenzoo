---
id: FIX-035
date: 2026-09-26
type: naming
specs: [PROD-GLOSSARY]
questions: []
---

## Problem

M5b specs use new concepts that were not in the glossary: the bowl's second carry slot
("pocket"), locked / unlocked levels of the joined zoo, enterable buildings (`interior`,
`door`), render regions.

## Resolution

Added `pocket`, `unlocked_level` / `locked_level`, `enterable_building`, `render_region`
(German terms marked † as proposals, Q-045).

## Changed files

- `specs/glossary.md`
