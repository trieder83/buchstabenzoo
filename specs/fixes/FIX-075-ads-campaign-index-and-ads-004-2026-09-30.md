---
id: FIX-075
date: 2026-09-30
type: contradiction
specs: [GAME-ADS, GAME-ADS-C1, GAME-ADS-C2, GAME-ADS-C3]
questions: []
---

## Problem
- ADS-004 ("an ad board is not interactable") contradicted the rule 4 exception and ADC1-002..005 (Math Fighter boards are readable and offer a gated link, Q-217).
- `specs/INDEX.md` was hand-maintained; it lacked GAME-ADS-C1..C3, and its Open Q / test counts were stale (retired rows counted, answered/new Q missing).

## Resolution
- ADS-004 now limits the passive rule to placeholder and non-`mathfighter` campaigns.
- Added `tools/spec_index.py` (frontmatter + test rows + open Q -> INDEX.md; also reports duplicate ids, unknown `depends_on`, folder/aspect mismatch; `--check`) and regenerated `specs/INDEX.md` (44 specs, 578 test cases).

## Changed files
`specs/10-gameplay/ad-boards.md`, `specs/INDEX.md`, `tools/spec_index.py`
