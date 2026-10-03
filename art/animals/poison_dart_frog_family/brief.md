# Brief — `poison_dart_frog_family` (family sheet: `poison_dart_frog`, `poison_dart_frog_female`, `frog_froglet`)

Spec: GAME-FAMILY (pairs and babies), ART-ANIMALS, GAME-NIGHT (terrarium, night level), ART-PIPELINE §3. Style: `art/style/style.md` (comic). Pattern: `art/animals/hippo_family/brief.md`, `art/animals/zebra_family.png`.
Status: **in-review** (generated 2026-10-03; redesigned 2026-10-03: male black-yellow, female orange-black, chosen `family_v6.jpg`; v1/v2 = old blue/yellow design, superseded). Manifest ids covered: `poison_dart_frog`, `poison_dart_frog_female`, `frog_froglet` (no manifest entries yet — they are added with the spec / on approval). No approved adult exists, so the MALE is designed on this sheet too.

## Character

- **Male** (`poison_dart_frog`): **black-and-yellow** "bumblebee" variant (glossy black body, 4–6 bold yellow bands/blotches on back and legs, yellow-tipped feet; user request 2026-10-03, replaces the blue-black v1/v2), round, big eyes; sits on a leaf.
- **Female** (`poison_dart_frog_female`): **orange-and-black** variant (bright orange body with 4–6 bold black patches/bands, black eye patch, yellow-green limbs with a few black spots, pale toe tips), ≈ 90 % (FAM-007).
- **Baby** (`frog_froglet`): ≈ 40 %, froglet with a tail stump, "mixed" pattern: orange body with a few black blotches and yellow legs.
- Patterns are simplified into a few big readable shapes (readable from the 55° game camera at ~12 m, buildable as vertex colours / decal).
- References (colour/pattern only, found by the user, small thumbnails, NOT copied): `ref/ref_black_yellow.jpg` (bottom frog = the male's pattern), `ref/ref_orange_black.jpg` (the female's pattern). The model must draw the cartoon style, never the photos.
- Friendly only: NO skulls, warning signs or poison drama anywhere; it is simply a colourful frog.
- Animations idle (throat pulse), hop, eat, happy, refuse.

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

Family sheet of one poison dart frog (a very colourful, friendly tropical frog) family: THREE animals side by side in one row, evenly spaced, all on the same baseline, all in the same three-quarter front view turned 45 degrees to the left of the viewer, the same orthographic camera and the same scale for all three, nothing cropped, nothing overlapping. From left to right: 1) the MALE: a very colourful cute round frog sitting on a big green leaf, deep glossy-flat cobalt blue (#2C6FD6) body with a few bold round black spots, bright sky-blue limbs, round sticky toe pads, huge shiny black eyes with a white highlight, a wide gentle closed smile; full size (100 %), about 5 cm in real life, drawn chunky and round. 2) the FEMALE, the same species and the same design, with ONE clear but gentle difference: the female is the yellow-orange variant: bright sunny yellow (#F7C826) body with orange (#F28C38) patches and a few small black spots, about 90 % of his size, slightly longer eyelashes; same leaf, same round shape. 3) the BABY of this pair: the BABY (froglet): a tiny froglet about 40 % of his size on a small leaf, still with a short little tadpole tail stump, light green-yellow body with a few blue-and-orange spots, huge head and giant eyes, tiny front feet. The baby is clearly the child of these two: same colours and markings, bigger head and bigger eyes in relation to the body. All three are friendly, calm and gentle with a neutral happy expression, closed smiling mouth, no teeth, no fangs, eyes with a white highlight. No clothes, no bows, no ribbons, no jewellery, no make-up, nothing stereotyped.
```

### `family_v3.jpg` … (black-yellow male, orange-black female)

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Family sheet of one poison dart frog (a very colourful, friendly tropical frog) family: THREE animals side by side in one row, evenly spaced, all on the same baseline, all in the same three-quarter front view turned 45 degrees to the left of the viewer, the same orthographic camera and the same scale for all three, nothing cropped, nothing overlapping, each sitting on its own big green leaf. From left to right: 1) the MALE: a cute round frog, glossy black body with 5 bold bright yellow (#F7D21E) bands and blotches on the back, sides and legs like a bumblebee (a yellow band across the head, two yellow bands across the back, yellow stripes on the thighs), yellow-tipped feet and toes, huge shiny black eyes with a white highlight, a wide gentle closed smile; full size (100 %). 2) the FEMALE, the same species and the same chunky design, the orange-and-black variant: bright orange (#F28A1E) body with 5 bold black patches and bands (a black band behind the head, black blotches along the back, a black patch around each eye), yellow-green arms and legs with a few black spots, pale cream toe tips, slightly longer eyelashes, about 90 % of his size. 3) the BABY of this pair: a froglet about 40 % of his size, still with a short little tadpole tail stump, orange body with 3 or 4 black blotches and yellow legs, huge head and giant eyes. The baby is clearly the child of these two: same colour families, bigger head and bigger eyes in relation to the body. The patterns are simple, big, graphic shapes with clean edges, no tiny speckles. All three are friendly, calm and gentle with a neutral happy expression, closed smiling mouth, no teeth, no fangs. No clothes, no bows, no ribbons, no jewellery, no make-up, nothing stereotyped.
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Same species and design in all three; friendly, never scary (no teeth/fangs, no drama).
- [ ] Female with the described gentle difference, readable from the game camera (FAM-007).
- [ ] Baby clearly the child; bigger head and eyes.
- [ ] Plain grey background; no text; nothing cropped.
- [ ] Buildable: low-poly round body, colours as vertex colours / decal, hop as bone animation.

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-10-03 | family_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: zebra_family.png | generated, to review |
| 2026-10-03 | family_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: zebra_family.png | generated, to review |
| 2026-10-03 | family_v3.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 2 + negative as 'Avoid' + extra text, ref: zebra_family.png, ref_black_yellow.jpg, ref_orange_black.jpg | good; female pattern less photo-like, baby only few spots (alternative) |
| 2026-10-03 | family_v4.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 2 + negative as 'Avoid' + extra text, ref: zebra_family.png, ref_black_yellow.jpg, ref_orange_black.jpg | good pattern, but a green fade at the bottom edge (not plain grey) (alternative) |
| 2026-10-03 | family_v5.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 2 + negative as 'Avoid' + extra text, ref: zebra_family.png, ref_black_yellow.jpg, ref_orange_black.jpg | rejected: unwanted text labels under the figures |
| 2026-10-03 | family_v6.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 2 + negative as 'Avoid' + extra text, ref: zebra_family.png, ref_black_yellow.jpg, ref_orange_black.jpg | **chosen**: strongest bumblebee bands, female with yellow-green spotted legs like the photo, baby with tail stump and black blotches |
