# Brief — `kit_garden` (vegetable garden kit)

Spec: GAME-GARDEN (§ Assets, item 9). Style: `art/style/style.md` (comic) — style
reference image for asset sheets: `art/props/kit_fences/sheet_v3.jpg` (the style frame as
reference blends its scene into the sheet). Status: **brief** (not yet generated — Gemini monthly spending cap reached 2026-09-26; re-checked the same day: still HTTP 429 RESOURCE_EXHAUSTED).

## Purpose

The vegetable garden where the player harvests treats (carrots, potatoes) into a basket and
brings them to the animals (GAME-GARDEN). Plants grow in 3 visible stages (GARD-004), so every
stage must read at phone size from the high camera, and **carrot vs. potato plants must be
clearly different from above** (tall feathery light-green tuft vs. low round broad-leaved
darker-green bush). Sign texts come from Fluent (GARD-009), so the `garden_sign` board is blank.

Props are modelled once and reused (ART-ENVIRONMENT, modular props). Budgets: props
≤ 500 triangles each, flat-colour textures only (ART-PIPELINE §10).

## Props in this kit

| Asset id | Size (game units, 1 = 1 m) | Notes |
|---|---|---|
| `garden_bed` | 1 m × 3 m, ≈ 0.25 m high | brown soil bed with wooden plank edge, modular |
| `carrot_plant` | ≈ 0.1 / 0.3 / 0.45 m | 3 stages: sprout → leafy → ripe (orange top visible) |
| `potato_plant` | ≈ 0.1 / 0.4 / 0.5 m | 3 stages: sprout → bush → bush with small white flowers |
| `carrot` | ≈ 0.25 m long | harvested item |
| `potato` | ≈ 0.08 m each | harvested item, 2–3 together |
| `basket` | ≈ 0.4 m wide | woven, handle, carrots + potatoes visible (carried) |
| `garden_fence` | 2 m × ≈ 0.6 m | low white picket fence segment (+ corner) |
| `garden_gate` | ≈ 1 m wide | matching picket gate, closed / open |
| `wheelbarrow` | ≈ 1.4 m long | wooden frame, yellow tub, one wheel |
| `watering_can` | ≈ 0.4 m | light blue |
| `garden_sign` | ≈ 0.8 m high | blank board on a stick (text from Fluent) |

## Image settings

- Two asset sheets (the full kit in one grid got too crowded): **plants + items** (prompt 1)
  and **garden furniture** (prompt 2). Each **16:9, 2K**, `gemini-3-pro-image`, reference
  `art/props/kit_fences/sheet_v3.jpg`.
- Generate 2 variants per sheet, pick one each; single props can later be cut out for
  modelling reference.

## Prompt 1 — plants and items (`sheet_plants_vN`)

First paragraph = STYLE block from `art/style/style.md`, verbatim.

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Game asset sheet: the props below are laid out in a neat grid with generous empty space between them, each prop isolated and complete, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a soft small contact shadow under each prop. Every prop is shown from the same elevated three-quarter top-down view as in a cozy zoo park simulation game (camera looking down at about 55 degrees, isometric-like, narrow field of view so vertical lines stay nearly parallel), all at the same real-world scale (a child is about 1.2 m tall for reference, not shown).

Vegetable garden plants and harvested items, in three rows. Row 1, carrot plant growth stages, each standing in a small round patch of brown soil: 1) a tiny carrot sprout, two or three thin feathery bright-green leaves; 2) a leafy young carrot plant, a fan-shaped tuft of fine feathery, lacy bright-green carrot leaves; 3) a ripe carrot plant, a big tall tuft of feathery bright-green leaves with the chunky bright orange top of the carrot clearly visible poking out of the soil. Row 2, potato plant growth stages, each standing in a small round mound of brown soil: 4) a small potato sprout, a few broad, round, dark-green leaves; 5) a round bushy potato plant with many broad, flat, oval dark-green leaves, low and wide; 6) the same bushy potato plant, slightly bigger, dotted with small white flowers with yellow centres. The carrot plants are tall, thin, feathery and light green; the potato plants are low, round, broad-leaved and darker green, so the two are clearly different when seen from above. Row 3, harvested items: 7) one harvested carrot lying on its side, bright orange with a green leafy top; 8) three small round light-brown potatoes lying together; 9) a woven light-brown wicker basket with a sturdy round handle arching over it, filled with orange carrots with green tops and light-brown potatoes, both clearly visible.
```

## Prompt 2 — garden furniture (`sheet_furniture_vN`)

First paragraph = STYLE block from `art/style/style.md`, verbatim.

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Game asset sheet: the props below are laid out in a neat grid with generous empty space between them, each prop isolated and complete, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a soft small contact shadow under each prop. Every prop is shown from the same elevated three-quarter top-down view as in a cozy zoo park simulation game (camera looking down at about 55 degrees, isometric-like, narrow field of view so vertical lines stay nearly parallel), all at the same real-world scale (a child is about 1.2 m tall for reference, not shown).

Vegetable garden furniture, in a 3 by 3 grid: 1) a long rectangular raised garden bed, about 1 m wide and 3 m long, a low border of warm-brown wooden planks filled with dark-brown, finely furrowed soil, empty (nothing growing); 2) a straight segment of low white wooden picket fence, knee-high, pointed pickets on two horizontal rails, with sturdy posts; 3) a matching low white picket garden gate between two posts with a small ball on top of each post, the gate closed; 4) the same garden gate swung open; 5) a corner piece of the low white picket fence; 6) a friendly wooden wheelbarrow with a sunny yellow tub, one big round wheel at the front and two handles at the back, empty; 7) a round metal watering can in cheerful light blue with a long spout, a sprinkler rose at its tip and a curved handle on top; 8) a garden sign: a small blank cream-coloured wooden board on a single wooden stick pushed into the ground, board tilted slightly back so it faces the high camera, completely blank; 9) a second garden bed module like 1) shown from its short end, to show the modular join.
```

### Negative prompt

```text
text, letters, words, numbers, writing, captions, writing on signs, watermark, signature, logo, brand names, UI, HUD, scenery, landscape, horizon, sky, characters, people, animals, hands, overlapping props, cropped props, perspective distortion, fisheye, eye-level view, dark, gloomy, dirty, broken, rotten vegetables, rubbish, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Comic style matches the approved prop sheets (outlines, flat colours, one shadow tone).
- [ ] All props present, same scale, same camera angle, none cropped or overlapping.
- [ ] Carrot and potato plants clearly different from above in every stage.
- [ ] 3 growth stages clearly distinguishable; ripe carrot shows the orange top.
- [ ] No text anywhere; garden sign blank.
- [ ] Readable at phone size from the high camera; buildable as low-poly props (≤ 500 tris).

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
