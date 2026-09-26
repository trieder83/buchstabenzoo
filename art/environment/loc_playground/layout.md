# Layout — `loc_playground`

Level: `level_2` (GAME-LEVEL-2, `assets/levels/level-2.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`playground_se_slide` (66, 15, 2, 3), `playground_se_swings` (69, 15, 4, 2), spot (68, 16), `path_l2_se` (54, 24, 16, 3).

## Cameras

- `overview.png`: Camera yaw north (image top = north), pitch ≈ 62°, target (68, 17), about 16 m from the target; frame covers slide, swings and the giraffe.
- `player_view.png`: Player at the east end of `path_l2_se` at (68, 23); camera yaw north (image top = north), pitch ≈ 55°, ≈ 14 m from the player.

## Modular props used (ART-ENVIRONMENT)

- `grass_tuft`
- `path_tile`
- `zoo_wall`
- `tree`

## Unique models

- `slide`
- `swings`

## Hiding places shown

- `loc_playground`
