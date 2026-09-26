# Brief — `hippo` (animal turnaround)

Spec: ART-ANIMALS (look, animations), ART-PIPELINE §3 (turnaround), GAME-ANIMALS, CONT-MISSIONS
(food box: **Melonen**; hiding place: pond (`loc_pond`), bathing among water lilies). Style: `art/style/style.md` (comic) —
style references: `art/environment/style_frame/style_frame.png` (approved) and the approved zebra
sheet `art/animals/zebra/sheet_v1.jpg` (line weight, eyes, colouring, layout). Status: **in-review**.

## Character

- Good-natured, sleepy, a little shy — loves to lie in still water with only eyes and ears showing.
- Comic proportions: huge round barrel body, very short stumpy legs, big wide square muzzle, tiny round ears on top, big eyes set high on the head (so they can peek out of the water).
- **Readability (~60 px, 55° camera):** big round lilac-grey blob with a wide pink muzzle and tiny ears on top — the wide muzzle and barrel body are the two identifying features.
- Friendly comic animal like the zebra: big round eyes with white highlight, mouth closed, no
  teeth shown, chunky rounded shapes, flat colours with one hard shadow tone.
- Size: Back height ≈ 1.4 m, top of the ears ≈ 1.7 m, length ≈ 3.0 m (real hippo ~1.5 m / 3.5 m; comic: shorter and rounder). Back slightly above the head of the 1.2 m girl.
- Animations (ART-ANIMALS, provisional Q-043): `idle`, `walk`, `eat`, `swim`, `happy` (+ `bathe` proposed as hiding-place idle at the pond, Q-043).
- Views: Quadruped: front, left side, back, ¾ — standing pose, all four feet on the ground (ART-ANIMALS §3).

## Colours (proposal — shared palette later, ADIR-002)

| Part | Colour |
|---|---|
| Body | `#9C8FB0` lilac-grey |
| Belly, muzzle | `#E9A6AE` soft pink |
| Ear insides, nostrils | `#C97A86` darker pink |
| Toenails | `#EDE3D2` cream |
| Eyes | `#1E1E1E` with white highlight |
| Outline | dark brown (renderer) |

## Image-generation prompts

One **sheet** at **21:9, 2K** with all four views (consistent angles), then split with
`tools/split_sheet.py` into single views. Prompt = the CHARACTER SHEET STYLE block (verbatim) +
the views paragraph + the design paragraph. Reference images: `style_frame.png` and the
approved zebra `sheet_v1.jpg` (with the `--extra` text in the generation log).

**Negative prompt:**

```text
text, letters, logo, watermark, labels, scenery, grass, ground plane, gradient background, cropped ears, cropped tail, extra views, more than four figures, extra legs, missing legs, extra heads, open mouth, big tusks, teeth, angry, scary, realistic hippo, wrinkles, warts, mud, water, swimming, lying down, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

### `sheet.png` (all four views in one image)

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Turnaround sheet of one hippo: four views of the SAME hippo side by side in one row, evenly spaced, same scale, feet on the same baseline, each view in its own quarter of the image: 1) exact front view — the hippo faces the camera straight on, body symmetric, face centred, both front legs side by side, body hidden behind the chest; 2) exact left side profile — the hippo faces the left edge of the image, all four legs visible, no face turned towards the camera; 3) exact back view — the rump and tail face the camera, no face visible at all; 4) three-quarter front view — turned 45 degrees to the left of the viewer. A friendly cartoon hippo, standing calmly with all four feet flat on the ground, legs straight, head up, looking forward, mouth closed, neutral happy smile. Huge round chunky barrel body, very short stumpy legs with three cream toenails on each foot, big wide rounded muzzle, big round expressive eyes with a white highlight set high on the head, tiny round ears with pink insides, two small nostrils on top of the muzzle, short thin tail with a small tuft. Lilac-grey body (#9C8FB0) with a soft pink belly and muzzle (#E9A6AE), flat colours, no wrinkles, no spots. Back height about 1.4 m; next to a 1.2 m tall child its back would be a little above her head. Gentle, sleepy and good-natured, not wild.
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Same hippo in all four views (proportions, markings, eyes).
- [ ] Comic style matches `style_frame.png` and the zebra sheet (outlines, flat colours, one shadow tone).
- [ ] Identifying features readable when shrunk to ≈ 60 px.
- [ ] Friendly face, no teeth, not scary.
- [ ] Correct view angles; nothing cropped; plain grey background; no text.
- [ ] Buildable as a low-poly model (≤ 3 000 tris); markings as flat-colour texture.
- [ ] Size relative to the 1.2 m girl OK (see `art/animals/lineup.png`).

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-09-26 | sheet_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, zebra/sheet_v1.jpg | **chosen** — split into front/side/back/three_quarter.png; side is a clean profile (one eye) |
| 2026-09-26 | sheet_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, zebra/sheet_v1.jpg | alternative — side view slightly turned towards the camera (both eyes) |

`--extra` for all first sheets: the two-reference-images text from the task ("…match its comic style, line weight, colouring, proportions style, eye style, grey background and the four-views-in-a-row layout exactly, but draw the animal described above.").
