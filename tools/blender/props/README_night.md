# Night, bedroom, gates, buildings, landmarks, garden kits

Scripts in this folder built on `night_lib.py` (multi-node assets). Same flow as the other kits
(`tools/blender/README.md`): each script builds every asset of its kit from code, exports one
`.glb` per asset, saves the `.blend` and renders `art/props/<kit>/model_preview.png` from the
55° game camera — **day on top, night-tinted below** (moonlight ramp, `*_glow` slots emissive,
a hard-edged point light at every `light*` empty).

```bash
B="/snap/bin/blender -b --factory-startup --python"
$B tools/blender/props/kit_night.py        # lamps, moon door            -> assets/models/props/
$B tools/blender/props/kit_bedroom.py      # bedroom + desk, key box     -> assets/models/props/
$B tools/blender/props/kit_gates.py        # garden gate, glass door, door, turnstile -> props/
$B tools/blender/props/kit_buildings.py    # houses                      -> assets/models/buildings/
$B tools/blender/props/kit_landmarks.py    # night-1 landmarks           -> assets/models/props/
$B tools/blender/props/kit_garden.py       # vegetable garden            -> assets/models/props/
$B tools/blender/props/kit_night.py -- moon_door lantern_post   # only these; focused preview to
                                           # $PREVIEW_DIR (default: the kit's art folder, focus_*.png)
python3 tools/blender/check_glb.py         # export rules + expected sizes of every kit
```

## Conventions (in addition to tools/blender/README.md)

- **Axes / front** as in all kits: glTF = Blender (x, z, −y); north = world −Z; **front =
  south = glTF +Z at yaw 0**. A `[[light]]` / `[[prop]]` / `[[item]]` `facing` maps to yaw:
  `-z` → 0 (front towards level south), `+x` → +90°, `-x` → −90°, `+z` → 180°.
- **Origin on the ground** (min Y = 0, APIPE-004). Wall-mounted and table-top pieces
  (`wall_lamp`, `board_lamp`, `window_moon`, `key_box`, `bedside_lamp`, `note_paper`,
  `cart_key`, `hand_lantern`) have their origin at their own lowest point; the game lifts them
  to the **mount height** / socket below.
