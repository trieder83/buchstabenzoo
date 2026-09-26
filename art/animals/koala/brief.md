# Brief — `koala` (animal turnaround)

Spec: ART-ANIMALS (look, animations), ART-PIPELINE §3 (turnaround), GAME-ANIMALS, CONT-MISSIONS
(food box: **Eukalyptus**; hiding place: tallest tree (`loc_tallest_tree`), sleeping in a fork of branches). Style: `art/style/style.md` (comic) —
style references: `art/environment/style_frame/style_frame.png` (approved) and the approved zebra
sheet `art/animals/zebra/sheet_v1.jpg` (line weight, eyes, colouring, layout). Status: **in-review**.

> **Game size:** 0.9 m, on all fours (user decision 2026-09-26) — comic-scaled up for readability; see ART-ANIMALS "Game sizes". Concept approved 2026-09-26.

## Character

- Sleepy, cosy, very cute — sleeps up to 20 hours a day.
- Comic proportions: small round body, very big round head, huge fluffy round ears, big black oval nose; short limbs with big grabbing paws.
- **Readability (~60 px, 55° camera):** grey ball with two huge fluffy white-rimmed ears and a big black nose — ears and nose are the identifying features. It is small, so the head is extra big.
- Pose decision (proposal): drawn **standing on all four paws** so it fits the `quadruped` rig and can follow the player on the ground; `climb` is a separate clip. Alternative for the user: sitting upright pose (more typical koala, but needs its own rig — see Q-043).
- Friendly comic animal like the zebra: big round eyes with white highlight, mouth closed, no
  teeth shown, chunky rounded shapes, flat colours with one hard shadow tone.
- Size: On all fours: back height ≈ 0.45 m, top of the ears ≈ 0.65 m, length ≈ 0.7 m (real ~0.4 m; comic: slightly bigger). About knee-to-hip height of the 1.2 m girl.
- Animations (ART-ANIMALS, provisional Q-043): `idle`, `climb`, `eat`, `happy` (+ ground `walk` needed if it follows on foot, Q-043).
- Views: Quadruped: front, left side, back, ¾ — standing pose, all four feet on the ground (ART-ANIMALS §3).

## Colours (proposal — shared palette later, ADIR-002)

| Part | Colour |
|---|---|
| Body, head | `#9EA3A8` soft grey |
| Ear fluff, chin, chest, belly | `#F2F0EB` off-white |
| Nose | `#2A2A2A` soft black |
| Paws, claws (short, rounded) | `#6E7278` dark grey |
| Eyes | `#1E1E1E` with white highlight |
| Outline | dark brown (renderer) |

## Image-generation prompts

One **sheet** at **21:9, 2K** with all four views (consistent angles), then split with
`tools/split_sheet.py` into single views. Prompt = the CHARACTER SHEET STYLE block (verbatim) +
the views paragraph + the design paragraph. Reference images: `style_frame.png` and the
approved zebra `sheet_v1.jpg` (with the `--extra` text in the generation log).

**Negative prompt:**

```text
text, letters, logo, watermark, labels, scenery, grass, ground plane, gradient background, cropped ears, cropped tail, extra views, more than four figures, extra legs, missing legs, extra heads, tree, branch, eucalyptus leaves, holding objects, sitting, standing on hind legs, open mouth, teeth, sharp claws, angry, scary, realistic koala, bear, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

### `sheet.png` (all four views in one image)

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Turnaround sheet of one koala: four views of the SAME koala side by side in one row, evenly spaced, same scale, feet on the same baseline, each view in its own quarter of the image: 1) exact front view — the koala faces the camera straight on, body symmetric, face centred, both front legs side by side, body hidden behind the chest; 2) exact left side profile — the koala faces the left edge of the image, all four legs visible, no face turned towards the camera; 3) exact back view — the rump and tail face the camera, no face visible at all; 4) three-quarter front view — turned 45 degrees to the left of the viewer. A friendly cartoon koala, standing calmly on all four paws flat on the ground, legs straight, head up, looking forward, mouth closed, neutral happy sleepy smile. Small round chunky body, short sturdy legs with big rounded paws, very big round head, huge round fluffy ears with off-white fluffy rims, a big black oval nose in the middle of the face, big round expressive eyes with a white highlight, no visible tail. Soft grey fur (#9EA3A8) with an off-white chin, chest and belly (#F2F0EB), flat colours. Back height about 0.45 m, top of the ears about 0.65 m; next to a 1.2 m tall child it would reach about to her hip. Cute, sleepy and gentle.
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Same koala in all four views (proportions, markings, eyes).
- [ ] Comic style matches `style_frame.png` and the zebra sheet (outlines, flat colours, one shadow tone).
- [ ] Identifying features readable when shrunk to ≈ 60 px.
- [ ] Friendly face, no teeth, not scary.
- [ ] Correct view angles; nothing cropped; plain grey background; no text.
- [ ] Buildable as a low-poly model (≤ 3 000 tris); markings as flat-colour texture.
- [ ] Size relative to the 1.2 m girl OK (see `art/animals/lineup.png`).

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-09-26 | sheet_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, zebra/sheet_v1.jpg | discarded (deleted) — inconsistent: upright biped in front/back, quadruped in side/¾ |
| 2026-09-26 | sheet_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, zebra/sheet_v1.jpg | alternative — consistent **upright (bipedal) teddy pose**; kept in case the user prefers the upright koala |
| 2026-09-26 | sheet_v3.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, zebra/sheet_v1.jpg | **chosen** — extra text forced all-fours pose; split with connected-component masking (figures overlap); side head was turned away → replaced by side_fix |
| 2026-09-26 | sheet_v4.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, zebra/sheet_v1.jpg | discarded (deleted) — upright again, side faces right, drawn baseline line |
| 2026-09-26 | side_fix.jpg | gemini-3-pro-image (2K, 3:4) | — | edit: Redraw the first image (the left side profile of this koala, which is the second…, ref: side.png, sheet_v3.jpg | used — head redrawn in true left profile, rescaled to the sheet scale/baseline and saved as side.png (source file deleted) |

sheet_v3/v4 used the same `--extra` plus: "IMPORTANT: the koala walks on ALL FOUR PAWS like the zebra in the reference — a four-legged horizontal body, front paws and hind paws all on the ground in every view, NOT standing upright on two legs like a teddy bear…".

`--extra` for all first sheets: the two-reference-images text from the task ("…match its comic style, line weight, colouring, proportions style, eye style, grey background and the four-views-in-a-row layout exactly, but draw the animal described above.").
