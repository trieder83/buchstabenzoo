# Brief — `kit_landmarks_l2` (Landmarks level 2)

Spec: ART-ENVIRONMENT, ART-PIPELINE (concept first), GAME-LEVEL-2, CONT-MISSIONS (koala, elephant, giraffe, lion riddles). Style: `art/style/style.md` (comic) — style reference image: `art/props/kit_nature/sheet_v1.jpg` (approved prop sheet; the style frame would blend its scene into a prop sheet). Status: **in-review** (concept sheets generated 2026-10-02; `concept_approved` stays false until a human approves).

## Purpose

The level-2 hiding places of the koala, elephant, giraffe and lion (tree house, tallest tree, blossom tree, fountain, log pile, big ball, lookout tower, train, playground, stage, deckchairs).

These landmarks are drawn as coloured placeholder boxes in the game today (`placeholder_kind` in `crates/zoo-core/src/scene.rs`). Each sheet shows ONE asset in five views (front, side, back, ¾, and the 55° game-camera view) so it can be modelled from the sheet. Riddle features are what the child must recognise from the high camera. Budgets: landmarks ≤ 1 500 triangles each (ART-PIPELINE §10), flat colours, outlines from the renderer.

## Assets in this kit

| Asset | Level element (rect, m) | Riddle features to recognise | Sheet prompt |
|---|---|---|---|
| `zoo_train` | train_se, 8 x 2 m (54,14) | locomotive, chimney, bell, two wagons, rails | prompt 1 → `sheet_zoo_train_v1/v2.jpg` |
| `stage` | stage_ne, 4 x 4 m (58,43) | round stage, pointed roof, drums, xylophone | prompt 2 → `sheet_stage_v1/v2.jpg` |
| `treehouse` | treehouse_e, 3 x 3 m (71,44) | tree house, rope ladder, wooden roof, window | prompt 3 → `sheet_treehouse_v1/v2.jpg` |
| `giant_tree` | tree_giant_e, 2 x 2 m (70,31) | tallest tree, thick trunk, crown above all trees | prompt 4 → `sheet_giant_tree_v1/v2.jpg` |
| `blossom_tree` | tree_blossom_ne, 2 x 2 m (70,55) | pink blossoms, falling petals, bees | prompt 5 → `sheet_blossom_tree_v1/v2.jpg` |
| `log_pile` | log_pile_nw, 4 x 2 m (27,45) | stacked logs, bark, sawdust | prompt 6 → `sheet_log_pile_v1/v2.jpg` |
| `play_ball` | ball_n, 2 x 2 m (50,57) | giant ball, red and white, round | prompt 7 → `sheet_play_ball_v1/v2.jpg` |
| `lookout_tower` | tower_sw, 3 x 3 m (36,15) | wooden tower, stairs, high platform | prompt 8 → `sheet_lookout_tower_v1/v2.jpg` |
| `playground` | playground_se_slide 2 x 3 m + playground_se_swings 4 x 2 m (66,15) | slide, swings | prompt 9 → `sheet_playground_v1/v2.jpg` |
| `deckchairs` | deckchairs_ne, 4 x 2 m (58,56) | striped deckchairs, sunshade | prompt 10 → `sheet_deckchairs_v1/v2.jpg` |
| `fountain` | fountain_sw, 3 x 3 m (27,24) | water jet, stone basin, coins, splashing | prompt 11 → `sheet_fountain_v1/v2.jpg` |

## Image settings

- One sheet per asset, **21:9, 2K**, `tools/gen_image.py <this brief> --prompt N --out sheet_<id>_v1.jpg sheet_<id>_v2.jpg --aspect 21:9 --size 2K --ref art/props/kit_nature/sheet_v1.jpg --extra "<ref note>"`.
- Reference-note (`--extra`): "The attached image only shows the drawing style of our approved props (line weight, flat colours, hard shadow tone). Do not copy its layout, its objects or its grid; draw the asset described above in five views in one row."
- 2 variants per asset; the better one is picked by looking at both and marked in the log. Views are not split (kit prop sheets are not split in this project).

## Prompts

First paragraph of every prompt = STYLE block from `art/style/style.md`, verbatim.

