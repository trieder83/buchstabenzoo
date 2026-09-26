# Brief — `player_boy`

Spec: ART-CHARACTERS (look), ART-RIG (skeleton, rest pose, expressions), ART-PIPELINE §3
(turnaround files). Status: **concept — not approved** (`assets/manifest.toml`).
Style: **comic** — `art/style/style.md` (Q-010 answered). Designed as the matching
counterpart of `player_girl` (same body, same face style, different hair and outfit).

## Character

- **Who:** the boy the child can play as; ~6–8 years old; energetic, cheerful, friendly.
- **Style:** comic 3D cartoon, cel-shaded — rounded chunky shapes, bold dark-brown
  outlines (drawn by the renderer in-game, ART-RIG §3.5), flat colours with one hard
  shadow tone, big expressive hand-drawn eyes.
- **Proportions:** identical to `player_girl` (ART-RIG §2.4 — both share the rest pose):
  - height 1.20 m to top of skull (hair tuft ≤ 4 cm more); large round head 0.34 m high,
    ≈ 0.34 wide → head-to-body ≈ 1 : 3.5
  - short rounded torso (hips at 0.50 m, shoulders at 0.84 m), chunky soft limbs,
    shoulder to wrist 0.34 m, round mitten-like hands with a thumb
  - hip to ankle 0.41 m, slightly oversized sneakers
- **Pose for all views:** A-pose — standing straight, legs straight at hip width, arms
  straight and 45° down from horizontal, palms down, head straight, neutral face.
- **Readability from the high game camera** (≈ 55° from above, Q-049): seen mostly from
  above and small on screen, so the **short chestnut hair with its big tuft** and the
  **mustard-yellow shirt with one green band** must read as big, clean colour areas; the
  yellow stands out against grass green, sand paths and wood brown. From above the boy
  differs from the girl by short hair (neck visible), yellow shoulders and bare knees.

## Clothing and details

- Hair: short chestnut brown, a big rounded tuft sticking up at the front, short sides,
  ears visible. **No hat.**
- Face (hand-drawn decal): same style as the girl — big round dark-brown eyes with a white
  highlight, short chestnut brows, small cheeky grin, light rosy cheeks; small rounded
  nose.
- T-shirt: mustard yellow, short sleeves to mid upper arm, one broad green horizontal band
  across the chest and a green edge on each sleeve, untucked.
- Navy knee-length shorts; skin-coloured knees.
- White socks to mid-calf with one green stripe; chunky green sneakers with white soles.

## Colours (proposal — map to the shared palette once it exists, ADIR-002)

Outline colour is the global dark brown from `art/style/style.md` (renderer).

| Part | Hex |
|---|---|
| Skin / shade | `#F2C29B` / `#D9A27E` |
| Cheeks | `#F0A08A` |
| Hair / shade | `#8A4B22` / `#6A3717` |
| Eyes | `#2B1B12` (+ white `#FFFFFF` highlight) |
| Mouth | `#C8645A` |
| Shirt / band | `#F2B632` / `#3F8F3A` |
| Shorts | `#2E3F6E` |
| Socks / stripe | `#FFFFFF` / `#3F8F3A` |
| Sneakers / soles | `#3F8F3A` / `#F4F4F0` |

## Image-generation prompts

Use the **same tool, seed, settings and framing as `player_girl`** so both sheets line
up: portrait **1024 × 1536**, character centred, feet on the same baseline at ~92 % of the
image height, top of the head at ~12 %. Save as `front.png`, `side.png`, `back.png`,
`three_quarter.png`, `expressions.png` in this folder, plus `palette.png` (or keep the
colour table above).

Every prompt below = the CHARACTER SHEET STYLE block of `art/style/style.md` (verbatim,
first paragraph) + the view paragraph. Every prompt uses the negative prompt below.

**Negative prompt** (all views):

```text
T-pose, dynamic pose, holding objects, hat, cap, headband, backpack, jewellery, text, logo, watermark, scenery, gradient background, cropped feet, extra fingers, adult proportions, thin stripes, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

### `front.png`

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Front view, full body from head to toe, facing the camera directly. A 7-year-old boy with a large round head (about one third of his body height), short chestnut brown hair with a big rounded tuft sticking up at the front and short sides, ears visible, light peach skin with rosy cheeks, big round dark brown eyes with a white highlight, small cheeky grin, mustard yellow short-sleeved T-shirt with one broad green horizontal band across the chest and green sleeve edges, untucked, navy blue knee-length shorts, bare knees, white socks to mid-calf with a green stripe, chunky green sneakers with white soles. A-pose: arms straight and 45 degrees down from horizontal, legs straight at hip width, neutral friendly expression.
```

