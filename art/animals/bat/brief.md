# Brief — `bat` (night animal turnaround)

Spec: GAME-NIGHT (night animals, rule 2 "friendly, never scary", NIGHT-006 eyeshine), ART-ANIMALS (look, sizes, animations), ART-PIPELINE §3 (turnaround). Level: **night_1 (night level 1)** — food box **Obst / fruit (fruit bat)**; hiding place (proposal): hanging under the bridge / in the old tree. Style: `art/style/style.md` (comic) — style references: `art/environment/style_frame/style_frame.png` (approved) and the approved zebra sheet `art/animals/zebra/sheet_v1.jpg`. Status: **brief** (not generated — Gemini monthly spending cap, HTTP 429, 2026-09-26).

> **Game size (proposal, user decides):** perched upright ≈ 0.8 m head to feet incl. ears; wingspan in flight ≈ 1.8 m (real flying fox ≈ 0.3 m body / 1.5 m wings; scaled up for readability). Comic-scaled like the day animals (ART-ANIMALS "Game sizes").

## Character

- **Fledermaus / bat (fruit bat)** — a night-zoo animal: awake at night, wanders more (GAME-NIGHT §5).
- A **fruit bat** (flying fox) — dog-like friendly face, NOT a vampire bat: rounded snout, big dark eyes, soft fluffy collar.
- Gentle, sleepy by day, playful at night; hangs upside down in its hiding place, flies after the player when following (flight pose later).
- **Pose decision (proposal):** the turnaround shows it **perched upright on its feet with the wings folded around the body like a cloak** — the most readable pose for the modeller and for following on the ground. The hanging-upside-down hiding pose = this pose rotated 180° (feet up), no extra sheet. A **flight pose** sheet (prompt 2) is optional and for later.
- **Readability:** warm orange-brown fluffy collar/head contrasting with dark-brown wings — at night the light collar and the big eyes read first.
- Friendly comic animal like the zebra: big round eyes with white highlight, mouth closed, no teeth, chunky rounded shapes, flat colours with one hard shadow tone. **Never scary** (GAME-NIGHT rule 2).
- **Night readability:** drawn in neutral studio light (the night look comes from the renderer: blue ambient + lantern light + emissive eyes, GAME-NIGHT §10). Colours are chosen warm and light enough to stay visible under blue night light; the eyes have a large clear iris so an eyeshine decal can light up (NIGHT-006).
- Rig: own small rig (wings; Q-043) — upright/perched body, wing bones for flight. Animations (proposal, Q-043): `idle` (perched), `walk` → replaced by `fly` (flap loop, follows the player in the air), `eat`, `happy`, `hang` (hiding idle, upside down) — proposal for Q-043.
- Views: front, left side (faces LEFT), back, ¾ — check that feet/paws point the same way as the head in the side view.

## Colours (proposal — shared palette later, ADIR-002)

| Part | Colour |
|---|---|
| Head, collar (fluffy mane) | `#D9873E` warm orange-brown |
| Body fur | `#8A5A3A` brown |
| Wing membranes | `#4A3328` dark brown, shadow `#35241C` |
| Ear insides, nose | `#C98B7A` dusty pink / `#2A2A2A` |
| Eyes | `#241A14` very dark brown, big white highlight |
| Outline | dark brown (renderer) |

## Image-generation prompts

One **sheet** at **21:9, 2K** with all four views (consistent angles), 2 variants, then split with `tools/split_sheet.py` (`--equal` if figures overlap their quarter) into `front.png`, `side.png`, `back.png`, `three_quarter.png`. Prompt = the CHARACTER SHEET STYLE block (verbatim) + the views/design paragraph. Command:

`tools/gen_image.py art/animals/bat/brief.md --prompt 1 --out sheet_v1.png sheet_v2.png --aspect 21:9 --size 2K --ref art/environment/style_frame/style_frame.png art/animals/zebra/sheet_v1.jpg --extra "The first reference image shows the game's art style; the second is an approved animal turnaround sheet — match its comic style, line weight, colouring, proportions style, eye style, grey background and the four-views-in-a-row layout exactly, but draw the animal described above."`

**Negative prompt:**

```text
text, letters, logo, watermark, labels, scenery, grass, ground plane, gradient background, night background, dark background, cropped ears, cropped tail, cropped feet, extra views, more than four figures, extra legs, missing legs, extra heads, open mouth, teeth, fangs, angry, scary, vampire, sharp teeth, red eyes, glowing red eyes, halloween, spooky, evil, blood, realistic bat, wrinkled skin, upside down, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

### `sheet.png` (all four views in one image)

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Turnaround sheet of one bat: four views of the SAME bat side by side in one row, evenly spaced, same scale, feet on the same baseline, each view in its own quarter of the image: 1) exact front view — the bat faces the camera straight on, body symmetric, face centred; 2) exact left side profile — the bat faces the left edge of the image, nose pointing left, no face turned towards the camera; 3) exact back view — the back faces the camera, no face visible at all; 4) three-quarter front view — turned 45 degrees to the left of the viewer. A friendly cartoon fruit bat (flying fox), perched upright on its two little feet with both wings neatly folded around its body like a cloak, head up, looking forward, mouth closed, sweet gentle smile. Round chunky body, a big round head with a short rounded dog-like snout and a small dark nose, two big rounded ears with pink insides, a thick fluffy warm orange-brown collar (#D9873E) around the neck and head, brown body fur (#8A5A3A), smooth dark-brown wing membranes (#4A3328) with a few simple finger lines and a small thumb claw at the top of each folded wing, no fangs. Very big round expressive eyes with a large clear iris and a white highlight (the game lets them shine softly in the lantern light at night). Perched upright it is about 0.8 m tall; next to a 1.2 m tall child its ears would reach about to her chest. Cute, sleepy-sweet and friendly, not spooky at all.
```

### `sheet_flight.png` (optional, later — flight pose)

Only if the user wants the bat to fly while following. 21:9, 2K, reference = the chosen `sheet_vN.jpg`.

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Turnaround sheet of one bat in flight: four views of the SAME flying bat side by side in one row, evenly spaced, same scale, each view in its own quarter of the image: 1) exact front view — facing the camera, wings spread symmetrically; 2) exact left side profile — flying towards the left edge of the image; 3) exact back view — no face visible; 4) exact top view from above, wings fully spread. A friendly cartoon fruit bat (flying fox) — the SAME bat as in the reference sheet — in flight: wings spread wide and slightly raised, head up, looking forward, mouth closed, sweet gentle smile, feet tucked under the body. Round chunky body, big round head with a short rounded dog-like snout, two big rounded ears with pink insides, thick fluffy warm orange-brown collar (#D9873E), brown body fur (#8A5A3A), smooth dark-brown wing membranes (#4A3328) with a few simple finger lines, simple rounded wing shapes. Very big round expressive eyes with a large clear iris and a white highlight (the game lets them shine softly in the lantern light at night). Wingspan about 1.8 m. Cute and friendly, not spooky at all.
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Same bat in all four views (proportions, markings, eyes).
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
