---
id: GAME-GARDEN
title: Vegetable garden and treats
aspect: gameplay
module: garden
status: draft
depends_on: [GAME-FEED, GAME-FAMILY, GAME-LAYOUT, GAME-SAVE, CONT-READING]
test_prefix: GARD
updated: 2026-09-28
---

# Vegetable garden and treats

## Goal

In the back of the zoo there is a **vegetable garden** (*Gemüsegarten*) where the child can
**harvest carrots and potatoes** (user request 2026-09-26). They are **treats**
(*Leckerlis*): collected into a basket and later given to animals — especially for **care
feeding and the baby challenge** (GAME-FAMILY). It adds a small, calm collecting activity
between missions.

## Garden

1. **Place:** a fenced garden with a small gate in the back of level 1 (layout by the
   `zoo-level-designer`, GAME-LEVEL-1), later one per day level. Beds with rows of plants,
   a wheelbarrow, a watering can and a **garden sign** per bed with the vegetable name
   (read via the text panel — reading is still part of it: *Karotten*, *Kartoffeln*).
   **Level 1** (GAME-LEVEL-1 "Vegetable garden", data proposal Q-102): `garden_veg` in the
   strip x 6–9, z 36–45 between the panda enclosure and the river, 2 m gate on the south
   side (5.6 s from the bridge), 2 carrot beds × 3 plant spots and 2 potato beds × 2 plant
   spots along a 2 m path, wheelbarrow and watering can at the hedge. Plant spots are data
   (`[[plant_spot]]`: `id`, `kind`, `pos`, `start_stage`, `stand`). Animals never enter
   the garden; following animals wait at the gate (Q-102). Sign texts per reading level:
   Q-103.
2. **Plants:** carrot plants show green leaves above the soil (the orange top just
   visible); potato plants are small bushes with a few white flowers. From the 55° camera
   the two are clearly different.
3. **Harvest:** standing in front of a plant (GAME-PLAYER §5 rules) and pressing interact
   pulls it out (short `pick_up` animation, a little soil puff): a carrot comes out whole;
   for a potato plant 2–3 potatoes pop out. The plant spot becomes empty soil and
   **regrows** after a while (proposal: 3 minutes of play time, visibly growing in 3 steps).
4. **Basket (treat bag):** harvested treats go into a **basket** carried on the arm/back — a
   separate slot from the hands, so food boxes, the fish bowl and treats never conflict.
   Capacity: 6 treats (proposal). The HUD shows the basket with carrot/potato counts
   (icons + numbers, no reading needed).

## Treats

5. **Who likes what** (proposal — Q-100): **carrots**: zebra, elephant, giraffe, hippo,
   monkey, panda; **potatoes**: elephant, hippo, panda. Koala (only
   eucalyptus), lion and snow fox don't take garden treats (proposal: they get their own
   later). Realistic, and the treat list per animal is data.
   The info board facts may mention the favourite treat (reading hint).
