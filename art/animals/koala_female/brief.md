# Brief — `koala_female` (animal turnaround)

Spec: GAME-FAMILY (pairs and babies), ART-ANIMALS, ART-PIPELINE §3. Style: `art/style/style.md`
(comic). References: the approved **male** koala (the existing `koala` art = the male):
`art/animals/koala/sheet_v3.jpg` and its fixed `side.png` (the sheet's side head is turned away —
do not copy that). Status: **in-review**.

## Character

- The koala's partner: same sleepy, cosy look; on all fours like the male (quadruped rig).
- **Difference to the male (Q-074 recommendation, not yet decided):** ~10 % smaller; a
  **larger, lighter cream chest-and-belly patch that reaches up to the chin** (the male's is a
  small off-white bib) and **slightly lighter, silvery grey fur**. No clothing, bows or
  stereotypes.
- Size: on all fours back ≈ 0.4 m, top of the ears ≈ 0.6 m (male 0.45 / 0.65 m;
  game scale see ART-ANIMALS "Game sizes").

## Colours

| Part | Colour |
|---|---|
| Body, head | `#AEB3B8` light silvery grey (male `#9EA3A8`) |
| Chin, chest, belly patch (large) | `#FAF6EC` cream |
| Ear fluff | `#F2F0EB` |
| Nose | `#2A2A2A` |

## Image-generation prompts

One **sheet** at **21:9, 2K**, split with `tools/split_sheet.py`. References: male
`koala/sheet_v3.jpg`. `--extra`: "The attached image is the approved turnaround sheet of the MALE koala of this game. Draw his female partner in exactly the same style, line weight, eye style, grey background and four-views-in-a-row layout, standing on all four paws like him. She must be clearly the same kind of koala, but slightly smaller, with lighter silvery grey fur and a large cream chest-and-belly patch reaching up to her chin. In the side view her whole head is in exact left profile, facing the left edge."

**Negative prompt:**



```text
text, letters, logo, watermark, labels, scenery, grass, ground plane, gradient background, cropped ears, extra views, more than four figures, extra legs, missing legs, extra heads, tree, branch, eucalyptus leaves, holding objects, sitting, standing on hind legs, open mouth, teeth, sharp claws, angry, scary, realistic koala, bear, bow, ribbon, flower, clothing, jewellery, make-up, baby on the back, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

### `sheet.png` (all four views in one image)

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Turnaround sheet of one koala: four views of the SAME koala side by side in one row, evenly spaced, same scale, feet on the same baseline, each view in its own quarter of the image: 1) exact front view — the koala faces the camera straight on, body symmetric, face centred, both front legs side by side; 2) exact left side profile — the koala and its head face the left edge of the image, all four legs visible, the face seen in profile; 3) exact back view — the rump faces the camera, no face visible at all; 4) three-quarter front view — turned 45 degrees to the left of the viewer. A friendly cartoon female koala, standing calmly on all four paws flat on the ground, legs straight, head up, looking forward, mouth closed, neutral happy sleepy smile. Small round chunky body, short sturdy legs with big rounded paws, very big round head, huge round fluffy ears with off-white fluffy rims, a big black oval nose in the middle of the face, big round expressive eyes with a white highlight, no visible tail. Light silvery grey fur (#AEB3B8) with a large cream chest-and-belly patch (#FAF6EC) that reaches from the belly up over the chest to the chin, flat colours. Back height about 0.4 m, top of the ears about 0.6 m, a little smaller than her male partner. Cute, sleepy and gentle.
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Same koala in all four views; clearly the same species/design as the male `koala`.
- [ ] Difference visible from the game camera (chest patch, lighter fur, size) (FAM-007).
- [ ] No clothing/bows; on all fours; side head in true profile; no text.
- [ ] Buildable with the male's rig and mesh, difference = texture + scale.

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-09-26 | sheet_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v3.jpg | discarded (deleted) — near copy of the male; side head turned away (copied from the ref) |
| 2026-09-26 | sheet_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v3.jpg | discarded (deleted) — same problems as v1, no eyelashes |
| 2026-09-26 | sheet_v3.jpg | gemini-3-pro-image (2K, 21:9) | — | edit: Edit the first image, a turnaround sheet of a female koala. Keep the layout, the…, ref: sheet_v1.jpg, side.png | discarded (deleted) — edit of v1: lighter fur, side head fixed, but front view upright |
| 2026-09-26 | sheet_v4.jpg | gemini-3-pro-image (2K, 21:9) | — | edit: Edit the first image, a turnaround sheet of a female koala. Keep the layout, the…, ref: sheet_v1.jpg, side.png | discarded (deleted) — edit of v1: front and side upright |
| 2026-09-26 | sheet_v5.jpg | gemini-3-pro-image (2K, 21:9) | — | edit: Edit the first image, a turnaround sheet of a female koala standing on all four …, ref: sheet_v3.jpg, front.png | **chosen** — edit of v3 (front view back on all fours); split with split_sheet.py --equal (seam split, ears overlap) |
| 2026-09-26 | sheet_v6.jpg | gemini-3-pro-image (2K, 21:9) | — | edit: Edit the first image, a turnaround sheet of a female koala standing on all four …, ref: sheet_v3.jpg, front.png | alternative — same as v5, darker grey |

Family lineup `art/animals/koala_family.png`: composite (no generation) of the ¾ views of
`koala` (male, 100 %), `koala_female` (92 %) and `koala_joey` (46 %), scaled by figure height.
Known issue: the chest patch came out only slightly larger than the male's; the visible
differences are the lighter silvery fur, the eyelashes and the size — decide under Q-074.
