---
id: GAME-FEED
title: Food boxes, bamboo forest, carrying and putting down items
aspect: gameplay
module: feeding
status: draft
depends_on: [GAME-WORLD, CONT-READING, GAME-PLAYER]
test_prefix: FEED
updated: 2026-10-03
---

# Food boxes and carrying food

## Goal

The child finds the right food by **reading the label on the food box**. Food is used to
make an escaped animal follow (GAME-RESCUE) and is eaten when it arrives home.

## Behaviour

1. Food boxes stand in the food storage. Each `food_box` holds one food and has a label with
   the food's name; label form depends on the reading level:

   Every label (decals on the crate **lid** and the crate front **and** the text panel, §6) shows a **pictogram** of
   the food plus the word (user request 2026-10-02). The pictogram only *supports* the word:
   the riddle and the info board name the food by word, so the older children still read.
   The pictogram's share of the label height (`pictogram_scale`, pictogram above the word):

   | Reading level | Label | `pictogram_scale` |
   |---|---|---|
   | `kiga` | big pictogram + word | 0.62 |
   | `klasse1` | pictogram + word at similar size | 0.50 |
   | `klasse2` | word large, smaller pictogram; boxes with similar-looking words side by side (e.g. *Heu* / *Hai*-Futter) (Q-032) | 0.34 |
   | `klasse3` | word large, small pictogram, plus distractor boxes | 0.28 |

   The scale shrinks with the level; the word always gets the rest (`1 - scale`).
   Pictograms are **vector drawings** (flat colours + dark outline, one drawer per food on a
   128 x 128 grid, `web/src/pictograms.ts`), not emoji: emoji fonts differ between headless
   Linux, Android and iOS (and may be missing). The same drawers serve the decals and the box
   panel; the HUD carry icon and other lists keep their emoji (`FOOD_ICONS`, not a label).
   Pictogram id = food id. Info board texts and facts do not change.

   **Lid and front (user request 2026-10-02, the front is unreadable from the 55° camera):**
   the legible side is the **lid** — a flat decal on the top face of every food crate
   (outside rows and inside the storages), pictogram above the word, sized by
   `pictogram_scale` (on the small lid drawn 1.25x, at most 0.78, so it reads from the camera). The **front** keeps a small label (pictogram left + word). The word stays
   the way to choose on every level (the board names the food by word); the lid pictogram
   only supports it. **One shared atlas texture** (1024 x 512 RGBA = 2 MB: 14 lid cells
   128 x 128 + 14 front cells 256 x 64, `text:food-atlas`, rendered by the host, redrawn on
   language/reading-level change) serves all boxes of all foods; the renderer draws
   consecutive atlas decals with the same normal in **one draw call** (all lids: 1, fronts:
   one per facing), so the boxes add at most about 4 draw calls.
2. Boxes are closed — the food is not visible, so the label must be read.
3. The player carries one food at a time. Taking another box puts the current food back.
4. Food is not consumed by showing it to an animal — only when the animals enter their
   enclosure (GAME-RESCUE §8). Food boxes never run out.
5. Whether the food storage is locked at the start (`quest_key`) is open (Q-033).
6. **Taking a box (decision for M4 — simplest reading-first flow):** interacting with an
   available food box (GAME-PLAYER §5: within 2 m, on the label side, facing it) opens the
   text panel with the box **label** (pictogram + word, sized per reading level, §1) and a
   big **take** button (hand icon, no reading needed). Pressing take — or interacting again
   while the panel is open — makes the player carry that food (§3). Closing the panel
   without taking changes nothing. The carried food is shown in the HUD (icon + word).
