# Brief — `slow_loris` (night animal turnaround)

Spec: GAME-NIGHT (night animals, rule 2 "friendly, never scary", NIGHT-006 eyeshine), ART-ANIMALS (look, sizes, animations), ART-PIPELINE §3 (turnaround). Level: **later night level** — food box **Nektar / nectar**; hiding place (proposal): slowly climbing in the flowering bush. Style: `art/style/style.md` (comic) — style references: `art/environment/style_frame/style_frame.png` (approved) and the approved zebra sheet `art/animals/zebra/sheet_v1.jpg`. Status: **brief** (not generated — Gemini monthly spending cap, HTTP 429, 2026-09-26).

> **Game size (proposal, user decides):** top of head ≈ 0.7 m on all fours (real ≈ 0.3 m). Comic-scaled like the day animals (ART-ANIMALS "Game sizes").

## Character

- **Plumplori / slow loris** — a night-zoo animal: awake at night, wanders more (GAME-NIGHT §5).
- Very slow, dreamy, sweet; moves in slow motion (funny for kids: the slowest follower).
- **Readability:** round fluffy body with **huge round eyes in dark eye patches** and a white stripe down the nose.
- Pose proposal: standing on all four with hand-like paws (it climbs on branches); a sitting-upright variant is possible (decision for the user).
- Friendly comic animal like the zebra: big round eyes with white highlight, mouth closed, no teeth, chunky rounded shapes, flat colours with one hard shadow tone. **Never scary** (GAME-NIGHT rule 2).
- **Night readability:** drawn in neutral studio light (the night look comes from the renderer: blue ambient + lantern light + emissive eyes, GAME-NIGHT §10). Colours are chosen warm and light enough to stay visible under blue night light; the eyes have a large clear iris so an eyeshine decal can light up (NIGHT-006).
- Rig: quadruped (slow climber) — or biped sitting; open for the user. Animations (proposal, Q-043): `idle`, `walk` (very slow), `eat` (licks nectar), `happy`.
- Views: front, left side (faces LEFT), back, ¾ — check that feet/paws point the same way as the head in the side view.

## Colours (proposal — shared palette later, ADIR-002)

| Part | Colour |
|---|---|
| Fur | `#C9A27A` warm tan, shadow `#A7815C` |
| Eye patches, back stripe | `#6E4A2E` brown |
| Nose stripe, muzzle | `#F4EBDD` cream |
| Hands, nose | `#D6A08A` soft pink |
| Eyes | `#E3A53A` amber iris, big pupil, white highlight |
| Outline | dark brown (renderer) |

## Image-generation prompts

One **sheet** at **21:9, 2K** with all four views (consistent angles), 2 variants, then split with `tools/split_sheet.py` (`--equal` if figures overlap their quarter) into `front.png`, `side.png`, `back.png`, `three_quarter.png`. Prompt = the CHARACTER SHEET STYLE block (verbatim) + the views/design paragraph. Command:

`tools/gen_image.py art/animals/slow_loris/brief.md --prompt 1 --out sheet_v1.png sheet_v2.png --aspect 21:9 --size 2K --ref art/environment/style_frame/style_frame.png art/animals/zebra/sheet_v1.jpg --extra "The first reference image shows the game's art style; the second is an approved animal turnaround sheet — match its comic style, line weight, colouring, proportions style, eye style, grey background and the four-views-in-a-row layout exactly, but draw the animal described above."`

**Negative prompt:**

```text
text, letters, logo, watermark, labels, scenery, grass, ground plane, gradient background, night background, dark background, cropped ears, cropped tail, cropped feet, extra views, more than four figures, extra legs, missing legs, extra heads, open mouth, teeth, fangs, angry, scary, realistic loris, sad, scared, tiny eyes, monkey, lemur tail, long tail, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

### `sheet.png` (all four views in one image)

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Turnaround sheet of one slow loris: four views of the SAME slow loris side by side in one row, evenly spaced, same scale, feet on the same baseline, each view in its own quarter of the image: 1) exact front view — the slow loris faces the camera straight on, body symmetric, face centred, both front legs side by side, body hidden behind the chest; 2) exact left side profile — the slow loris faces the left edge of the image, all four legs visible, no face turned towards the camera; 3) exact back view — the rump faces the camera, no face visible at all; 4) three-quarter front view — turned 45 degrees to the left of the viewer. A friendly cartoon slow loris, standing calmly on all four hand-like paws flat on the ground, head up, looking forward, mouth closed, sweet dreamy smile. Round chunky fluffy body, short sturdy arms and legs with soft pink hand-like paws, no visible tail, a round head with small round ears, HUGE round amber eyes (#E3A53A) with big pupils and white highlights, each in a round brown eye patch, a white stripe running down the nose between the eyes, a dark-brown stripe along the middle of the back. Warm tan fur (#C9A27A) as simple fluffy shapes. (The game lets the eyes shine softly in the lantern light at night.) About 0.7 m tall on all fours; next to a 1.2 m tall child it would reach about to her waist. Sweet, dreamy and friendly.
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Same slow loris in all four views (proportions, markings, eyes).
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
