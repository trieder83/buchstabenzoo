# Layout — `loc_leaves`

Level: `level_1` (GAME-LEVEL-1, `assets/levels/level-1.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`leaf_pile_ne` (15, 43, 4, 2); `loc_leaves` (14, 43, 7, 3) with animal spot (17, 44), wander area x 14–20, z 43–45; `trees_ne` (15, 34, 6, 9) south; `hedge_north_c` (13, 46, 9, 2) north; `path_ne_trail` (13, 31, 2, 12) south-west; `hedge_east_d` east.

## Cameras

- `overview.png`: Camera yaw south (image top = south), pitch ≈ 62°, target (17, 43), about 18 m from the target; frame covers the north hedge (bottom), the leaf pile, the tree group behind it (top) and the end of the trail.
- `player_view.png`: Player on the grass at (16.5, 45.5); camera yaw south (image top = south), pitch ≈ 55°, ≈ 14 m from the player; the leaf pile fills the upper half, the trees behind it.

## Modular props used (ART-ENVIRONMENT)

- `leaf_pile`
- `rake`
- `tree`
- `hedge`
- `path_tile`
- `grass_tuft`

## Hiding places shown

- `loc_leaves`
