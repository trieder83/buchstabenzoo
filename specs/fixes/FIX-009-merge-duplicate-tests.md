---
id: FIX-009
date: 2026-09-26
type: cleanup
specs: [CONT-MISSIONS, CONT-READING, GAME-RESCUE]
questions: []
---

## Problem

Duplicate test cases: MISS-002 = READ-002 (klasse1 ≤ 5 words), MISS-003 ≈ RESC-011 (riddle
must not name the place; RESC-011 did not define "name"), MISS-006 = ANIM-004 (hiding place
exists in layout data). RESC-011 sat between RESC-003 and RESC-004.

## Resolution

- READ-002 now explicitly covers all languages and the location riddles; MISS-002 retired.
- RESC-011 now defines the name as the `kiga` place word of the hiding place (matching
  rule itself is Q-039); MISS-003 retired; RESC-011 moved to its numeric position.
- MISS-006 retired in favour of ANIM-004; CONT-MISSIONS §2 points to ANIM-004.
- Retired ids stay in the table as "Retired" rows so they are never reused.

## Changed files

- `specs/20-content/missions/start-missions.md`, `specs/20-content/reading-levels.md`,
  `specs/10-gameplay/rescue-mission.md`
