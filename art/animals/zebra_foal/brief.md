# Brief — `zebra_foal` (animal turnaround)

Spec: GAME-FAMILY (baby, name: Fohlen / foal), ART-ANIMALS, ART-PIPELINE §3. Style:
`art/style/style.md` (comic). References: the approved male `art/animals/zebra/sheet_v1.jpg`.
Status: **in-review**.

## Character

- The baby of the zebra pair — appears after the care feedings (GAME-FAMILY §5), stays close
  to the female. Same rig as the parents, scaled (~45 %, GAME-FAMILY §8).
- Baby proportions (comic): **very long thin legs** relative to the small body, a **big head
  with extra big eyes**, a **fluffy, short, messy mane**, short fluffy tail, round belly.
  Wobbly, curious, adorable.
- Size: shoulder height ≈ 0.6 m (≈ 45 % of the male), top of the ears ≈ 1.0 m.
- Colours: as the parents; stripes a little softer brown-black (`#3A3330`) is allowed
  (real foals have brownish stripes) — keep them broad and few.

## Image-generation prompts

One **sheet** at **21:9, 2K** with all four views, split with `tools/split_sheet.py`.
Reference image: the male `zebra/sheet_v1.jpg`. `--extra`: "The attached image is the approved turnaround sheet of the adult male zebra of this game. Draw his baby foal in exactly the same style, colours, line weight, eye style, grey background and four-views-in-a-row layout, with the same view angles. The foal is clearly a baby of this zebra: long thin legs, big head, huge eyes, fluffy mane."

**Negative prompt:**



```text
text, letters, logo, watermark, scenery, grass, ground plane, gradient background, cropped hooves, cropped ears, extra views, more than four figures, adult zebra, parent animal, extra legs, missing legs, extra heads, open mouth with teeth, sharp teeth, angry, scary, realistic zebra, thin stripes, tiny stripes, saddle, bow, ribbon, clothing, lying down, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

### `sheet.png` (all four views in one image)

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Turnaround sheet of one baby zebra foal: four views of the SAME foal side by side in one row, evenly spaced, same scale, hooves on the same baseline, each view in its own quarter of the image: 1) exact front view — the foal faces the camera straight on, body symmetric, face centred, both front legs side by side; 2) exact left side profile — the foal faces the left edge of the image, all four legs visible, no face turned towards the camera; 3) exact back view — the rump and tail face the camera, no face visible at all; 4) three-quarter front view — turned 45 degrees to the left of the viewer. A friendly cute cartoon baby zebra foal standing on all four hooves: small round body with a round belly, very long thin knobbly legs, a big head with huge round expressive eyes with a white highlight, small rounded ears with pink insides, small black rounded muzzle, a short fluffy messy mane striped black and white, a short fluffy tail with a small black tuft. Off-white body (#F4F1E8) with a few bold broad soft black stripes (#3A3330), clear and simple, stripes on the legs as rings, no thin or tiny stripes. Shoulder height about 0.6 m, top of the ears about 1.0 m — a baby about half the size of its parents. Curious, wobbly and adorable, mouth closed, happy.
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Same foal in all four views; reads as the baby of the `zebra` pair (see `art/animals/zebra_family.png`).
- [ ] Baby proportions: long legs, big head/eyes, fluffy mane; clearly smaller than the parents.
- [ ] Friendly face, no teeth; correct view angles; nothing cropped; no text.
- [ ] Buildable with the parents' rig scaled to ~45 % (≤ 2 000 tris).

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-09-26 | sheet_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | discarded (deleted) — side view faced right, mane less fluffy |
| 2026-09-26 | sheet_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v1.jpg | **chosen** — fluffy mane, long legs; split into the four views |
