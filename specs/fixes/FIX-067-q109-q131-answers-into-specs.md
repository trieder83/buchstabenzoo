---
id: FIX-067
date: 2026-09-28
type: answered-question
specs: [GAME-CAMERA-VIEWS, GAME-CART, GAME-AMBIENT, ART-ANIMALS, ART-ENVIRONMENT, GAME-HINT, GAME-EVENTS]
questions: [Q-109, Q-110, Q-111, Q-112, Q-121, Q-122, Q-123, Q-124, Q-125, Q-127, Q-129, Q-130, Q-131]
---

## Problem

Q-109…Q-131 were answered "as recommended" on 2026-09-27 (commit 42bbaa6), but the answers
were never copied into the specs: GAME-CAMERA-VIEWS listed Q-109 as "partly answered" and
Q-110…Q-112, Q-123…Q-125 as open; GAME-AMBIENT rules 2 and 7 still had the old wording
(Q-121); ART-ANIMALS / ART-ENVIRONMENT still asked Q-122; GAME-HINT called the idle nudge
optional (Q-127); GAME-EVENTS called the storm repair a proposal (Q-130) and asked Q-131.

## Resolution

Copied each recommendation into its spec:
- GAME-CAMERA-VIEWS rule 2: `V` during look-around → first person (Q-124, new CAMV-023);
  look-around eye collision accepted for the PoC, pull-in ≥ 1.2 m next iteration (Q-111);
  rule 3 carried items as hands (Q-112); new rule 11: zoo view only while driving a golf cart
  (Q-125, new CAMV-024; GAME-CART rule 3 cross-reference); open-question list updated.
- GAME-AMBIENT rule 2: 2–4 adult ducks per river + up to 3 ducklings; rule 7: frog hops on
  the same pad or to a neighbouring pad ≤ 1 m away (Q-121, new AMB-014).
- ART-ANIMALS: which AANI tests apply to ambient animals, manifest entries after the user
  confirms the kit sheet counts (Q-122, new AANI-013); ART-ENVIRONMENT: static `duck`/`frog`
  removed from `kit_water`.
- GAME-HINT rule 6: idle nudge yes (90 s), hiding-area edge after 60 s (Q-127, new HINT-010).
- GAME-EVENTS: storm repair automatic (Q-130); bears are added (Q-131) — which level is a new
  question (Q-174).

## Changed files

- `specs/10-gameplay/camera-views.md`, `golf-carts.md`, `ambient.md`, `hints.md`, `events.md`
- `specs/30-art/animals.md`, `specs/30-art/environment.md`
