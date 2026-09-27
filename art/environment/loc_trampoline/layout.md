# Layout — `loc_trampoline`

Level: `level_3` (GAME-LEVEL-3, `assets/levels/level-3.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`trampoline_w` (-19, 50, 3, 3), spot (-18, 51), `loc_trampoline` (-22, 50, 6, 4); `stream_l3` west/north-west (x −22…−20, from z 52), `tree_willow` (weeping willow, x −18…−16, z 56…58) north, `carousel_sw` (x −16…−13, z 52…55) north-east, `hedge_l3_south_a` south, `wall_l3_west` west. Moved here from the west lawn by FIX-056 (22 m haze rule, 2026-09-27).

## Cameras

- `overview.png`: Camera yaw north (image top = north), pitch ≈ 62°, target (-17, 53), about 16 m from the target; frame covers the trampoline, the end of the stream (left), the weeping willow and the carousel above, the south hedge below.
- `player_view.png`: Player on the lawn at (-14.5, 50.5), south of the carousel; camera yaw north (image top = north), pitch ≈ 55°, ≈ 14 m from the player.

## Modular props used (ART-ENVIRONMENT)

- `trampoline_ground`
- `grass_tuft`
- `water_tile_flowing`
- `hedge`
- `tree` (weeping willow `tree_willow`, carousel `carousel_sw` in the background)

## Hiding places shown

- `loc_trampoline`
