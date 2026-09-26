# Brief — `kit_furniture` (bench and feeding trough)

Spec: ART-ENVIRONMENT (modular props), GAME-LEVEL-1 (`bench_plaza`, `bench_pond`: 2 × 1 m),
GAME-FAMILY §4 (care feeding: the enclosure's **feeding trough**). Style: `art/style/style.md`
(comic). Status: **in-review**.

## Purpose

Props that are still placeholders in the game. The feeding trough stands in each enclosure
with a pair (zebra, koala); the player puts the right food into it (GAME-FAMILY). It must
read as "food goes here" from the high camera, and it is **not a water trough** (the zebra
enclosure must contain no water — GAME-LEVEL-1). Budget ≤ 500 tris each.

## Items

| Asset id | Size (1 = 1 m) | Notes |
|---|---|---|
| `bench` | 2 × 0.6 m, seat 0.45 m | wooden park bench with backrest (as in the style frame) |
| `feeding_trough` | ≈ 1.6 × 0.6 m, ≈ 0.7 m high | wooden trough on legs, empty |
| `feeding_trough` (filled state) | same | the same trough filled with food (the game swaps a food mesh in; hay shown as an example) |

## Image settings

- Asset sheet, **16:9, 2K**. Reference image: the approved `art/props/kit_fences/sheet_v3.jpg`
  (the style frame blends its scene into asset sheets — not used).
- `--extra`: "Match the comic style, line weight, colours, camera angle and plain grey background of the attached asset sheet exactly, but draw the props described above."
- Generate 2 variants, keep the chosen one + one alternative.

## Prompt (`sheet.png`)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Game asset sheet: the props below are laid out with generous empty space between them, each prop isolated and complete, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a soft small contact shadow under each prop. Every prop is shown from the same elevated three-quarter top-down view as in a cozy zoo park simulation game (camera looking down at about 55 degrees, isometric-like, narrow field of view so vertical lines stay nearly parallel), all at the same scale, no ground plates, no scenery around them.

Three items in a row with generous space between them: 1) a wooden park bench about 2 m long with a backrest and armrests, chunky warm-brown planks, sturdy legs; 2) an empty wooden animal feeding trough about 1.6 m long and 0.7 m high: a long open box of chunky warm-brown planks with slanted sides, on four short sturdy legs, empty inside, dry, no water; 3) the SAME feeding trough filled with a heap of yellow-green hay. All signs, panels and label plates are completely blank: plain wooden or cream-coloured panels without any letters, words or numbers.
```

### Negative prompt

```text
text, letters, words, numbers, writing, captions, writing on signs, watermark, signature, logo, brand names, UI, HUD, scenery, landscape, horizon, sky, grass field, ground plane, characters, people, animals, overlapping props, cropped props, perspective distortion, fisheye, eye-level view, dark, gloomy, dirty, broken, rubbish, cage, cage bars, prison, water, water trough, bathtub, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Comic style matches the approved kit sheets (outlines, flat colours, one shadow tone).
- [ ] All items present, same camera angle, none cropped or overlapping; no text anywhere.
- [ ] Sizes/footprints plausible for GAME-LEVEL-1 (see table); readable at phone size.
- [ ] Cutaways (where shown): roof fully removed, interior tidy and readable from above (GAME-PLAYER §2 "Roofs disappear inside buildings").
- [ ] Buildable as low-poly models with a separate roof part (budget: buildings ≤ 3 000 tris, props ≤ 500).
## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-09-26 | sheet_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v3.jpg | **chosen** — bench and trough (empty / with hay), clearly not a water trough |
| 2026-09-26 | sheet_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v3.jpg | alternative — slightly smaller props |
