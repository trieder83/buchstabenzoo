# Brief — `loc_pond`

Spec: ART-ENVIRONMENT, CONT-MISSIONS §2, GAME-LEVEL-1. Status: **in-review — first mockups generated 2026-09-26**. Layout: `layout.md` in this
folder; level data `assets/levels/level-1.toml`.

## Purpose

Where the hippo bathes (riddle: *"Das Wasser ist still. Dort blühen Seerosen … mit Fröschen … Nur die Augen und Ohren schauen heraus"*). Shown in the **found** state.

## Must be visible

- **Still water:** a round pond (about 8 × 8 m), mirror-flat, reflecting the sky and trees; no current, no rapids.
- **Water lilies:** many round lily pads with pink and white flowers.
- **Frogs:** 2–3 green frogs sitting on stones and lily pads at the shore.
- Reeds and cattails along the shore; a short wooden jetty from the path into the pond; a bench on the shore.
- The hippo in the water near the jetty tip — **only eyes, ears and nostrils above the surface** (klasse3 riddle).

## Must not appear

- **No bridge, no ducks, no flowing water** (those belong to the river).
- No fence around the pond (it is not an enclosure).

## Props used

Modular (ART-ENVIRONMENT list): `water_tile`, `lily_pad`, `frog`, `reed`, `jetty_wood`, `bench`, `rock`, `grass_tuft`, `bush`, `tree`, `path_tile`.
Hiding places shown: `loc_pond`.

## Mood

Quiet, peaceful, a bit dreamy: calm reflections, frogs, soft green colours.

## Image settings

| File | Aspect / size | Camera (level coordinates, see `layout.md`) |
|---|---|---|
| `overview.png` | 16:9, 1920 × 1080 (SDXL: 1344 × 768) | Camera yaw west (image top = west), pitch ≈ 62°, target (-14, 24), about 25 m from the target; frame covers the pond, jetty, bench and the ring path. |
| `player_view.png` | 9:16, 1080 × 1920 (SDXL: 768 × 1344) | Player at the start of the jetty (-7.5, 22.5); camera yaw west (image top = west), pitch ≈ 55°, ≈ 14 m from the player; the pond fills the upper half. |
| `player_view_landscape.png` *(optional)* | 16:9, 1920 × 1080 | same as player view |

**Camera (all images):** high-angle view like a cozy zoo park simulation game (Q-049).
`overview.png`: pitch ≈ 60–65°, the whole area in frame. `player_view.png`: the in-game
follow camera, pitch ≈ 55°, ≈ 14 m from the player, narrow FOV (≈ 30–35°) so verticals stay
nearly parallel; the girl is small (≈ 1/12 of the image height) near the centre. No sky,
no horizon. Signs and info boards are tilted back towards the camera.

Decision on aspect ratios: `overview.png` is **16:9 landscape**; `player_view.png` is
**9:16 portrait** because the reference phone viewport is portrait (1080 × 2340, ADIR-001 /
PLAT-002) — the final orientation is Q-048. An optional `player_view_landscape.png` (16:9)
uses the same scene with the landscape camera paragraph below.

**Consistency tips**
- Generate the **style frame** first (`art/environment/style_frame/`) and use the approved
  image as **style reference** (image prompt / style reference / IP-Adapter, medium
  strength) for every mockup.
- Keep the STYLE block and the girl description **word for word**; change only the SCENE.
- Use **one fixed seed per area** for overview and player view; note it in the log below.
- For final mockups use the guided workflow (ART-PIPELINE §7): greybox render from
  `assets/levels/level-1.toml` as depth/edge guide + style reference + this prompt.
- Generate 4 candidates and pick the one that matches the layout best, not the prettiest.

## Prompts

**Style:** the first paragraph of every prompt is the STYLE block from `art/style/style.md`
(verbatim, comic style — Q-010 answered); every negative prompt ends with its NEGATIVE
suffix. Never edit those blocks here — change `art/style/style.md` and all briefs together.

### `overview.png`

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Elevated three-quarter top-down view like a cozy zoo park simulation game: a high camera looking down at about 60 to 65 degrees, isometric-like perspective with a narrow field of view so vertical lines stay nearly parallel, horizontal widescreen composition, the whole area in frame and neatly laid out like a diorama, the ground fills the image, no horizon, no sky.

A round, perfectly still pond in a small zoo seen from high above, about 8 m across, its mirror-flat water reflecting the sky. Many round water-lily pads with pink and white flowers float on it; green frogs sit on stones and lily pads at the shore; reeds and cattails grow along the edge. A short wooden jetty leads from a sandy path at the bottom of the image into the pond, and a wooden bench stands on the grassy shore to the right. Near the end of the jetty a hippo lies in the water with only its eyes, ears and nostrils above the surface. the player character, a small girl about 7 years old (1.2 m tall, head about one third of her body height), long straight dark-brown hair down her back, white T-shirt with four horizontal blue stripes, brown belt, blue jeans, dark-brown shoes, walks onto the jetty. Trees around the pond, no bridge, no ducks. Zoo world: light sand-beige paths of square paving blocks, wooden post-and-rail fences, tall dark-green hedges about 3 m high, round-topped trees, short green grass.

