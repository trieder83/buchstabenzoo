# Layout — `loc_meadow`

Level: `level_1` (GAME-LEVEL-1, `assets/levels/level-1.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`tall_grass_ne` (15, 31, 7, 3), `loc_meadow` (15, 31, 7, 3) with animal spot (17, 32), wander area x 15–20, z 31–33; `trees_ne` (15, 34, 6, 9) north of it; `path_ne_trail` (13, 31, 2, 12) west; `path_ne` (13, 28, 9, 3) south; `river_n` (10, 31, 3, 17) and `bridge_river` (10, 28, 3, 3) further west; `hedge_east_d` (22, 31, 2, 17) east.

## Cameras

- `overview.png`: Camera yaw north (image top = north), pitch ≈ 62°, target (16, 32), about 22 m from the target; frame covers the bridge (left), the trail, the tall grass, the front of `trees_ne` and the east hedge.
- `player_view.png`: Player on `path_ne` at (16.5, 29.5); camera yaw north (image top = north), pitch ≈ 55°, ≈ 14 m from the player; the tall grass fills the upper half.

## Modular props used (ART-ENVIRONMENT)

- `grass_tuft`
- `wildflowers`
- `butterfly`
- `tree`
- `path_tile`
- `water_tile_flowing`
- `bridge_wood`
- `hedge`

## Hiding places shown

- `loc_meadow`
