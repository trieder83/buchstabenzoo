---
id: FIX-077
date: 2026-10-01
type: decision
specs: [GAME-FAMILY, GAME-GARDEN, GAME-RESCUE, GAME-LAYOUT, GAME-LEVEL-1, GAME-LEVEL-2, GAME-LEVEL-3, GAME-EVENTS, GAME-NIGHT, GAME-ANIMALS, ART-ENVIRONMENT]
questions: [Q-006, Q-017, Q-022, Q-023, Q-033, Q-084, Q-085, Q-086, Q-087, Q-088, Q-089, Q-090, Q-092, Q-093, Q-094, Q-188, Q-240, Q-248, Q-308]
---

## Problem
User decisions of 2026-10-01 on the FIX-076 audit had to be moved into the specs.

## Resolution
- Q-006, 017, 022, 023, 033, 084-094, 188, 240-249 answered "confirmed as implemented"; the "(proposal ...)" headings and mentions of Q-085/087/088/089/090/092/093 in layout, level-1/2/3 and animals now say "confirmed 2026-10-01"; the level files' "Proposals used in this level (not yet decided)" headings became "Design assumptions of this level".
- Care feeding / trough retired: families rule 4 and rule 6 reworded, FAM-003 and FAM-005 and GARD-006 marked *Retired* (GARD-006 merged into FAM-008), trough wording removed from garden, level-2, environment (`feeding_trough` dropped), night, events (EVT-008 now cites FAM-008); Q-248 answered "no trough prop"; Q-101 obsolete.
- families.md: the three "Implementation status" blocks merged into one (2026-10-01); pairs table text now says every species is a pair (Q-308, new answered row); the Q-280 citations in `assets/levels/*.toml` comments became Q-308 (comment text only; further Q-280 mentions remain in Rust sources and `assets/i18n/*/missions.ftl`, not touched).
- GAME-RESCUE promoted to `implemented` (RESC-001...032 all have a code test).
- Not promoted: GAME-FEED (FEED-026 stage-node asset test missing) and GAME-LEVEL-1 (LAYOUT-L1-033 math-fighter note/key box and -037 burglar hideout have no test and their features are not built; -026...031, -034...036 have no test ID in the code, -012/-019/-032 are manual reviews). Nothing was retired there because those rows describe features that are still planned.
- Q-260/Q-261 answers already cite PLAY-016 (no PLAY-037 left).

## Changed files
`specs/open-questions.md`, `specs/10-gameplay/{families,garden,layout,animals,night,events,rescue-mission}.md`, `specs/10-gameplay/levels/level-{1,2,3}.md`, `specs/10-gameplay/levels/night-1.md`, `specs/30-art/environment.md`, `assets/levels/*.toml` (comments), `specs/INDEX.md`
