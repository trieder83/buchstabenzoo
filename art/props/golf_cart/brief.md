# Brief — `golf_cart` (Zoo golf cart)

Spec: GAME-CART (`specs/10-gameplay/golf-carts.md` rule 11), ART-PIPELINE (concept first; APIPE-010), ART-RIG (`drive` sitting clip of `player_girl`/`player_boy`). Style: `art/style/style.md` (comic) — style reference image: `art/props/kit_nature/sheet_v1.jpg` (approved prop sheet). Status: **in-review** (concept sheets generated 2026-10-06; `concept_approved` stays false until a human approves).

## Purpose

The 3 golf carts the child drives between places (GAME-CART). Look decision (user, 2026-10-06, changed the same day): **red-white — red body panels, white roof/canopy with red stripes** (not green: the zoo already has a lot of green). The green sheets v1/v2 and the green close-up are **superseded** (files kept). Today a placeholder box; modelling waits for approval of this concept.

## Size and fit

Cart about 2.6 m long x 1.4 m wide x 2.0 m high (roof top), seat height ~0.5 m, steering wheel ~0.45 m in front of the left seat; room for 2 children (player characters are 1.20 m tall standing, ART-CHARACTERS §2; seated ~0.85 m to the top of the head, so the roof must clear ~1.0 m above the seat). Cargo bed ~0.8 m x 0.9 m behind the seats where carried items (food boxes, basket, fish bowl) are visible. Budget: <= 1 000 triangles (props), flat colours, outlines from the renderer. Needed parts for the model: body, 4 wheels (may spin), striped roof + 4 posts, 2-seat bench, steering wheel (turns), headlights (glow at night, GAME-NIGHT), tail lights, cargo bed + basket, 2 blank emblem discs (zoo logo later).

## Image settings

- Sheet: **21:9, 2K**, `tools/gen_image.py <this brief> --prompt 1 --out sheet_v1.jpg sheet_v2.jpg --aspect 21:9 --size 2K --ref art/props/kit_nature/sheet_v1.jpg --extra "<ref note>"` (2 variants; views not split, as for kit prop sheets).
- Close-up: `--prompt 2 --out seat_closeup_v1.jpg --aspect 16:9 --size 2K` with the same reference.
- Reference-note (`--extra`): "The attached image only shows the drawing style of our approved props (line weight, flat colours, hard shadow tone). Do not copy its layout, its objects or its grid; draw the asset described above."

## Prompts

First paragraph of every prompt = STYLE block from `art/style/style.md`, verbatim.

### Prompt 1 — `sheet` (five-view sheet)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Vehicle sheet: ONE zoo golf cart shown in five views side by side in a single row, evenly spaced, the same object in every view, same scale, standing on the same baseline, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a small soft contact shadow under it. Views from left to right: 1) exact front view, orthographic; 2) exact left side view, orthographic; 3) exact back view, orthographic; 4) three-quarter view from the front left, slightly from above; 5) the game-camera view: seen from above at about 55 degrees as in a cozy zoo park simulation game (isometric-like, narrow field of view so vertical lines stay nearly parallel), drawn a little larger than the other views. No text, no letters, no numbers, no writing anywhere.

A cheerful little electric ZOO GOLF CART, about 2.6 m long, 1.4 m wide and 2.0 m high to the top of the roof, with chunky cartoon proportions: a rounded body painted bright cheerful RED with a white lower trim band and white mudguards, four fat black rubber wheels with light grey hub caps (the front wheels slightly smaller), a flat canopy ROOF on four slim white posts with bold red and white vertical STRIPES (white roof, red stripes) and a scalloped front and back edge, two side-by-side bench SEATS in warm cream with a red cushion (room for two children of 1.2 m height; the seat is about 0.5 m above the ground), a round STEERING WHEEL on a short column in front of the left seat with a small dashboard, a small windscreen frame without glass in front, two big round yellow HEADLIGHTS and a little round horn at the front, a small red tail light pair at the back, a small open CARGO BED behind the seats (wooden planks, low sides, about 0.8 m wide) holding a woven wicker basket, and a small empty round white emblem circle on each side of the body (left blank for a zoo logo, no drawing inside it). Open sides without doors so a child can hop in. No driver, no people, no animals, no real car brand look.
```

### Prompt 2 — `seat_closeup` (seat / steering close-up for the `drive` pose)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Detail close-up: ONE large illustration of the front seat area of the same zoo golf cart for a driving pose, in two views side by side with the same scale, on a plain light grey background (#E6E6E6) with only a small soft contact shadow, nothing cropped: 1) exact side view, orthographic, of the front part of the cart (roof, windscreen frame, steering wheel, dashboard, bench seat, floor, front wheel) with a plain grey SIZE-REFERENCE silhouette of a seated child (1.2 m tall when standing, simple featureless rounded figure, hands on the steering wheel, feet on the floor board, sitting upright) drawn semi-transparent inside the cart to show the fit; 2) three-quarter view from the front left, slightly from above, of the same seat area WITHOUT the silhouette, showing the round steering wheel with a small round centre hub, the dashboard with two simple round dials, a big round button for the horn, the cream bench seat with red cushion and the floor with a little foot pedal. The cart is bright red and white with a white roof with red stripes. No text, no letters, no numbers, no writing anywhere.
```

### Prompt 3 — `sheet_v3_red` (five-view sheet, RED-WHITE; replaces prompt 1)

