---
id: FIX-034
date: 2026-09-26
type: missing-tests
specs: [GAME-RESCUE, GAME-SAVE, GAME-LAYOUT]
questions: []
---

## Problem

M5b behaviour had tests in code without a spec test ID, or no spec test case at all:
perched animals (GAME-RESCUE §12; `perch_animals_sit_up_and_come_down` and the e2e
"Q-094 monkey" test), putting the bowl down / wrong enclosure (`bowl_put_down_and_wrong_enclosure`),
the host save slot `zoo.save` v2 and its migration (e2e "GAME-SAVE v2"), locked levels asleep
and missions in scope over the joined zoo (`missions_in_scope_are_the_unlocked_levels`, only
tagged RESC-025, which covers `[level] missions` of a single level).

## Resolution

Added RESC-026 (perches, unit + e2e), RESC-027 (bowl put down, wrong gate), SAVE-011 (host
slot `zoo.save` v2 and migration of `zoo.save.level-1`, e2e), LAYOUT-025 (locked level asleep
and not interactable, scope = union of unlocked levels). The code tests should carry these IDs.

## Changed files

- `specs/10-gameplay/rescue-mission.md`
- `specs/10-gameplay/save.md`
- `specs/10-gameplay/layout.md`
