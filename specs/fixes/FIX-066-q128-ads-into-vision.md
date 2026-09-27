---
id: FIX-066
date: 2026-09-28
type: answered-question
specs: [PROD-VISION, GAME-ADS]
questions: [Q-128]
---

## Problem

Q-128 (ad boards for children) was answered on 2026-09-27 ("passive boards only, own
cross-promotion first, legal check before any third-party ad; PROD-VISION updated
accordingly"), but PROD-VISION pillar 3 still said the boards were "under review" and
GAME-ADS still carried the "Conflict to resolve" note and listed Q-128 as open.

## Resolution

- PROD-VISION pillar 3: no ads in the UI; only passive in-world ad boards (no links, no
  tracking, no network), own cross-promotion first, legal check before third-party ads.
- GAME-ADS: new rule 6 (first campaigns = own cross-promotion) with test ADS-006 (manual);
  the conflict note now only says that CLAUDE.md ("no ads/external links") must be updated
  by the user; Q-128 marked answered.

## Changed files

- `specs/00-product/vision.md`, `specs/10-gameplay/ad-boards.md`
