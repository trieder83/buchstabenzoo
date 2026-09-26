# Layout — `env_entrance`

Level: `level_1` (GAME-LEVEL-1, `assets/levels/level-1.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`path_plaza` (-5, 0, 10, 8), `entrance_gate` (-3, -2, 6, 2), `map_board` (-7, 3, 1, 2), `bench_plaza` (6, 4, 2, 1); view reaches `path_ring_s` and the `food_storage` facade

## Cameras

- `overview.png`: Camera yaw north (image top = north), pitch ≈ 62°, target (0, 8), about 30 m from the target; frame covers x -14…16, z -3…20.
- `player_view.png`: Player at the spawn (0.5, 2.5); camera yaw north, pitch ≈ 55°, ≈ 14 m from the player (so ≈ 8 m south and 11.5 m above her, outside the gate).

## Modular props used (ART-ENVIRONMENT)

- `path_tile`
- `fence_wood`
- `hedge`
- `bench`
- `tree`
- `bush`
- `grass_tuft`
- `rock`
- `map_board`
- `flower_bed`
- `enclosure_sign`

## Unique models

- `entrance_arch`
- `food_storage_building`
- `rock_hill_cave`

## Hiding places shown

- none
