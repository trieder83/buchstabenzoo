# Layout — `loc_train`

Level: `level_2` (GAME-LEVEL-2, `assets/levels/level-2.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`train_se` (54, 14, 8, 2), spot (57, 16), `path_l2_station` (54, 17, 2, 7), `path_l2_se` (54, 24, 16, 3), `wall_l2_south`.

## Cameras

- `overview.png`: Camera yaw north (image top = north), pitch ≈ 62°, target (57, 17), about 18 m from the target; frame covers the train at the station, the platform path on the left.
- `player_view.png`: Player on `path_l2_station` at (55, 21); camera yaw north (image top = north), pitch ≈ 55°, ≈ 14 m from the player.

## Modular props used (ART-ENVIRONMENT)

- `path_tile`
- `zoo_wall`
- `grass_tuft`
- `bench`

## Unique models

- `zoo_train`

## Hiding places shown

- `loc_train`