Colour decision 2026-10-06 (user): red and white, not green (the zoo already has a lot of green). Layout = sheet_v1 (use it as `--ref` together with the style reference).

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Vehicle sheet: ONE zoo golf cart shown in five views side by side (the fifth view is required and must be clearly visible) in a single row, evenly spaced, the same object in every view, same scale, standing on the same baseline, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a small soft contact shadow under it. Views from left to right: 1) exact front view, orthographic; 2) exact left side view, orthographic; 3) exact back view, orthographic; 4) three-quarter view from the front left, slightly from above; 5) the game-camera view: seen from above at about 55 degrees as in a cozy zoo park simulation game (isometric-like, narrow field of view so vertical lines stay nearly parallel), drawn a little larger than the other views. No text, no letters, no numbers, no writing anywhere.

A cheerful little electric ZOO GOLF CART, about 2.6 m long, 1.4 m wide and 2.0 m high to the top of the roof, with chunky cartoon proportions: a rounded body painted bright CHERRY RED with a white lower trim band and white mudguards, four fat black rubber wheels with light grey hub caps (the front wheels slightly smaller), a flat canopy ROOF on four slim white posts with bold red and white vertical STRIPES (white canopy with red stripes) and a scalloped front and back edge, two side-by-side bench SEATS in warm cream with a red cushion (room for two children of 1.2 m height; the seat is about 0.5 m above the ground), a round STEERING WHEEL on a short column in front of the left seat with a small dashboard, a small windscreen frame without glass in front, two big round yellow HEADLIGHTS and a little round horn at the front, a small red tail light pair at the back, a small open CARGO BED behind the seats (wooden planks, low sides, about 0.8 m wide) holding a woven wicker basket, and a small empty round white emblem circle on each side of the body (left blank for a zoo logo, no drawing inside it). Open sides without doors so a child can hop in. No driver, no people, no animals, no real car brand look.

No green anywhere on the cart: the body panels are red, the roof is white with red stripes.
```

### Prompt 4 — `seat_closeup_v2_red` (close-up, RED-WHITE, whole cart visible; replaces prompt 2)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Detail close-up: ONE large illustration of the front seat area of the same zoo golf cart for a driving pose, in two views side by side with the same scale, on a plain light grey background (#E6E6E6) with only a small soft contact shadow, nothing cropped: 1) exact side view, orthographic, of the WHOLE cart from front bumper to cargo bed, fully visible and not cropped (roof, windscreen frame, steering wheel, dashboard, bench seat, floor, both wheels, cargo bed with basket) with a plain grey SIZE-REFERENCE silhouette of a seated child (1.2 m tall when standing, simple featureless rounded figure, hands on the steering wheel, feet on the floor board, sitting upright) drawn semi-transparent inside the cart to show the fit; 2) three-quarter view from the front left, slightly from above, of the same seat area WITHOUT the silhouette, showing the round steering wheel with a small round centre hub, the dashboard with two simple round dials, a big round button for the horn, the cream bench seat with green cushion and the floor with a little foot pedal. The cart is cherry red and white with a white roof with red stripes. No text, no letters, no numbers, no writing anywhere.

No green anywhere on the cart. Both views must be completely inside the image, nothing cropped at any edge.
```

### Negative prompt (all prompts)

```text
people, driver, passengers, animals, text, letters, numbers, logos, brand badges, doors, glass reflections, a realistic car, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-10-06 | sheet_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: kit_nature/sheet_v1.jpg | superseded (green) — 4 views (front, side, back, 3/4; game view missing), one 2-seat bench, scalloped striped roof, basket in cargo bed; no steering wheel detail in back view, windscreen frame thin |
| 2026-10-06 | sheet_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: kit_nature/sheet_v1.jpg | superseded (green), alternative — striped posts, but ghost sketches at the top, two separate seat rows (4 seats, contradicts 2 seats), brown cargo box, roof stripes less readable |
| 2026-10-06 | seat_closeup_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 2 + negative as 'Avoid' + extra text, ref: kit_nature/sheet_v1.jpg | superseded (green) — good: seated 1.2 m silhouette fits under the roof, hands on the wheel, feet on the floor; right half is cropped and the cart has a green/white nose (differs slightly from sheet v1); dashboard with 2 dials + horn button |
| 2026-10-06 | sheet_v3_red.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 3 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg, sheet_v1.jpg | generated, to review |
| 2026-10-06 | seat_closeup_v2_red.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 4 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg, sheet_v1.jpg | generated, to review |
| 2026-10-06 | sheet_v3_red.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 3 (red-white) + extra text, refs: sheet_v1.jpg (layout), kit_nature/sheet_v1.jpg (style) | **recommended** — red body, white roof with red stripes and scalloped edge, single 2-seat bench, basket, yellow headlights, emblem discs; 4 views again (front, side, back, 3/4 from above), no separate 55 degree view; back view shows the empty cargo bed with the basket; the roof/posts are white so the cart reads clearly against grass |
| 2026-10-06 | seat_closeup_v2_red.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 4 (red-white) + extra text, refs: sheet_v1.jpg, kit_nature/sheet_v1.jpg | good — left side view shows the whole cart (nose to cargo bed) with the 1.2 m seated silhouette fitting under the roof, hands on the wheel, feet on the pedal; right 3/4 view is still cropped at the right/bottom edge but shows steering wheel, dashboard with dials + horn button, bench seat, pedals; body tone is a slightly darker crimson than the sheet |
