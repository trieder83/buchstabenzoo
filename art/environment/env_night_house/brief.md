# Brief — `env_night_house` (the night house of `night_1`)

Spec: GAME-NIGHT (rule 4 "night house: dim indoor enclosures with red/blue light, like real zoo nocturnal houses", rules 2, 10), GAME-PLAYER §2 (enterable building, roof disappears inside), ART-ENVIRONMENT "Night art". Plan: `art/night/README.md`. Layout: `specs/10-gameplay/levels/night-1.md` (to be written by the `zoo-level-designer`; positions here are a mood proposal only). Style: `art/style/style.md` (comic) — style references: `art/environment/style_frame/style_frame.png` and, once approved, `art/environment/style_frame_night/style_frame_night.png`. Status: **brief** (not generated — Gemini monthly spending cap, HTTP 429, 2026-09-26).

## Purpose

The night house is the **landmark building of the night zoo**: a friendly, round-roofed
building where some night animals live in glass-fronted indoor enclosures. Its hint for
riddles is its **coloured light** (soft blue and warm red-orange windows). Inside it is dim
like a real nocturnal house — but in the game it must stay **readable and friendly**: flat
blue and warm red-orange light areas, bold outlines, every animal and every board visible
(NIGHT-005, NIGHT-009).

## Must be visible

- Exterior (prompt 1): a chunky rounded building ≈ 12 × 8 m with a low dark-green grass roof
  and a big painted moon and stars on the front wall, round porthole windows glowing soft
  blue and warm red-orange, a wide arched entrance with a `wall_lamp` on each side, an info
  board with its board lamp next to the door, lantern posts along the path, an old tree and
  bushes around it, fireflies.
- Cut-away (prompt 2): roof removed; a curved visitor corridor with a wooden floor; three
  glass-fronted indoor enclosures side by side — one lit **soft blue** (moonlight room: logs,
  branches), one lit **warm red-orange** (sand and rocks), one **blue** with a small tree
  trunk and leaves; blank info boards with board lamps in front of each glass; the girl with
  her lantern in the corridor. The enclosures are empty (the animals are still out — rescue
  loop).

## Must not appear

- No cages, no bars, no dark tunnels, no black rooms, no harsh red "danger" light.
- No text on any sign; no animals inside (empty before the rescue).

## Props used

`night_house` (+ cut-away), `wall_lamp`, `board_lamp`, `lantern_post`, `info_board`,
`path_tile`, `tree_round` (old tree variant from the night_1 layout), `bush`, `rock`, `firefly`.

## Image settings

| File | Aspect / size | Camera |
|---|---|---|
| `overview_v1/v2` → `overview.png` | 16:9, 2K | high camera ≈ 55–60°, whole building + surroundings in frame, no sky |
| `cutaway_v1/v2` → `cutaway.png` | 16:9, 2K | high camera ≈ 55°, roof removed, looking down into the corridor and the three enclosures |

References: style frame (+ approved night style frame, + `kit_night` sheet once approved).
`--extra`: "The first reference image is the approved style frame of this game: match its comic style, line weight, outlines and high camera look exactly, but not its layout or its daylight. No text anywhere."

## Prompt 1 — `overview` (exterior at night)

First paragraph = STYLE block from `art/style/style.md`, verbatim.

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

NIGHT LIGHTING — this replaces the midday sunlight described above: a calm, friendly night. There is no sun. Soft cool moonlight from the upper left gives a deep friendly blue palette — medium deep blue and blue-violet on grass, leaves and walls, lighter soft blue on the tops of objects, one hard-edged darker blue shadow tone, never pitch black; every object stays clearly visible with its bold dark outlines and flat colours. The second light is warm yellow-orange lamp light: lamps and lit windows glow as flat bright warm-yellow shapes and throw round, hard-edged pools of warm light on the ground. Cosy, magical and safe like a bedtime story for young children — nothing scary, nothing hidden in darkness.

Elevated three-quarter top-down view like a cozy zoo park simulation game, horizontal widescreen composition: a high camera looking down at about 55 to 60 degrees, isometric-like perspective with a narrow field of view so vertical lines stay nearly parallel, the ground fills the image, no horizon, no sky.

