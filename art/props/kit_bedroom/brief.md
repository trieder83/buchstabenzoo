# Brief — `kit_bedroom` (bed and bedroom corner in the zookeeper house)

Spec: GAME-NIGHT (rule 3 "Sleep": a bed in the zookeeper house; NIGHT-003), GAME-PLAYER §2 (enterable building, roof disappears inside), ART-ENVIRONMENT "Night art". Plan: `art/night/README.md`. House: `zookeeper_house` (`art/props/kit_buildings`, `art/environment/env_zookeeper_house`). Style: `art/style/style.md` (comic) — style references: `art/environment/style_frame/style_frame.png` (approved) and `art/props/kit_fences/sheet_v3.jpg`; for prompt 2 also the approved night style frame. Status: **in-review** (generated 2026-09-27 with gemini-3-pro-image — see the generation log for the chosen variant; awaiting user review).

## Purpose

The **bed** is one of the two night choices (🛏 sleep → dream/fade → next morning). It stands
in a cosy bedroom corner of the zookeeper house, seen from the high camera with the roof
cut away. The furniture is used by day and night; at night the **bedside lamp** and the
**window with the moon** glow (emissive), so the corner is the warmest, safest spot of the
night zoo. The bed must read at a glance as "go to sleep here" (big pillow, blanket turned
down).

Budgets: furniture ≤ 500 triangles each (bed ≤ 800), flat-colour textures only.

## Props in this kit

| Asset id | Size (1 unit = 1 m) | What it shows | Glow (emissive) | Prio night_1 |
|---|---|---|---|---|
| `bed` | 1.0 × 2.0 m, 0.5 m high, headboard 1.0 m | chunky wooden child-size bed, rounded headboard with a small carved star, big fluffy white pillow, **blue blanket** with white stars (as in `env_zookeeper_house`), corner turned down; interaction point at the long side. The player lies down on it (`sleep` animation, ART-RIG) | — | **P1** |
| `night_table` | 0.5 × 0.4 m, 0.55 m high | small wooden night table with one drawer, a book and a cup on it | — | **P1** |
| `bedside_lamp` | 0.4 m | round warm lamp with a cream shade on the night table | shade `#FFD66B`; point light r = 2.5 m | **P1** |
| `window_moon` | 1.0 × 1.2 m, in the wall | square wooden window with a cross bar and short blue curtains; through the panes a flat dark-blue night sky with a round cream moon and two stars (a texture, no real view) | moon `#FFF4C9`, stars `#FFFFFF` (night only; by day the panes show light blue sky) | **P1** |
| `rug_round` | Ø 1.4 m | round braided rug in soft blue and cream | — | P2 |
| `toy_chest` | 0.8 × 0.45 m | wooden chest with a rounded lid, a plush elephant peeking out | — | P3 |

## Image settings

- Prompt 1 — `sheet_bedroom` asset sheet (sheet light, for modelling), 16:9, 2K, 2 variants, refs: style frame + `kit_fences/sheet_v3.jpg`.
- Prompt 2 — `bedroom_night` the furnished bedroom corner at night, roof cut away, high camera, 16:9, 2K, 2 variants, refs: style frame (+ approved night style frame).
- `--extra` as in `art/props/kit_night/brief.md`.

## Prompt 1 — `sheet_bedroom`

First paragraph = STYLE block from `art/style/style.md`, verbatim.

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Game asset sheet: the props below are laid out in a neat grid with generous empty space between them, each prop isolated and complete, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a soft small contact shadow under each prop. Every prop is shown from the same elevated three-quarter top-down view as in a cozy zoo park simulation game (camera looking down at about 55 degrees, isometric-like, narrow field of view so vertical lines stay nearly parallel), all at the same scale.

