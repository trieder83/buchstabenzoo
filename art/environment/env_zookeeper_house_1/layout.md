# Layout — `env_zookeeper_house_1`

Level: `level_1` (GAME-LEVEL-1 "Zookeeper house", `assets/levels/level-1.toml`). Grid: 1 m cells,
origin at the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`zookeeper_house_1` (-14, 0, 6, 5), interior (-13, 1, 4, 3), door (-9, 2); `path_house` (-8, 1, 3, 2);
`path_house_n` (-14, 5, 9, 2); `map_board` (-7, 3, 1, 2); `path_plaza` (-5, 0, 10, 8) east;
`enc_zebra` (-20, 7, 11, 12) north; `bamboo_sw` (-22, 0, 4, 3) west.
Items/props: bed `bed_l1` (-12.0, 1.5), night table (-12.72, 2.3), window (-13.0, 2.5) in the west
wall, rug (-11.0, 2.4), toy chest (-12.45, 3.72), desk + note (-10.5, 3.7), key box (-7.95, 1.5)
outside the east wall, wall lamp (-7.95, 3.4).

## Cameras

- `overview.png`: yaw north (image top = north), pitch ≈ 62°, target (-10, 3), ≈ 16 m; roof cut away.
- `player_view.png`: player on the rug (-10.5, 2.5); yaw north, pitch ≈ 55°, ≈ 14 m; roof faded out.
- `night_view.png`: as `overview.png`, night.

## Modular props used (ART-ENVIRONMENT)

- `path_tile`
- `map_board`
- `fence_wood`
- `bush`
- `grass_tuft`
- `bamboo`
- `bed`, `night_table`, `bedside_lamp`, `window_moon`, `rug_round`, `toy_chest` (`kit_bedroom`)
- `wall_lamp` (`kit_night`)

## Unique models

- `zookeeper_house` (level-1 size 6 × 5 m)
- `desk`, `note_math_fighter`, `key_box` (GAME-CART 16)

## Hiding places shown

- none (`loc_bamboo` only at the edge)
