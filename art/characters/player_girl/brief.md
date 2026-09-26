# Brief — `player_girl`

Spec: ART-CHARACTERS (look), ART-RIG (skeleton, rest pose, expressions), ART-PIPELINE §3
(turnaround files). Status: **concept — not approved** (`assets/manifest.toml`).
Style: **comic** — `art/style/style.md` (Q-010 answered). Design reference (hair, clothes,
colours): the girl in `art/reference/ref-player-style.jpg`, translated from blocky to comic.

## Character

- **Who:** the girl the child can play as; ~6–8 years old; curious, calm, friendly.
- **Style:** comic 3D cartoon, cel-shaded — rounded chunky shapes, bold dark-brown
  outlines (drawn by the renderer in-game, ART-RIG §3.5), flat colours with one hard
  shadow tone, big expressive hand-drawn eyes.
- **Proportions** (must match ART-RIG §2.4 — the model is built from these):
  - height 1.20 m to top of skull (hair ≤ 4 cm more); large round head 0.34 m high,
    ≈ 0.34 wide → head-to-body ≈ 1 : 3.5
  - short rounded torso (hips at 0.50 m, shoulders at 0.84 m), chunky soft limbs,
    shoulder to wrist 0.34 m, round mitten-like hands with a thumb
  - hip to ankle 0.41 m, slightly oversized shoes
- **Pose for all views:** A-pose — standing straight, legs straight at hip width, arms
  straight and 45° down from horizontal, palms down, head straight, neutral face.
- **Readability from the high game camera** (≈ 55° from above, Q-049): seen mostly from
  above and small on screen, so the **dark-brown hair mass on top and down the back** and
  the **white-and-blue shirt on the shoulders** must read as two big, clean colour areas.
  Broad stripes only (3 on the body), no fine patterns.

## Clothing and details

- Hair: long, straight, dark brown, one chunky rounded mass down to the shoulder blades;
  soft straight fringe, slightly side-parted; ends flick slightly outwards. No hair
  accessories, **no hat**.
- Face (hand-drawn decal): big round dark-brown eyes with a white highlight, short brown
  brows, small rose smile, light rosy cheeks; small rounded nose.
- T-shirt: white, short sleeves to mid upper arm, 3 broad horizontal blue stripes on the
  body and one on each sleeve.
- Brown belt at the hips.
- Blue denim trousers to the ankle with a slightly darker cuff.
- White socks visible at the ankle; chunky dark brown shoes with white strap and sole.

## Colours (proposal — map to the shared palette once it exists, ADIR-002)

Outline colour is the global dark brown from `art/style/style.md` (renderer).

| Part | Hex |
|---|---|
| Skin / shade | `#F2C29B` / `#D9A27E` |
| Cheeks | `#F0A08A` |
| Hair / shade | `#4A2A17` / `#351D0F` |
| Eyes | `#2B1B12` (+ white `#FFFFFF` highlight) |
| Mouth | `#C8645A` |
| Shirt / stripes | `#F4F4F0` / `#2F5DA8` |
| Belt | `#6B3A1E` |
| Trousers / cuff | `#3559A0` / `#2A4780` |
| Socks | `#FFFFFF` |
| Shoes / sole | `#3B2314` / `#F4F4F0` |

## Image-generation prompts

Use the same tool, seed and settings for all views, portrait **1024 × 1536**, character
centred, feet on the same baseline at ~92 % of the image height, top of the head at
~12 %. Save as `front.png`, `side.png`, `back.png`, `three_quarter.png`,
`expressions.png` in this folder, plus `palette.png` (or keep the colour table above).

Every prompt below = the CHARACTER SHEET STYLE block of `art/style/style.md` (verbatim,
first paragraph) + the view paragraph. Every prompt uses the negative prompt below.

**Negative prompt** (all views):

```text
T-pose, dynamic pose, holding objects, hat, headband, backpack, jewellery, text, logo, watermark, scenery, gradient background, cropped feet, extra fingers, adult proportions, thin stripes, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

### `front.png`

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Front view, full body from head to toe, facing the camera directly. A 7-year-old girl with a large round head (about one third of her body height), long straight dark brown hair as one chunky rounded mass falling behind her shoulders with a soft straight fringe slightly parted to the side, light peach skin with rosy cheeks, big round dark brown eyes with a white highlight, small rose smile, white short-sleeved T-shirt with three broad horizontal blue stripes, brown belt, blue denim trousers to the ankle with darker cuffs, white socks, chunky dark brown shoes with white straps and soles. A-pose: arms straight and 45 degrees down from horizontal, legs straight at hip width, neutral friendly expression.
```

