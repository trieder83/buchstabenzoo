---
id: GAME-WORLD
title: Zoo world
aspect: gameplay
module: world
status: draft
depends_on: [PROD-VISION]
test_prefix: WORLD
updated: 2026-09-26
---

# Zoo world

## Behaviour

1. The zoo consists of the areas listed in ART-ENVIRONMENT (entrance, enclosures, food
   storage, pirate ship) and the hiding places of the rescue missions (`loc_*`,
   CONT-MISSIONS).
2. Every enclosure has an `enclosure_sign` with the animal name; on `kiga` the sign also
   shows the animal silhouette (cf. `art/environment/style_frame/style_frame.png`).
3. Whether the food storage is locked until the player finds the `key` on the
   `pirate_ship` (`quest_key`) is open (Q-033, GAME-FEED §5). WORLD-002/003 apply only if
   Q-033 keeps the lock.
4. World content (areas, positions, which animal/food belongs where) is data, loaded by
   `zoo-core` — not hard-coded in render code.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| WORLD-001 | Given reading level `kiga`, then every enclosure sign shows name + silhouette; on `klasse1`+ only the name. | unit |
| WORLD-002 | *(only if Q-033 keeps the lock)* Given the player has no key, when interacting with the food storage door, then it stays closed and a hint points to the pirate ship. | unit |
| WORLD-003 | *(only if Q-033 keeps the lock)* Given the player holds the key, when interacting with the food storage door, then it opens. | unit |
| WORLD-004 | Given the world data file, then every enclosure references an existing animal id. | unit |

## Open questions

- Q-006 Open world vs. areas unlocked in sequence.
- Q-017 Pirate ship location.
- Q-033 Food storage locked by `quest_key`?
