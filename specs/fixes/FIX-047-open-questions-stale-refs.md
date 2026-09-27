---
id: FIX-047
date: 2026-09-27
type: cleanup
specs: []
questions: []
---

## Problem

`open-questions.md` check:

- Q-065's answer referenced PLAY-030/029 for the roof cut-away; PLAY-030 is the panel
  readability test, the roof tests are PLAY-028/029.
- Q-066's answer contained unescaped `|` inside `flow = "N"|"E"|…`, which broke the table row.
- Q-046's answer still gave the grass factor 0.7 without saying it was superseded (now
  0.98 / 1.93 m/s ≈ 0.51, Q-024, GAME-PLAYER §6).

No duplicate ids; numbering Q-001…Q-126 continuous.

## Resolution

Corrected the test reference, escaped the pipes, added a "superseded" note to Q-046.

## Changed files

- `specs/open-questions.md`
