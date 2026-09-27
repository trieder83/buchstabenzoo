# Layout — `env_night1_overview`

Level: `night_1` (GAME-LEVEL-NIGHT-1, `assets/levels/night-1.toml`). Grid: 1 m cells, origin at
the zoo entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

Whole level, bounds (-72, 6, 48, 48). Moon door (level 1) (-24, 29, 2, 2); `path_n1_entry`
(-30, 29, 6, 2); `path_n1_plaza` (-39, 24, 9, 11); `food_storage_n1` (-44, 26, 5, 6);
`night_house` (-45, 39, 17, 5) with `model_rect` (-45, 39, 17, 13); indoor enclosures
`enc_n1_hedgehog` (-45, 44, 6, 8), `enc_n1_bat` (-39, 44, 5, 8), `enc_n1_owl` (-34, 44, 6, 8);
boards (-42, 38), (-39, 38), (-33, 38); loop `path_n1_ring_n` (-64, 35, 26, 3), `path_n1_ring_w`
(-64, 13, 3, 22), `path_n1_ring_s` (-61, 13, 25, 3), `path_n1_s_link` (-39, 16, 3, 8);
`grove_n1_center` (-59, 18, 13, 15), `grove_n1_north` (-57, 41, 12, 11); `telescope_n1` (-52, 38).

## Cameras

- `overview.png`: yaw west (image top = west), pitch ≈ 62°, target (-48, 30), whole level in frame.
- `top_down.png`: orthographic, north up (greybox render of `night-1.toml`).

## Modular props used (ART-ENVIRONMENT)

- `hedge`, `path_tile`, `tree` (round), `bush`, `grass_tuft`, `bench`, `map_board`, `info_board`
- `water_tile` (still, night shader), `reed`, `jetty_wood`
- `lantern_post`, `string_lights`, `board_lamp`, `wall_lamp`, `firefly` (`kit_night`)

## Unique models

- `moon_door` (`kit_night`), `night_house` (`env_night_house`), `food_hut`
- `windmill`, `tree_crooked`, `tree_hollow`, `potting_bench` (+ clay pots, white flowers),
  `tree_old` (mossy) + `mushroom`, `hill` (grassy, 2 m) + big stone, `fir_tree`, `brush_pile`, `telescope`

## Hiding places shown

- `loc_brush_pile`
- `loc_flowerpots`
- `loc_mushrooms`
- `loc_windmill`
- `loc_fireflies`
- `loc_hollow_tree`
- `loc_moon_pond`
- `loc_hilltop`
- `loc_fir`
