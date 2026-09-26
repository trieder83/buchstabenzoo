---
id: GAME-PLAYER
title: Player character, camera and controls
aspect: gameplay
module: player
status: draft
depends_on: [PROD-VISION, CONT-READING]
test_prefix: PLAY
updated: 2026-09-26
---

# Player character, camera and controls

## Behaviour

1. **Character choice.** On first start the child chooses between `player_girl` and
   `player_boy` by tapping one of two big 3D previews — no reading required. The choice is
   saved and can be changed in the settings.
2. **High-angle follow camera** (user decision 2026-09-26, Q-049), like zoo-park
   simulation games: looks down at ≈ 55°, ≈ 14 m from the player (portrait), narrow **vertical**
   FOV of 35° in every screen orientation (Q-052, user decision 2026-09-26; isometric-like),
   no sky. The player stays near the screen centre.
   - Rotation by dragging (touch) or mouse, in 45° steps with smooth easing.
   - Limited zoom (pinch / mouse wheel) between ≈ 10 m and ≈ 20 m distance; zoom-out is
     capped so an animal's hiding place is not visible from its own enclosure (GAME-LEVEL-1).
   - Objects between camera and player (trees, roofs, hedges) fade to semi-transparent;
     roofs of interactable interiors (food storage, cave) cut away when the player is near.
   - **Roofs disappear inside buildings** (user decision 2026-09-26): every building the
     player can enter (food storage, zookeeper house, enclosure shelters such as the hippo
     hut, the cave) has a walkable interior and a separate **roof part** (and, where it
     blocks the view, the upper part of the camera-facing wall). While the player is inside
     (or in the doorway), the roof fades out within 0.3 s so the child sees what happens
     inside; it fades back in 0.3 s after she leaves. Animals inside a building are visible
     the same way when the player is inside with them.
3. **Movement and camera controls.**
   - **Desktop:** WASD / arrow keys walk; mouse drag or `Q`/`R` rotates the camera in 45°
     steps; mouse wheel (or `+`/`-`) zooms; `E`, Space or Enter interacts. (Fix 2026-09-26:
     the spec listed `E` both for rotation and for interact; `E` stays interact, rotation
     moved to `Q`/`R` — FIX-024.)
   - **Touch — two thumbs** (user decision 2026-09-26, Q-018). Touch controls exist **only
     when the device has touch**: they are hidden until the first touch input and never shown
     on devices without touch.
     - **Left thumb — walk:** a *floating* joystick. Touching anywhere in the left half of
       the screen places the stick under the thumb; dragging sets direction and speed
       (deflection 0–100 %, dead zone 10 %); releasing stops the player. Directions are
       relative to the camera (up = away from the camera).
     - **Right thumb — camera and actions:** in the right half, a horizontal swipe (≥ 40 px)
       rotates the camera by one 45° step per swipe; a two-finger pinch zooms (10–20 m). The
       **interact button** (§4) sits bottom-right within right-thumb reach.
     - Both thumbs work at the same time (walk while rotating or pressing interact).
     - Controls stay inside the safe area, touch targets ≥ 64 px (CSS), work in portrait and
       landscape; the page never scrolls, zooms or selects text while playing.
   - UI overlays (joystick, buttons, text panel) are HTML elements of the host shell; their
     texts come from zoo-core via Fluent (TECH-ARCH).
   - *PoC implementation notes (M4):* the left/right split is the screen's vertical centre
     line; the stick radius is 60 CSS px; one swipe rotates at most one step (the gesture
     re-arms when the finger lifts); the touch interact button is 88 CSS px. The mouse keeps
     the continuous drag rotation (one step per 70 px). Desktop shows a key hint (`E`)
     instead of the touch button; it can also be clicked.
4. **Interact.** When the player is near an interactable (animal, visitor, food box, sign,
   door), a large icon button appears; tapping it (or `E`/Space/Enter) interacts.
   **Reading panels open and close automatically** (user decision 2026-09-26): when an
   info board or a food box becomes *available* (§5) the **close-up text panel** opens by
   itself after a short settle time of 0.25 s (so walking straight past without facing it
   does not flash it); when it stops being available (walked away > 2.5 m, turned away, or
   another interactable became nearer) the panel closes by itself after 0.3 s. The player
   can keep walking while a panel is open — the panel never blocks movement input and never
   covers the player (top of the screen in landscape, upper part in portrait). The manual
   close button (✖/Esc) still works; a manually closed panel stays closed until the player
   has left and re-entered the interactable's range. Actions stay explicit: taking a food
   box needs the take button (✋) or interact; animals and gates still use the interact
   button. The panel has a read-aloud button (CONT-READING).
   *PoC implementation notes (M4b):* the open/close state machine lives in zoo-core
   (`Game::update`, events `PanelOpened` / `PanelClosed`; `Game::close_panel` for ✖/Esc and
   after taking food); the host only shows/hides the panel. The panel is anchored at the top
   of the screen in both orientations (above HUD and gear while open), at most 38 % of the
   screen height, two columns (facts | riddle + food) in landscape; longer texts scroll
   inside it. Only the panel box takes pointer input.
