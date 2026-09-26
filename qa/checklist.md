# Gameplay QA checklist

Run for every gameplay QA pass (see `.claude/agents/gameplay-qa.md`). Measure with the debug
API (`window.__zoo.app`, web/README.md) — positions, speeds, states, events — and compare
with the spec numbers. Screenshot every finding (`qa/reports/img/`, small JPEGs).

Contexts: desktop 1920×1080 (keyboard/mouse); phone `hasTouch` 1080×2340 portrait and
2340×1080 landscape (device px) plus 412×892 CSS @ 2.625 (real phone CSS size), real CDP
multi-touch.

Automated coverage: `crates/zoo-core/tests/gameplay_qa.rs` (sweeps) and
`web/tests/e2e/gameplay/` (browser). Items marked *(auto)* are covered there.

## Movement (GAME-PLAYER §3, §6)
- [ ] Path speed and grass speed ±5 % (1 s in game time on the ring path / open grass) *(auto)*
- [ ] Surface transition blends within 0.2 s, no instant jump (PLAY-007)
- [ ] No jitter while walking or pushing against obstacles *(auto: PLAY-031)*
- [ ] Stops on key/stick release in the next frame *(auto)*
- [ ] W/A/S/D and stick directions camera-relative after every 45° step *(auto)*
- [ ] Diagonal keys are not faster than straight
- [ ] No stuck spots anywhere in the level *(auto: PLAY-031)*

## Collision (GAME-PLAYER §7)
- [ ] Whole-level sweep: no overlap with cells/props, never trapped *(auto: PLAY-031)*
- [ ] Head-on and at 30–60° into: info boards, enclosure signs, map board, food boxes,
      benches, trees, bushes, rocks, bamboo, fences, gates (closed / while leading), hedges,
      zoo wall, buildings, water (river, pond), bridge rails and deck, jetty, barriers
- [ ] Sliding keeps the tangential speed (speed × sin of the angle)
- [ ] Visual vs. collision: no invisible walls (gap between stop point and the visible
      surface ≤ player radius + 0.2 m), no walking into visible geometry
- [ ] Following animals never inside solids *(auto: RESC-006)*; do they clip props/fences visually?
- [ ] Player vs. animals (Q-072)

## Interaction (GAME-PLAYER §4–5)
- [ ] Every info board and food box: 0.5–2.5 m, every 15°, 16 facings = spec rule *(auto: PLAY-020)*
- [ ] Walk up to each board straight and at ±30° → prompt *(auto)*
- [ ] Behind / beside (> 60°) / facing away → no prompt
- [ ] Nearest wins (food box row) (PLAY-021)
- [ ] Panel opens/closes cleanly (key, button, ✖, Escape, walking away); no double panels on mashing
- [ ] Correct text for reading level and language; no raw Fluent keys anywhere *(auto: RESC-017)*
- [ ] Text fits the panel without scrolling, portrait and landscape *(auto: PLAY-030)*
- [ ] Panel does not cover the player; input still works while it is open *(auto: PLAY-025)*
- [ ] Out-of-scope content (animals without a model, missions not in the build) is not interactable

## Camera (GAME-PLAYER §2)
- [ ] Pitch 55°, distance 14 m default, 35° vertical FOV (PLAY-008); level starts at 20 m (LAYOUT-L1-011)
- [ ] Zoom limits 10–20 m by wheel, +/- and pinch *(auto)*
- [ ] 45° steps eased; mashing Q/R never leaves an odd angle
- [ ] Never shows sky; occluders fade; player visible behind the grove, hedges, storage roof

## Touch (GAME-PLAYER §3)
- [ ] Controls hidden until the first touch, never for mouse only (PLAY-013/014)
- [ ] Left thumb joystick: 60 % deflection = 60 % speed, up = away from camera *(auto)*
- [ ] Right-half swipe ≥ 40 px = exactly one step *(auto)*; pinch in the right half *(auto)*;
      pinch across the centre line (Q-071)
- [ ] Both thumbs at once: walk + swipe, walk + interact button *(auto)*
- [ ] No page scroll/zoom/selection (PLAY-018); double tap and long press
- [ ] Interact button ≥ 64 px, inside the viewport/safe area, portrait and landscape *(auto)*
- [ ] Rotating the phone while the stick is held: input resets, no stuck walking

## Mission flow (GAME-RESCUE, PROD-POC)
- [ ] Zebra mission in `de` and `en` at `kiga`…`klasse3` *(auto)*
- [ ] No food / wrong food → gentle feedback, zebra stays *(auto)*
- [ ] Wrong gate while leading → refuse, keeps following *(auto)*
- [ ] Walk away > 15 m → waits; back within 5 m → follows *(auto)*
- [ ] Completion triggers once; carried food consumed; celebration text *(auto)*
- [ ] Settings change (language/level) mid-mission and with a panel open

## Child-friendliness
- [ ] Touch targets ≥ 64 px; nothing requires fast reactions
- [ ] No confusing states (prompts without anything visible, raw keys, invisible walls,
      walking through closed gates)

## Performance smoke
- [ ] `draw_calls`, `instances`, `triangles`, `__zoo.frameMs` at spawn (20 m) and zoomed in (10 m)
- [ ] No console errors or warnings other than the placeholder notice
