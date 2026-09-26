# Layout — `env_hippo`

Level: `level_1` (GAME-LEVEL-1, `assets/levels/level-1.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`enc_hippo` (9, 11, 11, 12) with gate (9, 15, 1, 2) on the west fence, `board_hippo` (8, 17, 1, 1), `hedge_hippo_nw` (8, 19, 1, 4), `hedge_hippo_sw` (8, 11, 1, 4), `path_ring_e` (5, 11, 3, 16); north of it `river_e` (10, 24, 14, 3)

## Cameras

- `overview.png`: Camera yaw east (image top = east), pitch ≈ 62°, target (14, 17), about 25 m from the target; frame covers the enclosure, the ring path and the river bend.
- `player_view.png`: Player on `path_ring_e` at (6.5, 16.5) next to the board; camera yaw east (image top = east), pitch ≈ 55°, ≈ 14 m from the player (west of her, above the grove).

## Modular props used (ART-ENVIRONMENT)

- `fence_wood`
- `enclosure_sign`
- `info_board`
- `hedge`
- `rock`
- `grass_tuft`
- `bush`
- `path_tile`
- `water_tile`
- `water_tile_flowing`

## Unique models

- `hut_wood`
- `pool_tiled`

## Hiding places shown

- none
