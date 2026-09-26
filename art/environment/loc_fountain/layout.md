# Layout — `loc_fountain`

Level: `level_2` (GAME-LEVEL-2, `assets/levels/level-2.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`fountain_sw` (27, 24, 3, 3), spot (30, 25), `path_l2_entry` (24, 28, 12, 3), `path_l2_sw` (33, 16, 3, 12), `hedge_l2_w_a`.

## Cameras

- `overview.png`: Camera yaw north (image top = north), pitch ≈ 62°, target (30, 25), about 16 m from the target; frame covers the fountain between the entry path (top) and the side path (right).
- `player_view.png`: Player on `path_l2_sw` at (34, 24); camera yaw north (image top = north), pitch ≈ 55°, ≈ 14 m from the player.

## Modular props used (ART-ENVIRONMENT)

- `water_tile`
- `grass_tuft`
- `path_tile`
- `hedge`
- `bench`

## Unique models

- `fountain_stone`

## Hiding places shown

- `loc_fountain`
