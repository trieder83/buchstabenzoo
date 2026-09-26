# Brief — `monkey` (animal turnaround)

Spec: ART-ANIMALS (look, animations), ART-PIPELINE §3 (turnaround), GAME-ANIMALS, CONT-MISSIONS
(food box: **Bananen**; hiding place: pirate ship (`loc_pirate_ship`), top of the mast, "playing pirate"). Style: `art/style/style.md` (comic) —
style references: `art/environment/style_frame/style_frame.png` (approved) and the approved zebra
sheet `art/animals/zebra/sheet_v1.jpg` (line weight, eyes, colouring, layout). Status: **in-review**.

> **Game size:** 1.1 m (user decision 2026-09-26) — comic-scaled up for readability; see ART-ANIMALS "Game sizes". Concept approved 2026-09-26.

## Character

- Cheeky, playful, full of energy — the clown of the zoo.
- Comic proportions: small chunky body, big round head with a big heart-shaped light face, big round ears, long arms, short legs, long tail curling into a spiral at the end.
- **Readability (~60 px, 55° camera):** brown shape with a light face and a long spiral tail — tail and big round ears are the identifying features (important when it sits on the mast).
- Pose decision (proposal): drawn **standing upright on two feet** (comic monkey), arms hanging down. This means a biped rig, not `quadruped` — user decides (Q-043). No pirate accessories on the base model (a hat/bandana could be a separate prop).
- Baby (Q-040, only if `quest_monkey_baby` stays): same design, about half the height, bigger head in relation to the body, lighter face — prompt 2, one cheap sheet for review.
- Friendly comic animal like the zebra: big round eyes with white highlight, mouth closed, no
  teeth shown, chunky rounded shapes, flat colours with one hard shadow tone.
- Size: Standing height ≈ 0.9 m to the top of the head (real small monkeys ~0.5 m; comic: bigger). About the chest of the 1.2 m girl. Baby ≈ 0.45 m.
- Animations (ART-ANIMALS, provisional Q-043): `idle`, `walk`, `climb`, `eat`, `happy`; baby: `hide`, `wave`.
- Views: Monkey: bipedal standing pose (see Character).

## Colours (proposal — shared palette later, ADIR-002)

| Part | Colour |
|---|---|
| Fur | `#8A5A34` warm brown |
| Face, ears inside, belly, hands, feet | `#F1D2A8` light peach |
| Tail tip, darker hair tuft | `#6B4226` dark brown |
| Nose, mouth line | `#5A3A28` |
| Eyes | `#1E1E1E` with white highlight |
| Outline | dark brown (renderer) |

## Image-generation prompts

One **sheet** at **21:9, 2K** with all four views (consistent angles), then split with
`tools/split_sheet.py` into single views. Prompt = the CHARACTER SHEET STYLE block (verbatim) +
the views paragraph + the design paragraph. Reference images: `style_frame.png` and the
approved zebra `sheet_v1.jpg` (with the `--extra` text in the generation log).

**Negative prompt:**

```text
text, letters, logo, watermark, labels, scenery, grass, ground plane, gradient background, cropped ears, cropped tail, extra views, more than four figures, extra legs, missing legs, extra heads, pirate hat, clothes, banana, holding objects, climbing, open mouth, teeth, angry, scary, realistic monkey, gorilla, chimpanzee, ape without tail, four legs on ground, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

### `sheet.png` (all four views in one image)

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Turnaround sheet of one monkey: four views of the SAME monkey side by side in one row, evenly spaced, same scale, feet on the same baseline, each view in its own quarter of the image: 1) exact front view — the monkey faces the camera straight on, body symmetric, face centred, both feet side by side; 2) exact left side profile — the monkey faces the left edge of the image, no face turned towards the camera, tail visible behind; 3) exact back view — the back and curled tail face the camera, no face visible at all; 4) three-quarter front view — turned 45 degrees to the left of the viewer. A friendly cartoon monkey, standing upright on its two feet flat on the ground, legs straight, arms hanging relaxed at the sides, head up, looking forward, mouth closed, neutral happy cheeky smile. Small chunky rounded body, short legs with big feet, long arms with big hands, big round head with a large light heart-shaped face, big round ears with light insides, big round expressive eyes with a white highlight, a small tuft of hair on top, long tail curling into a spiral at the end. Warm brown fur (#8A5A34) with a light peach face, ear insides, belly, hands and feet (#F1D2A8), flat colours. Standing height about 0.9 m; next to a 1.2 m tall child it would reach about to her chest. Cheeky, playful and friendly.
```

### `baby_sheet.png` (optional — only if the baby stays, Q-040)

Reference: the chosen adult sheet.

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Turnaround sheet of one baby monkey: four views of the SAME baby monkey side by side in one row, evenly spaced, same scale, feet on the same baseline, each view in its own quarter of the image: 1) exact front view — facing the camera straight on, body symmetric, face centred; 2) exact left side profile — facing the left edge of the image, tail visible behind; 3) exact back view — back and curled tail face the camera, no face visible at all; 4) three-quarter front view — turned 45 degrees to the left of the viewer. A friendly cartoon baby monkey, the child of the adult monkey in the reference image: same design and colours, but only half as tall, much bigger head in relation to its small round body, extra big eyes, short limbs, standing upright on its two feet, arms relaxed, mouth closed, cute shy smile, long tail curling into a spiral. Warm brown fur (#8A5A34) with a light peach face, ear insides, belly, hands and feet (#F1D2A8), flat colours. Standing height about 0.45 m. Very cute, shy and friendly.
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Same monkey in all four views (proportions, markings, eyes).
- [ ] Comic style matches `style_frame.png` and the zebra sheet (outlines, flat colours, one shadow tone).
- [ ] Identifying features readable when shrunk to ≈ 60 px.
- [ ] Friendly face, no teeth, not scary.
- [ ] Correct view angles; nothing cropped; plain grey background; no text.
- [ ] Buildable as a low-poly model (≤ 3 000 tris); markings as flat-colour texture.
- [ ] Size relative to the 1.2 m girl OK (see `art/animals/lineup.png`).

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-09-26 | sheet_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, zebra/sheet_v1.jpg | alternative — good; tail hidden in the front view |
| 2026-09-26 | sheet_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, zebra/sheet_v1.jpg | **chosen** — split into the four views; spiral tail visible in all views |
| 2026-09-26 | baby_sheet_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 2 + negative as 'Avoid' + extra text, ref: style_frame.png, sheet_v2.jpg | discarded (deleted) — near-copy of the adult, no baby proportions |
| 2026-09-26 | baby_sheet_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 2 + negative as 'Avoid' + extra text, ref: front.png | **baby proposal** (Q-040) — clear baby proportions (big head, chubby limbs); not split |

`--extra` for all first sheets: the two-reference-images text from the task ("…match its comic style, line weight, colouring, proportions style, eye style, grey background and the four-views-in-a-row layout exactly, but draw the animal described above.").
