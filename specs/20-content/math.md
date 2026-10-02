---
id: CONT-MATH
title: Math tasks
aspect: content
module: math
status: draft
depends_on: [PROD-VISION, CONT-READING]
test_prefix: MATH
updated: 2026-10-01
---

# Math tasks

## Goal

Besides reading, the game can train math for grades 1–5, embedded in the zoo world — never
as a detached worksheet.

## Math levels (proposal — Q-034)

| Math level | Grade | Content (German curriculum, simplified) |
|---|---|---|
| `mathe1` | 1 | counting to 20, + and − up to 20 |
| `mathe2` | 2 | + and − up to 100, simple multiplication (1×1 intro), coins |
| `mathe3` | 3 | + and − up to 1000, 1×1 complete, division with remainder, time |
| `mathe4` | 4 | written addition/subtraction, multiplication up to 1 000 000, units (kg, m, l) |
| `mathe5` | 5 | fractions (½, ¼), decimals in money, simple word problems |

## Where math appears (proposals — Q-034)

1. **Food portions:** "3 Zebras fressen je 2 Bündel Gras. Wie viele Bündel brauchst du?" —
   the player takes the right number of bundles.
2. **Number lock:** the food storage or a gate opens with a code computed from a task.
3. **Counting:** "Wie viele Enten schwimmen im Teich?" to get a hint from a visitor.
4. **Word problems** (`mathe3`+) on info boards, combining reading and math.
5. **Coins** (GAME-ECON, from level 3): buy animal food and garden seeds, pay the animal doctor —
   `mathe1` sums ≤ 20, `mathe2` multiplication, `mathe3` change, `mathe4` bigger sums / units,
   `mathe5` decimals in money (Q-305 answered 2026-10-01; day only, optional, never blocks a mission).

## Behaviour

1. Math tasks are generated from templates with the seeded RNG, so they are reproducible
   in tests and varied between playthroughs.
2. Every task has exactly one correct answer; answer input is by big number buttons or
   picking objects, never free text.
3. Wrong answer → gentle feedback and a new attempt; after 2 wrong attempts a visual hint
   (e.g. objects to count). No penalty (Q-014).
4. Task text follows the reading level too (word problems are also reading tasks).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| MATH-001 | Given seed S and math level `mathe2`, then the generated task list is identical on every run. | unit |
| MATH-002 | Given any generated task at math level L, then all operands and the result are within L's number range. | unit |
| MATH-003 | Given any generated task, then its stored answer is the correct result of its expression. | unit |
| MATH-004 | Given 2 wrong answers, then a visual hint is shown. | unit |
| MATH-005 | Given math is disabled for a profile, then no rescue mission requires a math task. | unit |
| MATH-006 | Given any math task text, then it has a variant for every reading level (and every enabled language). | unit |

## Open questions

- Q-034 Separate math level per profile? Mandatory or optional? Which placements?
- Q-035 Do reading levels also go up to grade 5?
- Q-014 Failure/penalty rules (math: gentle retry only).
