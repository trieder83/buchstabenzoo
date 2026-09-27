---
id: GAME-CAMERA-VIEWS
title: Camera views — look-around (hold) and first person (toggle)
aspect: gameplay
module: camera-views
status: draft
depends_on: [GAME-PLAYER, GAME-RESCUE, GAME-LAYOUT, GAME-SAVE, ART-DIRECTION, TECH-ARCH]
test_prefix: CAMV
updated: 2026-09-27
---

# Camera views — look-around (hold) and first person (toggle)

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
2. **Look-around (hold).** While the look button is held — touch: the **eye button** 👁 next
   to the interact button (bottom right, ≥ 64 px); desktop: hold `F` or the **right mouse
   button** — the camera glides down to **3.5 m behind and 1.6 m above** the player,
   **pitch 12° down**, **vertical FOV 50°**. It starts looking in the zoo view's direction.
   Dragging (right thumb anywhere in the right half — or sliding the thumb that holds the eye
   button (implemented, part of the Q-109 proposal) — or the mouse while the right button is
   held) turns the view **smoothly** (not in 45° steps), at most **±180°** from where it
   started. The player can keep walking (left thumb / WASD / arrow keys, relative to the
   look-around view; ← → still step sideways here as in the zoo view, GAME-PLAYER §3).
   Releasing glides back to the zoo view (its rotation and zoom are unchanged).
   Rotation (`Q`/`R`, swipe steps) and zoom (wheel, `+`/`-`, pinch) input is ignored in both
   close views (proposal Q-123 a, CAMV-017). Look-around is not offered in first person (the
   eye button is hidden there; `F` and the right mouse button do nothing — CAMV-020).
3. **First person (toggle)** — controls: user decision 2026-09-27. **`V`** (desktop) or the
   round HUD button 👓 ("through my eyes", highlighted while on) toggles first person on and
   off. On touch the 👓 button sits **bottom right above the interact button** (≥ 64 px,
   inside the safe area) so the right thumb reaches it while the left thumb walks; on
   desktop the same button is shown and can be clicked. First person switches to the
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
     front of her chest, so it shows at the bottom of the screen (placeholder for hands, Q-112).
   - Toggling again glides back to the zoo view, which then looks in the zoo-view direction
     it had before (its 45° step and zoom are unchanged).
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
   level (1–3 and `night_1`) keeps ≥ 22.0 m planar (LAYOUT-N1-006 checks `night_1`).
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
   extra draw call (drawn in the outline pass where there is no geometry).
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
    `look_hold`, `toggle_first_person`, `look_drag`, `set_view_mode`.

## Acceptance criteria

- Holding the eye button / `F` / right mouse shows the zoo from behind the player with sky and
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
| CAMV-008 | Given the joined levels 1–3 and every mission's candidates, the player on every walkable cell centre ≤ 2.5 m from the own info board or next to the own gate, then every animal spot and wander cell centre (0.5 m, animal height, perch + 1 m) is ≥ 20.8 m (fog end, `FOG_END_M`) from the close-view eye; the level data keeps ≥ 22 m planar (GAME-LAYOUT "Sight"); `night_1` is covered by LAYOUT-N1-006 (22 m). | unit |
| CAMV-009 | Given the look-around camera at any yaw, portrait or landscape, then every world point inside its view frustum and ≥ 3 m from the eye is at least as far from the eye as from the player's feet (horizontally), so CAMV-008's player-position bound holds. | unit |
| CAMV-010 | Given the host input: holding `F` or the right mouse button sends look-hold on/off; `V` toggles first person; in a close view a mouse drag or a right-half touch drag sends continuous look drags (no 45° swipe steps), the left half still drives the joystick; the eye button sends look-hold while pressed. | unit (Vitest) |
| CAMV-011 | Given the view setting `first_person` is stored, when the game restarts, then it starts in first person; `look_around` is never stored; invalid stored values fall back to `zoo`. | unit (Vitest) |
| CAMV-012 | Given the game in the browser, when `V` is pressed, the player walks to the zebra info board and turns to look at it (arrow key), then the view is first person, the player's body is not drawn and the riddle panel opens; a mouse drag turns the view smoothly and looking away closes the panel; pressing `V` again returns to the zoo view. Screenshots `art/environment/poc/screenshot_camera_firstperson.png` (+ `_board`). | e2e |
| CAMV-013 | Given the game in the browser, when `F` is held, then within 0.5 s the view is look-around (camera 1.6 m high behind the player, sky pixels in the top part of the screen) and a mouse drag turns it; on release the zoo view returns within 0.5 s with the same yaw and zoom. Screenshot `art/environment/poc/screenshot_camera_lookaround.png`. | e2e |
| CAMV-014 | Given the same player position, then the draw calls in first person and in look-around are lower than in the zoo view at 20 m zoom. | e2e |
| CAMV-015 | Given first person while walking for 2 s, then the eye height stays 1.1 m ± 0.001 (no head bob) and per frame the view yaw changes by at most the eased turn (no snaps). | unit |
| CAMV-016 | Given the approved comic style frame, when a reviewer looks at the two screenshots, then the sky, clouds and haze read as the same comic style (flat colours, outlines on clouds, no photographic look) and nothing flickers while turning. | manual |
| CAMV-017 | Given look-around, then the occluder fade is on (blockers between eye and player fade) and the body is drawn; given first person, then no occluder fade is applied and the body is hidden; rotation and zoom input in either close view leaves the zoo view's 45° step and zoom unchanged (Q-123 a). Roofs of enterable buildings hide in every view (same region logic as PLAY-028; view-independent). | unit |
| CAMV-018 | Given every view change (zoo ↔ look-around, zoo ↔ first person, look-around → release) at any yaw and zoom, then at every frame of the glide the eye stays above the ground (eye height > 0.05 m, the close-view near plane) and moves smoothly: no 1/60 s step larger than 1.6 × the average step of the 0.4 s glide (eased, no cut, never through the ground; at 20 m zoom the peak is ≈ 1.25 m per frame). | unit |
| CAMV-019 | Given a touch device, then the 👓 first-person button is in the bottom-right thumb zone (above the interact button, fully inside the safe area); tapping it toggles first person on and off exactly like `V`, while the left thumb keeps walking. | e2e |
| CAMV-020 | Given first person, then the eye button is hidden and holding `F` or the right mouse button (`look_hold(true)`) leaves the view in first person; given the zoo view, then the 👓 button is shown on desktop and on touch and not highlighted. | unit |
| CAMV-021 | Given night (GAME-NIGHT), then the close-view sky is a dark-blue gradient (`#1E2A5A` top → `#3B4C8C` horizon), the haze equals the horizon colour and the fog end stays the day fog end (20.8 m, `FOG_END_M`; shining eyes beyond it are hidden); by day the day sky colours are unchanged (rule 7, Q-126). | unit |
| CAMV-022 | Given the player inside the zookeeper house (and the night house) in first person, then the roof is drawn and its ceiling is visible above (sky pixels absent in the upper screen area inside); switching to the zoo view hides the roof again (PLAY-028). | e2e |

