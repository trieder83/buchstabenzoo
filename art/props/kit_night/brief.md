# Brief — `kit_night` (night lights and the moon door)

Spec: GAME-NIGHT (rules 1, 2, 3, 5, 10; NIGHT-002, NIGHT-005, NIGHT-006), ART-ENVIRONMENT "Night art", ART-DIRECTION. Plan: `art/night/README.md`. Style: `art/style/style.md` (comic) — style references: `art/environment/style_frame/style_frame.png` (approved) and the approved prop sheet `art/props/kit_fences/sheet_v3.jpg`; once approved also `art/environment/style_frame_night/style_frame_night.png`. Status: **in-review** (generated 2026-09-27 with gemini-3-pro-image — see the generation log for the chosen variant; awaiting user review).

## Purpose

Everything that **glows** at night, plus the **moon door** into the night zoo. The renderer
makes the night (GAME-NIGHT §10); these props add the lamps it needs. Glowing parts are
separate `*_glow` material slots (emissive at night, pale cream by day) and every lamp has a
`light` empty for its point light (`art/night/README.md` "Emissive rule").

Budgets: props ≤ 500 triangles each (moon door ≤ 1 500), flat-colour textures only
(ART-PIPELINE §10).

## Props in this kit

| Asset id | Size (1 unit = 1 m) | What it shows | Glow (emissive) | Prio night_1 |
|---|---|---|---|---|
| `lantern_post` | 2.4 m high, lantern head ≈ 0.4 m | chunky warm-brown wooden post, a small curved arm, an old-fashioned four-sided lantern hanging from it (dark-brown metal frame, cream glass panes, small pointed roof) | glass `#FFD66B`; point light `#FFC46E`, r = 3 m, hard edge | **P1** |
| `string_lights` | 4 m and 6 m spans, sag 0.4 m | a dark cord with 8 / 12 round bulbs in alternating warm yellow, soft orange, pale cream; ends fit a hook on `lantern_post`, trees or fence posts | bulbs `#FFD66B`, `#FFB547`, `#FFF1C9`; no point lights (emissive only) | P2 |
| `hand_lantern` | 0.35 m high incl. handle | the player's small lantern: round chunky cream-and-red body (friendly toy-like), a top ring handle, round glass belly; carried at `socket_hand_l` (proposal, *Q-117*) | glass `#FFD66B`; player light circle `#FFC46E`, r = 2.5 m | **P1** |
| `board_lamp` | 0.45 m (arm + shade) | small curved wooden/brass arm with a round warm lamp shade, clipped to the top of the `info_board` and under the `map_board` roof (`socket_lamp`), shines down on the blank panel | shade underside `#FFD66B`; point light r = 1.2 m aimed at the panel | **P1** |
| `wall_lamp` | 0.4 m | small lantern on a wall bracket for doors of buildings and enclosure shelters | glass `#FFD66B`; point light r = 2 m | **P1** |
| `firefly` | 0.06 m (drawn × 2) | tiny round glowing dot with two tiny wings (particle, ≤ 4 tris) | `#EFFF8A` | P2 (*Q-115*) |
| `sky_moon`, `sky_stars` | textures | flat round pale-cream moon with a dark-brown outline and 3 flat crater spots; 4-point comic star sparkles in two sizes (close-view sky only, GAME-CAMERA-VIEWS rule 7; also reflected on water) | `#FFF4C9`, `#FFFFFF` / `#FFF1B8` | P2 |
| `moon_door` | 4 m wide, 4.5 m high (sign top) | friendly big wooden double gate set between two chunky stone pillars, dark-blue painted planks with small painted stars; above it a round wooden **moon sign** (crescent moon, cream on dark blue); a small lantern on each pillar. **States:** `closed` (by day: sign unlit, lanterns off), `opening` (sign glows, doors swing inwards, sparkles), `open` (both leaves open 90°, glowing sign, lanterns on, soft blue sparkle band in the opening). Leaves are separate nodes rotated in-game like `gate_wood`. | moon sign `#FFF4C9`, rim `#8FB8FF`, pillar lanterns `#FFD66B` | **P1** |

## Image settings

