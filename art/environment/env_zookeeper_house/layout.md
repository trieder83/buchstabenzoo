# Layout — `env_zookeeper_house`

Level: `level_3` (GAME-LEVEL-3, `assets/levels/level-3.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`zookeeper_house_3` (-11, 61, 7, 6), interior (-10, 62, 5, 4), door (-8, 61), `fish_bowl` at (-8.5, 64.5), `tap_l3` at (-6.5, 60.75), `food_storage_3` (-1, 61, 8, 6), `path_l3_ring_s` (-14, 58, 24, 3).

## Cameras

- `overview.png`: Camera yaw north (image top = north), pitch ≈ 62°, target (-4, 63), about 18 m from the target; frame covers the house (roof cut away) and the storage next door.
- `player_view.png`: Player inside the house next to the table, roof faded out at (-7.5, 63.5); camera yaw north (image top = north), pitch ≈ 55°, ≈ 14 m from the player.

## Modular props used (ART-ENVIRONMENT)

- `path_tile`
- `water_tap`
- `fish_bowl`
- `bench`
- `tree`
- `food_box`

## Unique models

- `zookeeper_house`
- `food_storage_building`
