---
id: GAME-HINT
title: Next-target hint
aspect: gameplay
module: hints
status: draft
depends_on: [GAME-PLAYER, GAME-RESCUE, GAME-LAYOUT, GAME-NIGHT, GAME-CAMERA-VIEWS, GAME-FEED, GAME-GARDEN]
test_prefix: HINT
updated: 2026-10-01
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
   5. at night: the bed or the moon door (GAME-NIGHT); from the end of the celebration of a
      day level (dusk) the bed. They are only offered when they are the way forward (dusk,
      night, or by day while the night zoo waits, Q-140), so they rank **before** the optional
      activities of 4 and together with 3 (Q-189 answered 2026-09-28).
   A target on the other side of the moon door (the night zoo from the day zoo, or back)
   is shown as the **moon door** while it is open; otherwise it is not offered.
   Pressing the button again within 12 s shows the **next** candidate (cycles through the
   top 3), so a child who doesn't want that target gets another.
4a. **Always works, always says what to do (user request 2026-09-29):** the hint has no
   waiting time — it answers on every press from the first second of the game, in every
   state, and never returns "nothing" (HINT-008). Rule 4's 60 s only changes *how exact* the
   target is, not *whether* there is one: right after the board was read the hint shows the
   **wide search area** (≥ 12 m circle around the hiding area) plus a "re-read the riddle"
   option (📋 in the edge arrow), after 60 s the ≥ 6 m circle of rule 4. Every hint shows
   **the next step as icon + short line** (Fluent `hint-<step>`, read-aloud on `kiga`): 📋
   "Lies die Tafel" → 📦 "Hol das Futter" → 🐾 "Suche das Tier hier" → 🚪 "Bring es nach
   Hause". The step is derived from the current mission (rule 3.2), so it also matches while
   the child follows several missions. The 🧭 button also works while a panel is open (it closes
   the panel first), while following animals, in the garden and at night.
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
8. **Compass shows what is missing** (GAME-NIGHT rule 11, user request 2026-10-02): the 🧭
   button carries the compass strip (the missing animals of the current level) and a task
   badge with the kind of the best candidate (cheap `next task`, polled at ≤ 4 Hz, does not
   show a hint). Tapping the compass is the hint plus an info bubble; while animals of the
   day level are missing the hint leads to the next missing animal's mission step (rule 3);
   the strip icon of `hint.animal` pulses. **On small screens** (GAME-PLAYER §3) the strip
   is collapsed (only the 🧭 and the task badge show, plus a pulsing dot while something is
   missing); tapping the compass gives the hint **and** opens the strip to the left of the
   compass in one row for ~6 s (max 5 icons + `+n`, ½ tags included), then it closes by
   itself; any other tap also closes it. Large screens always show the strip (HINT-023).

## Implementation (2026-09-28)

- **Core** (`zoo_core::hints`): `candidates(game, tracker)` lists every useful target, sorted
  by priority, then the straight distance, then id (deterministic); one entry per id.
  `HintTracker` (owned by the host glue, not saved) holds the shown target, the top-3
  cycle, the rule-4 search timers and the idle nudge. A hint is re-evaluated every 0.25 s
  while shown and hides when it is reached (≤ 2.5 m), when it is no longer a candidate
  (board read, food taken, …), after 12 s, or when the child goes to sleep.
- **Current mission:** while a group follows (or the goldfish is in the carried bowl) its
  gate is the mission target; else a started mission whose food is in the hands. Only the
  current mission's steps are priority 2; other started missions count as priority 3 (so
  the hint never sends the child to take a second food while the first is still needed).
  Without a current mission every started mission's step is priority 2.
- **Mission steps** (rule 3.2): the goldfish first needs the bowl (📦 pick up) and water
  (💧, the nearest water point); food missing → the nearest unlocked **food storage** (its
  door, the gap in the box row in front of it — never the right box, reading the labels stays
  the child's job, Q-187 answered 2026-09-28; without a storage building the centre of the boxes),
  every lying food the animal eats (📦 "pick up", GAME-FEED §13) and, for bamboo eaters, every
  full-grown cut spot (🎋, only because the board was read — GAME-FEED §16); right food in
  the hands → the board for the first 60 s after it was read, then the hiding-area edge.
- **Hiding-area circle** (rule 4): centre = the hiding place's spot, radius =
  max(3 m, `wander_radius_m`) + 1 m (≥ 6 m wide, and always ≥ 1 m beyond where the animal
  wanders, so the edge is never its position); the target is the point of the circle
  nearest to the child; none while she is inside the circle.
- **Optional** (rule 3.4): ripe plants while the basket has room; animals at home that
  like a treat in the basket or their own carried food (at the animal or its fence). Not in the game yet and therefore skipped:
  events (priority 1, GAME-EVENTS), golf carts and their key box (GAME-CART), the map board
  (GAME-MAP), ad boards.
- **Never stuck** (HINT-008): if nothing else is useful, the nearest board in scope (read
  again), else the bed, else an open moon door.
- **Idle nudge** (rule 6): useful actions are the child's interactions (every progress event,
  a panel opening, "not interested", a refusal, a missing bowl/water, a refused treat, a full
  basket) and entering a grid cell not visited before. Idle time does not count while a hint
  is shown, a reading panel is open, or the child sleeps.