- **Sheet A (lights)** — prompts 1 (sheet light, for modelling) and 2 (the same props at night). 16:9, 2K, 2 variants each.
- **Sheet B (moon door)** — prompts 3 (three states in sheet light) and 4 (the three states at night). 16:9, 2K, 2 variants each.
- References: style frame + `kit_fences/sheet_v3.jpg` (+ the approved night style frame for prompts 2 and 4).
- `--extra` (prompts 1, 3): "The first reference image shows the game's art style; the second is an approved prop sheet of this game — match its comic style, line weight, colours, camera angle, grey background and sheet layout exactly, but draw the props described above. No text anywhere."
- `--extra` (prompts 2, 4): same, plus "The third reference image is the approved night look of this game — match its night lighting."

Example: `tools/gen_image.py art/props/kit_night/brief.md --prompt 1 --out sheet_lights_v1.png sheet_lights_v2.png --aspect 16:9 --size 2K --ref art/environment/style_frame/style_frame.png art/props/kit_fences/sheet_v3.jpg --extra "…"`

## Prompt 1 — `sheet_lights` (sheet light, for modelling)

First paragraph = STYLE block from `art/style/style.md`, verbatim.

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Game asset sheet: the props below are laid out in a neat grid with generous empty space between them, each prop isolated and complete, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a soft small contact shadow under each prop. Every prop is shown from the same elevated three-quarter top-down view as in a cozy zoo park simulation game (camera looking down at about 55 degrees, isometric-like, narrow field of view so vertical lines stay nearly parallel), all at the same scale.

Night-light props for a friendly children's zoo, two rows with generous space between them. Top row: 1) a lantern post about 2.4 m tall: a chunky warm-brown wooden post with a small curved wooden arm at the top, an old-fashioned four-sided lantern hanging from the arm with a dark-brown metal frame, cream-yellow glass panes and a small pointed dark-brown roof; 2) a string of round light bulbs on a dark cord hanging in a gentle curve between two short wooden posts, the bulbs alternating warm yellow, soft orange and pale cream; 3) a small child's hand lantern about 35 cm tall: a round chunky toy-like body in cream and friendly red, a round glass belly with a warm yellow flame shape inside, a big round carrying ring on top; 4) a small board lamp: a curved brass arm with a round warm lamp shade, clipped to the top edge of a small blank cream-coloured wooden info board on one post, the lamp shade pointing down at the panel. Bottom row: 5) a small wall lantern on a curved wooden bracket, fixed to a short piece of warm-brown plank wall; 6) three tiny fireflies as round warm yellow-green dots with tiny wings, drawn large for clarity; 7) a flat round pale-cream comic moon disc with a bold dark-brown outline and three flat crater spots, and next to it four simple four-pointed comic star sparkles in two sizes. The glass, bulbs and shades are drawn as flat bright warm-yellow shapes so the glowing parts are easy to see. All boards are completely blank.
```

## Prompt 2 — `sheet_lights_night` (the same props at night)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

NIGHT LIGHTING — this replaces the midday sunlight described above: a calm, friendly night. There is no sun. Soft cool moonlight from the upper left gives a deep friendly blue palette — medium deep blue and blue-violet on grass, leaves and walls, lighter soft blue on the tops of objects, one hard-edged darker blue shadow tone, never pitch black; every object stays clearly visible with its bold dark outlines and flat colours. The second light is warm yellow-orange lamp light: lamps and lit windows glow as flat bright warm-yellow shapes and throw round, hard-edged pools of warm light on the ground. Cosy, magical and safe like a bedtime story for young children — nothing scary, nothing hidden in darkness.

Game asset sheet at night: the props below are laid out in a neat grid with generous empty space between them, each prop isolated and complete, nothing overlapping, nothing cropped, on a plain flat deep friendly blue background (#2E3E7A) with only a small hard-edged pool of warm light on the ground under each lamp. Every prop is shown from the same elevated three-quarter top-down view as in a cozy zoo park simulation game (camera looking down at about 55 degrees, isometric-like, narrow field of view so vertical lines stay nearly parallel), all at the same scale.

The same night-light props for a friendly children's zoo, two rows with generous space between them, now switched on at night. Top row: 1) a lantern post about 2.4 m tall: a chunky wooden post with a small curved arm, an old-fashioned four-sided lantern hanging from it, its glass panes glowing flat bright warm yellow, throwing a round hard-edged warm pool of light on the ground around the post; 2) a string of round light bulbs on a dark cord hanging in a gentle curve between two short wooden posts, the bulbs glowing warm yellow, soft orange and pale cream; 3) a small child's hand lantern about 35 cm tall, round, chunky and toy-like in cream and friendly red, its round glass belly glowing warm yellow, a small round light pool beneath it; 4) a small curved brass lamp clipped to the top of a small blank cream-coloured wooden info board, shining warm light down on the blank panel so the whole panel is bright and clearly lit. Bottom row: 5) a small wall lantern on a wooden bracket on a short piece of plank wall, glowing warm yellow; 6) three tiny fireflies glowing yellow-green; 7) a flat round pale-cream comic moon disc with a bold dark-brown outline and three flat crater spots, and four simple four-pointed comic star sparkles. Wood keeps its warm brown where the lamp light falls on it and turns soft blue elsewhere. Cosy and friendly. All boards are completely blank.
```