### `side.png`

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Exact left side profile view, full body from head to toe, the character faces the left edge of the image. A 7-year-old girl with a large round head (about one third of her body height), long straight dark brown hair as one chunky rounded mass reaching down to her shoulder blades, soft straight fringe, light peach skin with rosy cheeks, big round dark brown eye with a white highlight, small rounded nose, white short-sleeved T-shirt with three broad horizontal blue stripes, brown belt, blue denim trousers to the ankle with darker cuffs, white socks, chunky dark brown shoes with white straps and soles. A-pose: arms straight and 45 degrees down from horizontal, legs straight at hip width, neutral friendly expression.
```

### `back.png`

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Back view, full body from head to toe, the character faces away from the camera, no face visible. A 7-year-old girl with a large round head (about one third of her body height), long straight dark brown hair as one chunky rounded mass covering the back of her head and her upper back down to the shoulder blades, white short-sleeved T-shirt with three broad horizontal blue stripes, brown belt, blue denim trousers to the ankle with darker cuffs, white socks, chunky dark brown shoes with white soles. A-pose: arms straight and 45 degrees down from horizontal, legs straight at hip width.
```

### `three_quarter.png`

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Three-quarter front view, full body from head to toe, the character is turned 45 degrees to the left. A 7-year-old girl with a large round head (about one third of her body height), long straight dark brown hair as one chunky rounded mass falling behind her shoulders with a soft straight fringe slightly parted to the side, light peach skin with rosy cheeks, big round dark brown eyes with a white highlight, small rose smile, white short-sleeved T-shirt with three broad horizontal blue stripes, brown belt, blue denim trousers to the ankle with darker cuffs, white socks, chunky dark brown shoes with white straps and soles. A-pose: arms straight and 45 degrees down from horizontal, legs straight at hip width, neutral friendly expression.
```

### `expressions.png`

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Expression sheet of the same girl: this sheet shows only her head and shoulders, 8 portraits in a 4 by 2 grid, all the same size, all facing the camera. Large round head, long straight dark brown hair with a soft straight fringe, light peach skin with rosy cheeks, big round dark brown eyes with a white highlight, white T-shirt collar with a blue stripe. Expressions in this order: 1 neutral friendly, 2 eyes closed (blink), 3 happy smile, 4 laughing with open mouth, 5 talking with a small open mouth, 6 surprised with wide round eyes and round mouth, 7 thinking with eyes looking up and mouth pulled to one side, 8 slightly sad with lowered brows (gentle, not crying).
```

### Optional: `sheet.png` (all four views in one image)

If the tool drifts between separate generations, generate this wide sheet
(2048 × 1024) and crop it into `front.png`, `side.png`, `back.png`, `three_quarter.png`.

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Turnaround with four views side by side in one row, same scale and same baseline: front, left side profile, back, three-quarter front. A 7-year-old girl with a large round head (about one third of her body height), long straight dark brown hair as one chunky rounded mass falling behind her shoulders to the shoulder blades with a soft straight fringe slightly parted to the side, light peach skin with rosy cheeks, big round dark brown eyes with a white highlight, small rose smile, white short-sleeved T-shirt with three broad horizontal blue stripes, brown belt, blue denim trousers to the ankle with darker cuffs, white socks, chunky dark brown shoes with white straps and soles. A-pose in every view: arms straight and 45 degrees down from horizontal, legs straight at hip width, neutral friendly expression.
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Recognisably the girl from `art/reference/ref-player-style.jpg`, now in comic style.
- [ ] Every prompt contains the CHARACTER SHEET STYLE block and the NEGATIVE suffix
      verbatim (APIPE-010, ACHAR-008).
- [ ] Same scale, baseline and A-pose in all four views; plain background.
- [ ] Head-to-body ≈ 1 : 3.5; no headwear; broad stripes only.
- [ ] Distinguishable from `player_boy` from the front, from behind and from above.
- [ ] Expression sheet has all 8 expressions in the order of ART-RIG §6.2; eyes and mouth
      are clean shapes that work as a 64 × 64 px face decal.
- [ ] Hair ends above the shoulder blades (rigid on `head`, no hair bones — ART-RIG §3.3).
