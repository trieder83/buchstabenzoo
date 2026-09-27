---
id: FIX-065
date: 2026-09-28
type: answered-question
specs: [GAME-CAMERA-VIEWS, GAME-EVENTS, GAME-CART, GAME-LAYOUT, GAME-LEVEL-1, GAME-LEVEL-2, GAME-LEVEL-3, GAME-LEVEL-NIGHT-1, GAME-NIGHT, GAME-RESCUE, GAME-ANIMALS, CONT-MISSIONS, ART-ENVIRONMENT, ART-RIG, ART-CHARACTERS, TECH-WATER, PROD-GLOSSARY]
questions: [Q-036, Q-049, Q-066, Q-081, Q-086, Q-109, Q-119, Q-120, Q-123, Q-129, Q-132, Q-133, Q-134, Q-135, Q-136, Q-137, Q-138, Q-139, Q-140, Q-145, Q-148, Q-157]
---

## Problem

Many specs still called decided rules "proposal Q-###" or listed answered questions as
open, although the questions were answered (most "as recommended" on 2026-09-27): Q-133…Q-140
(night level data, night house, night foods, riddle scope, light/item/prop data, telescope,
burglar spots, sleeping by day), Q-145 (FIX-056 moves), Q-148 (entrance board text), Q-157
(no pockets beside doors), Q-119/Q-120/Q-132 (golf carts), Q-129 (event timing), Q-109/Q-123
(camera views), Q-066 (river flow), Q-081 (bamboo in the panda enclosure), Q-086 (sign
beside the gate), Q-036 (goldfish bowl), Q-049 (high camera). GAME-LEVEL-3 still had the
placeholder "proposal Q-new" for the third food storage.

## Resolution

Replaced the stale wording with "Q-### answered" (answer summarised where the list entry
asked the question), marked the open-question list entries as answered, and pointed
"proposal Q-new" to the existing open question Q-089 (food storage per level). No rule
content changed. The Q-086 "gate arch" text in GAME-LEVEL-1 is kept as history ("first
proposal, not chosen").

## Changed files

- `specs/10-gameplay/camera-views.md`, `events.md`, `golf-carts.md`, `layout.md`, `night.md`,
  `rescue-mission.md`, `animals.md`, `levels/level-1.md`, `levels/level-2.md`,
  `levels/level-3.md`, `levels/night-1.md`
- `specs/20-content/missions/start-missions.md`
- `specs/30-art/environment.md`, `character-rig-and-animation.md`, `characters.md`
- `specs/40-tech/water-rendering.md`, `specs/glossary.md`
