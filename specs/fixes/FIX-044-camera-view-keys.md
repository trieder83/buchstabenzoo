---
id: FIX-044
date: 2026-09-27
type: answered-question
specs: [GAME-PLAYER, GAME-CAMERA-VIEWS]
questions: [Q-109]
---

## Problem

The user decided on 2026-09-27: **`V` toggles first person**, **`F` / right mouse button
holds look-around**, touch 👓 toggle **bottom right above the interact button**, 👁 hold
button next to the interact button. GAME-CAMERA-VIEWS and the code followed the decision, but:

- GAME-PLAYER §3 still said "holding `V` … shows look-around, `F` toggles first person"
  (the old, swapped keys) and did not mention the 👓 button.
- Q-109 still proposed "`V` / right mouse (hold), toggle left of the gear + `F`" and was `open`.
- There was no single list of all desktop keys, so collisions (`Q`/`R`, `E`/Space/Enter,
  `M`, `Esc`, WASD/arrows, `F`, `V`) could not be checked in one place.
- GAME-CAMERA-VIEWS rule 3 described the 👓 toggle twice in one garbled sentence; rule 2's
  "the eye button is hidden, `F` does nothing" in first person had no test; CAMV-016 stood
  after CAMV-018; the implementation note listed CAMV-017/018/019 as untested although they
  exist.

## Resolution

- GAME-PLAYER §3: desktop key table per view (zoo / look-around / first person) — the one
  place listing all keys, no key with two meanings; touch paragraph names 👁 and 👓 with their
  positions; new PLAY-034 (key table, no double binding); Q-124 listed.
- GAME-CAMERA-VIEWS: rule 3 rewritten (same content, decision date kept); rule 2 names the
  right mouse button too and points to the new CAMV-020; rule 9 names the `zoo.view` key;
  test table in numeric order; implementation note lists the real coverage; open-question
  list updated (Q-109 partly answered, Q-124…Q-126).
- Q-109 → `partly answered` with the user's key/button decision; values, eye-button slide
  and `kiga` access stay open.

## Changed files

- `specs/10-gameplay/player.md`
- `specs/10-gameplay/camera-views.md`
- `specs/open-questions.md`
