# Brief — `env_garden`

Spec: ART-ENVIRONMENT, GAME-GARDEN, GAME-LEVEL-1 ("Vegetable garden"). Status: **brief — images not generated yet**. Layout: `layout.md` in this
folder; level data `assets/levels/level-1.toml` (`[[garden]]`, `[[garden_bed]]`, `[[plant_spot]]`, `path_garden`; data proposal Q-102).

## Purpose

The vegetable garden `garden_veg` in the back (north) of level 1, where the child harvests **carrots and potatoes** as treats for the animals (GAME-GARDEN). A calm, sunny collecting place between missions. Not a hiding place — animals are never found here, so it must not look like any riddle place (mud, sand, meadow, leaf pile, sprinkler).

## Must be visible

- A small **fenced garden** (4 × 10 m): low white picket fence at the bottom with a **small open double gate** (2 m), along the river bank on the right; the panda enclosure fence on the left (bottom part), a patch of yellow sand beyond the left fence at the top; the tall north hedge closes the top.
- A straight light path up the middle, **four raised wooden beds** of dry dark-brown soil beside it: two **carrot beds** (feathery green leaves, orange tops just visible, 3 plants each) nearest the gate, two **potato beds** (small round bushes with a few white flowers, 2 plants each) behind them — the two kinds clearly different from the 55° camera.
- A small **stake sign** at the front end of each bed, facing the path, with a **picture** of a carrot or a potato (the word is rendered by the game, blank in the mockup).
- **Wheelbarrow** (empty, green) and **watering can** (green) at the hedge end.
- The girl harvesting a carrot with a small soil puff, a basket on her arm.

## Must not appear

- No mud, puddles or glossy wet soil (`loc_mud`); no yellow soil (`loc_sand`).
- No rake, no leaf pile (`loc_leaves`); no butterflies, wildflowers or tall grass (`loc_meadow`).
- No sprinkler, hose or water spray (`loc_sprinkler`); the only water is the river outside the fence.
- No animals inside the garden; no vegetables in the wheelbarrow; no scarecrow, no greenhouse.

## Props used

Modular (ART-ENVIRONMENT list): `path_tile`, `fence_wood`, `hedge`, `water_tile_flowing`, `sand_tile`, `grass_tuft`, and the new garden props `garden_fence`, `garden_gate`, `garden_bed`, `carrot_plant`, `potato_plant`, `garden_sign`, `wheelbarrow`, `watering_can` (GAME-GARDEN §9); carried item `basket`.
Hiding places shown: none (`loc_sand` and `loc_river` only at the edges).

## Mood

Sunny, calm, tidy and rewarding: neat green rows, the joy of pulling out a big carrot.

## Image settings

| File | Aspect / size | Camera (level coordinates, see `layout.md`) |
|---|---|---|
| `overview.png` | 16:9, 1920 × 1080 (SDXL: 1344 × 768) | Camera yaw north (image top = north), pitch ≈ 62°, target (8, 40.5), about 18 m from the target; frame covers the grass north of the bridge (bottom), the garden, the north hedge (top), the sand and panda fence (left) and the river (right). |
| `player_view.png` | 9:16, 1080 × 1920 (SDXL: 768 × 1344) | Player on `path_garden` at (7.5, 39.5) facing west towards `carrot_w2`; camera yaw north (image top = north), pitch ≈ 55°, ≈ 14 m from the player; the whole garden fills the frame. |
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

A small fenced vegetable garden in a zoo seen from high above, about 4 m wide and 10 m long, running from the bottom to the top of the image. A low white wooden picket fence closes its bottom side with a small open double gate in the middle, and runs along its right side on the bank of a small flowing river. Inside, a straight path of light sand-beige paving blocks leads from the gate up the middle. On both sides of the path lie raised garden beds in simple wooden frames filled with dry, crumbly dark-brown soil: the two beds nearest the gate hold neat rows of carrot plants, bright green feathery leaves with the orange tops of the carrots just visible in the soil; the two beds behind them hold small round bushy potato plants with a few tiny white flowers. At the front end of each bed stands a small wooden stake sign facing the path, showing a simple picture of a carrot or a potato. At the top end, in front of a tall dark-green hedge, stands an empty green wheelbarrow and a small green watering can. Along the lower left side of the garden runs the wooden post-and-rail fence of a neighbouring enclosure; at the upper left the low picket fence separates the garden from a patch of yellow sand. The player character, a small girl about 7 years old (1.2 m tall, head about one third of her body height), long straight dark-brown hair down her back, white T-shirt with four horizontal blue stripes, brown belt, blue jeans, dark-brown shoes stands on the garden path next to a carrot bed, holding a freshly pulled carrot and smiling; a woven basket hangs on her arm. Zoo world: light sand-beige paths of square paving blocks, wooden post-and-rail fences, tall dark-green hedges about 3 m high, round-topped trees, short green grass.

