---
id: CONT-MATH
title: Math tasks
aspect: content
module: math
status: draft
depends_on: [PROD-VISION, CONT-READING]
test_prefix: MATH
updated: 2026-10-06
---

# Math tasks

## Goal

Besides reading, the game can train math for grades 1–5, embedded in the zoo world — never
as a detached worksheet.

## Math levels (proposal — Q-034; setting and default decided 2026-10-06, Q-368)

| Math level | Grade | Content (German curriculum, simplified) |
|---|---|---|
| `mathe1` | 1 | counting to 20, + and − up to 20 |
| `mathe2` | 2 | + and − up to 100, simple multiplication (1×1 intro), coins |
| `mathe3` | 3 | + and − up to 1000, 1×1 complete, division with remainder, time |
| `mathe4` | 4 | written addition/subtraction, multiplication up to 1 000 000, units (kg, m, l) |
| `mathe5` | 5 | fractions (½, ¼), decimals in money, simple word problems |

The **math level is a setting** next to the reading level (settings row `#settings-math`, key
`zoo.mathLevel`, GAME-CART rule 21), **default `mathe1`**; an unknown/missing value counts as
`mathe1`. It is independent of the reading level. Core: `MathLevel` (`Mathe1`…`Mathe5`),
`set_math_level(id) -> bool`, `math_level()`; saved in the game save (GAME-SAVE v3).

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
5. The "after 2 wrong attempts" visual aid (rule 3) is **3 wrong codes** at the cart lock
   (`LOCK_HELP_TRIES`, GAME-CART rule 15), because the child first has to find the note; the
   aid is then shown on the note panel: the 3rd wrong code sets `note_aid`, which stays set when the
   child reads the note again (that reading resets the tries) and ends with a new task (math level change).
   Saved (GAME-SAVE v3).

## Cart note (key-box combination, GAME-CART 12–18; decided 2026-10-06)

`cart_note_task(game_seed, math_level) -> MathTask` returns the **one** task of the golf-cart
note. It is deterministic and uses **its own RNG stream** (hash of the game seed, the tag `cart`
and the level index — it never consumes the game RNG, so existing seeded behaviour such as
hiding-place picks and saves stay unchanged). `MathTask` = `{ level, kind, operands, answer,
text_key, aid }`.

