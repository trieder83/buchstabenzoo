# Brief — `loc_sand`

Spec: ART-ENVIRONMENT, CONT-MISSIONS §1, GAME-LEVEL-1 ("Hiding places"). Status: **brief — images not generated yet**. Layout: `layout.md` in this
folder; level data `assets/levels/level-1.toml` (`[[hiding_place]]` / `[[scenery]]`, proposal Q-080).

## Purpose

Zebra hiding place `loc_sand` (level 1, north). Where the zebras take a dust bath when the riddle says *"Mein Fell juckt. Ich wälze mich gern. Der Boden ist gelb."* (klasse1) — dry yellow sand, no grass. Shown in the **found** state. One of three candidate hiding places of the zebra (discovery, user decision 2026-09-26); the approved image is also cropped for the `kiga` picture of this place.

## Must be visible

- **Sand, yellow and dry:** a 7 × 4 m patch of warm yellow sand, a few small grey rocks at its rim, a little dust puff where a zebra rolls.
- **No grass:** not a single tuft on the sand — a clear edge to the green lawn.
- One zebra rolling on its back in the sand, one standing next to it shaking off dust.
- The wooden fence of the panda enclosure along the bottom edge, the tall north hedge along the top, the river at the right edge.

## Must not appear

- Not a playground sandpit: no wooden border frame, no toys, no buckets.
- No water on or at the sand, no mud (brown/wet belongs to `loc_mud`).

## Props used

Modular (ART-ENVIRONMENT list): `sand_tile`, `rock`, `grass_tuft`, `fence_wood`, `hedge`, `water_tile_flowing`.
Hiding places shown: `loc_sand`.

## Mood

Warm, dry, playful: sunny sand, funny rolling zebra.

## Image settings

| File | Aspect / size | Camera (level coordinates, see `layout.md`) |
|---|---|---|
| `overview.png` | 16:9, 1920 × 1080 (SDXL: 1344 × 768) | Camera yaw north (image top = north), pitch ≈ 62°, target (2, 43), about 20 m from the target; frame covers the panda fence (bottom), the sand, the north hedge (top), `path_north` (left edge) and the river (right edge). |
| `player_view.png` | 9:16, 1080 × 1920 (SDXL: 768 × 1344) | Player on the grass at (−4.5, 43.5); camera yaw east (image top = east), pitch ≈ 55°, ≈ 14 m from the player; the sand patch fills the upper half. |
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

A bare patch of warm yellow sand in a small zoo seen from high above, about 7 m wide and 4 m deep, with a clean edge to the green lawn around it and a few small grey rocks at its rim; no grass grows on it. One striped zebra rolls on its back in the sand, kicking up a small cartoon dust puff; a second zebra stands beside it shaking off dust. Along the bottom of the image runs the wooden post-and-rail fence of another enclosure; a tall dark-green hedge closes the top; a small flowing river is at the right edge and a sandy path at the left edge. The player character, a small girl about 7 years old (1.2 m tall, head about one third of her body height), long straight dark-brown hair down her back, white T-shirt with four horizontal blue stripes, brown belt, blue jeans, dark-brown shoes walks across the lawn on the left towards the sand. Zoo world: light sand-beige paths of square paving blocks, wooden post-and-rail fences, tall dark-green hedges about 3 m high, round-topped trees, short green grass.

All signs and boards are blank: plain wooden or cream-coloured panels without any letters, words or numbers. The only markings allowed are a simple solid black animal silhouette on an enclosure sign and simple pictogram icons (shovel, padlock) on barrier signs. Enclosure signs and info boards are tilted back towards the camera so their faces are clearly visible from above.
```

### `player_view.png` (9:16 portrait)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

In-game view of a cozy zoo park simulation game, vertical phone-screen composition: a high follow camera looking down at about 55 degrees from about 14 m away, isometric-like perspective with a narrow field of view so vertical lines stay nearly parallel, no horizon, no sky. The small girl is seen from above near the centre of the image, her figure about one twelfth of the image height; the area around her, about 12 m across, is clearly laid out and readable: paths, fences and enclosures easy to tell apart.

Seen from above: the player character, a small girl about 7 years old (1.2 m tall, head about one third of her body height), long straight dark-brown hair down her back, white T-shirt with four horizontal blue stripes, brown belt, blue jeans, dark-brown shoes, stands on the green lawn in the centre of the image. The upper half of the image is a bare patch of warm yellow sand without any grass, a few small grey rocks at its rim; one striped zebra rolls on its back in the sand with a small cartoon dust puff, another zebra stands beside it and looks at the girl. A tall dark-green hedge runs along one side, a wooden fence along the other. Zoo world: light sand-beige paths of square paving blocks, wooden post-and-rail fences, tall dark-green hedges about 3 m high, round-topped trees, short green grass.

All signs and boards are blank: plain wooden or cream-coloured panels without any letters, words or numbers. The only markings allowed are a simple solid black animal silhouette on an enclosure sign and simple pictogram icons (shovel, padlock) on barrier signs. Enclosure signs and info boards are tilted back towards the camera so their faces are clearly visible from above.
```

### `player_view_landscape.png` (optional, 16:9)

Same as `player_view.png`, but replace the camera paragraph with:

```text
In-game view of a cozy zoo park simulation game, horizontal widescreen composition: a high follow camera looking down at about 55 degrees from about 16 m away, isometric-like perspective with a narrow field of view so vertical lines stay nearly parallel, no horizon, no sky. The small girl is seen from above near the centre of the image, her figure about one twelfth of the image height; the area around her, about 20 m across, is clearly laid out and readable.
```

### Negative prompt (all images of this area)

```text
text, letters, words, numbers, writing, captions, writing on signs, watermark, signature, logo, brand names, UI, HUD, buttons, photorealistic, realistic photo, photograph, realistic fur, hyper-detailed textures, dark, gloomy, night, fog, horror, scary, angry or menacing animals, sharp teeth, blood, gore, injury, dead animals, weapons, cages, cage bars, prison, rubbish, litter, crowds, clutter, distorted anatomy, extra legs, extra heads, fisheye distortion, tilted horizon, blurry, low resolution, jpeg artefacts, cropped main subject, sky, horizon, clouds, low camera angle, eye-level view, close-up, strong perspective distortion, sandpit frame, toys, buckets, spades, water, puddle, mud, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

Tools without a negative-prompt field: add "No text anywhere, no logos, not photorealistic, nothing scary." at the end of the prompt.

## Review checklist (before `concept_approved = true` — user decides)

- [ ] All riddle details readable from above: **yellow**, **dry** sand, **no grass** on it, zebras rolling.
- [ ] Clearly different from `loc_meadow` and `loc_river`; does not look like a playground sandpit or like mud.
- [ ] Style matches the approved style frame (Q-010).
- [ ] Layout matches `layout.md` and the level-1 map (positions, sizes, what is left/right).
- [ ] All signs, boards and labels blank — only animal silhouettes on enclosure signs; no text artefacts anywhere.
- [ ] Bright, friendly, nothing scary; no cages.
- [ ] High camera as specified (pitch, girl ≈ 1/12 of the image height near the centre, no sky); verticals nearly parallel.
- [ ] Readable from the high camera at phone size (view the image at ~25 %): the key riddle details still clear.
- [ ] Every prop visible is in the ART-ENVIRONMENT modular list or listed as a unique model in `layout.md`.

## Generation log

*(empty — no image generated yet)*
