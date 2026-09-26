# Layout — `env_koala`

Level: `level_2` (GAME-LEVEL-2, `assets/levels/level-2.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`enc_koala` (27, 33, 8, 10), gate (34, 36, 1, 2), `board_koala` (35, 39, 1, 1), `path_l2_ring_w` (36, 27, 3, 12), `map_board_l2` (32, 31, 1, 2), `path_l2_entry` (24, 28, 12, 3).

## Cameras

- `overview.png`: Camera yaw west (image top = west), pitch ≈ 62°, target (31, 38), about 22 m from the target; frame covers the enclosure and the ring path in front.
- `player_view.png`: Player next to the board on `path_l2_ring_w` at (37.5, 39.5); camera yaw west (image top = west), pitch ≈ 55°, ≈ 14 m from the player.

## Modular props used (ART-ENVIRONMENT)

- `fence_wood`
- `gate_wood`
- `enclosure_sign`
- `info_board`
- `path_tile`
- `grass_tuft`
- `bush`
- `feeding_trough`
- `tree`
- `hedge`
- `map_board`

## Unique models

- `koala_shelter`
- `eucalyptus_tree`
