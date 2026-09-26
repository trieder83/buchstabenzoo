# Gameplay QA — PoC M4 (commit 266540a), 2026-09-26

**Tested build:** the M4 release bundle (`web/dist` of 266540a, served statically) for all
browser measurements; zoo-core sweeps ran on the working tree (the path speed there is
already 1.75 m/s, which does not affect the collision and interaction results). **Speeds in
this report were measured with the M4 values: path 1.4 m/s, grass 0.98 m/s.** The new user
decisions (path 1.75 m/s, facts on the boards, default `de`, sign silhouettes, "Futter"
sign, auto-opening panels PLAY-023…027) were being built at the same time and are not
reported as missing.

Contexts: headless Chromium (SwiftShader WebGL2), desktop 1280×720 and 1920×1080 with
keyboard/mouse; touch 1080×2340 portrait and 2340×1080 landscape (device px, DPR 1) and
412×892 / 892×412 CSS @ 2.625 with real CDP multi-touch.

## Verdict

**Playable — no blocker in the zebra mission.** Collision and interaction logic are solid:
the user's "walking into billboards" is fixed (no overlap anywhere in the level, sweep of
8 500 walks), and interaction availability matches GAME-PLAYER §5 exactly (46 800 samples).
The zebra mission completes in `de` and `en` at all four reading levels, including wrong
food, wrong gate and walking away/coming back. The weak spots are around the edges of the
PoC scope and the reading panel on phones:

| # | Severity | Finding |
|---|---|---|
| F1 | **major** | Hippo and panda are interactable although only the zebra exists: raw Fluent keys on their boards, a prompt for an invisible hippo, invisible hippo "mission complete" |
| F2 | **major** | Panel text overflows on phones: M4 portrait `klasse3` needs 2× the panel; in the in-progress build (facts + smaller panel) the riddle is hidden below the facts at **every** level, portrait and landscape |
| F3 | **major** in M4 — fixed in the in-progress build | The text panel is a full-screen overlay that swallows all touches: no joystick, swipe or gear while a panel is open |
| F4 | minor in M4 — fixed in the in-progress build | The panel covers the player (portrait: whole screen; 1920×1080: screen centre) |
| F5 | minor | A pinch that starts across the screen centre walks instead of zooming |
| F6 | minor | Jetty: invisible wall 1.1 m before the jetty tip |
| F7 | minor | Hedge and zoo-wall bands: invisible wall 0.8–1.0 m in front of the visible hedge/wall |
| F8 | minor | Entrance arch looks open but is an invisible wall |
| F9 | minor | While leading, the player walks through closed gate models (also into foreign enclosures) |
| F10 | polish | Player walks through the zebra (animals not solid) |
| F11 | polish | At the panda board the grove canopy hides the player (fade circle leaves her dim) |
| F12 | polish | Perf smoke: no culling — 48 draw calls, 2 867 instances, 228 762 triangles at every zoom |
| F13 | polish | Collision footprints of bush/rock/cart smaller than their meshes (not reachable today) |
| F14 | polish | `__zoo.intervalMs` starts negative (debug metric) |

## What was measured (passes)

| Check | Spec | Result |
|---|---|---|
| Collision sweep: 1 072 free walkable cell centres × 16 directions × 4 s | PLAY-019, §7 | max overlap 0.0000 m, 0 trapped positions, max back-and-forth 1.85 cm (one frame at the entrance pillar corner) |
| Sliding along the west wall at 10/30/45/60/80° from the normal | §7 | tangential speed 0.170/0.483/0.692/0.849/0.965 m/s vs. expected 0.98·sin = 0.170/0.490/0.693/0.849/0.965 |
| Stop distances | r = 0.3 m | info boards 0.80 m from the board centre (cell edge + 0.3), bench 3.70 (edge 4.0), bridge rail 30.39 (rail 30.69), river bank 9.70 (edge 10.0), food box row z ≤ 10.06 (box front 10.36) |
| Bridge | §7 | deck walkable between the rails, no gap around the rail ends, water cells beside it solid |
| Interaction availability, 3 boards + 10 food boxes, 0.5–2.5 m, every 15°, 16 facings | PLAY-020 | 46 800 samples, 0 mismatches with the §5 rule; every board reachable in front (closest 0.80 m) |
| Walk up to each board straight and at ±30° | §5 | prompt shown after the collision stop at all 3 boards |
| Nearest wins in the food box row | PLAY-021 | ok |
| Directions after every 45° step (W/A/S/D) | §3 | exact for all 8 steps (e.g. step 1: W→NW, D→NE) |
| Speeds (game time, 1 s) | PLAY-005/006 | path 1.400 m/s, grass 0.980 m/s, W+D diagonal 0.980 (not faster) |
| Stop on release (real frames) | §3 | speed 0 in the next frame |
| Touch 1080×2340 portrait/landscape: 60 % stick, swipe, walk + swipe, walk + interact button, pinch in the right half 20 → 10 m | PLAY-015…017 | ok; interact button 96 px inside the viewport |
| Page gestures (double tap, spread, long swipe) | PLAY-018 | scale 1, no scroll, no selection |
| Rotating the phone while the stick is held | §3 | gestures reset, player stops (no stuck walking) |
| Zebra mission de/en × kiga…klasse3 | POC-002/003, RESC-010 | completes; texts from the Fluent files |
| No food / wrong food / wrong gate / leave > 15 m and come back | RESC-005/006/007 | "Ich habe Hunger!", "Hmm, das mag ich nicht.", "Hier wohne ich nicht!"; waits at 35 m, still waits at 6.5 m, follows again within 5 m |
| Follower never inside a solid cell/prop on a tour around the ring | RESC-006 | ok |
| Completion once | RESC-008 | gate no longer a target afterwards, carry consumed |
| Panel cap height | ADIR-003 | 3.2 % (portrait), 3.7 % (landscape 892×412) |
| Mashing E ×5 / R ×9 / Q ×9 | — | one panel; camera settles on a 45° multiple |

