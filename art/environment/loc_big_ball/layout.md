# Layout — `loc_big_ball`

Level: `level_2` (GAME-LEVEL-2, `assets/levels/level-2.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`ball_n` (50, 57, 2, 2), spot (53, 58), `enc_giraffe` (40, 44, 12, 12), `path_l2_ne` (52, 42, 3, 10), `hedge_l2_north`.

## Cameras

- `overview.png`: Camera yaw north (image top = north), pitch ≈ 62°, target (52, 57), about 16 m from the target; frame covers the ball on the lawn between the giraffe fence (bottom) and the hedge (top).
- `player_view.png`: Player at the top of `path_l2_ne` at (53, 51); camera yaw north (image top = north), pitch ≈ 55°, ≈ 14 m from the player.

## Modular props used (ART-ENVIRONMENT)

- `grass_tuft`
- `hedge`
- `fence_wood`
- `path_tile`

## Unique models

- `play_ball`

## Hiding places shown

- `loc_big_ball`
