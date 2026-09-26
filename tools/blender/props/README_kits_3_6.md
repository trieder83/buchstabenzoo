# Kits 3–6 — signs, nature, water, barriers (level-1 props)

Scripts in this folder, same flow as `kit_ground.py` / `kit_fences.py` (see
`tools/blender/README.md`): each builds every asset of its kit from code, exports one `.glb`
per asset to `assets/models/props/`, saves `assets/blender/props/<kit>.blend` and renders
`art/props/<kit>/model_preview.png` from the 55° game camera.

```bash
blender -b --factory-startup --python tools/blender/props/kit_signs.py      # + preview
blender -b --factory-startup --python tools/blender/props/kit_nature.py -- --no-preview
blender -b --factory-startup --python tools/blender/props/kit_water.py
blender -b --factory-startup --python tools/blender/props/kit_barriers.py
python3 tools/blender/check_props_3_6.py    # export rules + expected sizes of kits 3-6
```

`props_parts.py` holds the extra parts shared by these four kits (cylinders/cones, sticks,
lumpy icosphere blobs, X/Z prisms, flat polygons, blades, leaves, stars) and `run_kit()`
(build → report → export → save → preview). It only imports `lib/zoo_blender.py`.

## Axes and orientation

World axes as in GAME-LAYOUT "Coordinate spaces" (Q-056): +X east, +Y up, north = world −Z =
Blender +Y; level (x, z) → world (x, 0, −z). Models are never mirrored, only rotated about +Y
(+90° turns south → east, −90° turns south → west; a clockwise quarter turn is −90°).

**Front = south** (Q-061): at yaw 0 the readable side of every sign/board, the food box
label plates, the striped face of the road block, the cart side with the tools, the gate
straps/padlock, and the faces of the duck and frog point to level south (world +Z, Blender
−Y), towards the default camera.

## Assets

| Asset | Tris | Size x · h · z (m) | Origin / placement |
|---|---|---|---|
| `enclosure_sign` | 378 | 2.36 · 1.95 · 0.66 | centre between the two posts; panel tilted back 40° |
| `info_board` | 400 | 1.00 · 1.49 · 0.61 | post foot; panel 0.8 × 0.55, tilted back 40°, centre 1.08 m high |
| `map_board` | 364 | 2.26 · 2.43 · 0.60 | centre between the posts; panel 1.8 × 1.15, tilted back 15°, gable roof |
| `food_box` | 148 | 0.62 · 0.60 · 0.60 | centre of the footprint; label plate on the south face |
| `food_box_stack` | 444 | 1.29 · 1.20 · 0.73 | centre of the two bottom boxes; 3 label plates |
| `tree_round` | 380 | 3.34 · 4.96 · 2.98 | trunk centre |
| `tree_grove` | 366 | 2.80 · 5.52 · 2.65 | trunk centre (dense dark crown) |
| `tree_eucalyptus` | 376 | 3.20 · 7.01 · 1.83 | trunk centre (not used in level 1) |
| `bush` | 260 | 1.34 · 1.10 · 1.19 | centre |
| `flower_bed` | 350 | 2.00 · 0.49 · 1.01 | centre, long side along X |
| `rock` | 75 | 1.48 · 0.79 · 0.99 | centre of the big boulder (small stone to the east) |
| `bamboo` | 385 | 1.45 · 3.00 · 1.68 | centre of the clump |
| `reed` | 175 | 0.93 · 1.23 · 0.84 | centre (cattails) |
| `grass_tuft` | 34 | 0.21 · 0.34 · 0.22 | centre |
| `water_river_straight` | 2 | 1 · 0 · 1 | cell centre (tile) |
| `water_river_bank` | 21 | 1 · 0.15 · 1 | cell centre (tile) |
| `water_river_curve` | 51 | 1 · 0.15 · 1 | cell centre (tile) |
| `water_river_inner` | 22 | 1 · 0.05 · 1 | cell centre (tile) |
| `water_pond` | 2 | 1 · 0 · 1 | cell centre (tile) |
| `water_pond_edge` | 21 | 1 · 0.15 · 1 | cell centre (tile) |
| `water_pond_corner` | 51 | 1 · 0.15 · 1 | cell centre (tile) |
| `bridge_wood` | 404 | 3.40 · 1.27 · 2.61 | centre of the 3 × 3 bridge rect; deck along X; 4 piles in the water at x ±0.55, y ±1.18 (foam obstacles, Q-068) |
| `jetty_wood` | 236 | 3.80 · 0.70 · 1.80 | centre of the 3 × 2 jetty rect; water end = −X (0.8 m overhang) |
| `lily_pad` | 205 | 0.88 · 0.12 · 0.71 | waterline (place at y = 0) |
| `duck` | 254 | 0.31 · 0.34 · 0.52 | waterline, looks south |
| `frog` | 223 | 0.39 · 0.20 · 0.40 | waterline (sits on its own pad), looks south |
| `road_block` | 216 | 2.10 · 1.05 · 0.83 | centre; striped board along X |
| `repair_sign` | 119 | 0.73 · 1.32 · 0.21 | post foot; blue panel with a modelled shovel pictogram, tilted 15° |
| `zookeeper_cart` | 362 | 2.31 · 1.00 · 1.12 | centre of the bed; tow handle at +X |
| `traffic_cone` | 98 | 0.40 · 0.46 · 0.40 | centre of the base |
| `fallen_tree` | 323 | 2.42 · 1.58 · 4.26 | centre of the 2 × 3 rect; trunk along Y, roots south (y −1.75), crown north (y +2.65) |
| `gate_zoo_closed` | 408 | 3.12 · 2.60 · 0.72 | centre of the gate line; 3 m between the pillar centres ± 0.25 |

