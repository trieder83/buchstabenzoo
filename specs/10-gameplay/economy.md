---
id: GAME-ECON
title: Visitors, shops and coins (idea)
aspect: gameplay
module: economy
status: draft
depends_on: [GAME-LEVEL-3, GAME-FEED, GAME-EVENTS, CONT-MATH, GAME-SAVE, GAME-HINT]
test_prefix: ECON
updated: 2026-10-01
---

# Visitors, shops and coins (idea)

> **Status: idea collection (user request 2026-10-01).** Nothing here is decided or built;
> numbers are the user's first ideas, the rest are proposals. Decisions go to
> `specs/open-questions.md` (Q-300…Q-307).

## Goal

From **level 3** on the zoo is no longer empty: a few **visitors** come, buy **ice cream** at the ice
cream kiosk and **animal food** from a vending machine. The child keeps both stocked, earns
**coins**, and spends the coins on **simple maths tasks** that fit the zoo story (buy animal food,
pay the animal doctor). It adds a calm second loop next to the rescue missions and trains maths
(CONT-MATH) in a meaningful way. No real money, no time pressure, nothing can be lost.

## Behaviour

1. **When:** visitors appear from level 3 on (the level that has the `ice_cream_kiosk`,
   GAME-LEVEL-3), during the day only, **few at a time** (1–3, proposal Q-300). They walk along the
   streets (like ambient animals but with a character model), stop at a shop, buy, and leave. They
   never enter enclosures, never block doors or gates, and are not solid for the child (no
   pushing, like GAME-CART rule 41); they are friendly NPCs that can also give a hint (glossary
   `visitor`).
2. **Ice cream kiosk** (`ice_cream_kiosk`, level 3, freezer chest in front): a visitor buys one
   ice cream → the child gets **4 coins**. The kiosk has a **stock** (proposal: 8 scoops; shown as
   cones in the glass chest). At stock 0 the kiosk shows an "empty" icon and visitors wait ≤ 60 s,
   then leave without paying (no penalty).
3. **Animal food machine** (new prop `food_machine`, placed by the level designer beside a main path,
   level 3+): a visitor buys a bag of animal food → **2 coins**. Stock proposal: 6 bags. Visitors
   use the bag at a fence: an animal at home that likes the food comes to the fence and eats
   (hearts) — a small reward link to GAME-GARDEN treats (Q-301).
