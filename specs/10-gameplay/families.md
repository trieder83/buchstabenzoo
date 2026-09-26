---
id: GAME-FAMILY
title: Animal pairs and babies
aspect: gameplay
module: families
status: draft
depends_on: [GAME-ANIMALS, GAME-RESCUE, GAME-FEED, GAME-SAVE]
test_prefix: FAM
updated: 2026-09-26
---

# Animal pairs and babies

## Goal

Some species live in the zoo as a **pair — a male and a female** (user decision
2026-09-26: zebras and koalas). When the pair is home and gets the right food, they **may
get a baby later** — a long-term reward that makes caring for the animals worthwhile.

## Pairs

| Species | Pair | Baby name (de / en) |
|---|---|---|
| `zebra` | male + female | Fohlen / foal |
| `koala` | male + female | Koalababy (Joey) / joey |

Other species stay single until decided (Q-073).

## Behaviour

1. **Escape as a pair:** both animals of a pair wait at the **same hiding place** and wander
   near each other (each within the hiding area, ≤ 3 m, GAME-ANIMALS).
2. **Rescue as a pair:** showing the correct food to either of them makes **both** follow
   (one group, GAME-RESCUE §5); the mission completes when both are in the enclosure.
3. **Look:** male and female are clearly the same species and differ subtly but visibly for
   children: the male is ~10 % larger; the female has a small distinguishing detail
   (proposal: slightly different mane/ear tuft pattern; no clothing or bows — Q-074). Both
   are friendly; nothing stereotyped.
4. **Caring after the rescue:** at home, the enclosure has a **feeding trough**. Bringing the
   pair its correct food again (taken from the food storage, read from the box label) and
   putting it into the trough is one **care feeding**.
5. **Baby:** after **3 care feedings on 3 different play sessions** (proposal — Q-075), the
   next time the player comes to the enclosure a **baby** is there (small celebration, the
   baby's name via Fluent). One baby per pair in the PoC scope. The baby stays close to the
   female and wanders with the parents.
6. Wrong food in the trough: the animals don't eat it; gentle feedback (as RESC-005), no
   penalty.
7. **Saving:** pairs, care-feeding count and session ids, and the baby are part of the save
   (GAME-SAVE).
8. **Art:** each pair needs a male and a female variant and a baby model (turnaround
   sheets first — ART-PIPELINE). The baby uses the same rig as its parents, scaled (~45 %).

## Implementation status (M5a, 2026-09-26)

The pair logic is **not implemented yet**: there is no female zebra model (only
`zebra.glb`), so level 1 has one zebra (one animal group, as before). FAM-001…006 stay
open until the female and baby models exist (ART-ANIMALS, Q-074).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| FAM-001 | Given a new game, then the zebra pair and the koala pair each start at one shared hiding place, both escaped. | unit |
| FAM-002 | Given the correct food shown to one animal of a pair, then both follow as one group; the mission completes only when both are in the enclosure. | unit |
| FAM-003 | Given the pair at home, when the correct food is put in the trough, then the care-feeding count increases by one per play session at most. | unit |
| FAM-004 | Given 3 care feedings on 3 different sessions, when the player next comes within view of the enclosure, then a baby appears once with a celebration; it never appears twice. | unit |
| FAM-005 | Given wrong food in the trough, then it is not eaten and the count does not change. | unit |
| FAM-006 | Given a save with a baby and care-feeding progress, when restored, then both are unchanged. | unit |
| FAM-007 | Given male and female models side by side from the default camera, then children can tell they are a pair of the same species and spot the difference (manual review). | manual |

## Open questions

- Q-073 Which other species come as pairs?
- Q-074 How male and female differ visually.
- Q-075 What exactly triggers the baby (number of feedings, sessions vs. real days) and how many babies.
