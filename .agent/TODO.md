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
| 19 | **Golf carts + key box + "Math Fighter" note — gap analysis 2026-10-04 (plan, CART-001…014)**: models of desk, `note_paper`, `key_box`, `cart_key` are placed statically (scene/models.rs `item_model`, level-1.toml `[[item]]` l.1130-1155) but there is NO logic: no `Target::KeyBox/Note/Cart`, no math generator or `math_level` setting (MATH-001…006 also missing), no lock panel, no golf cart model (`zookeeper_cart` is a static repair prop), no drive mode, no `drive` clip, no save fields. Plan: **P0 spec** fix first: GAME-CART lacks parking coords/stand cells for carts (rule 1: only prose), no data shape for `[[cart]]`, "no reversing" vs proposal, cart vs `zookeeper_cart` prop naming (glossary), key box open state/visible key after taking, locked-cart feedback (🔒 icon + hint line), what the note shows without math level, math-level setting UI (Q-034 still proposal), hints priority 4 targets, save v3 fields (key, carts, seated); new Qs: (a) math_level default/setting place (rec: `mathe1` default, menu picker next to reading level), (b) wrong-code feedback after 3 tries (rec: pulse note + 🧭 hint to note), (c) carts of locked levels spawn locked, (d) lock panel digit padding "005" (Q-132 answered). **P1 math core** (zoo-core `math.rs`: `MathLevel`, seeded task gen, MATH-001…006, i18n `math.ftl` de+en) ~1 d. **P2 key/lock/note** (core `Target::KeyBox`/`Note`, `Game::enter_code`, tries counter, `has_cart_key` + HUD 🔑, hint targets in hints.rs, save v3; zoo-web `lock panel` + note reading panel; web/src/ui.ts big ▲/▼ wheels; swap closed/open key box mesh + hide key; CART-011…014 + e2e) ~2 d. **P3 cart** (Blender `tools/blender/props/golf_cart.py` + concept sheet + `drive` clip for player_girl/boy [character-artist] + manifest/check_glb; core `cart.rs` kinematics 4.5/2.0 m/s, box collision 1.3x2.3 slide, stop before animals/visitors, follow-wait GAME-RESCUE 6, carried items, no panels while driving, camera +3 m zoo only (CAMV-024), night headlights; render + web wiring + touch get-out button; CART-001…010 + e2e) ~3-4 d. **P4** gameplay-qa + performance run, spec-manager, i18n de/en. Total ~7-9 d; ordering P0 > P1 > P2 (key box playable alone, unlocks nothing visible until P3 — ship P2 with a "key found" message only if P3 is not ready, or hide behind a feature flag) > P3. Risks: never-stuck (cart must slide/turn on the spot; get-out always allowed; spawn out if blocked), hint loops (HINT-026…029: key box/note hint only while carts locked and nothing better; never repeat after key taken), no time pressure, riddle solvability (math task must be solvable at the child level; without math level -> mathe1; reading level text variants), cart stops for animals, saved seated state restore must find a free exit cell. Detail: see report in session 2026-10-04. | GAME-CART, CONT-MATH, GAME-HINT | needs cart concept art + Q-answers (a)-(c) |

## Operator setup (user, not an agent) — Google Analytics data stream
- GA4 needs a **Web data stream**: Firebase console → Project settings → Your apps → add Web app (or GA Admin → Data streams → Add stream → Web, URL `https://letterzoo.web.app`); copy the **Measurement ID `G-…`** into `web/src/analytics-config.ts` (`ANALYTICS_MEASUREMENT_ID`), redeploy. Stream settings: keep Enhanced measurement only for page views (turn off outbound clicks, site search, video, file downloads); Google signals off; ad personalization off; data retention 2 months; register custom dimensions app_language, reading_level, level_id, animal_id, species_id. Until the id is set the welcome/consent dialog and 📊 button stay invisible (PLAT-022…033).

