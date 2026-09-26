---
id: FIX-026
date: 2026-09-26
type: naming
specs: [GAME-FEED, PROD-POC, CONT-MISSIONS, CONT-L10N]
questions: []
---

## Problem

M4 changes introduced the term "label panel" (FEED-007, PROD-POC M4 notes) for what
GAME-PLAYER §4, GAME-FEED §6 and ART-DIRECTION call the (close-up) text panel. The new key
`mission-<animal>-home` was used (CONT-MISSIONS, GAME-RESCUE §11) but not listed in the
CONT-MISSIONS key scheme (behaviour 1). Q-064 lists CONT-MISSIONS and CONT-L10N as affected
but neither spec referenced it.

## Resolution

"label panel" → "text panel with the label" in FEED-007 and PROD-POC; `mission-<animal>-home`
added to CONT-MISSIONS behaviour 1; Q-064 added to the Open questions of CONT-MISSIONS and
CONT-L10N.

## Changed files

- `specs/10-gameplay/feeding.md`
- `specs/00-product/poc.md`
- `specs/20-content/missions/start-missions.md`
- `specs/20-content/localization.md`
