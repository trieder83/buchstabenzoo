---
id: FIX-046
date: 2026-09-27
type: contradiction
specs: [TECH-ARCH, ART-DIRECTION, GAME-SAVE, GAME-LAYOUT, GAME-CART, GAME-NIGHT, ART-CHARACTERS]
questions: []
---

## Problem

Cross-spec check of GAME-CAMERA-VIEWS:

- TECH-ARCH decision 9 said the close-view far plane *is* the fog end; GAME-CAMERA-VIEWS 6
  and CAMV-005 say fog end + 2 m (18 m). Decision 9 had no test reference.
- ART-DIRECTION 3 spoke of "the default zoo camera … ground only" instead of the glossary
  terms (zoo view / close views) and gave no values or tests.
- GAME-SAVE "What is saved" listed only language and reading level as settings; the view
  setting (`zoo.view`, GAME-CAMERA-VIEWS 9) was missing.
- GAME-LAYOUT's hiding-place sight rule only covered the zoo-view screen test; the fog rule
  of the close views (CAMV-008, margin Q-110) was not referenced where levels are designed.
- GAME-CART said "the girl" although the player may be `player_boy` (PLAY-001).
- ART-CHARACTERS did not reference Q-112 (first-person hands).

## Resolution

- TECH-ARCH 9: "fog end + 2 m = 18 m", tested by CAMV-001…005, 014, 018.
- ART-DIRECTION 3: "Zoo view: ground only, no sky. Close views: comic sky + haze 9–16 m",
  tests PLAY-009, CAMV-004, CAMV-016; night colours → Q-126.
- GAME-SAVE: settings row adds the camera view `zoo` / `first_person` (`zoo.view`).
- GAME-LAYOUT: sight rule references GAME-CAMERA-VIEWS 5 / CAMV-008 / Q-110.
- GAME-CART: "the girl" → the player (character); `drive` clip for both player characters;
  Q-125 listed. GAME-NIGHT: Q-126 listed. ART-CHARACTERS: Q-112 listed.

## Changed files

- `specs/40-tech/architecture.md`
- `specs/30-art/art-direction.md`
- `specs/30-art/characters.md`
- `specs/10-gameplay/save.md`
- `specs/10-gameplay/layout.md`
- `specs/10-gameplay/golf-carts.md`
- `specs/10-gameplay/night.md`
