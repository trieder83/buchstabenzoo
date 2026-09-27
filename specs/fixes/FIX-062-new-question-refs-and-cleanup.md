---
id: FIX-062
date: 2026-09-27
type: cleanup
specs: [GAME-NIGHT, GAME-LEVEL-1, GAME-LEVEL-2, GAME-LEVEL-3, GAME-LEVEL-NIGHT-1, GAME-FEED, GAME-GARDEN, GAME-LAYOUT, CONT-L10N, ART-ENVIRONMENT, ART-PIPELINE]
questions: []
---

## Problem

- Q-147…Q-152 were only referenced in GAME-NIGHT, GAME-LAYOUT and GAME-LEVEL-2. The other
  specs they affect did not list them, so the index "Open Q" column was incomplete.
- GAME-NIGHT still said "placeholder panes on every building facade", although the same
  section now says the building models bring their own glowing windows.
- The NIGHT test rows were out of order.
- `garden.md` and `asset-pipeline.md` were changed without bumping `updated:`.

## Resolution

- Added the Q-147…Q-154 references to the open-question lists of every affected spec.
- Placeholder panes are now limited to procedural building facades.
- Sorted the NIGHT rows by number.
- Bumped `updated:` to 2026-09-27 in both files.

## Changed files

- `specs/10-gameplay/night.md`, `levels/level-1.md`, `levels/level-2.md`, `levels/level-3.md`,
  `levels/night-1.md`, `feeding.md`, `garden.md`, `layout.md`,
  `specs/20-content/localization.md`, `specs/30-art/environment.md`,
  `specs/30-art/asset-pipeline.md`
