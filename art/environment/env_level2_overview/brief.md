# Brief — `env_level2_overview`

Spec: ART-ENVIRONMENT, GAME-LEVEL-2. Status: **brief — images not generated yet**. Layout: `layout.md` in this
folder; level data `assets/levels/level-2.toml`.

## Purpose

Bird's-eye picture of the whole of level 2 (GAME-LEVEL-2) for the review of the layout, and the source of the orthographic `top_down.png` used as the map art (GAME-MAP). Shows the level in the found-nothing state: all four enclosures empty, all landmarks of the 12 hiding places visible, no animals.

## Must be visible

- The ring path around food storage 2 and the dense central grove; the entry path from the west edge (where the fallen tree was) with the map board.
- Koala enclosure west, elephant enclosure east (with its tiled pool), giraffe enclosure north, lion enclosure south — each with gate, enclosure sign and info board facing the ring.
- In the corners: fountain and lookout tower (south-west), zoo train, slide and swings (south), giant tree (east of the elephants, clearly twice as tall as every other tree), music stage, the oak with the tree house at the east wall just east of the stage (FIX-056), deckchairs, pink blossom tree (north-east), giant red-and-white ball (north), log pile and flat rocks (north-west), construction fence with a small digger at the north end of the west edge.
- Outer zoo wall south and east, tall hedges west and north.

## Must not appear

- No animals (enclosures empty, hiding places empty).
- No sandpit, no rocks inside the lion enclosure, no fountain jet in the elephant pool.

## Props used

Modular (ART-ENVIRONMENT list): `path_tile`, `hedge`, `zoo_wall`, `fence_wood`, `gate_wood`, `enclosure_sign`, `info_board`, `map_board`, `tree`, `bush`, `grass_tuft`, `petal_decal`, `flat_rock_slab`, `construction_fence`, `digger`, `bench`.
Unique models: `food_storage_building`, `pool_tiled`, `treehouse_oak`, `tree_giant`, `tree_blossom`, `fountain_stone`, `log_pile`, `play_ball`, `lookout_tower`, `zoo_train`, `slide`, `swings`, `music_stage`, `deckchairs_sunshade`, `koala_shelter`, `elephant_house`, `giraffe_house`, `giraffe_feeding_rack`, `lion_sun_deck`.
Hiding places shown: `loc_treehouse`, `loc_tallest_tree`, `loc_blossom_tree`, `loc_fountain`, `loc_log_pile`, `loc_big_ball`, `loc_lookout_tower`, `loc_train`, `loc_playground`, `loc_sun_rocks`, `loc_stage`, `loc_deckchairs`.

## Mood

Bright summer morning, fresh and inviting: a bigger, more varied part of the zoo that the child has just unlocked.

## Image settings

| File | Aspect / size | Camera (level coordinates, see `layout.md`) |
|---|---|---|
| `overview.png` | 16:9, 1920 × 1080 (SDXL: 1344 × 768) | Camera yaw north (image top = north), pitch ≈ 62°, target (49, 37), about 60 m from the target; frame covers the whole level from the west hedge (x 24) to the east wall (x 75). |
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

A whole section of a small cartoon zoo seen from high above, about 50 m wide and 50 m deep, laid out like a board game. In the middle a square ring of sandy paths around a wooden food storage building and a dense clump of round trees. On the left side of the ring an empty koala enclosure with two grey-green eucalyptus trees, on the right an empty elephant enclosure with a square tiled pool, at the top an empty giraffe enclosure with a tall feeding rack, at the bottom an empty lion enclosure with a wooden sun deck. In the lower left corner a round stone fountain with a water jet and a wooden lookout tower; along the bottom a small colourful zoo train at a station, a slide and a double swing; on the right edge one giant tree twice as tall as all others; in the upper right a round wooden music stage with a pointed roof, right of it against the wall an old oak with a wooden tree house, three striped deckchairs under a sunshade and a tree covered in pink blossoms; at the top a giant red-and-white ball; in the upper left a neat pile of tree trunks, big flat grey rock slabs in the sun and a striped construction fence with a small yellow digger. Stone zoo wall along the bottom and right edge, tall hedges on the left and top. Zoo world: light sand-beige paths of square paving blocks, wooden post-and-rail fences, tall dark-green hedges about 3 m high, round-topped trees, short green grass.

All signs and boards are blank: plain wooden or cream-coloured panels without any letters, words or numbers. The only markings allowed are a simple solid black animal silhouette on an enclosure sign and simple pictogram icons (shovel, padlock) on barrier signs. Enclosure signs and info boards are tilted back towards the camera so their faces are clearly visible from above.
```

### Negative prompt (all images of this area)

```text
text, letters, words, numbers, writing, captions, writing on signs, watermark, signature, logo, brand names, UI, HUD, buttons, photorealistic, realistic photo, photograph, realistic fur, hyper-detailed textures, dark, gloomy, night, fog, horror, scary, angry or menacing animals, sharp teeth, blood, gore, injury, dead animals, weapons, cages, cage bars, prison, rubbish, litter, crowds, clutter, distorted anatomy, extra legs, extra heads, fisheye distortion, tilted horizon, blurry, low resolution, jpeg artefacts, cropped main subject, sky, horizon, clouds, low camera angle, eye-level view, close-up, strong perspective distortion, animals, visitors, sandpit, rocks inside the lion enclosure, voxels, cubes, blocky Minecraft style, pixel art, pixelated textures, photorealistic, realistic photo, realistic fur, hyper-detailed textures, soft painterly gradients, glossy plastic, anime, watercolour, sketchy lines, inconsistent line thickness
```

Tools without a negative-prompt field: add "No text anywhere, no logos, not photorealistic, nothing scary." at the end of the prompt.

## Review checklist (before `concept_approved = true` — user decides)

- [ ] Layout matches the level-2 ASCII map (positions, sizes, left/right).
- [ ] Every hiding-place landmark is recognisable from above without text.
- [ ] The giant tree is clearly the tallest tree.
- [ ] Style matches the approved style frame (Q-010).
- [ ] Layout matches `layout.md` and the GAME-LEVEL-2 map (positions, sizes, what is left/right).
- [ ] All signs, boards and labels blank — only animal silhouettes on enclosure signs; no text artefacts anywhere.
- [ ] Bright, friendly, nothing scary; no cages.
- [ ] High camera as specified (pitch, girl ≈ 1/12 of the image height near the centre, no sky); verticals nearly parallel.
- [ ] Readable from the high camera at phone size (view the image at ~25 %).
- [ ] Every prop visible is in the ART-ENVIRONMENT modular list or listed as a unique model in `layout.md`.

## Generation log

*(empty — no image generated yet)*
