# Brief — `kit_ground` (Kit 1 — ground and paths)

Spec: ART-ENVIRONMENT (modular props), GAME-LAYOUT (surfaces path/grass), GAME-LEVEL-1. Style: `art/style/style.md` (comic) — style
reference image: `art/environment/style_frame/style_frame.png` (approved). Status: **brief**.

## Purpose

Floor tiles for the tile-based map. Paths are faster to walk on than grass (GAME-PLAYER §6), so path and grass must be clearly different at a glance from the high camera.

Props are modelled once and reused (ART-ENVIRONMENT, modular props). Budgets: props
≤ 500 triangles each, flat-colour textures only (ART-PIPELINE §10).

## Props in this kit

| Asset id | Size (game units, 1 = 1 m) | Notes |
|---|---|---|
| `path_tile_straight` | 2 m × 2 m | light sand-beige paved path, straight |
| `path_tile_curve` | 2 m × 2 m | 90° curve |
| `path_tile_t` | 2 m × 2 m | T-junction |
| `path_tile_cross` | 2 m × 2 m | crossing |
| `path_tile_end` | 2 m × 2 m | rounded dead end |
| `plaza_tile` | 2 m × 2 m | larger paving stones for the entrance plaza |
| `grass_tile` | 2 m × 2 m | short fresh grass with a few tufts |
| `path_edge` | 2 m long | low edging stones between path and grass |
| `sand_tile` | 2 m × 2 m | warm sand for enclosure floors |

## Image settings

- One asset sheet, **16:9, 2K**, generated with the style frame as reference image.
- Generate 2 variants (`sheet_v1`, `sheet_v2`), pick one; single props can later be cut out
  for modelling reference.

## Prompt (`sheet.png`)

First paragraph = STYLE block from `art/style/style.md`, verbatim.

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Game asset sheet: the props below are laid out in a neat grid with generous empty space between them, each prop isolated and complete, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a soft small contact shadow under each prop. Every prop is shown from the same elevated three-quarter top-down view as in a cozy zoo park simulation game (camera looking down at about 55 degrees, isometric-like, narrow field of view so vertical lines stay nearly parallel), all at the same scale.

Nine square ground tiles, each a thin flat slab of exactly the same square size, shown like board-game tiles in a 3 by 3 grid: 1) straight light sand-beige paved path made of large rounded paving stones, grass along both sides; 2) the same path as a 90-degree curve; 3) the same path as a T-junction; 4) the same path as a crossing; 5) the same path ending in a rounded dead end; 6) an entrance plaza tile fully paved with larger light paving stones in a simple pattern; 7) a plain tile of short fresh green grass with a few small tufts; 8) a long narrow strip of low rounded edging stones (a path border); 9) a tile of warm light sand. The paved paths are clearly lighter and smoother than the grass, so path and grass are easy to tell apart. Paths continue seamlessly to the tile edges so the tiles can be placed next to each other.
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
| 2026-09-26 | sheet_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png | alternative (T-junction drawn as a corner) |
| 2026-09-26 | sheet_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png | to review — chosen |
