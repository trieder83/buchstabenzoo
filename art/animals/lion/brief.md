# Brief — `lion` (animal turnaround)

Spec: ART-ANIMALS (look, animations), ART-PIPELINE §3 (turnaround), GAME-ANIMALS, CONT-MISSIONS
(food box: **Fleisch**; hiding place: sun rocks (`loc_sun_rocks`), lazy, sunbathing). Style: `art/style/style.md` (comic) —
style references: `art/environment/style_frame/style_frame.png` (approved) and the approved zebra
sheet `art/animals/zebra/sheet_v1.jpg` (line weight, eyes, colouring, layout). Status: **in-review**.

## Character

- Lazy, proud but kind, a big cuddly cat — the king of the animals, today mostly sleepy.
- Comic proportions: chunky body, sturdy legs with big round paws, big round head with a huge round fluffy mane.
- **Readability (~60 px, 55° camera):** golden body with a big orange-brown mane ring around the head — the mane is the identifying feature (reads like a sun). No teeth, no claws.
- Friendly comic animal like the zebra: big round eyes with white highlight, mouth closed, no
  teeth shown, chunky rounded shapes, flat colours with one hard shadow tone.
- Size: Back height ≈ 1.1 m, top of the mane ≈ 1.5 m, length ≈ 2.2 m (real ~1.2 m). Its back is about at the shoulder of the 1.2 m girl.
- Animations (ART-ANIMALS, provisional Q-043): `idle`, `walk`, `eat`, `happy` (+ `sleep`/`lie` proposed as hiding-place idle on the sun rocks, Q-043).
- Views: Quadruped: front, left side, back, ¾ — standing pose, all four feet on the ground (ART-ANIMALS §3).

## Colours (proposal — shared palette later, ADIR-002)

| Part | Colour |
|---|---|
| Body | `#E9B656` golden yellow |
| Mane, tail tuft | `#C06A2B` warm orange-brown |
| Muzzle, belly, paws | `#F6E2B4` light cream |
| Nose | `#9A5A4A` dark rose-brown |
| Eyes | `#1E1E1E` with white highlight |
| Outline | dark brown (renderer) |

## Image-generation prompts

One **sheet** at **21:9, 2K** with all four views (consistent angles), then split with
`tools/split_sheet.py` into single views. Prompt = the CHARACTER SHEET STYLE block (verbatim) +
the views paragraph + the design paragraph. Reference images: `style_frame.png` and the
approved zebra `sheet_v1.jpg` (with the `--extra` text in the generation log).

**Negative prompt:**

```text
text, letters, logo, watermark, labels, scenery, grass, ground plane, gradient background, cropped ears, cropped tail, extra views, more than four figures, extra legs, missing legs, extra heads, open mouth, roaring, fangs, teeth, claws, angry, scary, hunting pose, realistic lion, lioness without mane, meat, rocks, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

### `sheet.png` (all four views in one image)

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Turnaround sheet of one lion: four views of the SAME lion side by side in one row, evenly spaced, same scale, feet on the same baseline, each view in its own quarter of the image: 1) exact front view — the lion faces the camera straight on, body symmetric, face centred, both front legs side by side, body hidden behind the chest; 2) exact left side profile — the lion faces the left edge of the image, all four legs visible, no face turned towards the camera; 3) exact back view — the rump and tail face the camera, no face visible at all; 4) three-quarter front view — turned 45 degrees to the left of the viewer. A friendly cartoon lion, standing calmly on all four paws flat on the ground, legs straight, head up, looking forward, mouth closed, neutral happy smile. Chunky rounded body, sturdy legs with big round paws and no claws, big round head with a huge round fluffy mane made of a few big rounded tufts that frames the face like a sun, round ears, light cream muzzle with a small rose-brown nose, big round expressive eyes with a white highlight, long thin tail with an orange-brown tuft. Golden yellow body (#E9B656), warm orange-brown mane (#C06A2B), light cream muzzle and belly (#F6E2B4), flat colours. Back height about 1.1 m; next to a 1.2 m tall child its back would be about at her shoulder. Kind, lazy and cuddly, not dangerous.
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Same lion in all four views (proportions, markings, eyes).
- [ ] Comic style matches `style_frame.png` and the zebra sheet (outlines, flat colours, one shadow tone).
- [ ] Identifying features readable when shrunk to ≈ 60 px.
- [ ] Friendly face, no teeth, not scary.
- [ ] Correct view angles; nothing cropped; plain grey background; no text.
- [ ] Buildable as a low-poly model (≤ 3 000 tris); markings as flat-colour texture.
- [ ] Size relative to the 1.2 m girl OK (see `art/animals/lineup.png`).

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-09-26 | sheet_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, zebra/sheet_v1.jpg | **chosen** — mane in few big rounded tufts; split with connected-component masking (tail overlaps back view) |
| 2026-09-26 | sheet_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, zebra/sheet_v1.jpg | alternative — mane more strand-detailed |

`--extra` for all first sheets: the two-reference-images text from the task ("…match its comic style, line weight, colouring, proportions style, eye style, grey background and the four-views-in-a-row layout exactly, but draw the animal described above.").
