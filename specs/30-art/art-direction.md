---
id: ART-DIRECTION
title: Art direction
aspect: art
module: art-direction
status: draft
depends_on: []
test_prefix: ADIR
updated: 2026-09-27
---

# Art direction

## Goal

A friendly, bright, readable **comic-style** 3D look for 4–11 year olds that runs smoothly
on phones, seen from a high-angle zoo-park camera (GAME-PLAYER).

## References

The **style** is defined only in `art/style/style.md` (comic, decided 2026-09-26, Q-010).
The reference images below are for content and layout ideas, not for the style. (The first three external voxel references were removed as outdated on 2026-09-27; our own approved art replaces them.)

| Image | What we take from it |
|---|---|
| `art/characters/player_girl/front.png` | Player character look (approved turnaround). |
| `art/environment/style_frame/style_frame.png` | The approved style frame: comic look, enclosure sign with silhouette, fences, grass, camera. |
| `art/props/kit_enclosure_buildings/sheet_zebra_hippo_v3.jpg` | Distinct enclosure buildings per animal (approved concept). |
| `art/reference/ref-zoo-layout.webp` | High-angle zoo-park camera, zoo layout with paths, kiosks and props. |

## Behaviour

1. **Comic style** (`art/style/style.md`): bold dark outlines, flat colours with one hard
   shadow tone (cel shading), rounded chunky shapes, big friendly eyes. No voxels, no pixel
   art, no realistic textures, no painterly gradients.
2. Outlines and cel shading are produced by the renderer (outline pass + 2-tone shader),
   not baked into textures or modelled (TECH-ARCH §7; outline technique Q-050).
3. Bright daylight, crisp shadows. **Zoo view** (default camera, GAME-PLAYER §2): ground
   only, no sky. **Close views** (look-around, first person): comic sky with flat rounded,
   outlined clouds and a pastel distance haze (distance fog) from 11.7 m, fully hiding from
   20.8 m (GAME-CAMERA-VIEWS 5, 7; tested by PLAY-009, CAMV-004, CAMV-016). Night colours of
   sky and haze: Q-126.
4. **Signs and labels are gameplay.** From the high camera, signs are tilted towards the
   camera and show large names/silhouettes; longer texts (info boards, food box labels,
   riddles) are read in a close-up text panel that opens on interaction (GAME-PLAYER §4).
5. Each enclosure has a distinct silhouette/material so children can recognise it without
   reading (important for `kiga`).
6. Colour palette is defined once (`assets/textures/palette.png` (generated from `tools/blender/palette.toml`)) and reused by all assets.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| ADIR-001 | Given the default high-angle camera on a 1080×2340 viewport and the player next to an enclosure sign, then the sign's name/silhouette has a height ≥ 3 % of the viewport height. | e2e |
| ADIR-002 | Given all approved concept palettes, then every colour is from the shared palette. | manual |
| ADIR-003 | Given the player interacts with an info board or food box, then a text panel opens with text cap height ≥ 3 % of the viewport height. | e2e |
| ADIR-004 | *Retired — duplicate of APIPE-010 (style blocks verbatim in every prompt).* | — |
| ADIR-005 | Given the approved style frame and a renderer screenshot of the same scene (zebra enclosure, default camera), then reviewers confirm the in-game cel shading (one hard shadow tone) and outlines match the style frame. | manual |

## Open questions

- Q-010 answered: comic style (`art/style/style.md`). Q-049 answered: high-angle camera (§3–4).
- Q-050 outline technique. Q-048 screen orientation (ADIR-001/003 use portrait).
