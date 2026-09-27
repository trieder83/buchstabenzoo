---
id: FIX-059
date: 2026-09-27
type: contradiction
specs: [GAME-CAMERA-VIEWS, GAME-NIGHT, PROD-GLOSSARY]
questions: [Q-126]
---

## Problem

The close-view fog end is 20.8 m since FIX-056 (CAMV rule 6/7, CAMV-008). Three places still
said 16 m:
- CAMV-021: "the fog end stays 16 m" at night.
- GAME-NIGHT open questions: "16 m fog end kept".
- The glossary `fog_end`: "16 m, Q-109".

Q-126 decided that the night keeps the **day** fog end, not a fixed 16 m.

## Resolution

- CAMV-021 now reads "stays the day fog end (20.8 m, `FOG_END_M`)".
- The GAME-NIGHT Q-126 note now reads "day fog end kept (20.8 m since FIX-056)".
- The glossary `fog_end` is updated to 20.8 m (was 16 m, Q-109) and points to the ≥ 22 m sight
  rule.

Check whether the CAMV-021 test code still asserts 16 m.

## Changed files

- `specs/10-gameplay/camera-views.md`, `specs/10-gameplay/night.md`, `specs/glossary.md`
