---
id: FIX-018
date: 2026-09-26
type: cleanup
specs: [ART-DIRECTION, ART-CHARACTERS, ART-PIPELINE]
questions: []
---

## Problem

Three tests checked the same thing — style blocks copied verbatim into every prompt:
APIPE-010 (all `art/**/brief.md`, STYLE / CHARACTER SHEET STYLE + NEGATIVE suffix),
ADIR-004 (weaker: "contains", no NEGATIVE) and ACHAR-008 (subset: `art/characters/`).
Test rows in ART-DIRECTION and ART-PIPELINE were out of numeric order.

## Resolution

- APIPE-010 is the single test; ADIR-004 and ACHAR-008 kept as "Retired" rows (ids never reused).
- Test tables of ART-DIRECTION and ART-PIPELINE sorted by id.

## Changed files

- `specs/30-art/art-direction.md`, `specs/30-art/characters.md`, `specs/30-art/asset-pipeline.md`
