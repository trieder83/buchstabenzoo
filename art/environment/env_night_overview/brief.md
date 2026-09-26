# Brief — `env_night_overview` (mood of night level 1)

Spec: GAME-NIGHT (rule 4 night zoo `night_1`, rules 1, 2, 5, 10), ART-ENVIRONMENT "Night art", GAME-LAYOUT. Plan: `art/night/README.md`. Layout: **not designed yet** — `specs/10-gameplay/levels/night-1.md` and `assets/levels/night-1.toml` belong to the `zoo-level-designer`; this is a **mood image** (no `layout.md`), positions are free. Once the layout exists, a layout-true overview follows the ART-PIPELINE greybox order. Style: `art/style/style.md` (comic) — references: `art/environment/style_frame/style_frame.png`, the approved night style frame, and `art/environment/env_level1_overview/overview.png` (for the overview camera). Status: **brief** (not generated — Gemini monthly spending cap, HTTP 429, 2026-09-26).

## Purpose

Show the user and the level designer what the night zoo **feels** like before its layout is
drawn: a moonlit forest garden behind the moon door, friendly and cosy, lit by lanterns,
with the landmarks of GAME-NIGHT rule 4 and the three night_1 animals' hiding places
(GAME-NIGHT table: hedgehog under the leaf pile by the hedge, bat under the bridge / in the
old tree, owl on the highest branch).

## Must be visible

- The **moon door** (open, glowing) at the bottom edge — the way in from the day zoo.
- Lantern-lit light sand-beige paths looping through the garden (lantern posts, string lights
  over one small plaza with benches).
- The **night house** (rounded, grass roof, blue and red-orange porthole windows).
- A **pond** with a small wooden bridge, stars reflected in the water, a few lily pads.
- **Old trees** (one very tall with a round knot hole), a **meadow** with fireflies, a **small
  hill** with rocks, a hedge with a **leaf pile**.
- Empty night enclosures with blank enclosure signs and info boards with small lamps; a food
  storage hut with lit windows.
- The girl with her lantern on the path near the moon door.

## Must not appear

- No animals visible (they are hidden — rescue loop); no sky, no text, nothing scary.

## Image settings

| File | Aspect / size | Camera |
|---|---|---|
| `overview_v1/v2` → `overview.png` | 16:9, 2K | high camera ≈ 60–65°, whole area in frame like a diorama, no sky (like `env_level1_overview`) |

`--extra`: "The first reference image is the approved style frame of this game: match its comic style, line weight, outlines and high camera look exactly, but not its layout or its daylight. The second reference image is the approved night look — match its night lighting. No text anywhere."

## Prompt 1 — `overview`

First paragraph = STYLE block from `art/style/style.md`, verbatim.

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

NIGHT LIGHTING — this replaces the midday sunlight described above: a calm, friendly night. There is no sun. Soft cool moonlight from the upper left gives a deep friendly blue palette — medium deep blue and blue-violet on grass, leaves and walls, lighter soft blue on the tops of objects, one hard-edged darker blue shadow tone, never pitch black; every object stays clearly visible with its bold dark outlines and flat colours. The second light is warm yellow-orange lamp light: lamps and lit windows glow as flat bright warm-yellow shapes and throw round, hard-edged pools of warm light on the ground. Cosy, magical and safe like a bedtime story for young children — nothing scary, nothing hidden in darkness.

Elevated three-quarter top-down view like a cozy zoo park simulation game: a high camera looking down at about 60 to 65 degrees, isometric-like perspective with a narrow field of view so vertical lines stay nearly parallel, horizontal widescreen composition, the whole area in frame and neatly laid out like a diorama, the ground fills the image, no horizon, no sky.

A friendly night zoo in a moonlit forest garden, surrounded by tall soft blue hedges and old round trees. At the bottom centre a big wooden gate between two stone pillars stands wide open: blue plank door leaves with small painted stars, a round sign above it with a glowing cream crescent moon, a warm lantern on each pillar, star sparkles in the doorway. From the gate a light sand-beige path loops through the garden, lined with wooden lantern posts whose lanterns glow warm yellow and throw round hard-edged pools of warm light. On the left a round pond with a small wooden bridge, lily pads and white star sparkles reflected in the blue water. At the top a friendly rounded night house with a dark-green grass roof, a painted moon on its wall and round porthole windows glowing soft blue and warm red-orange. On the right a small grassy hill with grey rocks and a meadow of tall grass with many tiny warm yellow-green fireflies. One very tall old tree with a big round knot hole stands near the pond; along the hedge lies a heap of autumn leaves. In the middle a small plaza with two benches under a string of warm glowing bulbs, and a small wooden food hut with warmly lit windows. Three empty enclosures with low wooden fences, each with a blank cream enclosure sign and a small blank info board with a little lamp on top. On the path near the gate a small girl (long straight dark-brown hair, white T-shirt with blue stripes, blue jeans), about one twentieth of the image height, carries a small glowing lantern with a soft warm light circle around her. No animals are visible. Magical, cosy and safe like a bedtime story for young children — warm lights against soft friendly blue, nothing scary. All signs and boards are blank, no text anywhere.
```

### Negative prompt

```text
text, letters, words, numbers, writing, captions, writing on signs, watermark, signature, logo, brand names, UI, HUD, buttons, daylight, sunshine, midday sun, pitch black, deep black shadows, unreadable dark areas, dark forest, dense spooky woods, bare dead trees, horror, scary, spooky, halloween, creepy, haunted, graveyard, menacing shapes, monsters, ghosts, glowing red eyes, eyes in the dark, skulls, fog, mist, thunderstorm, rain, animals, cages, cage bars, rubbish, crowds, clutter, distorted anatomy, fisheye distortion, blurry, low resolution, cropped main subject, sky, horizon, moon in the sky, clouds, low camera angle, eye-level view, close-up, strong perspective distortion, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Comic style; night look matches `style_frame_night` (blue + warm lanterns, no black).
- [ ] Friendly, cosy, magical — nothing scary for 4–6 year olds (NIGHT-009).
- [ ] Landmarks of GAME-NIGHT rule 4 recognisable: moon door, night house, pond, old trees, meadow, hill.
- [ ] Paths clearly lit; the girl clearly visible in her light circle.
- [ ] Readable at ≈ 25 % size (phone): lanterns, paths, landmarks.
- [ ] Mood only — the level designer derives the real `night_1` layout.

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