All signs and boards are blank: plain wooden or cream-coloured panels without any letters, words or numbers. The only markings allowed are a simple solid black animal silhouette on an enclosure sign and simple pictogram icons (shovel, padlock) on barrier signs. The small garden stake signs show only a simple coloured picture of a carrot or a potato. Enclosure signs and info boards are tilted back towards the camera so their faces are clearly visible from above.
```

### `player_view.png` (9:16 portrait)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

In-game view of a cozy zoo park simulation game, vertical phone-screen composition: a high follow camera looking down at about 55 degrees from about 14 m away, isometric-like perspective with a narrow field of view so vertical lines stay nearly parallel, no horizon, no sky. The small girl is seen from above near the centre of the image, her figure about one twelfth of the image height; the area around her, about 12 m across, is clearly laid out and readable: paths, fences and enclosures easy to tell apart.

Seen from above: the player character, a small girl about 7 years old (1.2 m tall, head about one third of her body height), long straight dark-brown hair down her back, white T-shirt with four horizontal blue stripes, brown belt, blue jeans, dark-brown shoes, stands on the light sand-beige garden path in the centre of the image, bending towards a raised wooden garden bed and pulling a carrot out of the dry, crumbly dark-brown soil with a small puff of soil; a woven basket with two carrots hangs on her arm. On both sides of the path lie raised beds: carrot plants with bright green feathery leaves and orange tops in rows nearest to her, small round potato plants with a few tiny white flowers further up. Small wooden stake signs with a carrot picture and a potato picture stand at the ends of the beds facing the path. A low white picket fence with a small open gate is at the bottom, a tall dark-green hedge with an empty green wheelbarrow and a green watering can at the top, a small flowing river along one side. Zoo world: light sand-beige paths of square paving blocks, wooden post-and-rail fences, tall dark-green hedges about 3 m high, round-topped trees, short green grass.

All signs and boards are blank: plain wooden or cream-coloured panels without any letters, words or numbers. The only markings allowed are a simple solid black animal silhouette on an enclosure sign and simple pictogram icons (shovel, padlock) on barrier signs. The small garden stake signs show only a simple coloured picture of a carrot or a potato. Enclosure signs and info boards are tilted back towards the camera so their faces are clearly visible from above.
```

### `player_view_landscape.png` (optional, 16:9)

Same as `player_view.png`, but replace the camera paragraph with:

```text
In-game view of a cozy zoo park simulation game, horizontal widescreen composition: a high follow camera looking down at about 55 degrees from about 16 m away, isometric-like perspective with a narrow field of view so vertical lines stay nearly parallel, no horizon, no sky. The small girl is seen from above near the centre of the image, her figure about one twelfth of the image height; the area around her, about 20 m across, is clearly laid out and readable.
```

### Negative prompt (all images of this area)

```text
text, letters, words, numbers, writing, captions, writing on signs, watermark, signature, logo, brand names, UI, HUD, buttons, photorealistic, realistic photo, photograph, realistic fur, hyper-detailed textures, dark, gloomy, night, fog, horror, scary, angry or menacing animals, sharp teeth, blood, gore, injury, dead animals, weapons, cages, cage bars, prison, rubbish, litter, crowds, clutter, distorted anatomy, extra legs, extra heads, fisheye distortion, tilted horizon, blurry, low resolution, jpeg artefacts, cropped main subject, sky, horizon, clouds, low camera angle, eye-level view, close-up, strong perspective distortion, mud, puddles, glossy wet soil, water spray, sprinkler, hose, rake, leaves on the ground, butterflies, wildflower meadow, tall grass, animals in the garden, vegetables in the wheelbarrow, scarecrow, greenhouse, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

Tools without a negative-prompt field: add "No text anywhere, no logos, not photorealistic, nothing scary." at the end of the prompt.

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Carrot beds and potato beds clearly different from the high camera; ripe plants look pull-able.
- [ ] Soil dry and crumbly — does not look like `loc_mud`; not yellow like `loc_sand`; no rake, butterflies, wildflowers or water spray.
- [ ] Fence low (the child sees over it), small gate clearly open and inviting.
- [ ] Stake signs show a carrot / potato picture, no letters.
- [ ] Style matches the approved style frame (Q-010).
- [ ] Layout matches `layout.md` and the level-1 map (positions, sizes, what is left/right).
- [ ] All signs, boards and labels blank — only pictures on the garden signs; no text artefacts anywhere.
- [ ] Bright, friendly, nothing scary; no cages.
- [ ] High camera as specified (pitch, girl ≈ 1/12 of the image height near the centre, no sky); verticals nearly parallel.
- [ ] Readable from the high camera at phone size (view the image at ~25 %).
- [ ] Every prop visible is in the ART-ENVIRONMENT modular list or listed in GAME-GARDEN §9.

## Generation log

*(empty — no image generated yet)*
