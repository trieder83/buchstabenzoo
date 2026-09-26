# Brief — `zebra` (animal turnaround)

Spec: ART-ANIMALS (look, animations), ART-PIPELINE §3 (turnaround), GAME-ANIMALS, CONT-MISSIONS
(mission 1). Style: `art/style/style.md` (comic) — style reference image:
`art/environment/style_frame/style_frame.png` (approved), where the zebra's look is already
set. Status: **brief**.

## Character

- Friendly, calm, a little curious — the first animal a child rescues (river, drinking).
- Comic proportions: chunky rounded body, short sturdy legs, big head with big round eyes.
- **Readability from the high game camera (~55°, small on screen):** few broad stripes, a
  clear black muzzle and a striped mane — recognisable as a zebra at ≈ 60 px size.
- Shoulder height ≈ 1.3 m (back about at the head height of the 1.2 m player).
- Animations (ART-ANIMALS, provisional Q-043): `idle`, `walk`, `eat`, `happy` (+ `drink`
  proposed for the river hiding place).

## Colours (proposal — shared palette later, ADIR-002)

| Part | Colour |
|---|---|
| Body base | `#F4F1E8` off-white |
| Stripes, mane stripes, tail tuft, hooves | `#262626` soft black |
| Muzzle | `#2E2E2E` |
| Ear insides | `#E8A0A8` pink |
| Eyes | `#1E1E1E` with white highlight |
| Outline | dark brown (renderer) |

## Image-generation prompts

**3:2 landscape** (the side view needs width) — all views 3:2, same scale, zebra
centred, hooves on the same baseline at ~88 % of the image height. Every prompt = the
CHARACTER SHEET STYLE block (verbatim) + the view paragraph + the design paragraph. Use the
style frame as reference image; for side/back/¾ also attach the approved `front.png` so the
zebra stays identical.

**Negative prompt** (all views):

```text
text, letters, logo, watermark, scenery, grass, ground plane, gradient background, cropped hooves, cropped ears, extra legs, missing legs, extra heads, open mouth with teeth, sharp teeth, angry, scary, realistic zebra, thin stripes, tiny stripes, saddle, harness, rider, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

### `front.png`

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Front view, full body from hooves to ears, the zebra faces the camera directly, both front legs visible side by side. A friendly cartoon zebra, standing calmly with all four hooves flat on the ground, legs straight, head up, looking forward, neutral happy expression. Chunky rounded body, short sturdy legs, large head with big round expressive eyes with a white highlight, small rounded ears with pink insides, black rounded muzzle, short upright mane striped black and white, thin tail with a black tuft. Off-white body (#F4F1E8) with bold broad black stripes (#262626) — about 8 to 10 broad stripes on the body, clear and simple, stripes on the legs as rings, no thin or tiny stripes. Shoulder height about 1.3 m; next to a 1.2 m tall child its back would be about at her head height. Friendly and gentle, not wild.
```