The key box has **3 digit wheels**: the answer is an integer **1…999** (never 0, never a
fraction), shown and entered with leading zeros (`5` → `005`; `pad3(answer)`). Every level has
tasks that are solvable by a child of that level with no helper, inside the level's number range
(the table of math levels) and inside 1…999 (the lock's range; this caps `mathe4`/`mathe5` — Q-373):

| Level | Task kinds (all results integer, 1…999) | Example | Lock |
|---|---|---|---|
| `mathe1` | `add`: a + b ≤ 20 (a, b ≥ 1); `sub`: a − b with a ≤ 20, 1 ≤ b < a | 2 + 3 | `005` |
| `mathe2` | `add`/`sub` ≤ 100 (result ≥ 1); `mul` a × b with a ∈ {2, 3, 4, 5, 10}, b ∈ 1…10 | 7 × 5 | `035` |
| `mathe3` | `add`/`sub` with results ≤ 999 (operands < 1000, result ≥ 1); `mul` a, b ∈ 2…10; `div` exact (dividend = a × b ≤ 100, divisor a ∈ 2…10) | 6 × 7 | `042` |
| `mathe4` | `add`/`sub` with 3-digit operands (carry/borrow), result ≤ 999; `mul` a × b ≤ 999 with a ∈ 11…99, b ∈ 2…9 or a ∈ 100…499, b ∈ 2; `unit` ("2 m = ? cm", whole numbers ≤ 999) | 125 + 238 | `363` |
| `mathe5` | `frac` ½, ¼, ¾, ⅕ of a whole number ≤ 999 whose result is an integer; `money` (sums of euro amounts with .50 that add up to whole euros ≤ 999); `word` (a short multiplication/addition story, result ≤ 999, also a reading task) | ¼ von 360 | `090` |

Rules:

1. **Solvable at every level:** every task of every level, for every seed, has exactly one
   correct integer answer in 1…999 that the generator computes with an independent evaluator
   and re-checks (MATH-003/007); no task needs a calculator, a remainder or a fraction in the
   answer.
2. **Variety:** at least 20 distinct tasks per level over 200 seeds, and every kind of the level
   occurs (MATH-013). Different seeds give different tasks (CART-012).
3. **Texts:** `math-cart-<kind>-<reading level>` per language (de, en) — the expression is shown
   as numerals on every level (`kiga`: with pictures: dots, tens-rods, groups), word problems
   (`word`) per reading level; every key has all four reading levels (MATH-006, MATH-012).
4. **Visual aid** (after `LOCK_HELP_TRIES` = 3 wrong codes, on the note panel): `add`/`sub`:
   two groups of dots (tens-rods and ones above 20); `mul`: a rows × columns block grid (blocks
   of ten above 10 × 10); `div`: dots in equal groups; `frac`: a bar divided into n parts with k
   shaded; `money`/`unit`/`word`: the underlying operation's aid. No penalty (Q-014).
5. **Level change:** the task is regenerated from the same seed with the new level
   (GAME-CART rule 18).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| MATH-001 | Given seed S and math level `mathe2`, then the generated task list is identical on every run. | unit |
| MATH-002 | Given any generated task at math level L, then all operands and the result are within L's number range. | unit |
| MATH-003 | Given any generated task, then its stored answer is the correct result of its expression. | unit |
| MATH-004 | Given 2 wrong answers, then a visual hint is shown. | unit |
| MATH-005 | Given math is disabled for a profile, then no rescue mission requires a math task. | unit |
| MATH-006 | Given any math task text, then it has a variant for every reading level (and every enabled language). | unit |
| MATH-007 | Given `cart_note_task(seed, level)` for 1000 seeds × `mathe1`…`mathe5`, then every answer is an integer in 1…999, all operands lie in the level's table above, and the stored answer equals an independent evaluation of the expression. | unit (`zoo-core/tests/math.rs`) |
| MATH-008 | Given any answer, then `pad3(answer)` has exactly 3 digits (`5` → `005`, `100` → `100`) and parses back to the answer; the lock accepts exactly that code. | unit |
| MATH-009 | Given the same seed and level, then the cart task is identical on every run and after any other play; generating it does not change the game RNG state (hiding-place picks and old saves stay the same). | unit |
| MATH-010 | Given a new game, then `math_level()` is `mathe1`; `set_math_level("mathe3")` returns true and is returned by `math_level()`; an unknown id returns false and changes nothing; a save/restore keeps it. | unit |
| MATH-011 | Given 3 wrong codes at the lock (2 wrong answers elsewhere), then the visual aid of the task's kind is offered (aid kind matches the task kind). | unit |
| MATH-012 | Given every cart-note text key, then all four reading levels × `de`/`en` exist, `kiga` shows pictures (aid) and the numerals. | unit (`i18n`) |
| MATH-013 | Given 200 seeds per level, then there are >= 20 distinct tasks per level and each kind of the level's table occurs. | unit |

## Open questions

- Q-034 Separate math level per profile? Mandatory or optional? Which placements? *Partly answered 2026-10-06 (Q-368): the level is a setting next to the reading level, default `mathe1`; "optional/switch-off by a parent" and the other placements stay open.*
- Q-373 (proposal) The key-box lock has 3 digits, so cart-note results are 1…999: `mathe4`/`mathe5` tasks are capped accordingly (no 6-digit products, decimals only as whole euros). Alternative: 4 wheels for the older levels.
- Q-035 Do reading levels also go up to grade 5?
- Q-014 Failure/penalty rules (math: gentle retry only).
