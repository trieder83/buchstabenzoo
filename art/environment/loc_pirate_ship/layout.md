# Layout — `loc_pirate_ship`

Level: `level_3` (GAME-LEVEL-3, `assets/levels/level-3.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`pirate_ship` (14, 56, 7, 4), spot (13, 58), `bark_mulch_se` (11, 55, 3, 6), `path_l3_entry` (7, 52, 17, 3), `map_board_l3` (18, 55, 2, 1).

## Cameras

- `overview.png`: Camera yaw north (image top = north), pitch ≈ 62°, target (16, 57), about 18 m from the target; frame covers the ship on its bark mulch, the entry path below.
- `player_view.png`: Player on `path_l3_entry` at (12, 53); camera yaw north (image top = north), pitch ≈ 55°, ≈ 14 m from the player.

## Modular props used (ART-ENVIRONMENT)

- `bark_mulch_tile`
- `path_tile`
- `map_board`
- `hedge`
- `grass_tuft`

## Unique models

- `pirate_ship`

## Hiding places shown

- `loc_pirate_ship`
