---
id: FIX-031
date: 2026-09-26
type: missing-tests
specs: [GAME-RESCUE]
questions: [Q-069, Q-082]
---

## Problem

Two decided M5a rules had no spec test case: a new game avoids each animal's hiding place of
the previous game (Q-082; covered only by an untagged e2e test in `web/tests/e2e/m5a.spec.ts`)
and only missions listed in `[level] missions` are interactable (Q-069; RESC-017 covers texts
only).

## Resolution

Added RESC-024 (e2e, last picks avoided) and RESC-025 (unit, only in-scope missions
interactable; no field = all in scope).

## Changed files

- `specs/10-gameplay/rescue-mission.md`
