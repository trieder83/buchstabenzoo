# Brief — `style_frame` (confirms the comic style)

Spec: ART-PIPELINE §5 (style frame first), ART-DIRECTION, ART-ENVIRONMENT. Status: **brief —
not generated yet**. Style decided: **comic** (Q-010, `art/style/style.md`). Camera decided:
high-angle follow camera (Q-049).

## Purpose

One image that **pins down the comic style** before any other mockup is made. Once
approved it is the **style reference image** for every later concept image (characters,
animals, environment) — uploaded as style/reference image in the generator, together with
the verbatim STYLE block from `art/style/style.md`.

| File | Content |
|---|---|
| `style_frame.png` | The zebra-enclosure scene below in the comic style |

Generate several candidates, pick the best; if the look is not right, adjust the blocks in
`art/style/style.md` (never only here) and regenerate.

## Camera — zoo-park simulation view (Q-049)

High-angle follow camera like cozy zoo/animal-park simulation games: pitch ≈ 55°, ≈ 16 m
from the girl, narrow FOV (≈ 30–35°) so verticals stay nearly parallel, isometric-like; no
sky, no horizon. The girl is small (≈ 1/12 of the image height) near the centre. Same camera
as the `player_view.png` of every mockup (landscape framing here).

**Game-design implication:** from this camera vertical signs are foreshortened — the
enclosure sign and the info board are **tilted back towards the camera** (≈ 30–45°), and
riddle texts are read in a close-up UI panel when interacting (GAME-PLAYER, ADIR-001).

## The scene

The **zebra enclosure** (GAME-LEVEL-1, `enc_zebra` seen from `path_ring_w`) with the
**player girl** (`art/characters/player_girl/brief.md`) for scale:

- A sand-beige paved path running diagonally from bottom left to top centre; the girl
  stands on it in the middle of the image.
- Right half: the zebra enclosure — wooden post-and-rail fence along the path, gate beside
  the girl, short grass, two round bushes, grey stone arch shelter near the top right,
  **one zebra** near the fence.
- Beside the gate a large wooden **enclosure sign** on two posts — **blank except for a
  black zebra silhouette** — and a small wooden **info board** on one post, **completely
  blank** (text is rendered by the game, ART-ENVIRONMENT §2); both tilted towards the camera.
- Left of the path: a tall green hedge and a wooden bench; a few trees in the corners.
- Bright midday daylight.

## Image settings

- **16:9 landscape, 1920 × 1080** (SDXL-type tools: 1344 × 768, then upscale).
- Generate 4+ candidates; note the seed of the chosen one.
- Save as `style_frame.png` in this folder, ≤ 2048 px on the long side.

## Prompt (`style_frame.png`)

First paragraph = STYLE block from `art/style/style.md`, verbatim.

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Elevated three-quarter top-down view like a cozy zoo park simulation game, horizontal widescreen composition: a high follow camera looking down at about 55 degrees from about 16 m away, isometric-like perspective with a narrow field of view so vertical lines stay nearly parallel, the ground fills the image, no horizon, no sky.

A sunny zoo corner with a zebra enclosure, seen from above. A light sand-beige paved path runs diagonally from the bottom left to the top centre of the image. In the middle of the image the player character stands on the path, small (about one twelfth of the image height) and seen from above: a small girl about 7 years old (1.2 m tall, head about one third of her body height), long straight dark-brown hair down her back, white T-shirt with four horizontal blue stripes, brown belt, blue jeans, dark-brown shoes. The right half of the image is the zebra enclosure: a meadow of short green grass with two round bushes, enclosed by a wooden post-and-rail fence along the path, a grey stone arch shelter near the top right, and one zebra standing near the fence, turned towards the girl; the zebra's back is about as high as her head. In the fence beside the girl a wooden gate; next to the gate a large wooden enclosure sign on two posts and, on the path side, a small wooden info board on one post; both are tilted back towards the camera so their faces are clearly visible from above. On the left of the path a tall green hedge about 3 m high and, in front of it, a wooden bench. A few round-topped trees in the image corners.

All signs and boards are blank: plain wooden or cream-coloured panels without any letters, words or numbers. The only marking allowed is a simple solid black zebra silhouette on the enclosure sign; it must be clearly readable from the high camera.
```

### Negative prompt

```text
text, letters, words, numbers, writing, captions, writing on signs, watermark, signature, logo, brand names, UI, HUD, buttons, photorealistic, realistic photo, photograph, realistic fur, hyper-detailed textures, dark, gloomy, night, fog, horror, scary, angry or menacing animals, sharp teeth, blood, gore, injury, dead animals, weapons, cages, cage bars, prison, rubbish, litter, crowds, clutter, distorted anatomy, extra legs, extra heads, fisheye distortion, tilted horizon, blurry, low resolution, jpeg artefacts, cropped main subject, sky, horizon, clouds, low camera angle, eye-level view, close-up, strong perspective distortion, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

Tools without a negative-prompt field: add "No text anywhere, no logos, not photorealistic,
not voxel, nothing scary, no sky." at the end of the prompt.

## Review checklist (user approves)

- [ ] Comic look: bold dark outlines, flat colours with one hard shadow tone, rounded
      chunky shapes, big friendly eyes — no voxels/pixels, no painterly gradients.
- [ ] High-angle view as specified: pitch ≈ 55°, no sky/horizon, verticals nearly parallel,
      girl ≈ 1/12 of the image height near the centre.
- [ ] Signs and board are blank; only the zebra silhouette on the enclosure sign.
- [ ] Readable from the high camera at phone size (view at about 25 %): the zebra silhouette
      on the tilted sign, the info board, the gate and the zebra are recognisable
      (important for `kiga`, ART-DIRECTION §4).
- [ ] The girl reads as the player character from above (long dark hair, striped shirt) and
      matches the scale of the zebra and the fence.
- [ ] Buildable within the mobile budgets (ART-PIPELINE §10): simple rounded low-poly
      shapes, flat colours; outlines and cel shading come from the renderer.
- [ ] Friendly, bright, nothing scary.
- [ ] On approval: set status `approved` in `art/catalog.js` and use this image as the
      style reference for all further concept images.

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| | | | | | |
