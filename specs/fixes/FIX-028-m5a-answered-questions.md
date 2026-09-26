---
id: FIX-028
date: 2026-09-26
type: answered-question
specs: [GAME-LAYOUT, GAME-LEVEL-1, ART-ENVIRONMENT, CONT-MISSIONS, GAME-RESCUE, GAME-PLAYER, PROD-POC]
questions: [Q-069, Q-080, Q-081, Q-083]
---

## Problem

After M5a several specs still called answered questions proposals: GAME-LAYOUT (`hiding_place`
row, "Scenery", "Hiding places" data) and GAME-LEVEL-1 "Hiding places" said "proposal Q-080";
ART-ENVIRONMENT left open "shade decal or a renderer feature — Q-080" although the answer
chose a flat ground decal; CONT-MISSIONS said two-word `kiga` labels are checked word by word
"(proposal, Q-083)" and listed Q-081/Q-083 as open; RESC-017 said "proposal, Q-069". GAME-PLAYER
§5 (listed as affected by Q-069) did not state the in-scope rule. PROD-POC title and goal
still described a zebra-only PoC and its asset row lacked the hippo and panda. The Q-024
answer still gave 1.75 m/s as the path speed.

## Resolution

Marked Q-080/Q-081/Q-083/Q-069 as answered at every mention; ART-ENVIRONMENT `shade_decal`
states the decal decision; GAME-PLAYER §5 gets the in-scope sentence (Q-069); PROD-POC title
"Proof of concept — level-1 rescue missions", goal and asset row name all three level-1
missions, Q-069 added to its open-questions list. Q-024 answer column gets the later
1.93 m/s decision appended (status unchanged — running still open).

## Changed files

- `specs/10-gameplay/layout.md`
- `specs/10-gameplay/levels/level-1.md`
- `specs/30-art/environment.md`
- `specs/20-content/missions/start-missions.md`
- `specs/10-gameplay/rescue-mission.md`
- `specs/10-gameplay/player.md`
- `specs/00-product/poc.md`
- `specs/open-questions.md`
