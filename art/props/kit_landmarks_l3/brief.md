# Brief — `kit_landmarks_l3` (Landmarks level 3)

Spec: ART-ENVIRONMENT, ART-PIPELINE (concept first), GAME-LEVEL-3, CONT-MISSIONS (monkey, goldfish, snow fox riddles). Style: `art/style/style.md` (comic) — style reference image: `art/props/kit_nature/sheet_v1.jpg` (approved prop sheet; the style frame would blend its scene into a prop sheet). Status: **in-review** (concept sheets generated 2026-10-02; `concept_approved` stays false until a human approves).

## Purpose

The level-3 hiding places of the monkey, goldfish and snow fox (pirate ship, carousel, waterfall, water wheel, willow, ice cream kiosk, washing line) and the stream pieces.

These landmarks are drawn as coloured placeholder boxes in the game today (`placeholder_kind` in `crates/zoo-core/src/scene.rs`). Each sheet shows ONE asset in five views (front, side, back, ¾, and the 55° game-camera view) so it can be modelled from the sheet. Riddle features are what the child must recognise from the high camera. Budgets: landmarks ≤ 1 500 triangles each (ART-PIPELINE §10), flat colours, outlines from the renderer.

## Assets in this kit

| Asset | Level element (rect, m) | Riddle features to recognise | Sheet prompt |
|---|---|---|---|
| `pirate_ship` | pirate_ship, 7 x 4 m (14,56) | ship, mast, sail, flag, treasure chest, rope ladder | prompt 1 → `sheet_pirate_ship_v1/v2.jpg` |
| `carousel` | carousel_sw, 4 x 4 m (-16,52) | carousel, wooden horses, turning striped roof | prompt 2 → `sheet_carousel_v1/v2.jpg` |
| `washing_line` | laundry_line, 5 x 1 m (-19,82) | white sheets, washing line, wind | prompt 3 → `sheet_washing_line_v1/v2.jpg` |
| `willow` | tree_willow, 3 x 3 m (-18,56) | weeping willow, hanging branches, shade on water | prompt 4 → `sheet_willow_v1/v2.jpg` |
| `mill_hut` | mill_hut, 3 x 3 m (-18,70) + wheel r 1.2 m | water wheel, wooden hut, stream | prompt 5 → `sheet_mill_hut_v1/v2.jpg` |
| `waterfall` | waterfall_rocks, 5 x 4 m (-22,88) | falling water, foam, splashing, rock ledge | prompt 6 → `sheet_waterfall_v1/v2.jpg` |
| `ice_cream_kiosk` | ice_cream_kiosk, 4 x 3 m (15,83) | freezer chest, cold air, cones, scoops | prompt 7 → `sheet_ice_cream_kiosk_v1/v2.jpg` |
| `stream_kit` | stream_l3, 3 x 36 m (-22,52) | pebbles, reeds, banks, grate | prompt 8 → `sheet_stream_kit_v1/v2.jpg` |

## Image settings

- One sheet per asset, **21:9, 2K**, `tools/gen_image.py <this brief> --prompt N --out sheet_<id>_v1.jpg sheet_<id>_v2.jpg --aspect 21:9 --size 2K --ref art/props/kit_nature/sheet_v1.jpg --extra "<ref note>"`.
- Reference-note (`--extra`): "The attached image only shows the drawing style of our approved props (line weight, flat colours, hard shadow tone). Do not copy its layout, its objects or its grid; draw the asset described above in five views in one row."
- 2 variants per asset; the better one is picked by looking at both and marked in the log. Views are not split (kit prop sheets are not split in this project).

## Prompts

First paragraph of every prompt = STYLE block from `art/style/style.md`, verbatim.

### Prompt 1 — `sheet_pirate_ship` (Pirate ship climbing frame)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Landmark sheet: ONE wooden pirate ship climbing frame on bark mulch shown in five views side by side in a single row, evenly spaced, the same object in every view, same scale, standing on the same baseline, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a small soft contact shadow under it. Views from left to right: 1) exact front view, orthographic; 2) exact left side view, orthographic; 3) exact back view, orthographic; 4) three-quarter view from the front left, slightly from above; 5) the game-camera view: seen from above at about 55 degrees as in a cozy zoo park simulation game (isometric-like, narrow field of view so vertical lines stay nearly parallel), drawn a little larger than the other views. No text, no letters, no numbers, no writing anywhere.

