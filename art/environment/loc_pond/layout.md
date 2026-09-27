# Layout — `loc_pond`

Level: `level_1` (GAME-LEVEL-1, `assets/levels/level-1.toml`). Grid: 1 m cells, origin at
the entrance gate, +X east, +Z north; rects are `x, z, w, d`.

## Area

`pond_water` (-19, 20, 8, 8), `jetty_pond` (-11, 22, 3, 2), `bench_pond` (-11, 26, 2, 1), `loc_pond` (-19, 19, 4, 10) with animal spot (-16, 26) — the hippo swims in the west half of the pond, next to the north shore and `path_moon` (FIX-056, 22 m haze rule; was (-19, 19, 11, 10), spot (-13, 22) near the jetty); `path_ring_w` (-8, 11, 3, 16) east of it

## Cameras

- `overview.png`: Camera yaw west (image top = west), pitch ≈ 62°, target (-14, 24), about 25 m from the target; frame covers the pond, jetty, bench and the ring path.
- `player_view.png`: Player at the start of the jetty (-7.5, 22.5); camera yaw west (image top = west), pitch ≈ 55°, ≈ 14 m from the player; the pond fills the upper half.

## Modular props used (ART-ENVIRONMENT)

- `water_tile`
- `lily_pad`
- `frog`
- `reed`
- `jetty_wood`
- `bench`
- `rock`
- `grass_tuft`
- `bush`
- `tree`
- `path_tile`

## Hiding places shown

- `loc_pond`
