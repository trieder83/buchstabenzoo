---
id: FIX-073
date: 2026-09-30
type: fix
specs: [GAME-FAMILY, ART-ANIMALS, GAME-LAYOUT]
questions: [Q-106, Q-203, Q-204]
---

# FIX-073 Family models exist, pair flag on

## Problem

`zebra_female`, `zebra_foal`, `koala_female`, `koala_joey` were approved but not modelled;
`pair` was off (Q-106); the manifest listed `climb` (no `walk`) for the koala family models.

## Resolution

- Models scripted (`tools/blender/animals/*.py`, ART-ANIMALS "Family models"), `animal_anims.toml`
  and `check_animal.py` SIZES extended; manifest clips of the koala family = adult's.
- `pair = true` for `enc_zebra` and `enc_koala`; tests adapted (RESC-001/008/014, save, zoo_game).
- Bug found by the pair data: a save restored both missions of a pair onto member 0 (fixed).
- New questions Q-203 (female detail), Q-204 (baby behaviour).

## Changed files
specs/10-gameplay/families.md, levels/level-2.md, specs/30-art/animals.md, specs/open-questions.md
