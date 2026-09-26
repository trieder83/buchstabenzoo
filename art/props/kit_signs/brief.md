# Brief — `kit_signs` (Kit 3 — signs, boards and food boxes)

Spec: ART-ENVIRONMENT (modular props), GAME-RESCUE, GAME-FEED, GAME-MAP, ART-DIRECTION §4. Style: `art/style/style.md` (comic) — style
reference image: `art/environment/style_frame/style_frame.png` (approved). Status: **brief**.

## Purpose

The reading props — the core of the gameplay. All text is rendered by the game (ART-ENVIRONMENT §2), so every panel and label plate is **blank**. From the high camera the panels are tilted back towards the camera (≈ 30–45°) so they stay readable.

Props are modelled once and reused (ART-ENVIRONMENT, modular props). Budgets: props
≤ 500 triangles each, flat-colour textures only (ART-PIPELINE §10).

## Props in this kit

| Asset id | Size (game units, 1 = 1 m) | Notes |
|---|---|---|
| `enclosure_sign` | ≈ 2 m wide, on two posts | blank panel + round silhouette slot; tilted back |
| `info_board` | ≈ 1 m wide, panel at child eye height | blank panel on one post, tilted back like a lectern |
| `map_board` | ≈ 2.5 m wide | blank map panel with a small roof |
| `food_box` | ≈ 0.6 m cube | closed wooden crate with blank label plate |
| `food_box_stack` |  | three stacked food boxes |

## Image settings

- One asset sheet, **16:9, 2K**, generated with the style frame as reference image.
- Generate 2 variants (`sheet_v1`, `sheet_v2`), pick one; single props can later be cut out
  for modelling reference.

## Prompt (`sheet.png`)

First paragraph = STYLE block from `art/style/style.md`, verbatim.

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Game asset sheet: the props below are laid out in a neat grid with generous empty space between them, each prop isolated and complete, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a soft small contact shadow under each prop. Every prop is shown from the same elevated three-quarter top-down view as in a cozy zoo park simulation game (camera looking down at about 55 degrees, isometric-like, narrow field of view so vertical lines stay nearly parallel), all at the same scale.

Five props in a row with generous space between them: 1) a large wooden enclosure sign on two chunky posts, a wide cream-coloured blank panel with a wooden frame and a small round cream-coloured blank slot on the left side for an animal silhouette, the panel tilted back by about 40 degrees so it faces up towards the camera; 2) a small wooden info board on a single post, a blank cream-coloured panel at the height of a child's eyes, tilted back like a reading desk, a small wooden roof above it; 3) a large wooden map board with a wide blank cream-coloured panel under a small wooden roof, on two posts; 4) a closed wooden food crate with a lid, chunky planks, a blank light label plate on its front; 5) three of the same closed food crates stacked. All panels and label plates are completely blank.
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
| 2026-09-26 | sheet_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png | discarded (style-frame scene blended into the sheet) |
| 2026-09-26 | sheet_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png | discarded (style-frame scene blended into the sheet) |
| 2026-09-26 | sheet_v3.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v2.jpg | alternative (extra crate, stack of four, roof on enclosure sign) |
| 2026-09-26 | sheet_v4.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v2.jpg | to review — chosen (ref: kit_fences sheet_v2 instead of the style frame) |
