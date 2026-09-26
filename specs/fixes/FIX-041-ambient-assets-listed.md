---
id: FIX-041
date: 2026-09-26
type: gap
specs: [ART-ANIMALS, GAME-AMBIENT]
questions: [Q-121, Q-122]
---

## Problem

The ambient models `duck`, `duckling`, `frog` (`assets/models/animals/`, clips in
`animal_anims.toml`) and the built-in `butterfly` mesh were referenced by GAME-AMBIENT and
TECH-WATER but listed in no ART-* spec. GAME-AMBIENT rules 2 and 7 disagree with its own
Implementation section.

## Resolution

Added an "Ambient animals" table to ART-ANIMALS (script, rig, clips, size, status — as
decided in GAME-AMBIENT and built by the scripts). Recorded Q-121 (rule 2/7 vs.
implementation) and Q-122 (manifest entries, concept gate, applicable AANI tests, leftover
static `duck` / `frog` props) instead of deciding them; linked both from the affected specs.

## Changed files

- `specs/30-art/animals.md`
- `specs/10-gameplay/ambient.md`
- `specs/open-questions.md`
