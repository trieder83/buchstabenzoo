# Layout — `loc_tallest_tree`

Level: `level_2` (GAME-LEVEL-2, `assets/levels/level-2.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`tree_giant_e` (70, 31, 2, 2), spot (69, 32), `path_l2_e` (67, 27, 3, 4), `enc_elephant` (55, 28, 12, 13), `wall_l2_east`.

## Cameras

- `overview.png`: Camera yaw north (image top = north), pitch ≈ 62°, target (69, 31), about 22 m from the target; frame covers the giant tree with the path below it and the elephant fence on the left.
- `player_view.png`: Player at the end of `path_l2_e` at (68, 28); camera yaw north (image top = north), pitch ≈ 55°, ≈ 14 m from the player.

## Modular props used (ART-ENVIRONMENT)

- `tree`
- `path_tile`
- `fence_wood`
- `zoo_wall`
- `grass_tuft`

## Unique models

- `tree_giant`

## Hiding places shown

- `loc_tallest_tree`
