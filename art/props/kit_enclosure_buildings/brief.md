# Brief — `kit_enclosure_buildings` (zebra, hippo and panda enclosure buildings)

Spec: ART-ENVIRONMENT (unique models: `stone_arch_shelter`, `hut_wood`, `pool_tiled`,
`panda_shelter`, `panda_platform`), GAME-LEVEL-1, GAME-PLAYER §2. Style: `art/style/style.md`
(comic). Reference for the buildings: the chosen sheets in this folder (the old voxel reference was removed as outdated, 2026-09-27). Status:
**in-review**.

## Purpose

Each enclosure gets one recognisable landmark building (zebra: stone arch; hippo: wooden hut +
tiled pool; panda: wooden shelter + climbing platform). Riddle constraints (GAME-LEVEL-1):
the zebra arch is **open on both sides** (must not read as a cave); the hippo pool is
**square with a tiled edge** — no lilies, no frogs (must not look like the pond); the panda
buildings are **wood only** — no stone (must not look like the cave). The hippo hut can be
entered → closed + cutaway.

## Items

| Asset id | Size (1 = 1 m) | Notes |
|---|---|---|
| `stone_arch_shelter` | ≈ 3.5 × 2 m, ≈ 2.8 m high | grey rounded stone arch, open on both ends (zebra, north-west corner of `enc_zebra`) |
| `hut_wood` | ≈ 4 × 3.5 m, ≈ 3 m high | wooden hippo hut, wide low door; cutaway: straw bed, water bucket |
| `pool_tiled` | ≈ 5 × 5 m, 0.4 m rim | square pool, light blue-white tiled rim, steps/ramp on one side, clear blue water |
| `panda_shelter` | ≈ 3 × 3 m, ≈ 2.8 m high | open wooden lean-to with a plank roof, straw inside |
| `panda_platform` | ≈ 3 × 3 m, deck ≈ 1.2 m high | wooden climbing platform with a ramp and a log ladder |
| `bamboo_feeding_rack` | ≈ 1.5 × 0.6 m, ≈ 1.2 m high | wooden rack holding cut bamboo stalks (GAME-LEVEL-1 `enc_panda`; proposal Q-074 there) |

Two sheets: prompt 1 = zebra + hippo, prompt 2 = panda.

## Image settings

- Asset sheet, **16:9, 2K**. Reference image: the approved `art/props/kit_fences/sheet_v3.jpg`
  (the style frame blends its scene into asset sheets — not used).
- `--extra`: "Match the comic style, line weight, colours, camera angle and plain grey background of the attached asset sheet exactly, but draw the props described above."
- Generate 2 variants, keep the chosen one + one alternative.

### Prompt 1 (`sheet_zebra_hippo`)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Game asset sheet: the props below are laid out with generous empty space between them, each prop isolated and complete, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a soft small contact shadow under each prop. Every prop is shown from the same elevated three-quarter top-down view as in a cozy zoo park simulation game (camera looking down at about 55 degrees, isometric-like, narrow field of view so vertical lines stay nearly parallel), all at the same scale, no ground plates, no scenery around them. A cutaway view means: the same building from the same camera angle with its whole roof lifted off and removed (not shown), and the upper part of the front walls cut away at a clean horizontal line, so the tidy interior is clearly visible from above, like a dollhouse; the cut wall tops are flat and clean.

