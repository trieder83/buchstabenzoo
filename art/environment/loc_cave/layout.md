# Layout — `loc_cave`

Level: `level_1` (GAME-LEVEL-1, `assets/levels/level-1.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`rock_hill_w` (9, 0, 1, 8), `rock_hill_back` (10, 0, 3, 5), `rock_hill_e` (13, 0, 9, 8), `path_cave_floor` (10, 5, 3, 3), `loc_cave` (10, 5, 3, 3) with animal spot (10, 5), `path_cave` (8, 8, 14, 3), `barrier_east_repair` (22, 8, 2, 3); north of the path the hippo fence and `hedge_hippo_sw`

## Cameras

- `overview.png`: Camera yaw south (image top = south), pitch ≈ 62°, target (14, 5), about 25 m from the target (camera above the hippo enclosure); frame covers x 6…24, z 0…14.
- `player_view.png`: Player on `path_cave` in front of the mouth at (11, 9.5); camera yaw south (image top = south), pitch ≈ 55°, ≈ 14 m from the player (north of her, above the hippo enclosure).

## Modular props used (ART-ENVIRONMENT)

- `rock`
- `path_tile`
- `road_block`
- `repair_sign`
- `zookeeper_cart`
- `traffic_cone`
- `hedge`
- `fence_wood`
- `grass_tuft`
- `bush`

## Unique models

- `rock_hill_cave`

## Hiding places shown

- `loc_cave`
