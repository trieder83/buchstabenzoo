# Brief — `kit_water` (Kit 5 — water)

Spec: ART-ENVIRONMENT (modular props), GAME-LEVEL-1 (loc_river, loc_pond), CONT-MISSIONS (zebra, hippo riddles). Style: `art/style/style.md` (comic) — style
reference image: `art/environment/style_frame/style_frame.png` (approved). Status: **brief**.

## Purpose

River and pond are riddle places: the river is **flowing** (zebra: *wo das Wasser fließt*, bridge, ducks), the pond is **still** (hippo: water lilies, frogs). They must look clearly different at a glance.

Props are modelled once and reused (ART-ENVIRONMENT, modular props). Budgets: props
≤ 500 triangles each, flat-colour textures only (ART-PIPELINE §10).

## Props in this kit

| Asset id | Size (game units, 1 = 1 m) | Notes |
|---|---|---|
| `water_river_straight` | 2 m × 2 m | flowing river tile with banks |
| `water_river_curve` | 2 m × 2 m | river curve |
| `water_pond` | 2 m × 2 m | still pond tile |
| `water_pond_edge` | 2 m × 2 m | pond edge with bank |
| `bridge_wood` | ≈ 6 m long | wooden footbridge (spans river tiles) |
| `jetty_wood` | ≈ 3 m | small wooden jetty |
| `lily_pad` | ≈ 0.5 m | lily pads with a flower |
| `duck` | ≈ 0.4 m | friendly duck |
| `frog` | ≈ 0.2 m | friendly green frog on a lily pad |

## Image settings

- One asset sheet, **16:9, 2K**, generated with an existing prop sheet (kit_fences sheet_v2) as style reference — the style frame as reference blends its scene into the sheet.
- Generate 2 variants (`sheet_v1`, `sheet_v2`), pick one; single props can later be cut out
  for modelling reference.

## Prompt (`sheet.png`)

First paragraph = STYLE block from `art/style/style.md`, verbatim.

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Game asset sheet: the props below are laid out in a neat grid with generous empty space between them, each prop isolated and complete, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a soft small contact shadow under each prop. Every prop is shown from the same elevated three-quarter top-down view as in a cozy zoo park simulation game (camera looking down at about 55 degrees, isometric-like, narrow field of view so vertical lines stay nearly parallel), all at the same scale.

Nine props in a 3 by 3 grid: 1) a square ground tile with a straight river running through it, lively blue water with white flow lines and small ripples showing it moves, grassy banks on both sides; 2) the same flowing river as a 90-degree curve; 3) a square tile of a calm, still pond, smooth darker blue water without flow lines, one soft reflection; 4) a pond edge tile, still water meeting a grassy bank with a few reeds; 5) an arched wooden footbridge with railings, long enough to cross the river; 6) a small wooden jetty on posts; 7) a group of round green lily pads with one pink water-lily flower; 8) a friendly white-and-yellow cartoon duck; 9) a friendly green cartoon frog sitting on a lily pad. The river tiles continue seamlessly to the tile edges.
```

### Negative prompt

```text
text, letters, words, numbers, writing, captions, writing on signs, watermark, signature, logo, brand names, UI, HUD, scenery, landscape, horizon, sky, characters, people, large animals, overlapping props, cropped props, perspective distortion, fisheye, eye-level view, dark, gloomy, dirty, broken, rubbish, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
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
| 2026-09-26 | sheet_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v2.jpg | to review — chosen (river clearly flowing, pond clearly still) |
| 2026-09-26 | sheet_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v2.jpg | alternative (river flows less visibly, bridge on a tile) |
