# Brief — `chameleon_family` (family sheet: `chameleon`, `chameleon_female`, `chameleon_baby`)

Spec: GAME-FAMILY (pairs and babies), ART-ANIMALS, GAME-NIGHT (terrarium, night level), ART-PIPELINE §3. Style: `art/style/style.md` (comic). Pattern: `art/animals/hippo_family/brief.md`, `art/animals/zebra_family.png`.
Status: **in-review** (generated 2026-10-03). Manifest ids covered: `chameleon`, `chameleon_female`, `chameleon_baby` (no manifest entries yet — they are added with the spec / on approval). No approved adult exists, so the MALE is designed on this sheet too.

## Character

- **Male** (`chameleon`): leaf-green with turquoise/yellow/orange patches, curled tail, big independent eyes, small crest + two horn bumps; about 35 cm.
- **Female** (`chameleon_female`): ≈ 90 %, softer pastel colours, smooth head without crest (FAM-007).
- **Baby** (`chameleon_baby`): ≈ 40 %, tiny, huge eyes.
- Animations idle (eyes turning), walk (slow rocking steps), eat, happy, refuse; colour patches as flat vertex colours / decal.

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

Family sheet of one veiled chameleon family: THREE animals side by side in one row, evenly spaced, all on the same baseline, all in the same three-quarter front view turned 45 degrees to the left of the viewer, the same orthographic camera and the same scale for all three, nothing cropped, nothing overlapping. From left to right: 1) the MALE: a friendly cartoon chameleon clinging to a short brown branch, rounded chunky body, leaf-green (#6DBE45) with a few flat colour patches in turquoise (#35B7C6), yellow (#F4C542) and orange (#F28C38), a tightly curled spiral tail, big round independently turned eyes in cone-shaped turrets (the two eyes looking in slightly different directions, funny but friendly), a small rounded casque crest and two tiny horn bumps on the head, gentle closed smile; full size (100 %), about 35 cm long. 2) the FEMALE, the same species and the same design, with ONE clear but gentle difference: the female is a little smaller (about 90 %) with softer, paler pastel colours (soft green, pale peach and light blue patches), NO crest or horn bumps (a smooth rounded head), and slightly longer eyelashes. 3) the BABY of this pair: the BABY: a tiny round chameleon about 40 % of his size, huge head and huge eyes, short tail curled in a small spiral, light fresh green with a few pale yellow patches, also on a little twig. The baby is clearly the child of these two: same colours and markings, bigger head and bigger eyes in relation to the body. All three are friendly, calm and gentle with a neutral happy expression, closed smiling mouth, no teeth, no fangs, eyes with a white highlight. No clothes, no bows, no ribbons, no jewellery, no make-up, nothing stereotyped.
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Same species and design in all three; friendly, never scary (no teeth/fangs, no drama).
- [ ] Female with the described gentle difference, readable from the game camera (FAM-007).
- [ ] Baby clearly the child; bigger head and eyes.
- [ ] Plain grey background; no text; nothing cropped.
- [ ] Buildable: low-poly body, separate eye turrets that can rotate, tail as a bone chain.

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-10-03 | family_v1.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: zebra_family.png | generated, to review |
| 2026-10-03 | family_v2.jpg | gemini-3-pro-image (2K, 21:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: zebra_family.png | generated, to review |
