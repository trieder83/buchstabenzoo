# Layout — `loc_sand`

Level: `level_1` (GAME-LEVEL-1, `assets/levels/level-1.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`sand_n` (-1, 42, 7, 4) = `loc_sand` (-1, 42, 7, 4) with animal spot (2, 44), wander area x −1–5, z 42–45; `enc_panda` (-6, 32, 12, 10) south (fence on z = 42 edge); `hedge_north_b` (-6, 46, 16, 2) north; `river_n` (10, 31, 3, 17) east; `path_north` (-9, 30, 3, 16) west.

## Cameras

- `overview.png`: Camera yaw north (image top = north), pitch ≈ 62°, target (2, 43), about 20 m from the target; frame covers the panda fence (bottom), the sand, the north hedge (top), `path_north` (left edge) and the river (right edge).
- `player_view.png`: Player on the grass at (−4.5, 43.5); camera yaw east (image top = east), pitch ≈ 55°, ≈ 14 m from the player; the sand patch fills the upper half.

## Modular props used (ART-ENVIRONMENT)

- `sand_tile`
- `rock`
- `grass_tuft`
- `fence_wood`
- `hedge`
- `water_tile_flowing`

## Hiding places shown

- `loc_sand`
