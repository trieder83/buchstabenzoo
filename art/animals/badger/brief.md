# Brief — `badger` (night animal turnaround)

Spec: GAME-NIGHT (night animals, rule 2 "friendly, never scary", NIGHT-006 eyeshine), ART-ANIMALS (look, sizes, animations), ART-PIPELINE §3 (turnaround). Level: **later night level** — food box **Würmer / worms**; hiding place (proposal): digging at the hill. Style: `art/style/style.md` (comic) — style references: `art/environment/style_frame/style_frame.png` (approved) and the approved zebra sheet `art/animals/zebra/sheet_v1.jpg`. Status: **in-review** (generated 2026-09-27 with gemini-3-pro-image — see the generation log for the chosen variant; awaiting user review).

> **Game size (proposal, user decides):** top of head ≈ 0.7 m, length ≈ 1.1 m (low and wide). Comic-scaled like the day animals (ART-ANIMALS "Game sizes").

## Character

- **Dachs / badger** — a night-zoo animal: awake at night, wanders more (GAME-NIGHT §5).
- Sturdy, cosy, hard-working digger; a gentle "grandpa" type.
- **Readability:** the white face with two broad black stripes over the eyes, on a low wide grey body.
- Friendly comic animal like the zebra: big round eyes with white highlight, mouth closed, no teeth, chunky rounded shapes, flat colours with one hard shadow tone. **Never scary** (GAME-NIGHT rule 2).
- **Night readability:** drawn in neutral studio light (the night look comes from the renderer: blue ambient + lantern light + emissive eyes, GAME-NIGHT §10). Colours are chosen warm and light enough to stay visible under blue night light; the eyes have a large clear iris so an eyeshine decal can light up (NIGHT-006).
- Rig: quadruped (low). Animations (proposal, Q-043): `idle`, `walk`, `eat`, `happy` (+ `dig` proposed).
- Views: front, left side (faces LEFT), back, ¾ — check that feet/paws point the same way as the head in the side view.

## Colours (proposal — shared palette later, ADIR-002)

| Part | Colour |
|---|---|
| Body | `#8C8C8A` grey, shadow `#6E6E6C` |
| Legs | `#3A3A3A` dark grey |
| Face | `#F4F2EC` white with two `#262626` stripes |
| Nose | `#2A2A2A` |
| Eyes | `#2A1E18` dark brown, white highlight |
| Outline | dark brown (renderer) |

## Image-generation prompts

One **sheet** at **21:9, 2K** with all four views (consistent angles), 2 variants, then split with `tools/split_sheet.py` (`--equal` if figures overlap their quarter) into `front.png`, `side.png`, `back.png`, `three_quarter.png`. Prompt = the CHARACTER SHEET STYLE block (verbatim) + the views/design paragraph. Command:

`tools/gen_image.py art/animals/badger/brief.md --prompt 1 --out sheet_v1.png sheet_v2.png --aspect 21:9 --size 2K --ref art/environment/style_frame/style_frame.png art/animals/zebra/sheet_v1.jpg --extra "The first reference image shows the game's art style; the second is an approved animal turnaround sheet — match its comic style, line weight, colouring, proportions style, eye style, grey background and the four-views-in-a-row layout exactly, but draw the animal described above."`

**Negative prompt:**

```text
text, letters, logo, watermark, labels, scenery, grass, ground plane, gradient background, night background, dark background, cropped ears, cropped tail, cropped feet, extra views, more than four figures, extra legs, missing legs, extra heads, open mouth, teeth, fangs, angry, scary, claws, realistic badger, honey badger, skunk, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

### `sheet.png` (all four views in one image)

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Turnaround sheet of one badger: four views of the SAME badger side by side in one row, evenly spaced, same scale, feet on the same baseline, each view in its own quarter of the image: 1) exact front view — the badger faces the camera straight on, body symmetric, face centred, both front legs side by side, body hidden behind the chest; 2) exact left side profile — the badger faces the left edge of the image, all four legs visible, no face turned towards the camera; 3) exact back view — the rump and short tail faces the camera, no face visible at all; 4) three-quarter front view — turned 45 degrees to the left of the viewer. A friendly cartoon badger, standing calmly on all four paws flat on the ground, head up, looking forward, mouth closed, neutral happy smile. Low, wide, chunky rounded body, short sturdy dark-grey legs with big rounded paws, a wedge-shaped head with a white face and two broad black stripes running from the nose over the eyes to the small round ears, a small black nose, a short stubby tail. Very big round expressive eyes with a large clear iris and a white highlight (the game lets them shine softly in the lantern light at night). Grey body fur (#8C8C8A) as simple flat shapes. Top of its head about 0.7 m, body about 1.1 m long; next to a 1.2 m tall child it would reach about to her hip. Cosy, gentle and friendly.
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Same badger in all four views (proportions, markings, eyes).
- [ ] Comic style matches `style_frame.png` and the zebra sheet (outlines, flat colours, one shadow tone).
- [ ] Identifying features readable when shrunk to ≈ 60 px from the 55° camera.
- [ ] Friendly, cute, **not scary** for 4–6 year olds (NIGHT-009); no teeth, no fangs.
- [ ] Correct view angles (side faces LEFT, feet agree); nothing cropped; plain grey background; no text.
- [ ] Colours stay readable under blue night light; big eyes suitable for eyeshine (NIGHT-006).
- [ ] Buildable as a low-poly model (≤ 3 000 tris); markings as flat-colour texture.
- [ ] Size relative to the 1.2 m girl OK (see `art/animals/night_lineup.png` once generated).

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-09-27 | sheet_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, sheet_v1.jpg | extra text also asked for 'all four paws, not upright' and 'no zebra parts'; discarded (deleted) — faded zebra-head fragments above every view |
| 2026-09-27 | sheet_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, sheet_v1.jpg | extra text also asked for 'all four paws, not upright' and 'no zebra parts'; downscaled to 2048 px; **chosen**, split → views — low wide grey body on four legs, white face with black stripes, side faces LEFT |