## Open questions

- Q-109 partly answered (user 2026-09-27: `V` toggles first person, `F` / right mouse holds
  look-around, 👓 bottom right above the interact button, 👁 hold button). Still open: the
  proposal values (look-around boom 3.5 m / 1.6 m / 12°, eye 1.1 m, FOV 50°, fog 9–16 m,
  0.4 s glides, turn speeds) and first person for `kiga`.
- Q-110 Layout rule for the fog margin: hiding places ≥ 17 m from their board's standing points
  (currently 16.97 m minimum).
- Q-111 Look-around camera collision (eye inside a hedge or wall).
- Q-112 Hands in first person (carried food/bowl as the only "hands" for now).
- Q-123 What `Q`/`R`, swipe steps, mouse wheel / `+`/`-` and pinch do in the close views
  (rules 2 and 3 say the zoo view's step and zoom stay unchanged; the implementation still
  changes them while a close view is shown).
- Q-124 `V` pressed while look-around is held (implemented: switches to first person).
- Q-125 Camera views while driving a golf cart (GAME-CART).
- Q-126 (answered 2026-09-27, as recommended) Night colours of the comic sky and haze (GAME-NIGHT).

## Implementation (2026-09-26)

- `zoo_core::view` — `ViewMode`, all constants (eye heights, boom, FOV, fog, turn speeds),
  `fog_amount`, `min_eye_distance`, `hidden_by_fog`; `Player::lock_facing` (facing = view).
- `zoo_render::camera::FollowCamera` — `set_view`, `turn`, blended `pose()` (eye, yaw,
  pitch, FOV, near/far), `fog()`, `sky_amount()`, `occluder_fade()`, `hides_player()`;
  `zoo_render::sky` — sky gradient, comic clouds and haze GLSL, inserted into the outline
  pass (`shaders::post_fs`).
- `zoo-web` — `look_hold`, `toggle_first_person`, `set_view_mode`, `view_mode`,
  `saved_view_mode`, `look_drag`; `F` / `V` / arrow keys in `key`; debug getters
  `camera_eye`, `camera_pitch_deg`, `camera_view_yaw_deg`, `camera_blend`, `camera_fog`,
  `player_drawn`.
- Host — `MouseGestures` (right button = look-around), `LookButton` (touch eye button),
  right-thumb drags also sent as `look_drag`; `#view-btn` 👓 and `#look-btn` 👁️; view stored
  as `zoo.view`.
- Tests — CAMV-001…005, 007, 009, 015, 017, 018 in `crates/zoo-render/src/camera.rs`;
  CAMV-006, 008 in `crates/zoo-core/tests/camera_views.rs`; CAMV-010/011 in
  `web/src/{input,ui}.test.ts`; CAMV-012…014, 019 in `web/tests/e2e/camera_views.spec.ts`.
  Not yet covered: CAMV-016 (manual review), CAMV-020 (new 2026-09-27).
