# Brief — `env_entrance`

Spec: ART-ENVIRONMENT, GAME-LEVEL-1. Status: **brief — images not generated yet**. Layout: `layout.md` in this
folder; level data `assets/levels/level-1.toml`.

## Purpose

The first thing the child sees. Spawn point of level 1 (cell (0, 2), facing north). It must feel welcoming and show at a glance where to go: the food storage straight ahead, the ring path left and right, an enclosure fence on the left.

## Must be visible

- Entrance arch gate in the outer zoo wall (south, behind the player), with a **blank** sign board on top; closed turnstiles.
- Stone-block zoo wall with a hedge on top running left and right from the gate.
- Square plaza of sand-beige paving blocks (10 × 8 m).
- Map board on the left edge of the plaza: a picture map made of simple coloured shapes and three animal silhouettes (zebra, hippo, panda) — no text.
- A bench on the right.
- Straight ahead (8 m north): the food storage — a wooden barn with a big open double door; left and right of it tall hedges; behind it the tops of a dense tree grove.
- The ring path crossing in front of the food storage (east–west).
- Left: the south-east corner of the zebra enclosure fence (empty enclosure).
- Right: the west flank of a grey rock hill (the cave mouth is **not** visible — it faces north).
- Optional: two friendly visitors (a parent and a child) at the map board (ART-ENVIRONMENT: "first visitors").

## Must not appear

- No animals in view (all escaped).
- No readable text anywhere — the map board uses pictures only.

## Props used

Modular (ART-ENVIRONMENT list): `path_tile`, `fence_wood`, `hedge`, `bench`, `tree`, `bush`, `grass_tuft`, `rock`, `map_board`, `flower_bed`, `enclosure_sign`.
Unique models (built once for this area): `entrance_arch`, `food_storage_building`, `rock_hill_cave`.

## Mood

Welcoming, sunny late morning, "the adventure starts here"; wide open plaza, clear sight line to the food storage.

## Image settings

| File | Aspect / size | Camera (level coordinates, see `layout.md`) |
|---|---|---|
| `overview.png` | 16:9, 1920 × 1080 (SDXL: 1344 × 768) | Camera yaw north (image top = north), pitch ≈ 62°, target (0, 8), about 30 m from the target; frame covers x -14…16, z -3…20. |
| `player_view.png` | 9:16, 1080 × 1920 (SDXL: 768 × 1344) | Player at the spawn (0.5, 2.5); camera yaw north, pitch ≈ 55°, ≈ 14 m from the player (so ≈ 8 m south and 11.5 m above her, outside the gate). |
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

A small zoo entrance area seen from high above. At the bottom of the image a light stone outer wall with a hedge on top runs across the whole width; in its middle a wooden entrance arch with a blank sign board and closed turnstiles. Above it a square plaza of sand-beige paving blocks with the player character, a small girl about 7 years old (1.2 m tall, head about one third of her body height), long straight dark-brown hair down her back, white T-shirt with four horizontal blue stripes, brown belt, blue jeans, dark-brown shoes, standing in its middle. At the left edge of the plaza a wooden map board, tilted up towards the camera, showing a simple picture map with coloured shapes and three black animal silhouettes (zebra, hippo, panda); a parent and a child visitor stand at it. A wooden bench on the right of the plaza. Above the plaza a sandy ring path crosses from left to right, and above that a wooden food storage barn with a pitched roof and a big open double door facing down towards the plaza, flanked by tall hedges; the tops of a dense grove of tall trees fill the area behind it at the top of the image. On the left an empty zebra enclosure with a wooden fence and a grey stone arch; on the right a large grey rock hill with moss on top, whose visible sides are solid (no opening). Sunny, welcoming. Zoo world: light sand-beige paths of square paving blocks, wooden post-and-rail fences, tall dark-green hedges about 3 m high, round-topped trees, short green grass.

All signs and boards are blank: plain wooden or cream-coloured panels without any letters, words or numbers. The only markings allowed are a simple solid black animal silhouette on an enclosure sign and simple pictogram icons (shovel, padlock) on barrier signs. Enclosure signs and info boards are tilted back towards the camera so their faces are clearly visible from above.
```

### `player_view.png` (9:16 portrait)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

In-game view of a cozy zoo park simulation game, vertical phone-screen composition: a high follow camera looking down at about 55 degrees from about 14 m away, isometric-like perspective with a narrow field of view so vertical lines stay nearly parallel, no horizon, no sky. The small girl is seen from above near the centre of the image, her figure about one twelfth of the image height; the area around her, about 12 m across, is clearly laid out and readable: paths, fences and enclosures easy to tell apart.

Seen from above: the player character, a small girl about 7 years old (1.2 m tall, head about one third of her body height), long straight dark-brown hair down her back, white T-shirt with four horizontal blue stripes, brown belt, blue jeans, dark-brown shoes, stands in the centre of a square plaza of sand-beige paving blocks just inside the zoo entrance. At the bottom edge of the image the top of a wooden entrance arch in a light stone wall with a hedge on top. Left of the girl a wooden map board tilted up towards the camera with a simple picture map and animal silhouettes; right of her a wooden bench. In the upper part of the image a sandy path crosses from left to right, and at the top edge the front of a wooden food storage barn with an open double door and a blank sign board, flanked by tall hedges. At the left edge the corner of a wooden enclosure fence, at the right edge the side of a grey rock hill with moss on top. Zoo world: light sand-beige paths of square paving blocks, wooden post-and-rail fences, tall dark-green hedges about 3 m high, round-topped trees, short green grass.

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

- [ ] Food storage is the clear goal at the top of the view; the gate at the bottom.
- [ ] Map board readable from above as a *picture* map (no text), animal silhouettes recognisable at phone size.
- [ ] Rock hill shows no cave opening from this side.
- [ ] Gate, wall and plaza feel safe and friendly; no animals visible.
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
