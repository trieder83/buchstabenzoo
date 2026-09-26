---
id: FIX-037
date: 2026-09-26
type: naming
specs: [ART-ENVIRONMENT]
questions: []
---

## Problem

ART-ENVIRONMENT still listed the river prop as `water_tile_flowing` ("moving texture") and the
`duck` / `frog` as static props. Since M3/M6 the river is the `water_river_*` tiles of
`kit_water`, animated by the water shader (TECH-WATER, no texture), and ducks/frogs are
animated ambient animals (GAME-AMBIENT).

## Resolution

Renamed to the `water_river_*` tiles (noting the former name, which the concept briefs under
`art/environment/*/brief.md` and `layout.md` still use) and marked `duck` / `frog` as ambient
animals listed in ART-ANIMALS.

## Changed files

- `specs/30-art/environment.md`
