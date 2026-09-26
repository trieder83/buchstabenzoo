---
id: FIX-020
date: 2026-09-26
type: answered-question
specs: [GAME-LAYOUT, GAME-LEVEL-1, TECH-ARCH, ART-RIG, ART-PIPELINE, ART-ENVIRONMENT, PROD-GLOSSARY]
questions: [Q-056]
---

## Problem

Axis handedness was undefined: level data uses +X east / +Z north, glTF is right-handed
Y-up, so a direct mapping mirrors the map (east on the left when looking north). Blocking
for GAME-LAYOUT, TECH-ARCH, GAME-LEVEL-1.

## Resolution

User decision 2026-09-26 written into the specs: level coordinates (x east, z north) in
`zoo-core`; world space right-handed Y-up; single conversion `world = (x, 0, −z)` in
`zoo_core::coords`; models never mirrored, oriented only by rotation about +Y.
- GAME-LAYOUT "Coordinate spaces", behaviour 5, LAYOUT-007…011; `depends_on` gains
  GAME-PLAYER (LAYOUT-009 uses its default camera). Added a bullet deriving the character
  front from ART-RIG §1 (glTF +Z = level south at yaw 0; spawn facing north = yaw 180°).
- TECH-ARCH decision 8.
- GAME-LEVEL-1 spawn section and behaviour 2 ("facing north (level +z = world −Z)").
- ART-RIG §1.1 and ART-PIPELINE §9 cross-reference the axes (north = −Z, never mirrored).
- Glossary rows `level_coords`, `world_space`.
- Q-056 marked `answered`. Remaining gap (front direction of props) → Q-061.

## Changed files

- `specs/10-gameplay/layout.md`, `specs/10-gameplay/levels/level-1.md`
- `specs/40-tech/architecture.md`
- `specs/30-art/character-rig-and-animation.md`, `specs/30-art/asset-pipeline.md`,
  `specs/30-art/environment.md`
- `specs/glossary.md`, `specs/open-questions.md`
