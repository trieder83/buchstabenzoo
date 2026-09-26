# Layout — `env_panda`

Level: `level_1` (GAME-LEVEL-1, `assets/levels/level-1.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`enc_panda` (-6, 32, 12, 10) with gate (-1, 32, 2, 1) on the south fence, `board_panda` (-4, 30, 1, 1), `path_ring_n` (-8, 27, 16, 3), `path_north` (-9, 30, 3, 16) with `barrier_north_gate` (-9, 46, 3, 2); east: river and bridge (10–13, 28–48)

## Cameras

- `overview.png`: Camera yaw north, pitch ≈ 62°, target (1, 37), about 28 m from the target; frame covers x -12…14, z 26…48 (enclosure, side path with north gate, river and bridge).
- `player_view.png`: Player on `path_ring_n` at (-2, 28.5) next to the board; camera yaw north, pitch ≈ 55°, ≈ 14 m from the player (south of her, above the grove).

## Modular props used (ART-ENVIRONMENT)

- `fence_wood`
- `enclosure_sign`
- `info_board`
- `bamboo`
- `hedge`
- `gate_wood`
- `path_tile`
- `tree`
- `grass_tuft`
- `bush`
- `water_tile_flowing`
- `bridge_wood`

## Unique models

- `panda_platform`
- `panda_shelter`

## Hiding places shown

- none
