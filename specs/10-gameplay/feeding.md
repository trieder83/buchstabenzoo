---
id: GAME-FEED
title: Food boxes, bamboo forest, carrying and putting down items
aspect: gameplay
module: feeding
status: draft
depends_on: [GAME-WORLD, CONT-READING, GAME-RESCUE, GAME-SAVE, GAME-HINT, GAME-PLAYER]
test_prefix: FEED
updated: 2026-09-27
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

## Putting an item down (user request 2026-09-27)

8. **Drop:** the child can put down the item in the hands — a food, the fish bowl (with water
   and fish if filled), the basket (with its treats) or the honey pot (GAME-EVENTS). Controls:
   a big **put-down button** (✋⬇ icon, no text) next to the carried-item icon in the HUD,
   shown only while something is carried; desktop key `G` (GAME-PLAYER §3 key table). The
   cart key (🔑) and the pocket are never dropped (nothing important can get lost).
   With the fish bowl in the hands and a food in the pocket, the bowl is put down and the
   pocket food moves to the hands.
9. **Where:** on the ground about 0.8 m in front of the player, standing on the visible
   surface (ground height, PLAY-035). If that spot is not free walkable ground — water,
   fence, wall, gate opening or its walkway (LAYOUT-032), enclosure inside, another item,
   food box — the nearest free walkable spot within 1.5 m is used; if there is none, nothing
   is dropped (gentle shake of the button). Not while sitting in a golf cart (GAME-CART).
10. **Lying items:** a dropped item stays where it was put (its model on the ground, readable
    icon above it when near), is saved and restored (GAME-SAVE), and is never taken by
    animals — animals do not eat dropped food. Dropping a food within 1.5 m of its own food
    box puts it back into the box instead (it disappears). At most **8** items lie in the
    zoo (user decision 2026-09-27); dropping a 9th food returns the oldest lying food to its box (never the bowl, basket
    or honey pot).