## Prompt 3 — `sheet_moon_door` (three states, sheet light)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Game asset sheet: the props below are laid out in a neat grid with generous empty space between them, each prop isolated and complete, nothing overlapping, nothing cropped, on a plain light grey background (#E6E6E6) with only a soft small contact shadow under each prop. Every prop is shown from the same elevated three-quarter top-down view as in a cozy zoo park simulation game (camera looking down at about 55 degrees, isometric-like, narrow field of view so vertical lines stay nearly parallel), all at the same scale.

Three views of the SAME magical but friendly zoo gate, the moon door, side by side in one row with generous space between them, same scale and same angle: a big wooden double gate about 4 m wide between two chunky pillars of light grey rounded stones with wooden caps, each pillar with a small old-fashioned lantern on top; the two door leaves are made of chunky vertical planks painted a friendly medium blue with a few small painted cream-yellow stars; above the gate, carried by a thick curved warm-brown wooden beam between the pillars, a big round wooden sign showing a simple cream-yellow crescent moon on a dark blue background. 1) CLOSED: both door leaves shut, lanterns and moon sign not lit; 2) OPENING: the door leaves swing half open inwards, the moon sign and the lanterns start to glow warm, a few small star sparkles appear in the gap; 3) OPEN: both door leaves fully open inwards, the moon sign glowing bright cream-yellow, the lanterns glowing warm yellow, a soft band of little blue and cream star sparkles in the open doorway and a light sand-beige path leading through it. The glowing parts are drawn as flat bright shapes. No letters on the sign — only the moon picture.
```

## Prompt 4 — `sheet_moon_door_night` (three states at night)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

NIGHT LIGHTING — this replaces the midday sunlight described above: a calm, friendly night. There is no sun. Soft cool moonlight from the upper left gives a deep friendly blue palette — medium deep blue and blue-violet on grass, leaves and walls, lighter soft blue on the tops of objects, one hard-edged darker blue shadow tone, never pitch black; every object stays clearly visible with its bold dark outlines and flat colours. The second light is warm yellow-orange lamp light: lamps and lit windows glow as flat bright warm-yellow shapes and throw round, hard-edged pools of warm light on the ground. Cosy, magical and safe like a bedtime story for young children — nothing scary, nothing hidden in darkness.

Game asset sheet at night: the props below are laid out in a neat grid with generous empty space between them, each prop isolated and complete, nothing overlapping, nothing cropped, on a plain flat deep friendly blue background (#2E3E7A) with only small hard-edged pools of warm light on the ground under the lamps. Every prop is shown from the same elevated three-quarter top-down view as in a cozy zoo park simulation game (camera looking down at about 55 degrees, isometric-like, narrow field of view so vertical lines stay nearly parallel), all at the same scale.

Three views of the SAME friendly zoo gate at night, the moon door, side by side in one row with generous space between them: a big wooden double gate about 4 m wide between two chunky stone pillars with a small lantern on each, blue plank door leaves with small painted stars, and above the gate a big round wooden sign with a simple crescent moon. 1) CLOSED: both leaves shut, the moon sign and lanterns unlit, the whole gate in soft friendly blue moonlight, still clearly visible; 2) OPENING: the leaves swing half open, the moon sign starts glowing cream-yellow, the lanterns glow warm yellow, a few cream and light-blue star sparkles float out of the gap; 3) OPEN: both leaves fully open, the moon sign glowing bright cream-yellow with a soft light-blue rim, both lanterns glowing warm yellow with round warm light pools on the ground, a gentle band of light-blue and cream star sparkles in the doorway and a light path leading through it. Inviting, magical and safe — like the door into a bedtime story. No letters on the sign — only the moon picture.
```

