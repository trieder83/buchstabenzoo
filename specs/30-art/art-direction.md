---
id: ART-DIRECTION
title: Art direction
aspect: art
module: art-direction
status: draft
depends_on: []
test_prefix: ADIR
updated: 2026-09-26
---

# Art direction

## Goal

A friendly, bright, readable 3D look for 4–9 year olds that runs smoothly on phones.

## References

| Image | What we take from it |
|---|---|
| `art/reference/ref-player-style.jpg` | Player character style, blocky/voxel humans and animals, warm daylight. |
| `art/reference/ref-enclosure-row.jpg` | Enclosure row with **signs showing name + silhouette**, wooden fences, grass field. |
| `art/reference/ref-enclosure-buildings.jpg` | Distinct enclosure buildings per animal (wooden hut, stone arch, glass house), large readable signs. |
| `art/reference/ref-zoo-layout.webp` | Zoo layout with paths, kiosks and props; purpose still open (Q-010). |

## Behaviour

1. Blocky, voxel-inspired shapes with flat colour areas; no realistic textures.
2. Bright daylight, soft shadows, blue sky with blocky clouds.
3. **Signs and labels are gameplay** — text on signs, food boxes and riddles must be
   legible at the default camera distance on a 6" phone (cap height ≥ 3 % of screen height
   when the player stands in front of it).
4. Each enclosure has a distinct silhouette/material so children can recognise it without
   reading (important for `kiga`).
5. Colour palette is defined once (`art/palette.png`) and reused by all assets.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| ADIR-001 | Given the player stands at the default interaction distance in front of an enclosure sign on a 1080×2340 viewport, then a screenshot shows the sign text with cap height ≥ 3 % of the viewport height. | e2e |
| ADIR-002 | Given all approved concept palettes, then every colour is from the shared palette. | manual |

## Open questions

- Q-010 Voxel vs. smooth low-poly; role of `ref-zoo-layout.webp`.