## Findings

### F1 — major: hippo and panda are interactable without content or model
- **Rule:** PROD-POC "Out of scope" (hippo/panda scenery only), CONT-L10N (no hard-coded or
  raw texts), RESC-003 only covers the zebra. Open question **Q-069** (added).
- **Repro (desktop, `de`, `klasse1`):** `app.debug_goto(-3.5, 28.8); app.debug_step(90)`,
  tap W, press E → panel shows `mission-panda-riddle-klasse1` (hippo: stand at (7.0, 17.5),
  face east → `mission-hippo-riddle-klasse1`). At the jetty tip (-10.7, 22.9) facing west the
  🤲 prompt appears for the invisible hippo; with melons it answers "Juhu, Futter! Ich komme
  mit.", follows invisibly, and walking into the hippo gate (9, 15) shows the celebration
  `mission-hippo-home`.
- **Expected:** nothing interactable that the child cannot see or read. **Actual:** raw keys,
  prompt at an empty pond, fake completion.
- **Screenshots:** `img/qa0926-f1-panda-board-raw-key.jpg`, `img/qa0926-f1-invisible-hippo-home.jpg`
- **Fix:** zoo-core `Game::interactables()` should only offer boards/animals/gates of animals
  whose mission is enabled (PoC: `SHOWN_ANIMALS` = zebra, e.g. `Game::set_enabled_missions`
  from zoo-web), or add the hippo/panda texts from CONT-MISSIONS §2/§3 (decision Q-069).
- **Tests:** `resc_014_every_interactable_has_texts` (ignored until Q-069),
  e2e `gameplay/panel.spec.ts` "RESC-017 …" (`test.fail`).

### F2 — major: reading panel overflows on portrait phones
- **Rule:** GAME-PLAYER §4 ("large text" panel), ADIR-003; checklist "no text outside the
  panel". No rule forbids scrolling → **Q-070** (added), proposed **PLAY-030**.
- **Repro:** 412×892 CSS @ 2.625, touch, `de` `klasse3`, open the zebra board.
- **Expected:** riddle + food word visible. **Actual:** panel content 1 540 px in a 758 px
  panel (en `klasse2`: 822 in 758); font 41 px (4.6 vh) with 84 px side padding leaves
  211 px for text → 1–2 words per line; the food word needs scrolling. Landscape fits
  (235 ≤ 249 px). The facts text (ANIM-006) will make it longer.
- **Screenshot:** `img/qa0926-f2-portrait-panel-klasse3.jpg`
- **In-progress build (M4b, 16:35, auto-panels + facts):** the panel is now a top strip
  (portrait 317 px, landscape 135 px of content height) and the facts come first — content
  height portrait de kiga/klasse1/klasse2/klasse3 = 397/678/1 214/2 295 px, landscape
  196/164/226/405 px: **the riddle (the core of the loop) is below the fold at every level
  on a phone**. Screenshot `img/qa0926-f2-m4b-portrait-riddle-hidden.jpg` (de `klasse1`: only
  the facts are visible).
- **Fix:** portrait side padding ≤ 24 px, full-width text, fit-to-panel font scaling or
  pages; show the riddle first (or facts collapsed behind a picture button) (Q-070).
