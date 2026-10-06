---
id: GAME-SAVE
title: Saving and restoring progress
aspect: gameplay
module: save
status: draft
depends_on: [GAME-PLAYER, GAME-RESCUE, GAME-ANIMALS, GAME-FEED, GAME-MAP, CONT-L10N]
test_prefix: SAVE
updated: 2026-10-06
---

# Saving and restoring progress

## Goal

A reload (or closing and reopening the app) continues exactly where the child stopped —
progress **and** the last positions of the player and the animals (user decision
2026-09-26, answers Q-011 for the PoC: local saving on the device).

## What is saved

| Area | Saved state |
|---|---|
| Game | save format version (2 since M5b, **3 with the golf carts**, GAME-CART "Save"), level id (`zoo` = the joined levels), RNG seed and RNG state, play time |
| Player | position (level coordinates), facing, carried food, current surface |
| Camera | yaw step, zoom distance |
| Animals | per animal: state (`escaped` / `following` / `in_enclosure`), chosen hiding place, position, facing, waiting flag, wandering (pause left, route) |
| Missions | per mission: started, info board read, completed; celebration already shown |
| World | opened barriers (= unlocked levels), gates, the fish bowl (position, carried, water, fish, put down by the child), explored map cells (GAME-MAP), panels manually closed (not needed — transient) |
| Lying items and bamboo | foods lying on the ground (`lying`: food, position, surface height; oldest first — GAME-FEED §10, FEED-015) and the regrowth time left per bamboo cut spot (`bamboo`: spot id, seconds; missing = full grown — GAME-FEED §15, FEED-021); optional fields, no version change |
| Time of day | `daytime` (GAME-NIGHT §8, NIGHT-008): phase (`day` / `dusk` / `night` / `sleeping` / `morning`), time in the phase, pending dusk, levels whose nightfall happened, levels whose barriers open the next morning, nights; a save made while sleeping restores as the finished morning; saves without it are day (no version change — the field is optional) |
| Golf carts and key | (version 3, all `#[serde(default)]`) `carts`: per cart `id`, `x`, `z`, `yaw`; `seated`: cart id or none; `has_cart_key`, `key_box_open`, `note_read`, `key_box_tries` (u8), `note_aid`; `math_level` (`mathe1`…`mathe5`, default `mathe1`) — GAME-CART "Save", CONT-MATH |
| Settings | language, reading level (already stored, CONT-L10N §6), math level (`zoo.mathLevel`, host setting, GAME-CART rule 21), camera view `zoo` / `first_person` (`zoo.view`, GAME-CAMERA-VIEWS 9; look-around is never stored) |
| Last picks | hiding place per animal of the last new game (`zoo.picks`, M5b; formerly `zoo.picks.level-1`; kept when the save is deleted, so the next new game avoids them — Q-082) |

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
8. **One save for the joined zoo (M5b, 2026-09-26):** format **version 2** with
   `level_id = "zoo"` (levels 1–3 joined, GAME-LAYOUT "Joining levels"): opened barriers
   (= unlocked levels), all ten animals (a pair's second animal as `member = 1`,
   GAME-FAMILY), missions, and the fish bowl (`bowl`: position, carried, water, fish —
   RESC-022). **Migration:** a version-1 save (level 1 only, M4b/M5a) is accepted by the zoo
   that contains level 1 — its level-1 state is kept, the later levels start with fresh
   seeded picks; `all_home` of a v1 save is recomputed. The host slot moves from
   `zoo.save.level-1` to `zoo.save` (the old slot is read once and removed); the last picks
   from `zoo.picks.level-1` to `zoo.picks` (all animals).

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
| SAVE-010 | Given a version-2 save of the joined zoo (levels 2–3 unlocked, the bowl carried with the fish), when it is restored, then it equals the original; given a version-1 save of level 1, then the zoo restores its level-1 state and starts levels 2–3 fresh; a v2 zoo save never loads into a single level. | unit |
| SAVE-011 | Given the web host after M5b, when the game autosaves, then the save is stored under `zoo.save` with version 2 and a reload restores the unlocked levels, completed missions and the player position; given an old `zoo.save.level-1` slot and no `zoo.save`, then it is read once, migrated and removed (§8). | e2e |
| SAVE-012 | Given a version-3 save with moved carts, the player seated in `cart_l2`, `has_cart_key`, `key_box_open`, `note_read`, `key_box_tries = 2` and `math_level = mathe3`, when restored, then the zoo equals the original (and a following recorded input sequence gives the same result, SAVE-007); `has_cart_key` without `key_box_open` is repaired to open. | unit |
| SAVE-013 | Given a version-2 save (no cart fields), when restored, then the carts stand on their parking poses, no key, closed box, `note_read = false`, `key_box_tries = 0`, `math_level = mathe1`, everything else unchanged; a version above 3 starts a new game (SAVE-005). | unit |

## Open questions

- Q-011 Profiles / multiple children on one device, cloud sync — later.
