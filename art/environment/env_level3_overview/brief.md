# Brief — `env_level3_overview`

Spec: ART-ENVIRONMENT, GAME-LEVEL-3. Status: **brief — images not generated yet**. Layout: `layout.md` in this
folder; level data `assets/levels/level-3.toml`.

## Purpose

Bird's-eye picture of the whole of level 3 (GAME-LEVEL-3) for the layout review and the orthographic `top_down.png` map art (GAME-MAP). Enclosures empty, hiding places empty.

## Must be visible

- The ring path around the zookeeper house, food storage 3 and dense trees; the entry path from the east edge (construction fence gone, ribbon and balloons) and the path from the south hedge gate.
- Monkey enclosure north, goldfish pond enclosure east, snow fox enclosure south, each with gate, sign and board facing the ring.
- West edge: the stream running south from a rock ledge with a waterfall at the north wall, a mill hut with a water wheel, a weeping willow; a ground trampoline and a washing line with white sheets on the lawn between stream and ring.
- Adventure playground with the pirate-ship climbing frame on bark mulch (south-east), a carousel (south-west), an ice cream kiosk with a freezer chest (north-east), a lawn with a sprinkler (north-west).
- Outer zoo wall west and north, tall hedges east and south.

## Must not appear

- No animals.
- No bridge, no ducks, no water lilies on the stream.
- No slide on the pirate ship, no skull flag.

## Props used

Modular (ART-ENVIRONMENT list): `path_tile`, `hedge`, `zoo_wall`, `fence_wood`, `gate_wood`, `enclosure_sign`, `info_board`, `map_board`, `tree`, `bush`, `grass_tuft`, `water_tile_flowing`, `bark_mulch_tile`, `trampoline_ground`, `sprinkler`, `washing_line`, `water_tap`, `reed`, `rock`.
Unique models: `zookeeper_house`, `food_storage_building`, `waterfall_ledge`, `mill_hut_wheel`, `willow`, `pirate_ship`, `carousel`, `ice_cream_kiosk`, `freezer_chest`, `pond_stone_rim`, `monkey_climbing_frame`, `snow_fox_den`.
Hiding places shown: `loc_pirate_ship`, `loc_carousel`, `loc_trampoline`, `loc_waterfall`, `loc_water_wheel`, `loc_willow`, `loc_ice_cream_kiosk`, `loc_sprinkler`, `loc_laundry`.

## Mood

Playful and adventurous: the newest part of the zoo with a playground, cool water and a little stream.

## Image settings

| File | Aspect / size | Camera (level coordinates, see `layout.md`) |
|---|---|---|
| `overview.png` | 16:9, 1920 × 1080 (SDXL: 1344 × 768) | Camera yaw north (image top = north), pitch ≈ 62°, target (0, 70), about 58 m from the target; frame covers the whole level from the west wall (x -24) to the east hedge (x 23). |
| `top_down.png` | 1:1, 2048 × 2048 | Orthographic, straight down, north up, 1 grid cell = 40 px; must match the ASCII map of the level spec exactly (map art for GAME-MAP). Made from the greybox render + style frame, not free generation. |

Camera, aspect ratios and consistency tips: as in `art/environment/loc_pond/brief.md` (high-angle
view like a cozy zoo park simulation game, Q-049; style frame as style reference; one fixed seed
per area; guided workflow from the level greybox, ART-PIPELINE §7).

## Prompts

**Style:** the first paragraph of every prompt is the STYLE block from `art/style/style.md`
(verbatim, comic style — Q-010 answered); every negative prompt ends with its NEGATIVE
suffix. Never edit those blocks here — change `art/style/style.md` and all briefs together.

### `overview.png`