6. **Giving (user report 2026-09-30):** giving works **directly at the animal**: a home animal
   is an interactable (`Target::Treat`) **at its own position** when the player is within 2 m
   facing it — inside the enclosure or at the fence (the nearest member of a pair). What is
   given: a liked treat from the basket, else the food in the hands if the animal eats it,
   else (refused) any treat / the carried food. Standing at the fence (≤ 3 m outside the
   rect) with something the animal likes, **the animal walks over and waits there**: when the
   child stands near the **feeding spot** (below), the group (male, female, baby) goes to its
   cells at 1 m/s without pausing; elsewhere at the fence it walks to the fence cells nearest
   to the child. It eats (`eat`, hearts, "Mmh,
   lecker!") if it likes it — **every member of the pair** reacts and turns to the child; a
   treat leaves the basket; **carried food stays in the hands** (boxes never run out,
   FEED-023) and brings no baby. Otherwise it sniffs and turns away (`refuse`, "Hmm, das mag
   ich nicht.", gentle RESC-005), the treat/food stays. With an empty basket and empty hands
   (or nothing liked) there is no prompt and no 🧭 hint. Treats are **not** used to rescue
   escaped animals.
6a. **Feeding spot** (`feed_spot = [x, z, w, d]` on an enclosure element, cells just inside
   the fence on the **gate side**, 2 cells wide; derived when absent): one empty cell (≥ 1 m)
   beyond the gate posts on either side, where the child can stand 0.9 m outside the fence
   with walkable cells around (≥ 1.5 m clear of hedges, water, boards). The treat hint
   (`hint-treat`, "Geh zum Tier und gib ihm etwas zu fressen") sends the child to its stand
   point. The child never walks into an enclosure (the gate is solid unless leading). A
   visible trough at the spot is not modelled yet (Q-248).
7. **Baby (GAME-FAMILY "Special food and babies", user request 2026-09-29):** a treat the
   animal likes makes it happy; given to a **male and female pair** at home, it makes them
   have a baby. This replaces the earlier 3-care-feedings / "one treat must be from the garden"
   proposal (Q-101 obsolete, Q-198 answered).
8. **Saving:** basket contents, garden plant states and regrow timers are saved (GAME-SAVE).

## Assets

9. New assets (concept sheet first — ART-PIPELINE): `garden_bed` (soil bed, 1 × 3 m
   modular), `carrot_plant` (in soil, 3 growth stages), `potato_plant` (3 stages),
   `carrot` and `potato` (items), `basket` (carried, with visible treats), `garden_fence` +
   `garden_gate` (low picket fence), `wheelbarrow`, `watering_can`, `garden_sign`.

## Implementation (2026-09-27)

- `[[garden]]`, `[[garden_bed]]`, `[[plant_spot]]` are loaded (`zoo_core::level`), joined
  with the levels; the fence lines block the grid steps across them (`Grid::step_open`),
  the beds, signs, fence pieces and tools are solid by their model footprints
  (`collision::footprint`, measured from the `kit_garden` meshes); the gate opens by itself
  within 2 m; following animals come to the gate and wait outside while the child is in
  the garden (proposal Q-102).
- `zoo_core::garden`: plant stages and regrowth (3 min, 3 visible steps), the basket (6
  treats), harvest (1 carrot / 2–3 seeded potatoes; a full basket refuses — what does not
  fit of a potato harvest stays in the soil), treats per animal (**proposal Q-100**, not
  answered yet), saved with the game (`SaveState::garden`).
- Interaction: ripe plants (🥕), the garden signs (reading panel: picture, word, sentence
  from klasse1 on — proposal Q-103), treats at the fence of an animal at home (🧺, the
  offered treat = the child's choice, default the first in the basket); the HUD shows the
  basket (🧺 + counts). Plants are drawn by stage (`carrot_plant_*`, `potato_plant_*`).
- Not implemented: GARD-006 (care feeding) — GAME-FAMILY does not exist in the engine yet;
  the "at least one treat" rule waits for Q-101.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| GARD-001 | Given a ripe carrot plant in front of the player, when she interacts, then the basket gains 1 carrot and the spot becomes empty soil. | unit |
| GARD-002 | Given a ripe potato plant, when harvested, then the basket gains 2–3 potatoes (seeded). | unit |
| GARD-003 | Given the basket holds 6 treats, then harvesting is refused with gentle feedback and the plant stays. | unit |
| GARD-004 | Given an empty spot, after 3 minutes of play time, then the plant is ripe again, passing 3 visible growth stages. | unit |
| GARD-005 | Given the zebra at home and a carrot in the basket, when the player gives it at the fence, then the zebra eats it and the carrot is removed; given a potato, it refuses and the potato stays. | unit |
| GARD-006 | Given a treat the pair likes (male and female at home), then giving it makes both happy and one baby appears once (GAME-FAMILY FAM-008); given a single animal, it is only happy. | unit |
| GARD-010 | Given the zebra at home and a liked carrot, when the player stands inside the enclosure within 2 m facing it (or at the fence after the zebra walked over), then `Target::Treat` is available at the animal, giving takes the carrot and both zebras of the pair turn to the child; with an empty basket and empty hands nothing is available. | unit |
| GARD-011 | Given the player carries the zebra's own food at home, when she gives it, then the zebra eats (`FoodEaten`), the food stays in the hands and no baby is born; a food it does not eat is refused (`FoodRefused`) and stays. | unit |
| GARD-012 | Given a pair at home and 4 carrots, when she walks up to them and gives, then both zebras react (eat/hearts), carrots leave the basket and exactly one baby appears. | e2e |
| GARD-013 | Given every enclosure of an animal that takes treats (zebra, elephant, giraffe, hippo, monkey, panda), then it has a feeding spot on the gate side: both cells in the home area, the stand point outside the fence and walkable, ≥ 1 cell from the gate posts. | unit |
| GARD-014 | Given a pair at the gate with one partner waiting far behind, when the child presses the gate, then nobody enters (gate press → `Waiting` message), the hint `partner:<animal>` sends her back to fetch it, and once it follows again both enter and the mission completes; afterwards the child standing on the gate cell can walk out. | unit |
| GARD-007 | Given the player carries a food box item or the fish bowl, then she can still harvest into the basket (separate slot). | unit |
| GARD-008 | Given a save with basket contents and growing plants, when restored, then both are unchanged. | unit |
| GARD-009 | Given the garden signs, then their texts come from Fluent per reading level and language (`garden-carrot`, `garden-potato`). | unit |

## Open questions

- Q-100 Which animals like which treat (and should koala/panda/snow fox get their own treats)?
- Q-101 Must at least one of the 3 care feedings be a garden treat?
- Q-102 garden in level 1 (gate opens within 2 m — proposal). Q-154 `basket`, `carrot`, `potato` models not yet listed in ART-ENVIRONMENT.
- Q-172 answered 2026-09-28: the basket is a separate slot (§4, GARD-007) and is never put down (GAME-FEED §8).