All ≤ 500 triangles, min Y = 0, one mesh, one material `palette`, no vertex colours
(`check_props_3_6.py`: 32/32 ok).

## Overlay panels (text, silhouettes, map drawn by the game)

Panels are blank and each kind has its own palette cell, so the renderer finds the face by
its UV (all texels of the cell are the same cream colour):

| Cell (index) | Asset | Panel face (world, yaw 0; printed by the script) |
|---|---|---|
| `sign_panel` (128) | enclosure_sign | centre (0.29, 1.36, 0.05), normal (0, 0.643, 0.766), ~1.30 × 0.72 m |
| `sign_slot` (129) | enclosure_sign | round slot, centre (−0.72, 1.40, 0.06), r 0.19 m, same normal |
| `info_panel` (130) | info_board | centre (0, 1.08, 0.07), normal (0, 0.643, 0.766), 0.68 × 0.43 m |
| `map_panel` (131) | map_board | centre (0, 1.29, 0.06), normal (0, 0.259, 0.966), 1.64 × 0.99 m |
| `label_plate` (132) | food_box(_stack) | centre (0, 0.32, 0.294), normal (0, 0, 1), 0.30 × 0.17 m |

The panel faces are quads (an 8-/12-gon for the slot) with all UVs on the cell centre; to
draw text the renderer needs its own UVs for the overlay quad (compute them from the face
corners, or draw a separate text quad 2 mm in front of the face).

## Water tiles (kit_water)

1 m tiles centred on the cell, autotiled from the 4-neighbour mask of water cells, exactly
like the path tiles (`tile_for()` in `kit_water.py`). A side without water gets a bank:
0.22 m grass strip at **y = 0.05** (the ground-tile top, so it meets grass/path tiles
flush) and a brown slope to the water surface at **y = 0**. Water sides of a tile are open
and flush with the neighbouring water tile.

| Tile (canonical) | Connections | Use |
|---|---|---|
| `water_river_straight` / `water_pond` | N E S W | inner cells |
| `water_river_bank` / `water_pond_edge` | N E S (bank W) | edge cells |
| `water_river_curve` / `water_pond_corner` | E S (banks N + W, round corner r 0.78) | outer corners |
| `water_river_inner` | N E S W, NE diagonal dry | inside of a river bend (small grass notch) |

Rotation: clockwise quarter turns k → yaw −90°·k. Count as water for the mask: bridge cells
over the river (water continues under `bridge_wood`) and cells beyond the level edge where
the river enters/leaves (under the north and east hedges). River and pond tiles differ only in the water cell (`water_river` 176, light blue, vs.
`water_pond` 179, dark): streaks, foam, rings and shimmer are drawn by the water shader
from a baked water field (TECH-WATER, Q-067), so tile rotation does not matter for the
look. The shape constants `M`, `SLOPE`, `R` are mirrored in `zoo_core::water` (visible
waterline 0.34 m inside a bank edge); WATER-003 checks them against the exported tiles.