5. **Interaction range and facing:** an interactable is *available* when the player is
   within 2 m of its interaction point **and in front of it**: for boards and signs the
   player stands on the readable side (within ±60° of the panel's facing direction) and
   faces it (player facing within ±75° of the direction to the board). When several are
   available, the nearest one wins. The interact button/hint and the text panel only appear
   for an available interactable (user report 2026-09-26: an info box must appear when the
   player is in front of an info board).
   *Interactables in the PoC (M4, proposal):* info boards (both facing rules; interaction
   point = board origin), food boxes (both rules, the label plate is the readable side;
   point = box centre), escaped animals (player faces them ±75°, distance to the animal
   position) and — only while the player leads animals — enclosure gates (faces ±75°,
   distance to the gate centre; interacting = leading the animals in, same as walking into
   the gate, GAME-RESCUE §7/§8). Enclosure signs are not interactable yet (no sign texts:
   Q-064). Only info boards, animals and gates of the missions in scope of the level are
   interactable (Q-069 answered; GAME-LAYOUT `[level] missions`). zoo-core decides
   availability and the result; the host only shows UI.
6. **Ground speed.** The player can walk on paths and on grass; on grass the player is
   slower (user decision 2026-09-26). **Path speed 1.93 m/s** — 1.75 m/s (+25 %) and then another
   +10 % (user decisions 2026-09-26, playtests); **grass speed stays 0.98 m/s**, i.e.
   `grass_speed_factor` ≈ 0.51. The walk clip (authored for 1.4 m/s) plays at speed ÷ 1.4
   (≈ 1.38× on paths, within the ART-RIG playback clamp of 0.8–1.4). Following animals use the same
   speeds. Cells covered by solid
   elements (fences, water, hedges, buildings) are not walkable (GAME-LAYOUT).
7. **Collision with props:** every placed prop with a footprint (info boards, signs, map
   board, benches, food boxes, trees, rocks, bamboo, carts, cones, barriers, bridge rails,
   buildings) is solid. The player is a circle of radius 0.3 m and cannot overlap a prop's
   collision shape (box or circle from the model's ground footprint); she slides along it.
   Small ground decoration (grass tufts, lily pads, flowers inside beds) is not solid.
   (User report 2026-09-26: the player could walk into billboards.)
   *Implementation (M4):* the scene assembly (prop placements from the level data) lives in
   zoo-core; each prop model has a footprint table (boxes/circles, e.g. `info_board` box
   1.0 × 0.5 m, `enclosure_sign` two post circles, trees a trunk circle, bridge rails) that
   zoo-core turns into collision shapes; the renderer only draws the same placements.
   Solid cells are collision boxes too. Grid paths (following animals, scripted player)
   avoid cells whose centre the 0.3 m circle cannot occupy.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| PLAY-001 | Given a fresh profile, when the game starts, then the character choice screen shows `player_girl` and `player_boy` and no text is needed to choose. | e2e |
| PLAY-002 | Given `player_boy` was chosen, when the game restarts, then `player_boy` is loaded. | unit |
| PLAY-003 | Given the player at 1.9 m from a visitor, then the interact button is shown; at 2.1 m it is hidden. | unit |
| PLAY-004 | Given an object between camera and player, then that object is rendered semi-transparent while it occludes the player. | unit |
| PLAY-005 | Given joystick input forward for 1 s on a path, then the player moves forward at walking speed ± 5 %. | unit |
| PLAY-006 | Given joystick input forward for 1 s on grass, then the player moves at walking speed × `grass_speed_factor` ± 5 %. | unit |
| PLAY-007 | Given the player walks from a path onto grass, then the speed changes within 0.2 s (no instant jump). | unit |
| PLAY-008 | Given the default camera, then its pitch is 55° ± 2°, its distance 14 m ± 0.5 m and its vertical FOV 35° in portrait and landscape. | unit |
| PLAY-009 | Given the player zooms in and out to the limits, then the camera distance stays within 10 m … 20 m, and at every distance and rotation no sky is visible (the horizon is above the top screen edge). | unit |
| PLAY-010 | Given the player interacts with an info board, then the text panel opens and shows the riddle for the current reading level. | e2e |
| PLAY-011 | Given the default camera, when the player drags to rotate, then the camera yaw settles on a multiple of 45° (eased, no snapping jump) and the player stays near the screen centre. | unit |
| PLAY-012 | Given the player walks into the food storage (or the cave), then the roof above the player is cut away/hidden while the player is inside or at the entrance, and restored after leaving. | e2e |
| PLAY-013 | Given a device without touch, when the game runs, then no touch control (joystick, touch interact button) is visible. | e2e |
| PLAY-014 | Given a touch device, when the first touch happens, then the touch controls become visible and stay visible. | e2e |
| PLAY-015 | Given a touch in the left half, when the thumb drags up by 60 % of the stick radius, then the player walks away from the camera at 60 % of the surface speed (± 5 %); on release the player stops. | e2e |
| PLAY-016 | Given a horizontal swipe of ≥ 40 px in the right half, then the camera rotates by exactly one 45° step in the swipe direction. | e2e |
| PLAY-017 | Given the left thumb holds the joystick, when the right thumb swipes, then the player keeps walking and the camera rotates (multi-touch). | e2e |
| PLAY-018 | Given a touch session, then the page does not scroll, zoom or select text (touch-action none, no browser gestures). | e2e |
| PLAY-019 | Given the player walks straight into an info board, a sign, a bench and a tree, then her collision circle (r = 0.3 m) never overlaps their collision shapes and she slides along them. | unit |
| PLAY-020 | Given the player stands 1.5 m in front of an info board facing it, then the interactable is available and the interact button/hint is shown; standing behind it, beside it (> 60° off its facing) or facing away, it is not. | unit |
| PLAY-021 | Given two interactables are available, then the nearest one is offered. | unit |
| PLAY-022 | Given the player walks over grass tufts and flowers inside beds (non-solid ground decoration, §7), then she is not blocked. | unit |
| PLAY-023 | Given the player walks up to an info board and faces it, then the text panel opens by itself within 0.25–0.5 s without pressing a button; walking away (> 2.5 m) or turning away closes it within 0.5 s. | e2e |
| PLAY-024 | Given the player walks past a food box without facing it (passes by in < 0.25 s of availability), then no panel flashes open. | unit |
| PLAY-025 | Given a panel is open, then joystick/keyboard movement still moves the player and the panel does not cover the player on screen (portrait and landscape). | e2e |
| PLAY-026 | Given the player closed a panel manually, then it does not reopen while she stays in range; after leaving and returning, it opens again. | unit |
| PLAY-027 | Given the auto-opened food box panel, then the food is only taken after pressing ✋/interact, never by just walking up. | unit |
| PLAY-030 | Given a phone in portrait or landscape (412×892 / 892×412 CSS px, 1080×2340 device px) and any reading level and language, when an info board panel opens, then all its text including the food word is visible inside the panel without scrolling and the cap height is ≥ 3 % of the viewport height (proposal, Q-070; QA 2026-09-26). | e2e |
| PLAY-031 | Given every walkable cell centre of level 1 not covered by a prop and 8 walking directions, when the player walks 3 s, then her circle never overlaps a solid cell or prop shape, she is never trapped (can move ≥ 0.15 m in some direction afterwards) and she does not jitter (< 2 cm back-and-forth) while pushing against walls, props and corners (QA 2026-09-26). | unit |
| PLAY-028 | Given the player walks through the door into an enterable building, then its roof (and the blocking wall top) fades out within 0.3 s while she is inside and fades back in within 0.3 s after she leaves; other buildings keep their roofs. | e2e |
| PLAY-029 | Given the player inside a building, then everything inside (floor, props, animals) is visible from the default camera at every 45° rotation. | e2e |

## Open questions

- Q-018 answered: two-thumb touch controls (§3), no tap-to-walk.
- Q-064 Enclosure sign texts (signs not interactable until decided, §5). Q-065 Food storage interior (roof cut-away, PLAY-012). Q-069 answered: only boards, animals and gates of missions in scope are interactable (§5). Q-097 out-of-reach escaped animal comes towards the player (reach, §5).
- Q-001 Player role (affects intro).
- Q-024 Walking/running speed (PLAY-005 needs a value). Q-025 Carrying and movement during `pick_up`/`give`. Q-028 Skin/hair choice.
- Q-049 answered: high-angle follow camera (§2). Q-052 answered: 35° vertical FOV. Q-048 screen orientation. Q-051 dialogue close-up.
