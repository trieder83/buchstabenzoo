---
id: FIX-025
date: 2026-09-26
type: naming
specs: [GAME-PLAYER]
questions: []
---

## Problem

GAME-PLAYER after the M4 changes: behaviour rule 7 (collision) stood before rule 6 (ground
speed); §4 still named only `E`/Space as interact keys although §3 (FIX-024) adds Enter;
PLAY-022 tested "lily-pad-free ground decoration", which does not match the non-solid
decoration list of §7; Q-064 and Q-065 (both list GAME-PLAYER as affected) were missing
from the Open questions section.

## Resolution

Rules reordered 5 → 6 → 7 (text unchanged); §4 now says `E`/Space/Enter; PLAY-022 names
grass tufts and flowers inside beds (§7); Q-064 and Q-065 added to Open questions.

## Changed files

- `specs/10-gameplay/player.md`
