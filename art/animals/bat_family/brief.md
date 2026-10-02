# Brief — `bat_family` (family sheet: bat_female + bat_pup)

Spec: GAME-FAMILY (pairs and babies, §3 look), ART-ANIMALS, ART-PIPELINE §3. Style: `art/style/style.md` (comic). Reference: the approved adult
`art/animals/bat/sheet_v2.jpg` (= the male, as for the zebra and koala families). Pattern: `art/animals/zebra_female/brief.md`, `art/animals/zebra_family.png`.
Status: **in-review** (generated 2026-10-02). Manifest ids covered: `bat_female`, `bat_pup` (both `concept_approved = false` until a human approves).

## Character

- **Female** (`bat_female`): same species, same rig and animations as `bat`, about 10 % smaller (≈ 91 %); one clear, gentle difference for children (FAM-007): slightly smaller and slimmer, a lighter warm brown with a soft pale chest, slightly smaller ears, longer eyelashes. No clothing, bows, make-up or other stereotypes.
- **Baby** (`bat_pup`): about 45 % of the male's height, bigger head and bigger eyes, rounder and fluffier; always with the female (GAME-FAMILY). Name in the game: pup (Fluent).
- Male for comparison: a friendly brown fruit bat standing with folded dark brown wings, a fluffy tan chest, large pointed ears with pink insides, a small snout.
- Size: adult male about 0.4 m tall standing.

## Image-generation prompts

One **family sheet** at **21:9, 2K**: male | female | baby side by side in the same ¾ view and scale. Reference: the chosen adult sheet `art/animals/bat/sheet_v2.jpg`.
`--extra`: "The attached image is the approved turnaround sheet of the adult male of this game. Draw his whole family in exactly the same style, colours, line weight, eye style and grey background. The first figure (male) must look like the male in the attached image; then his smaller female partner and their baby, as described above."
After approval: full turnarounds (front, side, back, ¾) of female and baby like the zebra family, or model directly from this sheet with the male's rig (decision of the user).

**Negative prompt:**

```text
text, letters, logo, watermark, scenery, grass, ground plane, gradient background, cropped feet, cropped ears, extra views, more than three figures, fewer than three figures, extra legs, missing legs, extra heads, open mouth with teeth, sharp teeth, angry, scary, realistic animal, bow, ribbon, flower, hair clip, jewellery, clothing, make-up, lipstick, pink cheeks, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

### `family_v1.jpg` / `family_v2.jpg`

```text
Comic-style 3D cartoon character sheet with a cel-shaded look: bold clean dark-brown outlines, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail. Even soft studio light, orthographic view, plain light grey background (#E6E6E6), same scale in every view.

Family sheet of one bat family: THREE animals standing side by side in one row, evenly spaced, hooves/feet on the same baseline (the goldfish swim at the same height), all in the same three-quarter front view turned 45 degrees to the left of the viewer, the same orthographic camera and the same scale for all three, nothing cropped, nothing overlapping. From left to right: 1) the MALE, exactly as in the reference image: a friendly brown fruit bat standing with folded dark brown wings, a fluffy tan chest, large pointed ears with pink insides, a small snout, full size (100 %); 2) the FEMALE, the same species and the same design and colours, about 10 % smaller (about 91 % of his height) with ONE clear but gentle difference: slightly smaller and slimmer, a lighter warm brown with a soft pale chest, slightly smaller ears, longer eyelashes; 3) the BABY (pup) of this pair, about 45 % of the male's height: a baby bat pup: very big head and huge shiny eyes, comically big ears, small round fluffy body, small wings wrapped around the body. The baby is clearly the child of these two: same colours and markings, but with a bigger head in relation to the body and bigger eyes. All three are friendly, calm and gentle with a neutral happy expression, mouth closed, no teeth, eyes with a white highlight. adult male about 0.4 m tall standing. No clothes, no bows, no ribbons, no jewellery, no make-up, nothing stereotyped.
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Same species and design in all three; the male matches the approved adult.
- [ ] Female ≈ 10 % smaller with the described gentle difference, readable from the game camera (FAM-007).
- [ ] Baby ≈ 45 %, bigger head and eyes, clearly the child.
- [ ] No clothing, bows or stereotypes; friendly faces, no teeth; plain grey background; no text; nothing cropped.
- [ ] Buildable with the male's rig and mesh (difference = scale + small detail; baby = reshaped).

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
