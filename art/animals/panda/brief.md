# Brief — `panda` (animal turnaround)

Spec: ART-ANIMALS (look, animations), ART-PIPELINE §3 (turnaround), GAME-ANIMALS, CONT-MISSIONS
(food box: **Bambus**; hiding place: cave (`loc_cave`), sleeping). Style: `art/style/style.md` (comic) —
style references: `art/environment/style_frame/style_frame.png` (approved) and the approved zebra
sheet `art/animals/zebra/sheet_v1.jpg` (line weight, eyes, colouring, layout). Status: **in-review**.

## Character

- Calm, cuddly, a little sleepy — the teddy bear of the zoo.
- Comic proportions: round chunky body, short thick legs, very big round head with round ears.
- **Readability (~60 px, 55° camera):** white body with black legs and a black shoulder band, black round ears and big black eye patches — the eye patches and black ears are the identifying features.
- Friendly comic animal like the zebra: big round eyes with white highlight, mouth closed, no
  teeth shown, chunky rounded shapes, flat colours with one hard shadow tone.
- Size: Standing on all fours: back height ≈ 0.85 m, top of the ears ≈ 1.1 m, length ≈ 1.5 m (real ~0.8 m / 1.6 m). Its head reaches about to the chest of the 1.2 m girl.
- Animations (ART-ANIMALS, provisional Q-043): `idle`, `walk`, `eat`, `happy` (+ `sleep` proposed as hiding-place idle in the cave, Q-043).
- Views: Quadruped: front, left side, back, ¾ — standing pose, all four feet on the ground (ART-ANIMALS §3).

## Colours (proposal — shared palette later, ADIR-002)

| Part | Colour |
|---|---|
| Body, face | `#F6F3EA` warm white |
| Legs, shoulder band, ears, eye patches, nose | `#262626` soft black |
| Eyes | `#1E1E1E` with white highlight, inside the black eye patch with a white ring |
| Paw pads | `#4A4A4A` dark grey |
| Outline | dark brown (renderer) |

## Image-generation prompts

One **sheet** at **21:9, 2K** with all four views (consistent angles), then split with
`tools/split_sheet.py` into single views. Prompt = the CHARACTER SHEET STYLE block (verbatim) +
the views paragraph + the design paragraph. Reference images: `style_frame.png` and the
approved zebra `sheet_v1.jpg` (with the `--extra` text in the generation log).

**Negative prompt:**

```text
text, letters, logo, watermark, labels, scenery, grass, ground plane, gradient background, cropped ears, cropped tail, extra views, more than four figures, extra legs, missing legs, extra heads, bamboo, holding objects, sitting, standing on hind legs, open mouth, teeth, claws, angry, scary, realistic bear, red panda, grey fur, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

### `sheet.png` (all four views in one image)

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Turnaround sheet of one panda: four views of the SAME panda side by side in one row, evenly spaced, same scale, feet on the same baseline, each view in its own quarter of the image: 1) exact front view — the panda faces the camera straight on, body symmetric, face centred, both front legs side by side, body hidden behind the chest; 2) exact left side profile — the panda faces the left edge of the image, all four legs visible, no face turned towards the camera; 3) exact back view — the rump and tail face the camera, no face visible at all; 4) three-quarter front view — turned 45 degrees to the left of the viewer. A friendly cartoon giant panda, standing calmly on all four paws flat on the ground, legs straight, head up, looking forward, mouth closed, neutral happy smile. Round chunky body, short thick legs, very big round head, round black ears, big round expressive eyes with a white highlight sitting inside large black teardrop-shaped eye patches, small black nose, short white stubby tail. Warm white body and head (#F6F3EA); all four legs black (#262626) and a broad black band over the shoulders connecting the front legs, the classic simple panda pattern with large clean shapes. Back height about 0.85 m; next to a 1.2 m tall child its head would reach about to her chest. Calm, cuddly and gentle.
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Same panda in all four views (proportions, markings, eyes).
- [ ] Comic style matches `style_frame.png` and the zebra sheet (outlines, flat colours, one shadow tone).
- [ ] Identifying features readable when shrunk to ≈ 60 px.
- [ ] Friendly face, no teeth, not scary.
- [ ] Correct view angles; nothing cropped; plain grey background; no text.
- [ ] Buildable as a low-poly model (≤ 3 000 tris); markings as flat-colour texture.
- [ ] Size relative to the 1.2 m girl OK (see `art/animals/lineup.png`).

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-09-26 | sheet_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, zebra/sheet_v1.jpg | alternative — side and ¾ face right (would need mirroring), weaker back view |
| 2026-09-26 | sheet_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, zebra/sheet_v1.jpg | **chosen** — split into the four views; all angles correct, side faces left |

`--extra` for all first sheets: the two-reference-images text from the task ("…match its comic style, line weight, colouring, proportions style, eye style, grey background and the four-views-in-a-row layout exactly, but draw the animal described above.").
