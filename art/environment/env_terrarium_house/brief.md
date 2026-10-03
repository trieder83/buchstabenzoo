# Brief — `env_terrarium_house` (the terrarium house of the night level)

Spec: GAME-NIGHT (terrarium, enterable night building), GAME-PLAYER §2 (enterable building, roof disappears inside), ART-ENVIRONMENT "Night art". Pattern: `art/environment/env_night_house/brief.md` (same look; use its `overview.png` and `cutaway.png` as references). Layout: to be written in the level spec (positions here are a mood proposal only). Style: `art/style/style.md` (comic). Status: **in-review** (generated 2026-10-03; awaiting user review).

## Purpose

A **small enterable building in the night level** that houses the terrarium animals (snake, chameleon, poison dart frog — see `art/animals/*_family/brief.md`). Friendly, round-roofed like the night house but smaller (≈ 9 × 6 m) and recognisable by its **big warm-lit glass windows** with terrarium cases visible through them and a **big animal icon (a snake-and-leaf pictogram)** above the door — no text.

## Must be visible

- Exterior (prompt 1): chunky rounded building ≈ 9 × 6 m, warm-brown wooden walls, low rounded dark-green grass roof, wide arched entrance with a `wall_lamp` on each side, a big round pictogram sign above the door (a green snake curled around a leaf — no letters), two big square windows glowing warm yellow-orange through which glass terrarium cases with green plants are visible, a small blank info board with its lamp, lantern posts on the sand-beige path, bushes, rocks, fireflies, the girl with her lantern.
- Cut-away (prompt 2): roof removed; a wooden walkway along the front; three lit glass terrarium cases along the back and side walls: left **snake case** (grey rocks and a thick branch), middle **chameleon case** (several branches and big leaves), right **poison dart frog case** (ferns and a small pond); warm lamps over each case, a blank board stand in front of each; the girl with her lantern. The cases are empty (animals still out — rescue loop).

## Must not appear

- No cages, no bars, no dark rooms, no scary reptile drama; no text on any sign; no animals inside (empty before the rescue).

## Props used

`terrarium_house` (+ cut-away), `terrarium_case` (`art/props/kit_terrarium/brief.md`), `wall_lamp`, `board_lamp`, `lantern_post`, `info_board`, `path_tile`, `bush`, `rock`, `firefly`.

## Image settings

| File | Aspect / size | Camera |
|---|---|---|
| `overview_v1/v2` | 16:9, 2K | high camera ≈ 55–60°, whole building + surroundings in frame, no sky |
| `cutaway_v1/v2` | 16:9, 2K | high camera ≈ 55°, roof removed, looking down into the room |

References: `art/environment/env_night_house/overview.png` (prompt 1) / `cutaway.png` (prompt 2).
`--extra`: "The attached image is the approved night house of this game: match its comic style, line weight, night lighting, high camera and look exactly, but draw the different, smaller building described above. No text anywhere."

## Prompt 1 — `overview` (exterior at night)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

NIGHT LIGHTING — this replaces the midday sunlight described above: a calm, friendly night. There is no sun. Soft cool moonlight from the upper left gives a deep friendly blue palette — medium deep blue and blue-violet on grass, leaves and walls, lighter soft blue on the tops of objects, one hard-edged darker blue shadow tone, never pitch black; every object stays clearly visible with its bold dark outlines and flat colours. The second light is warm yellow-orange lamp light: lamps and lit windows glow as flat bright warm-yellow shapes and throw round, hard-edged pools of warm light on the ground. Cosy, magical and safe like a bedtime story for young children — nothing scary, nothing hidden in darkness.

Elevated three-quarter top-down view like a cozy zoo park simulation game, horizontal widescreen composition: a high camera looking down at about 55 to 60 degrees, isometric-like perspective with a narrow field of view so vertical lines stay nearly parallel, the ground fills the image, no horizon, no sky.

