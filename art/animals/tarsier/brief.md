# Brief — `tarsier` (night animal turnaround)

Spec: GAME-NIGHT (night animals, rule 2 "friendly, never scary", NIGHT-006 eyeshine), ART-ANIMALS (look, sizes, animations), ART-PIPELINE §3 (turnaround). Level: **later night level** — food box **Grillen / crickets**; hiding place (proposal): with giant eyes in the bamboo. Style: `art/style/style.md` (comic) — style references: `art/environment/style_frame/style_frame.png` (approved) and the approved zebra sheet `art/animals/zebra/sheet_v1.jpg`. Status: **in-review** (generated 2026-09-27 with gemini-3-pro-image — see the generation log for the chosen variant; awaiting user review).

> **Game size (proposal, user decides):** sitting upright ≈ 0.7 m (real ≈ 0.15 m — the most scaled-up animal). Comic-scaled like the day animals (ART-ANIMALS "Game sizes").

## Character

- **Koboldmaki / tarsier** — a night-zoo animal: awake at night, wanders more (GAME-NIGHT §5).
- Tiny, jumpy, amazed at everything; hops like a little kangaroo, turns its head far round.
- **Readability:** the **two giant eyes** (each as big as its brain, comic: each eye about a quarter of the head height) and big round ears on a small fluffy body with long thin legs and a long thin tail.
- Eyes must stay cute: shiny, with big highlights and a small smile — never staring or creepy (GAME-NIGHT rule 2; check with the user).
- Friendly comic animal like the zebra: big round eyes with white highlight, mouth closed, no teeth, chunky rounded shapes, flat colours with one hard shadow tone. **Never scary** (GAME-NIGHT rule 2).
- **Night readability:** drawn in neutral studio light (the night look comes from the renderer: blue ambient + lantern light + emissive eyes, GAME-NIGHT §10). Colours are chosen warm and light enough to stay visible under blue night light; the eyes have a large clear iris so an eyeshine decal can light up (NIGHT-006).
- Rig: biped (sits upright, hops on its long legs). Animations (proposal, Q-043): `idle`, `walk` → `hop`, `eat`, `happy` (+ `look` head turn).
- Views: front, left side (faces LEFT), back, ¾ — check that feet/paws point the same way as the head in the side view.

## Colours (proposal — shared palette later, ADIR-002)

| Part | Colour |
|---|---|
| Fur | `#B08A62` warm brown-grey |
| Face, belly | `#E3CFB0` light beige |
| Ears, hands, feet | `#D8A88E` soft pink-beige |
| Tail tuft | `#8C6A48` |
| Eyes | `#C98A2E` amber iris, big pupil, big white highlights |
| Outline | dark brown (renderer) |

## Image-generation prompts

One **sheet** at **21:9, 2K** with all four views (consistent angles), 2 variants, then split with `tools/split_sheet.py` (`--equal` if figures overlap their quarter) into `front.png`, `side.png`, `back.png`, `three_quarter.png`. Prompt = the CHARACTER SHEET STYLE block (verbatim) + the views/design paragraph. Command:

`tools/gen_image.py art/animals/tarsier/brief.md --prompt 1 --out sheet_v1.png sheet_v2.png --aspect 21:9 --size 2K --ref art/environment/style_frame/style_frame.png art/animals/zebra/sheet_v1.jpg --extra "The first reference image shows the game's art style; the second is an approved animal turnaround sheet — match its comic style, line weight, colouring, proportions style, eye style, grey background and the four-views-in-a-row layout exactly, but draw the animal described above."`

**Negative prompt:**

```text
text, letters, logo, watermark, labels, scenery, grass, ground plane, gradient background, night background, dark background, cropped ears, cropped tail, cropped feet, extra views, more than four figures, extra legs, missing legs, extra heads, open mouth, teeth, fangs, angry, scary, creepy, staring, alien, gremlin, horror, bloodshot eyes, realistic tarsier, bony fingers, halloween, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

### `sheet.png` (all four views in one image)

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Turnaround sheet of one tarsier: four views of the SAME tarsier side by side in one row, evenly spaced, same scale, feet on the same baseline, each view in its own quarter of the image: 1) exact front view — the tarsier faces the camera straight on, body symmetric, face centred; 2) exact left side profile — the tarsier faces the left edge of the image, nose pointing left, no face turned towards the camera; 3) exact back view — the back faces the camera, no face visible at all; 4) three-quarter front view — turned 45 degrees to the left of the viewer. A friendly cartoon tarsier, standing upright on its two long hind legs with long feet flat on the ground like a little kangaroo, small hands held in front of its chest, head up, looking forward, mouth closed, sweet amazed little smile. A small round fluffy body, a big round head with two giant round shiny amber eyes (#C98A2E) with big pupils and big white highlights, big round thin ears, a tiny nose, long thin legs, soft round fingertips, a long thin tail with a small tuft at the end resting on the ground behind it. Warm brown-grey fur (#B08A62), light beige face and belly (#E3CFB0). (The game lets the eyes shine softly in the lantern light at night.) Sitting upright it is about 0.7 m tall; next to a 1.2 m tall child it would reach about to her waist. Very cute, amazed and friendly — the eyes are adorable, never creepy.
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Same tarsier in all four views (proportions, markings, eyes).
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
| 2026-09-27 | sheet_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, sheet_v1.jpg | extra text also asked for 'no zebra parts'; downscaled to 2048 px; **chosen**, split → views — upright on long legs, giant amber eyes, big round ears, tufted tail, side faces LEFT |
| 2026-09-27 | sheet_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, sheet_v1.jpg | extra text also asked for 'no zebra parts'; downscaled to 2048 px; alternative — thinner, tail in the back view bends oddly |