11. **Pick up again:** standing within 2 m of a lying item, the interact action (`E`/Space/
    Enter, touch action button with the item's icon) picks it up. If the hands are already
    full, the two items swap (the held one is put down on the same spot); the food + bowl
    pocket rule (GAME-RESCUE, Q-084) still applies.
12. **Following animals keep following** when their food is put down (they follow the child
    once they were shown the right food, GAME-RESCUE §5–6) — user decision 2026-09-27
    (Q-155). Showing food needs it in the hands.
    **Giving consumes:** an item **given** to an animal is always used up (user decision
    2026-09-27) — food when the animals enter their enclosure (§4; taken from the hands if
    carried), treats when given at the fence (GAME-GARDEN). Only *showing* food (to make
    animals follow) never consumes it.
13. **Hints:** a lying item the current mission needs (e.g. the fish bowl, the right food)
    is a hint target "pick up" (GAME-HINT priority 2, 📦/🐟 icon).

## Bamboo from the bamboo forest (user request 2026-09-27)

14. **Second source for bamboo:** besides the bamboo food box, the child can **cut bamboo**
    in a **bamboo forest** (every `[[scenery]]` of kind `bamboo` with `harvestable = true`;
    level 1: `bamboo_sw` at `loc_bamboo`). Each forest has **cut spots** along its edge that
    are reachable from walkable ground (level 1: 4 spots, proposal). Standing at a spot
    (within 1.5 m, facing it), the interact action (`E`/Space/Enter, touch action button with
    a bamboo icon) snaps off a stalk with the hands (short `pick_up` clip, leaves rustle — no
    knife or tool shown, child-safe; proposal Q-156) → the player carries the food `bamboo`,
    exactly like bamboo from the box (it works for the panda mission and is eaten at home).
    Food already in the hands is put down at the player's feet (§8–10 rules).
15. **Regrowing:** a cut spot shows a short stump and regrows in stages (stump → young shoot
    → full stalk) and can be cut again when full grown: **3 minutes of play time** (proposal
    Q-156; paused time does not count). Regrowth is saved (GAME-SAVE) and seeded-free
    (deterministic). A forest never disappears; uncut stalks stay dense so the thicket keeps
    its look, collider and its role as a hiding place (`loc_bamboo`).
16. **Reading still matters:** cutting bamboo does not replace the riddle — the child must
    still read the info board to know the panda wants bamboo and where it hides. The hint
    system (GAME-HINT) may offer the nearest ripe cut spot as an alternative to the food box
    only when the panda board was read.
17. **Art:** the bamboo forest model gets separate cut-spot stalk nodes with the stages
    `stalk_full`, `stalk_young`, `stump` (ART-ENVIRONMENT; the game swaps them by state).

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
| FEED-009 | Given the player carries hay, when `G` is pressed (desktop) or the put-down button is tapped, then hay lies on free walkable ground ≈ 0.8 m in front of her, standing on the surface, and she carries nothing. | unit + e2e |
| FEED-010 | Given the spot in front is water, a fence, a gate walkway or inside an enclosure, then the item lies on the nearest free walkable spot within 1.5 m, or is not dropped when none exists; in a golf cart nothing is dropped. | unit |
| FEED-011 | Given hay lying on the ground, when the player within 2 m interacts, then she carries hay; given she carries bamboo, then bamboo lies on that spot and she carries hay (swap). | unit |
| FEED-012 | Given the filled fish bowl with the goldfish and a food in the pocket, when dropped, then the bowl with water and fish lies on the ground, the food moves to the hands, and picking the bowl up again restores bowl + fish (food to the pocket). | unit |
| FEED-013 | Given zebras following and the player drops the grass, then the zebras keep following; animals never eat or take lying food. | unit |
| FEED-014 | Given a food dropped within 1.5 m of its own food box, then it goes back into the box; given 8 lying items, dropping another food returns the oldest lying food to its box and never removes the bowl, basket or honey pot. | unit |
| FEED-015 | Given items lying in the zoo, when saved and restored, then every item lies at the same place with the same content. | unit |
| FEED-016 | Given nothing carried or only the cart key, then no put-down button is shown and `G` does nothing. | e2e |
| FEED-017 | Given a full-grown cut spot of a harvestable bamboo forest and the player standing at it, when she interacts (desktop `E` or the touch action button), then she carries `bamboo` and the spot becomes a stump. | unit + e2e |
| FEED-018 | Given bamboo cut from the forest, when shown to the panda (right riddle state), then the panda follows exactly as with bamboo from the box, and it is eaten at home. | unit |
| FEED-019 | Given a cut spot, then it is not cuttable for 3 minutes of play time, shows stump → young shoot → full stalk, and is cuttable again afterwards; paused time does not count. | unit |
| FEED-020 | Given the player carries hay and cuts bamboo, then hay lies at her feet and she carries bamboo. | unit |
| FEED-021 | Given cut spots in different growth stages, when saved and restored, then every spot keeps its stage and remaining regrowth time. | unit |
| FEED-022 | Given every harvestable bamboo forest in the level data, then each cut spot has a walkable stand point within 1.5 m, and the forest stays solid and keeps its hiding place valid. | unit |
| FEED-023 | Given zebras following and the player carrying grass, when they enter their enclosure, then the grass is consumed (the player carries nothing); given the grass was put down earlier, the lying grass stays; showing food alone never consumes it. | unit |

## Open questions

- Q-032 Distractor design per reading level, Q-033 storage locked by `quest_key`?
- Q-065 Food storage interior (PoC: boxes outside along the facade).
- Q-025 How food is carried, Q-042 `give` clip vs. food not consumed, Q-034 food portions vs. one food at a time.
- Q-156 Bamboo forest: cut spots per forest, regrowth time, snapping with the hands (proposal).
- Q-155 answered: animals keep following when their food is put down; at most 8 lying items; giving consumes.
- Q-150 Food boxes standing in front of the (non-enterable) food storage door.
