# Brief — `snow_fox` (animal turnaround)

Spec: ART-ANIMALS (look, animations), ART-PIPELINE §3 (turnaround), GAME-ANIMALS, CONT-MISSIONS
(food box: **Beeren**; hiding place: ice cream kiosk (`loc_ice_cream_kiosk`), cooling off near the freezer). Style: `art/style/style.md` (comic) —
style references: `art/environment/style_frame/style_frame.png` (approved) and the approved zebra
sheet `art/animals/zebra/sheet_v1.jpg` (line weight, eyes, colouring, layout). Status: **in-review**.

> **Game size:** 0.9 m (user decision 2026-09-26) — comic-scaled up for readability; see ART-ANIMALS "Game sizes". Concept approved 2026-09-26.

## Character

- Lively, clever, a little cheeky; comes from the snow and is too warm in the zoo.
- Comic proportions: small round body, short legs, big head with pointed rounded ears, very big fluffy tail (almost as big as the body).
- **Readability (~60 px, 55° camera):** small snow-white shape with a huge fluffy tail and pointed ears — tail and ears are the identifying features. White winter coat (clearly different from a red fox); pale blue-grey shadow tone so it does not vanish on light paths.
- Friendly comic animal like the zebra: big round eyes with white highlight, mouth closed, no
  teeth shown, chunky rounded shapes, flat colours with one hard shadow tone.
- Size: Back height ≈ 0.4 m, top of the ears ≈ 0.6 m, length ≈ 0.9 m with tail (real ~0.3 m; comic: slightly bigger). About knee-high to the 1.2 m girl.
- Animations (ART-ANIMALS, provisional Q-043): `idle`, `walk`, `eat`, `happy` (+ `pant`/`fan` proposed as hiding-place idle at the kiosk, Q-043).
- Views: Quadruped: front, left side, back, ¾ — standing pose, all four feet on the ground (ART-ANIMALS §3).

## Colours (proposal — shared palette later, ADIR-002)

| Part | Colour |
|---|---|
| Fur | `#FAFAF7` snow white |
| Shadow tone | `#C9D6E2` pale blue-grey |
| Ear insides | `#F1B8C0` soft pink |
| Nose | `#2A2A2A` soft black |
| Eyes | `#3A2A20` dark brown with white highlight |
| Outline | dark brown (renderer) |

## Image-generation prompts

One **sheet** at **21:9, 2K** with all four views (consistent angles), then split with
`tools/split_sheet.py` into single views. Prompt = the CHARACTER SHEET STYLE block (verbatim) +
the views paragraph + the design paragraph. Reference images: `style_frame.png` and the
approved zebra `sheet_v1.jpg` (with the `--extra` text in the generation log).

**Negative prompt:**

```text
text, letters, logo, watermark, labels, scenery, grass, ground plane, gradient background, cropped ears, cropped tail, extra views, more than four figures, extra legs, missing legs, extra heads, red fox, orange fur, brown fur, grey fur, snow, ice cream, open mouth, teeth, fangs, angry, scary, sly look, realistic fox, thin tail, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

### `sheet.png` (all four views in one image)

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Turnaround sheet of one snow fox: four views of the SAME snow fox side by side in one row, evenly spaced, same scale, feet on the same baseline, each view in its own quarter of the image: 1) exact front view — the snow fox faces the camera straight on, body symmetric, face centred, both front legs side by side, body hidden behind the chest; 2) exact left side profile — the snow fox faces the left edge of the image, all four legs visible, no face turned towards the camera; 3) exact back view — the rump and tail face the camera, no face visible at all; 4) three-quarter front view — turned 45 degrees to the left of the viewer. A friendly cartoon arctic snow fox, standing calmly on all four paws flat on the ground, legs straight, head up, looking forward, mouth closed, neutral happy smile. Small round chunky fluffy body, short sturdy legs, big round head with a short pointed muzzle and a small black nose, two big pointed ears with rounded tips and pink insides, big round expressive dark brown eyes with a white highlight, a huge fluffy bushy tail almost as big as the body. Snow-white fur (#FAFAF7) with a pale blue-grey shadow tone (#C9D6E2), a few big fluffy tufts on the chest and cheeks, flat colours. Back height about 0.4 m; next to a 1.2 m tall child it would reach about to her knee. Cute, lively and friendly.
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Same snow fox in all four views (proportions, markings, eyes).
- [ ] Comic style matches `style_frame.png` and the zebra sheet (outlines, flat colours, one shadow tone).
- [ ] Identifying features readable when shrunk to ≈ 60 px.
- [ ] Friendly face, no teeth, not scary.
- [ ] Correct view angles; nothing cropped; plain grey background; no text.
- [ ] Buildable as a low-poly model (≤ 3 000 tris); markings as flat-colour texture.
- [ ] Size relative to the 1.2 m girl OK (see `art/animals/lineup.png`).

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-09-26 | sheet_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, zebra/sheet_v1.jpg | alternative — back view is an unreadable white blob (tail covers everything) |
| 2026-09-26 | sheet_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, zebra/sheet_v1.jpg | **chosen** — split into the four views; note the tail is up in side/¾ but hanging in the back view |

`--extra` for all first sheets: the two-reference-images text from the task ("…match its comic style, line weight, colouring, proportions style, eye style, grey background and the four-views-in-a-row layout exactly, but draw the animal described above.").
