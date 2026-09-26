# Brief — `rock_hill_cave` (rock hill with cave, closed + cutaway)

Spec: ART-ENVIRONMENT (unique model), GAME-LEVEL-1 (`rock_hill_w` 9, 0, 1, 8;
`rock_hill_back` 10, 0, 3, 5; `rock_hill_e` 13, 0, 9, 8; `path_cave_floor` 10, 5, 3, 3;
mouth faces north), GAME-PLAYER §2 (the rock roof is cut away while the player is in the
cave), CONT-MISSIONS (panda riddle: dark, cool, stone, echo). Style: `art/style/style.md`
(comic). Status: **in-review**.

## Purpose

The panda's hiding place `loc_cave`. From outside it must read as **dark and cool, but never
scary**. The rock roof above the 3 × 3 m cave floor is a separate part that is removed while
the player is inside → the sheet shows the hill **closed** and as a **cutaway** (roof removed,
cave floor and walls visible from above).

## Items

| View | Size (1 = 1 m) | Notes |
|---|---|---|
| closed | ≈ 13 × 8 m footprint, ≈ 4–5 m high | chunky rounded grey rock hill, a few bushes and grass tufts on top; a dark cave mouth ≈ 3 m wide, ≈ 2.5 m high on the front side (the side facing the camera = north in the game) |
| cutaway | same | the rock roof over the cave removed: a 3 × 3 m floor of grey flat stones, blue-grey cool shadow tones, damp rock walls, a few small pebbles and a puddle — empty (no panda in the model sheet) |

## Image settings

- Asset sheet, **16:9, 2K**. Reference image: the approved `art/props/kit_fences/sheet_v3.jpg`
  (the style frame blends its scene into asset sheets — not used).
- `--extra`: "Match the comic style, line weight, colours, camera angle and plain grey background of the attached asset sheet exactly, but draw the props described above."
- Generate 2 variants, keep the chosen one + one alternative.

## Prompt (`sheet.png`)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Game asset sheet: the props below are laid out with generous empty space between them, each prop isolated and complete, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a soft small contact shadow under each prop. Every prop is shown from the same elevated three-quarter top-down view as in a cozy zoo park simulation game (camera looking down at about 55 degrees, isometric-like, narrow field of view so vertical lines stay nearly parallel), all at the same scale, no ground plates, no scenery around them.

Two items side by side with generous space between them, the same rock hill twice: 1) a big friendly rock hill about 13 m wide, 8 m deep and 4 to 5 m high, made of chunky rounded light-grey boulders with simple flat cel-shaded faces, a few round green bushes and grass tufts on top, on the front side facing the camera a cave mouth about 3 m wide and 2.5 m high, the opening dark blue-grey inside but friendly, not scary; 2) the SAME rock hill with the rock roof over the cave lifted off and removed (not shown), like a dollhouse cut, so the cave interior is seen from above: a small room about 3 by 3 m with a floor of flat grey stones, cool blue-grey shadow tones on the rock walls, a few pebbles and a small puddle, the cut rock tops are flat and clean, the rest of the hill unchanged. All signs, panels and label plates are completely blank: plain wooden or cream-coloured panels without any letters, words or numbers.
```

### Negative prompt

```text
text, letters, words, numbers, writing, captions, writing on signs, watermark, signature, logo, brand names, UI, HUD, scenery, landscape, horizon, sky, grass field, ground plane, characters, people, animals, overlapping props, cropped props, perspective distortion, fisheye, eye-level view, dark, gloomy, dirty, broken, rubbish, cage, cage bars, prison, monster, bats, skull, bones, spider webs, glowing eyes, lava, mine, rails, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Comic style matches the approved kit sheets (outlines, flat colours, one shadow tone).
- [ ] All items present, same camera angle, none cropped or overlapping; no text anywhere.
- [ ] Sizes/footprints plausible for GAME-LEVEL-1 (see table); readable at phone size.
- [ ] Cutaways (where shown): roof fully removed, interior tidy and readable from above (GAME-PLAYER §2 "Roofs disappear inside buildings").
- [ ] Buildable as low-poly models with a separate roof part (budget: buildings ≤ 3 000 tris, props ≤ 500).
- [ ] Cave reads dark and cool from the high camera, not scary; cutaway shows a 3 × 3 m floor.
## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-09-26 | sheet_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v3.jpg | alternative / source of v3 — wooden mine posts inside the cutaway |
| 2026-09-26 | sheet_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v3.jpg | discarded (deleted) — rocks with faces, mouth on the side |
| 2026-09-26 | sheet_v3.jpg | gemini-3-pro-image (2K, 16:9) | — | edit: Edit this asset sheet. Keep everything exactly the same (style, camera, position…, ref: sheet_v1.jpg | **chosen** — edit of v1: wooden posts removed |
