# QA 2026-09-27 — "No door blocked by boxes or similar" (LAYOUT-031/032/033)

Scope: commit 5fdc481 (door/gate models everywhere) plus the working tree; the joined zoo
(levels 1–3 + `night_1`), every opening of `LevelScene::openings` (22: 13 enclosure gates incl.
3 night-house glass doors, 7 building doors, garden gate, moon door), the entrance turnstiles
and the 5 barriers. Method: zoo-core simulations with the real movement/collision
(`Game::update`, `Player::step_with`, 1/60 s steps), a data sweep of everything standing near
each opening (placements, night scene, food boxes, items, props, taps, lights), a 0.1 m
reachability flood fill inside every enterable building, and Playwright runs (desktop
1920×1080 keyboard, 1280×720 review shots, phone 1080×2340 touch joystick).

## Verdict

**Pass after fixes.** Every enterable door (zookeeper houses 1 and 3, night house), the garden
gate and the moon door (at night) are walked through in both directions, by keyboard and by
touch; no invisible walls. Led animals enter all 13 enclosures through their gates. Every info
board and enclosure sign stands beside its gate, none overlaps a gate opening. Inside the
buildings the bed, desk note, key box stand cell and fish bowl are reachable from the door.
The user's rule "doors are never blocked" was violated by the food-box rows in front of the
three food storages (Q-150) — **fixed** after the user's answer. Remaining: one major (lantern
posts have no collider at night, F4) and three small pockets beside doors (Q-157).

| # | Severity | Finding | Status |
|---|---|---|---|
| F1 | major (user rule) | Food boxes in front of the food storage doors (L1, L2, L3) | **fixed** (Q-150 answered) |
| F2 | minor | `board_zebra` flush with the zebra gate post: pocket between board and fence | Q-157 |
| F3 | minor | `tap_l3` 0.35 m beside the zookeeper-house-3 door: pocket at the facade | Q-157 |
| F4 | major | Lantern posts have no collider at night — the child walks through them | open, test LAYOUT-035 (ignored) |
| F5 | polish | Two ring lantern posts in gate leading lanes (koala, lion) | **fixed** (data) |
| F6 | polish | `board_n1_bat` 0.12 m in front of the night-house facade, 1 m from the door: pocket | Q-157 |
| F7 | info | POC-001 "walks > 0.8 m in 1.5 s" fails: frame-rate flakiness, not a movement regression | test fixed |

## Findings

### F1 — Food boxes blocked the food storage doors (major, user rule; fixed)
- Rule: GAME-LAYOUT "Doors are never blocked", LAYOUT-032; Q-150 (answered by the user
  2026-09-27: move the box in front of the door).
- Before: L1 boxes `meat` (0.4, 10.66) and `leaves` (1.2, 10.66) stood 0.84 m in front of the
  door x 0…1; the player stopped 0.91 m before the door. L2 `bamboo` (38.66, 29.8) / `grass`
  (38.66, 30.6) in front of the door z 30…31; L3 `meat` (3.4, 60.66) / `leaves` (4.2, 60.66) in
  front of the door x 3…4. The night food hut was already fine (boxes at ±1.4 m).
- Fix (data): the two displaced boxes of each storage moved to the ends of the row — L1 meat
  −4.4, leaves 4.4; L2 bamboo z 26.6, grass z 35.4; L3 meat −1.4, leaves 7.4. Free gap now
  1.78 m (box edges L1 −0.09…1.69, L2 29.31…31.09, L3 2.91…4.69), the whole door inside it.
  All boxes on walkable ground with a reachable reading spot (FEED-008, LAYOUT-L1-013,
  LAYOUT-L3-011 green). The only other change: LAYOUT-L1-013 allows the row ends up to 1 m
  beyond the facade corners (spec row updated).
- Screenshot after: `qa/reports/img/2026-09-27-doors-storage-gap.jpg`.
- Tests: LAYOUT-032 (both unit tests, no Q-150 exception any more, plus a gap ≥ 1.2 m check);
  e2e "LAYOUT-032 / Q-150: she walks right up to every food storage door".

