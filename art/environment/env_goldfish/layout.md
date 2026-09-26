# Layout — `env_goldfish`

Level: `level_3` (GAME-LEVEL-3, `assets/levels/level-3.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`enc_goldfish` (11, 63, 11, 10), gate step (11, 67, 1, 2), `board_goldfish` (10, 70, 1, 1), `goldfish_pond` (13, 64, 8, 8), `path_l3_ring_e` (7, 61, 3, 16).

## Cameras

- `overview.png`: Camera yaw east (image top = east), pitch ≈ 62°, target (16, 68), about 20 m from the target; frame covers the pond enclosure and the ring path in front.
- `player_view.png`: Player next to the board on `path_l3_ring_e` at (8.5, 70.5); camera yaw east (image top = east), pitch ≈ 55°, ≈ 14 m from the player.

## Modular props used (ART-ENVIRONMENT)

- `fence_wood`
- `enclosure_sign`
- `info_board`
- `path_tile`
- `grass_tuft`
- `reed`
- `rock`
- `fish_bowl`

## Unique models

- `pond_stone_rim`
