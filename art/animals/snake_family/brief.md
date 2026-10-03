# Brief — `snake_family` (family sheet: `snake`, `snake_female`, `snake_hatchling`)

Spec: GAME-FAMILY (pairs and babies), ART-ANIMALS, GAME-NIGHT (terrarium, night level), ART-PIPELINE §3. Style: `art/style/style.md` (comic). Pattern: `art/animals/hippo_family/brief.md`, `art/animals/zebra_family.png`.
Status: **in-review** (generated 2026-10-03). Manifest ids covered: `snake`, `snake_female`, `snake_hatchling` (no manifest entries yet — they are added with the spec / on approval). No approved adult exists, so the MALE is designed on this sheet too.

## Character

- **Male** (`snake`): green corn snake with yellow-orange saddle blotches (#7BC043 / #F2A93B), cream belly, big eyes, smiling, S-curve; about 1.2 m long.
- **Female** (`snake_female`): same species, ≈ 90 % length, slimmer, lighter cream-yellow belly stripe, softer smaller spots (FAM-007).
- **Baby** (`snake_hatchling`): ≈ 35 % length, chubby, curled, huge head and eyes.
- Limbless: modelled as a spline-like segmented body on a simple bone chain (no legs); animations idle (gentle sway), slither, eat, happy, refuse.

## Image-generation prompts

One **family sheet** at **21:9, 2K**: male | female | baby side by side in the same ¾ view and scale. Reference: `art/animals/zebra_family.png` (layout, line weight and background only).
`--extra`: "The attached image is an approved family sheet of this game: copy its layout, comic style, line weight, eye style and plain grey background exactly, but draw the animals described above."

**Negative prompt:**

```text
text, letters, logo, watermark, scenery, grass, ground plane, gradient background, cropped feet, cropped tail, extra views, more than three figures, fewer than three figures, extra heads, open mouth with teeth, sharp teeth, fangs, forked tongue, angry, scary, menacing, realistic animal, skull, warning symbols, blood, bow, ribbon, flower, jewellery, clothing, make-up, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

### `family_v1.jpg` / `family_v2.jpg`

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Family sheet of one corn snake family: THREE animals side by side in one row, evenly spaced, all on the same baseline, all in the same three-quarter front view turned 45 degrees to the left of the viewer, the same orthographic camera and the same scale for all three, nothing cropped, nothing overlapping. From left to right: 1) the MALE: a friendly cartoon corn snake lying in a relaxed S-curve with its head raised, body about 1.2 m long and as thick as a child's arm, bright fresh green (#7BC043) with a pattern of warm yellow-orange (#F2A93B) saddle-shaped blotches outlined in dark brown along the back, cream belly, big round shiny eyes with a white highlight, a gentle closed smile, no tongue showing; full size (100 %). 2) the FEMALE, the same species and the same design, with ONE clear but gentle difference: the female is slimmer and a little shorter (about 90 % of his length), with a lighter cream-yellow belly stripe clearly visible along her side and softer, smaller, gently rounded pattern spots instead of the bold blotches; same green and orange; slightly longer eyelashes. 3) the BABY of this pair: the HATCHLING: a tiny chubby baby snake, about 35 % of his length but very round and fat, curled up in a little spiral with the head on top, huge head and giant shiny eyes, the same green with the same yellow-orange blotches, cream belly. The baby is clearly the child of these two: same colours and markings, bigger head and bigger eyes in relation to the body. All three are friendly, calm and gentle with a neutral happy expression, closed smiling mouth, no teeth, no fangs, eyes with a white highlight. No clothes, no bows, no ribbons, no jewellery, no make-up, nothing stereotyped.
```

### Suggestion A (`family_vA*.jpg`): banded kingsnake, orange-black-cream (2026-10-03)

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Family sheet of one snake family: THREE animals side by side in one row, evenly spaced, all on the same baseline, all in the same three-quarter front view turned 45 degrees to the left of the viewer, the same orthographic camera and the same scale for all three, nothing cropped, nothing overlapping. No text, no labels, no captions, no letters anywhere. From left to right: 1) the MALE: a friendly cartoon KINGSNAKE (milk-snake style) coiled in a neat chunky pile of two loops with its round head raised on top, body as thick as a child's arm, with BOLD clean BANDS in three colours: warm orange (#F28C28), black and cream-white (#FFF1CC), repeating as wide rings around the whole body (orange wide, thin cream edge, black band, thin cream edge, orange ...), no red, no yellow, no scary warning look. Round smiling head with a big friendly closed smile, a tiny pink tongue tip peeking out, huge round shiny eyes with white highlights, cream chin; full size (100 %). 2) the FEMALE, same species and design with ONE clear but gentle difference: slimmer, about 90 % of his size, and her orange bands are a lighter soft peach-orange (#F8B070) with the black bands thinner, so her whole body looks lighter; slightly longer eyelashes. 3) the BABY of this pair, the HATCHLING: tiny, very round and chubby, curled in a little spiral about 35 % of his size, huge head and giant shiny eyes, same orange-black-cream bands. Clearly the child of these two. All three friendly, calm and gentle, smiling, no teeth, no fangs. No clothes, no bows, no ribbons, no jewellery, no make-up.
```

### Suggestion B (`family_vB*.jpg`): chunky green python, yellow belly, blue-white speckles (2026-10-03)

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Family sheet of one snake family: THREE animals side by side in one row, evenly spaced, all on the same baseline, all in the same three-quarter front view turned 45 degrees to the left of the viewer, the same orthographic camera and the same scale for all three, nothing cropped, nothing overlapping. No text, no labels, no captions, no letters anywhere. From left to right: 1) the MALE: a calm, chunky, plump cartoon BALL PYTHON / tree python lying in a big round coil of two loops with its head resting on top of the coil, very smooth and fat like a sleeping cushion, two-tone: a leaf-green (#5DBB63) back and a bright sunny-yellow (#FFD84A) belly visible on the side of the coil, with a stripe of blue-white (#BFE3FF) round speckle dots running along the middle of the back. Sleepy-friendly big eyes with heavy relaxed lids and a white highlight, a gentle closed smile; full size (100 %). 2) the FEMALE, same species and design with ONE clear but gentle difference: slimmer, about 90 % of his size, with a wider, paler lemon-cream (#FFF3A0) belly stripe and the blue-white speckle dots in a larger, sparser pattern; slightly longer eyelashes. 3) the BABY of this pair, the HATCHLING: tiny, very round and chubby, curled in a small ball about 35 % of his size with the head resting on top, huge head and giant sleepy shiny eyes, same green, yellow and blue-white dots. Clearly the child of these two. All three friendly, calm and gentle, smiling, no teeth, no fangs, no tongue. No clothes, no bows, no ribbons, no jewellery, no make-up.
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Same species and design in all three; friendly, never scary (no teeth/fangs, no drama).
- [ ] Female with the described gentle difference, readable from the game camera (FAM-007).
- [ ] Baby clearly the child; bigger head and eyes.
- [ ] Plain grey background; no text; nothing cropped.
- [ ] Buildable: limbless body = chain of segments with a simple bone chain, pattern as vertex colours / small decal.

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-10-03 | family_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: zebra_family.png | generated, to review |
| 2026-10-03 | family_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: zebra_family.png | generated, to review |
| 2026-10-03 | family_v3.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text; --extra changed to a coiled pose (compact 2-3 loops, head on top, short thick body). Chosen: v4 (v3 has unwanted text labels), ref: zebra_family.png | generated, to review |
| 2026-10-03 | family_v4.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text (coiled pose as v3), ref: zebra_family.png | generated, to review |
| 2026-10-03 | family_vA1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 2 (suggestion A) + negative as 'Avoid' + extra text, ref: zebra_family.png | **suggestion A chosen**: cleanest, same coil/scale in all three, bands readable, male orange vs. peach female, cheeky baby; no tongue visible |
| 2026-10-03 | family_vA2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 2 + negative as 'Avoid' + extra text, ref: zebra_family.png | alternative A: good, but tail flicks and bands are thin/uneven; no tongue |
| 2026-10-03 | family_vA3.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 2 + negative as 'Avoid' + extra text, ref: zebra_family.png | alternative A: has the tiny tongue and thick black rings, but the female is an open loop pose and not coiled like the male (inconsistent) |
| 2026-10-03 | family_vB1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 3 + negative as 'Avoid' + extra text, ref: zebra_family.png | rejected: text labels MALE/FEMALE/HATCHLING under the figures (though good colours) |
| 2026-10-03 | family_vB2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 3 + negative as 'Avoid' + extra text, ref: zebra_family.png | **suggestion B chosen**: clearest blue-white speckle stripe on the male, yellow belly, happy baby; heads are raised, not resting on the coil, female pose has a tail loop |
| 2026-10-03 | family_vB3.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 3 + negative as 'Avoid' + extra text, ref: zebra_family.png | alternative B: consistent family, but speckles on the male are smeared streaks and the baby looks sad/tired |
