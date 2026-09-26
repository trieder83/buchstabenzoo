# Layout — `loc_carousel`

Level: `level_3` (GAME-LEVEL-3, `assets/levels/level-3.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`carousel_sw` (-16, 52, 4, 4), spot (-12, 54), `path_l3_south` (-9, 50, 3, 8), `stream_l3` (-22, 52, 3, 36).

## Cameras

- `overview.png`: Camera yaw north (image top = north), pitch ≈ 62°, target (-13, 54), about 16 m from the target; frame covers the carousel, the stream on the left, the side path on the right.
- `player_view.png`: Player on `path_l3_south` at (-8, 54); camera yaw north (image top = north), pitch ≈ 55°, ≈ 14 m from the player.

## Modular props used (ART-ENVIRONMENT)

- `path_tile`
- `grass_tuft`
- `water_tile_flowing`
- `hedge`
- `bench`

## Unique models

- `carousel`

## Hiding places shown

- `loc_carousel`
