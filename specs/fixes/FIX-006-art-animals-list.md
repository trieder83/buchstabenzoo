---
id: FIX-006
date: 2026-09-26
type: gap
specs: [ART-ANIMALS]
questions: []
---

## Problem

ART-ANIMALS listed the 7 animals of the old concept, while GAME-ANIMALS, CONT-MISSIONS and
`art/catalog.js` use the 10-animal candidate list (Q-002). AANI-001 would fail for giraffe,
lion and snow fox.

## Resolution

Added `giraffe`, `lion`, `snow_fox` with the common animation set of ART-ANIMALS §1 (`idle`,
`walk`, `eat`, `happy`); marked the list as provisional until Q-043; the monkey baby variant
marked as depending on Q-040. Missing clips (hiding-place idles, reactions, koala/goldfish
locomotion while following) are not invented — raised as Q-043.

## Changed files

- `specs/30-art/animals.md`
