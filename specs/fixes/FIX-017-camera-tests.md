---
id: FIX-017
date: 2026-09-26
type: missing-tests
specs: [GAME-PLAYER]
questions: []
---

## Problem

GAME-PLAYER §2 (decided camera) had rules without tests: minimum zoom 10 m, no sky,
45° rotation steps, roof cut-away of food storage/cave. Test rows were out of numeric
order. The spec relies on CONT-READING (read-aloud in the text panel) without listing it.

## Resolution

- PLAY-009 extended (10–20 m range, no sky at any zoom/rotation); new PLAY-011 (45° rotation
  steps) and PLAY-012 (roof cut-away); rows sorted.
- depends_on + CONT-READING; open-questions list extended (Q-048, Q-051, Q-052).

## Changed files

- `specs/10-gameplay/player.md`