### Negative prompt (all four prompts)

```text
text, letters, words, numbers, writing, captions, writing on signs, watermark, signature, logo, brand names, UI, HUD, scenery, landscape, horizon, sky, characters, people, animals, overlapping props, cropped props, perspective distortion, fisheye, eye-level view, electric street lamps, modern LED lights, neon, candles with open fire, fire, torches, pitch black, deep black shadows, horror, scary, spooky, halloween, creepy, jack-o-lanterns, skulls, menacing shapes, monsters, ghosts, glowing red eyes, fog, broken, rubbish, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Comic style matches `style_frame.png` / `kit_fences` (outlines, flat colours, one shadow tone).
- [ ] All props present, same scale, same camera angle, none cropped or overlapping.
- [ ] Glowing parts clearly separable (flat bright shapes → `*_glow` material slots).
- [ ] Night sheets: warm, cosy, readable, **nothing scary** (NIGHT-009); info board panel fully lit.
- [ ] Moon door: the three states read at a glance; inviting, not threatening; no text on the sign.
- [ ] Hand lantern reads as a child's toy-like lantern at ≈ 20 px.
- [ ] Buildable as low-poly props (≤ 500 tris, moon door ≤ 1 500).

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| 2026-09-27 | sheet_lights_night_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 2 + negative as 'Avoid' + extra text, ref: style_frame.png, sheet_v3.jpg, style_frame_night_v1.jpg | downscaled to 2048 px; **chosen** — deep blue background, warm glow and hard light pool under the lantern post, lit info board; issues: added two unrequested pieces (hedge corner, stone wall — from the night style frame), lantern post/hand lantern designs differ from the day sheet, soft bloom halos |
| 2026-09-27 | sheet_lights_night_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 2 + negative as 'Avoid' + extra text, ref: style_frame.png, sheet_v3.jpg, style_frame_night_v1.jpg | discarded (deleted) — grey daylight background, blue shadow pools look like puddles; does not show the night look |
| 2026-09-27 | sheet_lights_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, sheet_v3.jpg | downscaled to 2048 px; alternative — good lantern post; hand lantern is a kerosene lantern with a flame, fireflies drawn as bugs |
| 2026-09-27 | sheet_lights_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, sheet_v3.jpg | downscaled to 2048 px; **chosen** — all 7 items; toy-like chunky red/cream hand lantern with a big glowing belly (reads at small size), fireflies as glowing dots with tiny wings (fits the ≤ 4-tri particle), moon with craters + 4 stars |
| 2026-09-27 | sheet_moon_door_night_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 4 + negative as 'Avoid' + extra text, ref: style_frame.png, sheet_v3.jpg, style_frame_night_v1.jpg | downscaled to 2048 px; **chosen** — three states read at a glance, sign glows cream with the light-blue rim when open, warm light pools, sparkle band; issues: front-on camera (not 55°), grey background, unlit lanterns on the pillar tops plus lit ones on the pillar fronts (inconsistent) |
| 2026-09-27 | sheet_moon_door_night_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 4 + negative as 'Avoid' + extra text, ref: style_frame.png, sheet_v3.jpg, style_frame_night_v1.jpg | downscaled to 2048 px; alternative — 55° view, but odd extra wooden posts above the pillars and a different gate design |
| 2026-09-27 | sheet_moon_door_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 3 + negative as 'Avoid' + extra text, ref: style_frame.png, sheet_v3.jpg | downscaled to 2048 px; alternative — clear states but the sheet repeats all three states twice (two rows), lanterns hang on the pillar sides |
| 2026-09-27 | sheet_moon_door_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 3 + negative as 'Avoid' + extra text, ref: style_frame.png, sheet_v3.jpg | downscaled to 2048 px; **chosen** — one row of the three states, lanterns on the pillar tops, round moon sign on a beam, sparkles + star path in the open state; beam is straight (brief: curved) |
