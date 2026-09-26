# Brief — `kit_buildings` (entrance arch and zookeeper house)

Spec: ART-ENVIRONMENT (unique models of level 1), GAME-LEVEL-1, GAME-PLAYER §2 ("Roofs disappear
inside buildings"). Style: `art/style/style.md` (comic). Status: **in-review**.

## Purpose

The two human buildings of level 1 besides the food storage (own brief:
`art/props/food_storage_building/`). The zookeeper house can be entered, so it is shown
once closed and once as a **cutaway with the roof removed** (the roof is a separate model
part that fades out while the player is inside).

## Items

| Asset id | Size (1 = 1 m) | Notes |
|---|---|---|
| `entrance_arch` | footprint 6 × 2 m (`entrance_gate` -3, -2, 6, 2), arch ≈ 4.5 m high, passage ≈ 3 m wide | two chunky stone-and-wood pillars, wooden arch beam with a **blank** sign board, closed turnstiles, a small ticket booth on one side (optional) |
| `zookeeper_house` | ≈ 4 × 4 m, eaves ≈ 2.5 m | small cosy wooden house, door, window, chimney; cutaway: table, chair, shelf with buckets and a broom, a hook with a cap — no people. *Not placed in `level-1.toml` yet* (placement: level designer) |

## Image settings

- Asset sheet, **16:9, 2K**. Reference image: the approved `art/props/kit_fences/sheet_v3.jpg`
  (the style frame blends its scene into asset sheets — not used).
- `--extra`: "Match the comic style, line weight, colours, camera angle and plain grey background of the attached asset sheet exactly, but draw the props described above."
- Generate 2 variants, keep the chosen one + one alternative.

## Prompt (`sheet.png`)



```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Game asset sheet: the props below are laid out with generous empty space between them, each prop isolated and complete, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a soft small contact shadow under each prop. Every prop is shown from the same elevated three-quarter top-down view as in a cozy zoo park simulation game (camera looking down at about 55 degrees, isometric-like, narrow field of view so vertical lines stay nearly parallel), all at the same scale, no ground plates, no scenery around them. A cutaway view means: the same building from the same camera angle with its whole roof lifted off and removed (not shown), and the upper part of the front walls cut away at a clean horizontal line, so the tidy interior is clearly visible from above, like a dollhouse; the cut wall tops are flat and clean.

Three items in a row with generous space between them: 1) a friendly zoo entrance gate about 6 m wide and 4.5 m high: two chunky pillars of light grey rounded stones with wooden caps, a thick curved warm-brown wooden arch beam between them carrying a large blank cream-coloured sign board with a wooden frame, below the arch a row of three closed wooden turnstiles, beside one pillar a tiny wooden ticket booth with a small window and a little roof; 2) a small cosy wooden zookeeper house about 4 by 4 m: warm-brown plank walls, a dark red shingle roof with a small stone chimney, a green front door, one window with white frame and flower box, a small porch step; 3) the SAME zookeeper house as a cutaway with the roof removed: inside a simple wooden table with a chair, a wall shelf with metal buckets and folded towels, a broom leaning in a corner, a coat hook with a green cap, a rug on the wooden floor, no people. All signs, panels and label plates are completely blank: plain wooden or cream-coloured panels without any letters, words or numbers.
```

### Negative prompt

```text
text, letters, words, numbers, writing, captions, writing on signs, watermark, signature, logo, brand names, UI, HUD, scenery, landscape, horizon, sky, grass field, ground plane, characters, people, animals, overlapping props, cropped props, perspective distortion, fisheye, eye-level view, dark, gloomy, dirty, broken, rubbish, cage, cage bars, prison, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
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
| 2026-09-26 | sheet_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v3.jpg | alternative — nicer symmetric arch, but the house cutaway keeps the back half of the roof |
| 2026-09-26 | sheet_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v3.jpg | **chosen** — cutaway with the roof fully removed; arch beam slightly lopsided (fix when modelling) |
