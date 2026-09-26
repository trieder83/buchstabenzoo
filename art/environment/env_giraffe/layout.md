# Layout — `env_giraffe`

Level: `level_2` (GAME-LEVEL-2, `assets/levels/level-2.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`enc_giraffe` (40, 44, 12, 12), gate (45, 44, 2, 1), `board_giraffe` (43, 42, 1, 1), `path_l2_ring_n` (36, 39, 18, 3).

## Cameras

- `overview.png`: Camera yaw north (image top = north), pitch ≈ 62°, target (46, 49), about 24 m from the target; frame covers the enclosure and the ring path in front.
- `player_view.png`: Player next to the board on `path_l2_ring_n` at (44.5, 40.5); camera yaw north (image top = north), pitch ≈ 55°, ≈ 14 m from the player.

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
- `tree`

## Unique models

- `giraffe_house`
- `giraffe_feeding_rack`
