# Layout — `loc_river`

Level: `level_1` (GAME-LEVEL-1, `assets/levels/level-1.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`river_n` (10, 31, 3, 17), `bridge_river` (10, 28, 3, 3), `river_mid` (10, 27, 3, 1), `river_e` (10, 24, 14, 3), `path_bridge_w` (8, 28, 2, 3), `path_ne` (13, 28, 9, 3), `barrier_ne_tree` (22, 28, 2, 3), `loc_river` (6, 33, 4, 3) with animal spot (8, 33) (FIX-056; was (6, 31, 4, 4) / (8, 32) in this mockup), `trees_ne` (15, 34, 6, 9)

## Cameras

- `overview.png`: Camera yaw north, pitch ≈ 62°, target (13, 32), about 28 m from the target; frame covers x 2…24, z 22…44.
- `player_view.png`: Player at the ring's north-east corner (6.5, 28.5); camera yaw north, pitch ≈ 55°, ≈ 14 m from the player; the bridge and the zebras (8, 33) are in the upper right of the frame.

## Modular props used (ART-ENVIRONMENT)

- `water_tile_flowing`
- `bridge_wood`
- `duck`
- `rock`
- `reed`
- `grass_tuft`
- `bush`
- `tree`
- `hedge`
- `path_tile`
- `fallen_tree`

## Unique models

- `river_grate`

## Hiding places shown

- `loc_river`
