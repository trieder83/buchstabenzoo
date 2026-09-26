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
6. **Taking a box (decision for M4 — simplest reading-first flow):** interacting with an
   available food box (GAME-PLAYER §5: within 2 m, on the label side, facing it) opens the
   text panel with the box **label** (picture + word on `kiga`, word only otherwise) and a
   big **take** button (hand icon, no reading needed). Pressing take — or interacting again
   while the panel is open — makes the player carry that food (§3). Closing the panel
   without taking changes nothing. The carried food is shown in the HUD (icon + word).
7. **Box positions** come from the level data (`[[food_box]]` entries: food id, level
   position of the box centre, label facing); every food has exactly one box. *PoC:* the
   storage interior is not modelled yet, so level 1 places the 10 boxes in a row along the
   storage's south facade (Q-065).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| FEED-001 | Given reading level `kiga`, then every food box label shows a picture and one word. | unit |
| FEED-002 | Given reading level `klasse1`+, then no food box label shows a picture. | unit |
| FEED-003 | Given the player carries hay, when taking the bamboo box, then the player carries bamboo and hay is back. | unit |
| FEED-004 | Given the player shows grass to zebras, then the player still carries grass. | unit |
| FEED-005 | Given every animal's correct food, then a food box with that food exists in the storage. | unit |
| FEED-006 | Given the player has taken food from the grass box 10 times, when taking it again, then the player carries grass (boxes never run out). | unit |
| FEED-007 | Given the player stands in front of the grass box facing it, when interacting, then the text panel with the grass label opens and the player carries nothing yet; when taking, the player carries grass. | unit |
| FEED-008 | Given the level data, then every food has exactly one food box, every box stands on walkable ground next to the food storage, and a walkable standing point within 2 m in front of each box's label exists. | unit |

## Open questions

- Q-032 Distractor design per reading level, Q-033 storage locked by `quest_key`?
- Q-065 Food storage interior (PoC: boxes outside along the facade).
- Q-025 How food is carried, Q-042 `give` clip vs. food not consumed, Q-034 food portions vs. one food at a time.
