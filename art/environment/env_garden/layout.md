# Layout — `env_garden`

Level: `level_1` (GAME-LEVEL-1 "Vegetable garden", `assets/levels/level-1.toml`). Grid: 1 m
cells, origin at the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`garden_veg` (6, 36, 4, 10): gate opening x 7.0–9.0 on the line z = 36.0 (south);
`path_garden` (7, 36, 2, 9) up the middle; beds `bed_carrot_w` (6, 38, 1, 3),
`bed_carrot_e` (9, 38, 1, 3), `bed_potato_w` (6, 42, 1, 3), `bed_potato_e` (9, 42, 1, 3);
garden signs at (6.5, 37.45), (9.5, 37.45) (carrot) and (6.5, 41.45), (9.5, 41.45) (potato),
facing the path; wheelbarrow (7.2, 45.45), watering can (9.45, 45.4).
Plant spots: carrots (6.5 | 9.5, 38.5 / 39.5 / 40.5), potatoes (6.5 | 9.5, 42.75 / 44.25).
Neighbours: `enc_panda` (-6, 32, 12, 10) west of z 36–41 (its fence on x = 6.0); `sand_n`
(-1, 42, 7, 4) west of z 42–45 behind the garden fence; `river_n` (10, 31, 3, 17) east;
`hedge_north_b` (-6, 46, 16, 2) north; the grass of `loc_river` (6, 30, 4, 6) and the bridge
south of the gate.

```
       x: 6  7  8  9
  46      %  %  %  %
  45      w  w  _  a
  44      P  =  =  P
  43      P  =  =  P
  42      P  =  =  P
  41      !  =  =  !
  40      K  =  =  K
  39      K  =  =  K
  38      K  =  =  K
  37      !  =  =  !
  36      _  n  n  _
  35      .  .  .  .
```

## Cameras

- `overview.png`: camera yaw north (image top = north), pitch ≈ 62°, target (8, 40.5), about 18 m from the target; frame covers the grass north of the bridge (bottom), the garden, the north hedge (top), the sand and panda fence (left) and the river (right).
- `player_view.png`: player on `path_garden` at (7.5, 39.5) facing west towards `carrot_w2`; camera yaw north, pitch ≈ 55°, ≈ 14 m from the player; the whole garden in frame.

## Modular props used (ART-ENVIRONMENT)

- `path_tile`
- `fence_wood`
- `hedge`
- `water_tile_flowing`
- `sand_tile`
- `grass_tuft`
- `garden_fence`, `garden_gate`, `garden_bed`, `carrot_plant`, `potato_plant`, `garden_sign`, `wheelbarrow`, `watering_can` (new, GAME-GARDEN §9)

## Hiding places shown

- none (edges only: `loc_sand`, `loc_river`)