Four items in a row with generous space between them: 1) a zebra shelter: a big friendly arch of light grey rounded stones, about 3.5 m long and 2.8 m high, like a short stone tunnel that is open at both ends so you can see straight through it, a little grass on its top; 2) a wooden hippo hut about 4 by 3.5 m: warm-brown log walls, a low sloping green plank roof, a wide low open doorway, a round window; 3) the SAME hippo hut as a cutaway with the roof removed: inside a big round bed of yellow straw on the plank floor, a wooden water bucket, a hay rack on the wall; 4) a square hippo pool about 5 by 5 m sunk into the ground: a clean raised rim of light blue and white square tiles, a wide shallow ramp of tiles on one side for walking in, clear light-blue water with simple flat cartoon ripples, no plants in the water. All signs, panels and label plates are completely blank: plain wooden or cream-coloured panels without any letters, words or numbers.
```

### Prompt 2 (`sheet_panda`)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Game asset sheet: the props below are laid out with generous empty space between them, each prop isolated and complete, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a soft small contact shadow under each prop. Every prop is shown from the same elevated three-quarter top-down view as in a cozy zoo park simulation game (camera looking down at about 55 degrees, isometric-like, narrow field of view so vertical lines stay nearly parallel), all at the same scale, no ground plates, no scenery around them.

Three wooden items in a row with generous space between them, all made only of wood, no stone: 1) a panda shelter: an open wooden lean-to about 3 by 3 m and 2.8 m high, three plank walls and a sloping plank roof, open at the front, a bed of yellow straw inside; 2) a wooden panda climbing platform: a square plank deck about 3 by 3 m on four thick log posts at about 1.2 m height, a railing of round logs, a wide wooden ramp with cross slats leading up, a short log ladder on another side; 3) a small wooden bamboo feeding rack about 1.5 m wide: a trough-like rack of planks on short legs holding a bundle of cut green bamboo stalks with a few leaves. All signs, panels and label plates are completely blank: plain wooden or cream-coloured panels without any letters, words or numbers.
```

### Negative prompt

```text
text, letters, words, numbers, writing, captions, writing on signs, watermark, signature, logo, brand names, UI, HUD, scenery, landscape, horizon, sky, grass field, ground plane, characters, people, animals, overlapping props, cropped props, perspective distortion, fisheye, eye-level view, dark, gloomy, dirty, broken, rubbish, cage, cage bars, prison, water lilies, frogs, reeds, cave, dark hole, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Comic style matches the approved kit sheets (outlines, flat colours, one shadow tone).
- [ ] All items present, same camera angle, none cropped or overlapping; no text anywhere.
- [ ] Sizes/footprints plausible for GAME-LEVEL-1 (see table); readable at phone size.
- [ ] Cutaways (where shown): roof fully removed, interior tidy and readable from above (GAME-PLAYER §2 "Roofs disappear inside buildings").
- [ ] Buildable as low-poly models with a separate roof part (budget: buildings ≤ 3 000 tris, props ≤ 500).
- [ ] Zebra arch clearly open on both sides (not a cave); pool clearly tiled and square (not the pond); panda items wood only.
## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-09-26 | sheet_panda_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 2 + negative as 'Avoid' + extra text, ref: sheet_v3.jpg | alternative — plain plank ramp |
| 2026-09-26 | sheet_panda_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 2 + negative as 'Avoid' + extra text, ref: sheet_v3.jpg | **chosen** — ramp with cross slats, bamboo bundle in the rack |
| 2026-09-26 | sheet_zebra_hippo_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v3.jpg | alternative / source of v3 — extra duplicate hut, cutaway kept half the roof |
| 2026-09-26 | sheet_zebra_hippo_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v3.jpg | discarded (deleted) — cutaway roof shown as a lifted lid |
| 2026-09-26 | sheet_zebra_hippo_v3.jpg | gemini-3-pro-image (2K, 16:9) | — | edit: Edit this asset sheet. Keep everything exactly the same (style, camera, position…, ref: sheet_zebra_hippo_v1.jpg | **chosen** — edit of v1: duplicate hut removed, cutaway roof removed |
| 2026-09-26 | sheet_zebra_hippo_v4.jpg | gemini-3-pro-image (2K, 16:9) | — | edit: Edit this asset sheet. Keep everything exactly the same (style, camera, position…, ref: sheet_zebra_hippo_v1.jpg | discarded (deleted) — near-identical to v3 |
