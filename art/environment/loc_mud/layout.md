# Layout — `loc_mud`

Level: `level_1` (GAME-LEVEL-1, `assets/levels/level-1.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`mud_nw` (-20, 42, 7, 4) = `loc_mud` (-20, 42, 7, 4) with animal spot (-17, 44), wander area x −20–−14, z 42–45; `trees_nw` (-20, 32, 8, 10) south; `hedge_north_a` (-22, 46, 13, 2) north; `wall_west` (-24, 0, 2, 48) west; `path_north` (-9, 30, 3, 16) and `barrier_north_gate` (-9, 46, 3, 2) east.

## Cameras

- `overview.png`: Camera yaw north (image top = north), pitch ≈ 62°, target (−16, 43), about 20 m from the target; frame covers the tree tops (bottom), the mud, the north hedge (top), the west wall (left) and `path_north` with the closed gate (right).
- `player_view.png`: Player on the grass at (−10.5, 43.5) next to `path_north`; camera yaw west (image top = west), pitch ≈ 55°, ≈ 14 m from the player; the mud puddle fills the upper half.

## Modular props used (ART-ENVIRONMENT)

- `mud_tile`
- `grass_tuft`
- `tree`
- `hedge`
- `gate_wood`
- `zoo_wall`

## Hiding places shown

- `loc_mud`
