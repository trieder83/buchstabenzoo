# Brief — `kit_nature` (Kit 4 — nature)

Spec: ART-ENVIRONMENT (modular props), GAME-LEVEL-1, CONT-MISSIONS (riddle clues: bamboo, eucalyptus, reed). Style: `art/style/style.md` (comic) — style
reference image: `art/environment/style_frame/style_frame.png` (approved). Status: **brief**.

## Purpose

Vegetation that fills the zoo. Some plants are riddle clues (bamboo, eucalyptus, reed), so they must be recognisable at phone size from the high camera.

Props are modelled once and reused (ART-ENVIRONMENT, modular props). Budgets: props
≤ 500 triangles each, flat-colour textures only (ART-PIPELINE §10).

## Props in this kit

| Asset id | Size (game units, 1 = 1 m) | Notes |
|---|---|---|
| `tree_round` | ≈ 5 m high | round-topped deciduous tree |
| `tree_grove` | ≈ 6 m high | dense tall tree for the hidden grove |
| `tree_eucalyptus` | ≈ 7 m high | tall slim eucalyptus, blue-green leaves |
| `bush` | ≈ 1.2 m | round bush |
| `flower_bed` | 2 m × 1 m | low bed with colourful flowers |
| `rock` | ≈ 1 m | rounded grey boulder |
| `bamboo` | ≈ 3 m | clump of bamboo stalks |
| `reed` | ≈ 1 m | clump of reeds (pond edge) |
| `grass_tuft` | ≈ 0.3 m | small tuft of tall grass |

## Image settings

- One asset sheet, **16:9, 2K**, generated with an existing prop sheet (kit_fences sheet_v2) as style reference — the style frame as reference blends its scene into the sheet.
- Generate 2 variants (`sheet_v1`, `sheet_v2`), pick one; single props can later be cut out
  for modelling reference.

## Prompt (`sheet.png`)

First paragraph = STYLE block from `art/style/style.md`, verbatim.

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Game asset sheet: the props below are laid out in a neat grid with generous empty space between them, each prop isolated and complete, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a soft small contact shadow under each prop. Every prop is shown from the same elevated three-quarter top-down view as in a cozy zoo park simulation game (camera looking down at about 55 degrees, isometric-like, narrow field of view so vertical lines stay nearly parallel), all at the same scale.

Nine props in a 3 by 3 grid: 1) a round-topped leafy deciduous tree with a chunky brown trunk; 2) a taller, very dense dark-green tree with a full rounded crown (for a hidden grove); 3) a tall slim eucalyptus tree with a pale smooth trunk and blue-green leaves hanging in clusters; 4) a round green bush; 5) a low rectangular flower bed with a wooden border and bright red, yellow and purple flowers; 6) a rounded grey boulder; 7) a clump of green bamboo stalks with visible joints and narrow leaves; 8) a clump of tall reeds with brown cattail heads; 9) a small tuft of tall grass.
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
| 2026-09-26 | sheet_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v2.jpg | to review — chosen |
| 2026-09-26 | sheet_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v2.jpg | alternative (nearly identical) |