A friendly night house in a moonlit zoo garden: a chunky rounded building about 12 by 8 m with thick warm-brown wooden walls and a low rounded dark-green grass roof, on its front wall a big simple painted cream crescent moon and a few painted stars; round porthole windows along the walls glow flat soft blue and flat warm red-orange; a wide arched wooden entrance door, open, with a warm yellow wall lantern on each side; next to the door a small blank cream-coloured info board with a little lamp on top shining on its panel. A light sand-beige path leads to the door, lined with three wooden lantern posts, each lantern glowing warm yellow and throwing a round hard-edged pool of warm light on the path. Around the building an old friendly round tree with a big knot hole, soft blue bushes, a few grey rocks and a few tiny warm yellow-green fireflies. In the middle of the path a small girl (long straight dark-brown hair, white T-shirt with blue stripes, blue jeans), about one twelfth of the image height, holds a small glowing lantern with a soft warm light circle around her. Magical, cosy and inviting like a bedtime story — nothing scary. All boards are blank, no text anywhere.
```

## Prompt 2 — `cutaway` (inside, roof removed)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

NIGHT LIGHTING — this replaces the midday sunlight described above: a calm, friendly night. There is no sun. Soft cool moonlight from the upper left gives a deep friendly blue palette — medium deep blue and blue-violet on grass, leaves and walls, lighter soft blue on the tops of objects, one hard-edged darker blue shadow tone, never pitch black; every object stays clearly visible with its bold dark outlines and flat colours. The second light is warm yellow-orange lamp light: lamps and lit windows glow as flat bright warm-yellow shapes and throw round, hard-edged pools of warm light on the ground. Cosy, magical and safe like a bedtime story for young children — nothing scary, nothing hidden in darkness.

Elevated three-quarter top-down view like a cozy zoo park simulation game, horizontal widescreen composition: a high camera looking down at about 55 degrees, isometric-like perspective with a narrow field of view so vertical lines stay nearly parallel, no horizon, no sky.

Inside a friendly zoo night house for nocturnal animals, seen from above with the roof completely removed like a doll's house, so we look down over thick warm-brown wooden walls. A gently curved visitor corridor with a warm wooden floor runs along the front, lit by a few small warm yellow wall lamps. Behind low wooden railings and big clean glass fronts lie three small indoor enclosures side by side, each lit in its own soft flat colour but everything clearly visible: the left one in soft moonlight blue with mossy logs and a leaf pile; the middle one in warm red-orange light with sand, smooth rocks and a small hollow log; the right one in soft blue with a short tree trunk, climbing branches and leafy plants. All three enclosures are empty and tidy. In front of each glass stands a small blank cream-coloured info board with a little warm lamp on top lighting its panel. In the corridor a small girl (long straight dark-brown hair, white T-shirt with blue stripes, blue jeans) holds a small glowing lantern, a soft warm light circle around her. Calm, magical and friendly like a museum at bedtime — dim colours but no dark corners, nothing scary. All boards are blank, no text anywhere.
```

### Negative prompt (both prompts)

```text
text, letters, words, numbers, writing, captions, writing on signs, watermark, signature, logo, brand names, UI, HUD, daylight, sunshine, pitch black, deep black shadows, black rooms, dark tunnels, unreadable dark areas, harsh red danger light, alarm lights, horror, scary, spooky, halloween, creepy, haunted house, menacing shapes, monsters, ghosts, glowing red eyes, skulls, cobwebs, spiders, fog, cages, cage bars, prison, laboratory, animals inside the enclosures, crowds, clutter, rubbish, fisheye distortion, sky, horizon, moon in the sky, low camera angle, eye-level view, close-up, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Comic style (outlines, flat colours, one shadow tone) — night look matches `style_frame_night`.
- [ ] Friendly and inviting from outside; the coloured windows make it a clear landmark.
- [ ] Inside: dim coloured light but everything readable; boards lit; no black areas (NIGHT-005, NIGHT-009).
- [ ] The red-orange light reads warm, not alarming (Q-116).
- [ ] Enclosures empty; no cages or bars; no text.

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
