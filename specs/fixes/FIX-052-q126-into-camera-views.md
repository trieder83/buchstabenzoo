---
id: FIX-052
date: 2026-09-27
type: answered-question
specs: [GAME-CAMERA-VIEWS]
questions: [Q-126]
---

## Problem

Q-126 was answered ("as recommended"), but GAME-CAMERA-VIEWS rule 7 lacked one part of the
recommendation (shining eyes are hidden by the haze beyond 16 m like all geometry), its open
questions list still showed Q-126 as open, and the night sky of rule 7 had no test case (the
renderer test `night_sky_is_dark_blue` carries no ID).

## Resolution

Added the eyeshine/haze sentence to rule 7, marked Q-126 answered in the list, added
**CAMV-021** (night sky colours, haze = horizon colour, fog end 16 m, day colours unchanged).

## Changed files

- `specs/10-gameplay/camera-views.md`
