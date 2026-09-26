# Brief — `kit_fences` (Kit 2 — fences, gates, hedges, walls)

Spec: ART-ENVIRONMENT (modular props), GAME-LAYOUT (barrier/boundary/enclosure), GAME-LEVEL-1. Style: `art/style/style.md` (comic) — style
reference image: `art/environment/style_frame/style_frame.png` (approved). Status: **brief**.

## Purpose

Everything that encloses an enclosure or seals the level. Fences keep animals in but must look friendly (never cages — glossary). Hedges and the outer wall form the permanent zoo boundary.

Props are modelled once and reused (ART-ENVIRONMENT, modular props). Budgets: props
≤ 500 triangles each, flat-colour textures only (ART-PIPELINE §10).

## Props in this kit

| Asset id | Size (game units, 1 = 1 m) | Notes |
|---|---|---|
| `fence_wood` | 2 m long (2.18 m incl. posts), 1.1 m high | post-and-rail, 3 rails, posts at both ends and in the middle |
| `fence_wood_1m` | 1 m long (1.18 m incl. posts), 1.1 m high | same fence, posts at both ends (Q-057) |
| `fence_wood_corner` | L piece, 1 m arms, 1.1 m high | corner post with rails towards east and north |
| `fence_wood_end` | post 0.25 m, 1.22 m high | end post with rounded top |
| `gate_wood` | leaf 1.8 m wide, 1.05 m high | enclosure gate leaf for a 2 m opening; origin on the hinge axis, **opened in-game by rotating it** about +Y (no separate closed/open models) |
| `hedge` | 2 m long, 3 m high, ~1 m thick | tall trimmed hedge |
| `hedge_1m` | 1 m long, 3 m high, ~1 m thick | same hedge, 1 m (Q-057) |
| `hedge_corner` | L piece, 1 m arms (1.5 m × 1.5 m), 3 m high | hedge corner |
| `zoo_wall` | 2 m long, 2.5 m high, 0.6 m thick (0.8 m cap) | outer zoo wall, light stone with wooden cap |
| `zoo_wall_1m` | 1 m long, 2.5 m high, 0.6 m thick (0.8 m cap) | same wall, 1 m (Q-057) |
| `zoo_wall_corner` | L piece, 1 m arms (1.4 m × 1.4 m), 2.5 m high | wall corner |

Straight runs are filled with 2 m pieces plus one 1 m piece at the end for odd lengths;
hedge/wall bands of the level (1–2 cells deep) get one row on the band's centre line
(GAME-LAYOUT "Modular edges", Q-057; joins Q-059). The concept sheet below was generated
from the first list (with `gate_wood_closed` / `gate_wood_open`); the prompt is kept as
generated.

## Image settings

- One asset sheet, **16:9, 2K**, generated with the style frame as reference image.
- Generate 2 variants (`sheet_v1`, `sheet_v2`), pick one; single props can later be cut out
  for modelling reference.

## Prompt (`sheet.png`)

First paragraph = STYLE block from `art/style/style.md`, verbatim.

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Game asset sheet: the props below are laid out in a neat grid with generous empty space between them, each prop isolated and complete, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a soft small contact shadow under each prop. Every prop is shown from the same elevated three-quarter top-down view as in a cozy zoo park simulation game (camera looking down at about 55 degrees, isometric-like, narrow field of view so vertical lines stay nearly parallel), all at the same scale.

Nine props in a 3 by 3 grid: 1) a straight wooden post-and-rail fence segment, warm brown wood, three rounded horizontal rails between two chunky posts with rounded tops, about as high as a child's shoulder; 2) a single wooden corner post of the same fence; 3) a single wooden end post; 4) a closed wooden enclosure gate with a diagonal brace and a simple metal latch, same wood as the fence; 5) the same gate standing open; 6) a straight segment of a tall, neatly trimmed dark-green hedge, clearly taller than an adult, with rounded top edges; 7) a corner piece of the same hedge; 8) a straight segment of a friendly outer zoo wall made of light grey rounded stones with a warm wooden cap on top; 9) a corner piece of the same wall. No bars, no wire, no cages.
```

### Negative prompt

```text
text, letters, words, numbers, writing, captions, writing on signs, watermark, signature, logo, brand names, UI, HUD, scenery, landscape, horizon, sky, characters, people, animals, overlapping props, cropped props, perspective distortion, fisheye, eye-level view, dark, gloomy, dirty, broken, rubbish, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Comic style matches `style_frame.png` (outlines, flat colours, one shadow tone).
- [ ] All props present, same scale, same camera angle, none cropped or overlapping.
- [ ] No text anywhere; panels and label plates blank.
- [ ] Readable at phone size from the high camera.
- [ ] Buildable as simple low-poly props (≤ 500 tris each).

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-09-26 | sheet_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png | alternative (corner rails float) |
| 2026-09-26 | sheet_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png | superseded by v3 (edit), removed — in git history |
| 2026-09-26 | sheet_v3.jpg | gemini-3-pro-image (2K, 16:9) | — | edit: Edit this asset sheet. Keep everything exactly the same (all props, positions, c…, ref: sheet_v2.jpg | to review — chosen (edit of v2: single end post) |
| 2026-09-26 | sheet_v4.jpg | gemini-3-pro-image (2K, 16:9) | — | edit: Edit this asset sheet. Keep everything exactly the same (all props, positions, c…, ref: sheet_v2.jpg | discarded (post too thin, deleted) |