All signs and boards are blank: plain wooden or cream-coloured panels without any letters, words or numbers. The only markings allowed are a simple solid black animal silhouette on an enclosure sign and simple pictogram icons (shovel, padlock) on barrier signs. Enclosure signs and info boards are tilted back towards the camera so their faces are clearly visible from above.
```

### `player_view.png` (9:16 portrait)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

In-game view of a cozy zoo park simulation game, vertical phone-screen composition: a high follow camera looking down at about 55 degrees from about 14 m away, isometric-like perspective with a narrow field of view so vertical lines stay nearly parallel, no horizon, no sky. The small girl is seen from above near the centre of the image, her figure about one twelfth of the image height; the area around her, about 12 m across, is clearly laid out and readable: paths, fences and enclosures easy to tell apart.

Seen from above: the player character, a small girl about 7 years old (1.2 m tall, head about one third of her body height), long straight dark-brown hair down her back, white T-shirt with four horizontal blue stripes, brown belt, blue jeans, dark-brown shoes, stands at the start of a short wooden jetty in the centre of the image. The upper half of the image is a round, perfectly still pond with mirror-flat water covered with round water-lily pads with pink and white flowers; green frogs sit on lily pads and a stone; reeds along the shore. Near the end of the jetty a hippo lies in the water with only its eyes, ears and nostrils above the surface, looking up. A sandy path runs across the bottom of the image; a wooden bench on the grassy shore to the right. Zoo world: light sand-beige paths of square paving blocks, wooden post-and-rail fences, tall dark-green hedges about 3 m high, round-topped trees, short green grass.

All signs and boards are blank: plain wooden or cream-coloured panels without any letters, words or numbers. The only markings allowed are a simple solid black animal silhouette on an enclosure sign and simple pictogram icons (shovel, padlock) on barrier signs. Enclosure signs and info boards are tilted back towards the camera so their faces are clearly visible from above.
```

### `player_view_landscape.png` (optional, 16:9)

Same as `player_view.png`, but replace the camera paragraph with:

```text
In-game view of a cozy zoo park simulation game, horizontal widescreen composition: a high follow camera looking down at about 55 degrees from about 16 m away, isometric-like perspective with a narrow field of view so vertical lines stay nearly parallel, no horizon, no sky. The small girl is seen from above near the centre of the image, her figure about one twelfth of the image height; the area around her, about 20 m across, is clearly laid out and readable.
```

### Negative prompt (all images of this area)

```text
text, letters, words, numbers, writing, captions, writing on signs, watermark, signature, logo, brand names, UI, HUD, buttons, photorealistic, realistic photo, photograph, realistic fur, hyper-detailed textures, dark, gloomy, night, fog, horror, scary, angry or menacing animals, sharp teeth, blood, gore, injury, dead animals, weapons, cages, cage bars, prison, rubbish, litter, crowds, clutter, distorted anatomy, extra legs, extra heads, fisheye distortion, tilted horizon, blurry, low resolution, jpeg artefacts, cropped main subject, sky, horizon, clouds, low camera angle, eye-level view, close-up, strong perspective distortion, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

Tools without a negative-prompt field: add "No text anywhere, no logos, not photorealistic, nothing scary." at the end of the prompt.

## Review checklist (before `concept_approved = true` — user decides)

- [ ] All riddle details readable from above: **still** water, **water lilies**, **frogs**; hippo shows only eyes and ears.
- [ ] Clearly different from `loc_river` and from the hippo's square tiled pool.
- [ ] No bridge, no ducks, no current.
- [ ] Style matches the approved style frame (Q-010).
- [ ] Layout matches `layout.md` and the level-1 map (positions, sizes, what is left/right).
- [ ] All signs, boards and labels blank — only animal silhouettes on enclosure signs; no text artefacts anywhere.
- [ ] Bright, friendly, nothing scary; no cages (enclosures have fences).
- [ ] High camera as specified (pitch, girl ≈ 1/12 of the image height near the centre, no sky); verticals nearly parallel.
- [ ] Readable from the high camera at phone size (view the image at ~25 %): paths, fences, gates, sign silhouettes and the key riddle details still clear.
- [ ] Every prop visible is in the ART-ENVIRONMENT modular list or listed as a unique model in `layout.md`.

## Generation log

| Date | File | Tool / model | Seed | Prompt changes | Result |
|---|---|---|---|---|---|
| | | | | | |
| 2026-09-26 | player_view_v1.jpg | gemini-3-pro-image (2K, 9:16) | — | prompt 2 + negative as 'Avoid' + extra text, ref: style_frame.png, sheet_v1.jpg | alternative — hippo enclosure sign and shovel sign at the jetty |
| 2026-09-26 | player_view_v2.jpg | gemini-3-pro-image (2K, 9:16) | — | prompt 2 + negative as 'Avoid' + extra text, ref: style_frame.png, sheet_v1.jpg | **chosen** → player_view.png — girl on the jetty, hippo in front, frogs and lilies |
| 2026-09-26 | overview_v1.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, sheet_v1.jpg | alternative — pond fenced like an enclosure with several signs |
| 2026-09-26 | overview_v2.jpg | gemini-3-pro-image (2K, 16:9) | — | prompt 1 + negative as 'Avoid' + extra text, ref: style_frame.png, sheet_v1.jpg | **chosen** → overview.png — round still pond, lilies, frogs, reeds, jetty, hippo eyes/ears, bench; one stray hippo sign on the shore |

**Mockup notes (2026-09-26):** generated with gemini-3-pro-image (2K), chosen image downscaled
to ≤ 2048 px and saved as PNG, the alternative kept as `*_v*.jpg`. References: the style frame
plus the approved kit/model sheets named in the log; `--extra`: "The first attached image is the approved style frame of this game: match its comic style, colours, line weight, outlines and high camera look exactly, but not its layout. The other attached image(s) are approved asset sheets of props that appear in this scene: draw those props exactly like on the sheets (shapes, colours, materials); do not copy the grey sheet background or the sheet layout. No text anywhere."
These are concept mockups from text only (no greybox guide yet) — positions are approximate.
**The level layout may still change:** the level designer is adding more hiding places
(GAME-LEVEL-1 `[[hiding_place]]`), so the mockups must be re-checked against
`specs/10-gameplay/levels/level-1.md` before approval.