```text
Comic-style 3D cartoon game art with a cel-shaded look: bold clean dark-brown outlines around every object, flat colour areas with one hard-edged shadow tone, simple rounded chunky shapes with friendly exaggerated proportions, big expressive eyes on people and animals, like a colourful children's comic book brought into 3D. No gradients, no fine texture detail, no noise. Bright warm midday sunlight from the upper left, crisp hard-edged shadows. Friendly saturated palette: fresh grass green, warm wood brown, light sand-beige paths, light stone grey, water blue, white. Clean, uncluttered, child-friendly, cheerful mobile game look, crisp focus across the whole image.

Elevated three-quarter top-down view like a cozy zoo park simulation game: a high camera looking down at about 60 to 65 degrees, isometric-like perspective with a narrow field of view so vertical lines stay nearly parallel, horizontal widescreen composition, the whole area in frame and neatly laid out like a diorama, the ground fills the image, no horizon, no sky.

A whole section of a small cartoon zoo seen from high above, about 48 m wide and 46 m deep, laid out like a board game. In the middle a square ring of sandy paths around a small wooden zookeeper house with a red roof, a wooden food storage building and a clump of round trees. At the top an empty monkey enclosure with a climbing frame of logs and ropes; on the right an empty enclosure with a round pond with a low stone rim and water plants; at the bottom an empty small enclosure with a wooden den under two pine trees. Along the whole left edge a narrow clear stream runs down from a waterfall that falls over a rock ledge in the top left corner, past a tiny mill hut with a turning wooden water wheel and a weeping willow whose branches hang into the water. Between stream and paths a round blue ground trampoline in the lawn and a washing line with big white sheets. In the lower right an adventure playground: a wooden pirate-ship climbing frame with a mast, a white sail and a black flag on soft brown bark mulch. In the lower left a small carousel with wooden horses under a striped roof. In the upper right a small ice cream kiosk with a striped awning and a white freezer chest. In the upper left a lawn with a garden sprinkler spraying arcs of water drops with a small rainbow. Stone zoo wall along the left and top edge, tall hedges on the right and bottom. Zoo world: light sand-beige paths of square paving blocks, wooden post-and-rail fences, tall dark-green hedges about 3 m high, round-topped trees, short green grass.

All signs and boards are blank: plain wooden or cream-coloured panels without any letters, words or numbers. The only markings allowed are a simple solid black animal silhouette on an enclosure sign and simple pictogram icons (shovel, padlock) on barrier signs. Enclosure signs and info boards are tilted back towards the camera so their faces are clearly visible from above.
```

### Negative prompt (all images of this area)

```text
text, letters, words, numbers, writing, captions, writing on signs, watermark, signature, logo, brand names, UI, HUD, buttons, photorealistic, realistic photo, photograph, realistic fur, hyper-detailed textures, dark, gloomy, night, fog, horror, scary, angry or menacing animals, sharp teeth, blood, gore, injury, dead animals, weapons, cages, cage bars, prison, rubbish, litter, crowds, clutter, distorted anatomy, extra legs, extra heads, fisheye distortion, tilted horizon, blurry, low resolution, jpeg artefacts, cropped main subject, sky, horizon, clouds, low camera angle, eye-level view, close-up, strong perspective distortion, animals, visitors, bridge, ducks, water lilies, slide, skull, pirate skull flag, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

Tools without a negative-prompt field: add "No text anywhere, no logos, not photorealistic, nothing scary." at the end of the prompt.

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Layout matches the level-3 ASCII map.
- [ ] Waterfall, water wheel and willow clearly different from each other; the stream has no bridge.
- [ ] Pirate ship without slide or skull.
- [ ] Style matches the approved style frame (Q-010).
- [ ] Layout matches `layout.md` and the GAME-LEVEL-3 map (positions, sizes, what is left/right).
- [ ] All signs, boards and labels blank — only animal silhouettes on enclosure signs; no text artefacts anywhere.
- [ ] Bright, friendly, nothing scary; no cages.
- [ ] High camera as specified (pitch, girl ≈ 1/12 of the image height near the centre, no sky); verticals nearly parallel.
- [ ] Readable from the high camera at phone size (view the image at ~25 %).
- [ ] Every prop visible is in the ART-ENVIRONMENT modular list or listed as a unique model in `layout.md`.

## Generation log

*(empty — no image generated yet)*