- **Host** (`App::hint_press`, `hint_json`, key `H`): the 🧭 button `#compass-btn` (68 px,
  right, directly below the ⚙ gear), the bouncing indicator `#hint-marker` (DOM overlay,
  always on top — visible through trees) with the target icon, and at the screen border the
  edge arrow `#hint-edge` (arrow + icon + 1–5 dots, one per 10 m of walking distance, the
  grid path measured once per second). Icons: 📋 board, 📦 food / pick up, 🐾 hiding area,
  🚪 gate, 🎋 bamboo, 💧 water, 🥕 garden, 🧺 treat, 🛏 bed, 🌙 moon door.
- The rule-4 search timer is not saved: after a reload the board is shown again for 60 s
  (Q-186 answered 2026-09-28).

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
| HINT-011 | Given the panda board read and no bamboo carried, then the food storage and every full-grown cut spot are priority-2 targets (before the board was read no cut spot is offered); given the right food lying on the ground, it is a "pick up" target, a lying wrong food is not (GAME-FEED §13/§16). | unit |
| HINT-012 | Given dusk after level 1, then the target is the bed; given night, then the open moon door comes first (the night boards are behind it) and the bed is among the targets; through the door, the target is a night board. | unit |
| HINT-013 | Given the compass (🧭 with strip and badge) tapped, then the hint shows and the info bubble appears; there is no separate moon pane; the badge kind equals the kind of the shown hint's candidate list head. | e2e |
| HINT-019 | NEVER STUCK (a): given random day states incl. split pairs and members that disagree about "complete" (an old one-zebra save), while a mission of the zoo is open, then the first hint has priority <= 3 (never only the optional garden). | unit |
| HINT-020 | NEVER STUCK (c): given 120 s without mission progress while a mission is open, then the hint is `help` (icon 👀, `hint-help`, priority 1) at the missing animal's exact position; after 180 s the escaped animals of open missions walk to < 8 m of the player; progress (food taken, following, home...) restarts the timer. | unit |
| HINT-021 | NEVER STUCK: given random / split-pair / old-save states and a save+restore in the middle, a child who only follows the hints (incl. pair partner steps and `help`) brings all level-1 animals home within 60 hint steps. | unit |
| HINT-023 | Given a small screen (780×360 and 360×780), then the strip is collapsed at the start (no strip icon visible, the badge and 🧭 visible, `data-missing` > 0); a compass tap shows the hint and the strip (`.expanded`, icons left of the compass inside the viewport, overlapping no other control); it collapses after ~6 s and on a tap on the canvas; on 1280×720 the strip stays visible without a tap. | e2e |
| HINT-022 | Given a species whose members are all in their enclosure, then no hint candidate belongs to it and the compass strip does not list it, even when its mission flags are still incomplete (RESC-033). | unit |
| HINT-018 | NEVER STUCK (split pair): given a zebra pair with member 0 at home and member 1 still out (board read or not, several seeds), then the first hint is about the missing zebra (board / food / search area / gate), never only optional garden work, and the compass strip still lists the zebra as not home. | unit |
| HINT-017 | Given a home animal, then the treat hint (`hint-treat`: "Geh zum Tier und gib ihm etwas zu fressen") exists only while the basket holds a treat it likes or the hands hold its food; it points at its fence (from outside) or at the animal (inside). | unit |
| HINT-015 | Given any state (new game, board read 1 s ago, board read 70 s ago, animal following, carrying the wrong food), when the hint is pressed, then it returns a target immediately (no waiting time) with a next-step key `hint-<step>` that matches the state (read / take food / search / lead home); 1 s after reading the board the target is the ≥ 12 m search area, after 60 s the ≥ 6 m circle, never the animal's position. | unit |
| HINT-016 | Given a hint is shown, then the indicator shows the step icon and the line (Fluent, `de` and `en`); on `kiga` the line is read aloud; pressed while a reading panel is open, the panel closes and the hint shows. | e2e |
| HINT-014 | Given a child who only follows the hints (several seeds, every level-1 hiding place at least once), then all level-1 animals come home, night falls, and the bed is among the top 3 hints and brings the morning. | unit |

## NEVER STUCK (HINT-018…021, 2026-10-01)

Whenever a mission of an unlocked level is open there is a hint of priority <= 3 (the mission
step; if the mission has no usable step, `help` at the animal not at home). The mission state of
a species is that of the whole group: started when any member's is, complete only when all are
(`Game::mission`). After 120 s without mission progress (`STALL_HELP_S`; progress = an animal
changing state, a mission started/complete, food taken/put down, a lying food) the hint is the
`help` target (priority 1) at the escaped member's exact position; after 180 s (`STALL_WALK_S`)
escaped animals of open missions walk to the player (Q-262). The timer is not saved and does not
run in the dark.

## Open questions

- Q-195 answered 2026-09-30 (yes): right after the board was read the hint shows the wide area (≥ 12 m), the exact circle after 60 s (rule 4a).
- Q-262 open: stall rescue times (120 s `help` hint, 180 s animals walk to the player; NEVER STUCK).
- Q-127 answered 2026-09-27: idle nudge yes (90 s); hiding-area edge after 60 s.
- Q-186 answered 2026-09-28 (yes): the rule-4 search timer is not saved; after a reload the
  board is shown again for 60 s.
- Q-189 answered 2026-09-28 (yes): the bed / moon door rank before optional activities (rule 3.5).
- Q-187 answered 2026-09-28 (yes): the food-storage hint points at the storage (its door since
  the labelled boxes stand outside in a row, Q-181), never at the right box.
