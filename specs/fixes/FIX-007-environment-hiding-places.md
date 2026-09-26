---
id: FIX-007
date: 2026-09-26
type: gap
specs: [ART-ENVIRONMENT]
questions: []
---

## Problem

ART-ENVIRONMENT had no mockups for the giraffe, lion and snow fox enclosures, did not cover
the 10 hiding places (`loc_*`) of CONT-MISSIONS, and the `info_board` (every enclosure has
one, GAME-ANIMALS) was not a listed prop.

## Resolution

Added `env_giraffe`, `env_lion`, `env_snow_fox`; added a "Hiding places" table with the
details each riddle relies on (copied from CONT-MISSIONS' designer column) and test AENV-004
(every hiding place appears in a mockup layout); added `info_board` to the modular props.
Assignment of places to mockups and their layout representation left to Q-044.

## Changed files

- `specs/30-art/environment.md`
