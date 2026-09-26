# Brief — `giraffe` (animal turnaround)

Spec: ART-ANIMALS (look, animations), ART-PIPELINE §3 (turnaround), GAME-ANIMALS, CONT-MISSIONS
(food box: **Blätter**; hiding place: playground (`loc_playground`), curious, nibbling leaves). Style: `art/style/style.md` (comic) —
style references: `art/environment/style_frame/style_frame.png` (approved) and the approved zebra
sheet `art/animals/zebra/sheet_v1.jpg` (line weight, eyes, colouring, layout). Status: **in-review**.

## Character

- Curious, a little clumsy, sweet — followed the sound of laughing children.
- Comic proportions: long neck (shortened a bit for the camera), chunky body, long but sturdy legs, round head with two ossicones, big eyes with lashes.
- **Readability (~60 px, 55° camera):** tall yellow shape with **few, large** brown patches — the long neck and big patches are the identifying features (small patches blur at 60 px).
- Friendly comic animal like the zebra: big round eyes with white highlight, mouth closed, no
  teeth shown, chunky rounded shapes, flat colours with one hard shadow tone.
- Size: Back height ≈ 2.3 m, top of the ossicones ≈ 4.5 m (real ~5 m — the riddle math uses 5 m; comic: neck shortened). Almost four times the height of the 1.2 m girl, much taller than the 1.8 m slide.
- Animations (ART-ANIMALS, provisional Q-043): `idle`, `walk`, `eat`, `happy` (`eat` = nibble leaves from a tree, head high).
- Views: Quadruped: front, left side, back, ¾ — standing pose, all four feet on the ground (ART-ANIMALS §3).

## Colours (proposal — shared palette later, ADIR-002)

| Part | Colour |
|---|---|
| Body base | `#F2C75C` warm yellow |
| Patches, mane, tail tuft | `#B8702E` warm brown |
| Muzzle, belly | `#F7E3B0` light cream |
| Ossicone tips, hooves | `#6B4424` dark brown |
| Eyes | `#1E1E1E` with white highlight |
| Outline | dark brown (renderer) |

## Image-generation prompts

One **sheet** at **21:9, 2K** with all four views (consistent angles), then split with
`tools/split_sheet.py` into single views. Prompt = the CHARACTER SHEET STYLE block (verbatim) +
the views paragraph + the design paragraph. Reference images: `style_frame.png` and the
approved zebra `sheet_v1.jpg` (with the `--extra` text in the generation log).

**Negative prompt:**

```text
text, letters, logo, watermark, labels, scenery, grass, ground plane, gradient background, cropped ears, cropped tail, extra views, more than four figures, extra legs, missing legs, extra heads, tree, leaves, open mouth, tongue out, teeth, angry, scary, realistic giraffe, many tiny spots, net pattern, thin spots, cropped head, cropped hooves, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

### `sheet.png` (all four views in one image)

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Turnaround sheet of one giraffe: four views of the SAME giraffe side by side in one row, evenly spaced, same scale, feet on the same baseline, each view in its own quarter of the image: 1) exact front view — the giraffe faces the camera straight on, body symmetric, face centred, both front legs side by side, body hidden behind the chest; 2) exact left side profile — the giraffe faces the left edge of the image, all four legs visible, no face turned towards the camera; 3) exact back view — the rump and tail face the camera, no face visible at all; 4) three-quarter front view — turned 45 degrees to the left of the viewer. A friendly cartoon giraffe, standing calmly with all four hooves flat on the ground, legs straight, long neck upright, head up, looking forward, mouth closed, neutral happy smile. Chunky rounded body, long but sturdy legs, long neck, round head with a light cream muzzle, two short rounded ossicones with dark brown tips, small ears, big round expressive eyes with a white highlight and long eyelashes, short brown mane along the neck, thin tail with a dark brown tuft. Warm yellow coat (#F2C75C) with FEW LARGE rounded brown patches (#B8702E) — about 10 to 12 big patches on the body and 5 to 6 on the neck, clean simple shapes, no tiny spots; legs yellow with a few patches, dark brown hooves. Top of the head about 4.5 m; next to a 1.2 m tall child it would be almost four times her height. Gentle, curious and sweet.
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Same giraffe in all four views (proportions, markings, eyes).
- [ ] Comic style matches `style_frame.png` and the zebra sheet (outlines, flat colours, one shadow tone).
- [ ] Identifying features readable when shrunk to ≈ 60 px.
- [ ] Friendly face, no teeth, not scary.
- [ ] Correct view angles; nothing cropped; plain grey background; no text.
- [ ] Buildable as a low-poly model (≤ 3 000 tris); markings as flat-colour texture.
- [ ] Size relative to the 1.2 m girl OK (see `art/animals/lineup.png`).

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-09-26 | sheet_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, zebra/sheet_v1.jpg | **chosen** — split into the four views; few large patches (readable at 60 px) |
| 2026-09-26 | sheet_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, zebra/sheet_v1.jpg | alternative — many smaller spots (reads worse at 60 px) |

`--extra` for all first sheets: the two-reference-images text from the task ("…match its comic style, line weight, colouring, proportions style, eye style, grey background and the four-views-in-a-row layout exactly, but draw the animal described above.").
