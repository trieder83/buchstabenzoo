# Layout — `env_snow_fox`

Level: `level_3` (GAME-LEVEL-3, `assets/levels/level-3.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`enc_snow_fox` (-5, 50, 10, 7), gate (-1, 56, 2, 1), `board_snow_fox` (-3, 57, 1, 1), `path_l3_ring_s` (-14, 58, 24, 3).

## Cameras

- `overview.png`: Camera yaw south (image top = south), pitch ≈ 62°, target (0, 53), about 20 m from the target; frame covers the enclosure and the ring path above it.
- `player_view.png`: Player next to the board on `path_l3_ring_s` at (-2.5, 58.5); camera yaw south (image top = south), pitch ≈ 55°, ≈ 14 m from the player.

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
- `tree`

## Unique models

- `snow_fox_den`
- `pine_tree`