- **Tests:** e2e `gameplay/panel.spec.ts` "PLAY-030 …" for portrait and landscape × de/en ×
  4 levels (all `test.fail` until Q-070 is decided and implemented).

### F3 — major (becomes a blocker with auto-opening panels): panel blocks all touch input
- **Rule:** PLAY-025 (new): "with a panel open, joystick/keyboard movement still moves the
  player".
- **Repro:** touch context, open the zebra board with the 👀 button, drag the left thumb →
  the player does not move (`#panel { position: fixed; inset: 0 }` receives the pointer
  events; the canvas never sees them). Also the gear button, mouse-drag rotation and swipes
  are blocked; only ✖ works. Keyboard still walks on desktop.
- **Why it matters now:** with PLAY-023 the panel opens by itself in front of a board; on a
  phone the child then cannot walk away (the only way out is ✖, and PLAY-026 keeps it closed).
- **Fix:** make the overlay `pointer-events: none` and only `.panel-body` (and its buttons)
  `pointer-events: auto`; drop the dimming background or keep it non-interactive.
- **Status:** fixed in the in-progress build (panel no longer blocks input; PLAY-025 passes).
- **Tests:** e2e `gameplay/panel.spec.ts` "PLAY-025 portrait/landscape phone" (passes on
  the in-progress build).

### F4 — minor: the panel covers the player
- **Rule:** PLAY-025 ("the panel does not cover the player on screen"); the CSS comment says
  "the player stays visible".
- **Actual:** 1920×1080: panel body y 96–650, player at y ≈ 540. Portrait: panel body
  96–868 of 892. **Screenshot:** `img/qa0926-f4-panel-covers-player.jpg` (the settings menu
  is also unclickable under the overlay).
- **Status:** fixed in the in-progress build (top strip), which in turn worsened F2.

### F5 — minor: pinch across the screen centre walks instead of zooming
- **Rule:** §3 lists pinch under the right thumb; children pinch in the middle → **Q-071**.
- **Repro:** two touches at x = 40 % and 60 % of the width, spreading → camera distance
  unchanged (20 → 20), player speed 1.4 (left finger became the joystick).
- **Fix (if Q-071 = yes):** in `TouchGestures.down`, a second pointer within ~150 ms of the
  first turns both into a pinch and releases the stick.

### F6 — minor: jetty ends in an invisible wall
- **Rule:** GAME-LEVEL-1 `jetty_pond` = cells x −11…−9; the `jetty_wood` model is 3.8 m long
  (x −2.3…+1.5) and reaches x = −11.8 over the pond cell.
- **Repro:** `debug_goto(-8.5, 23)`, hold A 4 s → stops at x = −10.70; the planks continue
  1.1 m further. **Screenshot:** `img/qa0926-f6-jetty-end.jpg`
- **Fix:** build `jetty_wood` 3 m long (ends at the pond edge) or place it so its tip is at
  x = −11.0 (level designer / kit script).

### F7 — minor: invisible walls at hedge and zoo-wall bands
- **Rule:** GAME-LAYOUT "Modular edges" (bands drawn as one row on the centre line, cells
  solid over the whole band — proposal Q-060 (b), joins Q-059).
- **Measured:** north hedge (2 cells deep) stop at z = 45.70, visible hedge face 46.48 →
  0.78 m of grass the child cannot enter; west zoo wall stop x = −21.70, wall face (low part)
  −22.70 → 1.0 m. **Screenshot:** `img/qa0926-f7-hedge-gap.jpg`
- **Fix:** for 2-deep bands either draw two rows / thicker pieces or make only the cells
  under the piece solid; input for Q-060.

### F8 — minor: the entrance arch looks open
- **Rule:** GAME-LEVEL-1 `entrance_gate`: "turnstiles behind the player are closed".
- **Repro:** from the spawn walk south → stops at z = 0.30 inside the open arch with the
  grass outside visible. **Screenshot:** `img/qa0926-f8-entrance-open-arch.jpg`
- **Fix:** until the entrance model exists, add a closed turnstile/gate placeholder in the
  arch (scene assembly, `Building "entrance"`).

### F9 — minor: walking through closed gates while leading
- **Rule:** Q-057 answer ("single `gate_wood` rotated open in-game"), GAME-RESCUE §7.
- **Repro:** lead the zebra, walk east into the hippo gate (9, 15) → the player stands at
  x = 9.70, 1 m inside the hippo enclosure, behind the closed gate model; "Hier wohne ich
  nicht!" is shown. Same at the zebra gate (the gate never opens for the zebra either).
  **Screenshot:** `img/qa0926-f9-through-closed-gate.jpg`
