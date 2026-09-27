---
id: GAME-GARDEN
title: Vegetable garden and treats
aspect: gameplay
module: garden
status: draft
depends_on: [GAME-FEED, GAME-FAMILY, GAME-LAYOUT, GAME-SAVE, CONT-READING]
test_prefix: GARD
updated: 2026-09-27
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
6. **Giving a treat:** at an animal that is **at home** (in its enclosure), standing at the
   fence/feeding trough, interact with a treat selected → the animal comes over, eats it
   (`eat`, happy hearts) if it likes it; otherwise it sniffs and turns away (`refuse`), the
   treat stays in the basket. Treats are **not** used to rescue escaped animals (the rescue
   still needs the reading-based food box).
7. **Baby challenge (GAME-FAMILY):** a treat the animal likes counts as a **care feeding**
   like the correct food from the storage (one per play session). Proposal: the pair needs
   its 3 care feedings as before, and at least one of them must be a treat from the garden
   (so the garden matters) — Q-101.
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
| GARD-006 | Given a treat the pair likes, then giving it counts as one care feeding (max one per session, GAME-FAMILY FAM-003). | unit |
| GARD-007 | Given the player carries a food box item or the fish bowl, then she can still harvest into the basket (separate slot). | unit |
| GARD-008 | Given a save with basket contents and growing plants, when restored, then both are unchanged. | unit |
| GARD-009 | Given the garden signs, then their texts come from Fluent per reading level and language (`garden-carrot`, `garden-potato`). | unit |

## Open questions

- Q-100 Which animals like which treat (and should koala/panda/snow fox get their own treats)?
- Q-101 Must at least one of the 3 care feedings be a garden treat?
- Q-102 garden in level 1 (gate opens within 2 m — proposal). Q-154 `basket`, `carrot`, `potato` models not yet listed in ART-ENVIRONMENT.
