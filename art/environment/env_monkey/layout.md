# Layout — `env_monkey`

Level: `level_3` (GAME-LEVEL-3, `assets/levels/level-3.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`enc_monkey` (-4, 82, 12, 10), gate (1, 82, 2, 1), `board_monkey` (-2, 80, 1, 1), `path_l3_ring_n` (-14, 77, 24, 3).

## Cameras

- `overview.png`: Camera yaw north (image top = north), pitch ≈ 62°, target (2, 86), about 22 m from the target; frame covers the enclosure and the ring path in front.
- `player_view.png`: Player next to the board on `path_l3_ring_n` at (-0.5, 78.5); camera yaw north (image top = north), pitch ≈ 55°, ≈ 14 m from the player.

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

- `monkey_climbing_frame`
- `monkey_house`