## Level-1 mapping (`assets/levels/level-1.toml`)

Rect centre in level coordinates = (x + w/2, z + d/2) (cell i spans [i, i+1]); convert with
`zoo_core::coords::level_to_world`.

| toml element (`type`/`kind`) | Model(s) | Yaw (front) |
|---|---|---|
| `pond_water` (landmark/pond) | `water_pond*` tiles over 8 × 8 cells; `reed` on the bank, `lily_pad` ×3–5, `frog` ×2–3 | any |
| `jetty_pond` (path/jetty) | `jetty_wood` at the rect centre | 0 (water end −X = west, pond side) |
| `river_n`, `river_mid`, `river_e` (landmark/river) | `water_river_*` tiles (bridge cells too); `duck` ×3 near the bridge | streak axis along the flow |
| `bridge_river` (path/bridge) | `bridge_wood` at the rect centre | 0 (deck W–E over the N–S river) |
| `board_zebra` / `board_panda` / `board_hippo` (info_board) | `info_board` | proposal: face the ring path — zebra +90 (east), panda 0 (south), hippo −90 (west) |
| enclosure gates (`gate` of `enc_*`) | `enclosure_sign` beside the gate, outside the fence | same rule: zebra +90, panda 0, hippo −90 |
| `map_board` (landmark) | `map_board` | +90 (faces the plaza to the east) |
| `food_storage` | 10 × `food_box` / some `food_box_stack` inside (Q-047) | 0 |
| `grove_center` (tree_grove) | `tree_grove` every ~2.5 m + `bush` border | random |
| `trees_nw`, `trees_ne` (trees) | `tree_round` every ~3 m | random |
| `barrier_ne_tree` (fallen_tree) | `fallen_tree` at the rect centre | 0 |
| `barrier_north_gate` (closed_gate) | `gate_zoo_closed` at the rect centre | 0 |
| `barrier_east_repair` (road_block) | `road_block` at the rect centre, `repair_sign` in front (≈ yaw −35), `zookeeper_cart`, 2–3 `traffic_cone` (see the preview group) | road block −90 (faces west, across the path) |
| panda enclosure | `bamboo` clumps | random |
| zebra enclosure, lawns | `bush`, `grass_tuft`, `rock`, `flower_bed` | random |

## Palette cells added (appended to `palette.toml`)

128–132 overlay panels, 144–161 nature, 176–189 water and small animals, 192–201 barriers
(including `brass` for the padlock). No existing index was changed.

## Open points

1. **Walk height on bridge and jetty.** The bridge deck is arched (top y = 0.07 + 0.43·(1 −
   (x/1.7)²), up to 0.5 m); the jetty deck is at y = 0.2. `zoo-core` treats path cells as
   flat ground — the player would sink into the deck unless GAME-PLAYER gets a height
   sample for bridge/jetty cells (or the deck is flattened). Needs a spec decision.
2. **River flow animation.** The streaks are static geometry. Proposal: the renderer
   scrolls/animates faces of the `water_river_light` / `water_foam` cells (the flow
   direction per tile follows from its rotation; the canonical tile flows along N–S).
3. **Sizes adapted to the level-1 rects** (briefs said otherwise): `fallen_tree` 4.3 m (brief
   6 m; rect 2 × 3, reaches ~1.2 m into `hedge_east_d`), `gate_zoo_closed` 3.1 m (brief 4 m;
   gap between the hedges is 3 m), `map_board` 2.26 m (brief 2.5 m; rect is 2 cells, 0.13 m
   overhang per side), `bridge_wood` 3.4 m (brief 6 m; river is 3 cells), water tiles 1 m
   (brief 2 m; like kit_ground).
4. **Sign orientation** (face the ring path vs. face the default camera) is a level-design
   decision; the table above is a proposal.
5. **Not modelled:** river rapids with stones and foam (`river_n` notes — can be dressed with
   `rock` + foam for now), the river grate, pond reflection highlight (pond stays plain).
6. `check_glb.py` has no expected sizes for these assets, so running it over all models
   reports them as FAIL ("no expected size"); `check_props_3_6.py` has them (merge the
   `SIZES` table into `check_glb.EXPECTED_SIZES` when convenient).
7. Preview caveat: the Freestyle stand-in outlines thin blades (reed, grass tuft, bamboo
   leaves) heavily; judge them in the real renderer.