A big friendly wooden pirate ship climbing frame for the playground, 7 m long and 4 m wide, built from chunky brown planks: a curved hull with a raised bow and stern, round porthole windows, a deck with a railing, a tall wooden MAST (about 5 m high) with a little round CROW'S NEST on top (a barrel-shaped lookout where a monkey can sit), a big white SAIL (square, slightly curved, plain), a black pirate FLAG on top with a big WHITE PAW PRINT (friendly, absolutely no skull, no bones), a small wooden TREASURE CHEST with gold trim and golden coins on the deck, a ship's wheel at the stern and a hanging ROPE LADDER on the side. It stands on a patch of brown BARK MULCH ground. No pirates, no people, no cannons, no weapons.
```

### Prompt 2 — `sheet_carousel` (Carousel)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Landmark sheet: ONE small carousel with wooden horses and a striped round roof shown in five views side by side in a single row, evenly spaced, the same object in every view, same scale, standing on the same baseline, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a small soft contact shadow under it. Views from left to right: 1) exact front view, orthographic; 2) exact left side view, orthographic; 3) exact back view, orthographic; 4) three-quarter view from the front left, slightly from above; 5) the game-camera view: seen from above at about 55 degrees as in a cozy zoo park simulation game (isometric-like, narrow field of view so vertical lines stay nearly parallel), drawn a little larger than the other views. No text, no letters, no numbers, no writing anywhere.

A small cheerful carousel, 4 m across: a round low wooden platform floor, a central pole in a cylindrical hub decorated with painted golden bands, a round tent-like roof in red, white and blue wedge stripes with a pointed little top and a scalloped edge, four slim golden poles each carrying a chunky WOODEN HORSE (white, brown, grey and cream cartoon toy horses with simple painted saddles and flowing mane, front legs raised in a gentle gallop pose, big friendly eyes, closed mouth), a small step to get on. Painted look, no real animal, no people, no light bulbs with writing.
```

### Prompt 3 — `sheet_washing_line` (Washing line with white sheets)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Landmark sheet: ONE washing line with big white sheets and towels shown in five views side by side in a single row, evenly spaced, the same object in every view, same scale, standing on the same baseline, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a small soft contact shadow under it. Views from left to right: 1) exact front view, orthographic; 2) exact left side view, orthographic; 3) exact back view, orthographic; 4) three-quarter view from the front left, slightly from above; 5) the game-camera view: seen from above at about 55 degrees as in a cozy zoo park simulation game (isometric-like, narrow field of view so vertical lines stay nearly parallel), drawn a little larger than the other views. No text, no letters, no numbers, no writing anywhere.

A washing line of the zookeepers: two chunky wooden posts 5 m apart with a rope between them, hung with BIG PLAIN WHITE SHEETS and two smaller white towels, held by colourful wooden clothes pegs, the sheets billowing and flapping in the wind in soft simple folds (white with one light blue-grey shadow tone), one sheet hanging at the left of the line, two at the middle, a towel at the right; next to the left post a woven wicker LAUNDRY BASKET with a white cloth in it and a small cloth PEG BAG hanging from the line. White sheets must read clearly against green grass (strong dark outline). No clothes with patterns, no text.
```

### Prompt 4 — `sheet_willow` (Weeping willow at the stream)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Landmark sheet: ONE weeping willow tree with long hanging branches shown in five views side by side in a single row, evenly spaced, the same object in every view, same scale, standing on the same baseline, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a small soft contact shadow under it. Views from left to right: 1) exact front view, orthographic; 2) exact left side view, orthographic; 3) exact back view, orthographic; 4) three-quarter view from the front left, slightly from above; 5) the game-camera view: seen from above at about 55 degrees as in a cozy zoo park simulation game (isometric-like, narrow field of view so vertical lines stay nearly parallel), drawn a little larger than the other views. No text, no letters, no numbers, no writing anywhere.

A weeping willow about 6 m high with a thick, slightly leaning brown trunk and a big round dome-shaped crown of LONG HANGING BRANCHES in light yellow-green that fall down in curtains all the way to the ground and into the water (strands drawn as simple bold vertical bundles with pointed leaf tips). It stands on a small piece of grassy stream bank, with a little patch of flat blue stream water in front where the tips of the hanging branches dip into the water and make small ripple rings. Calm and shady, very simple readable silhouette.
```

