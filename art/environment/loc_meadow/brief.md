# Brief — `loc_meadow`

Spec: ART-ENVIRONMENT, CONT-MISSIONS §1, GAME-LEVEL-1 ("Hiding places"). Status: **brief — images not generated yet**. Layout: `layout.md` in this
folder; level data `assets/levels/level-1.toml` (`[[hiding_place]]` / `[[scenery]]`, proposal Q-080).

## Purpose

Zebra hiding place `loc_meadow` (level 1, north-east). Where the zebras graze when the riddle says *"Ich habe Hunger. Das Gras ist hoch. Dort fliegen Schmetterlinge."* (klasse1) — tall grass, wildflowers, butterflies, big trees behind. Shown in the **found** state. One of three candidate hiding places of the zebra (discovery, user decision 2026-09-26); the approved image is also cropped for the `kiga` picture of this place.

## Must be visible

- **Tall grass:** a 7 × 3 m strip of knee-high grass tufts (reaching the zebras' bellies) — clearly taller than the short lawn everywhere else.
- **Wildflowers:** red, yellow and white flower heads scattered in the tall grass.
- **Butterflies:** 3–4 colourful butterflies fluttering over the grass.
- **Big trees behind:** the dense tree group `trees_ne` right behind (north of) the meadow.
- Two zebras grazing in the tall grass; the narrow stepping-stone trail on the left, the river with the wooden bridge at the left edge, the sandy side path at the bottom.

## Must not appear

- No flower *beds* with wooden borders (it is a wild meadow, not a garden).
- Zebras not drinking, not at the water (that is `loc_river`); no fence around the meadow.

## Props used

Modular (ART-ENVIRONMENT list): `grass_tuft`, `wildflowers`, `butterfly`, `tree`, `path_tile`, `water_tile_flowing`, `bridge_wood`, `hedge`.
Hiding places shown: `loc_meadow`.

## Mood

Sunny, lively, summery: humming butterflies, swaying grass.

## Image settings

| File | Aspect / size | Camera (level coordinates, see `layout.md`) |
|---|---|---|
| `overview.png` | 16:9, 1920 × 1080 (SDXL: 1344 × 768) | Camera yaw north (image top = north), pitch ≈ 62°, target (16, 32), about 22 m from the target; frame covers the bridge (left), the trail, the tall grass, the front of `trees_ne` and the east hedge. |
| `player_view.png` | 9:16, 1080 × 1920 (SDXL: 768 × 1344) | Player on `path_ne` at (16.5, 29.5); camera yaw north (image top = north), pitch ≈ 55°, ≈ 14 m from the player; the tall grass fills the upper half. |
| `player_view_landscape.png` *(optional)* | 16:9, 1920 × 1080 | same as player view |

Camera, aspect ratios and consistency tips: as in `art/environment/loc_pond/brief.md` (high-angle
view like a cozy zoo park simulation game, Q-049; style frame as style reference; one fixed seed
per area; guided workflow from the level-1 greybox, ART-PIPELINE §7).

## Prompts

**Style:** the first paragraph of every prompt is the STYLE block from `art/style/style.md`
(verbatim, comic style — Q-010 answered); every negative prompt ends with its NEGATIVE
suffix. Never edit those blocks here — change `art/style/style.md` and all briefs together.

### `overview.png`

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Elevated three-quarter top-down view like a cozy zoo park simulation game: a high camera looking down at about 60 to 65 degrees, isometric-like perspective with a narrow field of view so vertical lines stay nearly parallel, horizontal widescreen composition, the whole area in frame and neatly laid out like a diorama, the ground fills the image, no horizon, no sky.

A strip of wild meadow in a small zoo seen from high above, about 7 m long and 3 m deep: knee-high green grass tufts, much taller than the short lawn around, with red, yellow and white wildflowers between them and four colourful butterflies fluttering above. Two striped zebras stand belly-deep in the tall grass and graze. Behind the meadow at the top of the image stands a dense group of big round-topped trees. On the left a narrow trail of stepping stones runs up beside a small flowing river with a wooden arched bridge; a sandy path runs along the bottom of the image; a tall dark-green hedge closes the right edge. The player character, a small girl about 7 years old (1.2 m tall, head about one third of her body height), long straight dark-brown hair down her back, white T-shirt with four horizontal blue stripes, brown belt, blue jeans, dark-brown shoes walks along the path at the bottom. Zoo world: light sand-beige paths of square paving blocks, wooden post-and-rail fences, tall dark-green hedges about 3 m high, round-topped trees, short green grass.

All signs and boards are blank: plain wooden or cream-coloured panels without any letters, words or numbers. The only markings allowed are a simple solid black animal silhouette on an enclosure sign and simple pictogram icons (shovel, padlock) on barrier signs. Enclosure signs and info boards are tilted back towards the camera so their faces are clearly visible from above.
```

### `player_view.png` (9:16 portrait)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

In-game view of a cozy zoo park simulation game, vertical phone-screen composition: a high follow camera looking down at about 55 degrees from about 14 m away, isometric-like perspective with a narrow field of view so vertical lines stay nearly parallel, no horizon, no sky. The small girl is seen from above near the centre of the image, her figure about one twelfth of the image height; the area around her, about 12 m across, is clearly laid out and readable: paths, fences and enclosures easy to tell apart.

Seen from above: the player character, a small girl about 7 years old (1.2 m tall, head about one third of her body height), long straight dark-brown hair down her back, white T-shirt with four horizontal blue stripes, brown belt, blue jeans, dark-brown shoes, stands on a sandy path in the centre of the image. The upper half of the image is a wild meadow of knee-high grass tufts with red, yellow and white wildflowers and four colourful butterflies; two striped zebras stand belly-deep in the tall grass and graze, looking up at the girl. Big round-topped trees at the top edge. On the left edge a narrow stepping-stone trail and a glimpse of a flowing river. Zoo world: light sand-beige paths of square paving blocks, wooden post-and-rail fences, tall dark-green hedges about 3 m high, round-topped trees, short green grass.

All signs and boards are blank: plain wooden or cream-coloured panels without any letters, words or numbers. The only markings allowed are a simple solid black animal silhouette on an enclosure sign and simple pictogram icons (shovel, padlock) on barrier signs. Enclosure signs and info boards are tilted back towards the camera so their faces are clearly visible from above.
```

### `player_view_landscape.png` (optional, 16:9)

Same as `player_view.png`, but replace the camera paragraph with:

```text
In-game view of a cozy zoo park simulation game, horizontal widescreen composition: a high follow camera looking down at about 55 degrees from about 16 m away, isometric-like perspective with a narrow field of view so vertical lines stay nearly parallel, no horizon, no sky. The small girl is seen from above near the centre of the image, her figure about one twelfth of the image height; the area around her, about 20 m across, is clearly laid out and readable.
```

### Negative prompt (all images of this area)

```text
text, letters, words, numbers, writing, captions, writing on signs, watermark, signature, logo, brand names, UI, HUD, buttons, photorealistic, realistic photo, photograph, realistic fur, hyper-detailed textures, dark, gloomy, night, fog, horror, scary, angry or menacing animals, sharp teeth, blood, gore, injury, dead animals, weapons, cages, cage bars, prison, rubbish, litter, crowds, clutter, distorted anatomy, extra legs, extra heads, fisheye distortion, tilted horizon, blurry, low resolution, jpeg artefacts, cropped main subject, sky, horizon, clouds, low camera angle, eye-level view, close-up, strong perspective distortion, flower beds, garden borders, water trough, drinking zebras, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

Tools without a negative-prompt field: add "No text anywhere, no logos, not photorealistic, nothing scary." at the end of the prompt.

## Review checklist (before `concept_approved = true` — user decides)

- [ ] All riddle details readable from above: **tall** grass (clearly taller than elsewhere), **wildflowers**, **butterflies**, big trees behind.
- [ ] Clearly different from `loc_river` (zebras graze, not drink) and from `loc_sand` (green and tall vs. yellow and bare).
- [ ] Style matches the approved style frame (Q-010).
- [ ] Layout matches `layout.md` and the level-1 map (positions, sizes, what is left/right).
- [ ] All signs, boards and labels blank — only animal silhouettes on enclosure signs; no text artefacts anywhere.
- [ ] Bright, friendly, nothing scary; no cages.
- [ ] High camera as specified (pitch, girl ≈ 1/12 of the image height near the centre, no sky); verticals nearly parallel.
- [ ] Readable from the high camera at phone size (view the image at ~25 %): the key riddle details still clear.
- [ ] Every prop visible is in the ART-ENVIRONMENT modular list or listed as a unique model in `layout.md`.

## Generation log

*(empty — no image generated yet)*
