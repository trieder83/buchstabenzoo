# Brief — `env_zookeeper_house_2` (level-2 zookeeper house with the bed)

Spec: GAME-LEVEL-2 "Bed of level 2", GAME-NIGHT rule 3, GAME-LAYOUT rule 16 (every bed indoors,
user request 2026-10-01), ART-ENVIRONMENT. Level data `assets/levels/level-2.toml`
(`zookeeper_house_2`, rect (27, 15, 6, 5), door (32, 17)). Style: `art/style/style.md` (comic).
Status: **brief — images not generated yet** (written 2026-10-01 by the level designer).

## Purpose

The small enterable zookeeper house in the south-west corner of level 2 where the child **sleeps**
after level 2's nightfall. It is the level-1 house (`env_zookeeper_house_1`, same model
`zookeeper_house`, same room) without the desk and the key box: reuse that brief's look.

## Must be visible

- `overview.png`: the south-west corner of level 2 from the zoo camera: the red-roofed 6 × 5 m
  house with the door on the **east** side opening onto the side path `path_l2_sw`, a wall lamp on
  the east wall north of the door, hedge to the west, the fountain basin (`fountain_sw`) and the lookout
  tower in the background; grass around the house.
- `player_view.png`: the roof cut away, the child standing at the bed: child-size bed with the
  blue star blanket along the south wall (headboard west), night table with bedside lamp, moon
  window in the west wall, round rug, toy chest. No desk, no key box.

## Must not appear

- No text, no animal, no fish bowl; nothing that looks like a hiding place (riddle guards: no
  tree house, no water jet, no tower platform on the house).

Props: `zookeeper_house` (closed + cut-away), `kit_bedroom` (`bed`, `night_table`, `window_moon`,
`rug_round`, `toy_chest`), `wall_lamp`. Mood: warm, cosy, safe, like the level-1 house.
