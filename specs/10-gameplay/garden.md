---
id: GAME-GARDEN
title: Vegetable garden, fruit garden and treats
aspect: gameplay
module: garden
status: draft
depends_on: [GAME-FEED, GAME-FAMILY, GAME-LAYOUT, GAME-SAVE, CONT-READING]
test_prefix: GARD
updated: 2026-10-03
---

# Vegetable garden, fruit garden and treats

## Goal

In the back of the zoo there is a **vegetable garden** (*Gemüsegarten*) where the child can
**harvest carrots and potatoes** (user request 2026-09-26). They are **treats**
(*Leckerlis*): collected into a basket and later given to animals — especially for **care
feeding and the baby challenge** (GAME-FAMILY). It adds a small, calm collecting activity
between missions.

**Fruit garden (user request 2026-10-01):** level 3 (the level with the monkey) has a second,
smaller garden, the **fruit garden** (*Obstgarten*, `garden_fruit`), where the child picks
**apples** and **oranges** from small fruit trees. They are treats like carrots and potatoes
(same basket, same giving rules); the monkey likes both. Level 1 keeps the vegetable garden
only; the two gardens never share a level.

## Garden

1. **Place:** a fenced garden with a small gate in the back of level 1 (layout by the
   `zoo-level-designer`, GAME-LEVEL-1), later one per day level. Beds with rows of plants,
   a wheelbarrow, a watering can and a **garden sign** per bed with the vegetable name
   (read via the text panel — reading is still part of it: *Karotten*, *Kartoffeln*).
   **Level 1** (GAME-LEVEL-1 "Vegetable garden", data proposal Q-102): `garden_veg` in the
   strip x 6–9, z 36–45 between the panda enclosure and the river, 2 m gate on the south
   side (5.6 s from the bridge), 2 carrot beds × 3 plant spots and 2 potato beds × 2 plant
   spots along a 2 m path, wheelbarrow and watering can at the hedge. **Level 3**
   (GAME-LEVEL-3 "Fruit garden", Q-320): `garden_fruit`, a 10 × 3 m fenced garden whose 2 m
   gate on the north side opens onto `path_l3_ne` (a street, rule 8), an **apple bed**
   (2 apple trees) and an **orange bed** (2 orange trees) behind a garden path along the
   south edge, a sign per bed (*Äpfel*, *Orangen*) and a wheelbarrow. Plant spots are data
   (`[[plant_spot]]`: `id`, `kind`, `pos`, `start_stage`, `stand`). Animals never enter
   the garden; following animals wait at the gate (Q-102). Sign texts per reading level:
   Q-103.
2. **Plants:** carrot plants show green leaves above the soil (the orange top just
   visible); potato plants are small bushes with a few white flowers. From the 55° camera
   the two are clearly different. **Fruit trees** (level 3) are small dwarf trees, ~1.5 m
   high, on a soil bed like the vegetables: a **sprout** (a thin sapling), a **young** tree
   (green crown, no fruit) and a **ripe** tree with 4–5 red apples (apple tree: round light
   crown) or orange fruits (orange tree: dark glossy crown). From the 55° camera apple tree
   and orange tree differ in crown and fruit colour, and a ripe tree is clearly different
   from a young one. The fruit is modelled on the tree only (no separate fruit item in the
   world).
3. **Harvest:** standing in front of a plant (GAME-PLAYER §5 rules) and pressing interact
   pulls it out (short `pick_up` animation, a little soil puff): a carrot comes out whole;
   for a potato plant 2–3 potatoes pop out; a ripe fruit tree gives **1 fruit** (an apple or
   an orange: the child reaches up, a fruit drops into the basket, the tree shows no fruit). The plant spot becomes empty soil and
   **regrows** after a while (proposal: 3 minutes of play time, visibly growing in 3 steps).
4. **Basket (treat bag):** harvested treats go into a **basket** carried on the arm/back — a
   separate slot from the hands, so food boxes, the fish bowl and treats never conflict.
   Capacity: 6 treats **in total** (proposal), whatever the kinds. The HUD shows the basket with
   one icon + count per treat kind it holds (🥕 🥔 🍎 🍊, numbers, no reading needed). The
   basket is saved per treat kind; old saves with carrots and potatoes only load unchanged
   (GARD-017).

## Treats

