---
id: GAME-CAMERA-VIEWS
title: Camera views — zoo view, first person and look-around (one view button)
aspect: gameplay
module: camera-views
status: draft
depends_on: [GAME-PLAYER, GAME-RESCUE, GAME-LAYOUT, GAME-SAVE, ART-DIRECTION, TECH-ARCH]
test_prefix: CAMV
updated: 2026-10-03
---

# Camera views — zoo view, first person and look-around (one view button)

## Goal

Next to the high-angle **zoo view** of GAME-PLAYER §2 (the default) the child can look at the
zoo from close up: the **look-around view** while a button is held (a quick look at the
horizon, e.g. "where is the giraffe tower?") and the **first-person view** as a toggle (walk
through the zoo with the player character's eyes). Both must keep the riddles fair (GAME-RESCUE rule 1:
a hiding place is never visible from its own info board), stay comfortable for children
(no motion sickness) and cost no more draw calls than the zoo view.

## Behaviour

1. **Three views** (`view_mode`): `zoo` (GAME-PLAYER §2, default, unchanged), `look_around`
   (held) and `first_person` (toggled). Every change of view **glides** within 0.4 s
   (eased, smoothstep): eye position, view direction, field of view, near/far plane, fog and
   sky blend together — never a cut.
2. **Look-around.** On touch it is a **persistent mode** reached with the view button
   (rule 3a: the third view of the cycle, 👁 icon, no hold needed); on desktop it is held
   with `F` or the **right mouse button** (hold semantics unchanged: only the hold that
   started it releases it — a look-around chosen with the button stays until the next tap).
   In look-around the camera glides down to **3.5 m behind and 1.6 m above** the player,
   **pitch 12° down**, **vertical FOV 50°**. It starts looking in the zoo view's direction.
   Dragging (right thumb anywhere in the right half, or the mouse while the right button is
   held) turns the view **smoothly** (not in 45° steps), at most **±180°** from where it
   started. The player can keep walking (left thumb / WASD / arrow keys, relative to the
   look-around view; ← → still step sideways here as in the zoo view, GAME-PLAYER §3).
   Releasing glides back to the zoo view (its rotation and zoom are unchanged).
   Rotation (`Q`/`R`, swipe steps) and zoom (wheel, `+`/`-`, pinch) input is ignored in both
   close views (Q-123 answered: option a, CAMV-017). `F` and the right mouse button do nothing in first person (CAMV-020); from first person
   the view button goes on to look-around (rule 3a).
   Pressing `V` / 👓 while look-around is held glides straight to first person; releasing the
   look button afterwards does nothing (Q-124 answered, CAMV-023). The look-around eye may end
   up inside a hedge or wall — accepted for the PoC; the next camera iteration pulls the eye in
   along the boom when a solid cell/prop is between player and eye (min 1.2 m; Q-111 answered).
3. **First person (toggle)** — controls: user decision 2026-09-27. **`V`** (desktop) or the
   view button (rule 3a) toggles first person on and off (`V`) or cycles on. First person switches to the
   **eye height 1.1 m** above the player's feet, looking in the **player's facing**
   direction, vertical FOV 50°.
   - The view turns smoothly by dragging (right thumb in the right half, or the mouse with any
     button) and with the **arrow keys ← →** (↑ ↓ keep walking); **pitch** by vertical drag,
     limited to **−20° … +20°** (0° = level).
   - **Walking is relative to the view**: stick up / `W` walks where the child looks, left /
     right steps sideways (`A`/`D`, stick) **without turning**.
   - **Facing = view direction**: the player's facing is locked to the view direction, so the
     interaction rules of GAME-PLAYER §5 (range 2 m, readable side ±60°, facing ±75°) use
     what the child looks at; the reading panels open and close by themselves as in the zoo
     view (GAME-PLAYER §4).
   - The player character's **body is not drawn**. What she carries (food box, fish bowl) stays drawn in
     front of her chest, so it shows at the bottom of the screen (placeholder for hands, Q-112 answered: kept for the PoC).
   - Toggling again glides back to the zoo view, which then looks in the zoo-view direction
     it had before (its 45° step and zoom are unchanged).
