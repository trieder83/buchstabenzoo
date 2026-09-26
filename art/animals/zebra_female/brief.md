# Brief — `zebra_female` (animal turnaround)

Spec: GAME-FAMILY (pairs and babies), ART-ANIMALS, ART-PIPELINE §3 (turnaround). Style:
`art/style/style.md` (comic). References: the approved **male** zebra
`art/animals/zebra/sheet_v1.jpg` (the existing `zebra` art = the male, GAME-FAMILY) — the
female must clearly be the same species and the same design. Status: **in-review**.

## Character

- The zebra's partner: same friendly, calm look, same rig (`quadruped`) and animations as `zebra`.
- **Difference to the male (Q-074 recommendation, not yet decided):** the male is ~10 % larger;
  the female is ~10 % smaller and has a **small natural detail** — a **soft forelock that falls
  forward over her forehead** (the male's mane stands straight up) and **slightly longer
  eyelashes**. No clothing, bows, make-up or other stereotypes.
- Size: shoulder height ≈ 1.2 m (male ≈ 1.3 m).
- Colours: identical to `zebra` (`#F4F1E8` body, `#262626` stripes, pink ear insides).

## Image-generation prompts

One **sheet** at **21:9, 2K** with all four views, split with `tools/split_sheet.py`.
Reference image: the male `zebra/sheet_v1.jpg` (only — the style frame blends scenery in).
`--extra`: "The attached image is the approved turnaround sheet of the MALE zebra of this game. Draw his female partner in exactly the same style, colours, line weight, eye style, grey background and four-views-in-a-row layout, with the same view angles. She must be clearly the same kind of zebra, but slightly smaller and with the soft forelock falling forward over her forehead as described."

**Negative prompt:**



```text
text, letters, logo, watermark, scenery, grass, ground plane, gradient background, cropped hooves, cropped ears, extra views, more than four figures, extra legs, missing legs, extra heads, open mouth with teeth, sharp teeth, angry, scary, realistic zebra, thin stripes, tiny stripes, saddle, harness, rider, bow, ribbon, flower, hair clip, jewellery, clothing, make-up, lipstick, pink cheeks, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

### `sheet.png` (all four views in one image)

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Turnaround sheet of one zebra: four views of the SAME zebra side by side in one row, evenly spaced, same scale, hooves on the same baseline, each view in its own quarter of the image: 1) exact front view — the zebra faces the camera straight on, body symmetric, face centred, both front legs side by side, body hidden behind the chest; 2) exact left side profile — the zebra faces the left edge of the image, all four legs visible, no face turned towards the camera; 3) exact back view — the rump and tail face the camera, no face visible at all; 4) three-quarter front view — turned 45 degrees to the left of the viewer. A friendly cartoon female zebra, standing calmly with all four hooves flat on the ground, legs straight, head up, looking forward, neutral happy expression. Chunky rounded body, short sturdy legs, large head with big round expressive eyes with a white highlight and slightly longer dark eyelashes, small rounded ears with pink insides, black rounded muzzle, short mane striped black and white whose front part forms a soft rounded forelock that falls forward over her forehead between the ears, thin tail with a black tuft. Off-white body (#F4F1E8) with bold broad black stripes (#262626) — about 8 to 10 broad stripes on the body, clear and simple, stripes on the legs as rings, no thin or tiny stripes. Shoulder height about 1.2 m, a little smaller and slimmer than her male partner. Friendly and gentle, not wild.
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Same zebra in all four views; clearly the same species/design as the male `zebra`.
- [ ] Children can spot the difference to the male (forelock, size) from the game camera (FAM-007).
- [ ] No clothing, bows or other stereotypes; friendly face, no teeth.
- [ ] Correct view angles; nothing cropped; plain grey background; no text.
- [ ] Buildable with the male's rig and mesh (≤ 3 000 tris), difference = mane/forelock + scale.

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-09-26 | sheet_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | discarded (deleted) — good, forelock curls on top (less distinct from the male's upright mane than v2) |
| 2026-09-26 | sheet_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | **chosen** — forelock falls forward over the forehead, eyelashes; split into front/side/back/three_quarter.png |

Family lineup `art/animals/zebra_family.png`: composite (no generation) of the ¾ views of
`zebra` (male, 100 %), `zebra_female` (91 %) and `zebra_foal` (50 %), scaled by figure height.
