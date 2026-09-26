# Brief — `loc_river`

Spec: ART-ENVIRONMENT, CONT-MISSIONS §1, GAME-LEVEL-1. Status: **in-review — first mockups generated 2026-09-26**. Layout: `layout.md` in this
folder; level data `assets/levels/level-1.toml`.

## Purpose

Where the zebras drink (riddle: *"Ich suche fließendes Wasser … wo das Wasser fließt und Enten schwimmen … dort, wo eine Brücke über das Wasser führt"*). Shown in the **found** state: zebras drinking on the bank.

## Must be visible

- **Flowing water:** a 3 m wide river coming from under the north hedge, flowing towards the viewer and bending east; visible current lines, small rapids with stones and white foam blocks.
- **Bridge:** a wooden arched footbridge crossing the river right next to the zebras.
- **Ducks:** three ducks swimming near the bridge.
- Zebras (2–3) drinking on the west bank, heads lowered to the water.
- Behind the bridge the path continues east to a **fallen tree** lying across it (barrier to level 2), then a tall hedge.
- River leaves the level under the east hedge through a wooden/stone grate.

## Must not appear

- No water lilies, no frogs, no still water (that is the pond).
- Nothing dangerous: gentle river, shallow bank, fence-free but calm.

## Props used

Modular (ART-ENVIRONMENT list): `water_tile_flowing`, `bridge_wood`, `duck`, `rock`, `reed`, `grass_tuft`, `bush`, `tree`, `hedge`, `path_tile`, `fallen_tree`.
Unique models (built once for this area): `river_grate`.
Hiding places shown: `loc_river`.

## Mood

Lively and fresh: sparkling moving water, ducks, a happy discovery moment ("there they are!").

## Image settings

| File | Aspect / size | Camera (level coordinates, see `layout.md`) |
|---|---|---|
| `overview.png` | 16:9, 1920 × 1080 (SDXL: 1344 × 768) | Camera yaw north, pitch ≈ 62°, target (13, 32), about 28 m from the target; frame covers x 2…24, z 22…44. |
| `player_view.png` | 9:16, 1080 × 1920 (SDXL: 768 × 1344) | Player at the ring's north-east corner (6.5, 28.5); camera yaw north, pitch ≈ 55°, ≈ 14 m from the player; the bridge and the zebras (8, 32) are in the upper right of the frame. |
| `player_view_landscape.png` *(optional)* | 16:9, 1920 × 1080 | same as player view |

**Camera (all images):** high-angle view like a cozy zoo park simulation game (Q-049).
`overview.png`: pitch ≈ 60–65°, the whole area in frame. `player_view.png`: the in-game
follow camera, pitch ≈ 55°, ≈ 14 m from the player, narrow FOV (≈ 30–35°) so verticals stay
nearly parallel; the girl is small (≈ 1/12 of the image height) near the centre. No sky,
no horizon. Signs and info boards are tilted back towards the camera.

Decision on aspect ratios: `overview.png` is **16:9 landscape**; `player_view.png` is
**9:16 portrait** because the reference phone viewport is portrait (1080 × 2340, ADIR-001 /
PLAT-002) — the final orientation is Q-048. An optional `player_view_landscape.png` (16:9)
uses the same scene with the landscape camera paragraph below.

**Consistency tips**
- Generate the **style frame** first (`art/environment/style_frame/`) and use the approved
  image as **style reference** (image prompt / style reference / IP-Adapter, medium
  strength) for every mockup.
- Keep the STYLE block and the girl description **word for word**; change only the SCENE.
- Use **one fixed seed per area** for overview and player view; note it in the log below.
- For final mockups use the guided workflow (ART-PIPELINE §7): greybox render from
  `assets/levels/level-1.toml` as depth/edge guide + style reference + this prompt.
- Generate 4 candidates and pick the one that matches the layout best, not the prettiest.

## Prompts

**Style:** the first paragraph of every prompt is the STYLE block from `art/style/style.md`
(verbatim, comic style — Q-010 answered); every negative prompt ends with its NEGATIVE
suffix. Never edit those blocks here — change `art/style/style.md` and all briefs together.

### `overview.png`

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Elevated three-quarter top-down view like a cozy zoo park simulation game: a high camera looking down at about 60 to 65 degrees, isometric-like perspective with a narrow field of view so vertical lines stay nearly parallel, horizontal widescreen composition, the whole area in frame and neatly laid out like a diorama, the ground fills the image, no horizon, no sky.

A narrow river about 3 m wide flows through a sunny corner of a small zoo, seen from high above: it comes from under a tall green hedge at the top of the image, flows down with visible current lines and small rapids of stones and white foam, and bends to the right to leave under the hedge at the right edge through a grate. A small wooden arched footbridge crosses the river; three ducks swim near it. On the grassy left bank just above the bridge, three zebras lower their heads to drink. From the bridge a sandy path leads to the right to a big fallen tree lying across the path in front of the hedge. A group of trees on the far bank. A sandy path runs along the bottom-left; the player character, a small girl about 7 years old (1.2 m tall, head about one third of her body height), long straight dark-brown hair down her back, white T-shirt with four horizontal blue stripes, brown belt, blue jeans, dark-brown shoes, walks along it towards the zebras. Zoo world: light sand-beige paths of square paving blocks, wooden post-and-rail fences, tall dark-green hedges about 3 m high, round-topped trees, short green grass.

