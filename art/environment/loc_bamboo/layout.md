# Layout — `loc_bamboo`

Level: `level_1` (GAME-LEVEL-1, `assets/levels/level-1.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`bamboo_sw` (-22, 0, 4, 3); `loc_bamboo` (-22, 0, 8, 6) with animal spot (−18, 2), wander area x −20–−15, z 0–5; `wall_west` (-24, 0, 2, 48) and `wall_south_w` (-24, -2, 21, 2) behind; `enc_zebra` (-20, 7, 11, 12) north; `map_board` (-7, 3, 1, 2) and `path_plaza` (-5, 0, 10, 8) east.

## Cameras

- `overview.png`: Camera yaw west (image top = west), pitch ≈ 62°, target (−17, 3), about 22 m from the target; frame covers the wall corner and thicket (top-left), the lawn, the zebra fence (right edge) and the map board (bottom).
- `player_view.png`: Player on the grass at (−12.5, 2.5); camera yaw west (image top = west), pitch ≈ 55°, ≈ 14 m from the player; the thicket fills the upper half.

## Modular props used (ART-ENVIRONMENT)

- `bamboo`
- `zoo_wall`
- `grass_tuft`
- `map_board`
- `fence_wood`
- `path_tile`

## Hiding places shown

- `loc_bamboo`
