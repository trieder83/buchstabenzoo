---
id: FIX-008
date: 2026-09-26
type: naming
specs: [PROD-GLOSSARY, GAME-FEED, GAME-QUESTS, CONT-READING, CONT-MATH, CONT-MISSIONS, CONT-L10N]
questions: []
---

## Problem

Terms used consistently across the new specs were missing from the glossary (rescue mission,
hiding place, location riddle, info board, food storage, animal states, math task, math
level). Several tables/rules said just "level" for the reading level or math level, which the
glossary reserves for GAME-LAYOUT levels.

## Resolution

- Glossary: added `rescue_mission`, `hiding_place`, `location_riddle`, `info_board`,
  `food_storage`, `escaped`/`following`/`in_enclosure`, `math_task`, `math_level`; German
  terms not yet used anywhere are marked † as proposals (Q-045). `reading_level` now says
  never to write just "level".
- "Level" → "Reading level" / "Math level" in GAME-FEED, CONT-READING, CONT-MATH, the 10
  riddle tables of CONT-MISSIONS, GAME-QUESTS and CONT-L10N key naming.

## Changed files

- `specs/glossary.md`, `specs/10-gameplay/feeding.md`, `specs/10-gameplay/quests/overview.md`,
  `specs/20-content/reading-levels.md`, `specs/20-content/math.md`,
  `specs/20-content/missions/start-missions.md`, `specs/20-content/localization.md`
