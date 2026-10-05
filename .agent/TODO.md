# Work queue

Prioritized. Status: `todo` · `doing` (agent) · `blocked` (Q-###) · `done` (commit). Keep
each item one line + links; details live in the specs. The main session maintains this
file; agents update the status of their own item.

## Now

| # | Item | Owner | Status | Spec |
|---|---|---|---|---|
| 1 | Food boxes outside + labelled boxes inside (Q-181, Q-194) | general | done (6373543) | GAME-FEED §7 |
| 2 | Street under the moon door (Q-182), hints tests timing-independent | general | done (6373543) | LAYOUT-040, HINT-012 |
| 3 | Outline anti-aliasing (Q-191), haze culling on (Q-193) | performance | paused (state committed): haze culling on (done); AA = prototypes only, see PERF-R-016 "Next" | PERF-R-016, PERF-R-018, CAMV-014 |
| 4 | Commit all, deploy | main | done (6373543 live 2026-09-29) | PLAT-003…009 |
| 5 | spec-manager run (INDEX, FIX log) | spec-manager | todo | — |

## Next (specified, not built)

| # | Item | Spec | Notes |
|---|---|---|---|
| 6 | Map (explored cells, `M`) | GAME-MAP, MAP-001…009 | |
| 7 | Character choice girl/boy | GAME-PLAYER PLAY-001/002 | |
| 8 | Math tasks + key box + "Math Fighter" note | CONT-MATH, GAME-CART §12–16 | models exist |
| 9 | Golf carts | GAME-CART | needs cart concept art |
| 10 | Per-frame allocations (PERF-R-004) | PERF budgets | |
| 11 | Terrarium garden `night_2` (snake, chameleon, poison dart frog) | GAME-LEVEL-NIGHT-2 | built 2026-10-04 with placeholders (uncommitted); needs models, perf run; 7 further night animals still open |
| 12 | Day animals `sleep` clip | ART-ANIMALS | reuse `night_kit.QuadClips` |
| 13 | Frog hops between pads (AMB-014) | GAME-AMBIENT | |
| 14 | Families / babies | GAME-FAMILY | needs female + baby models |
| 15 | Events: burglars, storm; bees (honey deferred to level 4) | GAME-EVENTS | |
| 16 | Ad boards (passive, own cross-promotion) | GAME-ADS | level designer places boards |
| 17 | Level 4 with bears | Q-175 answered | needs level spec + concept |
| 18 | Concept art + manifest for `gate_zoo`, bamboo stages, mill wheel | APIPE, Q-153/154 | modelled before concept approval |

## Operator setup (user, not an agent) — Google Analytics data stream
- GA4 needs a **Web data stream**: Firebase console → Project settings → Your apps → add Web app (or GA Admin → Data streams → Add stream → Web, URL `https://letterzoo.web.app`); copy the **Measurement ID `G-…`** into `web/src/analytics-config.ts` (`ANALYTICS_MEASUREMENT_ID`), redeploy. Stream settings: keep Enhanced measurement only for page views (turn off outbound clicks, site search, video, file downloads); Google signals off; ad personalization off; data retention 2 months; register custom dimensions app_language, reading_level, level_id, animal_id, species_id. Until the id is set the welcome/consent dialog and 📊 button stay invisible (PLAT-022…033).

