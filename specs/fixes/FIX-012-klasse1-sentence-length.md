---
id: FIX-012
date: 2026-09-26
type: contradiction
specs: [CONT-MISSIONS]
questions: []
---

## Problem

The English `klasse1` koala riddle "I sit at the very top." has 6 words and violates
READ-002 (≤ 5 words per `klasse1` sentence). All other `klasse1` riddle sentences (de, en)
were checked and pass.

## Resolution

Changed to "I sit at the top." (same meaning as "Ich sitze ganz oben."). The koala riddle
as a whole is still under review because it contains its place word (Q-039).

## Changed files

- `specs/20-content/missions/start-missions.md`
