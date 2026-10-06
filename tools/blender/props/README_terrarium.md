# Terrarium house models (`kit_terrarium.py`)

Real models for the terrarium house of `night_2` (concept: `art/environment/env_terrarium_house/overview_v2.jpg`
+ `cutaway_v2.jpg`, approved 2026-10-06; spec `AENV-017` / `AENV-018`, `specs/10-gameplay/levels/night-2.md`).
Source of truth is the script; built live through the Blender MCP and ported, then run headless:

```bash
/snap/bin/blender -b --factory-startup --python tools/blender/props/kit_terrarium.py   # all (+ house preview)
/snap/bin/blender -b --factory-startup --python tools/blender/props/kit_terrarium.py -- terrarium_tree  # one asset
python3 tools/blender/check_glb.py            # budgets + sizes (entries in check_glb.py)
```

Outputs: `assets/models/props/<asset>.glb`, `assets/blender/props/kit_terrarium.blend`,
`art/props/kit_terrarium/model_preview.png` (all props + the three assembled cases, day / night) and
`model_preview_house.png` (closed house + cut-away house with the cases in place).
New palette colours (`tools/blender/palette.toml`): `rock_warm`, `rock_warm_dark`, `rock_warm_light`; new glow slots
(`night_lib.GLOW`): `terrarium_amber_glow` #F2A93B, `terrarium_violet_glow` #9B6BE0, `terrarium_teal_glow` #5EE0C8.

## Frames (read first)

- **House frame** (`terrarium_house`): origin = middle of the south facade at ground level, glTF +Z (front) = the street
  = level south, x = level east, glTF -Z (Blender +Y) = level north. Level position of a house point `(hx, hy)` (hy
  northwards) = `(-84.5 + hx, 39 + hy)`. Footprint = `model_rect` (-94, 39, 19, 11). Place with **yaw 0** at
  level **(-84.5, 39.0)**, ground height 0.
- **Case props** are authored in the *viewer frame of a case*: glTF +Z (front) = the glass side, x = to the viewer's
  right, glTF -Z = into the case. The wiring turns every piece by the case yaw (snake **+90** = glass east, chameleon **0** =
  glass south, frog **-90** = glass west) plus its own yaw. Yaw convention as everywhere: `-z` facing = 0, `+x` = +90,
  `-x` = -90, `+z` = 180 (additive).
- **Floors** have their origin at the floor centre (ground level), top at height 0.08; every prop stands on that top
  (height 0.08). Lamps hang on the back wall (see below); wall pieces have the wall plane at glTF z = 0, stick out to +Z
  and have their origin at their own lowest point (the game lifts them to the mount height).
- `CASES` in the script holds the same placements as data (`level_placement(case, mx, my)` converts a case-frame point
  to level coordinates); the previews are built from it.

## Models

