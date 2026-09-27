# Brief — `env_night1_overview` (layout-true overview of night level 1)

Spec: GAME-LEVEL-NIGHT-1, GAME-NIGHT, ART-ENVIRONMENT "Night art". Layout: `layout.md` in this
folder; level data `assets/levels/night-1.toml`. Style: `art/style/style.md` + the night lighting
paragraph of `art/night/README.md`. References: `art/environment/style_frame/style_frame.png`, the
approved night style frame, `art/environment/env_night_overview/overview.png` (approved **mood**
image — match its look, not its layout), `art/environment/env_night_house/overview.png`. Status:
**brief — images not generated yet** (written 2026-09-27). Order (ART-PIPELINE): greybox of
`night-1.toml` → this overview from the greybox render + style frames → approval → modelling.

## Purpose

The real layout of `night_1` at night: where the moon door opens, where the plaza, food hut and
night house are, and all nine hiding places with their night clues (AENV-004: `layout.md` lists
them). Also the GAME-MAP art of the night zoo.

## Must be visible (camera yaw **west**: image top = west, the moon door at the bottom)

- Bottom centre: the open **moon door** in the zoo wall (glowing moon sign, lanterns on both
  pillars), the entry path leading up (west) to a small **plaza** under an X of string lights,
  a blank map board beside it, a bench.
- Above the plaza (west): a small wooden **food hut** with warm windows and four food crates in
  front of it.
- Right of the plaza (north): the rounded **night house** (grass roof, painted moon, blue and warm
  red-orange portholes) with three blank lit info boards in front of it; behind it dense round
  trees.
- A **loop path** with lantern posts: from the plaza up (west) along the north ring past a small
  **toy telescope**, down the far side, and back along the south ring (left side of the image).
- The hidden middle: a dense block of big round old trees (bush border).
- Far side (top of the image, west strip, from left = south to right = north): a **heap of dry
  twigs** in the hedge corner, an old **potting bench with clay flowerpots** and white flowers, a
  big mossy old tree with a **ring of brown mushrooms**, a small grassy **hill with one big round
  stone**, and in the top-right corner one tall dark pointed **fir tree**.
- Left side (south strip, from top = west to bottom = east): a little wooden **windmill** with
  four sails, a low **meadow full of fireflies** with a small crooked tree, a small round **pond**
  mirroring the moon and stars with reeds, a short jetty and a wooden post; near the plaza a very
  thick old **tree with a round knothole**.
- The girl with her lantern on the entry path.

## Must not appear

- No animals visible (they are hidden); no text; no sky; nothing scary.
- No leaf pile, no bridge, no lilies/frogs/ducks on the pond, no fireflies outside the meadow,
  no second fir, no second hollow tree (riddle guards, GAME-LEVEL-NIGHT-1).

## Image settings

| File | Aspect / size | Camera (level coordinates) |
|---|---|---|
| `overview.png` | 16:9, 2K | yaw west (image top = west), pitch ≈ 62°, target (−48, 30), whole level in frame |
| `top_down.png` | 1:1 | orthographic, north up, matching the ASCII map of GAME-LEVEL-NIGHT-1 (greybox render) |

## Prompt — `overview.png`

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

NIGHT LIGHTING — this replaces the midday sunlight described above: a calm, friendly night. There is no sun. Soft cool moonlight from the upper left gives a deep friendly blue palette — medium deep blue and blue-violet on grass, leaves and walls, lighter soft blue on the tops of objects, one hard-edged darker blue shadow tone, never pitch black; every object stays clearly visible with its bold dark outlines and flat colours. The second light is warm yellow-orange lamp light: lamps and lit windows glow as flat bright warm-yellow shapes and throw round, hard-edged pools of warm light on the ground. Cosy, magical and safe like a bedtime story for young children — nothing scary, nothing hidden in darkness.

Elevated three-quarter top-down view like a cozy zoo park simulation game: a high camera looking down at about 60 to 65 degrees, isometric-like perspective with a narrow field of view so vertical lines stay nearly parallel, horizontal widescreen composition, the whole area in frame and neatly laid out like a diorama, the ground fills the image, no horizon, no sky.

A friendly night zoo in a moonlit forest garden, surrounded on all sides by tall soft blue hedges, seen with the entrance at the bottom. At the bottom centre a big wooden gate in a stone wall stands wide open: blue plank door leaves with small painted stars, a round sign above it with a glowing cream crescent moon, a warm lantern on each stone pillar. From the gate a light sand-beige path leads up to a small square plaza under two crossing strings of warm glowing bulbs, with a blank map board and a wooden bench beside it. Just above the plaza stands a small wooden food hut with warmly lit windows and four wooden crates in front of it. To the right of the plaza a friendly rounded night house with a dark-green grass roof, a painted moon on its wall and round porthole windows glowing soft blue and warm red-orange; three small blank info boards with little lamps stand in a row in front of it. A sand-beige path lined with wooden lantern posts (each lantern glowing warm yellow and throwing a round hard-edged pool of warm light) loops around a dense block of big round old trees in the middle of the garden: it runs up the right side past a small toy telescope on a wooden stand, along the top, and back down the left side to the plaza. Beyond the path at the top edge, from left to right: a heap of dry brown twigs in the hedge corner; an old wooden potting bench with stacked clay flowerpots and little white flowers; a big mossy old tree with a ring of round brown mushrooms at its foot; a small round grassy hill with one big round grey stone on top in bright moonlight; in the top-right corner one tall dark pointed fir tree with cones. Along the left edge, from top to bottom: a little wooden windmill with four sails; a low meadow full of tiny warm yellow-green fireflies around a small crooked tree; a small round still pond mirroring the moon and white stars, with reeds, a short wooden jetty and one wooden post at the shore; near the plaza one very thick old tree with a big round knothole. On the path just above the gate a small girl (long straight dark-brown hair, white T-shirt with blue stripes, blue jeans), about one twentieth of the image height, carries a small glowing lantern with a soft warm light circle around her. No animals are visible. Magical, cosy and safe like a bedtime story for young children — warm lights against soft friendly blue, nothing scary.

All signs and boards are blank: plain wooden or cream-coloured panels without any letters, words or numbers. The only markings allowed are a simple solid black animal silhouette on an enclosure sign and simple pictogram icons (shovel, padlock) on barrier signs. Enclosure signs and info boards are tilted back towards the camera so their faces are clearly visible from above.
```

### Negative prompt

```text
text, letters, words, numbers, writing, captions, writing on signs, watermark, signature, logo, brand names, UI, HUD, buttons, daylight, sunshine, midday sun, pitch black, deep black shadows, unreadable dark areas, dark forest, dense spooky woods, bare dead trees, horror, scary, spooky, halloween, creepy, haunted, graveyard, menacing shapes, monsters, ghosts, glowing red eyes, eyes in the dark, skulls, fog, mist, thunderstorm, rain, animals, cages, cage bars, rubbish, crowds, clutter, distorted anatomy, fisheye distortion, blurry, low resolution, cropped main subject, sky, horizon, moon in the sky, clouds, low camera angle, eye-level view, close-up, strong perspective distortion, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

Add before the suffix: "leaf pile, bridge, water lilies, frogs, ducks, second fir tree".

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Layout matches `layout.md` and the ASCII map (moon door bottom, plaza, food hut, night house right, loop, places on the far side).
- [ ] Each hiding place readable at phone size by its night clue: twigs, flowerpots + white flowers, mushrooms, hill + stone, fir, windmill, fireflies, moon in the pond, knothole.
- [ ] Night look as the approved night style frame; friendly, nothing scary (NIGHT-009); no text.

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
