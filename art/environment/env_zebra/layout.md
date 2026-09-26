# Layout — `env_zebra`

Level: `level_1` (GAME-LEVEL-1, `assets/levels/level-1.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`enc_zebra` (-20, 7, 11, 12) with gate (-10, 12, 1, 2) on the east fence, `board_zebra` (-9, 14, 1, 1), `path_ring_w` (-8, 11, 3, 16) in front, `hedge_center_w` / `grove_center` across the path

## Cameras

- `overview.png`: Camera yaw west (image top = west), pitch ≈ 62°, target (-14, 13), about 25 m from the target; frame covers the enclosure and the ring path in front.
- `player_view.png`: Player on `path_ring_w` at (-6.5, 14.5) next to the board; camera yaw west (image top = west), pitch ≈ 55°, ≈ 14 m from the player (east of her, above the ring).

## Modular props used (ART-ENVIRONMENT)

- `fence_wood`
- `enclosure_sign`
- `info_board`
- `bush`
- `grass_tuft`
- `tree`
- `hedge`
- `path_tile`
- `rock`

## Unique models

- `stone_arch_shelter`

## Hiding places shown

- none
