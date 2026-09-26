# Brief — `goldfish` (animal turnaround)

Spec: ART-ANIMALS (look, animations), ART-PIPELINE §3 (turnaround), GAME-ANIMALS, CONT-MISSIONS
(food box: **Fischfutter**; hiding place: fountain (`loc_fountain`), swimming in circles among the coins). Style: `art/style/style.md` (comic) —
style references: `art/environment/style_frame/style_frame.png` (approved) and the approved zebra
sheet `art/animals/zebra/sheet_v1.jpg` (line weight, eyes, colouring, layout). Status: **in-review**.

> **Game size:** 0.6 m long (user decision 2026-09-26) — comic-scaled up for readability; see ART-ANIMALS "Game sizes". Concept approved 2026-09-26.

## Character

- Cheerful, bubbly, a little restless — always swimming.
- Comic proportions: round egg-shaped body, very big eyes, big flowing double tail fin, small fins; no legs.
- **Readability (~60 px, 55° camera — mostly seen from above in the fountain basin):** bright orange body with a big white-tipped tail fin — colour and tail are the identifying features. Therefore the views are adapted: **side, top, front, ¾** (the top view is the in-game view), swimming pose, no ground.
- Friendly comic animal like the zebra: big round eyes with white highlight, mouth closed, no
  teeth shown, chunky rounded shapes, flat colours with one hard shadow tone.
- Size: Length ≈ 0.30 m incl. tail, height ≈ 0.14 m (real ~0.1–0.2 m; comic: bigger so it reads in the fountain). Transported in a bucket (Q-036).
- Animations (ART-ANIMALS, provisional Q-043): `swim`, `eat` (+ locomotion while following in the bucket, Q-043); needs its own small rig (not `quadruped`)..
- Views: Goldfish views (adapted, proposal — user decides): **side, top, front, ¾**, swimming pose, no ground; split names `side top front three_quarter`.

## Colours (proposal — shared palette later, ADIR-002)

| Part | Colour |
|---|---|
| Body | `#F28A1E` bright orange |
| Belly, fin tips, tail tips | `#FFF1DC` cream white |
| Fins, tail | `#F5A64A` light orange |
| Mouth | `#E06A55` coral |
| Eyes | `#1E1E1E` large, with white highlight and white eye ring |
| Outline | dark brown (renderer) |

## Image-generation prompts

One **sheet** at **21:9, 2K** with all four views (consistent angles), then split with
`tools/split_sheet.py` into single views. Prompt = the CHARACTER SHEET STYLE block (verbatim) +
the views paragraph + the design paragraph. Reference images: `style_frame.png` and the
approved zebra `sheet_v1.jpg` (with the `--extra` text in the generation log).

**Negative prompt:**

```text
text, letters, logo, watermark, labels, scenery, grass, ground plane, gradient background, cropped ears, cropped tail, extra views, more than four figures, extra legs, missing legs, extra heads, water, bubbles, fishbowl, bucket, coins, ground plane, shadow on ground, legs, arms, teeth, sharp teeth, angry, scary, realistic fish, detailed scales, shark, piranha, koi pattern, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

### `sheet.png` (all four views in one image)

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Turnaround sheet of one goldfish: four views of the SAME goldfish side by side in one row, evenly spaced, same scale, all at the same height, each view in its own quarter of the image, the fish floating as if swimming: 1) exact left side profile — the fish faces the left edge of the image, full side silhouette with all fins; 2) exact top view — seen straight from above, head pointing up to the top of the image, both pectoral fins and the tail fin spread; 3) exact front view — the fish faces the camera straight on, both eyes visible, round mouth centred, body symmetric; 4) three-quarter front view — turned 45 degrees to the left of the viewer. A friendly cartoon goldfish in a calm swimming pose, floating with no ground, mouth closed in a small round happy smile. Round chunky egg-shaped body, very big round expressive eyes with a white highlight, a big flowing double tail fin, a small dorsal fin on top, two small pectoral fins and small belly fins, no legs, no arms. Bright orange body (#F28A1E) with a cream-white belly and cream-white tips on the tail and fins (#FFF1DC), a few big rounded scale shapes drawn as simple lines at most, flat colours. About 0.3 m long including the tail. Cheerful, cute and bubbly.
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Same goldfish in all four views (proportions, markings, eyes).
- [ ] Comic style matches `style_frame.png` and the zebra sheet (outlines, flat colours, one shadow tone).
- [ ] Identifying features readable when shrunk to ≈ 60 px.
- [ ] Friendly face, no teeth, not scary.
- [ ] Correct view angles; nothing cropped; plain grey background; no text.
- [ ] Buildable as a low-poly model (≤ 3 000 tris); markings as flat-colour texture.
- [ ] Size relative to the 1.2 m girl OK (see `art/animals/lineup.png`).

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-09-26 | sheet_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, zebra/sheet_v1.jpg | discarded (deleted) — zebra legs from the reference sheet leaked under every fish |
| 2026-09-26 | sheet_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, zebra/sheet_v1.jpg | **chosen** — split into side/top/front/three_quarter.png; no legs, correct views |
| 2026-09-26 | sheet_v3.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png | discarded (deleted) — returned the style frame scene instead of a sheet (style frame only as ref) |
| 2026-09-26 | sheet_v4.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: zebra/sheet_v1.jpg | discarded (deleted) — zebra legs leaked again (zebra sheet only as ref). No alternative sheet kept |

`--extra` for all first sheets: the two-reference-images text from the task ("…match its comic style, line weight, colouring, proportions style, eye style, grey background and the four-views-in-a-row layout exactly, but draw the animal described above.").