### Prompt 5 — `sheet_mill_hut` (Mill hut with water wheel)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Landmark sheet: ONE tiny wooden mill hut with a big wooden water wheel in a stream shown in five views side by side in a single row, evenly spaced, the same object in every view, same scale, standing on the same baseline, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a small soft contact shadow under it. Views from left to right: 1) exact front view, orthographic; 2) exact left side view, orthographic; 3) exact back view, orthographic; 4) three-quarter view from the front left, slightly from above; 5) the game-camera view: seen from above at about 55 degrees as in a cozy zoo park simulation game (isometric-like, narrow field of view so vertical lines stay nearly parallel), drawn a little larger than the other views. No text, no letters, no numbers, no writing anywhere.

A tiny cosy wooden MILL HUT, footprint 3 x 3 m, about 3 m high, with plank walls, a steep pitched roof of dark brown shingles, a small door and a small window with shutters, standing on the east bank of a stream. On its west wall, in the water, turns a BIG WOODEN WATER WHEEL with radius 1.2 m and 0.5 m wide: a thick axle sticking out of the hut wall, a wooden hub, eight flat wooden paddles around a round rim with spokes; the lowest paddles dip a third of the radius into a short strip of flat blue stream water, with white foam where the paddles enter and leave the water. No miller, no sacks with writing.
```

### Prompt 6 — `sheet_waterfall` (Waterfall on a rock ledge)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Landmark sheet: ONE rock ledge with a waterfall, foam and the start of the stream shown in five views side by side in a single row, evenly spaced, the same object in every view, same scale, standing on the same baseline, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a small soft contact shadow under it. Views from left to right: 1) exact front view, orthographic; 2) exact left side view, orthographic; 3) exact back view, orthographic; 4) three-quarter view from the front left, slightly from above; 5) the game-camera view: seen from above at about 55 degrees as in a cozy zoo park simulation game (isometric-like, narrow field of view so vertical lines stay nearly parallel), drawn a little larger than the other views. No text, no letters, no numbers, no writing anywhere.

A rock ledge against the north wall of the zoo, 5 m wide and 4 m deep, made of a few big rounded stacked light-grey stones with some green moss patches (like the rock hill of the zoo), 2.5 m high; in the middle a smooth sheet of flat light-blue WATER pours over the edge and FALLS 2.5 m in a few bold vertical streaks into a small round pool at the bottom with a big ring of white FOAM and a few splash droplets; from the pool the narrow stream begins and flows toward the viewer in the front view. The falling water is the clear main feature. No bridge, no ducks, no fish, no lilies.
```

### Prompt 7 — `sheet_ice_cream_kiosk` (Ice cream kiosk)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Landmark sheet: ONE ice cream kiosk with a striped awning, a cone on the roof and a freezer chest shown in five views side by side in a single row, evenly spaced, the same object in every view, same scale, standing on the same baseline, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a small soft contact shadow under it. Views from left to right: 1) exact front view, orthographic; 2) exact left side view, orthographic; 3) exact back view, orthographic; 4) three-quarter view from the front left, slightly from above; 5) the game-camera view: seen from above at about 55 degrees as in a cozy zoo park simulation game (isometric-like, narrow field of view so vertical lines stay nearly parallel), drawn a little larger than the other views. No text, no letters, no numbers, no writing anywhere.