4. **Refilling** (the child's job, never forced):
   - **Ice cream** comes from the **fridge** (`fridge`, new prop inside the storage building, the
     enterable `food_storage_3`, GAME-FEED §7 inside boxes): take an ice box (one item in the hands,
     like food boxes: put down / pick up, GAME-FEED §8/§13), carry it to the kiosk chest, interact →
     stock refilled to full. The fridge never runs out (like food boxes).
   - **Animal food bags** come from the storage boxes (a bag crate in the storage) and go into the
     machine the same way.
   - A shop with stock 0 is shown by an icon above it; the 🧭 hint may offer "Eis nachfüllen" /
     "refill the ice cream" as an **optional** step (priority 4, GAME-HINT) — it never replaces a
     mission step (never stuck rule, `.claude/agents/zoo-level-designer.md`).
5. **Coins:** a purse in the HUD (coin icon + number, no text needed); earnings pop up as flying
   coins; saved with the game (GAME-SAVE). Cap proposal 999 (Q-302). Coins are never taken away
   except by a task the child chooses to do.
6. **Maths with coins** (CONT-MATH, task templates seeded, big number buttons, wrong answer =
   gentle retry and a visual hint, no penalty — CONT-MATH Behaviour 2/3):
   - **Buy animal food:** a feed shop counter (or the machine itself) shows a price list; the child
     buys bags/boxes of special food: "3 bags cost 2 coins each — how many coins?" (`mathe1`: sums ≤ 20;
     `mathe2`: multiplication; `mathe3`+: change, bigger sums, `mathe5`: decimals in money).
     Bought food can be a treat for animals (links to GAME-GARDEN/GAME-FAMILY special food).
   - **Pay the animal doctor:** now and then (rare, harmless, never urgent) an animal at home gets a
     "sniffle" icon; the **animal doctor** (a visitor NPC with a case, comes to the zoo) asks a fee in
     coins; the child pays the exact amount by picking coins (counting / change tasks); then the
     animal is happy again. If the child cannot or does not want to pay, nothing bad happens (the
     doctor waits, the animal stays a bit sniffly, no game state depends on it).
   - The maths level comes from the settings (`math_level`, independent of the reading level).
7. **Reading:** price tags, the doctor's note and shop signs use Fluent text per reading level
   (`kiga`: pictures and numbers only).
8. **Save:** coins, kiosk and machine stock, visitor schedule seed, doctor state; visitors
   themselves are not saved (they respawn).
9. **Art/audio needed (later):** visitor models (`character-artist`, 3–4 variants incl. children and
   adults, animation `walk`, `idle`, `buy`), animal doctor, `food_machine`, `fridge`, ice boxes,
   coin icon, sounds (coin, ice cream, machine); concept art first (ART-PIPELINE).
10. **Not in scope:** real money, in-app purchases, ads in the shop, penalties, timers that
    punish, competition.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| ECON-001 | Given levels 1–2, then no visitors appear; given level 3 unlocked, then ≤ 3 visitors are present at a time during the day and none at night. | unit |
| ECON-002 | Given a visitor buys ice cream with stock > 0, then the purse gains 4 coins and the kiosk stock drops by 1; given stock 0, then the visitor waits ≤ 60 s, leaves, and the purse is unchanged. | unit |
| ECON-003 | Given a visitor buys animal food from the machine with stock > 0, then the purse gains 2 coins and the stock drops by 1. | unit |
| ECON-004 | Given the child carries an ice box from the fridge and interacts with the kiosk chest, then the stock is full again and the hands are empty; the fridge never runs out. | unit |
| ECON-005 | Given a shop with stock 0, then an empty icon shows above it and the 🧭 hint may offer the refill only as an optional (priority 4) target, never while a mission step exists (never-stuck rule). | unit |
| ECON-006 | Given a maths task for `mathe1`…`mathe5`, then it is reproducible from the seed, has exactly one answer, uses coin amounts of the right size and gives a gentle hint after 2 wrong answers without losing coins. | unit |
| ECON-007 | Given the child pays the doctor the right amount, then the coins are spent and the sniffly animal is happy; given the child declines, then nothing changes and no mission is blocked. | unit |
| ECON-008 | Given a save with coins, stocks and doctor state, when restored, then all are unchanged; visitors respawn. | unit |
| ECON-009 | Given a visitor on a street, then the child can walk through it without being blocked or pushing it, and it never enters an enclosure or blocks a gate or door (LAYOUT rules). | unit |
| ECON-010 | Given the economy running for 30 min of simulated play with random child actions (including never refilling), then no mission becomes unfinishable and the hint always has a mission target (HINT-021 style fuzz). | unit |

## Open questions

- Q-300 How many visitors, when (daytime schedule), and do they come in groups/families (children + adults)?
- Q-301 Do visitors feed the animals with the machine's food (animals at the fence eat it, hearts), or is it only coins?
- Q-302 Coin cap, prices for the maths shop (animal food, doctor fee), and what the coins buy besides food and the doctor (garden seeds? decorations?).
- Q-303 Stock sizes (kiosk 8, machine 6) and refill carrying rules (one box at a time, also the fish bowl pocket rule).
- Q-304 The animal doctor: how often, which animals, one fixed visit per level or random; does a sniffly animal change behaviour?
- Q-305 Which maths levels map to which tasks (change, multiplication, decimals) — CONT-MATH Q-034.
- Q-306 Does the economy continue in later levels/night, and does level 4 (bears, honey) add shops?
- Q-307 Art: visitor/doctor/machine/fridge concept sheets (character-artist for people) before any modelling.
