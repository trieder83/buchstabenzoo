---
id: GAME-HINT
title: Next-target hint
aspect: gameplay
module: hints
status: draft
depends_on: [GAME-PLAYER, GAME-RESCUE, GAME-LAYOUT, GAME-NIGHT, GAME-CAMERA-VIEWS]
test_prefix: HINT
updated: 2026-09-28
---

# Next-target hint

## Goal

The game must never feel stuck or boring (user request 2026-09-27). The game is not linear,
but the child can always ask **"what can I do next?"**: a hint button directly **below the
settings gear** shows **one next possible target** and where to walk.

## Behaviour

1. **Button:** a round 🧭 button under the gear (≥ 64 px, safe area; touch and desktop;
   desktop key `H`). No text needed.
2. **One target:** pressing it picks **one** next possible target from the current state and
   shows it for 12 s (or until reached / pressed again):
   - an **indicator** above the target in the world (bouncing arrow/star, comic style, drawn on
     top so it is visible through trees),
   - an **edge arrow** at the screen border pointing to it when it is off-screen, with the
     walking distance as dots (no numbers needed),
   - a small icon of what to do there (📋 read board, 📦 take food, 🐾 animal, 🚪 gate, 🥕
     garden, 🔑 key box, 🛏 bed, 🌙 moon door, ❗ event).
3. **Choosing the target (not linear):** from all currently useful targets the game picks
   the best one with this priority, nearest first within a priority:
   1. an active **event** target (GAME-EVENTS) — e.g. burglars, broken fence,
   2. the **current mission's next step**: carrying the right food and the animal not yet
      found → the animal's hiding area (only the *area*, never closer than the riddle allows —
      see rule 4); animal following → its gate; food missing → the food storage (or a
      lying item the mission needs, "pick up" — GAME-FEED §13; for bamboo, after the panda
      board was read, also the nearest ripe cut spot — GAME-FEED §16); board not read yet →
      the nearest unread info board,
   3. an **unstarted mission**: the nearest unread info board,
   4. **optional activities**: garden (if the basket has room and plants are ripe), treats for
      animals at home, the golf-cart key box, the map board,
   5. at night: the bed or the moon door (GAME-NIGHT).
   Pressing the button again within 12 s shows the **next** candidate (cycles through the
   top 3), so a child who doesn't want that target gets another.
4. **Riddles stay fair:** the hint never points directly at a hidden animal the child has
   not found yet. Instead it points at the **info board** (to read the riddle again) or,
   after the board was read and 60 s have passed without finding it, at the **edge of the
   hiding area** (a circle ≥ 6 m wide around it), not the animal.
5. **Reading level:** on `kiga` the hint may also play the target's name via read-aloud later
   (Q-008). No text is required to use the hint.
6. **Idle nudge** (Q-127 answered: yes): if the child does nothing useful for 90 s, the 🧭
   button pulses gently once (no forced popup). The hiding-area edge hint of rule 4 comes after
   60 s (Q-127 answered).
7. The hint logic lives in zoo-core (deterministic, testable); the host only draws the
   indicator and the button.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| HINT-001 | Given a new game, when the hint is pressed, then the target is the nearest unread info board. | unit |
| HINT-002 | Given the zebra board read and no food carried, then the target is the food storage. | unit |
| HINT-003 | Given the right food carried and the zebra not found for < 60 s, then the target is the zebra's info board; after 60 s it is the edge of its hiding area (≥ 6 m circle), never the animal's position. | unit |
| HINT-004 | Given the zebra following, then the target is the zebra gate. | unit |
| HINT-005 | Given an active event, then the event target wins over missions. | unit |
| HINT-006 | Given the hint pressed 3 times within 12 s, then 3 different targets are shown in turn (if 3 exist). | unit |
| HINT-007 | Given the target off-screen, then an edge arrow points at it; given it on-screen, an indicator bounces above it, visible through trees. | e2e |
| HINT-008 | Given any reachable game state (fuzz over 1 000 seeded states), then the hint always returns a target (the game is never stuck). | unit |
| HINT-009 | Given touch, then the 🧭 button is directly below the gear, ≥ 64 px, and works; desktop `H` does the same. | e2e |
| HINT-010 | Given 90 s without a useful action (no interaction, no new cell explored), then the 🧭 button pulses once and no popup opens; any useful action restarts the 90 s (rule 6, Q-127). | unit |

## Open questions

- Q-127 answered 2026-09-27: idle nudge yes (90 s); hiding-area edge after 60 s.
