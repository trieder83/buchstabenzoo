---
id: FIX-003
date: 2026-09-26
type: cleanup
specs: [ART-DIRECTION, ART-PIPELINE, ART-CHARACTERS, GAME-WORLD]
questions: []
---

## Problem

Reference images lived in `images/` (e.g. `z.webp`) and concept briefs in `assets/concept/`,
mixing concept art with game assets and making review hard.

## Resolution

- Reference images moved to `art/reference/` and renamed (`ref-player-style.jpg`,
  `ref-enclosure-row.jpg`, `ref-enclosure-buildings.jpg`, `ref-zoo-layout.webp`).
- Concept briefs moved to `art/<kind>/<asset_id>/`; review page `art/index.html` with data in
  `art/catalog.js` (ART-PIPELINE stage 1, APIPE-007/008).
- All spec references updated; verified that no spec still points to `images/` or
  `assets/concept/`. (FIX-001's mention of `z.webp` is history and stays.)

## Changed files

- `specs/30-art/art-direction.md`, `specs/30-art/asset-pipeline.md`, `specs/30-art/characters.md`,
  `specs/10-gameplay/world.md`, `CLAUDE.md`, `art/**`
