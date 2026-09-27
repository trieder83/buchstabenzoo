# Brief — `env_zookeeper_house_1` (level-1 zookeeper house with the bed and the cart key box)

Spec: GAME-LEVEL-1 "Zookeeper house", GAME-NIGHT rule 3, GAME-CART 12–16, ART-ENVIRONMENT. Layout:
`layout.md` in this folder; level data `assets/levels/level-1.toml`. Style: `art/style/style.md`
(comic). References: `art/environment/style_frame/style_frame.png`, `art/props/kit_bedroom`
(approved bedroom props), `art/environment/env_zookeeper_house` (level-3 house, same building
family). Status: **brief — images not generated yet** (written 2026-09-27 by the level designer).

## Purpose

The small enterable zookeeper house `zookeeper_house_1` west of the entrance plaza: where the
child **sleeps** at night (bed, GAME-NIGHT) and where she finds the **"Math Fighter" note** on the
desk and the **key box** with the golf-cart key outside (GAME-CART). Shown closed (by day, from the
plaza) and with the roof cut away (player inside), plus one night view of the bedroom corner.

## Must be visible

- Small wooden house 6 × 5 m with a red roof, the door on the **east** side facing the plaza; a
  short path from the plaza to the door; a small wooden **key box with a 3-wheel combination lock**
  on the wall **left of the door (south)**, a wall lamp right of the door.
- Cut-away: along the south wall a child-size **bed** with the blue star blanket (headboard west),
  a night table with a bedside lamp at its head, a square window with blue curtains in the west
  wall, a round blue-and-cream rug, a toy chest with a plush elephant; against the north wall a
  wooden **desk** with one sheet of paper on it (blank — the game draws the text).
- Around: the map board and the plaza edge to the east, the path along the north side of the
  house, the zebra enclosure fence to the north, the bamboo corner at the far west edge.

## Must not appear

- No text anywhere (the note is blank paper, the key box has plain number wheels without digits).
- No fish bowl (that is the level-3 house), no bamboo growing at the house (bamboo only at
  `loc_bamboo`), no animals.

## Props used

Modular (ART-ENVIRONMENT list): `path_tile`, `map_board`, `fence_wood`, `bench`, `bush`,
`grass_tuft`. `kit_bedroom`: `bed`, `night_table`, `bedside_lamp`, `window_moon`, `rug_round`,
`toy_chest`. New: `desk`, `note_math_fighter`, `key_box` (GAME-CART 16), `wall_lamp` (`kit_night`).
Unique model: `zookeeper_house` (same model family as `zookeeper_house_3`, smaller footprint).

## Mood

Homely, warm wood, safe — the child's own little house in the zoo.

## Image settings

| File | Aspect / size | Camera (level coordinates, see `layout.md`) |
|---|---|---|
| `overview.png` | 16:9, 1920 × 1080 | yaw north (image top = north), pitch ≈ 62°, target (−10, 3), ≈ 16 m; roof cut away |
| `player_view.png` | 9:16, 1080 × 1920 | player on the rug at (−10.5, 2.5), yaw north, pitch ≈ 55°, ≈ 14 m; roof faded out |
| `night_view.png` | 16:9, 1920 × 1080 | as `overview.png`, at night (NIGHT LIGHTING paragraph), bedside lamp and wall lamp glowing |

## Prompts

First paragraph = STYLE block of `art/style/style.md`, verbatim (APIPE-010); the girl description
verbatim.

### `overview.png`

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Elevated three-quarter top-down view like a cozy zoo park simulation game: a high camera looking down at about 60 to 65 degrees, isometric-like perspective with a narrow field of view so vertical lines stay nearly parallel, horizontal widescreen composition, the whole area in frame and neatly laid out like a diorama, the ground fills the image, no horizon, no sky.

A small wooden zookeeper house, about 6 by 5 metres, with its red roof removed like a doll's house so we look down into the single room, standing on short green grass next to the entrance plaza of a small zoo. The door is in the right-hand (east) wall and opens onto a short sand-beige path that leads right to the plaza; on the outside wall just below the door hangs a small wooden key box with a three-wheel combination lock, and a small wall lamp hangs just above the door. Inside: along the bottom (south) wall a chunky child-size wooden bed with a rounded headboard on the left, a big white pillow and a blue blanket with small white stars; a small night table with a round bedside lamp beside the headboard; a square window with short blue curtains in the left wall; a round blue-and-cream braided rug in the middle of the wooden floor; a wooden toy chest with a plush elephant peeking out in the top-left corner; a wooden desk against the top (north) wall with a single blank sheet of paper on it. Outside: a sand-beige path runs along the top side of the house from the plaza towards a dense bamboo thicket at the far left edge; a blank picture map board on two posts stands right of the house; a wooden post-and-rail enclosure fence runs across the top of the image. the player character, a small girl about 7 years old (1.2 m tall, head about one third of her body height), long straight dark-brown hair down her back, white T-shirt with four horizontal blue stripes, brown belt, blue jeans, dark-brown shoes, stands on the path in front of the door. Zoo world: light sand-beige paths of square paving blocks, wooden post-and-rail fences, tall dark-green hedges about 3 m high, round-topped trees, short green grass.

