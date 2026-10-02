---
id: FIX-076
date: 2026-10-01
type: consistency
specs: [PROD-VISION, PROD-POC, GAME-ADS, GAME-RESCUE, GAME-HINT, GAME-LAYOUT, GAME-ECON, CONT-MATH, ART-SOUND, PERF-BUDGETS, PROD-GLOSSARY, GAME-ANIMALS]
questions: [Q-128, Q-195, Q-200, Q-248, Q-249, Q-262]
---

## Problem
- PROD-VISION said ad boards are "passive, no links, no network"; Q-217 / Q-240 (readable own boards, gated link, signed external content) superseded that. Q-128's answer still read "no links, no network".
- PROD-POC M5a listed the old grass speed 0.98 m/s without a note (now 1.45 m/s).
- GAME-RESCUE intro said "replayable from the welcome board"; implemented and tested (RESC-031) is the settings button ❓.
- Test IDs used in code without a spec row: RESC-030 (split pair, the outside member follows), HINT-018 (split-pair hint), ADS-023 (release build accepts the real signed manifest).
- GAME-HINT: heading "rule 3.1b" pointed at a rule that does not exist; Q-195 listed as a proposal although answered 2026-09-30.
- GAME-LAYOUT "Level design rules" had two rules numbered 12 and a "(proposal — Q-202)" for an answered question; the NEVER STUCK rule is now 15 (Q-262 and glossary references updated).
- GAME-ECON: stray `****` markup, reference to "GAME-CART rule 41" (the rule is 5), banner said nothing was decided although Q-300…307 are answered. CONT-MATH did not mention the coin tasks.
- ART-SOUND "audio budget to be added" and Q-200 listed open although answered; PERF-BUDGETS had no audio budget row.
- open-questions.md: Q-248, Q-249, Q-262 rows had missing columns; Q-084…Q-094 were `open` although the answer column says "Implemented as proposed" (now `proposed`).
- Glossary lacked pair, baby, treat, basket, feeding spot, welcome board, intro, never stuck, ad board / campaign / parental gate.
- Long specs had no table of contents; the 10-gameplay folder had no topic grouping.

## Resolution
- Texts aligned as above; added the three missing test rows; PERF-BUDGETS budget 24 (audio <= 1.5 MB, ASND-008); glossary rows (German names marked †, Q-045).
- Added a **Contents:** line to layout, levels 1-3, night-1, night, start-missions, environment, ART-ANIMALS, ART-RIG, TECH-WATER.
- `specs/README.md`: reading guide for `10-gameplay/` grouped by topic. INDEX, DECISIONS, CODEMAP regenerated.

## Changed files
`specs/00-product/vision.md`, `specs/00-product/poc.md`, `specs/10-gameplay/{ad-boards,rescue-mission,hints,layout,economy,night}.md`, `specs/10-gameplay/levels/*.md` (Contents only), `specs/20-content/math.md`, `specs/20-content/missions/start-missions.md`, `specs/30-art/{sound,animals,environment,character-rig-and-animation}.md`, `specs/40-tech/water-rendering.md`, `specs/50-performance/budgets.md`, `specs/glossary.md`, `specs/open-questions.md`, `specs/README.md`, `specs/INDEX.md`
