---
id: FIX-016
date: 2026-09-26
type: answered-question
specs: [GAME-PLAYER, GAME-LEVEL-1, GAME-LAYOUT, ART-ENVIRONMENT, ART-DIRECTION, ART-PIPELINE, ART-CHARACTERS]
questions: [Q-049]
---

## Problem

The user decided the high-angle (≈ 55°) follow camera and GAME-PLAYER §2 was rewritten, but
Q-049 was still `open`, and GAME-LEVEL-1 (spawn text, "open issues" section, rule 7,
LAYOUT-L1-006), GAME-LAYOUT and ART-ENVIRONMENT still described the camera as pending,
with an eye-level line-of-sight test that no longer matches the game camera.

## Resolution

- Q-049 marked `answered` with where it is specified.
- GAME-LEVEL-1: spawn camera = GAME-PLAYER §2; section renamed "High-angle camera (Q-049
  answered)", items updated to the decided roof cut-away / text panel; rule 7 and
  LAYOUT-L1-006 now the screen test from the Q-049 recommendation (hiding place off-screen
  at every 45° rotation and 10–20 m zoom); `blocks_view` column marked informative;
  depends_on + GAME-PLAYER.
- GAME-LAYOUT: sight-blocking note says the check is a per-level screen test.
- ART-ENVIRONMENT, ART-CHARACTERS, ART-DIRECTION, ART-PIPELINE: Q-049 references marked answered;
  mockup camera points to GAME-PLAYER §2 (default ≈ 14 m instead of "12–16 m").
- Q-044 note updated (`blocks_view` now informative only).
- New Q-052 (FOV axis) because the screen test and LAYOUT-L1-011 depend on it.

## Changed files

- `specs/open-questions.md`, `specs/10-gameplay/levels/level-1.md`,
  `specs/10-gameplay/layout.md`, `specs/30-art/environment.md`,
  `specs/30-art/characters.md`, `specs/30-art/art-direction.md`, `specs/30-art/asset-pipeline.md`