### F2 — Pocket beside the zebra gate (minor, Q-157)
- Rule: no rule — proposal (LAYOUT-033 keeps enclosure signs ≥ 0.5 m from the gate post, info
  boards have no such rule). `board_zebra` (−9, 14) footprint ends at z 13.99, the gate opening
  starts at z 14.0; every other info board keeps ≥ 0.99 m.
- Repro: `Game` at (−7.5, 16.0), walk straight at the gate (−9.3, 13.0) → stuck at
  (−8.70, 15.31), wedged between board and fence, hidden behind the board. Browser: teleport
  (−7.5, 16), hold S+A (camera yaw 0) 3 s.
- Expected: reaches the gate cell (as at every other gate). Actual: stuck; backing out works.
- Screenshot: `qa/reports/img/2026-09-27-doors-zebra-board.jpg`.
- Suggested fix: `board_zebra` → (−9, 15) (the zebra reading spot in ~10 tests moves 1 m north)
  — design decision, Q-157.

### F3 — Pocket between the tap and the zookeeper-house-3 facade (minor, Q-157)
- `tap_l3` (−6.5, 60.75), collider r 0.15, 0.1 m in front of the facade and 0.35 m beside the
  door edge (x −7). Repro: player at (−5, 59) walking at the door (−7.5, 62.5) → stuck at
  (−6.05, 60.69); same from (−4.5, 60). Straight approach is fine.
- Screenshot: `qa/reports/img/2026-09-27-doors-zh3-tap.jpg`.
- Suggested fix: tap flush on the facade and ≥ 1 m from the door, e.g. (−5.5, 60.9) (Q-157).

### F4 — Lantern posts have no collider at night (major)
- Rule: GAME-LAYOUT `[[light]]` placement rules (Q-118/Q-137): "collider C(0, 0, 0.12) only
  while visible". Known issue class "walking into props".
- Repro: `debug_set_daytime('night')`, player at (45.8, 24.25) (or 52.2, 24.25), hold east →
  she walks straight through `l2_lantern_ring_s_e` (53.0, 24.25); `Level::colliders()` has no
  night shapes at all (`NightScene` placements never reach collision).
- Screenshot: `qa/reports/img/2026-09-27-doors-lamp-post-night.jpg` (post drawn through her).
- Suggested fix (implementing agent): night-only collider shapes for `lantern_post` (and the
  `string_post`/string-light posts) in `Level`, switched with the daytime like barrier
  colliders; then remove `#[ignore]` from LAYOUT-035.
- Test: `layout_035_lantern_posts_are_solid_at_night_only` (ignored until fixed; fails today).

### F5 — Lamp posts in gate leading lanes (polish; fixed)
- Rule: "posts never on a gate's leading cells" (GAME-LAYOUT `[[light]]`).
- `l2_lantern_ring_s_m` stood on the lion gate axis, 2.25 m in front of the gate (on the path
  where the child turns in with the lion); `l2_lantern_ring_w_n` stood on the edge line of the
  koala gate lane, 1.25 m out. Moved along their path edge: (47.0, 24.25) → (48.5, 24.25) and
  (36.25, 38.0) → (36.25, 38.5); `level-2.md` coordinates updated. Both are still 0.25 m inside
  the path edge; the gate posts beside the gates are unchanged.
- Test: LAYOUT-032 (no lamp post in the 2.5 m lane in front of any opening).

### F6 — Pocket behind the night-house bat board (polish, Q-157)
- `board_n1_bat` (−38.5, 38.5) stands 0.12 m in front of the facade, 1 m west of the door;
  walking along the facade from the west at the door the child sticks at (−39.31, 38.70).
  The door walkway itself is free (0.99 m to the board).
- Screenshot: `qa/reports/img/2026-09-27-doors-nighthouse-board.jpg`.

