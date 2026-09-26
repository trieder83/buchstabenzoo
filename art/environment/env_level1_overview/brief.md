# Brief — `env_level1_overview`

Spec: GAME-LEVEL-1. Status: **brief — images not generated yet**. Layout: `layout.md` in this
folder; level data `assets/levels/level-1.toml`.

## Purpose

One picture of the whole level exactly as laid out in GAME-LEVEL-1, to check proportions, distances and the "ring around a hidden middle" idea before building, and as the basis for the in-game map board art.

## Must be visible

- Layout exactly as the ASCII map (north = up = away from the viewer): entrance gate in the south wall (front centre), plaza, ring path around a central block (food storage in its south part, dense tree grove behind, tall hedges beside the storage).
- West of the ring: zebra enclosure (stone arch) and, north of it, the round lily pond with a jetty.
- North of the ring: panda enclosure (bamboo) and west of it a side path to a closed gate in the north hedge.
- North-east: the river coming from the north hedge, a wooden bridge next to the ring's north-east corner, the river bending east out under the hedge; path behind the bridge ending at a fallen tree.
- East of the ring: hippo enclosure (square pool, hut) framed by hedges at its gate.
- South-east: the grey rock hill with the cave mouth facing north onto a service path that ends at a "path under repair" barrier.
- Outer wall south and west, tall hedges north and east, tree groups in the north-west and north-east corners.
- Animals in the found state at their hiding places: zebras at the river bank by the bridge, hippo in the pond, panda in the cave mouth; enclosures empty.

## Must not appear

- No other enclosures, buildings or water than in the map.
- No text; signs blank / silhouettes only.

## Props used

Modular (ART-ENVIRONMENT list): `path_tile`, `fence_wood`, `hedge`, `tree`, `bush`, `grass_tuft`, `rock`, `water_tile`, `water_tile_flowing`, `bridge_wood`, `jetty_wood`, `lily_pad`, `duck`, `frog`, `reed`, `bamboo`, `bench`, `map_board`, `gate_wood`, `road_block`, `repair_sign`, `zookeeper_cart`, `traffic_cone`, `fallen_tree`, `enclosure_sign`, `info_board`, `food_box`, `flower_bed`.
Unique models (built once for this area): `entrance_arch`, `food_storage_building`, `stone_arch_shelter`, `hut_wood`, `pool_tiled`, `panda_platform`, `panda_shelter`, `rock_hill_cave`, `river_grate`.
Hiding places shown: `loc_river`, `loc_pond`, `loc_cave`.

## Mood

Compact, cheerful toy-like diorama; everything visible at once, sunny.

## Image settings

| File | Aspect / size | Camera (level coordinates, see `layout.md`) |
|---|---|---|
| `overview.png` | 16:9, 1920 × 1080 | Camera yaw north (image top = north), pitch ≈ 60°, target (0, 23), far enough (≈ 70 m, narrow FOV) that the whole bounds (-24…24, -2…48) fit a 16:9 frame. |
| `top_down.png` | 1:1, 1536 × 1536 (level is ~48 × 50 m) | orthographic camera straight down over (0, 23), north up, whole bounds in frame (basis for the map board art). |

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

A small compact zoo on a square piece of land, seen from high above as a neatly laid-out diorama (image top = north). Front centre: an entrance arch in a light stone outer wall that runs along the front and the left side; the back and right sides are closed by tall green hedges. Behind the entrance a small paved plaza, then a rectangular sandy ring path. Inside the ring: a wooden food storage barn at the front and a dense grove of tall trees behind it, with tall hedges beside the barn. Left of the ring: an empty zebra enclosure with a wooden fence and a grey stone arch, and behind it a round still pond with water lilies, a short jetty and a hippo showing only eyes and ears. Behind the ring: an empty panda enclosure with bamboo and a wooden platform; to its left a side path to a closed wooden gate in the back hedge. Right-back: a narrow river enters under the back hedge, flows towards the front under a small wooden bridge next to the ring's back-right corner, where three zebras drink and ducks swim, then bends right and leaves under the right hedge; a path behind the bridge ends at a fallen tree. Right of the ring: an empty hippo enclosure with a square tiled pool and a wooden hut. Front right: a hill of grey stone blocks with a dark cave opening facing away from the entrance, a sleeping panda inside, and a service path ending at a striped road block with a zookeeper cart. Tree groups in the back corners. The small girl stands on the plaza: a small girl about 7 years old (1.2 m tall, head about one third of her body height), long straight dark-brown hair down her back, white T-shirt with four horizontal blue stripes, brown belt, blue jeans, dark-brown shoes. Zoo world: light sand-beige paths of square paving blocks, wooden post-and-rail fences, tall dark-green hedges about 3 m high, round-topped trees, short green grass.

All signs and boards are blank: plain wooden or cream-coloured panels without any letters, words or numbers. The only markings allowed are a simple solid black animal silhouette on an enclosure sign and simple pictogram icons (shovel, padlock) on barrier signs. Enclosure signs and info boards are tilted back towards the camera so their faces are clearly visible from above.
```

### `top_down.png` (1:1)

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Orthographic top-down game map view looking straight down, north at the top, no perspective, the whole square zoo area filling the image. Same layout as described: entrance at the bottom centre, outer stone wall along the bottom and the left edge, tall hedges along the top and the right edge; rectangular sandy ring path in the middle around a barn (bottom part) and a dense tree grove (top part); zebra enclosure left of the ring with the round lily pond above it; panda enclosure above the ring with a side path to a closed gate at the top edge; a narrow river entering at the top right of centre, flowing down to a wooden bridge at the ring's top-right corner and bending right out through the right hedge, a fallen tree on the path beyond the bridge; hippo enclosure right of the ring; grey rock hill at the bottom right with its cave opening facing up, a service path above it ending at a road block at the right edge.

Zoo world: light sand-beige paths of square paving blocks, wooden post-and-rail fences, tall dark-green hedges about 3 m high, round-topped trees, short green grass.

All signs and boards are blank: plain wooden or cream-coloured panels without any letters, words or numbers. The only markings allowed are a simple solid black animal silhouette on an enclosure sign and simple pictogram icons (shovel, padlock) on barrier signs. Enclosure signs and info boards are tilted back towards the camera so their faces are clearly visible from above.
```

### Negative prompt (all images of this area)

```text
text, letters, words, numbers, writing, captions, writing on signs, watermark, signature, logo, brand names, UI, HUD, buttons, photorealistic, realistic photo, photograph, realistic fur, hyper-detailed textures, dark, gloomy, night, fog, horror, scary, angry or menacing animals, sharp teeth, blood, gore, injury, dead animals, weapons, cages, cage bars, prison, rubbish, litter, crowds, clutter, distorted anatomy, extra legs, extra heads, fisheye distortion, tilted horizon, blurry, low resolution, jpeg artefacts, cropped main subject, sky, horizon, clouds, low camera angle, eye-level view, close-up, strong perspective distortion, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

Tools without a negative-prompt field: add "No text anywhere, no logos, not photorealistic, nothing scary." at the end of the prompt.

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Positions and proportions match the ASCII map (ring ~17 × 22 m, enclosures ~11 × 12 m, pond ~8 m, river 3 m wide).
- [ ] Each hiding place is far from its own enclosure (never in the same player-camera view, GAME-LEVEL-1 Q-049 notes).
- [ ] River and pond clearly different at a glance.
- [ ] Level edge is closed everywhere (wall, hedge, river grate, three barriers).
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
