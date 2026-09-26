# Brief — `koala_joey` (animal turnaround)

Spec: GAME-FAMILY (baby, name: Koalababy (Joey) / joey), ART-ANIMALS, ART-PIPELINE §3. Style:
`art/style/style.md` (comic). References: the approved male `art/animals/koala/sheet_v3.jpg`.
Status: **in-review**.

## Character

- The baby of the koala pair; stays close to the female (GAME-FAMILY §5). Same rig as the
  parents, scaled (~45 %). On all fours (it could later ride on the mother's back — not in
  scope, would be a new question).
- Baby proportions: **even bigger head** relative to the body, **huge fluffy ears**, big
  shiny eyes, tiny round body, stubby legs, a small nose. Extra fluffy.
- Size: back ≈ 0.2 m, top of the ears ≈ 0.3 m (≈ 45 % of the male).
- Colours: as the male (`#9EA3A8` grey, `#F2F0EB` off-white bib, `#2A2A2A` nose), slightly lighter.

## Image-generation prompts

One **sheet** at **21:9, 2K**, split with `tools/split_sheet.py`. Reference: male
`koala/sheet_v3.jpg`. `--extra`: "The attached image is the approved turnaround sheet of the adult male koala of this game. Draw his baby joey in exactly the same style, colours, line weight, eye style, grey background and four-views-in-a-row layout, standing on all four paws. The joey is clearly a baby of this koala: tiny body, even bigger head and ears, huge eyes, extra fluffy. In the side view its whole head is in exact left profile, facing the left edge."

**Negative prompt:**



```text
text, letters, logo, watermark, labels, scenery, grass, ground plane, gradient background, cropped ears, extra views, more than four figures, adult koala, parent animal, extra legs, missing legs, extra heads, tree, branch, eucalyptus leaves, holding objects, sitting, standing on hind legs, open mouth, teeth, sharp claws, angry, scary, realistic koala, bear, bow, ribbon, clothing, pacifier, diaper, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

### `sheet.png` (all four views in one image)

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Turnaround sheet of one baby koala joey: four views of the SAME joey side by side in one row, evenly spaced, same scale, feet on the same baseline, each view in its own quarter of the image: 1) exact front view — the joey faces the camera straight on, body symmetric, face centred; 2) exact left side profile — the joey and its head face the left edge of the image, all four legs visible, the face seen in profile; 3) exact back view — the rump faces the camera, no face visible at all; 4) three-quarter front view — turned 45 degrees to the left of the viewer. A friendly cute cartoon baby koala standing on all four stubby paws: a tiny round fluffy body, an extra big round head, huge round fluffy ears with off-white fluffy rims, huge round shiny expressive eyes with a white highlight, a small black oval nose, mouth closed with a tiny happy smile, no visible tail. Light soft grey fur (#A6ABB0) with an off-white chin and chest (#F2F0EB), flat colours, a few fluffy tufts on the head. Back height about 0.2 m, top of the ears about 0.3 m — a baby about half the size of its parents. Adorable, curious and gentle.
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Same joey in all four views; reads as the baby of the `koala` pair (see `art/animals/koala_family.png`).
- [ ] Baby proportions: big head/ears/eyes, tiny body; on all fours; no clothing/props.
- [ ] Correct view angles; nothing cropped; plain grey background; no text.
- [ ] Buildable with the parents' rig scaled to ~45 %.

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-09-26 | sheet_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v3.jpg | alternative — on all fours, side head in ¾ instead of profile, less baby-like |
| 2026-09-26 | sheet_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v3.jpg | discarded (deleted) — upright teddy pose |
| 2026-09-26 | sheet_v3.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_v3.jpg | **chosen** — --extra plus 'IMPORTANT: on ALL FOUR PAWS in every view…'; split with --equal (seam split) |
