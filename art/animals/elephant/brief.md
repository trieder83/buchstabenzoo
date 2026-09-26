# Brief — `elephant` (animal turnaround)

Spec: ART-ANIMALS (look, animations), ART-PIPELINE §3 (turnaround), GAME-ANIMALS, CONT-MISSIONS
(food box: **Heu**; hiding place: mud pool (`loc_mud_pool`), rolling in mud). Style: `art/style/style.md` (comic) —
style references: `art/environment/style_frame/style_frame.png` (approved) and the approved zebra
sheet `art/animals/zebra/sheet_v1.jpg` (line weight, eyes, colouring, layout). Status: **in-review**.

## Character

- Big, gentle, playful — likes splashing in mud.
- Comic proportions: big round body, thick pillar legs, big head with huge round flappy ears and a thick curved trunk.
- **Readability (~60 px, 55° camera):** the biggest grey shape with two huge ears seen from above and a trunk — ears and trunk are the identifying features. No tusks (friendlier, no sharp shapes).
- Friendly comic animal like the zebra: big round eyes with white highlight, mouth closed, no
  teeth shown, chunky rounded shapes, flat colours with one hard shadow tone.
- Size: Back height ≈ 2.5 m, top of the head ≈ 3.0 m, length ≈ 3.5 m (real African elephant ~3 m; comic: slightly smaller so it fits the camera). About twice the height of the 1.2 m girl.
- Animations (ART-ANIMALS, provisional Q-043): `idle`, `walk`, `eat`, `drink`, `happy` (+ `splash`/`roll` proposed as hiding-place idle at the mud pool, Q-043).
- Views: Quadruped: front, left side, back, ¾ — standing pose, all four feet on the ground (ART-ANIMALS §3).

## Colours (proposal — shared palette later, ADIR-002)

| Part | Colour |
|---|---|
| Body | `#9AA3AE` blue-grey |
| Ear insides | `#E7A9B0` soft pink |
| Toenails | `#EDE3D2` cream |
| Tail tuft | `#4A4F57` dark grey |
| Eyes | `#1E1E1E` with white highlight |
| Outline | dark brown (renderer) |

## Image-generation prompts

One **sheet** at **21:9, 2K** with all four views (consistent angles), then split with
`tools/split_sheet.py` into single views. Prompt = the CHARACTER SHEET STYLE block (verbatim) +
the views paragraph + the design paragraph. Reference images: `style_frame.png` and the
approved zebra `sheet_v1.jpg` (with the `--extra` text in the generation log).

**Negative prompt:**

```text
text, letters, logo, watermark, labels, scenery, grass, ground plane, gradient background, cropped ears, cropped tail, extra views, more than four figures, extra legs, missing legs, extra heads, tusks, ivory, open mouth, teeth, angry, scary, realistic elephant, wrinkles, rough skin texture, mud, water, saddle, rider, raised trunk, trumpeting, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

### `sheet.png` (all four views in one image)

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Turnaround sheet of one elephant: four views of the SAME elephant side by side in one row, evenly spaced, same scale, feet on the same baseline, each view in its own quarter of the image: 1) exact front view — the elephant faces the camera straight on, body symmetric, face centred, both front legs side by side, body hidden behind the chest; 2) exact left side profile — the elephant faces the left edge of the image, all four legs visible, no face turned towards the camera; 3) exact back view — the rump and tail face the camera, no face visible at all; 4) three-quarter front view — turned 45 degrees to the left of the viewer. A friendly cartoon elephant, standing calmly with all four feet flat on the ground, legs straight, head up, looking forward, mouth closed, neutral happy smile, trunk hanging down relaxed with a small curl at the tip. Big round chunky body, thick short pillar legs with three cream toenails on each foot, big round head, huge round flappy ears with pink insides, thick trunk, big round expressive eyes with a white highlight and small eyelashes, no tusks, thin tail with a dark tuft. Blue-grey body (#9AA3AE), flat colours, no wrinkles. Back height about 2.5 m; next to a 1.2 m tall child it would be about twice her height. Gentle, kind and playful, not wild.
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Same elephant in all four views (proportions, markings, eyes).
- [ ] Comic style matches `style_frame.png` and the zebra sheet (outlines, flat colours, one shadow tone).
- [ ] Identifying features readable when shrunk to ≈ 60 px.
- [ ] Friendly face, no teeth, not scary.
- [ ] Correct view angles; nothing cropped; plain grey background; no text.
- [ ] Buildable as a low-poly model (≤ 3 000 tris); markings as flat-colour texture.
- [ ] Size relative to the 1.2 m girl OK (see `art/animals/lineup.png`).

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-09-26 | sheet_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, zebra/sheet_v1.jpg | **chosen** — split with connected-component masking (ears/trunk overlap neighbours) |
| 2026-09-26 | sheet_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, zebra/sheet_v1.jpg | alternative — ¾ view has a slightly open mouth |

`--extra` for all first sheets: the two-reference-images text from the task ("…match its comic style, line weight, colouring, proportions style, eye style, grey background and the four-views-in-a-row layout exactly, but draw the animal described above.").
