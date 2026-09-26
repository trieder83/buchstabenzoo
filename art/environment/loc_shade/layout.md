# Layout — `loc_shade`

Level: `level_1` (GAME-LEVEL-1, `assets/levels/level-1.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`shade_w` (-22, 33, 2, 7) = `loc_shade` (-22, 33, 2, 7) with animal spot (−22, 36), wander area x −22–−21, z 33–39; `wall_west` (-24, 0, 2, 48) west; `trees_nw` (-20, 32, 8, 10) east; `mud_nw` / `loc_mud` (-20, 42, 7, 4) north; `pond_water` (-19, 20, 8, 8) south.

## Cameras

- `overview.png`: Camera yaw north (image top = north), pitch ≈ 62°, target (−20, 36), about 18 m from the target; frame covers the wall (left), the shaded strip, the tree group (right), the lawn south of the trees (bottom).
- `player_view.png`: Player on the grass at (−21.5, 31.5); camera yaw north (image top = north), pitch ≈ 55°, ≈ 14 m from the player; the shaded strip with the hippo fills the upper half.

## Modular props used (ART-ENVIRONMENT)

- `shade_decal`
- `tree`
- `zoo_wall`
- `grass_tuft`
- `hedge`

## Hiding places shown

- `loc_shade`
