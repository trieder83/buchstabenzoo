---
id: FIX-004
date: 2026-09-26
type: contradiction
specs: [GAME-WORLD, GAME-QUESTS, ART-ENVIRONMENT, PROD-GLOSSARY]
questions: []
---

## Problem

GAME-WORLD §3 (and WORLD-002/003, QUEST-004, the glossary entries `key`/`pirate_ship`,
`env_food_storage` "locked door") stated the food storage is locked until the key is found,
while the newer GAME-FEED §5 leaves this open (Q-033) after the rescue-mission redesign.

## Resolution

Aligned everything to the open state: GAME-WORLD §3 now refers to Q-033; WORLD-002/003 and
QUEST-004 are marked "only if Q-033 keeps the lock"; glossary `key` is conditional and
`pirate_ship` also names the monkey hiding place; ART-ENVIRONMENT's food storage and pirate
ship rows are conditional. Q-033's affected specs extended.

## Changed files

- `specs/10-gameplay/world.md`, `specs/10-gameplay/quests/overview.md`,
  `specs/30-art/environment.md`, `specs/glossary.md`, `specs/open-questions.md`