7. **Box positions** come from the level data (`[[food_box]]` entries: food id, level
   position of the box centre, label facing); every food has **at least one** labelled box
   per storage (FEED-008). Every food storage (and the night food hut) is **enterable**
   (GAME-LAYOUT "Enterable buildings", user request 2026-09-28); the outside row stays
   **complete and unique** — one box per food, no repeats (Q-181 answered 2026-09-28): a row
   in front of the storage's door facade (night hut: its east facade), labels facing out
   towards the arriving child, with a free gap ≥ 1.2 m in front of the door (Q-150,
   LAYOUT-032); a walkable standing point within 2 m in front of every label (GAME-PLAYER
   §5). The "Futter" board hangs above the storage door (bottom 2.3 m). Positions:
   GAME-LEVEL-1/2/3, GAME-LEVEL-NIGHT-1.
   **More boxes inside** (Q-181 "some additional boxes can go inside"; Q-194 answered
   2026-09-29): 2–6 more `[[food_box]]` entries stand inside every (enterable) storage
   against the inner walls, on the 0.15 m plank platform of the solid 1 m wall band, labels
   facing the free floor of the interior, with a walkable standing point within 2 m
   (GAME-PLAYER §5), solid (collider + wall band), never in the door walkway (LAYOUT-032).
   They are **real, labelled food boxes**, taken exactly like the outside ones (interact →
   label panel → take, §6): their food **may repeat** a food already outside (Q-194: "the
   same food outside and inside" is allowed) — FEED-008's "one box per food" / "no repeats"
   check counts only the outside row; FEED-027 checks the inside boxes' geometry, FEED-028
   that interacting with one gives its food like any other box. The `klasse3` distractor
   boxes (§1 table) will stand inside too once they exist (Q-032); today there are none.

## Putting an item down (user request 2026-09-27)

8. **Drop:** the child can put down the item in the hands — a food or the fish bowl (with
   water and fish if filled). The basket (with its treats, GAME-GARDEN §4) and the honey pot
   (GAME-EVENTS) are **not** hand items: they sit in their own slot on the arm/back and are
   **never put down** (user decision 2026-09-28, Q-172). Controls:
   a big **put-down button** (✋⬇ icon, no text) next to the carried-item icon in the HUD,
   shown only while something is carried; desktop key `G` (GAME-PLAYER §3 key table). The
   cart key (🔑) and the pocket are never dropped (nothing important can get lost).
   With the fish bowl in the hands and a food in the pocket, the bowl is put down and the
   pocket food moves to the hands. With the bowl in the hands and nothing else to interact
   with, the interact action (`E` / the action button) also puts the bowl down (same rules
   as `G`; Q-158 (d)).
9. **Where:** on the ground about 0.8 m in front of the player, standing on the visible
   surface (ground height, PLAY-035). If that spot is not free walkable ground — water,
   fence, wall, gate opening or its walkway (LAYOUT-032), enclosure inside, another item,
   food box — the nearest free walkable spot within 1.5 m is used; if there is none, nothing
   is dropped (gentle shake of the button). Not while sitting in a golf cart (GAME-CART).
   A gate or door walkway is a circle of the opening's half width + 1 m around the opening
   centre (Q-158 (e)).
10. **Lying items:** a dropped item stays where it was put (its model on the ground, readable
    icon above it when near), is saved and restored (GAME-SAVE), and is never taken by
    animals — animals do not eat dropped food. Dropping a food within 1.5 m of its own food
    box puts it back into the box instead (it disappears); the 1.5 m are measured from the
    drop spot (0.8 m in front of her), not from her feet (Q-158 (b)). At most **8** items
    lie in the zoo (user decision 2026-09-27); putting down a 9th item — a food or the bowl
    (Q-158 (a)) — returns the oldest lying **food** to its box (never the bowl; the basket and
    the honey pot never lie on the ground, §8).
11. **Pick up again:** standing within 2 m of a lying item, the interact action (`E`/Space/
    Enter, touch action button with the item's icon) picks it up. If the hands are already
    full, the two items swap (the held one is put down on the same spot); the food + bowl
    pocket rule (GAME-RESCUE, Q-084) still applies: picking up a lying food while the bowl is
    in the hands puts the food into the pocket, and a food already in the pocket swaps with it
    (it lies on the same spot; Q-158 (c)).
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
    in a **bamboo forest** (every `decoration` element of kind `bamboo` with `harvestable = true`;
    level 1: `bamboo_sw` at `loc_bamboo`). Each forest has **cut spots** along its edge that
    are reachable from walkable ground (level 1: 4 spots, Q-156 answered; only `bamboo_sw` for now). Standing at a spot
    (within 1.5 m, facing it), the interact action (`E`/Space/Enter, touch action button with
    a bamboo icon) snaps off a stalk with the hands (short `pick_up` clip, leaves rustle — no
    knife or tool shown, child-safe; Q-156 answered) → the player carries the food `bamboo`,
    exactly like bamboo from the box (it works for the panda mission and is eaten at home).
    Food already in the hands is put down at the player's feet (§8–10 rules). With the bowl
    in the hands the bamboo goes into the pocket, and a food already in the pocket is put
    down at her feet (Q-158 (f)).
15. **Regrowing:** a cut spot shows a short stump and regrows in stages (stump → young shoot
    → full stalk) and can be cut again when full grown: **3 minutes of play time** (Q-156
    answered; paused time does not count). Regrowth is saved (GAME-SAVE) and seeded-free
    (deterministic). A forest never disappears; uncut stalks stay dense so the thicket keeps
    its look, collider and its role as a hiding place (`loc_bamboo`).
16. **Reading still matters:** cutting bamboo does not replace the riddle — the child must
    still read the info board to know the panda wants bamboo and where it hides. The hint
    system (GAME-HINT) may offer the nearest ripe cut spot as an alternative to the food box
    only when the panda board was read.
17. **Art:** the bamboo forest model gets separate cut-spot stalk nodes with the stages
    `stalk_full`, `stalk_young`, `stump` (ART-ENVIRONMENT; the game swaps them by state).

## Implementation (2026-09-27)

- **Data:** a bamboo forest is a `decoration` element of kind `bamboo` with
  `harvestable = true`; its cut spots are `[[cut_spot]]` entries (`id`, `forest`, `pos` = foot
  of the stalk inside the forest rect at its edge, `stand` = walkable point ≤ 1.5 m in front).
  Level 1: `cut_bamboo_n1`, `cut_bamboo_n2` (north edge), `cut_bamboo_e1`, `cut_bamboo_e2`
  (east edge) of `bamboo_sw`. The forest cells stay solid; the stalks of the cut spots are not.
- **Logic** (`zoo_core::carrying`): "hands" = the fish bowl if carried, else the food; the one
  food slot is the pocket while the bowl is carried. A drop spot is free when its cell is
  walkable (no water, fence, wall, gate, enclosure), no prop collider is within 0.25 m, it is
  more than the opening's half width + 1 m from every door/gate centre, ≥ 0.5 m from other
  lying items and ≥ 0.7 m from food boxes; else the nearest free point within 1.5 m (rings of
  0.1 m, 32 directions). "Within 1.5 m of its own box" is measured from the drop spot. Picking
  up a food with the bowl in the hands puts it into the pocket (a pocket food swaps with it).
  Cutting bamboo with a food in the hands (or pocket) puts that food down at her feet.
  Regrowth: stump for the first 90 s, young shoot for the next 90 s, then full. These are
  the rules of §8–11 and §14 (Q-158 answered 2026-09-28: accepted as implemented).
- **Presentation:** the put-down button `#drop-btn` (✋⬇, 76 px, below the carried-item HUD,
  shakes when nothing can be put down), key `G`; lying foods are drawn as the small closed
  food box on the surface with the food icon above them within 5 m; cut-spot stalks are
  placeholder boxes (full stalk 2.9 m, young shoot 1.2 m, stump 0.3 m) until the bamboo model
  has the §17 nodes.
- The basket (GAME-GARDEN keeps treats without the hands) and the honey pot (GAME-EVENTS)
  are no hand items and have no drop path in `carrying` (Q-172); the drop only ever takes
  the food or the bowl.
- The hint targets of §13/§16 are implemented in GAME-HINT (`zoo_core::hints`, HINT-011).
- **Not yet:** golf carts (GAME-CART) do not exist yet, the `pick_up` clip and the rustle sound of §14.

## Basic food and treats (user request 2026-10-03, proposal — Q-334…Q-352)

Every food has **two possible roles per species**, and the same food may be the basic food of one species and the treat of another (meat: basic for the lion, treat for the snow fox; crickets: basic for the chameleon, treat for the poison dart frog).

- **Basic food** (*Grundfutter*) — the food from the food box that makes the **escaped** animal follow in the mission (today `info.foods`, GAME-RESCUE) and gives only hearts at home (no baby).
- **Treat** (*Leckerli*) — the special food given to the **pair at home**: hearts **and the baby** (once per species, GAME-FAMILY). A treat is either a **garden treat** (carrot, potato, apple, orange: carried in the basket, GAME-GARDEN) or a **box food** (carried in the hands). **Both are one concept, `treat`**, in the rule and in the data: `Role::{Basic, Treat, None}` per (species, food-or-garden-item); a species lists its basic foods and its treats; one item is never both roles for the same species. This **replaces** the former fallback "the species' own favourite food is its special food" (FAM-030, Q-281).

### Master table (all 16 species)

| Species | Level | Basic food (box) | Treat(s) | Treat source |
|---|---|---|---|---|
| `zebra` | 1 | Gras / grass | carrot, apple | garden (level 1 beds; level-3 orchard) |
| `hippo` | 1 | Melonen / melons | carrot, potato | garden (level 1) |
| `panda` | 1 | Bambus / bamboo | carrot, potato, apple | garden |
| `koala` | 2 | Eukalyptus / eucalyptus | **Blätter / leaves** (young leaves) | box `leaves` (level-2 storage; already stocked) |
| `elephant` | 2 | Heu / hay | carrot, potato, apple, orange | garden |
| `giraffe` | 2 | Blätter / leaves | carrot, apple, orange | garden |
| `lion` | 2 | Fleisch / meat | **Knochen / bone** (new) | box `bone` inside the level-2 storage |
| `monkey` | 3 | Bananen / bananas | carrot, apple, orange | garden |
| `snow_fox` | 3 | Beeren / berries | **Fleisch / meat** (user) | box `meat` (level-3 storage; already stocked) |
| `goldfish` | 3 | Fischfutter / fish food | **Blätter / leaves** (lettuce leaf) | box `leaves` (level-3 storage; already stocked) |
| `hedgehog` | night 1 | Käfer / beetles | **Obst / fruit** (apple pieces) | box `fruit` (night_1 storage) |
| `bat` | night 1 | Obst / fruit | **Nektar / nectar** | box `nectar` (night_1 storage) |
| `owl` | night 1 | Käfer / beetles | **Würmer / worms** | box `worms` (night_1 storage) |
| `snake` | night 2 | Fisch / fish (new) | **Eier / eggs** (user, new) | box `eggs` inside the night_2 storage |
| `chameleon` | night 2 | Grillen / crickets (new) | **Frostinsekten / frozen insects** (user, new) | box at the fridge inside the night_2 storage |
| `poison_dart_frog` | night 2 | Fliegen / flies (new) | **Grillen / crickets** | box `crickets` (night_2 storage, outside) |

**Where the treats come from:** (1) garden treats as today (GAME-GARDEN). (2) Box treats are ordinary boxes of the same level's storage: the three day storages already hold all ten day foods (so `leaves`, `meat` need no data change); `bone` is **one new box inside the level-2 storage**; the night_1 storage already holds `fruit`, `nectar`, `worms`; the night_2 storage holds the new foods (GAME-LEVEL-NIGHT-2 "Night food storage"). **Treat-only foods stand inside** the storage (the child reads "Leckerli" on the board and goes in), basic foods outside, so a distractor row never hides the basic food. (3) `frozen_insects` stands at a **fridge** prop inside the night_2 hut (GAME-ECON `fridge`, never runs out, snowflake sticker) — a box like any other (interact → label panel → take). No box is star-marked: the role belongs to the animal, not to the box (Q-338).
**New foods (6, total 20):** `fish`, `crickets`, `flies`, `eggs`, `frozen_insects`, `bone`; ids lowercase, word keys `food-<id>`: de Fisch, Grillen, Fliegen, Eier, Frostinsekten, Knochen; en fish, crickets, flies, eggs, frozen insects, bone. **Pictograms to draw** (`web/src/pictograms.ts`, 128 × 128, flat colours + dark outline, one drawer each; FEED-031 counts 20): fish (side-on, orange, smile-free), cricket (green, long hind leg, antennae), fly (round dark body, two clear wings, big eyes), eggs (two cream eggs, one speckled), frozen_insects (a cricket silhouette in a pale-blue ice cube with a small snowflake), bone (cartoon bone, cream). No prey-animal pictures that look scary (Q-077): the fish is a plain cartoon fish; eggs, no chicks; no mice.

### Rules (GAME-RESCUE, GAME-GARDEN 6, GAME-FAMILY)

1. **Escaped animal, mission:** only its **basic food** makes it follow. A **treat** shown to an escaped animal gets a gentle "Hmm, später!" (no refusal sound, no penalty, same gentle bubble as RESC-005) and it does **not** follow; neither does a wrong food (RESC-005). The mission and the hints never require a treat.
2. **Animal at home, giving** (GAME-GARDEN 6 giving rules unchanged: at the animal, ≤ 2 m facing it, or at the fence with the animal walking over):
   - **basic food** → `eat`, hearts, "Mmh, lecker!", **no baby**;
   - **its treat** → `eat`, more hearts, and for a **pair at home** (male + female) the baby celebration; **one baby per pair, once per species**, saved (FAM-009), never twice; a second treat gives hearts only;
   - **neither** → gentle refusal (`refuse`, "Hmm, das mag ich nicht.", RESC-005), the item stays.
3. **Carrying:** box foods (basic or treat) stay in the hands after giving (boxes never run out, FEED-023); garden treats leave the basket as today. The child picks a box food by reading the label, the same way as for the mission.
4. **Hints:** the existing 🧭 treat hint (GAME-GARDEN 6a, `hint-treat`) fires when the child **carries a treat of a home species whose pair has no baby yet** (garden basket or box in the hands) and leads to that animal's feed spot — priority > 3, **never the only hint** while a mission is incomplete (HINT-024). There is **no** hint that fetches a treat with empty hands (babies are optional, Q-339). The info board is where the child learns the treat.
5. **Never stuck:** treats and babies are optional; no mission, gate or morning waits for them.
6. **Saves:** the saved baby flags are unchanged; a game that already got a baby by the old own-favourite-food fallback keeps it (no loss, no refund); only the new rule applies from now on (FAM-032).

### Info board (user request 2026-10-03: "it needs to be clear on the info board")

The panel order stays riddle first (RESC, QA finding F2): name → **riddle** → **basic food line** → **treat line** → facts → pair note. The riddle never names a food; the board names both roles by the **exact words of the box labels** (`food-<id>`) and the garden words (`garden-<id>`).

| Reading level | Basic food line | Treat line |
|---|---|---|
| `kiga` | big pictogram (`pictogram_scale` 0.62), no text | big pictogram(s) with a **⭐ marker** and a small source icon (🌱 garden / 📦 box); no text required |
| `klasse1` | pictogram + "Grundfutter: Beeren" | ⭐ pictogram + "Leckerli: Fleisch" |
| `klasse2` | "Grundfutter: Beeren" + small pictogram | "Leckerli für ein Baby: Fleisch" + small ⭐ pictogram |
| `klasse3` | "Grundfutter: Beeren. Damit folgt dir das Tier." | "Leckerli: Fleisch. Damit bekommt ein Paar ein Baby." |

en: "Basic food: berries" / "Treat: meat" (klasse2 "Treat for a baby: meat"; klasse3 "Basic food: berries. The animal follows you for it." / "Treat: meat. A pair gets a baby for it."). Several treats are listed separated by " · " (elephant: Karotten · Kartoffeln · Äpfel · Orangen, at most 4 pictograms in a row). Keys: `board-basic-<reading_level>`, `board-treat-<reading_level>` (prefix words) + `food-<id>` / `garden-<id>` (the food words); species data `foods` (basic) and `treats` (list) feed both lines; fits next to the existing food-word line on phones (Q-070: the treat line is the same height as the basic line, the facts follow below). ANIM-003 is extended: every basic **and** treat word equals the label word of that item.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| FEED-001 | Given reading level `kiga`, then every food box label has a pictogram (id = food id) with `pictogram_scale` 0.62 and one word. | unit |
| FEED-002 | Given any reading level, then every label of every food has a pictogram and a word; given reading levels in order `kiga`..`klasse3`, then `pictogram_scale` strictly decreases, `klasse1` is the similar-size level (0.45..=0.55) and `klasse2`/`klasse3` are below 0.4 (word larger than the pictogram). | unit |
| FEED-003 | Given the player carries hay, when taking the bamboo box, then the player carries bamboo and hay is back. | unit |
| FEED-004 | Given the player shows grass to zebras, then the player still carries grass. | unit |
| FEED-005 | Given every animal's correct food, then a food box with that food exists in the storage. | unit |
| FEED-006 | Given the player has taken food from the grass box 10 times, when taking it again, then the player carries grass (boxes never run out). | unit |
| FEED-007 | Given the player stands in front of the grass box facing it, when interacting, then the text panel with the grass label opens and the player carries nothing yet; when taking, the player carries grass. | unit |
| FEED-008 | Given the level data, then every food has exactly one labelled box in the outside row of each storage (no repeats there); every box (outside or inside) stands within 1 m of an (enterable) food storage / food hut rect, and a walkable standing point within 2 m in front of each box's label exists (Q-181 answered: boxes outside; Q-194 answered: more boxes inside may repeat a food). | unit |
| FEED-009 | Given the player carries hay, when `G` is pressed (desktop) or the put-down button is tapped, then hay lies on free walkable ground ≈ 0.8 m in front of her, standing on the surface, and she carries nothing. | unit + e2e |
| FEED-010 | Given the spot in front is water, a fence, a gate walkway or inside an enclosure, then the item lies on the nearest free walkable spot within 1.5 m, or is not dropped when none exists; in a golf cart nothing is dropped. | unit |
| FEED-011 | Given hay lying on the ground, when the player within 2 m interacts, then she carries hay; given she carries bamboo, then bamboo lies on that spot and she carries hay (swap). | unit |
| FEED-012 | Given the filled fish bowl with the goldfish and a food in the pocket, when dropped, then the bowl with water and fish lies on the ground, the food moves to the hands, and picking the bowl up again restores bowl + fish (food to the pocket). | unit |
| FEED-013 | Given zebras following and the player drops the grass, then the zebras keep following; animals never eat or take lying food. | unit |
| FEED-014 | Given a food dropped within 1.5 m of its own food box (measured from the drop spot), then it goes back into the box; given 8 lying items, putting down another food or the bowl returns the oldest lying food to its box and never removes the bowl; the basket and the honey pot are never put down and never lie on the ground (Q-172). | unit |
| FEED-015 | Given items lying in the zoo, when saved and restored, then every item lies at the same place with the same content. | unit |
| FEED-016 | Given nothing carried or only the cart key, then no put-down button is shown and `G` does nothing. | e2e |
| FEED-017 | Given a full-grown cut spot of a harvestable bamboo forest and the player standing at it, when she interacts (desktop `E` or the touch action button), then she carries `bamboo` and the spot becomes a stump. | unit + e2e |
| FEED-018 | Given bamboo cut from the forest, when shown to the panda (right riddle state), then the panda follows exactly as with bamboo from the box, and it is eaten at home. | unit |
| FEED-019 | Given a cut spot, then it is not cuttable for 3 minutes of play time, shows stump → young shoot → full stalk, and is cuttable again afterwards; paused time does not count. | unit |
| FEED-020 | Given the player carries hay and cuts bamboo, then hay lies at her feet and she carries bamboo. | unit |
| FEED-021 | Given cut spots in different growth stages, when saved and restored, then every spot keeps its stage and remaining regrowth time. | unit |
| FEED-022 | Given every harvestable bamboo forest in the level data, then each cut spot has a walkable stand point within 1.5 m, and the forest stays solid and keeps its hiding place valid. | unit |
| FEED-023 | Given zebras following and the player carrying grass, when they enter their enclosure, then the grass is consumed (the player carries nothing); given the grass was put down earlier, the lying grass stays; showing food alone never consumes it. | unit |
| FEED-024 | Given the current mission needs an item that lies on the ground (the fish bowl, or the mission's food) and the child carries nothing better, when the hint is asked, then a candidate of GAME-HINT priority 2 is "pick up" at that lying item (§13). | unit |
| FEED-025 | Given the panda info board not read, then no hint points at a bamboo cut spot; given it was read and a cut spot is full grown, then the nearest ripe cut spot may be a hint candidate for bamboo beside the bamboo food box (§16). | unit |
| FEED-026 | Given the exported bamboo forest model, then every cut spot has the stage nodes `stalk_full`, `stalk_young` and `stump` (§17). | asset |
| FEED-027 | Given every food storage / food hut, then 2–6 `[[food_box]]` entries stand inside it (whose position falls inside the storage's rect), each stands on the 0.15 m plank platform of the solid wall band, is solid for the player, and has a walkable standing point within 2 m in front of its label, clear of the door walkway (Q-194 answered 2026-09-29). | unit |
| FEED-028 | Given the player stands in front of a food box inside a storage facing it, when interacting, then the text panel with that box's label opens exactly as for an outside box; when taking, the player carries that food (Q-194 answered 2026-09-29: inside boxes are real food boxes). | unit |
| FEED-029 | Given every food (day and night) and every reading level, then `FoodBox::label` gives a non-empty pictogram id equal to the food id, a `pictogram_scale` from the §1 table, and the word key of that food. | unit |
| FEED-030 | Given a level, then every `[[food_box]]` (outside and inside) has a lid decal (facing up) and a front decal (facing the box facing), all cells of the one atlas `text:food-atlas`; lids are adjacent in the decal list (1 draw), fronts are grouped by facing (<= 4 runs); changing the reading level re-requests the atlas. Start view: the decals add <= 5 draw calls (e2e log). | unit + e2e |
| FEED-035 | Given the atlas, then every food has a lid cell (128 x 128) and a front cell (256 x 64) inside 1024 x 512, without overlap, UVs inside 0..1, <= 2 MB. | unit |
| FEED-031 | Given the host's pictogram set, then there is one distinct drawer per food (all 20 once the six new foods exist: 14 now), deterministic (same draw calls twice) and drawn inside the 128 x 128 grid. | vitest |
| FEED-032 | Given a lid cell, then the host draws the pictogram (share `pictogram_scale` of the height) above the word (rest), the word fitted to the width; the front cell has a smaller pictogram left of the word. | vitest |
| FEED-033 | Given the box panel of a food box, then it shows the vector pictogram and the word on every reading level; the pictogram is bigger on `kiga` than on `klasse3`. | e2e |
| FEED-034 | Given the zoo camera in front of the food box row on `kiga` and `klasse3`, then the labels are legible (screenshots `qa/reports/img/`). | e2e |
| FEED-36 | Given the master table of "Basic food and treats", then every one of the 16 species has ≥ 1 basic food and ≥ 1 treat, no item is both for the same species, every box-food treat has a box in the species' own level storage (outside or inside), and meat is basic for the lion and a treat for the snow fox. | unit |
| FEED-37 | Given an escaped animal and its basic food, then it follows; given one of its treats or a wrong food, then it does not follow and shows the gentle bubble. | unit |
| FEED-38 | Given a pair at home and its basic food, then hearts and no baby; given its treat (garden basket or box in the hands), then hearts and exactly one baby (once per species, saved); given a second treat, then hearts only; given a neither-food, then gentle refusal. | unit |
| FEED-39 | Given the snow fox pair at home and a meat box in the hands, then one baby is born and the meat stays in the hands; given the lion pair and the same meat, then hearts only (basic). | unit |
| FEED-40 | Given a species with a garden treat (zebra) and its basic food gras, then no baby (unchanged); given the old own-favourite-food rule (FAM-030), then it is retired (the species' treat per the table is the only baby trigger). | unit |
| FEED-41 | Given the info board of every species at every reading level and language, then it has a basic food line and a treat line with the words of the matching box labels / garden words (extends ANIM-003); `kiga` shows pictograms with a ⭐ on the treat and no text. | unit |
| FEED-42 | Given the board panel on a phone portrait and landscape, then riddle, basic line, treat line are visible without scrolling on `klasse3` and the treat line has the star marker. | e2e |
| FEED-43 | Given the host's pictogram set, then drawers exist for `fish`, `crickets`, `flies`, `eggs`, `frozen_insects`, `bone` (distinct, inside the 128 × 128 grid). | vitest |
| FEED-44 | Given the hands hold a treat of a home pair without a baby, then the 🧭 offers the treat hint with priority > 3; with empty hands no treat hint exists; with a mission incomplete the mission step stays the first hint. | unit |

## Open questions

- Q-032 Distractor design per reading level, Q-033 storage locked by `quest_key`?
- Q-065 Food storage interior — superseded by Q-181 (answered 2026-09-28: storages enterable, labelled boxes outside, extra boxes inside). Q-194 stock boxes inside (unlabelled decoration; `klasse3` distractors inside once they exist).
- Q-025 How food is carried, Q-042 `give` clip vs. food not consumed, Q-034 food portions vs. one food at a time.
- Q-156 answered 2026-09-27: 4 cut spots on `bamboo_sw`, 3 min regrowth, hands only, only `bamboo_sw` for now.
- Q-158 answered 2026-09-28: the put-down details (a)–(f) are rules now (§8–11, §14).
- Q-155 answered: animals keep following when their food is put down; at most 8 lying items; giving consumes.
- Q-150 answered 2026-09-27: the food-box rows leave a free gap ≥ 1.2 m in front of every food storage door (LAYOUT-032).
- Q-172 answered 2026-09-28: basket and honey pot are a separate slot and are never put down (§8, FEED-014; GAME-GARDEN §4, GARD-007 unchanged).
- Q-154 the bamboo forest model with cut-spot stage nodes (§17) is not yet listed in ART-ENVIRONMENT.