### F7 — POC-001 smoke failure is frame-rate flakiness (info; test fixed)
- Rerun on an idle machine: failed twice (0.64 m, 0.68 m; needs > 0.8 m).
- Cause: headless Chromium advances the rAF timestamp ≈ 1/60 s per callback, but software
  WebGL at 1080×2340 draws only ≈ 11 fps (17 callbacks in 1.5 s wall time, each reported as
  16.7 ms), and the host clamps dt to 0.1 s — so 1.5 s of wall time is only ≈ 0.3–0.7 s of game
  time. In game time the walk is exact: 1.5 s fixed steps from the spawn = 2.895 m = 1.93 m/s
  (path speed, PLAY-005); 1.84 m per game second in the browser run. **No movement
  regression.** The frames got slower with the new building models (219 k triangles, 20 draws
  at the spawn view).
- Fix (test only): POC-001 now holds W / D for 1.5 s of *game* time (`app.time()`), keeping the
  real keyboard path. Green.
- Side note (polish): `web/src/main.ts` initialises `last = performance.now()` before the first
  rAF, so the first dt is negative (`intervalMs` starts negative); harmless (clamped to 0).

## Verified OK (no finding)

- **Walk-through (zoo-core sim + e2e keyboard + e2e touch):** zookeeper house 1 (E door), house 3
  (S door), night house (S door), garden gate, moon door (night): out → in and in → out in
  2.3–3.8 s, centred, no side push. Closed enclosure gates are solid when not leading.
- **Approach at an angle (LAYOUT-034):** all openings from 3 m in front, ±45° at 2.5 m and along
  the frontage (1.5 m out, 3 m aside) — only the three Q-157 pockets stop the player.
- **Walkway sweep:** no collider, food box, item, prop, tap, lamp post or string-light post in
  any opening or its 1.4 m walkway (after F1/F5); wall lamps and the key box hang on the facade
  beside the doors (≥ 0.8 m aside).
- **Leading animals:** all 13 enclosures (incl. hedgehog, bat, owl through the glass doors at
  night): the led animal enters its enclosure when the player steps on the gate (1.6–2.5 s from
  3.5 m out), ends inside its rect.
- **Info boards:** all 13 beside their gate (board centre 1.5–4.5 m along the fence from the gate
  centre; night boards 6 m away in front of the house), none overlaps a gate opening; reading
  spots (1 m in front) are outside the gate lanes. Enclosure signs: LAYOUT-033 green.
- **Inside buildings:** reachability flood fill from each door (player radius): house 1 — bed
  (usable from 0.85 m), note and key-box stand cells reachable, only the bed cells are solid;
  house 3 — fish bowl usable from 0.62 m (table and one cabinet cell solid); night house hall —
  every hall cell reachable, glass doors reachable.
- **Barriers:** after opening, `barrier_ne_tree`, `barrier_l2_construction`, `barrier_north_gate`
  and the moon door are walked through both ways; `barrier_east_repair` leads nowhere yet.
- **Entrance:** the three turnstiles stay closed by design (QA F8); nothing stands in the arch.

## Tests added / changed

- `crates/zoo-core/tests/openings.rs`
  - LAYOUT-032 `layout_032_doors_and_gates_keep_a_free_walkway`: Q-150 exception removed.
  - LAYOUT-032 `layout_032_no_lamp_box_item_or_prop_in_a_walkway` (new): things without a
    collider, lamp posts in the 2.5 m leading lane, food-box gap ≥ 1.2 m at storage doors.
  - LAYOUT-034 `layout_034_openings_reachable_from_the_front_and_at_an_angle` (new; the Q-157
    pockets are listed as known exceptions — remove each entry when fixed).
  - LAYOUT-035 `layout_035_lantern_posts_are_solid_at_night_only` (new, `#[ignore]` until F4).
- `crates/zoo-core/tests/layout.rs` LAYOUT-L1-013: row ends up to 1 m beyond the facade.
- `web/tests/e2e/gameplay/doors.spec.ts` (new): keyboard walk-through of every walkable opening,
  storage doors reachable (Q-150), closed gate solid, touch walk-through, review shots.
- `web/tests/e2e/smoke.spec.ts` POC-001: walk measured in game time.
- Spec: LAYOUT-032 extended, LAYOUT-034/035 added (`specs/10-gameplay/layout.md`); checklist
  item added (`qa/checklist.md`); Q-157 added.
