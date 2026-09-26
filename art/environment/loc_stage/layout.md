# Layout — `loc_stage`

Level: `level_2` (GAME-LEVEL-2, `assets/levels/level-2.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`stage_ne` (58, 43, 4, 4), spot (57, 45), `path_l2_ne` (52, 42, 3, 10), `enc_elephant` (55, 28, 12, 13).

## Cameras

- `overview.png`: Camera yaw north (image top = north), pitch ≈ 62°, target (58, 45), about 16 m from the target; frame covers the stage, the side path on the left, the elephant fence below.
- `player_view.png`: Player on `path_l2_ne` at (53, 44); camera yaw north (image top = north), pitch ≈ 55°, ≈ 14 m from the player.

## Modular props used (ART-ENVIRONMENT)

- `path_tile`
- `grass_tuft`
- `fence_wood`
- `bench`

## Unique models

- `music_stage`

## Hiding places shown

- `loc_stage`
