# Layout — `loc_leaves`

Level: `level_1` (GAME-LEVEL-1, `assets/levels/level-1.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`leaf_pile_ne` (20, 43, 2, 3); `bench_leaves` (19, 43, 1, 1) with the rake; `loc_leaves` (18, 43, 4, 3) with animal spot (21, 45), wander area = the 9 cells x 20–21 z 43, x 19–21 z 44, x 18–21 z 45 (FIX-056, 22 m haze rule); `path_leaves_trail` (18, 34, 1, 9) through the trees; `trees_ne` (15, 34, 6, 9) south; `hedge_north_c` (13, 46, 9, 2) north; `path_ne_trail` (13, 31, 2, 12) south-west; `hedge_east_d` east.

## Cameras

- `overview.png`: Camera yaw south (image top = south), pitch ≈ 62°, target (19, 43), about 18 m from the target; frame covers the north hedge (bottom), the east hedge (left), the leaf pile with the bench, the tree group behind it (top) and the end of the trail.
- `player_view.png`: Player on the grass at (18.5, 45.5); camera yaw south (image top = south), pitch ≈ 55°, ≈ 14 m from the player; the leaf pile fills the upper half, the trees behind it.

## Modular props used (ART-ENVIRONMENT)

- `leaf_pile`
- `rake`
- `bench`
- `tree`
- `hedge`
- `path_tile`
- `grass_tuft`

## Hiding places shown

- `loc_leaves`