Cosy bedroom furniture for a small wooden zookeeper house, two rows with generous space between them. Top row: 1) a chunky child-size wooden bed about 2 m long, a rounded warm-brown wooden headboard with a small carved star, a big fluffy white pillow and a friendly blue blanket with small white stars, the blanket corner turned down invitingly; 2) a small wooden night table with one drawer, a small book and a cup on top; 3) a round bedside lamp with a warm cream lamp shade, its shade drawn in flat bright warm yellow as if switched on. Bottom row: 4) a square wooden window piece set in a short piece of warm-brown plank wall, a wooden cross bar, short blue curtains at the sides, and through the panes a flat dark-blue night sky with a round pale-cream comic moon with a dark outline and two small four-pointed stars; 5) a round braided rug in soft blue and cream; 6) a wooden toy chest with a rounded lid, slightly open, a cute grey plush elephant peeking out. Simple, rounded and inviting.
```

## Prompt 2 — `bedroom_night` (scene, roof cut away)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

NIGHT LIGHTING — this replaces the midday sunlight described above: a calm, friendly night. There is no sun. Soft cool moonlight from the upper left gives a deep friendly blue palette — medium deep blue and blue-violet on grass, leaves and walls, lighter soft blue on the tops of objects, one hard-edged darker blue shadow tone, never pitch black; every object stays clearly visible with its bold dark outlines and flat colours. The second light is warm yellow-orange lamp light: lamps and lit windows glow as flat bright warm-yellow shapes and throw round, hard-edged pools of warm light on the ground. Cosy, magical and safe like a bedtime story for young children — nothing scary, nothing hidden in darkness.

Elevated three-quarter top-down view like a cozy zoo park simulation game, horizontal widescreen composition: a high camera looking down at about 55 degrees, isometric-like perspective with a narrow field of view so vertical lines stay nearly parallel, no horizon, no sky.

Inside a small cosy wooden zookeeper house at night, seen from above with the roof completely removed like a doll's house, so we look down into the room over the warm-brown plank walls. In the bedroom corner: a chunky child-size wooden bed with a rounded headboard, a big fluffy white pillow and a blue blanket with small white stars, the corner turned down invitingly; beside it a small wooden night table with a round bedside lamp glowing flat bright warm yellow, throwing a round hard-edged pool of warm light over the bed and the floor; a square window in the wall with short blue curtains, showing a round pale-cream moon and two stars in the dark-blue night outside; a round soft blue-and-cream rug on the wooden floor; a wooden toy chest with a plush elephant peeking out. In the other half of the room, dimmer but still clearly visible in soft blue: a table with a big empty round glass bowl, shelves with buckets, a broom and a pair of rubber boots by the door. A small girl (long straight dark-brown hair, white T-shirt with blue stripes, blue jeans), small in the image, stands on the rug next to the bed holding a small glowing lantern, a soft warm light circle around her. Outside the walls the grass is soft deep blue with a few tiny warm yellow fireflies. The room is the warmest, safest-feeling place — cosy like a bedtime story, nothing scary. No text anywhere.
```

### Negative prompt (both prompts)

```text
text, letters, words, numbers, writing, captions, writing on signs, watermark, signature, logo, brand names, UI, HUD, adult bedroom, double bed, bunk bed, hospital bed, cage, dark corners, pitch black, deep black shadows, horror, scary, spooky, halloween, creepy, monsters under the bed, shadows of monsters, menacing shapes, ghosts, glowing red eyes, fog, mess, clutter, rubbish, broken furniture, roof, ceiling covering the room, sky, horizon, low camera angle, eye-level view, fisheye, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Comic style matches the style frame (outlines, flat colours, one shadow tone).
- [ ] The bed reads instantly as "sleep here" from the high camera at phone size.
- [ ] The bedroom at night is warm, safe and cosy — nothing scary (NIGHT-009), no dark corners.
- [ ] Lamp shade and window moon are clear flat glowing areas (→ `*_glow` slots).
- [ ] Furniture fits the 4 × 4 m (kit_buildings) / 7 × 6 m (env_zookeeper_house) house — size open, see Q-092.
- [ ] Buildable as low-poly props (≤ 500 tris, bed ≤ 800).

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-09-27 | bedroom_night_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 2 + negative as 'Avoid' + extra text, ref: style_frame.png, style_frame_night_v1.jpg | downscaled to 2048 px; alternative — cosy, but a moon/sky corner is visible, two glass bowls and duplicated broom |
| 2026-09-27 | bedroom_night_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 2 + negative as 'Avoid' + extra text, ref: style_frame.png, style_frame_night_v1.jpg | downscaled to 2048 px; **chosen** — one coherent cut-away room: bed, lit bedside lamp with light pool, window with moon, rug, toy chest with elephant, bowl table, broom and boots by the door; girl on-model with her lantern; warm and safe |
| 2026-09-27 | sheet_bedroom_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, sheet_v3.jpg | downscaled to 2048 px; **chosen** — all 6 props, lamp shade glows flat yellow (separable glow slot), plush elephant clearly readable; window shows a crescent moon (brief: round moon) |
| 2026-09-27 | sheet_bedroom_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, sheet_v3.jpg | downscaled to 2048 px; alternative — round moon in the window as briefed, but the whole lamp glows and the elephant is harder to read |
