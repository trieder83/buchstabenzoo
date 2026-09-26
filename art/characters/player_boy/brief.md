# Brief — `player_boy`

Spec: ART-CHARACTERS (look), ART-RIG (skeleton, rest pose, expressions), ART-PIPELINE §3
(turnaround files). Status: **concept — not approved** (`assets/manifest.toml`).
Style reference: `art/reference/ref-player-style.jpg`; designed as the matching
counterpart of `player_girl` (same body, same face style, different hair and outfit).

## Character

- **Who:** the boy the child can play as; ~6–8 years old; energetic, cheerful, friendly.
- **Style:** blocky / voxel-inspired (Minecraft-like), pixel-art face, flat colour areas,
  no outlines, soft daylight.
- **Proportions:** identical to `player_girl` (ART-RIG §2.4 — both share the rest pose):
  - height 1.20 m to top of skull (hair tuft ≤ 4 cm more); head block 0.34 m high, 0.34
    wide, 0.32 deep → head-to-body ≈ 1 : 3.5
  - torso block 0.30 wide × 0.34 high × 0.18 deep (hips at 0.50 m, shoulders at 0.84 m)
  - arms 0.10 × 0.10 blocks, shoulder to wrist 0.34 m, cube hands
  - legs 0.12 × 0.12 blocks, hip to ankle 0.41 m, feet/shoes 0.08 high
- **Pose for all views:** A-pose — standing straight, legs straight at hip width, arms
  straight and 45° down from horizontal, palms down, head straight, neutral face.
- **Silhouette key:** short hair with a tuft on top (back of the neck visible), bare knees
  (shorts), warm yellow shirt — reads differently from the girl from every side.

## Clothing and details

- Hair: short chestnut brown, a blocky tuft sticking up at the front, short sides, ears
  visible. **No hat.**
- Face: same style as the girl — 2 × 2 px dark brown eyes with a white highlight pixel,
  short chestnut brows, small rose mouth.
- T-shirt: mustard yellow, short sleeves to mid upper arm, one broad green horizontal band
  across the chest and a green edge on each sleeve, untucked.
- Navy knee-length shorts; skin-coloured knees.
- White socks to mid-calf with one green stripe; green sneakers with white soles.

## Colours (proposal — map to the shared palette once it exists, ADIR-002)

| Part | Hex |
|---|---|
| Skin / shade | `#F2C29B` / `#D9A27E` |
| Hair / highlight | `#8A4B22` / `#A8612F` |
| Eyes | `#2B1B12` (+ white `#FFFFFF` highlight) |
| Mouth | `#C8645A` |
| Shirt / band | `#F2B632` / `#3F8F3A` |
| Shorts | `#2E3F6E` |
| Socks / stripe | `#FFFFFF` / `#3F8F3A` |
| Sneakers / soles | `#3F8F3A` / `#F4F4F0` |

## Image-generation prompts

Use the **same tool, settings and framing as `player_girl`** so both sheets line up:
portrait **1024 × 1536**, character centred, feet on the same baseline at ~92 % of the
image height, top of the head at ~12 %. Save as `front.png`, `side.png`, `back.png`,
`three_quarter.png`, `expressions.png` in this folder, plus `palette.png` (or keep the
colour table above).

**Shared style block** (append to every prompt — identical to `player_girl`):

> blocky voxel-style 3D character, Minecraft-inspired, cube head, box-shaped torso and
> limbs, pixel-art face, flat colours, no outlines, soft even studio light, orthographic
> camera, full body visible, A-pose with arms straight and 45 degrees down from
> horizontal, legs straight at hip width, neutral expression, plain light grey background
> (#E6E6E6), no shadow on the background, no props, no text, character turnaround
> reference sheet style

**Negative prompt:**

> realistic, smooth skin, anime, outlines, dynamic pose, T-pose, holding objects, hat,
> cap, backpack, jewellery, text, logo, watermark, gradient background, scenery,
> perspective distortion, cropped feet, extra fingers, adult proportions

**Character block** (insert before the style block):

> a 7-year-old boy, head about one third of his body height, short chestnut brown hair
> with a blocky tuft sticking up at the front and short sides, ears visible, light peach
> skin, small dark brown pixel eyes with a white highlight, small rose mouth, mustard
> yellow short-sleeved T-shirt with one broad green horizontal band across the chest and
> green sleeve edges, untucked, navy blue knee-length shorts, bare knees, white socks to
> mid-calf with a green stripe, green sneakers with white soles

| File | Prompt |
|---|---|
| `front.png` | "Front view, facing the camera directly, " + character block + style block |
| `side.png` | "Exact left side profile view, character facing to the left of the image, " + character block + style block + ", the hair tuft visible above the forehead" |
| `back.png` | "Back view, character facing away from the camera, " + character block + style block + ", short hair with the back of the neck visible, no face visible" |
| `three_quarter.png` | "Three-quarter front view, character turned 45 degrees to the left, " + character block + style block |
| `expressions.png` | "Expression sheet of the same blocky voxel boy's cube head only, 8 heads in a 4 by 2 grid, all the same size and facing the camera, pixel-art faces: 1 neutral, 2 eyes closed blink, 3 happy smile, 4 laughing with open mouth, 5 talking with small open mouth, 6 surprised with round eyes and round mouth, 7 thinking with eyes looking up and mouth to one side, 8 slightly sad with lowered brows (gentle, not crying), " + character block + ", flat colours, plain light grey background (#E6E6E6), no text" |

**Tip for consistency:** if the tool drifts between separate generations, generate one
wide sheet (2048 × 1024) with the prompt "Character turnaround sheet with four views side
by side in one row, same scale and baseline: front, left side, back, three-quarter, " +
character block + style block, then crop it into the four files. For a direct
comparison, also generate "the blocky voxel girl and boy standing side by side in
A-pose, front view, same height" (girl character block + "and" + boy character block +
style block) — useful for the choice-screen check (ACHAR-005).

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Clearly a matching pair with `player_girl` (same body, same face style, same height).
- [ ] Same scale, baseline and A-pose in all four views; plain background.
- [ ] Head-to-body ≈ 1 : 3.5; no headwear.
- [ ] Distinguishable from `player_girl` from the front and from behind.
- [ ] Expression sheet has all 8 expressions in the order of ART-RIG §6.2.
- [ ] Rigid-skinning exceptions: _none planned_.