3a. **One view button** (user request 2026-10-03, replaces the former 👓 `#view-btn` and 👁
   `#look-btn`). Tapping it cycles **zoo view → first person → look-around → zoo view**
   (`App::cycle_view()` returns the new view id). **First person → look-around is a direct
   zoom out** (user answer to Q-326, 2026-10-03): the camera pulls back from the eye (1.1 m)
   along a straight line to the look-around pose (3.5 m behind, 1.6 m up) in 0.5 s
   (`PULL_BACK_S`, smoothstep), the view direction stays, the pitch eases to −12°, the FOV
   stays 50°, never a cut and no detour through the zoo pose. The player's body, hidden in
   first person, reappears once the pull-back is 30 % along (`PULL_BACK_BODY_AT`, the camera
   has left the head), so it does not pop in. Look-around → zoo is the normal 0.4 s glide
   up, zoo → first person is unchanged. Transitions between two close views are direct
   pose interpolations (CAMV-028). The button shows
   the icon of the **current** view (🗺️ zoo, 👓 first person, 👁️ look-around), carries
   `data-view="<id>"` and an `aria-label` naming the **next** view (Fluent
   `ui-view-cycle-<id>`: "Nächste Ansicht: …" / "Next view: …"), is **≥ 64 px** and is
   highlighted in the two close views. It sits at the lower end of the right-hand control
   column above the interact button (GAME-PLAYER §3 "Small screens"). Look-around chosen this
   way stays until the next tap; dragging in the right half turns it. It is **never stored**:
   a reload starts in the saved zoo / first-person view (rule 9). Desktop keeps `V` (toggle
   first person) and `F` / right mouse (hold look-around); both stay in step with the button.
4. **Kids' comfort (motion sickness care).** No head bob, no camera shake, no roll; the eye
   height stays constant while walking. Turning is slow and eased (drag 0.25° per CSS px,
   keys 90°/s, easing rate 12/s — no instant snaps). FOV 50° vertical (comfortable, not
   fish-eye). The zoo view stays the default; transitions never go through the ground.