A small cheerful wooden ICE CREAM KIOSK, footprint 4 x 3 m, about 3 m high: cream-coloured plank walls, a serving window in the front with a big striped awning in pink and white stripes with a scalloped edge, a flat roof with a BIG ICE-CREAM-CONE icon on top (a giant three-scoop cone sculpture in pink, mint and vanilla, no text), and in front of the window a white FREEZER CHEST with a clear glass lid and little cold white mist curls rising from it, plus a small wooden CONE STAND holding six waffle cones. No sign with writing, no prices, no people.
```

### Prompt 8 — `sheet_stream_kit` (Stream pieces (banks, pebbles, reeds, grate))

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Landmark sheet: ONE stream kit: a straight narrow clear stream section with banks, pebbles, reeds and the iron grate in the wall shown in five views side by side in a single row, evenly spaced, the same object in every view, same scale, standing on the same baseline, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a small soft contact shadow under it. Views from left to right: 1) exact front view, orthographic; 2) exact left side view, orthographic; 3) exact back view, orthographic; 4) three-quarter view from the front left, slightly from above; 5) the game-camera view: seen from above at about 55 degrees as in a cozy zoo park simulation game (isometric-like, narrow field of view so vertical lines stay nearly parallel), drawn a little larger than the other views. No text, no letters, no numbers, no writing anywhere.

A STREAM KIT, drawn as one short straight section of a narrow clear stream, 3 m wide, as a diorama slab: flat light-blue water with a few pale flow lines, a bed of rounded light PEBBLES visible through the water, low grassy banks with a thin sand-brown edge on both sides, two small clumps of green REEDS with brown cattail heads at the left bank and a few flat stepping stones. In the back view a short piece of grey zoo WALL crosses the stream with a low arched opening closed by a black iron GRATE (vertical bars) where the stream leaves the zoo. No bridge, no ducks, no lily pads, no fish.
```

### Negative prompt (all sheets)

```text
text, letters, words, numbers, writing, captions, writing on signs, watermark, signature, logo, brand names, UI, HUD, scenery, landscape, horizon, sky, people, children, real animals, overlapping views, cropped views, extra views, perspective distortion, fisheye, eye-level view, dark, gloomy, dirty, broken, rubbish, skull, weapons, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

## Review checklist (before `concept_approved = true` — user decides, per asset)

- [ ] Comic style matches the approved props (outlines, flat colours, one shadow tone).
- [ ] Five views of the same object, same scale, nothing cropped, plain grey background, no text.
- [ ] The riddle features are clearly readable, also from the 55° view at phone size.
- [ ] Size and parts match the level spec (rect and description in the table above).
- [ ] Buildable as simple low-poly model (≤ 1 500 tris), no scary or violent details.

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-10-02 | sheet_willow_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 4 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | alternative — grassy slab with water |
| 2026-10-02 | sheet_willow_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 4 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | **chosen** — clean five views, hanging branches dip into the water |
| 2026-10-02 | sheet_carousel_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 2 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | discarded (deleted) — horses blurred, ghost plants |
| 2026-10-02 | sheet_carousel_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 2 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | **chosen** — striped roof, 4 wooden horses, steps (55° view slightly smudged at the roof tip) |
| 2026-10-02 | sheet_washing_line_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 3 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | **chosen** — white sheets, towels, pegs, basket, peg bag (layout wraps to two rows) |
| 2026-10-02 | sheet_washing_line_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 3 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | discarded (deleted) — duplicate rows |
| 2026-10-02 | sheet_pirate_ship_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | discarded (deleted) — ghost in the side view, 4 views only |
| 2026-10-02 | sheet_pirate_ship_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | **chosen** — hull, mast, crow's nest, sail, paw flag, treasure chest, rope ladder, bark mulch |
| 2026-10-02 | sheet_mill_hut_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 5 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | discarded (deleted) — blurred, ghost plants |
| 2026-10-02 | sheet_mill_hut_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 5 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | **chosen** — hut, 8-paddle wheel in a stream strip with foam (ghost grass at the bottom) |
| 2026-10-02 | sheet_waterfall_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 6 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | **chosen** — mossy rock ledge, falling water, foam, stream start (two-row layout) |
| 2026-10-02 | sheet_waterfall_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 6 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | discarded (deleted) — reference ghosts |
| 2026-10-02 | sheet_ice_cream_kiosk_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 7 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | **chosen** — striped awning, cone on the roof, freezer chest with mist, cone stand (55° view hazy) |
| 2026-10-02 | sheet_ice_cream_kiosk_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 7 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | discarded (deleted) — hazy ghost sheet |
| 2026-10-02 | sheet_stream_kit_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 8 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | **chosen** — water, pebbles, reeds, banks, arched grate in the wall |
| 2026-10-02 | sheet_stream_kit_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 8 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | discarded (deleted) — reference ghosts |
