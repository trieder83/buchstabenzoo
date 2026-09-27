# Brief — `owl` (night animal turnaround)

Spec: GAME-NIGHT (night animals, rule 2 "friendly, never scary", NIGHT-006 eyeshine), ART-ANIMALS (look, sizes, animations), ART-PIPELINE §3 (turnaround). Level: **night_1 (night level 1)** — food box **Käfer / beetles (Q-077: no prey animals in food boxes)**; hiding place (proposal): on the highest branch, round eyes in the moonlight. Style: `art/style/style.md` (comic) — style references: `art/environment/style_frame/style_frame.png` (approved) and the approved zebra sheet `art/animals/zebra/sheet_v1.jpg`. Status: **in-review** (generated 2026-09-27 with gemini-3-pro-image — see the generation log for the chosen variant; awaiting user review).

> **Game size:** perched **≈ 0.8 m** tall — user decision 2026-09-27 (the proposal was ≈ 1.0 m, about the girl's shoulder; model v1 is scaled down uniformly to 0.80 m, proportions unchanged; the sheet prompt below keeps the original 1.0 m wording as generated). Real tawny owl ≈ 0.4 m; comic-scaled up like the day animals (ART-ANIMALS "Game sizes").

## Character

- **Eule / owl** — a night-zoo animal: awake at night, wanders more (GAME-NIGHT §5).
- Wise, calm, a little sleepy-grumpy in a sweet way; blinks slowly; can turn its head far (nice `look` animation).
- Comic proportions: an egg-shaped chunky body, a big round head merged with the body, two small ear tufts, short feathered legs with big friendly feet.
- **Readability:** round shape with **two giant round eyes in a pale heart-shaped face disc** — the eyes are the silhouette at night (NIGHT-006 eyeshine).
- **Pose:** perched/standing upright with wings folded at the sides (the perch is the branch/ground; no branch in the sheet). Flight pose later if a flying follow is wanted (open question for the user).
- Friendly comic animal like the zebra: big round eyes with white highlight, mouth closed, no teeth, chunky rounded shapes, flat colours with one hard shadow tone. **Never scary** (GAME-NIGHT rule 2).
- **Night readability:** drawn in neutral studio light (the night look comes from the renderer: blue ambient + lantern light + emissive eyes, GAME-NIGHT §10). Colours are chosen warm and light enough to stay visible under blue night light; the eyes have a large clear iris so an eyeshine decal can light up (NIGHT-006).
- Rig: own bird rig (Q-043): body, head turn, wings, feet. Animations (proposal, Q-043): `idle` (perched), `walk` (hop/waddle) or `fly`, `eat`, `happy`, `look` (head turn) — proposal for Q-043.
- Views: front, left side (faces LEFT), back, ¾ — check that feet/paws point the same way as the head in the side view.

## Colours (proposal — shared palette later, ADIR-002)

| Part | Colour |
|---|---|
| Body feathers | `#9A6A43` warm brown, spots `#6E4A2E` |
| Face disc, belly | `#EAD7B4` cream |
| Beak, feet | `#E7A93A` warm yellow-orange |
| Eyes | `#F5B82E` golden iris, `#1E1E1E` big pupil, white highlight |
| Ear tufts, wing tips | `#6E4A2E` dark brown |
| Outline | dark brown (renderer) |

## Image-generation prompts

One **sheet** at **21:9, 2K** with all four views (consistent angles), 2 variants, then split with `tools/split_sheet.py` (`--equal` if figures overlap their quarter) into `front.png`, `side.png`, `back.png`, `three_quarter.png`. Prompt = the CHARACTER SHEET STYLE block (verbatim) + the views/design paragraph. Command:

`tools/gen_image.py art/animals/owl/brief.md --prompt 1 --out sheet_v1.png sheet_v2.png --aspect 21:9 --size 2K --ref art/environment/style_frame/style_frame.png art/animals/zebra/sheet_v1.jpg --extra "The first reference image shows the game's art style; the second is an approved animal turnaround sheet — match its comic style, line weight, colouring, proportions style, eye style, grey background and the four-views-in-a-row layout exactly, but draw the animal described above."`

**Negative prompt:**

```text
text, letters, logo, watermark, labels, scenery, grass, ground plane, gradient background, night background, dark background, cropped ears, cropped tail, cropped feet, extra views, more than four figures, extra legs, missing legs, extra heads, open mouth, teeth, fangs, angry, scary, angry eyebrows, frown, sharp talons, claws gripping prey, mouse, prey, open beak, spooky, halloween, realistic owl, detailed feathers, branch, perch, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

### `sheet.png` (all four views in one image)

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Turnaround sheet of one owl: four views of the SAME owl side by side in one row, evenly spaced, same scale, feet on the same baseline, each view in its own quarter of the image: 1) exact front view — the owl faces the camera straight on, body symmetric, face centred; 2) exact left side profile — the owl faces the left edge of the image, beak pointing left, no face turned towards the camera; 3) exact back view — the back faces the camera, no face visible at all; 4) three-quarter front view — turned 45 degrees to the left of the viewer. A friendly cartoon owl, standing upright on its two feet with both wings folded at its sides, head up, looking forward, beak closed, calm wise gentle smile. Chunky egg-shaped body with a big round head merged into it, two small feathery ear tufts, a pale cream heart-shaped face disc (#EAD7B4), a small hooked warm yellow-orange beak, short feathered legs with big friendly rounded yellow-orange feet (no sharp talons). Two giant round golden eyes (#F5B82E) with big dark pupils and white highlights (the game lets them shine softly in the lantern light at night). Warm brown feathers (#9A6A43) drawn as a few big simple shapes with a few darker brown spots, cream belly, darker brown wing tips. Standing it is about 1.0 m tall; next to a 1.2 m tall child it would reach about to her shoulder. Cute, wise and friendly.
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Same owl in all four views (proportions, markings, eyes).
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
| 2026-09-27 | sheet_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, sheet_v1.jpg | downscaled to 2048 px; **chosen**, split → front/side/back/three_quarter — egg-shaped body, ear tufts, giant golden eyes in a pale face disc, side faces LEFT with feet pointing left |
| 2026-09-27 | sheet_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, sheet_v1.jpg | downscaled to 2048 px; alternative — side view faces RIGHT |