### Prompt 1 — `sheet_zoo_train` (Zoo train at its station)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Landmark sheet: ONE little zoo train with a short piece of track and a small station platform shown in five views side by side in a single row, evenly spaced, the same object in every view, same scale, standing on the same baseline, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a small soft contact shadow under it. Views from left to right: 1) exact front view, orthographic; 2) exact left side view, orthographic; 3) exact back view, orthographic; 4) three-quarter view from the front left, slightly from above; 5) the game-camera view: seen from above at about 55 degrees as in a cozy zoo park simulation game (isometric-like, narrow field of view so vertical lines stay nearly parallel), drawn a little larger than the other views. No text, no letters, no numbers, no writing anywhere.

A toy-like cheerful little zoo train, about 7 m long and 1.6 m wide, standing on a short straight piece of track (two rails with wooden sleepers, 8 m long) next to a small low wooden station platform with a little pitched awning roof on two posts (no sign, no text). The engine is bright red with a short black chimney, a rounded cream boiler, a small cab with a window, big round wheels and a brass BELL hanging on top (clearly visible and big); behind it two open wagons, one blue and one yellow, each with two wooden benches, small wheels, linked by chunky couplings. Engine about 2.2 m high. Chunky friendly rounded shapes, no steam clouds.
```

### Prompt 2 — `sheet_stage` (Round music stage)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Landmark sheet: ONE round wooden music stage with a pointed roof, drums and a xylophone shown in five views side by side in a single row, evenly spaced, the same object in every view, same scale, standing on the same baseline, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a small soft contact shadow under it. Views from left to right: 1) exact front view, orthographic; 2) exact left side view, orthographic; 3) exact back view, orthographic; 4) three-quarter view from the front left, slightly from above; 5) the game-camera view: seen from above at about 55 degrees as in a cozy zoo park simulation game (isometric-like, narrow field of view so vertical lines stay nearly parallel), drawn a little larger than the other views. No text, no letters, no numbers, no writing anywhere.

A round wooden music stage, 4 m across and only 0.5 m high (two wide wooden steps at the front), with four slim wooden posts at the edge carrying a tall pointed conical roof in red and yellow stripes with a little flag on the tip. On the stage stand a drum set (one big red bass drum, two small round drums and a golden cymbal on stands) and a large colourful XYLOPHONE (rainbow-coloured wooden bars on a low wooden frame). No people, no microphones, no speakers, no text.
```

### Prompt 3 — `sheet_treehouse` (Old oak with tree house)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Landmark sheet: ONE old oak tree with a wooden tree house shown in five views side by side in a single row, evenly spaced, the same object in every view, same scale, standing on the same baseline, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a small soft contact shadow under it. Views from left to right: 1) exact front view, orthographic; 2) exact left side view, orthographic; 3) exact back view, orthographic; 4) three-quarter view from the front left, slightly from above; 5) the game-camera view: seen from above at about 55 degrees as in a cozy zoo park simulation game (isometric-like, narrow field of view so vertical lines stay nearly parallel), drawn a little larger than the other views. No text, no letters, no numbers, no writing anywhere.

A thick old oak tree about 7 m high with a wide round green crown and a chunky brown trunk. Resting in the big fork of the trunk at about 3.5 m height is a charming little wooden tree house: plank walls, a small pitched roof of dark brown shingles, a square window with a little shutter, a small door opening and a small wooden platform in front, and a hanging ROPE LADDER with wooden rungs reaching down to the ground. The tree house is clearly the main feature and readable from above. No people, no flags with writing.
```

### Prompt 4 — `sheet_giant_tree` (The tallest tree of the zoo)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Landmark sheet: ONE giant tree, the tallest tree of the zoo shown in five views side by side in a single row, evenly spaced, the same object in every view, same scale, standing on the same baseline, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a small soft contact shadow under it. Views from left to right: 1) exact front view, orthographic; 2) exact left side view, orthographic; 3) exact back view, orthographic; 4) three-quarter view from the front left, slightly from above; 5) the game-camera view: seen from above at about 55 degrees as in a cozy zoo park simulation game (isometric-like, narrow field of view so vertical lines stay nearly parallel), drawn a little larger than the other views. No text, no letters, no numbers, no writing anywhere.

A gigantic ancient broad-leaved tree about 12 m high, twice as tall as every normal tree of the zoo (a normal round tree is 5 m, so this one is drawn with a very tall, very THICK trunk about 1.6 m wide with a few chunky roots, and a huge rounded dark-green crown that clearly towers above everything else, built from a few big overlapping leafy blobs). In the side view only, draw one small ordinary 5 m round tree standing beside the giant tree for size comparison. Simple, strong silhouette.
```

