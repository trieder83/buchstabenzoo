# Work queue

Prioritized. Status: `todo` · `doing` (agent) · `blocked` (Q-###) · `done` (commit). Keep
each item one line + links; details live in the specs. The main session maintains this
file; agents update the status of their own item.

## Now

| # | Item | Owner | Status | Spec |
|---|---|---|---|---|
| 1 | Food boxes back outside, extra crates inside (Q-181, Q-194) | general | doing | GAME-FEED §7, LAYOUT-041 |
| 2 | Street under the moon door (Q-182), hints tests timing-independent | general | doing | LAYOUT-040, HINT-012 |
| 3 | Outline anti-aliasing (Q-191), haze culling on (Q-193) | performance | doing | PERF-R-018, CAMV-014 |
| 4 | Commit all, deploy (skill `deploy`) | main | todo | PLAT-003…009 |
| 5 | spec-manager run (INDEX, FIX log) | spec-manager | todo | — |

## Next (specified, not built)

| # | Item | Spec | Notes |
|---|---|---|---|
| 6 | Map (explored cells, `M`) | GAME-MAP, MAP-001…009 | |
| 7 | Character choice girl/boy | GAME-PLAYER PLAY-001/002 | |
| 8 | Math tasks + key box + "Math Fighter" note | CONT-MATH, GAME-CART §12–16 | models exist |
| 9 | Golf carts | GAME-CART | needs cart concept art |
| 10 | Per-frame allocations (PERF-R-004) | PERF budgets | |
| 11 | Remaining 7 night animals in a second night level | GAME-NIGHT, ART-ANIMALS | needs level spec |
| 12 | Day animals `sleep` clip | ART-ANIMALS | reuse `night_kit.QuadClips` |
| 13 | Frog hops between pads (AMB-014) | GAME-AMBIENT | |
| 14 | Families / babies | GAME-FAMILY | needs female + baby models |
| 15 | Events: burglars, storm; bees (honey deferred to level 4) | GAME-EVENTS | |
| 16 | Ad boards (passive, own cross-promotion) | GAME-ADS | level designer places boards |
| 17 | Level 4 with bears | Q-175 answered | needs level spec + concept |
| 18 | Concept art + manifest for `gate_zoo`, bamboo stages, mill wheel | APIPE, Q-153/154 | modelled before concept approval |
