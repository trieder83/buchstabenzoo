# Layout — `env_elephant`

Level: `level_2` (GAME-LEVEL-2, `assets/levels/level-2.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`enc_elephant` (55, 28, 12, 13), gate (55, 33, 1, 2), `board_elephant` (54, 36, 1, 1), `elephant_pool` (59, 31, 7, 8), `path_l2_ring_e` (51, 27, 3, 12).

## Cameras

- `overview.png`: Camera yaw east (image top = east), pitch ≈ 62°, target (61, 34), about 26 m from the target; frame covers the enclosure with its pool and the ring path in front.
- `player_view.png`: Player next to the board on `path_l2_ring_e` at (52.5, 36.5); camera yaw east (image top = east), pitch ≈ 55°, ≈ 14 m from the player.

## Modular props used (ART-ENVIRONMENT)

- `fence_wood`
- `gate_wood`
- `enclosure_sign`
- `info_board`
- `path_tile`
- `grass_tuft`
- `bush`
- `feeding_trough`
- `rock`
- `hedge`

## Unique models

- `pool_tiled`
- `elephant_house`
- `hay_rack`
