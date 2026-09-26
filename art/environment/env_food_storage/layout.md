# Layout — `env_food_storage`

Level: `level_1` (GAME-LEVEL-1, `assets/levels/level-1.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`food_storage` (-4, 11, 8, 6), door cell (0, 11) on the south facade; `hedge_center_w` (-5, 11, 1, 6), `hedge_center_e` (4, 11, 1, 6); `path_ring_s` (-8, 8, 16, 3) in front; `grove_center` (-5, 17, 10, 10) behind

## Cameras

- `overview.png`: Camera yaw north, pitch ≈ 62°, target (0, 13), about 25 m from the target; frame covers x -10…10, z 6…22.
- `player_view.png`: Player just inside the door at (0.5, 12.5); camera yaw north, pitch ≈ 55°, ≈ 14 m from the player; barn roof cut away.

## Modular props used (ART-ENVIRONMENT)

- `food_box`
- `path_tile`
- `hedge`
- `tree`
- `bush`
- `grass_tuft`
- `flower_bed`

## Unique models

- `food_storage_building`

## Hiding places shown

- none
