---
id: FIX-064
date: 2026-09-28
type: contradiction
specs: [GAME-FEED]
questions: []
---

## Problem

GAME-FEED got `depends_on: [GAME-WORLD, CONT-READING, GAME-RESCUE, GAME-SAVE, GAME-HINT,
GAME-PLAYER]` with the put-down / bamboo-forest sections. GAME-RESCUE and GAME-SAVE already
depend on GAME-FEED, and GAME-HINT reaches it via GAME-RESCUE, so `depends_on` had one large
cycle (GAME-FEED, GAME-RESCUE, GAME-SAVE, GAME-HINT, GAME-NIGHT, GAME-CAMERA-VIEWS,
GAME-LAYOUT, GAME-MAP, ART-ENVIRONMENT, CONT-MISSIONS).

## Resolution

Applied the FIX-005 direction rule: the data spec (GAME-FEED) does not depend on the flow,
save or hint specs that use it. Removed GAME-RESCUE, GAME-SAVE and GAME-HINT from GAME-FEED;
kept GAME-PLAYER (key table, interaction range). The text cross-references stay. Checked by
script: no cycles, all ids exist.

## Changed files

- `specs/10-gameplay/feeding.md`