### `side.png`

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Exact left side profile view, full body from nose to tail, the zebra faces the left edge of the image, all four legs visible. A friendly cartoon zebra, standing calmly with all four hooves flat on the ground, legs straight, head up, looking forward, neutral happy expression. Chunky rounded body, short sturdy legs, large head with big round expressive eyes with a white highlight, small rounded ears with pink insides, black rounded muzzle, short upright mane striped black and white, thin tail with a black tuft. Off-white body (#F4F1E8) with bold broad black stripes (#262626) — about 8 to 10 broad stripes on the body, clear and simple, stripes on the legs as rings, no thin or tiny stripes. Shoulder height about 1.3 m; next to a 1.2 m tall child its back would be about at her head height. Friendly and gentle, not wild.
```

### `back.png`

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Back view, full body, the zebra faces away from the camera, tail and hind legs visible, no face visible. A friendly cartoon zebra, standing calmly with all four hooves flat on the ground, legs straight, head up, looking forward, neutral happy expression. Chunky rounded body, short sturdy legs, large head with big round expressive eyes with a white highlight, small rounded ears with pink insides, black rounded muzzle, short upright mane striped black and white, thin tail with a black tuft. Off-white body (#F4F1E8) with bold broad black stripes (#262626) — about 8 to 10 broad stripes on the body, clear and simple, stripes on the legs as rings, no thin or tiny stripes. Shoulder height about 1.3 m; next to a 1.2 m tall child its back would be about at her head height. Friendly and gentle, not wild.
```

### `three_quarter.png`

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Three-quarter front view, full body from nose to tail, the zebra is turned 45 degrees to the left of the viewer. A friendly cartoon zebra, standing calmly with all four hooves flat on the ground, legs straight, head up, looking forward, neutral happy expression. Chunky rounded body, short sturdy legs, large head with big round expressive eyes with a white highlight, small rounded ears with pink insides, black rounded muzzle, short upright mane striped black and white, thin tail with a black tuft. Off-white body (#F4F1E8) with bold broad black stripes (#262626) — about 8 to 10 broad stripes on the body, clear and simple, stripes on the legs as rings, no thin or tiny stripes. Shoulder height about 1.3 m; next to a 1.2 m tall child its back would be about at her head height. Friendly and gentle, not wild.
```

### `sheet.png` (all four views in one image — preferred for consistent angles)

Generate at **21:9**, then split into four equal parts → `front.png`, `side.png`,
`back.png`, `three_quarter.png`.

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Turnaround sheet of one zebra: four views of the SAME zebra side by side in one row, evenly spaced, same scale, hooves on the same baseline, each view in its own quarter of the image: 1) exact front view — the zebra faces the camera straight on, body symmetric, face centred, both front legs side by side, body hidden behind the chest; 2) exact left side profile — the zebra faces the left edge of the image, all four legs visible, no face turned towards the camera; 3) exact back view — the rump and tail face the camera, no face visible at all; 4) three-quarter front view — turned 45 degrees to the left of the viewer. A friendly cartoon zebra, standing calmly with all four hooves flat on the ground, legs straight, head up, looking forward, neutral happy expression. Chunky rounded body, short sturdy legs, large head with big round expressive eyes with a white highlight, small rounded ears with pink insides, black rounded muzzle, short upright mane striped black and white, thin tail with a black tuft. Off-white body (#F4F1E8) with bold broad black stripes (#262626) — about 8 to 10 broad stripes on the body, clear and simple, stripes on the legs as rings, no thin or tiny stripes. Shoulder height about 1.3 m; next to a 1.2 m tall child its back would be about at her head height. Friendly and gentle, not wild.
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Same zebra in all four views (stripe pattern, proportions, eyes, mane).
- [ ] Comic style matches `style_frame.png` (outlines, flat colours, one shadow tone).
- [ ] Broad, simple stripes; readable as a zebra when shrunk to ≈ 60 px.
- [ ] Friendly face, no teeth, not scary.
- [ ] Standing pose, all hooves on the baseline, nothing cropped, plain grey background.
- [ ] Buildable as a low-poly model (≤ 3 000 tris); stripes as flat-colour texture.

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-09-26 | front.jpg | gemini-3-pro-image (1K, 3:2) | — | prompt 1 + negative as 'Avoid', ref: style_frame.png + extra text | discarded — views not at the right angles (front was ¾, back was rear ¾) |
| 2026-09-26 | side.jpg | gemini-3-pro-image (1K, 3:2) | — | prompt 2 + negative as 'Avoid', ref: style_frame.png, front.jpg + extra text | discarded — views not at the right angles (front was ¾, back was rear ¾) |
| 2026-09-26 | back.jpg | gemini-3-pro-image (1K, 3:2) | — | prompt 3 + negative as 'Avoid', ref: style_frame.png, front.jpg + extra text | discarded — views not at the right angles (front was ¾, back was rear ¾) |
| 2026-09-26 | three_quarter.jpg | gemini-3-pro-image (1K, 3:2) | — | prompt 4 + negative as 'Avoid', ref: style_frame.png, front.jpg + extra text | discarded — views not at the right angles (front was ¾, back was rear ¾) |
| 2026-09-26 | sheet_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 5 + negative as 'Avoid', ref: style_frame.png, front.jpg + extra text | to review — split into front/side/back/three_quarter.png |
| 2026-09-26 | sheet_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 5 + negative as 'Avoid', ref: style_frame.png, front.jpg + extra text | alternative (slightly blurry edges) |