5. **Distance fog keeps the riddles fair.** (Distance fog / haze of the camera, not the map
   `fog` of GAME-MAP.) In both close views a soft **pastel comic haze**
   in the sky colour starts at **11.7 m** from the eye and **fully hides everything from 20.8 m** (view distance +30 %, user decision 2026-09-27; was 9–16 m)
   on (the *visibility distance*, `FOG_END_M`). Hedges, trees and buildings still block the
   view as in the zoo view. Therefore every hiding place (animal spot and wander area, at
   0.5 m, the animal's height and perch + 1 m) must be ≥ 20.8 m from every point where the
   child can read its own info board (≤ 2.5 m from the board, the panel range) or stands next
   to its gate — the same standing points as the screen tests LAYOUT-L1-006/L2-006/L3-006.
   The look-around eye is 3.5 m *behind* the player along its view direction, so any point it
   sees (more than ≈ 2.6 m away) is farther from that eye than from the player — the
   player's position is the binding eye position for both views (CAMV-009).
   *Measured 2026-09-26 (16 m fog):* nearest wander cell 16.97 m (`loc_trampoline` from the
   monkey board) — Q-110 added the layout margin (GAME-LAYOUT "Sight": ≥ 22 m planar since
   the fog end grew to 20.8 m). *Measured 2026-09-27 after FIX-056:* nearest wander cell
   22.01 m from the close-view eye (`loc_big_ball` from the elephant gate's corner cell); every
   level (1–3 and `night_1`) kept ≥ 22.0 m planar (LAYOUT-N1-006 checks `night_1`). The
   Q-157 move of `board_zebra` broke it (`loc_river` 21.26 m); Q-171 answered 2026-09-28:
   `board_zebra` moved south of the gate (zebra places ≥ 24.2 m), and the 22 m planar margin
   is now tested for levels 1–3 too (CAMV-008, `camv_008_level_data_keeps_the_22_m_margin`;
   nearest 22.00 m, `loc_big_ball` from the elephant gate's corner cell).
6. **Draw distance = fog end.** In the close views the far plane is the fog end + 2 m
   (22.8 m = fog end + 2 m; the zoo view has no fog and a 120 m far plane), and static batches whose bounds lie beyond it (or outside the view) are culled —
   so the close views cost fewer draw calls than the zoo view at maximum zoom-out (20 m).
   *Implementation:* each static batch keeps the bounds of its instances per 8 m ground
   chunk and is drawn only when one chunk is in the frustum; decal quads and skinned
   characters (animals, ambient crowds) are frustum-culled too (all views benefit).
   *Measured 2026-09-26 (1280×720, CAMV-014):* level-1 spawn zoo 30 / first person 28 /
   look-around 29; level-1 ring north 35 / 20 / 27; level-2 spawn 41 / 24 / 32 draw calls
   (before the chunk/decal/character culling the zoo view needed 46 / 49 / 58).
7. **Comic sky.** Only in the close views (the zoo view never sees the horizon, PLAY-009):
   a vertical gradient from a pale cream-blue haze at the horizon to a friendly blue at the
   top, with a few flat, rounded, **blocky comic clouds** low above the horizon (white, one
   flat shadow tone at the bottom, dark-brown outline like all comic lines, ART-DIRECTION).
   No sun disc. The fog colour is the sky gradient in the same direction; in the last part of
   the fog (amount 0.85 → 1) it blends to the full sky including clouds, so a fully hidden
   object is exactly the sky (no silhouette cut into a cloud) while half-hidden houses never
   show ghost clouds. Night mode (GAME-NIGHT, Q-126) recolours the sky and haze: dark-blue
   gradient `#1E2A5A` → `#3B4C8C` (the haze = the horizon blue, fog end 20.8 m kept), blue
   clouds, an outlined comic moon and a few 4-point stars; shining animal eyes (NIGHT-006) are
   hidden by the haze beyond 20.8 m like all geometry (Q-126 answered). The sky costs no
   extra draw call (drawn in the outline pass where there is no geometry). On weak phones the
   automatic low quality tier leaves the clouds out (PERF-BUDGETS rule 5, Q-170).
8. **Near plane and occluders.** The close views use a **0.05 m near plane**, so walls in
   front of the eye are not cut open. The occluder fade (GAME-PLAYER §2) stays on in
   look-around (objects between the camera and the player fade) and is off in first person.
   Roofs of enterable buildings hide while the player is inside, in every view.
9. **Settings.** The chosen view (`zoo` or `first_person`) is stored in the host settings
   (key `zoo.view`, next to language and reading level) and restored at start; look-around
   is never stored.
   The game save (GAME-SAVE) keeps storing only the zoo-view rotation and zoom.
10. **Logic lives in Rust.** Poses, transitions, turn limits, fog and cull distances are
    in `zoo-render::camera` (pure, unit-tested) with constants from `zoo_core::view`; the
    facing lock in `zoo_core::player`; the host only maps buttons, keys and drags to
    `look_hold`, `toggle_first_person`, `cycle_view`, `look_drag`, `set_view_mode`.
11. **Golf carts** (Q-125 answered): while driving a golf cart (GAME-CART) only the zoo view
    is available — the view button is hidden, `V`, `F` and the right mouse button are
    ignored; a stored first-person view returns when the child gets out.

## Acceptance criteria

- Holding `F` / right mouse (touch: choosing look-around with the view button) shows the zoo from behind the player with sky and
  fog within 0.4 s; releasing returns to the unchanged zoo view within 0.4 s.
- In first person the child can walk to an info board, look at it and the riddle panel opens;
  looking away closes it.
- From no info board and no gate can a hiding place be seen in either close view.
- Draw calls in both close views are below the zoo view's at maximum zoom-out at the same spot.


## Roofs and ceilings in first person (user decision 2026-09-27)

In **first person** the roof of a building the player is in stays **visible** and shows a
**ceiling from inside** (the "roof disappears inside" rule of GAME-PLAYER §2 / PLAY-028
applies only to the zoo view and look-around, where the camera is above or behind the
player). Roof models therefore have a proper inner ceiling surface (not just back faces);
the ceiling is lit by indoor lamps at night. Passing through a door in first person does
not fade anything.
*Implementation (2026-09-27):* `zoo_core::view::roof_hidden(inside, view)` — hidden only
when inside and not in first person; building models hide their `roof` (with the inner
ceiling) and `walls_upper` parts by the instance hide mask, procedural roofs by their
render region, a building's name board with its roof.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| CAMV-001 | Given the zoo view, when look-around is held, then after 0.4 s the eye is 3.5 m ± 0.05 behind (horizontally) and 1.6 m ± 0.05 above the player's feet, the pitch is 12° ± 0.5° down and the vertical FOV 50°; halfway through the eye lies between both poses (no jump > 1 m per 1/60 s frame); after release it is back at the zoo pose (PLAY-008 values) within 0.4 s. | unit |
| CAMV-002 | Given look-around, when the view is dragged by any amount, then the yaw changes continuously (no 45° steps), never more than 180° from its start, and after release the zoo view's 45° step and zoom are unchanged. | unit |
| CAMV-003 | Given first person toggled with player facing F, then after 0.4 s the eye is 1.1 m above the feet, the view direction is F, the vertical FOV 50° and the near plane 0.05 m; pitch drags are clamped to −20° … +20°; toggling back returns to the zoo pose. | unit |
| CAMV-004 | Given first person or look-around, then the horizon is on screen (sky visible) in portrait and landscape; given the zoo view, PLAY-009 still holds (no sky). | unit |
| CAMV-005 | Given a close view, then fog starts at 11.7 m and is 1.0 from 20.8 m on, and the far plane is 22.8 m; given the zoo view, then there is no fog and the far plane is 120 m; during a transition fog and far plane change monotonically. | unit |
| CAMV-006 | Given first person looking at an info board from its readable side within 2 m, then it is available; when the player walks sideways (stick right) her facing stays the view direction and the board stays available; turning the view away makes it unavailable; interact opens the riddle. Leaving first person, the facing follows the walk direction again. | unit |
| CAMV-007 | Given first person looking in direction D, then stick up walks along D and stick right walks 90° clockwise of D (seen from above), and the player's yaw does not change while walking. | unit |
| CAMV-008 | Given the joined levels 1–3 and every mission's candidates, the player on every walkable cell centre ≤ 2.5 m from the own info board or next to the own gate, then every animal spot and wander cell centre (0.5 m, animal height, perch + 1 m) is ≥ 20.8 m (fog end, `FOG_END_M`) from the close-view eye; the level data keeps every wander cell and spot ≥ 22 m planar (cell centres) from the same standing points (GAME-LAYOUT "Sight"; tested for levels 1–3 since Q-171); `night_1` is covered by LAYOUT-N1-006 (22 m). | unit |
| CAMV-009 | Given the look-around camera at any yaw, portrait or landscape, then every world point inside its view frustum and ≥ 3 m from the eye is at least as far from the eye as from the player's feet (horizontally), so CAMV-008's player-position bound holds. | unit |
| CAMV-010 | Given the host input: holding `F` or the right mouse button sends look-hold on/off; `V` toggles first person; in a close view a mouse drag or a right-half touch drag sends continuous look drags (no 45° swipe steps), the left half still drives the joystick; there is no separate eye button any more. | unit (Vitest) |
| CAMV-011 | Given the view setting `first_person` is stored, when the game restarts, then it starts in first person; `look_around` is never stored; invalid stored values fall back to `zoo`. | unit (Vitest) |
| CAMV-012 | Given the game in the browser, when `V` is pressed, the player walks to the zebra info board and turns to look at it (arrow key), then the view is first person, the player's body is not drawn and the riddle panel opens; a mouse drag turns the view smoothly and looking away closes the panel; pressing `V` again returns to the zoo view. Screenshots `art/environment/poc/screenshot_camera_firstperson.png` (+ `_board`). | e2e |
| CAMV-013 | Given the game in the browser, when `F` is held, then within 0.5 s the view is look-around (camera 1.6 m high behind the player, sky pixels in the top part of the screen) and a mouse drag turns it; on release the zoo view returns within 0.5 s with the same yaw and zoom. Screenshot `art/environment/poc/screenshot_camera_lookaround.png`. | e2e |
| CAMV-014 | Given the same player position, then the draw calls in first person and in look-around are lower than in the zoo view at 20 m zoom. | e2e |
| CAMV-015 | Given first person while walking for 2 s, then the eye height stays 1.1 m ± 0.001 (no head bob) and per frame the view yaw changes by at most the eased turn (no snaps). | unit |
| CAMV-016 | Given the approved comic style frame, when a reviewer looks at the two screenshots, then the sky, clouds and haze read as the same comic style (flat colours, outlines on clouds, no photographic look) and nothing flickers while turning. | manual |
| CAMV-017 | Given look-around, then the occluder fade is on (blockers between eye and player fade) and the body is drawn; given first person, then no occluder fade is applied and the body is hidden; rotation and zoom input in either close view leaves the zoo view's 45° step and zoom unchanged (Q-123 a). Roofs of enterable buildings hide in every view (same region logic as PLAY-028; view-independent). | unit |
| CAMV-018 | Given every view change (zoo ↔ look-around, zoo ↔ first person, look-around → release) at any yaw and zoom, then at every frame of the glide the eye stays above the ground (eye height > 0.05 m, the close-view near plane) and moves smoothly: no 1/60 s step larger than 1.6 × the average step of the 0.4 s glide (eased, no cut, never through the ground; at 20 m zoom the peak is ≈ 1.25 m per frame). | unit |
| CAMV-019 | Given a touch device, then the view button is in the bottom-right thumb zone (above the interact button, fully inside the safe area, ≥ 64 px); tapping it from the zoo view enters first person exactly like `V`, while the left thumb keeps walking. | e2e |
| CAMV-020 | Given first person, then holding `F` or the right mouse button (`look_hold(true)`) leaves the view in first person; given the zoo view, then the view button is shown on desktop and on touch, shows 🗺️ and is not highlighted. | unit |
| CAMV-021 | Given night (GAME-NIGHT), then the close-view sky is a dark-blue gradient (`#1E2A5A` top → `#3B4C8C` horizon), the haze equals the horizon colour and the fog end stays the day fog end (20.8 m, `FOG_END_M`; shining eyes beyond it are hidden); by day the day sky colours are unchanged (rule 7, Q-126). | unit |
| CAMV-022 | Given the player inside the zookeeper house (and the night house) in first person, then the roof is drawn and its ceiling is visible above (sky pixels absent in the upper screen area inside); switching to the zoo view hides the roof again (PLAY-028). | e2e |
| CAMV-023 | Given look-around held, when `V` is pressed, then the view glides to first person within 0.4 s; releasing `F` afterwards keeps first person (rule 2, Q-124). | unit |
| CAMV-024 | Given first person stored and the player gets into a golf cart, then the zoo view is shown, the view button is hidden and `V`/`F` do nothing while driving; when she gets out, first person returns (rule 11, Q-125). | unit |
| CAMV-025 | Given the zoo view, when the view button is tapped three times, then `view_mode()` is `first_person`, `look_around`, `zoo` in that order, the camera mode follows and the button's `data-view` / icon (🗺️ 👓 👁️, vitest `VIEW_ICONS`) / `aria-label` (Fluent `ui-view-cycle-<id>`, de + en) show the current view and name the next; `V` and the button stay in step (`V` in look-around → first person). | e2e, unit (Vitest) |
| CAMV-026 | Given look-around chosen with the button, then it stays after the tap (no hold needed), a right-half drag turns it, `F` pressed and released does not leave it, the next tap returns to the zoo view; a reload (or `saved_view_mode`) gives zoo / first person, never look-around. | e2e |
| CAMV-027 | Given the 780×360, 360×780, 412×892 and 892×412 viewports, then the single view button is ≥ 64 px, inside the viewport and overlaps no other control (PLAY-037); `#look-btn` no longer exists. | e2e |
| CAMV-028 | Given first person, when the view button is tapped (`cycle_view()`), then the camera moves from the eye to the look-around pose in ≤ 0.6 s along an eased straight path: its distance from the starting eye never shrinks, it never rises above 3 m (the zoo pose is ~11 m up and is never shown), the FOV stays 50°, the body stays hidden until 30 % of the pull-back and is shown afterwards, and the end pose is 3.5 m behind / 1.6 m up / pitch −12°. | unit (zoo-render), e2e |

## Open questions

- Q-109 answered 2026-09-27: `V` toggles first person, `F` / right mouse holds look-around,
  👓 bottom right above the interact button, 👁 hold button; the proposal values (look-around
  boom 3.5 m / 1.6 m / 12°, eye 1.1 m, FOV 50°, 0.4 s glides, turn speeds) are accepted, the
  HUD toggle is offered on every reading level (the zoo view stays the default).
- Q-110 answered 2026-09-27: fog margin as a layout rule (GAME-LAYOUT "Sight"; now ≥ 22 m since
  the fog end grew to 20.8 m).
- Q-111 answered 2026-09-27: look-around eye inside a hedge or wall accepted for the PoC; pull
  in (min 1.2 m) in the next camera iteration (rule 2).
- Q-112 answered 2026-09-27: carried food/bowl are the only "hands" in first person for the PoC.
- Q-123 answered 2026-09-27: rotation and zoom input is ignored in the close views (rule 2, CAMV-017).
- Q-124 answered 2026-09-27: `V` while look-around is held switches to first person (rule 2, CAMV-023).
- Q-125 answered 2026-09-27: only the zoo view while driving a golf cart (rule 11, CAMV-024).
- Q-126 (answered 2026-09-27, as recommended) Night colours of the comic sky and haze (GAME-NIGHT).
- Q-171 answered 2026-09-28: `board_zebra` moved south of the zebra gate; CAMV-008 now also checks the ≥ 22 m planar margin of the level data for levels 1–3.

## Implementation (2026-09-26)

- `zoo_core::view` — `ViewMode`, all constants (eye heights, boom, FOV, fog, turn speeds),
  `fog_amount`, `min_eye_distance`, `hidden_by_fog`; `Player::lock_facing` (facing = view).
- `zoo_render::camera::FollowCamera` — `set_view`, `turn`, blended `pose()` (eye, yaw,
  pitch, FOV, near/far), `fog()`, `sky_amount()`, `occluder_fade()`, `hides_player()`;
  `zoo_render::sky` — sky gradient, comic clouds and haze GLSL, inserted into the outline
  pass (`shaders::post_fs`).
- `zoo-web` — `look_hold`, `toggle_first_person`, `cycle_view`, `set_view_mode`, `view_mode`,
  `saved_view_mode`, `look_drag`; `F` / `V` / arrow keys in `key`; debug getters
  `camera_eye`, `camera_pitch_deg`, `camera_view_yaw_deg`, `camera_blend`, `camera_fog`,
  `player_drawn`.
- Host — `MouseGestures` (right button = look-around), right-thumb drags also sent as `look_drag`; the one `#view-btn` (`cycle_view`); view stored
  as `zoo.view`.
- Tests — CAMV-001…005, 007, 009, 015, 017, 018 in `crates/zoo-render/src/camera.rs`;
  CAMV-006, 008 in `crates/zoo-core/tests/camera_views.rs`; CAMV-010/011 in
  `web/src/{input,ui}.test.ts`; CAMV-012…014, 019 in `web/tests/e2e/camera_views.spec.ts`.
  Not yet covered: CAMV-016 (manual review), CAMV-020 (new 2026-09-27).

- Q-326 answered 2026-10-03 (user): the transition from first person to the view behind is a zoom out (rule 3a, CAMV-028).
