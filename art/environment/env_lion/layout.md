# Layout — `env_lion`

Level: `level_2` (GAME-LEVEL-2, `assets/levels/level-2.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`enc_lion` (41, 14, 12, 8), gate (46, 21, 2, 1), `board_lion` (44, 23, 1, 1), `path_l2_ring_s` (36, 24, 18, 3).

## Cameras

- `overview.png`: Camera yaw south (image top = south), pitch ≈ 62°, target (46, 18), about 22 m from the target; frame covers the enclosure and the ring path above it.
- `player_view.png`: Player next to the board on `path_l2_ring_s` at (44.5, 24.5); camera yaw south (image top = south), pitch ≈ 55°, ≈ 14 m from the player.

## Modular props used (ART-ENVIRONMENT)

- `fence_wood`
- `gate_wood`
- `enclosure_sign`
- `info_board`
- `path_tile`
- `grass_tuft`
- `bush`
- `feeding_trough`
- `hedge`

## Unique models

- `lion_sun_deck`
