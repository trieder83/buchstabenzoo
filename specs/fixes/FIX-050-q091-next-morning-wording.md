---
id: FIX-050
date: 2026-09-27
type: answered-question
specs: [GAME-LAYOUT, GAME-LEVEL-1, GAME-LEVEL-2, GAME-LEVEL-3, GAME-NIGHT, PROD-POC]
questions: [Q-091]
---

## Problem

Q-091 is answered (barriers open the next morning, implemented with NIGHT-010), but several
specs still described the temporary M5b rule "right after the celebration": the
`barrier_ne_tree` / `barrier_north_gate` rows and the moon-door note of GAME-LEVEL-1, the
"Implementation status" of GAME-LEVEL-2/3, the PoC M5b notes, GAME-NIGHT "Implementation data",
and the GAME-LAYOUT open-question list. LAYOUT-L1-010 still said the fallen tree is walkable
as soon as the three missions are complete (the code test `resc_009_l1_010_…` already checks
"only the next morning").

## Resolution

Reworded every outdated passage to "the next morning" (Q-091 answered); PoC notes marked as
superseded (kept as history); LAYOUT-L1-010 now says the tree stays solid until the next
morning (with `night_1` joined: the morning after `night_1` is complete, NIGHT-010).

## Changed files

- `specs/10-gameplay/layout.md`, `specs/10-gameplay/levels/level-1.md`,
  `specs/10-gameplay/levels/level-2.md`, `specs/10-gameplay/levels/level-3.md`,
  `specs/10-gameplay/night.md`, `specs/00-product/poc.md`
