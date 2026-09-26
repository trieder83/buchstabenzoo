# Layout — `env_level1_overview`

Level: `level_1` (GAME-LEVEL-1, `assets/levels/level-1.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

whole level: bounds (-24, -2, 48, 50) — see the ASCII map in `specs/10-gameplay/levels/level-1.md`

## Cameras

- `overview.png`: Camera yaw north (image top = north), pitch ≈ 60°, target (0, 23), far enough (≈ 70 m, narrow FOV) that the whole bounds (-24…24, -2…48) fit a 16:9 frame.
- `top_down.png`: orthographic camera straight down over (0, 23), north up, whole bounds in frame (basis for the map board art).

## Modular props used (ART-ENVIRONMENT)

- `path_tile`
- `fence_wood`
- `hedge`
- `tree`
- `bush`
- `grass_tuft`
- `rock`
- `water_tile`
- `water_tile_flowing`
- `bridge_wood`
- `jetty_wood`
- `lily_pad`
- `duck`
- `frog`
- `reed`
- `bamboo`
- `bench`
- `map_board`
- `gate_wood`
- `road_block`
- `repair_sign`
- `zookeeper_cart`
- `traffic_cone`
- `fallen_tree`
- `enclosure_sign`
- `info_board`
- `food_box`
- `flower_bed`

## Unique models

- `entrance_arch`
- `food_storage_building`
- `stone_arch_shelter`
- `hut_wood`
- `pool_tiled`
- `panda_platform`
- `panda_shelter`
- `rock_hill_cave`
- `river_grate`

## Hiding places shown

- `loc_river`
- `loc_pond`
- `loc_cave`