A friendly small terrarium house in a moonlit zoo garden: a chunky rounded building about 9 by 6 m with thick warm-brown wooden walls and a low rounded dark-green grass roof; two big square windows in the front wall glow flat warm yellow-orange, and through the glass we see glass terrarium cases with green plants, branches and rocks inside (empty of animals); above the wide arched wooden entrance door, open, a big round cream sign with a simple painted green snake curled around a leaf (a pictogram, no letters); a warm yellow wall lantern on each side of the door; next to the door a small blank cream-coloured info board with a little lamp on top. A light sand-beige path leads to the door, lined with two wooden lantern posts, each lantern glowing warm yellow and throwing a round hard-edged pool of warm light on the path. Around the building soft blue bushes, a few grey rocks, big leafy tropical plants in pots and a few tiny warm yellow-green fireflies. In the middle of the path a small girl (long straight dark-brown hair, white T-shirt with blue stripes, blue jeans), about one twelfth of the image height, holds a small glowing lantern with a soft warm light circle around her. Magical, cosy and inviting like a bedtime story — nothing scary. All boards are blank, no text anywhere.
```

## Prompt 2 — `cutaway` (inside, roof removed)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

NIGHT LIGHTING — this replaces the midday sunlight described above: a calm, friendly night. There is no sun. Soft cool moonlight from the upper left gives a deep friendly blue palette — medium deep blue and blue-violet on grass, leaves and walls, lighter soft blue on the tops of objects, one hard-edged darker blue shadow tone, never pitch black; every object stays clearly visible with its bold dark outlines and flat colours. The second light is warm yellow-orange lamp light: lamps and lit windows glow as flat bright warm-yellow shapes and throw round, hard-edged pools of warm light on the ground. Cosy, magical and safe like a bedtime story for young children — nothing scary, nothing hidden in darkness.

Elevated three-quarter top-down view like a cozy zoo park simulation game, horizontal widescreen composition: a high camera looking down at about 55 degrees, isometric-like perspective with a narrow field of view so vertical lines stay nearly parallel, no horizon, no sky.

Inside a friendly small zoo terrarium house, seen from above with the roof completely removed like a doll's house, so we look down over thick warm-brown wooden walls. A warm wooden walkway runs through the room in front of the walls, lit by a few small warm yellow wall lamps. Along the back wall and the side wall stand three big glass terrarium cases with a warm wooden rim and a warm lamp above each, each lit in warm yellow-orange light so everything is clearly visible: the left case with grey rocks, sand and a thick climbing branch (snake case); the middle case with several thin branches and big green leaves (chameleon case); the right case with ferns, moss and a small blue pond (poison dart frog case). All three cases are empty and tidy. In front of each case stands a small blank cream-coloured info board stand with a little warm lamp on top. In the room a small girl (long straight dark-brown hair, white T-shirt with blue stripes, blue jeans) holds a small glowing lantern, a soft warm light circle around her. Calm, magical and friendly like a museum at bedtime — dim colours but no dark corners, nothing scary. All boards are blank, no text anywhere.
```

### Negative prompt (both prompts)

```text
text, letters, words, numbers, writing, captions, writing on signs, watermark, signature, logo, brand names, UI, HUD, daylight, sunshine, pitch black, deep black shadows, black rooms, dark tunnels, unreadable dark areas, harsh red danger light, horror, scary, spooky, creepy, snakes inside the cases, animals inside the cases, skulls, cobwebs, spiders, fog, cages, cage bars, prison, laboratory, crowds, clutter, rubbish, fisheye distortion, sky, horizon, low camera angle, eye-level view, close-up, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Same comic night look as the night house; friendly and inviting from outside; the lit windows + snake pictogram make it recognisable.
- [ ] Inside: three cases readable (rocks+branch / branches+leaves / ferns+pond); everything lit, no black areas.
- [ ] Cases empty; no cages or bars; no text.

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-10-03 | overview_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: overview.png | generated, to review |
| 2026-10-03 | overview_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: overview.png | generated, to review |
| 2026-10-03 | cutaway_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 2 + negative as 'Avoid' + extra text, ref: cutaway.png | generated, to review |
| 2026-10-03 | cutaway_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 2 + negative as 'Avoid' + extra text, ref: cutaway.png | generated, to review |