### Prompt 5 — `sheet_blossom_tree` (Blossom tree)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Landmark sheet: ONE tree full of pink blossoms shown in five views side by side in a single row, evenly spaced, the same object in every view, same scale, standing on the same baseline, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a small soft contact shadow under it. Views from left to right: 1) exact front view, orthographic; 2) exact left side view, orthographic; 3) exact back view, orthographic; 4) three-quarter view from the front left, slightly from above; 5) the game-camera view: seen from above at about 55 degrees as in a cozy zoo park simulation game (isometric-like, narrow field of view so vertical lines stay nearly parallel), drawn a little larger than the other views. No text, no letters, no numbers, no writing anywhere.

A round-topped tree about 5 m high with a chunky brown trunk and a big round crown completely covered in soft PINK blossoms (cherry-blossom look, flat pink with a lighter pink and a few white blobs, only a little green leaf), many pale pink petals drifting down in the air around the crown, a roundish carpet of fallen pink petals on the grass around the trunk, and two or three friendly cartoon bees (yellow and black, tiny, with little wings) buzzing near the blossoms. Very cheerful and soft.
```

### Prompt 6 — `sheet_log_pile` (Log pile)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Landmark sheet: ONE pile of stacked tree trunks with sawdust shown in five views side by side in a single row, evenly spaced, the same object in every view, same scale, standing on the same baseline, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a small soft contact shadow under it. Views from left to right: 1) exact front view, orthographic; 2) exact left side view, orthographic; 3) exact back view, orthographic; 4) three-quarter view from the front left, slightly from above; 5) the game-camera view: seen from above at about 55 degrees as in a cozy zoo park simulation game (isometric-like, narrow field of view so vertical lines stay nearly parallel), drawn a little larger than the other views. No text, no letters, no numbers, no writing anywhere.

A neat but rustic pile of stacked tree logs, 4 m long and 2 m wide, about 1.4 m high, stacked in a pyramid of three layers (roughly 6, 5 and 4 logs) lying in the same direction, so the round cut ends with light yellow-brown annual rings face the viewer in the front view; rough dark-brown bark on the sides, a few short wooden wedges holding the pile at both ends, a heap of pale yellow SAWDUST on the ground in front and a couple of wood chips. No axe, no tools, no chopping block.
```

### Prompt 7 — `sheet_play_ball` (Giant play ball)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Landmark sheet: ONE giant red-and-white striped beach ball shown in five views side by side in a single row, evenly spaced, the same object in every view, same scale, standing on the same baseline, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a small soft contact shadow under it. Views from left to right: 1) exact front view, orthographic; 2) exact left side view, orthographic; 3) exact back view, orthographic; 4) three-quarter view from the front left, slightly from above; 5) the game-camera view: seen from above at about 55 degrees as in a cozy zoo park simulation game (isometric-like, narrow field of view so vertical lines stay nearly parallel), drawn a little larger than the other views. No text, no letters, no numbers, no writing anywhere.

