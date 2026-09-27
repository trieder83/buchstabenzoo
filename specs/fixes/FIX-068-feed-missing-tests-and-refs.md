---
id: FIX-068
date: 2026-09-28
type: missing-tests
specs: [GAME-FEED, GAME-HINT, GAME-RESCUE]
questions: [Q-155]
---

## Problem

- GAME-FEED §13 (lying item as a hint target), §16 (cut spot as a hint only after the panda
  board was read) and §17 (bamboo model stage nodes) had no test case.
- GAME-HINT rule 3 did not mention lying items or cut spots, although GAME-FEED §13/§16 refer
  to "GAME-HINT priority 2".
- GAME-RESCUE §6 did not say that following animals keep following when their food is put
  down (Q-155 answered), and the bowl rule still said "interact on free ground".

## Resolution

- Added FEED-024 (lying item → "pick up" hint), FEED-025 (cut-spot hint only after the panda
  board was read), FEED-026 (asset: `stalk_full` / `stalk_young` / `stump` nodes).
- GAME-HINT rule 3.2: lying item the mission needs and (after the panda board) the nearest
  ripe cut spot count as targets.
- GAME-RESCUE §6 and goldfish-bowl rule 7: cross-references to GAME-FEED §8–12 / Q-155.
- GAME-FEED open questions: Q-172 (basket / honey pot), Q-154 (bamboo model nodes).

## Changed files

- `specs/10-gameplay/feeding.md`, `hints.md`, `rescue-mission.md`
