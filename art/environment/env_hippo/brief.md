# Brief — `env_hippo`

Spec: ART-ENVIRONMENT, GAME-LEVEL-1. Status: **brief — images not generated yet**. Layout: `layout.md` in this
folder; level data `assets/levels/level-1.toml`.

## Purpose

Start of the hippo mission: sign + info board (riddle → still pond with lilies, food word → melons). Empty at level start.

## Must be visible

- Wooden fence, 11 × 12 m, gate on the west side facing the ring path, framed left and right by tall hedges.
- Enclosure sign above the gate — **blank except for a black hippo silhouette**; info board beside the gate, **blank**.
- Inside: a **square pool with a tiled light-stone edge** (clearly man-made), big grey stones, a wooden hut with a pitched roof (cf. `ref-enclosure-buildings.jpg`), grass and mud patches.
- Behind the enclosure (north): the river bend flowing east (visible current) — a deliberate contrast to the still pond the riddle describes.

## Must not appear

- **No water lilies, frogs or reeds** in the pool (must not look like `loc_pond`).
- No hippos (escaped). Optional `home.png` with hippos inside.

## Props used

Modular (ART-ENVIRONMENT list): `fence_wood`, `enclosure_sign`, `info_board`, `hedge`, `rock`, `grass_tuft`, `bush`, `path_tile`, `water_tile`, `water_tile_flowing`.
Unique models (built once for this area): `hut_wood`, `pool_tiled`.

## Mood

Warm and sunny, a bit muddy and splashy; the empty pool looks inviting but abandoned.

## Image settings

| File | Aspect / size | Camera (level coordinates, see `layout.md`) |
|---|---|---|
| `overview.png` | 16:9, 1920 × 1080 (SDXL: 1344 × 768) | Camera yaw east (image top = east), pitch ≈ 62°, target (14, 17), about 25 m from the target; frame covers the enclosure, the ring path and the river bend. |
| `player_view.png` | 9:16, 1080 × 1920 (SDXL: 768 × 1344) | Player on `path_ring_e` at (6.5, 16.5) next to the board; camera yaw east (image top = east), pitch ≈ 55°, ≈ 14 m from the player (west of her, above the grove). |
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

An empty hippo enclosure in a small zoo seen from high above: a wooden post-and-rail fence around grass and mud patches, a square swimming pool with a light stone tiled edge and clear blue water, several big grey stones, and a wooden hut with a pitched roof. The gate in the fence at the bottom of the image faces a sandy paved path and is framed by two tall green hedges; beside the gate a large wooden enclosure sign with a black hippo silhouette and a small wooden info board on a post. Along the left edge of the image a narrow river flows past with visible ripples. the player character, a small girl about 7 years old (1.2 m tall, head about one third of her body height), long straight dark-brown hair down her back, white T-shirt with four horizontal blue stripes, brown belt, blue jeans, dark-brown shoes, stands on the path in front of the info board. No animals. Zoo world: light sand-beige paths of square paving blocks, wooden post-and-rail fences, tall dark-green hedges about 3 m high, round-topped trees, short green grass.

All signs and boards are blank: plain wooden or cream-coloured panels without any letters, words or numbers. The only markings allowed are a simple solid black animal silhouette on an enclosure sign and simple pictogram icons (shovel, padlock) on barrier signs. Enclosure signs and info boards are tilted back towards the camera so their faces are clearly visible from above.
```

### `player_view.png` (9:16 portrait)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

In-game view of a cozy zoo park simulation game, vertical phone-screen composition: a high follow camera looking down at about 55 degrees from about 14 m away, isometric-like perspective with a narrow field of view so vertical lines stay nearly parallel, no horizon, no sky. The small girl is seen from above near the centre of the image, her figure about one twelfth of the image height; the area around her, about 12 m across, is clearly laid out and readable: paths, fences and enclosures easy to tell apart.

Seen from above: the player character, a small girl about 7 years old (1.2 m tall, head about one third of her body height), long straight dark-brown hair down her back, white T-shirt with four horizontal blue stripes, brown belt, blue jeans, dark-brown shoes, stands on a sandy paved path in the centre of the image, next to a small wooden info board on a post and the wooden gate of an empty hippo enclosure; two tall green hedges frame the gate. Beside the gate a large wooden enclosure sign with a black hippo silhouette. The upper half of the image shows the enclosure behind its fence: a square pool with a light stone tiled edge, big grey stones and a wooden hut with a pitched roof; no animals. At the upper left corner a glimpse of a flowing river; at the bottom edge the tops of tall trees. Zoo world: light sand-beige paths of square paving blocks, wooden post-and-rail fences, tall dark-green hedges about 3 m high, round-topped trees, short green grass.

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

- [ ] Pool is square and tiled — nobody could mistake it for the lily pond, even from above.
- [ ] Sign silhouette = hippo; board blank; both readable from above.
- [ ] Hedges frame the gate.
- [ ] Enclosure empty.
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
