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
3. **Movement.**
   - Touch: virtual joystick bottom-left; alternatively tap-to-walk on the ground.
   - Desktop: WASD / arrow keys, mouse drag to rotate the camera.
4. **Interact.** When the player is near an interactable (animal, visitor, food box, sign,
   door), a large icon button appears; tapping it (or `E`/Space) interacts. Reading
   interactions (info board, food box label, sign) open a **close-up text panel** with
   large text and read-aloud button (CONT-READING); closing it returns to the camera.
5. Interaction range: 2 m.
6. **Ground speed.** The player can walk on paths and on grass; on grass the player is
   slower (user decision 2026-09-26). Speed on grass = path speed × `grass_speed_factor`
   (proposal: 0.7). Path walking speed: Q-024 (proposal 1.4 m/s). Cells covered by solid
   elements (fences, water, hedges, buildings) are not walkable (GAME-LAYOUT).

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

## Open questions

- Q-018 Tap-to-walk, joystick, or both on touch devices?
- Q-001 Player role (affects intro).
- Q-024 Walking/running speed (PLAY-005 needs a value). Q-025 Carrying and movement during `pick_up`/`give`. Q-028 Skin/hair choice.
- Q-049 answered: high-angle follow camera (§2). Q-052 answered: 35° vertical FOV. Q-048 screen orientation. Q-051 dialogue close-up.