All signs and boards are blank: plain wooden or cream-coloured panels without any letters, words or numbers. The only markings allowed are a simple solid black animal silhouette on an enclosure sign and simple pictogram icons (shovel, padlock) on barrier signs. Enclosure signs and info boards are tilted back towards the camera so their faces are clearly visible from above.
```

### `player_view.png` (9:16 portrait)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

In-game view of a cozy zoo park simulation game, vertical phone-screen composition: a high follow camera looking down at about 55 degrees from about 14 m away, isometric-like perspective with a narrow field of view so vertical lines stay nearly parallel, no horizon, no sky. The small girl is seen from above near the centre of the image, her figure about one twelfth of the image height; the area around her, about 12 m across, is clearly laid out and readable: paths, fences and enclosures easy to tell apart.

Seen from above: the player character, a small girl about 7 years old (1.2 m tall, head about one third of her body height), long straight dark-brown hair down her back, white T-shirt with four horizontal blue stripes, brown belt, blue jeans, dark-brown shoes, stands on a round blue-and-cream rug in the middle of a small cosy wooden room whose roof is removed like a doll's house. Below her along the wall a chunky child-size wooden bed with a big white pillow and a blue blanket with small white stars, a night table with a round bedside lamp at its head; a square window with blue curtains in the left wall; a toy chest with a plush elephant in the top-left corner; a wooden desk against the top wall with a single blank sheet of paper on it. The open door is in the right wall; outside it a small wooden key box with a three-wheel combination lock hangs on the wall, and a sand-beige path leads right to a plaza. Zoo world: light sand-beige paths of square paving blocks, wooden post-and-rail fences, tall dark-green hedges about 3 m high, round-topped trees, short green grass.

All signs and boards are blank: plain wooden or cream-coloured panels without any letters, words or numbers. The only markings allowed are a simple solid black animal silhouette on an enclosure sign and simple pictogram icons (shovel, padlock) on barrier signs. Enclosure signs and info boards are tilted back towards the camera so their faces are clearly visible from above.
```

### `night_view.png`

Same scene paragraph as `overview.png`; the STYLE block is followed directly by the night
paragraph (`art/night/README.md`), verbatim (quoted here, not a prompt block):

> NIGHT LIGHTING — this replaces the midday sunlight described above: a calm, friendly night. There is no sun. Soft cool moonlight from the upper left gives a deep friendly blue palette — medium deep blue and blue-violet on grass, leaves and walls, lighter soft blue on the tops of objects, one hard-edged darker blue shadow tone, never pitch black; every object stays clearly visible with its bold dark outlines and flat colours. The second light is warm yellow-orange lamp light: lamps and lit windows glow as flat bright warm-yellow shapes and throw round, hard-edged pools of warm light on the ground. Cosy, magical and safe like a bedtime story for young children — nothing scary, nothing hidden in darkness.


and the scene adds: "The bedside lamp glows flat bright warm yellow and throws a round hard-edged
pool of warm light over the bed; the window shows a round pale-cream moon and two stars; the wall
lamp over the door glows and lights the key box."

### Negative prompt (day images)

```text
text, letters, words, numbers, writing, captions, writing on signs, watermark, signature, logo, brand names, UI, HUD, buttons, photorealistic, realistic photo, photograph, realistic fur, hyper-detailed textures, dark, gloomy, night, fog, horror, scary, angry or menacing animals, sharp teeth, blood, gore, injury, dead animals, weapons, cages, cage bars, prison, rubbish, litter, crowds, clutter, distorted anatomy, extra legs, extra heads, fisheye distortion, tilted horizon, blurry, low resolution, jpeg artefacts, cropped main subject, sky, horizon, clouds, low camera angle, eye-level view, close-up, strong perspective distortion, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

Night image: the negative prompt of `art/environment/env_night_overview/brief.md` (night negatives
+ NEGATIVE suffix), with "fish bowl" added.

## Review checklist (before `concept_approved = true` — user decides)

- [ ] The bed reads instantly as "sleep here" from the high camera at phone size.
- [ ] Desk with one sheet of paper and the key box by the door are recognisable (without text).
- [ ] Door faces the plaza (east); layout matches `layout.md` and the level-1 map.
- [ ] Comic style as the style frame; cosy, nothing scary; no text anywhere.

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
