# Brief — `kit_terrarium` (terrarium case prop sheet)

Spec: GAME-NIGHT (terrarium), ART-PROPS, ART-PIPELINE §3. Style: `art/style/style.md` (comic). Pattern: `art/props/kit_night/brief.md`. Status: **in-review** (generated 2026-10-03).

## Purpose

The glass **terrarium case** used in `terrarium_house` (`art/environment/env_terrarium_house/brief.md`): a glass case with a warm wooden rim, a lamp on top, a front panel with a slot for the animal pictogram (the case is the enclosure for the snake, chameleon or poison dart frog), and three interior dressing variants (snake: rocks + branch; chameleon: branches + leaves; frog: ferns + pond). Budget: props ≤ 500 triangles each (ART-PIPELINE §10), flat colours; glass drawn as a flat pale-blue translucent shape with a white highlight stripe.

## Image settings

16:9, 2K, 2 variants. References: `art/props/kit_night/sheet_lights_v2.jpg` (sheet layout) + `art/environment/env_night_house/cutaway.png` (look).
`--extra`: "The first reference image is an approved prop sheet of this game: match its comic style, line weight, camera angle, grey background and sheet layout exactly, but draw the props described above. No text anywhere."

## Prompt 1 — `sheet`

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Game asset sheet: the props below are laid out in a neat grid with generous empty space between them, each prop isolated and complete, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a soft small contact shadow under each prop. Every prop is shown from the same elevated three-quarter top-down view as in a cozy zoo park simulation game (camera looking down at about 55 degrees, isometric-like, narrow field of view so vertical lines stay nearly parallel), all at the same scale.

Terrarium props for a friendly children's zoo, two rows. Top row: 1) an EMPTY terrarium case about 1.6 m wide and 1.2 m high: a chunky warm-brown wooden base and rim, big clear glass front and side panes drawn as flat pale-blue translucent shapes with a white highlight stripe, a flat lid with a small warm lamp shade on top, a sand floor; on the front of the wooden base a small blank cream rounded panel slot where a pictogram will go; 2) the SNAKE case: the same case dressed with grey rocks and a thick curved branch, with a small cream pictogram panel showing a green curled snake; 3) the CHAMELEON case: the same case dressed with several thin branches and big green leaves, pictogram panel with a green chameleon. Bottom row: 4) the POISON DART FROG case: the same case dressed with ferns, moss and a small blue pond, pictogram panel with a blue frog on a leaf; 5) loose dressing parts for modelling: a thick curved branch, a thin forked branch, a pile of three grey rocks, a big green leaf, a fern, a small round blue pond; 6) a small free-standing board stand with a blank cream panel and a little warm lamp. The lamps are drawn as flat bright warm-yellow shapes. Everything is friendly and chunky, nothing scary, no animals, all boards blank, no text.
```

### Negative prompt

```text
text, letters, words, numbers, writing, watermark, logo, scenery, cages, cage bars, animals, snakes, spiders, skulls, scary, realistic glass reflections, clutter, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Case readable: rim, glass, lamp, pictogram slot; three dressing variants distinct.
- [ ] Same comic style as the other prop sheets; flat colours; no text.

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-10-03 | sheet_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_lights_v2.jpg, cutaway.png | generated, to review |
| 2026-10-03 | sheet_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: sheet_lights_v2.jpg, cutaway.png | generated, to review |