A giant round toy ball 1.8 m in diameter (the elephants' play ball), made of curved wedge-shaped panels alternating bright RED and WHITE, with a small white round cap at the top and a small red round cap at the bottom like a beach ball, perfectly round, resting on short green grass with a soft contact shadow. Shiny only through one flat white highlight blob. Nothing else on the sheet but the ball (the five views of a ball look similar, vary the stripe position only slightly).
```

### Prompt 8 — `sheet_lookout_tower` (Wooden lookout tower)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Landmark sheet: ONE wooden lookout tower with stairs and a platform shown in five views side by side in a single row, evenly spaced, the same object in every view, same scale, standing on the same baseline, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a small soft contact shadow under it. Views from left to right: 1) exact front view, orthographic; 2) exact left side view, orthographic; 3) exact back view, orthographic; 4) three-quarter view from the front left, slightly from above; 5) the game-camera view: seen from above at about 55 degrees as in a cozy zoo park simulation game (isometric-like, narrow field of view so vertical lines stay nearly parallel), drawn a little larger than the other views. No text, no letters, no numbers, no writing anywhere.

A wooden lookout tower for visitors, footprint 3 x 3 m, with four slightly slanting chunky timber legs with cross braces, a zig-zag wooden STAIRCASE with a handrail winding up one side, a square platform at 4 m height with a wooden railing all around and a small pitched roof with dark brown shingles on four posts above the platform, overall about 6 m high. No people, no flags with writing, no binoculars.
```

### Prompt 9 — `sheet_playground` (Playground slide and swings)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Landmark sheet: ONE playground slide and double swing standing side by side shown in five views side by side in a single row, evenly spaced, the same object in every view, same scale, standing on the same baseline, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a small soft contact shadow under it. Views from left to right: 1) exact front view, orthographic; 2) exact left side view, orthographic; 3) exact back view, orthographic; 4) three-quarter view from the front left, slightly from above; 5) the game-camera view: seen from above at about 55 degrees as in a cozy zoo park simulation game (isometric-like, narrow field of view so vertical lines stay nearly parallel), drawn a little larger than the other views. No text, no letters, no numbers, no writing anywhere.

TWO playground pieces drawn together as one group in every view, standing next to each other about 0.8 m apart with the slide on the left and the swing on the right: (a) a bright red-and-yellow children's SLIDE 1.8 m high with a short wooden ladder of five rungs, a small platform with a railing and a long smooth curved slide chute; (b) a wooden DOUBLE SWING with an A-frame of two chunky wooden poles and a top beam 2.2 m high, two blue seats hanging from short chains (chains drawn as simple bold lines). Footprints about 2 x 3 m and 4 x 2 m. Friendly toy-like and clear.
```

### Prompt 10 — `sheet_deckchairs` (Deckchairs with sunshade)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Landmark sheet: ONE three striped deckchairs under a big sunshade shown in five views side by side in a single row, evenly spaced, the same object in every view, same scale, standing on the same baseline, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a small soft contact shadow under it. Views from left to right: 1) exact front view, orthographic; 2) exact left side view, orthographic; 3) exact back view, orthographic; 4) three-quarter view from the front left, slightly from above; 5) the game-camera view: seen from above at about 55 degrees as in a cozy zoo park simulation game (isometric-like, narrow field of view so vertical lines stay nearly parallel), drawn a little larger than the other views. No text, no letters, no numbers, no writing anywhere.

THREE classic folding wooden deckchairs standing next to each other in a row (one with blue-and-white stripes, one with red-and-white stripes, one with yellow-and-white stripes), reclined at a comfortable angle, and a big round SUNSHADE (beach parasol) planted in a small base behind them, with a scalloped edge and big alternating white and orange wedge panels, 2.4 m wide, casting a hard-edged shadow over the chairs. No people, no towels with writing, no drinks.
```

### Prompt 11 — `sheet_fountain` (Fountain)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Landmark sheet: ONE round stone fountain with a water jet and coins shown in five views side by side in a single row, evenly spaced, the same object in every view, same scale, standing on the same baseline, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a small soft contact shadow under it. Views from left to right: 1) exact front view, orthographic; 2) exact left side view, orthographic; 3) exact back view, orthographic; 4) three-quarter view from the front left, slightly from above; 5) the game-camera view: seen from above at about 55 degrees as in a cozy zoo park simulation game (isometric-like, narrow field of view so vertical lines stay nearly parallel), drawn a little larger than the other views. No text, no letters, no numbers, no writing anywhere.

A round light-grey stone fountain, 3 m across, with a low chunky circular basin wall about 0.6 m high, filled with flat light-blue water, a central stone pedestal column about 0.9 m high with a small stone bowl on top, and a tall arcing WATER JET shooting up about 1.5 m and falling back in a simple arch of white-blue water drops around it (flat shapes, drawn in a few bold rounded splashes), white splash foam where it lands, and several shining golden COINS (flat gold discs with a white sparkle) on the basin bottom visible through the water. No statues, no animals, no text.
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
| 2026-10-02 | sheet_zoo_train_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | **chosen** — engine, bell, 2 wagons, track and platform all clear |
| 2026-10-02 | sheet_zoo_train_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | alternative (plain wagons, similar) |
| 2026-10-02 | sheet_stage_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 2 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | **chosen** — pointed striped roof, drums and rainbow xylophone clear |
| 2026-10-02 | sheet_stage_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 2 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | discarded (deleted) — smudged edges and stray plants from the reference |
| 2026-10-02 | sheet_treehouse_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 3 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | **chosen** — house, window and rope ladder readable (faint reference ghost bottom left) |
| 2026-10-02 | sheet_treehouse_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 3 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | alternative — cleaner but house hidden under the crown |
| 2026-10-02 | sheet_log_pile_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 6 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | discarded (deleted) — blurred ghost sheet |
| 2026-10-02 | sheet_log_pile_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 6 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | discarded (deleted) — blurred ghost sheet |
| 2026-10-02 | sheet_giant_tree_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 4 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | discarded (deleted) — reference ghosts |
| 2026-10-02 | sheet_giant_tree_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 4 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | discarded (deleted) — too squat for a 12 m tree |
| 2026-10-02 | sheet_play_ball_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 7 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | discarded (deleted) — reference ghosts |
| 2026-10-02 | sheet_play_ball_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 7 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | discarded (deleted) — reference ghosts |
| 2026-10-02 | sheet_blossom_tree_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 5 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | discarded (deleted) — blurry, reference ghosts |
| 2026-10-02 | sheet_blossom_tree_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 5 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | discarded (deleted) — sheet row not clean |
| 2026-10-02 | sheet_deckchairs_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 10 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | discarded (deleted) — only the sunshade, reference ghosts |
| 2026-10-02 | sheet_deckchairs_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 10 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | discarded (deleted) — reference ghosts |
| 2026-10-02 | sheet_playground_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 9 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | discarded (deleted) — blurred ghost sheet |
| 2026-10-02 | sheet_playground_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 9 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | discarded (deleted) — reference ghosts |
| 2026-10-02 | sheet_lookout_tower_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 8 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | **chosen** — zig-zag stairs, 4 m platform, roof, clean |
| 2026-10-02 | sheet_lookout_tower_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 8 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | alternative — stairs hide the legs more |
| 2026-10-02 | sheet_fountain_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 11 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | discarded (deleted) — trees from the reference, no fountain |
| 2026-10-02 | sheet_fountain_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 11 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | discarded (deleted) — ghost fountain |
| 2026-10-02 | sheet_giant_tree_v3.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 4 + negative as 'Avoid' + extra text, ref: sheet_zoo_train_v1.jpg | discarded (deleted) — trunk too short |
| 2026-10-02 | sheet_giant_tree_v4.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 4 + negative as 'Avoid' + extra text, ref: sheet_zoo_train_v1.jpg | **chosen** — very tall thick trunk, crown high; comparison tree in the side view (weak: reads as a plain tall tree, the user may want it even taller) |
| 2026-10-02 | sheet_log_pile_v3.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 6 + negative as 'Avoid' + extra text, ref: sheet_zoo_train_v1.jpg | **chosen** — clean pyramid, rings, bark, sawdust |
| 2026-10-02 | sheet_log_pile_v4.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 6 + negative as 'Avoid' + extra text, ref: sheet_zoo_train_v1.jpg | discarded (deleted) — train fragments from the reference |
| 2026-10-02 | sheet_blossom_tree_v3.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 5 + negative as 'Avoid' + extra text, ref: sheet_zoo_train_v1.jpg | discarded (deleted) — train fragments from the reference |
| 2026-10-02 | sheet_blossom_tree_v4.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 5 + negative as 'Avoid' + extra text, ref: sheet_zoo_train_v1.jpg | **chosen** — pink crown, falling petals, petal carpet, bees |
| 2026-10-02 | sheet_deckchairs_v3.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 10 + negative as 'Avoid' + extra text, ref: sheet_zoo_train_v1.jpg | **chosen** — three striped chairs + sunshade, clean |
| 2026-10-02 | sheet_deckchairs_v4.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 10 + negative as 'Avoid' + extra text, ref: sheet_zoo_train_v1.jpg | discarded (deleted) — train fragments from the reference |
| 2026-10-02 | sheet_playground_v3.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 9 + negative as 'Avoid' + extra text, ref: sheet_zoo_train_v1.jpg | alternative — clean, stray ghost ladder at top |
| 2026-10-02 | sheet_playground_v4.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 9 + negative as 'Avoid' + extra text, ref: sheet_zoo_train_v1.jpg | **chosen** — red/yellow slide + double swing, clean (sand patch instead of grass) |
| 2026-10-02 | sheet_fountain_v3.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 11 + negative as 'Avoid' + extra text | **chosen** — jet, stone basin, coins, foam (generated without reference) |
| 2026-10-02 | sheet_fountain_v4.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 11 + negative as 'Avoid' + extra text | alternative (jet more slender) |
| 2026-10-02 | sheet_play_ball_v3.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 7 + negative as 'Avoid' + extra text | **chosen** — clean, red/white panels, caps (generated without reference) |
| 2026-10-02 | sheet_play_ball_v4.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 7 + negative as 'Avoid' + extra text | alternative (nearly identical) |
