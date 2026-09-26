---
id: FIX-024
date: 2026-09-26
type: contradiction
specs: [GAME-PLAYER]
questions: []
---

## Problem

GAME-PLAYER §3 said "mouse drag or Q/E rotates the camera" and, in the same line and in §4,
"`E`/Space interacts". One key cannot do both.

## Resolution

`E` (plus Space and Enter) interacts, as §4 and the M4 brief require; keyboard camera
rotation moved to `Q` (left) / `R` (right). Mouse drag rotation is unchanged. Implemented
in `crates/zoo-web` (`App::key`) and the host shell (`web/src/input.ts`).