All signs and boards are blank: plain wooden or cream-coloured panels without any letters, words or numbers. The only markings allowed are a simple solid black animal silhouette on an enclosure sign and simple pictogram icons (shovel, padlock) on barrier signs. Enclosure signs and info boards are tilted back towards the camera so their faces are clearly visible from above.
```

### `player_view.png` (9:16 portrait)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

In-game view of a cozy zoo park simulation game, vertical phone-screen composition: a high follow camera looking down at about 55 degrees from about 14 m away, isometric-like perspective with a narrow field of view so vertical lines stay nearly parallel, no horizon, no sky. The small girl is seen from above near the centre of the image, her figure about one twelfth of the image height; the area around her, about 12 m across, is clearly laid out and readable: paths, fences and enclosures easy to tell apart.

Seen from above: the player character, a small girl about 7 years old (1.2 m tall, head about one third of her body height), long straight dark-brown hair down her back, white T-shirt with four horizontal blue stripes, brown belt, blue jeans, dark-brown shoes, stands at the corner of a sandy path in the centre of the image. In the upper right part of the image a narrow river flows from the top edge downwards with ripples and small white foamy rapids over stones; a small wooden arched footbridge crosses it, three ducks swim beside the bridge, and three zebras stand on the grassy bank next to the bridge with their heads lowered, drinking. A path leads from the girl's corner over the bridge to the right. Grass and a few trees around. Zoo world: light sand-beige paths of square paving blocks, wooden post-and-rail fences, tall dark-green hedges about 3 m high, round-topped trees, short green grass.

All signs and boards are blank: plain wooden or cream-coloured panels without any letters, words or numbers. The only markings allowed are a simple solid black animal silhouette on an enclosure sign and simple pictogram icons (shovel, padlock) on barrier signs. Enclosure signs and info boards are tilted back towards the camera so their faces are clearly visible from above.
```

### `player_view_landscape.png` (optional, 16:9)

Same as `player_view.png`, but replace the camera paragraph with:

```text
In-game view of a cozy zoo park simulation game, horizontal widescreen composition: a high follow camera looking down at about 55 degrees from about 16 m away, isometric-like perspective with a narrow field of view so vertical lines stay nearly parallel, no horizon, no sky. The small girl is seen from above near the centre of the image, her figure about one twelfth of the image height; the area around her, about 20 m across, is clearly laid out and readable.
```

### Negative prompt (all images of this area)

```text
text, letters, words, numbers, writing, captions, writing on signs, watermark, signature, logo, brand names, UI, HUD, buttons, photorealistic, realistic photo, photograph, realistic fur, hyper-detailed textures, dark, gloomy, night, fog, horror, scary, angry or menacing animals, sharp teeth, blood, gore, injury, dead animals, weapons, cages, cage bars, prison, rubbish, litter, crowds, clutter, distorted anatomy, extra legs, extra heads, fisheye distortion, tilted horizon, blurry, low resolution, jpeg artefacts, cropped main subject, sky, horizon, clouds, low camera angle, eye-level view, close-up, strong perspective distortion, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

Tools without a negative-prompt field: add "No text anywhere, no logos, not photorealistic, nothing scary." at the end of the prompt.

## Review checklist (before `concept_approved = true` — user decides)

- [ ] All three riddle details clearly visible from above: **flowing** water, **bridge**, **ducks**.
- [ ] Obviously different from `loc_pond` (long and moving vs. round and still; bridge and ducks only here).
- [ ] Zebras drinking, calm, friendly, recognisable from above.
- [ ] Fallen tree (overview) reads as "blocked for now", not as danger.
- [ ] Style matches the approved style frame (Q-010).
- [ ] Layout matches `layout.md` and the level-1 map (positions, sizes, what is left/right).
- [ ] All signs, boards and labels blank — only animal silhouettes on enclosure signs; no text artefacts anywhere.
- [ ] Bright, friendly, nothing scary; no cages (enclosures have fences).
- [ ] High camera as specified (pitch, girl ≈ 1/12 of the image height near the centre, no sky); verticals nearly parallel.
- [ ] Readable from the high camera at phone size (view the image at ~25 %): paths, fences, gates, sign silhouettes and the key riddle details still clear.
- [ ] Every prop visible is in the ART-ENVIRONMENT modular list or listed as a unique model in `layout.md`.

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| | | | | | |
| 2026-09-26 | player_view_v1.jpg | gemini-3-pro-image (2K, 9:16) | — | prompt 2 + negative as 'Avoid' + extra text, ref: style_frame.png, sheet_v1.jpg | **chosen** → player_view.png — zebras drinking, bridge, ducks; stray sign with horse silhouette on a side fence (ignore) |
| 2026-09-26 | player_view_v2.jpg | gemini-3-pro-image (2K, 9:16) | — | prompt 2 + negative as 'Avoid' + extra text, ref: style_frame.png, sheet_v1.jpg | alternative — zebra sign on the river bank (misleading) |
| 2026-09-26 | overview_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, sheet_v1.jpg | alternative — river grates look like cage bars, a zebra enclosure sign by the river |
| 2026-09-26 | overview_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, sheet_v1.jpg | **chosen** → overview.png — flowing river with rapids, bridge, 3 zebras drinking, ducks, fallen tree, grates |

**Mockup notes (2026-09-26):** generated with gemini-3-pro-image (2K), chosen image downscaled
to ≤ 2048 px and saved as PNG, the alternative kept as `*_v*.jpg`. References: the style frame
plus the approved kit/model sheets named in the log; `--extra`: "The first attached image is the approved style frame of this game: match its comic style, colours, line weight, outlines and high camera look exactly, but not its layout. The other attached image(s) are approved asset sheets of props that appear in this scene: draw those props exactly like on the sheets (shapes, colours, materials); do not copy the grey sheet background or the sheet layout. No text anywhere."
These are concept mockups from text only (no greybox guide yet) — positions are approximate.
**The level layout may still change:** the level designer is adding more hiding places
(GAME-LEVEL-1 `[[hiding_place]]`), so the mockups must be re-checked against
`specs/10-gameplay/levels/level-1.md` before approval.
