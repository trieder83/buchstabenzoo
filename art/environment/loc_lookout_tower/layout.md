# Layout — `loc_lookout_tower`

Level: `level_2` (GAME-LEVEL-2, `assets/levels/level-2.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`tower_sw` (36, 15, 3, 3), spot (39, 17), `path_l2_sw` (33, 16, 3, 12), `enc_lion` (41, 14, 12, 8).

## Cameras

- `overview.png`: Camera yaw north (image top = north), pitch ≈ 62°, target (38, 17), about 18 m from the target; frame covers the tower with the side path on the left and the lion fence on the right.
- `player_view.png`: Player on `path_l2_sw` at (34, 18); camera yaw north (image top = north), pitch ≈ 55°, ≈ 14 m from the player.

## Modular props used (ART-ENVIRONMENT)

- `path_tile`
- `grass_tuft`
- `fence_wood`
- `zoo_wall`

## Unique models

- `lookout_tower`

## Hiding places shown

- `loc_lookout_tower`
