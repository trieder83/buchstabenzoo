---
id: FIX-072
date: 2026-09-30
type: cleanup
specs: [GAME-LAYOUT, GAME-ANIMALS, GAME-RESCUE]
questions: [Q-197, Q-201, Q-202]
---

## Problem

- `INDEX.md` was stale: ART-SOUND (new, `30-art/sound.md`) missing; test counts and open-question
  lists out of date (HINT-015/016, FAM-008/009, LAYOUT-044…046, ANIM-013, RESC-001 …).
- `layout.md` test table had LAYOUT-046/044/045 before LAYOUT-042/043; `animals.md` had ANIM-013
  before ANIM-008.
- `layout.md` "walkable ground" still said grass 0.98 m/s (GAME-PLAYER §6 is 1.45 m/s since 2026-09-29).
- `updated:` not bumped on animals, layout, rescue-mission.

## Resolution

- Regenerated `INDEX.md`: 41 specs (draft 39, implemented 2), 564 test cases; questions answered 89,
  open 90, partly answered 7, proposed 2. No frontmatter errors, duplicate spec/test ids, unknown
  dependencies or unknown Q-### references.
- Test rows re-ordered by id; grass speed in layout.md set to 1.45 m/s; `updated: 2026-09-30`.

## Changed files
specs/INDEX.md, specs/10-gameplay/layout.md, animals.md, rescue-mission.md
