# Brief — `player_girl`

Spec: ART-CHARACTERS (look), ART-RIG (skeleton, rest pose, expressions), ART-PIPELINE §3
(turnaround files). Status: **concept — not approved** (`assets/manifest.toml`).
Style reference: the girl in `art/reference/ref-player-style.jpg`.

## Character

- **Who:** the girl the child can play as; ~6–8 years old; curious, calm, friendly.
- **Style:** blocky / voxel-inspired (Minecraft-like), pixel-art face, flat colour areas,
  no outlines, soft daylight.
- **Proportions** (must match ART-RIG §2.4 — the model is built from these):
  - height 1.20 m to top of skull (hair ≤ 4 cm more); head block 0.34 m high, 0.34 wide,
    0.32 deep → head-to-body ≈ 1 : 3.5
  - torso block 0.30 wide × 0.34 high × 0.18 deep (hips at 0.50 m, shoulders at 0.84 m)
  - arms 0.10 × 0.10 blocks, shoulder to wrist 0.34 m, cube hands
  - legs 0.12 × 0.12 blocks, hip to ankle 0.41 m, feet/shoes 0.08 high
- **Pose for all views:** A-pose — standing straight, legs straight at hip width, arms
  straight and 45° down from horizontal, palms down, head straight, neutral face.
- **Silhouette key:** long hair down the back (readable from behind), striped shirt.

## Clothing and details

- Hair: long, straight, dark brown, down to the upper back; straight blocky fringe, slightly
  side-parted; one block-step curl at the ends. No hair accessories, **no hat**.
- Face: 2 × 2 px dark brown eyes with a white highlight pixel, short brown brows, small
  rose mouth.
- T-shirt: white, short sleeves to mid upper arm, 4 horizontal blue stripes on body and
  sleeves.
- Brown belt at the hips.
- Blue denim trousers to the ankle with a slightly darker cuff.
- White socks visible at the ankle; dark brown shoes with white strap and sole.

## Colours (proposal — map to the shared palette once it exists, ADIR-002)

| Part | Hex |
|---|---|
| Skin / shade | `#F2C29B` / `#D9A27E` |
| Hair / highlight | `#4A2A17` / `#6B3E22` |
| Eyes | `#2B1B12` (+ white `#FFFFFF` highlight) |
| Mouth | `#C8645A` |
| Shirt / stripes | `#F4F4F0` / `#2F5DA8` |
| Belt | `#6B3A1E` |
| Trousers / cuff | `#3559A0` / `#2A4780` |
| Socks | `#FFFFFF` |
| Shoes / sole | `#3B2314` / `#F4F4F0` |

## Image-generation prompts

Use the same seed/settings for all views if the tool allows it, portrait **1024 × 1536**,
character centred, feet on the same baseline at ~92 % of the image height, top of the
head at ~12 %. Save as `front.png`, `side.png`, `back.png`, `three_quarter.png`,
`expressions.png` in this folder, plus `palette.png` (or keep the colour table above).

**Shared style block** (append to every prompt):

> blocky voxel-style 3D character, Minecraft-inspired, cube head, box-shaped torso and
> limbs, pixel-art face, flat colours, no outlines, soft even studio light, orthographic
> camera, full body visible, A-pose with arms straight and 45 degrees down from
> horizontal, legs straight at hip width, neutral expression, plain light grey background
> (#E6E6E6), no shadow on the background, no props, no text, character turnaround
> reference sheet style

**Negative prompt:**

> realistic, smooth skin, anime, outlines, dynamic pose, T-pose, holding objects, hat,
> backpack, jewellery, text, logo, watermark, gradient background, scenery, perspective
> distortion, cropped feet, extra fingers, adult proportions

**Character block** (insert before the style block):

> a 7-year-old girl, head about one third of her body height, long straight dark brown
> hair falling down her back with a straight blocky fringe slightly parted to the side,
> light peach skin, small dark brown pixel eyes with a white highlight, small rose mouth,
> white short-sleeved T-shirt with four horizontal blue stripes, brown belt, blue denim
> trousers to the ankle with darker cuffs, white socks, dark brown shoes with white straps
> and soles

| File | Prompt |
|---|---|
| `front.png` | "Front view, facing the camera directly, " + character block + style block |
| `side.png` | "Exact left side profile view, character facing to the left of the image, " + character block + style block + ", the long hair visible down the back" |
| `back.png` | "Back view, character facing away from the camera, " + character block + style block + ", long dark brown hair covering the upper back, no face visible" |
| `three_quarter.png` | "Three-quarter front view, character turned 45 degrees to the left, " + character block + style block |
| `expressions.png` | "Expression sheet of the same blocky voxel girl's cube head only, 8 heads in a 4 by 2 grid, all the same size and facing the camera, pixel-art faces: 1 neutral, 2 eyes closed blink, 3 happy smile, 4 laughing with open mouth, 5 talking with small open mouth, 6 surprised with round eyes and round mouth, 7 thinking with eyes looking up and mouth to one side, 8 slightly sad with lowered brows (gentle, not crying), " + character block + ", flat colours, plain light grey background (#E6E6E6), no text" |

**Tip for consistency:** if the tool drifts between separate generations, generate one
wide sheet (2048 × 1024) with the prompt "Character turnaround sheet with four views side
by side in one row, same scale and baseline: front, left side, back, three-quarter, " +
character block + style block, then crop it into the four files.

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Recognisably the girl from `art/reference/ref-player-style.jpg`.
- [ ] Same scale, baseline and A-pose in all four views; plain background.
- [ ] Head-to-body ≈ 1 : 3.5; no headwear.
- [ ] Distinguishable from `player_boy` from the front and from behind.
- [ ] Expression sheet has all 8 expressions in the order of ART-RIG §6.2.
- [ ] Rigid-skinning exceptions (if the long hair must bend over the shoulders): _none
  planned — hair is rigid on `head`_.
