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
   (GAME-ANIMALS). **Discovery** (user decision 2026-09-26): every animal has **at least 3
   candidate hiding places spread over the reachable map of its level**, and one is picked
   per playthrough with the seeded RNG, so the animal is not always in the same place.
   Rules for the pick: no two animals share a hiding place; each chosen place is far from
   its own enclosure (not visible from its info board — LAYOUT-L1-006); the picks are spread
   over the map (the chosen places of a level's animals are ≥ 12 m apart). The chosen place
   is saved (GAME-SAVE) and does not change on reload. Every candidate place has its own
   location riddle per reading level and language (CONT-MISSIONS).
   *Implementation (M5a):* the pick follows Q-082 (answered): seeded uniform pick, redraw
   of the whole set while two spots are < 12 m apart (max 64, then the first valid
   combination), and a new game avoids each animal's place of the previous game (the host
   keeps the last picks). A new game gets a random seed; `?seed=N` in the URL fixes it
   (tests). Only the missions in scope of the level are interactable (Q-069 answered:
   level 1 = zebra, hippo, panda).
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
11. *Presentation (PoC M4, generalised in M5a):* every animal of the level is drawn with
    its model `assets/models/animals/<id>.glb` (a placeholder box animal if it is missing or
    fails to load). Resting clip = the hiding place's `pose` while escaped (`drink` at the
    river, `eat`, `sleep`, `idle`), `swim` while in water (pond, pool), else `idle`;
    locomotion = `walk`, or `swim` in water; a clip the model lacks falls back to `idle` /
    `walk` (e.g. no `sleep` → `idle`). Walk playback = speed ÷ the authored speed of
    `animal_anims.toml`, clamped (ART-RIG). Swimming animals sink by their water line
    (hippo 0.9 m, `zoo_core::animals::swim_sink_m`) so only back, eyes and ears show.
    Following animals walk behind the player; events map to clips — `not_interested` and
    `refuse` → `refuse`, `started_following` → `happy`, entering the enclosure → `eat` then
    `happy`; mission complete shows a short celebration (confetti) and
    `mission-<animal>-home`. Interacting at an enclosure gate while leading animals is the
    same as walking into it (GAME-PLAYER §5). Arriving animals step onto the cell just
    inside the gate first, then wander at home (GAME-ANIMALS).


## Animals that cannot walk behind the player — the goldfish bowl

User decision 2026-09-26 (answers Q-036): a goldfish cannot follow the player over land, so
its rescue has extra steps with a **big glass bowl** (*Goldfischglas*, `fish_bowl`):

1. **Find the bowl:** a big empty glass bowl stands somewhere in the goldfish's level (a
   findable place, e.g. the zookeeper house; the info board gives a hint that a bowl is
   needed). Interact → the player carries it with both hands (`socket_carry`).
2. **Fill it with water:** at any water source of the level (tap/pump, pond, river edge)
   interact → the bowl is filled (visible water in the glass). An empty bowl never works.
3. **Food:** the goldfish's food (`Fischfutter`, read from the food box label as usual).
   While carrying the bowl the player can also hold one food (the food goes into the
   **pocket**, shown in the HUD — proposal, Q-084), so bowl + food can be carried together.
4. **Find the goldfish:** it is **in a river** (its hiding places are river/stream spots of
   its level — riddles as usual; the old fountain place is replaced, CONT-MISSIONS).
5. **Feed it:** interact at the riverbank with fish food → the goldfish swims to the bank
   and, **if the bowl is filled**, **jumps into the bowl** (splash, `happy`). Feedback
   without bowl: it swims happily but cannot come along ("Ich brauche ein Glas mit Wasser!");
   with an empty bowl: "Im Glas ist ja kein Wasser!".
6. **Carry it home:** the player walks back carrying the bowl with the fish (walking speed
   × 0.9, careful — proposal, Q-084) and puts it at the goldfish's home (aquarium/pond
   enclosure): interact → the fish jumps in, `happy`, mission complete.
7. The bowl, its water and the fish in it are part of the save (GAME-SAVE). Putting the bowl
   down anywhere (interact on free ground) is allowed; the fish stays safe in it.
8. The mechanic is generic (`carry container` + `fill` + `animal enters container`) so later
   swimming animals can reuse it; data per animal says which container it needs.

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
| RESC-017 | Given level 1 as built, then every info board and animal that is interactable has its texts (location riddle for every reading level, home message) in `de` and `en` — no panel, bubble or celebration shows a raw Fluent key (Q-069 answered; QA 2026-09-26). | unit |
| RESC-014 | Given 1 000 different seeds, then every animal of level 1 is placed at each of its ≥ 3 candidate hiding places at least once, no two animals ever share a place, and chosen places of one playthrough are ≥ 12 m apart. | unit |
| RESC-015 | Given any seed and reading level, then the info board shows the riddle of the hiding place actually chosen for that playthrough. | unit |
| RESC-016 | Given a saved game, when it is restored, then every animal is still at (or wandering around) the same chosen hiding place. | unit |
| RESC-018 | Given the goldfish mission, when the player has no bowl and feeds the fish, then it stays in the river with the "needs a bowl" feedback. | unit |
| RESC-019 | Given the player carries an empty bowl and feeds the fish, then it stays with the "no water" feedback; after filling at a water source and feeding again, it jumps into the bowl. | unit |
| RESC-020 | Given the player carries the bowl, then she can additionally hold one food (pocket) and the HUD shows both. | unit |
| RESC-021 | Given the fish is in the bowl, when the player puts the bowl at the goldfish home, then the fish is in its enclosure and the mission completes. | unit |
| RESC-022 | Given a save with a carried filled bowl and the fish in it, when restored, then bowl, water and fish are unchanged. | unit |
| RESC-023 | Given the player carries the bowl with the fish, then her speed is 0.9 × the surface speed. | unit |
| RESC-024 | Given the last picks of the previous game (host key `zoo.picks.level-1`), when a new game starts with any seed, then no animal gets the hiding place it had in the previous game (§1 implementation note, Q-082 answered). | e2e |
| RESC-025 | Given a level whose `[level] missions` lists only some of its enclosures' animals, then only the info boards, animals and gates of the listed missions are interactable; without the field every animal is in scope (§1, Q-069 answered, GAME-LAYOUT). | unit |

## Open questions

- Q-030 How many animals per species (herd follows as a group)?
- Q-034 Math tasks: separate math level, where they appear, can they block?
- Q-031 What happens after all animals are home (end, sandbox, next zoo)?
- Q-022/Q-023 How missions map to levels and barriers.
- Q-020 Several missions active at once? Q-041 Food shown to a second group while one follows.
- Q-036 Goldfish transport, Q-039 riddle/place-word rule, Q-040 monkey baby quest.
- Q-097 Out-of-reach escaped animal comes towards the player (proposal, GAME-ANIMALS). Q-069 (answered) only missions in scope are interactable. Q-082 (answered) picking rule.
