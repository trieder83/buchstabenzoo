---
id: FIX-043
date: 2026-09-26
type: contradiction
specs: [GAME-CAMERA-VIEWS, GAME-PLAYER]
questions: []
---

## Problem

Narrow check of the new GAME-CAMERA-VIEWS spec against GAME-PLAYER, ART-DIRECTION, the
glossary and the implementation:

- GAME-PLAYER §3 still said arrow keys always walk and the right thumb always rotates in
  45° steps; the new `V` / `F` keys, the right-mouse look-around, the eye button and the
  first-person arrow-key turning were only in GAME-CAMERA-VIEWS. PLAY-009 ("no sky at every
  distance and rotation") did not say it applies to the zoo view only.
- GAME-CAMERA-VIEWS rule 2 did not say what ← → do in look-around (they still step
  sideways, as implemented) and did not mention that sliding the thumb on the eye button
  turns the view (implemented; the only way to turn while walking on touch).
- "the girl's eyes/body" although the player can choose `player_boy` (PLAY-001).
- "fog" in GAME-CAMERA-VIEWS collides with the glossary `fog` (map fog, GAME-MAP).
- CAMV-005 tested the zoo view's 120 m far plane and "no fog", which no rule stated.
- Rules 4 ("transitions never go through the ground") and 8 (occluder fade on in
  look-around / off in first person, roofs hidden in every view) had no test case.
- `depends_on` lacked GAME-LAYOUT (LAYOUT-L1/L2/L3-006 standing points, Q-110) and
  GAME-SAVE (rule 9).
- Q-109 described the first-person toggle as "below the gear"; the spec and the host put it
  left of the gear.

## Resolution

- GAME-PLAYER §3: cross-reference to the close-view controls (desktop keys/mouse, eye
  button, smooth right-half drags); PLAY-009 scoped to `view_mode` `zoo`; open-question
  list names Q-109…Q-112, Q-123.
- GAME-CAMERA-VIEWS: rule 2 names arrow keys and the eye-button slide (flagged as part of
  the Q-109 proposal) and points to the new Q-123; "girl" → player character; rule 5 says
  it is the camera's distance fog, not the map fog; rule 6 states the zoo view's 120 m far
  plane and no fog; new CAMV-017 (rule 8) and CAMV-018 (rule 4); depends_on + GAME-LAYOUT,
  GAME-SAVE; Q-123 listed.
- Glossary `fog`: note that it is not the camera's distance fog (`fog_end`).
- Q-109 wording corrected (toggle left of the gear, eye-button slide); new Q-123 (close-view
  rotation/zoom keys) for the unresolved spec/code mismatch.
- `INDEX.md` regenerated (33 specs, 384 active test cases).

## Changed files

- `specs/10-gameplay/camera-views.md`
- `specs/10-gameplay/player.md`
- `specs/glossary.md`
- `specs/open-questions.md`
- `specs/INDEX.md`
