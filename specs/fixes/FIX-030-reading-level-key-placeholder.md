---
id: FIX-030
date: 2026-09-26
type: naming
specs: [CONT-MISSIONS, GAME-ANIMALS, GAME-LEVEL-3]
questions: []
---

## Problem

Fluent key schemes used the placeholder `<level>` for the reading level
(`mission-<animal>-riddle-<hiding_place>-<level>`, `mission-zebra-facts-<level>`,
`mission-goldfish-bowl-hint-<level>`, 53 places), while the glossary reserves "level" for map
levels and GAME-LEVEL-1 / CONT-MISSIONS "Several hiding places" already wrote
`<reading_level>`.

## Resolution

Placeholder renamed to `<reading_level>` in every key scheme (keys themselves unchanged).

## Changed files

- `specs/20-content/missions/start-missions.md`
- `specs/10-gameplay/animals.md`
- `specs/10-gameplay/levels/level-3.md`
