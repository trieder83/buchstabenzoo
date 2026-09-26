# Brief — `kit_barriers` (Kit 6 — barriers)

Spec: ART-ENVIRONMENT (modular props), GAME-LAYOUT (barrier), GAME-LEVEL-1 (barrier_ne_tree, barrier_north_gate, barrier_east_repair). Style: `art/style/style.md` (comic) — style
reference image: `art/environment/style_frame/style_frame.png` (approved). Status: **brief**.

## Purpose

Child-friendly barriers that close off locked parts of the zoo (never invisible walls). They must say "not yet" in a friendly way — no danger signs. Pictograms are allowed on the repair sign (shovel), no text.

Props are modelled once and reused (ART-ENVIRONMENT, modular props). Budgets: props
≤ 500 triangles each, flat-colour textures only (ART-PIPELINE §10).

## Props in this kit

| Asset id | Size (game units, 1 = 1 m) | Notes |
|---|---|---|
| `road_block` | ≈ 2 m wide | wooden barrier with red-and-white striped board |
| `repair_sign` | ≈ 1 m | blank sign with a shovel pictogram |
| `zookeeper_cart` | ≈ 2 m long | small green zookeeper cart with tools |
| `traffic_cone` | ≈ 0.5 m | orange traffic cone |
| `fallen_tree` | ≈ 6 m long | fallen tree lying across a path |
| `gate_zoo_closed` | ≈ 4 m wide | large closed zoo gate between two pillars |

## Image settings

- One asset sheet, **16:9, 2K**, generated with an existing prop sheet (kit_fences sheet_v2) as style reference — the style frame as reference blends its scene into the sheet.
- Generate 2 variants (`sheet_v1`, `sheet_v2`), pick one; single props can later be cut out
  for modelling reference.

## Prompt (`sheet.png`)

First paragraph = STYLE block from `art/style/style.md`, verbatim.

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Game asset sheet: the props below are laid out in a neat grid with generous empty space between them, each prop isolated and complete, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a soft small contact shadow under each prop. Every prop is shown from the same elevated three-quarter top-down view as in a cozy zoo park simulation game (camera looking down at about 55 degrees, isometric-like, narrow field of view so vertical lines stay nearly parallel), all at the same scale.

Six props in two rows of three: 1) a friendly wooden road barrier on two legs with a red-and-white striped board; 2) a small wooden sign on a post with a simple white shovel pictogram on a blue panel and nothing else; 3) a small green zookeeper utility cart with a flat bed carrying a shovel, a rake and a wheelbarrow-like bucket; 4) an orange traffic cone with a white band; 5) a big fallen tree lying on its side, trunk with roots on one end and a leafy crown on the other, long enough to block a path; 6) a large closed double gate made of wooden planks with iron fittings, between two chunky stone pillars, clearly closed. Friendly and tidy, nothing broken or dangerous-looking.
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
| 2026-09-26 | sheet_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v2.jpg | alternative (root ball on the fallen tree, plainer gate) |
