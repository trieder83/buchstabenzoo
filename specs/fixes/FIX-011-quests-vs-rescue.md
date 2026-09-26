---
id: FIX-011
date: 2026-09-26
type: contradiction
specs: [GAME-QUESTS]
questions: []
---

## Problem

GAME-QUESTS still described the pre-rescue quest model: it asserted "only one quest is
active" although Q-020 is open, did not relate quests to rescue missions, and
`quest_monkey_baby` assumes a monkey mother in her enclosure although every enclosure is
empty at start (GAME-RESCUE §1).

## Resolution

- Rule 3 now states that parallel quests/missions are open (Q-020, extended to missions).
- New rule 4: quests are optional steps attached to rescue missions (GAME-RESCUE §10);
  location riddles live in GAME-RESCUE / CONT-MISSIONS.
- `quest_key` marked as depending on Q-033; `quest_monkey_baby` marked as conflicting with
  the escape premise and raised as Q-040 (no design change made).

## Changed files

- `specs/10-gameplay/quests/overview.md`, `specs/open-questions.md`