| Model | Tris | Size (w x h x d, m) | Nodes / empties / slots |
|---|---|---|---|
| `terrarium_house` | 1290 | 19.76 x 5.9 x 11.92 | child nodes `shell_upper` (upper walls 1..3.15 m, corner posts, windows, door lintel — **hide when the player is inside**), `roof` (dome roof incl. glowing skylight — **hide when inside**); root = 1 m wall base, hall floor (planks), door jambs + arch, emblem plaque (stays); empties `socket_door` (hinge centre of the door leaf, 0.5 m inside the south face), `socket_emblem` (front of the cream plaque, 4.0 m high; lay the animal pictogram decal here, ~1.9 m diameter), `light_hall`; slots `palette`, `window_glow` |
| `terrarium_front` | 34 | 1.0 x 2.4 x 0.17 | 1 m glass front module: wooden railing base 0.7 m, `glass` pane, top rail |
| `terrarium_frame` / `terrarium_frame_wide` | 56 | 4.4 / 7.4 x 2.62 x 0.28 | case frame (2 posts + lintel at 2.3 m + braces) around a 4 m / 7 m glass front |
| `terrarium_lamp` / `_uv` / `_teal` | 94 | 0.48 x 0.8 x 0.74 | wall lamp (back wall plane z = 0, shade towards +Z), glow slot `terrarium_amber_glow` / `_violet_glow` / `_teal_glow`, empty `light` (bulb) |
| `terrarium_dish` | 38 | 0.63 x 0.09 x 0.63 | water dish |
| `terrarium_floor_snake` | 259 | 4.0 x 0.13 x 4.62 | sand floor, darker dunes, 10 flagstones (stone floor) on one side, pebbles |
| `terrarium_rock_warm` | 94 | 1.92 x 1.16 x 1.31 | warm rock pile |
| `terrarium_rock_flat` | 77 | 1.85 x 0.37 x 1.21 | flat sun-warmed basking stone |
| `terrarium_branch_low` | 106 | 2.78 x 1.18 x 0.38 | thick arched branch the snake coils on |
| `terrarium_floor_frog` | 281 | 4.2 x 0.19 x 4.6 | sand bank, moss mounds, stone-rimmed pool; node **`water_pool`** (pool surface, can wobble / shimmer; pool centre (+0.6, +0.5), radii 1.25 x 0.95 in the floor frame) |
| `terrarium_waterfall` | 200 | 2.17 x 1.32 x 1.58 | three stacked stones + moss; node **`water_sheet`** (pivot = top lip at y 1.25, 3 flat-colour strips; animate by scaling Y 1.0..1.1 / small x shimmer, or UV scroll if the renderer adds it), foam at the foot; empty `socket_mist` |
| `mist_puff` | 45 | 0.85 x 0.34 x 0.45 | cream-white mist cloud; put it at `socket_mist`, scale / bob it |
| `terrarium_leaf_big` | 69 | 1.91 x 1.19 x 1.93 | big leaf plant (7 double-sided leaves) |
| `terrarium_moss_log` | 95 | 1.61 x 0.84 x 0.47 | mossy log lying along x, fern tuft |
| `terrarium_fern` | 36 | 0.72 x 0.57 x 0.81 | low fern |
| `terrarium_floor_chameleon` | 244 | 7.0 x 0.17 x 3.6 | earth floor, soil / sand patches, twigs, moss, stones |
| `terrarium_branch` | 171 | 2.6 x 2.15 x 1.39 | climbing branches (3 limbs + forks + leaves) on a stone; empties `socket_perch_1..3` |
| `terrarium_tree` | 230 | 2.92 x 2.24 x 1.14 | small leafy tree with three perch limbs; empties `socket_perch_1..3` |
| `info_board_wall` | 284 | 0.9 x 1.24 x 0.23 | wall-mounted framed text panel (`info_panel` face, tilted 15 deg) + round cream pictogram plate above it; empties `socket_pictogram` (plate face centre), `socket_lamp` |
| `fridge` | 104 | 0.7 x 1.15 x 0.68 | small cream fridge with a steel handle and a blue snowflake sticker (front = +Z) |

The 2 m `glass_door` (kit_gates) stays the gate; the glass fronts leave its gap free (below).

## Placement table (level coordinates: x east, z north; height in m above the ground; yaw in the game convention)

House and wall pieces

| Model | Where | Position | Yaw |
|---|---|---|---|
| `terrarium_house` | whole house | (-84.5, 39.0), h 0 | 0 |
| `info_board_wall` | `board_n2_snake` / `_chameleon` / `_frog` | cell centres (-89.5, 39.0) / (-86.5, 39.0) / (-80.5, 39.0), flush with the south wall (z = 39.0), mount height ~1.1 m (origin = lowest point) | 0 |
| emblem decal | over the door | at `socket_emblem` = (-84.5, 38.37), h 4.0, facing south | 0 |
| door leaf (`door_wood` / existing placeholder door) | door cell | at `socket_door` = (-84.5, 39.5) | 0 |
| `fridge` (food hut `food_storage_n2`) | back (west) wall inside the hut | (-90.6, 30.5), h 0 (suggestion; the frozen-insects box goes at its feet) | +90 |

Snake case `enc_n2_snake` (-94, 41, 5, 4): case yaw +90, floor centre **(-91.35, 43.0)**, glass line x = -89.0, back wall x = -93.7

| Model | Case-frame offset (right, into, height) | Level position | Yaw |
|---|---|---|---|
| `terrarium_floor_snake` | (0, 0, 0) | (-91.35, 43.00) | 90 |
| `terrarium_rock_warm` | (+1.00, +1.40, 0.08) | (-92.75, 44.00) | 110 |
| `terrarium_rock_flat` | (+0.70, -0.20, 0.08) | (-91.15, 43.70) | 100 |
| `terrarium_branch_low` | (+0.10, +0.55, 0.08) | (-91.90, 43.10) | 82 |
| `terrarium_dish` | (+1.40, -1.50, 0.08) | (-89.85, 44.40) | 90 |
| `terrarium_lamp` (amber) | (+0.60, +2.35, **1.95**) | (-93.70, 43.60) | 90 |
| `terrarium_front` x2 | (-1.5 / +1.5, -2.40, 0) | (-88.95, 41.50) / (-88.95, 44.50) | 90 |
| `terrarium_frame` | (0, -2.40, 0) | (-88.95, 43.00) | 90 |

Chameleon case `enc_n2_chameleon` (-88, 46, 7, 4): case yaw 0, floor centre **(-84.5, 47.85)**, glass line z = 46.0, back wall z = 49.7

