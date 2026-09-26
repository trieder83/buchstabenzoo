# Brief — `env_food_storage`

Spec: ART-ENVIRONMENT, GAME-FEED, GAME-LEVEL-1. Status: **in-review — first mockups generated 2026-09-26**. Layout: `layout.md` in this
folder; level data `assets/levels/level-1.toml`.

## Purpose

Where the child reads the **food box labels** and takes the right food (GAME-FEED). Unlocked from the start in level 1 (proposal, Q-033). Sits in the middle of the ring so every enclosure is close.

## Must be visible

- Wooden barn, 8 m wide × 6 m deep, about 4.5 m high, pitched roof; big double door on the south side (towards the ring path), open.
- Above the door a **blank** sign board (optional icon: a simple crate silhouette), tilted towards the camera.
- Inside: **10 closed wooden food boxes** (Q-047) in one row on low pallets along the back wall, each with a **blank cream label panel** on its top/front, angled so it faces the high camera (labels are rendered by the game — never bake text, ART-ENVIRONMENT §2). Food must not be visible (GAME-FEED §2).
- `player_view.png`: the roof is cut away (simulation-game cut-away view) so the girl and the boxes inside are visible from above — proposal, Q-049.
- Tall hedges left and right of the barn, dense tall trees behind (they hide the rest of the zoo).
- The sandy ring path in front.

## Must not appear

- No padlock or locked door (level 1 is unlocked, Q-033).
- No visible food, no pictures on the boxes (kiga pictures are rendered by the game).

## Props used

Modular (ART-ENVIRONMENT list): `food_box`, `path_tile`, `hedge`, `tree`, `bush`, `grass_tuft`, `flower_bed`.
Unique models (built once for this area): `food_storage_building`.

## Mood

Tidy, warm and inviting like a farm barn; bright daylight falls through the open door onto the row of boxes.

## Image settings

| File | Aspect / size | Camera (level coordinates, see `layout.md`) |
|---|---|---|
| `overview.png` | 16:9, 1920 × 1080 (SDXL: 1344 × 768) | Camera yaw north, pitch ≈ 62°, target (0, 13), about 25 m from the target; frame covers x -10…10, z 6…22. |
| `player_view.png` | 9:16, 1080 × 1920 (SDXL: 768 × 1344) | Player just inside the door at (0.5, 12.5); camera yaw north, pitch ≈ 55°, ≈ 14 m from the player; barn roof cut away. |
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

A wooden food storage barn in the middle of a small zoo seen from high above: 8 m wide with a pitched roof and a big open double door facing the bottom of the image, a blank sign board above the door. Through the open door a few closed wooden crates with blank cream label panels are visible. Tall green hedges stand left and right of the barn; above it in the image a dense grove of tall trees. A sandy paved path runs along the bottom of the image from left to right; the player character, a small girl about 7 years old (1.2 m tall, head about one third of her body height), long straight dark-brown hair down her back, white T-shirt with four horizontal blue stripes, brown belt, blue jeans, dark-brown shoes, walks towards the door. Flower beds beside the door. Zoo world: light sand-beige paths of square paving blocks, wooden post-and-rail fences, tall dark-green hedges about 3 m high, round-topped trees, short green grass.

All signs and boards are blank: plain wooden or cream-coloured panels without any letters, words or numbers. The only markings allowed are a simple solid black animal silhouette on an enclosure sign and simple pictogram icons (shovel, padlock) on barrier signs. Enclosure signs and info boards are tilted back towards the camera so their faces are clearly visible from above.
```

### `player_view.png` (9:16 portrait)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

In-game view of a cozy zoo park simulation game, vertical phone-screen composition: a high follow camera looking down at about 55 degrees from about 14 m away, isometric-like perspective with a narrow field of view so vertical lines stay nearly parallel, no horizon, no sky. The small girl is seen from above near the centre of the image, her figure about one twelfth of the image height; the area around her, about 12 m across, is clearly laid out and readable: paths, fences and enclosures easy to tell apart.

Seen from above in a cut-away view with the roof removed: the inside of a wooden food storage barn with plank walls. the player character, a small girl about 7 years old (1.2 m tall, head about one third of her body height), long straight dark-brown hair down her back, white T-shirt with four horizontal blue stripes, brown belt, blue jeans, dark-brown shoes, stands in the middle of the floor just inside the open double door at the bottom of the image. Along the back wall at the top of the image a row of ten closed wooden crates on low pallets, each with a large blank cream-coloured label panel angled up towards the camera. A few sacks and a broom in a corner, warm daylight, tidy. Outside the walls at the image edges: tall green hedges and the sandy path. Zoo world: light sand-beige paths of square paving blocks, wooden post-and-rail fences, tall dark-green hedges about 3 m high, round-topped trees, short green grass.

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

- [ ] Ten closed boxes, all label panels blank, facing the camera and large enough (at phone size a label is at least ~3 % of the screen height, ADIR-001 — re-check for the high camera, Q-049).
- [ ] No food visible, no text or pictures on the boxes.
- [ ] Door open, no lock.
- [ ] Cut-away view reads clearly (walls visible, roof gone) — tell us if an open-front canopy would read better.
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
| 2026-09-26 | player_view_v1.jpg | gemini-3-pro-image (2K, 9:16) | — | prompt 2 + negative as 'Avoid' + extra text, ref: style_frame.png, sheet_v2.jpg, sheet_v5.jpg | **chosen** → player_view.png — interior with the roof removed (as GAME-PLAYER §2), crates with blank labels (labels on the lids, not the fronts) |
| 2026-09-26 | player_view_v2.jpg | gemini-3-pro-image (2K, 9:16) | — | prompt 2 + negative as 'Avoid' + extra text, ref: style_frame.png, sheet_v2.jpg, sheet_v5.jpg | alternative — fewer crates |
| 2026-09-26 | overview_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, sheet_v2.jpg, sheet_v5.jpg | alternative — barn between hedges, extra fences |
| 2026-09-26 | overview_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, sheet_v2.jpg, sheet_v5.jpg | **chosen** → overview.png — barn with open double door between the two hedges, grove behind, flower beds |

**Mockup notes (2026-09-26):** generated with gemini-3-pro-image (2K), chosen image downscaled
to ≤ 2048 px and saved as PNG, the alternative kept as `*_v*.jpg`. References: the style frame
plus the approved kit/model sheets named in the log; `--extra`: "The first attached image is the approved style frame of this game: match its comic style, colours, line weight, outlines and high camera look exactly, but not its layout. The other attached image(s) are approved asset sheets of props that appear in this scene: draw those props exactly like on the sheets (shapes, colours, materials); do not copy the grey sheet background or the sheet layout. No text anywhere."
These are concept mockups from text only (no greybox guide yet) — positions are approximate.
**The level layout may still change:** the level designer is adding more hiding places
(GAME-LEVEL-1 `[[hiding_place]]`), so the mockups must be re-checked against
`specs/10-gameplay/levels/level-1.md` before approval.
