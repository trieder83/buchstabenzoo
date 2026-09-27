---
id: FIX-051
date: 2026-09-27
type: contradiction
specs: [GAME-NIGHT, PROD-GLOSSARY]
questions: [Q-078]
---

## Problem

GAME-NIGHT goal and rule 1 said nightfall comes when all animals "of the day zoo" are home and
"happens once per zoo", while rule 7 / Q-078 (answered: day and night levels alternate, level 2
opens after `night_1`) and the engine section schedule nightfall once per completed **day
level**. The glossary entry `nightfall` had the same old wording.

## Resolution

Goal, rule 1 and the glossary now say "all animals of a day level" / "once per completed day
level (day and night levels alternate, Q-078)" — the only reading compatible with the answered
Q-078. The open consequence for level 2 → level 3 went to Q-141.

## Changed files

- `specs/10-gameplay/night.md`, `specs/glossary.md`