| Model | Case-frame offset | Level position | Yaw |
|---|---|---|---|
| `terrarium_floor_chameleon` | (0, 0, 0) | (-84.50, 47.85) | 0 |
| `terrarium_branch` | (-1.80, +0.40, 0.08) | (-86.30, 48.25) | 0 |
| `terrarium_branch` (2nd) | (+0.40, +0.90, 0.08) | (-84.10, 48.75) | 180 |
| `terrarium_tree` | (+2.10, +0.30, 0.08) | (-82.40, 48.15) | 0 |
| `terrarium_leaf_big` | (-2.50, -0.40, 0.08) | (-87.00, 47.45) | 30 |
| `terrarium_leaf_big` | (+2.60, -0.60, 0.08) | (-81.90, 47.25) | -160 |
| `terrarium_lamp_uv` | (0, +1.85, **1.95**) | (-84.50, 49.70) | 0 |
| `terrarium_front` x5 | x = -3.0, -2.0, +1.0, +2.0, +3.0 (the gap -1.5..+0.5 = the 2 m gate) | (-87.5 / -86.5 / -83.5 / -82.5 / -81.5, 46.0) | 0 |
| `terrarium_frame_wide` | (0, -1.85, 0) | (-84.50, 46.00) | 0 |

Frog case `enc_n2_frog` (-80, 41, 5, 4): case yaw -90, floor centre **(-77.65, 43.0)**, glass line x = -80.0, back wall x = -75.3

| Model | Case-frame offset | Level position | Yaw |
|---|---|---|---|
| `terrarium_floor_frog` | (0, 0, 0) | (-77.65, 43.00) | -90 |
| `terrarium_waterfall` | (+0.60, +1.70, 0.08) | (-75.95, 42.40) | -90 |
| `mist_puff` | (+0.60, +0.95, 0.18) | (-76.70, 42.40) | -90 |
| `terrarium_leaf_big` | (-1.40, +1.60, 0.08) | (-76.05, 44.40) | -90 |
| `terrarium_leaf_big` | (+1.30, -1.20, 0.08) | (-78.85, 41.70) | 0 |
| `terrarium_moss_log` | (-1.00, -0.90, 0.08) | (-78.55, 44.00) | -102 |
| `terrarium_fern` | (-0.30, +1.90, 0.08) | (-75.75, 43.30) | -90 |
| `terrarium_lamp_teal` | (-0.80, +2.35, **1.95**) | (-75.30, 43.80) | -90 |
| `terrarium_front` x2 | (-1.5 / +1.5, -2.40, 0) | (-80.05, 44.50) / (-80.05, 41.50) | -90 |
| `terrarium_frame` | (0, -2.40, 0) | (-80.05, 43.00) | -90 |

Notes for the wiring

- **Wired 2026-10-06** (`crates/zoo-core/src/scene/terrarium.rs`, `round_house.rs`): the table above is used as is, except the chameleon's two leaves, moved from (-3.0, -0.8) to (-2.5, -0.4) , from (+3.1, -1.0) to (+2.6, -0.6) and the frog's second leaf from (+1.6, -1.55) to (+1.3, -1.2) because their 2.6 m leaves poked through the glass line into the hall (0.25 m) and 0.8 m into the side walls. Wall boards use `info_board_wall` at scale 1.6, mount 1.0 m (the pictogram plate was too small at scale 1.0 from the game camera); the board lamp sits at `socket_lamp` (y 1.98, 0.16 out). The fridge is `[[prop]] fridge_n2` in `night-2.toml`.

- The glass fronts in the table are 0.05 m off the exact edge (x = -89 / -80, z = 46) on purpose; shift them onto the
  edge if collision needs it (the old placeholder put its panes on the edge).
- Gates: snake / frog `gate` rects are 2 m (y 3..5 of the case) = the gap between the two 1 m panes; chameleon gate
  x -86..-84 is the gap between the 2 m and 3 m runs. The `glass_door` stays at the gate; `terrarium_frame` spans over it.
- Lamps: the `light` empty is the bulb; the case lights in `night-2.toml` (`n2_indoor_*`) can keep their colours:
  amber #F2A93B, violet #9B6BE0 (teal #5EE0C8 for the frog; the toml has a teal-ish light already).
- The house shell replaces the placeholder's wall mass, case walls, upper walls, dome roof and the floor; the glass
  fronts, frames, lamps, boards and the case dressing are separate placements. Collision / walkable cells are unchanged.
  `roof` and `shell_upper` should fade / hide like `roof_boxes` do today (PLAY-028).
- Day / night: window and skylight glow come from the `window_glow` slot; lamps from their own `*_glow` slots.
