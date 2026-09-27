---
id: GAME-EVENTS
title: Zoo events
aspect: gameplay
module: events
status: draft
depends_on: [GAME-RESCUE, GAME-NIGHT, GAME-FAMILY, GAME-GARDEN, GAME-HINT, GAME-SAVE]
test_prefix: EVT
updated: 2026-09-27
---

# Zoo events

## Goal

Occasional **events** keep the zoo lively after (and between) the rescue missions (user
request 2026-09-27). Each event shows a **mission indicator** (the hint system points to it,
GAME-HINT priority 1), is child-friendly, never scary and never punishing.

## General rules

1. At most **one event at a time**; events start only when no animal is following the player
   and not during the first play session (the core loop comes first). Timing is seeded; the
   minimum gap between events is 10 minutes of play time (proposal Q-129).
2. An event starts with a short friendly cut-in (icon + sound + one-line text per reading
   level, Fluent), then a ❗ indicator appears in the HUD and in the world.
3. Nothing is lost when an event is not solved: at worst it ends by itself (e.g. the police
   come anyway) with a gentle message. Events are saved (GAME-SAVE).

## Event 1 — Night visitors (burglars)

4. **When:** at night only (GAME-NIGHT), in an unlocked day level.
5. **Story:** two comic, clumsy burglars (*Langfinger*, friendly cartoon style — striped
   shirts, masks like raccoons, a sack; never threatening, no weapons) sneak in to take
   something (e.g. the food boxes or the fish bowl). A **mission indicator** shows where they
   were seen.
6. **Catch them in time:** the child follows their **footprints / dropped items** (reading
   clue on a note they dropped per reading level) and finds them within a time window
   (proposal: 3 minutes of play time, shown as a moon clock — no stress numbers). Standing
   next to them and pressing interact shines the lantern on them → they freeze in surprise →
   the **police** arrive (friendly police car with blue light, two comic police officers) and
   take them away; small celebration.
7. **Not in time:** the burglars run to the zoo wall and the police catch them outside
   anyway (message: "Die Polizei hat sie trotzdem erwischt!"); whatever they took is found
   again next to the entrance.

## Event 2 — Storm

8. **When:** during the day, after at least one level is complete.
9. **Story:** a short comic storm (darker sky, rain streaks, wind in the trees, 10–15 s, no
   lightning strikes near the player) **damages 4 fence segments** of completed enclosures;
   the animals of those enclosures escape again to **new** hiding places (GAME-RESCUE
   discovery rules — riddles on the boards update).
10. **Repair and rescue:** the broken fence parts are marked; the child brings the animals
    back like a normal mission. The zookeeper repairs a fence when its animals are back (no
    tool gameplay needed — proposal; Q-130). The storm never hits enclosures with a
    following/unfinished mission.

## Event 3 — Bees and honey

11. **When:** during the day, in spring/summer mood (any time in the PoC), after the garden
    exists (GAME-GARDEN).
12. **Story:** bees are spotted (small buzzing swarm near flowers) and build a **hive** in a
    tree. Once the hive is finished (after a while), the child can collect **honey** (put
    the honey pot from the zookeeper house under the hive → a few seconds of bees working →
    a honey pot fills; bees never sting, they buzz around happily).
13. **Honey for the bears:** honey is a treat (GAME-GARDEN basket) that **bears** love.
    Feeding the bear pair honey counts as care feeding (GAME-FAMILY) → the bears may get a
    baby. **Bears are a new species** (not in the current animal list) — they need a
    concept, a mission and an enclosure (Q-131).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| EVT-001 | Given an event is active, then no second event starts, and the hint points to the event. | unit |
| EVT-002 | Given night in an unlocked level, when the burglar event starts, then a mission indicator shows where they were seen and a dropped note has a clue per reading level. | unit |
| EVT-003 | Given the player reaches the burglars within the time window and interacts, then they freeze, the police arrive and take them; the taken item returns. | unit |
| EVT-004 | Given the time window passes, then the police catch them anyway, the item is back at the entrance and nothing is lost. | unit |
| EVT-005 | Given the storm event, then exactly 4 fence segments of completed enclosures break, their animals go to new hiding places and the riddles update. | unit |
| EVT-006 | Given the storm's animals are back, then their fences are repaired. | unit |
| EVT-007 | Given the bee event, when the hive is finished and the honey pot is put under it, then honey is added to the basket; bees never hurt the player. | unit |
| EVT-008 | Given honey fed to the bear pair, then it counts as a care feeding (FAM-003). | unit |
| EVT-009 | Given any event, then all its texts exist per reading level and language, and reviewers confirm nothing is scary (manual). | unit + manual |
| EVT-010 | Given a save during an event, when restored, then the event continues in the same state. | unit |

## Open questions

- Q-129 Event timing (gap 10 min, not in the first session, burglar window 3 min).
- Q-130 Storm repair: automatic by the zookeeper or a small repair task for the child?
- Q-131 Bears: add bears as a new species (pair, enclosure, mission, level)?
