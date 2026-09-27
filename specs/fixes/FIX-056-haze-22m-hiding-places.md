---
id: FIX-056
date: 2026-09-27
type: contradiction
specs: [GAME-LEVEL-1, GAME-LEVEL-2, GAME-LEVEL-3, GAME-LEVEL-NIGHT-1, GAME-LAYOUT, GAME-CAMERA-VIEWS, ART-ENVIRONMENT]
questions: [Q-145]
---

## Problem

The user raised the close-view view distance by 30 % (2026-09-27): the haze now starts at
11.7 m and fully hides from 20.8 m (was 9–16 m), and GAME-LAYOUT "Sight" now requires every
wander cell of a hiding place to be ≥ 22 m (planar, cell centres) from its own board's and
gate's standing points (walkable cells ≤ 2.5 m from the info board, cells around the gate incl.
its diagonal corners). The level data still followed the old 17 m rule: CAMV-008 failed for
`loc_trampoline` (308 violating samples), `loc_pond` (192), `loc_big_ball` (156), `loc_leaves`
(42), `loc_treehouse` (20), `loc_river` (18), `loc_lookout_tower` (5), `loc_stage` (2); below
22 m but still inside the fog test were `loc_fountain` (21.5 m), `loc_pirate_ship` (21.6 m) and
`loc_laundry` (21.95 m); in `night_1` (not covered by CAMV-008) `loc_hollow_tree` was 17.7 m.

## Resolution

Preference order: clip the wander area on the board side → move the spot inside the place →
move the board along its enclosure → move the whole place. Per place (old → new):

| Place | Change | Wander cells | Nearest standing point |
|---|---|---|---|
| `loc_river` (L1) | rect 6,30,4,6 → 6,33,4,3; spot (8,32) → (8,33) (bridge still ≤ 4 m, LAYOUT-L1-007) | 19 → 12 | 19.8 → 22.0 m |
| `loc_pond` (L1) | rect −19,19,11,10 → −19,19,4,10 (west half of the pond); spot (−13,22) → (−16,26) at the north shore (2 m from `path_moon`, LAYOUT-L1-009) | 22 → 14 | 18.1 → 22.6 m |
| `loc_leaves` (L1) | rect 14,43,7,3 → 18,43,4,3; spot (17,44) → (21,45); `leaf_pile_ne` 15,43,4,2 → 20,43,2,3; new `bench_leaves` (19,43,1,1, solid — removes the one cell < 22 m) and `path_leaves_trail` (18,34,1,9) through `trees_ne` (keeps `loc_leaves` → `loc_meadow` at 7.7 s; 10.9 s without) | 17 → 9 | 18.4 → 22.0 m |
| `loc_fountain` (L2) | rect 28,22,5,6 → 28,22,3,6 | 20 → 10 | 21.5 → 22.2 m |
| `loc_big_ball` (L2) | `board_elephant` (54,36) → (54,31) (south of its gate; board lamp moved with it); rect 51,55,6,5 → 51,57,6,3 | 20 → 14 | 17.0 → 22.0 m |
| `loc_lookout_tower` (L2) | rect 37,14,4,7 → 37,14,4,5 | 16 → 11 | 20.2 → 22.1 m |
| `loc_stage` (L2) | rect 55,42,5,7 → 55,45,5,4 | 19 → 12 | 20.6 → 22.4 m |
| `loc_treehouse` (L2) | whole place moved (only 4 cells of the south-west corner were ≥ 22 m from the koala gate): oak `treehouse_sw` 28,15,3,3 → `treehouse_e` 71,44,3,3; rect 26,14,4,5 → 67,42,7,6 (clear of the burglar hideout); spot (27,15) → (70,44) | 11 → 21 | 18.8 → 30.3 m |
| `loc_pirate_ship` (L3) | `wander_radius_m` 3.0 → 2.5 | 20 → 14 | 21.6 → 22.2 m |
| `loc_laundry` (L3) | rect −19,77,6,5 → −19,77,4,5 | 22 → 17 | 21.95 → 22.2 m |
| `loc_trampoline` (L3) | whole place moved (boxed in by stream, path and trees, 17–22 m from the monkey board): `trampoline_w` −18,63,3,3 → −19,50,3,3; rect −19,62,5,6 → −22,50,6,4; spot (−17,64) → (−18,51) | 26 → 15 | 17.0 → 28.7 m |
| `loc_hollow_tree` (N1) | whole place moved 3 m south-east: `tree_hollow_n1` −29,19,2,2 → −28,16,2,2; rect −33,16,7,4 → −31,12,5,4; spot (−30,19) → (−27,15) | 15 → 11 | 17.7 → 22.5 m |

Level minima now: `level_1` 22.0 m, `level_2` 22.0 m, `level_3` 22.2 m, `night_1` 22.1 m;
CAMV-008 nearest close-view eye distance 22.01 m. Riddles are unchanged (no place changed its
features). Spread: level 1 all 27 combinations (minimum 13.6 m), level 2 57 of 81 (was 48),
level 3 12 of 27 (was 10), `night_1` all 27 (minimum 12.1 m). Walking neighbours changed
where a moved place lost its old neighbour: level 1 zebra board → panda board (pond → panda
board 7.2 s), level 2 `loc_treehouse` → `loc_blossom_tree` 7.6 s, level 3 `loc_trampoline` →
`loc_carousel` 5.1 s, `night_1` `loc_moon_pond` → food boxes 7.2 s. The design choices are
proposals (Q-145).

Tests: `night_1` is now covered by the 22 m rule — LAYOUT-N1-006 checks every wander cell
≥ 22 m (was 17 m) from the standing points; CAMV-008's text states the 20.8 m fog end.
Test fixtures updated to the new spec tables: `layout.rs` (L1-005 pairs and numbers, L1-014
cell counts), `levels23.rs` (L2/L3-005 pairs and numbers, valid combinations 57 / 12),
`night1_layout.rs` (N1-005 pairs, N1-006 22 m, N1-007 count), `wander.rs` (the Q-097 hippo test
now stands on the pond's north shore instead of the jetty). The scene's leaf-pile heaps are
placed by a hash of the rect origin; the rect 20,43,2,3 was chosen so no two heaps z-fight
(ARCH-005).

## Changed files

- `assets/levels/level-1.toml`, `assets/levels/level-2.toml`, `assets/levels/level-3.toml`,
  `assets/levels/night-1.toml`
- `specs/10-gameplay/levels/level-1.md`, `level-2.md`, `level-3.md`, `night-1.md`,
  `specs/10-gameplay/layout.md`, `specs/10-gameplay/camera-views.md`,
  `specs/30-art/environment.md`, `specs/open-questions.md`
- `art/environment/loc_treehouse/`, `loc_trampoline/`, `loc_leaves/`, `loc_pond/`,
  `loc_river/` (brief/layout), `env_elephant/layout.md`, `env_level2_overview/brief.md`,
  `env_level3_overview/brief.md`
- `crates/zoo-core/tests/layout.rs`, `levels23.rs`, `night1_layout.rs`, `wander.rs` (fixtures
  only)
