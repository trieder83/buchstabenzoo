---
id: GAME-QUESTS
title: Quests and riddles
aspect: gameplay
module: quests
status: draft
depends_on: [GAME-WORLD, GAME-RESCUE, CONT-READING]
test_prefix: QUEST
updated: 2026-09-26
---

# Quests and riddles

## Behaviour

1. A quest is data: id, steps, required items, hints, reward. Quest logic lives in `zoo-core`.
2. Every riddle and hint exists for every reading level and every language (CONT-L10N).
3. Whether several quests (and rescue missions) can be active at the same time is open
   (Q-020).
4. Quests are optional steps attached to rescue missions (GAME-RESCUE §10); the main
   goal is the rescue of all animals (GAME-RESCUE). Location riddles of rescue missions
   are specified in GAME-RESCUE / CONT-MISSIONS, not here.

## Quests

### Find the key on the pirate ship (`quest_key`) — depends on Q-033

If the food storage is locked (Q-033): A visitor/zookeeper says the key was taken to the pirate ship.
The child explores the ship and solves a reading riddle to find where the key is hidden.

### Find the lost monkey baby (`quest_monkey_baby`) — conflicts with the escape premise, Q-040

1. The monkey mother is sad; the baby is missing.
2. The child asks visitors. Each visitor gives an **indirect** hint
   (e.g. "Ich habe etwas Kleines bei den gestreiften Tieren gesehen" → zebra enclosure).
3. Hints can be wrong-looking but never false; at least one hint per reading level leads directly.
4. The child finds the baby at the hinted place and brings it back.
5. The hiding place is chosen from a list per playthrough (seeded RNG).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| QUEST-001 | Given seed S, then `quest_monkey_baby` always picks the same hiding place. | unit |
| QUEST-002 | Given any hiding place, then at least one visitor hint refers to it at each reading level. | unit |
| QUEST-003 | Given the baby is brought to the monkey mother, then the quest completes and both play `happy`. | unit |
| QUEST-004 | *(only if Q-033 keeps the lock)* Given `quest_key` is completed, then the player holds `key`. | unit |
| QUEST-005 | Given every riddle, then text exists for all reading levels and all enabled languages. | unit |

## Open questions

- Q-015 Pirate NPC, Q-017 pirate ship location, Q-020 parallel quests?
- Q-033 Is `quest_key` needed at all? Q-040 How `quest_monkey_baby` fits the escape premise.
