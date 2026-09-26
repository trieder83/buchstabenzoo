---
id: GAME-PLAYER
title: Player character, camera and controls
aspect: gameplay
module: player
status: draft
depends_on: [PROD-VISION]
test_prefix: PLAY
updated: 2026-09-26
---

# Player character, camera and controls

## Behaviour

1. **Character choice.** On first start the child chooses between `player_girl` and
   `player_boy` by tapping one of two big 3D previews — no reading required. The choice is
   saved and can be changed in the settings.
2. **Third-person camera** follows behind and slightly above the player, avoids clipping
   through walls/enclosures, and can be rotated by dragging (touch) or mouse.
3. **Movement.**
   - Touch: virtual joystick bottom-left; alternatively tap-to-walk on the ground.
   - Desktop: WASD / arrow keys, mouse drag to rotate the camera.
4. **Interact.** When the player is near an interactable (animal, visitor, food box, sign,
   door), a large icon button appears; tapping it (or `E`/Space) interacts.
5. Interaction range: 2 m.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| PLAY-001 | Given a fresh profile, when the game starts, then the character choice screen shows `player_girl` and `player_boy` and no text is needed to choose. | e2e |
| PLAY-002 | Given `player_boy` was chosen, when the game restarts, then `player_boy` is loaded. | unit |
| PLAY-003 | Given the player at 1.9 m from a visitor, then the interact button is shown; at 2.1 m it is hidden. | unit |
| PLAY-004 | Given a wall between camera target and camera, then the camera moves closer so the player stays visible. | unit |
| PLAY-005 | Given joystick input forward for 1 s, then the player moves forward at walking speed ± 5 %. | unit |

## Open questions

- Q-018 Tap-to-walk, joystick, or both on touch devices?
- Q-001 Player role (affects intro).
- Q-024 Walking/running speed (PLAY-005 needs a value). Q-025 Carrying and movement during `pick_up`/`give`. Q-028 Skin/hair choice.