5. **Who likes what** (proposal — Q-100, fruit Q-321): four treats, the list per animal is data
   (`garden::likes`); a pair eats it together.

   | Treat | Grows in | Liked by | Refused by |
   |---|---|---|---|
   | carrot (🥕) | level 1 vegetable garden | zebra, elephant, giraffe, hippo, monkey, panda | koala, lion, snow fox, goldfish |
   | potato (🥔) | level 1 vegetable garden | elephant, hippo, panda | zebra, giraffe, monkey, koala, lion, snow fox, goldfish |
   | apple (🍎) | level 3 fruit garden | **monkey**, elephant, giraffe, zebra, panda | hippo, koala, lion, snow fox, goldfish |
   | orange (🍊) | level 3 fruit garden | **monkey**, elephant, giraffe | zebra, hippo, panda, koala, lion, snow fox, goldfish |

   Bananas are the monkey's **food** (food box, GAME-FEED), not a treat. Koala, lion, snow fox, goldfish and the night animals take no garden treat: since 2026-10-03 (proposal Q-334…Q-352) each has a **box-food treat** (GAME-FEED "Basic food and treats": koala leaves, lion bone, snow fox meat, goldfish leaves, hedgehog fruit, bat nectar, owl worms, snake eggs, chameleon frozen insects, poison dart frog crickets). Garden treats and box treats are one concept, `treat` (a treat is whatever the species' treat list holds); only the carrying differs (basket vs. hands). Realistic, and the treat list per animal is data.
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
   trough is wanted: the spot needs no prop (Q-248 answered 2026-10-01).
7. **Baby (GAME-FAMILY "Special food and babies", user request 2026-09-29):** a treat the
   animal likes makes it happy; given to a **male and female pair** at home, it makes them
   have a baby. This replaces the earlier 3-care-feedings / "one treat must be from the garden"
   proposal (Q-101 obsolete, Q-198 answered).
8. **Saving:** basket contents, garden plant states and regrow timers are saved (GAME-SAVE).

## Assets

9. New assets (concept sheet first — ART-PIPELINE): `garden_bed` (soil bed, 1 × 3 m
   modular), `carrot_plant` (in soil, 3 growth stages), `potato_plant` (3 stages),
   `apple_tree_{sprout,young,ripe}` and `orange_tree_{sprout,young,ripe}` (fruit trees, ≤ 400
   triangles each, built in `tools/blender/props/kit_garden.py`, origin = base on the soil),
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
- GARD-006 is covered by FAM-008 (retired here); the care-feeding / "at least one treat" rules are gone (Q-198, Q-101 obsolete).

## Fruit garden (2026-10-01)

- **Data:** `garden_fruit` in `assets/levels/level-3.toml` with `[[garden_bed]]`
  (`plant = "apple" | "orange"`, `sign_key = "garden-apple" | "garden-orange"`) and four
  `[[plant_spot]]` (`kind = "apple" | "orange"`, 2 per bed). The kind of a plant spot is a
  treat id (`Treat::from_id`); the model of a stage is `<kind>_tree_<stage>` for the fruit
  kinds and `<kind>_plant_<stage>` for the vegetables (`empty` = no model).
- **Same rules as the vegetables:** interact at the stand cell harvests (GARD-001), 3 minutes
  to regrow in 3 visible steps (GARD-004), basket capacity 6 in total (GARD-003), saved with
  the game (GARD-008), the gate opens by itself within 2 m, animals never enter the garden,
  following animals wait at the gate (Q-102).
- **Never stuck:** the garden is optional, its hint (`plant:<spot>`) has the lowest priority
  and is never the only hint (HINT rule 3.4); the garden is reachable from the level-3 spawn
  over streets (GARD-018).
- Giving fruit to a pair of monkeys at home makes them eat, hearts, and (once) a baby
  (FAM-008, GARD-020); a pair that does not like the fruit sniffs and turns away (GARD-005).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| GARD-001 | Given a ripe carrot plant in front of the player, when she interacts, then the basket gains 1 carrot and the spot becomes empty soil. | unit |
