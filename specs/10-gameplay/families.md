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
4. **Caring after the rescue** (treats from the vegetable garden also count — GAME-GARDEN): at home, the enclosure has a **feeding trough**. Bringing the
   pair its correct food again (taken from the food storage, read from the box label) and
   putting it into the trough is one **care feeding**.
5. **Baby:** when the pair at home is given a liked special food ("Special food and babies", Q-198 answered; replaces the former 3-care-feedings rule), the
   next time the player comes to the enclosure a **baby** is there (small celebration, the
   baby's name via Fluent). One baby per pair in the PoC scope. The baby is a **real member of
   the group at home** (GARD-013): it keeps to a cell next to the female inside the fence (never
   at the fence), follows her, is called to the feeding spot with the pair (third place) and
   reacts to treats. Its position is not saved; on load it is placed next to the female. A pair
   enters its enclosure **together**: a partner still waiting far behind (RESC-006) is called
   to catch up first, then both enter (GARD-014).
6. Wrong food in the trough: the animals don't eat it; gentle feedback (as RESC-005), no
   penalty.
7. **Saving:** pairs, the baby flag, are part of the save
   (GAME-SAVE).
8. **Art:** each pair needs a male and a female variant and a baby model (turnaround
   sheets first — ART-PIPELINE). The baby uses the same rig as its parents, scaled (~45 %).

## Special food and babies (user request 2026-09-29)

**Special food** (a liked treat such as a carrot — GAME-GARDEN "Treats" — or another food the
species loves, data per species, Q-100) makes an animal **happy**: it comes over, eats it,
hearts appear (`eat` + happy hearts, GARD-005). If the enclosure holds a **male and a female**
of the species (a pair, rule above) and either is given special food, both are happy and the
pair **makes a baby**: a short celebration (hearts between the two), and a **baby** appears
in the enclosure the next time the child is within view (rule 5 name/model). One baby per pair
(Q-075). A single animal (no pair) is only happy — no baby. Wrong or ordinary food changes
nothing. This **replaces the counting rule of rule 5** ("3 care feedings on 3 sessions"):
happy-making special food is what triggers the baby; feeding the correct storage food only gives a happy reaction (hearts), never a baby (Q-198 answered 2026-09-30). Babies are a reward
for care, never required for the mission or blocking. The species that are pairs stay zebra and
koala until decided (Q-073); koalas take no garden treats, so they need their own special food
(eucalyptus treat, Q-100).

## Implementation status (M5a, 2026-09-26)

No pair logic yet; there is no female zebra model (only `zebra.glb`), so level 1 has one
zebra (one animal group, as before). Superseded for rules 1–2 by M5b below; FAM-003…006
stay open until the female and baby models exist (ART-ANIMALS, Q-074).

## Implementation status (2026-09-30, family models)

- Models exist: `zebra_female`, `zebra_foal`, `koala_female`, `koala_joey` (ART-ANIMALS
  "Family models"). `pair = true` is **on** for `enc_zebra` (level 1) and `enc_koala`
  (level 2): every game has two zebras and two koalas (FAM-001/002 run on the level data).
- Member 1 is drawn with the female model (`zoo_core::animals::female_model`, host falls back
  to the male model if the file is missing). The baby (`game.babies`, rule 5) is drawn with its
  own model next to the female, copying her clips (`baby_model`); it has no own logic or
  position yet (Q-204).
- Saving: the mission entries of a pair are restored by member order (was: both onto member 0).
- Riddle plural (Q-106) is now consistent with the game.
- FAM-003…006: care-feeding trough and celebration are not implemented (Q-198 replaced the
  counting rule; FAM-004/008/009 cover the baby via special food in `zoo-core`).

## Implementation status (M5b, 2026-09-26)

- Rules 1 and 2 are implemented in `zoo-core` behind level data: an enclosure element with
  `pair = true` gets two animals of its species (member 0 and 1, same model) that start at
  the same picked hiding place (the second one on a neighbouring wander cell), follow together when
  either is shown the right food, and complete the mission only when both are home (a
  waiting one must be fetched). Saves keep both (`member`, GAME-SAVE v2).
- (Superseded 2026-09-30: the flag is on in the level data, see above.)
- Rules 3–8 (look, care feeding, babies) remain open (FAM-003…007).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| FAM-001 | Given a new game, then the zebra pair and the koala pair each start at one shared hiding place, both escaped. | unit |
| FAM-002 | Given the correct food shown to one animal of a pair, then both follow as one group; the mission completes only when both are in the enclosure. | unit |
| FAM-003 | Given the pair at home, when the correct storage food is put in the trough, then both show hearts and no baby appears. | unit |
| FAM-004 | Given a liked special food given to the pair at home, when the player next comes within view of the enclosure, then a baby appears once with a celebration; it never appears twice. | unit |
| FAM-005 | Given wrong food in the trough, then it is not eaten and the count does not change. | unit |
| FAM-006 | Given a save with a baby, when restored, then both are unchanged. | unit |
| FAM-008 | Given a pair (male + female) at home and a carrot given to the zebras, then both are happy (hearts) and exactly one baby appears once; given the same carrot to a single animal (no pair), then it is happy and no baby appears. | unit |
| FAM-009 | Given a pair at home and ordinary food or a disliked treat, then no baby appears and no penalty; given a saved game after the baby, then it is restored and never appears twice. | unit |
| FAM-010 | Given a pair, a baby and a child with a liked treat at the feeding spot, then male, female and baby stand on the spot cells (ranks 0, 1, 2), inside the fence, and all three turn to the child (GARD-013, unit). | unit |
| FAM-007 | Given male and female models side by side from the default camera, then children can tell they are a pair of the same species and spot the difference (manual review). | manual |

## Open questions

- Q-198 answered 2026-09-30: special food to a pair triggers the baby; see "Special food and babies".
- Q-073 Which other species come as pairs?
- Q-074 How male and female differ visually.
- Q-106 answered by the pair flag being on (2026-09-30).
- Q-203 Female zebra's distinguishing detail (forelock, lashes) not modelled; Q-204 baby behaviour and placement.
- Q-075 What exactly triggers the baby (number of feedings, sessions vs. real days) and how many babies.
