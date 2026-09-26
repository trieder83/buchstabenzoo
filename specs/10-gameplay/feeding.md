---
id: GAME-FEED
title: Food boxes and carrying food
aspect: gameplay
module: feeding
status: draft
depends_on: [GAME-WORLD, CONT-READING]
test_prefix: FEED
updated: 2026-09-26
---

# Food boxes and carrying food

## Goal

The child finds the right food by **reading the label on the food box**. Food is used to
make an escaped animal follow (GAME-RESCUE) and is eaten when it arrives home.

## Behaviour

1. Food boxes stand in the food storage. Each `food_box` holds one food and has a label with
   the food's name; label form depends on the reading level:

   | Reading level | Label |
   |---|---|
   | `kiga` | picture + word |
   | `klasse1` | word only |
   | `klasse2` | word only; boxes with similar-looking words side by side (e.g. *Heu* / *Hai*-Futter) (Q-032) |
   | `klasse3` | word only, plus distractor boxes |

2. Boxes are closed — the food is not visible, so the label must be read.
3. The player carries one food at a time. Taking another box puts the current food back.
4. Food is not consumed by showing it to an animal — only when the animals enter their
   enclosure (GAME-RESCUE §8). Food boxes never run out.
5. Whether the food storage is locked at the start (`quest_key`) is open (Q-033).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| FEED-001 | Given reading level `kiga`, then every food box label shows a picture and one word. | unit |
| FEED-002 | Given reading level `klasse1`+, then no food box label shows a picture. | unit |
| FEED-003 | Given the player carries hay, when taking the bamboo box, then the player carries bamboo and hay is back. | unit |
| FEED-004 | Given the player shows grass to zebras, then the player still carries grass. | unit |
| FEED-005 | Given every animal's correct food, then a food box with that food exists in the storage. | unit |
| FEED-006 | Given the player has taken food from the grass box 10 times, when taking it again, then the player carries grass (boxes never run out). | unit |

## Open questions

- Q-032 Distractor design per reading level, Q-033 storage locked by `quest_key`?
- Q-025 How food is carried, Q-042 `give` clip vs. food not consumed, Q-034 food portions vs. one food at a time.