- **Fix:** rotate `gate_wood` open while the player leads animals (or when the player is
  within 2 m of the gate while leading), closed again after `InEnclosure`.

### F10 — polish: the player walks through the zebra
- **Rule:** none → **Q-072** (added).
- **Measured:** on a follow tour the zebra–player distance drops to 0.01 m when the player
  turns back; at the river the girl can stand inside the drinking zebra.
  **Screenshot:** `img/qa0926-f10-player-in-zebra.jpg`

### F11 — polish: player hard to see behind the grove at the panda board
- **Rule:** GAME-PLAYER §2 / PLAY-004 (occluders fade).
- **Repro:** stand in front of the panda board (-3.5, 28.8), camera 0°, 10–14 m: the grove
  canopies (z 17–27) are between camera and player; the dither circle shows her only dimly.
  **Screenshot:** `img/qa0926-f11-grove-occlusion.jpg`
- **Fix:** larger fade radius or full fade for tree crowns that intersect the camera→player
  segment.

### F12 — polish: performance smoke
- 48 draw calls, 2 867 instances, 228 762 triangles at spawn (20 m) **and** at 10 m zoom — no
  frustum culling. `app.frame()` CPU 0.7–7 ms (SwiftShader). No budget in the specs
  (proposal: per-batch frustum culling of static instances, check on the POC-005 phone).

### F13 — polish: footprints smaller than the meshes
- `bush` r 0.55 vs. mesh ±0.67 m; `rock` circle r 0.5 at the origin vs. mesh x −0.61…+0.86;
  `zookeeper_cart` box ±1.15 vs. mesh −0.94…+1.38; `info_board` box z −0.30…+0.20 vs. mesh
  −0.37…+0.25. None of them is reachable today (inside enclosures / barrier cells / solid
  board cell), so no visible bug yet. Suggest an asset test that each footprint covers the
  mesh outline between 0.1 and 1.3 m height within 0.1 m.

### F14 — polish: `__zoo.intervalMs` starts negative
- `main.ts` sets `last = performance.now()` before the first `requestAnimationFrame`
  timestamp, so the first dt is negative (−201 ms seen) and the moving average needs seconds
  to recover. Game time is safe (dt is clamped). Fix: initialise `last` from the first rAF
  timestamp.

## Notes on current behaviour (not bugs)
- Panels open only with E / 👀 today; walking away closes them (target change). Auto
  open/close (PLAY-023…027) is being implemented — see F3/F4 before it lands.
- The facing rule (±75°) means a child walking along the path past a board sees the prompt
  only between about 1.7 m before and 0.3 m past the board; pushing towards the board turns
  her (collision keeps her in place) — works, but worth watching in playtests.
- In SwiftShader, frames longer than 100 ms are clamped, so real-time speeds in headless runs
  are lower than game-time speeds (1.63 m in 1.51 s wall clock); not an issue on devices.

## Tests added
- `crates/zoo-core/tests/gameplay_qa.rs`: `play_029_collision_sweep_whole_level`,
  `play_029_no_jitter_against_obstacles`, `play_020_availability_sweep_all_boards_and_boxes`,
  `play_020_walk_up_to_each_board_gives_the_prompt`, `resc_006_follower_walks_around_obstacles`,
  `resc_014_every_interactable_has_texts` (ignored, F1/Q-069).
- `web/tests/e2e/gameplay/zebra-mission-levels.spec.ts`: RESC-010/POC-002/003 × de/en × 4
  levels with no-food, wrong-food, wrong-gate and wait/resume detours.
- `web/tests/e2e/gameplay/controls.spec.ts`: PLAY-015/016/017 + pinch on 1080×2340 portrait
  and landscape; camera-relative keys for all 8 steps and path/grass speed on 1920×1080.
- `web/tests/e2e/gameplay/panel.spec.ts`: PLAY-030 (portrait + landscape fit, 16 ×
  `test.fail`), PLAY-025 (passes), RESC-017 hippo/panda boards (`test.fail`).
- Run on the in-progress build (web/dist 16:35): 35/35 as expected (18 expected failures);
  on the M4 build the mission/controls tests pass as well. `npm run test:e2e` could not be
  run end to end because port 4173 was in use by the parallel implementation run.
- Spec rows added: PLAY-030, PLAY-031 (GAME-PLAYER), RESC-017 (GAME-RESCUE).
- `qa/checklist.md` created.

## Open questions added
Q-069 (out-of-scope animals interactable), Q-070 (panel fit in portrait vs. ADIR-003),
Q-071 (pinch across the centre line), Q-072 (animals solid for the player).
