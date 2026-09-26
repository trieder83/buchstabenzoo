# Layout — `loc_ice_cream_kiosk`

Level: `level_3` (GAME-LEVEL-3, `assets/levels/level-3.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`ice_cream_kiosk` (15, 83, 4, 3), spot (17, 81), `path_l3_ne` (10, 77, 10, 3), `hedge_l3_east_b`.

## Cameras

- `overview.png`: Camera yaw north (image top = north), pitch ≈ 62°, target (17, 83), about 16 m from the target; frame covers kiosk, freezer chest, path below.
- `player_view.png`: Player on `path_l3_ne` at (15, 78); camera yaw north (image top = north), pitch ≈ 55°, ≈ 14 m from the player.

## Modular props used (ART-ENVIRONMENT)

- `path_tile`
- `grass_tuft`
- `bench`
- `hedge`

## Unique models

- `ice_cream_kiosk`
- `freezer_chest`

## Hiding places shown

- `loc_ice_cream_kiosk`