### `side.png`

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Exact left side profile view, full body from head to toe, the character faces the left edge of the image. A 7-year-old boy with a large round head (about one third of his body height), short chestnut brown hair with a big rounded tuft sticking up above the forehead and short sides, ear visible, light peach skin with rosy cheeks, big round dark brown eye with a white highlight, small rounded nose, mustard yellow short-sleeved T-shirt with one broad green horizontal band across the chest and green sleeve edges, untucked, navy blue knee-length shorts, bare knees, white socks to mid-calf with a green stripe, chunky green sneakers with white soles. A-pose: arms straight and 45 degrees down from horizontal, legs straight at hip width, neutral friendly expression.
```

### `back.png`

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Back view, full body from head to toe, the character faces away from the camera, no face visible. A 7-year-old boy with a large round head (about one third of his body height), short chestnut brown hair with the tip of a rounded tuft showing on top, short at the back so the neck is visible, ears visible, mustard yellow short-sleeved T-shirt with one broad green horizontal band around the chest and green sleeve edges, untucked, navy blue knee-length shorts, bare knees, white socks to mid-calf with a green stripe, chunky green sneakers with white soles. A-pose: arms straight and 45 degrees down from horizontal, legs straight at hip width.
```

### `three_quarter.png`

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Three-quarter front view, full body from head to toe, the character is turned 45 degrees to the left. A 7-year-old boy with a large round head (about one third of his body height), short chestnut brown hair with a big rounded tuft sticking up at the front and short sides, ears visible, light peach skin with rosy cheeks, big round dark brown eyes with a white highlight, small cheeky grin, mustard yellow short-sleeved T-shirt with one broad green horizontal band across the chest and green sleeve edges, untucked, navy blue knee-length shorts, bare knees, white socks to mid-calf with a green stripe, chunky green sneakers with white soles. A-pose: arms straight and 45 degrees down from horizontal, legs straight at hip width, neutral friendly expression.
```

### `expressions.png`

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Expression sheet of the same boy: this sheet shows only his head and shoulders, 8 portraits in a 4 by 2 grid, all the same size, all facing the camera. Large round head, short chestnut brown hair with a big rounded tuft at the front, ears visible, light peach skin with rosy cheeks, big round dark brown eyes with a white highlight, mustard yellow T-shirt collar with a green band. Expressions in this order: 1 neutral friendly, 2 eyes closed (blink), 3 happy smile, 4 laughing with open mouth, 5 talking with a small open mouth, 6 surprised with wide round eyes and round mouth, 7 thinking with eyes looking up and mouth pulled to one side, 8 slightly sad with lowered brows (gentle, not crying).
```

### Optional: `sheet.png` (all four views in one image)

If the tool drifts between separate generations, generate this wide sheet
(2048 × 1024) and crop it into `front.png`, `side.png`, `back.png`, `three_quarter.png`.

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Turnaround with four views side by side in one row, same scale and same baseline: front, left side profile, back, three-quarter front. A 7-year-old boy with a large round head (about one third of his body height), short chestnut brown hair with a big rounded tuft sticking up at the front and short sides, ears visible, light peach skin with rosy cheeks, big round dark brown eyes with a white highlight, small cheeky grin, mustard yellow short-sleeved T-shirt with one broad green horizontal band across the chest and green sleeve edges, untucked, navy blue knee-length shorts, bare knees, white socks to mid-calf with a green stripe, chunky green sneakers with white soles. A-pose in every view: arms straight and 45 degrees down from horizontal, legs straight at hip width, neutral friendly expression.
```

### Optional: `pair_front.png` (girl and boy side by side — for ACHAR-005)

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Front view of two children, full body from head to toe, standing side by side at the same height and scale, both in A-pose with arms straight and 45 degrees down from horizontal and legs straight at hip width, neutral friendly expressions. Left: a 7-year-old girl with a large round head, long straight dark brown hair as one chunky rounded mass falling behind her shoulders with a soft straight fringe, light peach skin with rosy cheeks, big round dark brown eyes, white short-sleeved T-shirt with three broad horizontal blue stripes, brown belt, blue denim trousers with darker cuffs, white socks, chunky dark brown shoes with white straps. Right: a 7-year-old boy with a large round head, short chestnut brown hair with a big rounded tuft at the front, ears visible, light peach skin with rosy cheeks, big round dark brown eyes, mustard yellow short-sleeved T-shirt with one broad green band across the chest, navy knee-length shorts, bare knees, white socks with a green stripe, chunky green sneakers.
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Clearly a matching pair with `player_girl` (same body, same face style, same height).
- [ ] Every prompt contains the CHARACTER SHEET STYLE block and the NEGATIVE suffix
      verbatim (APIPE-010, ACHAR-008).
- [ ] Same scale, baseline and A-pose in all four views; plain background.
- [ ] Head-to-body ≈ 1 : 3.5; no headwear.
- [ ] Distinguishable from `player_girl` from the front, from behind and from above.
- [ ] Expression sheet has all 8 expressions in the order of ART-RIG §6.2; eyes and mouth
      are clean shapes that work as a 64 × 64 px face decal.
| 2026-09-26 | sheet_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 6 + negative as 'Avoid', ref: style_frame.png, front.png + extra text | alternative (no hair tuft) |
| 2026-09-26 | sheet_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 6 + negative as 'Avoid', ref: style_frame.png, front.png + extra text | to review — chosen, split into the four views; side view mirrored (was facing right) |
