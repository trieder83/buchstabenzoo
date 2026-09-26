---
id: GAME-SAVE
title: Saving and restoring progress
aspect: gameplay
module: save
status: draft
depends_on: [GAME-PLAYER, GAME-RESCUE, GAME-ANIMALS, GAME-FEED, GAME-MAP, CONT-L10N]
test_prefix: SAVE
updated: 2026-09-26
---

# Saving and restoring progress

## Goal

A reload (or closing and reopening the app) continues exactly where the child stopped —
progress **and** the last positions of the player and the animals (user decision
2026-09-26, answers Q-011 for the PoC: local saving on the device).

## What is saved

| Area | Saved state |
|---|---|
| Game | save format version, level id, RNG seed and RNG state, play time |
| Player | position (level coordinates), facing, carried food, current surface |
| Camera | yaw step, zoom distance |
| Animals | per animal: state (`escaped` / `following` / `in_enclosure`), chosen hiding place, position, facing, waiting flag, wandering (pause left, route) |
| Missions | per mission: started, info board read, completed; celebration already shown |
| World | opened barriers, gates, explored map cells (GAME-MAP), panels manually closed (not needed — transient) |
| Settings | language, reading level (already stored, CONT-L10N §6) |
| Last picks | hiding place per animal of the last new game (`zoo.picks.level-1`, kept when the save is deleted, so the next new game avoids them — Q-082) |

Transient things are **not** saved: an open text panel, running one-shot animations,
feedback bubbles, confetti. After a restore, idle/walk animations start from their pose.

## Behaviour

1. **Where:** on the device only (browser `localStorage`, later the Capacitor storage), one
   save slot per profile (profiles: Q-011 still open — one slot for the PoC).
2. **Format:** the game state is serialised **in Rust** (zoo-core, serde → JSON) with a
   `version` field; the host only stores/loads the string. Size ≤ 64 KB.
3. **When:** autosave after every progress event (food taken, animal follows, animal home,
   mission completed, barrier opened), every 5 s while the player moves, and on
   `visibilitychange` (hidden) / `pagehide`. Saving never causes a visible hitch (≤ 2 ms).
4. **Restore:** on start, a valid save for the current level is loaded before the first
   frame: player, camera, animals and missions appear exactly as saved (following animals
   stand behind the player). Positions that are not walkable any more (level data changed)
   are moved to the nearest walkable cell; animals `in_enclosure` are placed inside their
   enclosure.
5. **Invalid saves:** unknown version, other level, or broken data → start a new game
   (never crash); a newer migration may convert old versions later.
6. **New game:** the settings menu has a "new game" button (icon + confirm with a big
   yes/no icon pair, no reading needed) that deletes the save and restarts the level.
7. Determinism: restoring a save and then applying the same inputs gives the same result as
   never having reloaded (RNG state is part of the save).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| SAVE-001 | Given any game state, when it is saved and restored, then the restored state equals the original (player, camera, animals, missions, world, RNG). | unit |
| SAVE-002 | Given the zebra is `following`, when the page is reloaded, then after restore the zebra follows the player again, standing behind her. | e2e |
| SAVE-003 | Given the zebra mission is completed, when the page is reloaded, then the zebra is in its enclosure, the mission counts as completed and the celebration does not play again. | e2e |
| SAVE-004 | Given the player carries grass and stands at position P facing F, when the page is reloaded, then she is at P (± 0.05 m) facing F and still carries grass. | e2e |
| SAVE-005 | Given a save with an unknown version or broken JSON, when the game starts, then a new game starts and no error is shown. | unit |
| SAVE-006 | Given a saved position that is no longer walkable, when restoring, then the player is placed on the nearest walkable cell. | unit |
| SAVE-007 | Given a restored game and a recorded input sequence, then the resulting state equals the state of the same inputs without a reload. | unit |
| SAVE-008 | Given "new game" is confirmed in the settings, then the save is deleted and the level starts fresh. | e2e |
| SAVE-009 | Given the player walks for 6 s without progress events, then at least one autosave happened; saving takes ≤ 2 ms. | unit |

## Open questions

- Q-011 Profiles / multiple children on one device, cloud sync — later.
