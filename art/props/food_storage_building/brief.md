# Brief — `food_storage_building` (food storage barn, closed + cutaway)

Spec: ART-ENVIRONMENT (unique models), GAME-LEVEL-1 (`food_storage` -4, 11, 8, 6; door at
cell (0, 11) on the south facade), GAME-FEED, GAME-PLAYER §2 ("Roofs disappear inside
buildings"). Style: `art/style/style.md` (comic). Status: **in-review**.

## Purpose

The central building of every mission: the child walks in, reads the food box labels and
takes the right box. **User decision:** the roof must be removable to see inside — the sheet
shows the barn once **closed** and once as a **cutaway with the roof removed**, showing the
interior with shelves and the food boxes (labels blank — rendered by the game, 10 boxes in
level 1, Q-047).

## Items

| View | Size (1 = 1 m) | Notes |
|---|---|---|
| closed | 8 × 6 m footprint, eaves ≈ 3 m, ridge ≈ 5 m | red-brown wooden barn, wide open double door in the middle of the long (front) side, white trim, small round window in the gable |
| cutaway | same | roof removed, front wall cut low; wooden shelves along the back and side walls, 10 closed food crates (as `food_box` of `kit_signs`) on low shelves and the floor, blank label plates facing the door, a hand cart, straw on the floor |

## Image settings

- Asset sheet, **16:9, 2K**. Reference image: the approved `art/props/kit_fences/sheet_v3.jpg`
  (the style frame blends its scene into asset sheets — not used).
- `--extra`: "Match the comic style, line weight, colours, camera angle and plain grey background of the attached asset sheet exactly, but draw the props described above."
- Generate 2 variants, keep the chosen one + one alternative.
- Second reference: `art/props/kit_signs/sheet_v5.jpg` (the approved `food_box`), so the crates match.

## Prompt (`sheet.png`)



```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Game asset sheet: the props below are laid out with generous empty space between them, each prop isolated and complete, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a soft small contact shadow under each prop. Every prop is shown from the same elevated three-quarter top-down view as in a cozy zoo park simulation game (camera looking down at about 55 degrees, isometric-like, narrow field of view so vertical lines stay nearly parallel), all at the same scale, no ground plates, no scenery around them. A cutaway view means: the same building from the same camera angle with its whole roof lifted off and removed (not shown), and the upper part of the front walls cut away at a clean horizontal line, so the tidy interior is clearly visible from above, like a dollhouse; the cut wall tops are flat and clean.

Two items side by side with generous space between them, the same building twice: 1) a friendly zoo food storage barn about 8 m wide and 6 m deep, walls of vertical warm red-brown wooden planks with white corner trim, a gable roof of dark brown shingles, the long front side faces the camera with a wide double door in the middle standing wide open (dark inside), a small round window in the gable, a blank cream-coloured sign board above the door; 2) the SAME barn as a cutaway with the roof removed and the front wall cut away low: inside on a wooden plank floor, simple wooden shelves along the back and side walls, ten identical closed wooden food crates with chunky planks and a blank light label plate on the front, standing in a neat row on low shelves and on the floor with their label plates facing the open door, a little wooden hand cart, a few wisps of straw. The crates look exactly like the crates in the second attached image. All signs, panels and label plates are completely blank: plain wooden or cream-coloured panels without any letters, words or numbers.
```

### Negative prompt

```text
text, letters, words, numbers, writing, captions, writing on signs, watermark, signature, logo, brand names, UI, HUD, scenery, landscape, horizon, sky, grass field, ground plane, characters, people, animals, overlapping props, cropped props, perspective distortion, fisheye, eye-level view, dark, gloomy, dirty, broken, rubbish, cage, cage bars, prison, open crates, spilled food, fruit on the floor, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Comic style matches the approved kit sheets (outlines, flat colours, one shadow tone).
- [ ] All items present, same camera angle, none cropped or overlapping; no text anywhere.
- [ ] Sizes/footprints plausible for GAME-LEVEL-1 (see table); readable at phone size.
- [ ] Cutaways (where shown): roof fully removed, interior tidy and readable from above (GAME-PLAYER §2 "Roofs disappear inside buildings").
- [ ] Buildable as low-poly models with a separate roof part (budget: buildings ≤ 3 000 tris, props ≤ 500).
- [ ] Crates match the approved `food_box`; 10 crates, all labels blank and facing the door.
## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-09-26 | sheet_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v3.jpg, sheet_v5.jpg | alternative — clean cutaway but only 7 crates, shelves empty |
| 2026-09-26 | sheet_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v3.jpg, sheet_v5.jpg | **chosen** — ≈ 10 crates on shelves and floor, labels blank, front wall cut low; door is in the gable end (ridge runs north–south — fine for the 8 m south facade) |