| GARD-002 | Given a ripe potato plant, when harvested, then the basket gains 2–3 potatoes (seeded). | unit |
| GARD-003 | Given the basket holds 6 treats, then harvesting is refused with gentle feedback and the plant stays. | unit |
| GARD-004 | Given an empty spot, after 3 minutes of play time, then the plant is ripe again, passing 3 visible growth stages. | unit |
| GARD-005 | Given the zebra at home and a carrot in the basket, when the player gives it at the fence, then the zebra eats it and the carrot is removed; given a potato, it refuses and the potato stays. | unit |
| GARD-006 | *Retired 2026-10-01 — merged into FAM-008 (treat to a pair makes one baby).* | — |
| GARD-010 | Given the zebra at home and a liked carrot, when the player stands inside the enclosure within 2 m facing it (or at the fence after the zebra walked over), then `Target::Treat` is available at the animal, giving takes the carrot and both zebras of the pair turn to the child; with an empty basket and empty hands nothing is available. | unit |
| GARD-011 | Given the player carries the zebra's own food at home, when she gives it, then the zebra eats (`FoodEaten`), the food stays in the hands and no baby is born; a food it does not eat is refused (`FoodRefused`) and stays. | unit |
| GARD-012 | Given a pair at home and 4 carrots, when she walks up to them and gives, then both zebras react (eat/hearts), carrots leave the basket and exactly one baby appears. | e2e |
| GARD-013 | Given every enclosure of an animal that takes treats (zebra, elephant, giraffe, hippo, monkey, panda), then it has a feeding spot on the gate side: both cells in the home area, the stand point outside the fence and walkable, ≥ 1 cell from the gate posts. | unit |
| GARD-014 | Given a pair at the gate with one partner waiting far behind, when the child presses the gate, then nobody enters (gate press → `Waiting` message), the hint `partner:<animal>` sends her back to fetch it, and once it follows again both enter and the mission completes; afterwards the child standing on the gate cell can walk out. | unit |
| GARD-015 | Given the four treats (`Treat::ALL`), then `id`/`from_id`/`label_key` round-trip, `likes()` follows the table in §5 for every animal of the zoo (monkey: carrot, apple, orange yes, potato no; koala, lion, snow fox, goldfish: none), and `garden-<id>` exists in de and en. | unit |
| GARD-016 | Given a ripe apple (orange) spot, when harvested, then the basket gains exactly 1 apple (orange) and the spot is empty soil; given 5 treats in the basket, then one more fruit fits and the next harvest is refused (capacity 6 in total over all kinds). | unit |
| GARD-017 | Given a basket with all four treats, when saved and restored, then it is unchanged; given an old save whose basket JSON has only `carrots` and `potatoes`, then it loads with 0 apples and oranges; a save of more than 6 treats is clamped. | unit |
| GARD-018 | Given the joined levels 1–3, then `garden_fruit` has 2 apple and 2 orange plant spots in 2 beds with signs `garden-apple` / `garden-orange` (word + picture), every `stand` cell is a walkable street/path cell within 1.2 m of its plant, the garden gate lies on the street `path_l3_ne` side, no garden rect overlaps a solid element, hiding place, scenery, barrier or ad board, and the gate is reachable from the level-3 spawn over walkable cells. | unit |
| GARD-019 | Given the fruit garden signs and reading levels `kiga`…`klasse3`, then every sign has its word (`garden-apple` Äpfel / Apples, `garden-orange` Orangen / Oranges) and a sentence for `klasse1`–`klasse3` in de and en. | unit |
| GARD-020 | Given the monkey pair at home and one apple (orange) in the basket, when the child gives it, then both monkeys react (`TreatEaten`), the fruit leaves the basket and exactly one baby appears (FAM-008); given a potato, the monkeys refuse and the potato stays. | unit |
| GARD-021 | Given the monkeys at home and an apple in the basket, then `gift_liked("monkey")` is true and the hint list holds `treat:monkey` (a baby is still possible); with only a potato it does not; with the baby born, never (HINT-026). | unit |
| GARD-022 | Given level 3 in a browser, when the child stands at an apple tree and presses interact, then the basket HUD shows 🍎1; at an orange tree 🍊1; when she gives the fruit to the monkeys at home, both eat and the HUD count drops. | e2e |
| GARD-023 | Given `apple_tree_{sprout,young,ripe}` and `orange_tree_{sprout,young,ripe}`, then the six `.glb` files exist, load, have ≤ 400 triangles each and are in the asset list of level 3 (`required_assets`); the ripe models are taller than the sprouts. | unit |
| GARD-024 | Given the child stands in level 3 and the monkey pair is at home (plant hints only for treats a pair at home still wants, HINT-026), then the garden hint prefers the plants of that level and its icon shows what grows at the target (🍎 apple, 🍊 orange, 🥔 potato, 🥕 carrot) instead of always carrots (user report 2026-10-03). | unit |
| GARD-025 | Given the basket already holds a treat (e.g. an apple), then the 🧭 garden hint does not offer the plants of that treat any more (apple trees) but still offers the others (oranges); what the child already carries is never a task (user report 2026-10-03). | unit |
| GARD-007 | Given the player carries a food box item or the fish bowl, then she can still harvest into the basket (separate slot). | unit |
| GARD-008 | Given a save with basket contents and growing plants, when restored, then both are unchanged. | unit |
| GARD-009 | Given the garden signs, then their texts come from Fluent per reading level and language (`garden-carrot`, `garden-potato`). | unit |

## Open questions

- Q-100 Which animals like which treat (and should koala/panda/snow fox get their own treats)?
- Q-101 obsolete (Q-198: special food makes the baby).
- Q-102 garden in level 1 (gate opens within 2 m — proposal). Q-154 `basket`, `carrot`, `potato` models not yet listed in ART-ENVIRONMENT.
- Q-320 Fruit garden of level 3 (location, size, tree count: `garden_fruit` at x 11–20, z 74–76 north of `enc_goldfish`, gate onto `path_l3_ne`, 2 apple + 2 orange trees). Q-321 Which animals like apples / oranges (table §5). Q-322 One fruit per harvest and no separate fruit items. Q-323 Fruit-tree art: brief and preview in `art/props/kit_garden/` (concept sheet not generated; `kit_garden` stays `concept_approved = true`, user to review the preview). Q-324 Sign picture of the fruit signs (emoji placeholder 🍎 / 🍊 and the word).
- Q-172 answered 2026-09-28: the basket is a separate slot (§4, GARD-007) and is never put down (GAME-FEED §8).
