# Layout — `loc_treehouse`

Level: `level_2` (GAME-LEVEL-2, `assets/levels/level-2.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`treehouse_e` (71, 44, 3, 3), spot (70, 44), `loc_treehouse` (67, 42, 7, 6); `wall_l2_east` right behind it (east), `stage_ne` (58, 43, 4, 4) to the west, the north fence of `enc_elephant` (z 40) to the south, the end of `path_l2_ne_e` (x ≤ 66, z 49–51) to the north-west. Moved here from the south-west corner by FIX-056 (22 m haze rule, 2026-09-27).

## Cameras

- `overview.png`: Camera yaw north (image top = north), pitch ≈ 62°, target (68, 45), about 18 m from the target; frame covers the tree house at the east wall (right), the music stage (left), the elephant fence (bottom) and the end of the north path (top left).
- `player_view.png`: Player at the east end of `path_l2_ne_e` (65.5, 49.5); camera yaw north (image top = north), pitch ≈ 55°, ≈ 14 m from the player.

## Modular props used (ART-ENVIRONMENT)

- `tree`
- `grass_tuft`
- `path_tile`
- `zoo_wall`
- `fence`
- `bush`

## Unique models

- `treehouse_oak`

## Hiding places shown

- `loc_treehouse`