- **Material slots** (one mesh node may hold several primitives):
  - `palette` — the shared palette atlas, as in every kit.
  - `*_glow` — glowing parts. Base colour = palette atlas (UVs on the DAY colour cell, e.g.
    `lamp_glass` cream, `window_sky` light blue) and **`emissiveFactor` = the night glow
    colour**. The renderer draws them like `palette` by day and adds the emission (flat, no
    bloom) only in night mode (GAME-NIGHT §10). Slots used: `lamp_glow` #FFD66B,
    `bulb_glow` #FFD66B, `bulb_orange_glow` #FFB547, `bulb_cream_glow` #FFF1C9, `window_glow`
    #FFC857, `door_glow` #FFC857, `window_blue_glow` #8FB8FF, `window_red_glow` #E8735A,
    `moon_glow` #FFF4C9, `rim_glow` #8FB8FF.
  - `glass` — see-through panes: flat base colour `glass_tint` #CFE8F5, alpha 0.35,
    `alphaMode BLEND`, no texture (draw after the opaque pass, or skip).
  - `*_face` — blank text faces the game draws on: `note_face` (#FBF8EF), `sign_face`
    (#F3E6C8); flat colour, no texture, **UVs 0..1** across the face (u = left → right,
    v = bottom → top as the reader sees it).
- **Nodes.** The root node is named after the asset and has no transform. Moving / hideable
  parts are **child nodes with a translation only** = their pivot (hinge axis foot, hub,
  roof origin); rotation / scale are never exported. The node's mesh is stored relative to
  the pivot, so "open" = rotate the node about its own origin.
- **Empties** (nodes without a mesh): `light`, `light_*` = point-light position of a lamp;
  `socket_*` = attachment points. Radii are data (below), not in the file (no extras).
- The loader in `zoo-assets` keeps one mesh but tags every vertex with its part (direct
  child node of the root) and pivot, keeps the material names / emission / blend mode, the
  empties and the text faces (ARCH-006); the renderer moves / hides the parts by their node
  names (ARCH-007).

## Assets

`tris` = total of all nodes. Sizes x · y (height) · z in metres (`check_glb.EXPECTED_SIZES`).

### kit_night (props, ≤ 500; moon door ≤ 1 500 per its brief)

| Asset | Tris | Size | Nodes / slots / empties |
|---|---|---|---|
| `lantern_post` | 256 | 0.36 · 2.45 · 0.88 | `lamp_glow`; `light` (0, 1.74, 0.57) r 3.0 m. Arm and lantern point to +Z (= `facing`) |
| `string_lights` | 380 | 6.08 · 2.85 · 0.20 | post at x 0 + a 6 m cord (sag 0.42 m) to the hook point at x +6.0, 10 bulbs cycling `bulb_glow` / `bulb_orange_glow` / `bulb_cream_glow`; no point lights (emissive only) |
| `string_post` | 96 | 0.16 · 2.85 · 0.16 | the end post of a string-light line |
| `wall_lamp` | 216 | 0.30 · 0.77 · 0.47 | wall plane = glTF z 0, lamp towards +Z; `lamp_glow`; `light` (0, 0.17, 0.32) r 2.0 m. Mount: origin on the facade 1.6 m above the ground |
| `board_lamp` | 124 | 0.18 · 0.31 · 0.55 | brass clip lamp, shade 0.47 m in front of the clip; `lamp_glow` (shade inside); `light` (0, 0.12, 0.43) r 1.2 m |
| `hand_lantern` | 230 | 0.20 · 0.39 · 0.17 | `lamp_glow`; `light` (0, 0.135, 0) r 2.5 m (GAME-NIGHT uses 2.3 m for the player light); `socket_handle` (0, 0.385, 0) = grip point → attach so it coincides with the hand socket |
| `moon_door` | 1 430 | 3.54 · 4.60 · 1.04 | root: stone pillars (inner faces x ±1.0, 2 m door rect), beam, moon sign (`moon_glow` crescent, `rim_glow` ring, front and back), lanterns on the pillar caps (`lamp_glow`, `light_l` / `light_r` (∓1.35, 3.62, 0) r 2.5 m). Child nodes **`leaf_l`** pivot (−0.96, 0, −0.04), **`leaf_r`** pivot (0.96, 0, −0.04) |
| `moon_door_open` | 1 430 | 3.54 · 4.60 · 1.73 | the same with both leaves baked open 90° (for a renderer without node transforms) |

**Moon door opening:** leaves open to the BACK (−Z): `leaf_l` +90° about +Y, `leaf_r` −90°.
In level 1 (`moon_door` rect −24, 29, 2, 2 in the west wall) place it at the rect centre with
yaw **+90°** — front towards the day zoo (east), leaves swing west into the night zoo. Sign
and lanterns are dark by day, glow at night; the sparkle band in the open doorway is a
renderer effect (not modelled).

**Board-lamp sockets** (board_lamp origin in the board's model space, glTF; same yaw as the
board): `info_board` (0, 1.405, −0.236) — the clip sits on the roof ridge, the shade hangs
over the panel; `map_board` (0, 1.93, −0.09) — under the little roof.

### kit_bedroom (props, ≤ 500)

| Asset | Tris | Size | Notes |
|---|---|---|---|
| `bed` | 416 | 2.00 · 1.00 · 1.00 | long axis X, headboard +X at yaw 0, the used long side = front (+Z). `bed_l1` (`facing +z`) → yaw 180° → headboard west, used side north |
| `night_table` | 256 | 0.50 · 0.63 · 0.42 | drawer to the front; `socket_lamp` (0.08, 0.55, −0.08) for `bedside_lamp` |
| `bedside_lamp` | 122 | 0.25 · 0.36 · 0.26 | shade = `lamp_glow`; `light` (0, 0.27, 0) r 2.0 m (level `l1_indoor_bedside`) |
| `window_moon` | 318 | 1.26 · 1.04 · 0.17 | wall plane glTF z 0, room side +Z; panes `window_sky` (day). Child node **`night_sky`** (pivot 0): dark sky panel + moon + 2 stars (`moon_glow`) — **show it only at night**. Mount: origin 1.0 m above the floor on the inner wall face |
| `rug_round` | 260 | Ø 1.40 · 0.02 | concentric rings, tops 3 mm apart (no coplanar faces) |
| `toy_chest` | 406 | 0.82 · 0.85 · 0.54 | plush elephant; child node **`lid`** pivot (0, 0.42, 0.225) = back hinge, modelled OPEN 70° (rotate +70° about +X to close) |
| `desk` | 236 | 1.20 · 0.90 · 0.60 | `socket_note` (−0.10, 0.72, 0.05) for `note_paper` |
| `note_paper` | 10 | 0.24 · 0.003 · 0.32 | top face = **`note_face`** slot (UV 0..1, v towards −Z = away from a reader standing at +Z) — the game draws "Math Fighter" and the task |
| `key_box` | 332 | 0.38 · 0.47 · 0.20 | wall plane glTF z 0; 3-wheel lock (blank wheel faces `stripe_white`); child node **`door`** pivot (−0.17, 0, 0.12) = hinge, open = −100° about +Y; `socket_key` (0, 0.32, 0.07). Mount: origin 1.0 m above the ground on the facade (level x −8.0 for `key_box_l1`, yaw +90) |
| `cart_key` | 166 | 0.13 · 0.16 · 0.01 | green tag + brass key; `socket_ring` (0, 0.15, 0) → hang on `key_box.socket_key` |

### kit_gates (props, ≤ 500)

| Asset | Tris | Size | Nodes |
|---|---|---|---|
| `garden_gate` | 492 | 2.24 · 1.12 · 0.14 | root = two ball-topped posts (centres x ±1.05); **`leaf_l`** pivot (−0.99, 0, 0), **`leaf_r`** pivot (0.99, 0, 0), 0.98 m picket leaves; open to the back: `leaf_l` +90°, `leaf_r` −90° about +Y (turn the gate so its back faces into the garden) |
| `glass_door` | 284 | 1.95 · 2.20 · 0.14 | **`leaf_l`** pivot (−0.975, 0, −0.03), **`leaf_r`** pivot (0.975, 0, −0.03); frames `palette`, panes **`glass`**; open to the back like the garden gate. Place at the centre of an indoor-enclosure gate rect (2 m), yaw 0 (front = hall) |
| `door_wood` | 312 | 0.94 · 2.10 · 0.20 | ROOT = the leaf, origin = hinge axis foot, leaf along +X (x 0.01…0.95); round window `door_glow`; open = rotate the root about +Y (+90° swings it to the back / inside) |
| `turnstile` | 160 | 1.15 · 1.01 · 0.58 | one 1.2 m lane: pedestal at x −0.45 (root), **`arms`** node pivot (−0.33, 0, 0) (swing about +Y) |

**Door placements** (door_wood hinge in the building's model space, glTF, and level coords):

| Building | Hinge (glTF) | Level (x, z) | Yaw |
|---|---|---|---|
| `zookeeper_house` | (2.85, 0, 0.47) | (−8.15, 2.03) | +90° |
| `food_storage` | (0.03, 0, 2.90) | (0.03, 11.10) | 0° |
| `food_hut` | (2.40, 0, −0.03) | (−39.10, 29.03) | +90° |
| `night_house` | (−0.47, 0, 6.35) | (−36.97, 39.15) | 0° |

**Turnstiles** in `entrance_arch`: three units at x −1.2, 0, +1.2 of the arch origin, glTF z
−0.5 (level z −0.5, the placeholder row), yaw 0 — 8 cm between the units, no shared faces.

**`gate_wood` check (user request 2026-09-27):** the enclosure gate cells are 2 m; the fence
posts (0.18 m) stand on the cell edges, so the clear opening is 1.82 m. `gate_wood` is placed
at `gate.start + 0.09` (inner face of the hinge post) and spans local x 0.02…1.80 = 1.78 m,
leaving 2 cm at both posts — no gap, no overlap. Verified in the kit_gates preview (sample row:
`fence_wood | gate_wood | fence_wood`, closed and 60° open). No model change.

### kit_buildings (assets/models/buildings/, budget 4 000 per building in check_glb)

Origin = **centre of the level rect** (night house: of `model_rect`), yaw 0.

| Asset | Tris | Size | Level rect | Notes |
|---|---|---|---|---|
| `zookeeper_house` | 1 388 | 6.43 · 4.37 · 5.11 | (−14, 0, 6, 5) | walls 0.3 m, inner faces on the interior rect (x −2.0, y ±1.5 Blender), east facade on the rect edge; stone plinth ring with flowers on W / S / N (the 1 m wall band); boot bench + drawers in the solid strip beside the door; windows S / N / W (`window_glow`; the W pane is recessed 2 cm behind the `window_moon` prop); chimney |
| `food_storage` | 1 198 | 8.04 · 4.53 · 6.71 | (−4, 11, 8, 6) | red barn, gambrel roof (ridge N–S), south facade on the rect edge (the game's "Futter" board hangs 5 cm in front), round attic window + side windows `window_glow`, a few crates inside |
| `food_hut` | 1 218 | 5.80 · 3.62 · 6.68 | (−44, 26, 5, 6) night_1 | plank hut, gable roof N–S, east facade on the rect edge, lit serving hatch with a counter and red awning (east), windows S / W |
| `entrance_arch` | 692 | 6.16 · 5.00 · 1.76 | (−3, −2, 6, 2) | stone pillars 1.2 × 1.6 (inner faces x ±1.8), caps + posts, curved beam (top 5.0 m), blank board `sign_face` 2.6 × 0.8 m (bottom 3.55 m, front). ARCH-005: caps, posts, beam and board all have distinct extents — no coplanar faces |
| `night_house` | 3 196 | 17.60 · 4.90 · 13.60 | model_rect (−45, 39, 17, 13) | hall y −6.5…−1.5 (Blender), door S; three indoor enclosures behind the glass front (y −1.5), gates left open for `glass_door`; vaulted grass roof (ridge N–S) with the wooden ceiling underneath; painted moon + stars on the south facade above the name board; blue / red-orange portholes; planters in the solid wall band; ceiling lamps `light_hall` (−0.5, 3.65, 4.0) r 4 m warm, `light_hedgehog` (−5.5, 3.09, −2.0) r 3.5 red-orange, `light_bat` (0, 3.65, −2.0) and `light_owl` (5.5, 3.09, −2.0) r 3.5 blue (= the `indoor` [[light]] positions of night-1.toml) |

**Nodes of every building:** root = floor, walls up to 1.0 m, plinths, built-ins, dressing;
**`walls_upper`** (pivot y 1.0) = walls, gables, window frames and panes above 1 m;
**`roof`** (pivot y = eave height) = roof, chimney, ceiling lamps AND the **inner ceiling**
(down-facing faces in `ceiling_wood`; the ceiling is part of the `roof` node, not a sibling);
night house also **`glass`** (pivot (0, 0, 1.5)) = the glass panes. Zoo view with the player
inside (PLAY-028): hide `roof` and `walls_upper`. First person (CAMV-022): keep everything —
walls are closed boxes / prisms, so their inner faces exist; the ceiling is lit by the indoor
lights. Doors are separate `door_wood` models (table above).

Budget: buildings are the size of 6–17 m level rects, seen from every side and from inside
(first person), with interiors and ceilings; ~1.2–1.4 k tris for a small house and ~3.2 k for
the 17 × 13 m night house (≈ one animal) — still one draw per material slot.

### kit_landmarks (props, ≤ 500) — night_1

| Asset | Tris | Size | Level element | Notes |
|---|---|---|---|---|
| `windmill` | 480 | 2.28 · 3.89 · 1.85 | `windmill_n1` (−57, 8, 2, 2) | tower (root) + **`sails`** node, pivot = hub (0, 2.75, 0.8) on the front; rotate about the node's +Z axis |
| `hollow_tree` | 291 | 4.05 · 6.33 · 2.98 | `tree_hollow_n1` (−29, 19, 2, 2) | knot hole Ø 1.0 m at 2.5 m facing +Z; `socket_hole` (0, 2.3, 0.66) |
| `old_tree` | 289 | 4.91 · 6.36 · 3.57 | `tree_old_n1` (−70, 32, 2, 2) | moss at the foot |
| `crooked_tree` | 181 | 2.31 · 3.32 · 1.35 | `tree_crooked_n1` (−52, 10, 1, 1) | low branch towards +X; `socket_branch` (0.7, 1.76, 0) = bat hang point |
| `fir_tree` | 271 | 3.03 · 9.06 · 3.08 | `fir_n1` (−62, 48, 2, 2) | 5 tiers, cones |
| `rock_hill` | 115 | 2.94 · 1.95 · 2.93 | `hill_n1` (−69, 42, 3, 3) | grassy hill + big round stone; `socket_top` (−0.45, 1.12, 0.2) |
| `brush_pile` | 235 | 3.01 · 0.97 · 2.45 | `brush_pile_n1` (−70, 8, 3, 2) | twigs only, no leaves (riddle guard) |
| `mushroom_patch` | 381 | 1.90 · 0.37 · 1.71 | `mushroom_ring_n1` (−68, 31, 2, 2) | brown / cream mushrooms (never red with dots) |
| `flower_pots` | 225 | 0.86 · 0.49 · 0.53 | loose decoration | clay pots with white night flowers + stacked empty pots |
| `potting_bench` | 495 | 1.90 · 1.34 · 0.62 | `potting_bench_n1` (−70, 21, 1, 2) | long axis X: yaw +90 against the west hedge (front = east) |
| `telescope` | 148 | 0.92 · 1.60 · 0.98 | `telescope_n1` (−52, 38, 1, 1) | tube points up to the front-left |

`food_hut` is in kit_buildings. `tree_crooked` / `tree_old` / `tree_hollow` / `fir_tree` /
`hill` / `mushroom_ring` level kinds map to `crooked_tree` / `old_tree` / `hollow_tree` /
`fir_tree` / `rock_hill` / `mushroom_patch`.

### kit_garden (props, ≤ 500)

`garden_bed` 145 (0.82 · 0.25 · 2.90, long axis = glTF Z, soil top y 0.22);
`carrot_plant_{sprout,young,ripe}` 68 / 88 / 154 and `potato_plant_{sprout,young,ripe}`
58 / 98 / 158 (origin = plant base, place on the soil at y 0.22; stage `empty` = no model);
`carrot` 24, `potato` 90 (three), `basket` 264; `garden_fence` 190 / `garden_fence_1m`
(0.8 m pickets, posts at the piece ends like `fence_wood`); `wheelbarrow` 96 (wheel to +Z);
`watering_can` 134 (spout to +Z); `garden_sign` 52 (`sign_face` board, tilted back 10°).
`garden_gate` is in kit_gates.

## Palette cells added

64–127 (night lights, moon door, bedroom, buildings, landmarks, garden) and 202–204 (garden);
see `palette.toml`. No existing index changed.

## Open points

1. ~~Renderer support~~ — done 2026-09-27 (TECH-ARCH "Multi-node assets", ARCH-006/007/008):
   parts move / hide in the vertex shader, `*_glow` / `eye_glow` / `glass` / `*_face` slots,
   `light*` / `socket_*` empties; `moon_door_open` is no longer needed by the game.
2. **Food-storage "Futter" board vs. door:** done — board bottom 2.3 m (in the gable). The
   `food_hut` board follows the same rule and now stands in front of the roof slope like a
   roof sign (Q-151).
3. **String lights:** equal spans ≤ 6 m with the cord stretched per span, `string_post` at
   the far end (proposal Q-147).
4. **Moon door vs. zoo wall:** the door stands at the centre of its barrier rect
   (`zoo_core::scene::moon_door_pose`, front towards its day level).
5. `ART-PIPELINE` §1 now lists the kind `buildings`.
6. Entrance turnstiles stand 0.2 m nearer the plaza than the row above (glTF z −0.7) so the
   closed row sits on the walkable edge of the arch cells (LAYOUT-019).
