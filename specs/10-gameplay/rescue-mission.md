---
id: GAME-RESCUE
title: Rescue mission — core loop
aspect: gameplay
module: rescue-mission
status: draft
depends_on: [PROD-VISION, GAME-ANIMALS, GAME-FEED, GAME-WORLD, CONT-READING, CONT-MATH]
test_prefix: RESC
updated: 2026-09-26
---

# Rescue mission — core loop

## Goal

The animals have escaped and the enclosures are empty. The child brings every animal back.
Each animal is one **rescue mission**; the game goal is reached when every enclosure is
occupied again.

## The loop (example: zebras)

| # | Step | Reading involved | Result |
|---|---|---|---|
| 1 | Go to the **empty enclosure** of the zebras. | Enclosure sign | Mission `zebra` starts. |
| 2 | Read the **info board** at the enclosure: a simple **riddle** tells where the zebras went, and a text says what they eat. | Riddle + food text at the child's reading level | Child understands the riddle, e.g. *"Ich habe Durst. Ich gehe dorthin, wo das Wasser fließt."* → river. |
| 3 | Go to the **food storage** and choose the right **food box** by reading its label. | Food box labels | Player carries the right food. |
| 4 | **Search** the zoo at the place the riddle describes (e.g. drinking at the river). | Understanding the riddle | Zebras found. |
| 5 | **Show the food** to the zebras. Right food → they **follow** the player. Wrong food → they are not interested. | — | Zebras follow. |
| 6 | **Lead** them back into their enclosure. | Enclosure signs | Zebras enter, eat, play `happy`. Mission complete. |

## Behaviour

1. At game start every enclosure is empty and every animal is at one of its **hiding places**
   (GAME-ANIMALS). Hiding place is picked per playthrough with the seeded RNG.
2. **Reading comprehension is the core.** The info board holds a **location riddle** for the
   animal's actual hiding place. The riddle describes the place indirectly (what the animal
   does there, what is there) — it never names the place directly on `klasse1`+.
   Understanding the riddle is enough to find the animal; guessing by running everywhere
   should be slower than reading.
3. Optional **math tasks** (CONT-MATH) can be placed on the way, e.g. a number lock on the
   food storage or "how many food portions for 3 zebras?". They never block a mission for a
   child whose math level is not set (Q-034).
4. A mission starts when the player first reads the info board of that enclosure. Animals can
   also be found before reading the board; they still only follow when shown the right food.
5. Only one group of animals follows the player at a time (Q-030); what happens when food
   is shown to a second group meanwhile is open (Q-041).
6. Following animals keep a short distance behind the player, walk around obstacles, and wait
   if the player gets too far away (> 15 m); they follow again when the player is back
   within 5 m — they never get lost again.
7. Leading following animals into the **wrong** enclosure: they stop at the gate and refuse;
   the correct enclosure's sign is not revealed — the child has to read.
8. When the animals enter their own enclosure, the carried food is used up, they eat
   (`eat`, then `happy`), the enclosure gate closes, and the mission is complete.
9. When all missions are complete, the final celebration plays (Q-031).
10. A mission may add a quest step (e.g. `quest_monkey_baby`, `quest_key` in GAME-QUESTS).
11. *Presentation (PoC M4):* escaped animals play `drink` at the river (else `idle`);
    following animals walk behind the player; events map to clips — `not_interested` and
    `refuse` → `refuse`, `started_following` → `happy`, entering the enclosure → `eat` then
    `happy`; mission complete shows a short celebration (confetti) and
    `mission-<animal>-home`. Interacting at an enclosure gate while leading animals is the
    same as walking into it (GAME-PLAYER §5).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| RESC-001 | Given a new game, then every enclosure is empty and every animal is at one of its hiding places. | unit |
| RESC-002 | Given seed S, then every animal's hiding place is the same on every run. | unit |
| RESC-003 | Given any animal, hiding place, reading level and language, then a location riddle exists for that hiding place. | unit |
| RESC-004 | Given the player carries grass and shows it to the zebras, then the zebras' state becomes `following`. | unit |
| RESC-005 | Given the player carries bamboo and shows it to the zebras, then the zebras stay `escaped` and emit `not_interested`. | unit |
| RESC-006 | Given zebras `following`, when the player is 20 m away, then the zebras wait; when the player returns within 5 m, they follow again. | unit |
| RESC-007 | Given zebras `following`, when the player enters the panda enclosure gate, then the zebras stop at the gate and emit `refuse`. | unit |
| RESC-008 | Given zebras `following`, when the player enters the zebra enclosure, then the zebras are `in_enclosure`, the carried food is consumed, and mission `zebra` is complete. | unit |
| RESC-009 | Given all missions complete, then the game emits `all_animals_home`. | unit |
| RESC-010 | Given reading level `klasse1`, when the player plays the zebra mission end to end (read board → pick grass → find at river → lead home), then the mission completes. | e2e |
| RESC-011 | Given reading level `klasse1`+, then no location riddle contains the name of its hiding place, i.e. the place word shown on `kiga` for that hiding place (CONT-MISSIONS), in the same language (matching rule: Q-039). | unit |
| RESC-012 | Given mission `zebra` not started, when the player reads the zebra info board for the first time, then mission `zebra` is started. | unit |
| RESC-013 | Given the zebra info board has not been read, when the player shows grass to the zebras, then the zebras become `following`. | unit |

## Open questions

- Q-030 How many animals per species (herd follows as a group)?
- Q-034 Math tasks: separate math level, where they appear, can they block?
- Q-031 What happens after all animals are home (end, sandbox, next zoo)?
- Q-022/Q-023 How missions map to levels and barriers.
- Q-020 Several missions active at once? Q-041 Food shown to a second group while one follows.
- Q-036 Goldfish transport, Q-039 riddle/place-word rule, Q-040 monkey baby quest.
