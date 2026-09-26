---
id: FIX-040
date: 2026-09-26
type: missing-tests
specs: [TECH-WATER, GAME-AMBIENT]
questions: []
---

## Problem

M6 behaviour rules without a test case: TECH-WATER behaviour 6 (obstacle foam list),
13 (ripples of ambient animals), 14 (fountain still basin), 15 (water hides what is below);
GAME-AMBIENT rules 1/7 (clip set — covered only by the untagged test
`amb_clip_durations_match_the_anim_table`), 4 (look/readability), 5 (wake and dip rings),
11 (no collision, no prompt).

## Resolution

Added WATER-013 (ripple list, unit), WATER-014 (fountain basin, unit), WATER-015 (hidden
below the surface, manual), WATER-016 (obstacle list, unit), AMB-010 (clips and durations,
unit — the untagged test should carry this ID), AMB-011 (no collision / prompt, unit),
AMB-012 (wake and dip ring reach the shader, unit), AMB-013 (readability review, manual).
The code tests should carry these IDs.

## Changed files

- `specs/40-tech/water-rendering.md`
- `specs/10-gameplay/ambient.md`
