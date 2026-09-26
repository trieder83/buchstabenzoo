---
id: FIX-021
date: 2026-09-26
type: answered-question
specs: [GAME-LAYOUT, ART-ENVIRONMENT, GAME-LEVEL-1, PROD-GLOSSARY]
questions: [Q-057]
---

## Problem

Fence/hedge/wall models were 2 m only, while many level-1 edges have odd lengths; the
ART-ENVIRONMENT kit description still implied 2 m ground tiles and separate closed/open
gate models.

## Resolution

User decision 2026-09-26 written into the specs:
- GAME-LAYOUT "Modular edges": 1 m variants, fill rule (2 m segments from the run start,
  one 1 m segment at the end for odd lengths, `segment_run`), corner pieces at turns;
  behaviour 6, LAYOUT-012/013.
- ART-ENVIRONMENT "Built kits" table (`kit_ground` 1 m tiles, `kit_fences` 2 m + 1 m
  pieces, single `gate_wood`), behaviour 4, AENV-005/006.
- Glossary rows `run` (German term marked † proposal) and `segment`.
- Q-057 answer: section reference corrected from "Modular props" to "Built kits".
- The two level-design placement proposals (enclosure fence outline, bands as one centre
  row) are tagged Q-060 for confirmation; band joins stay open as Q-059.

## Changed files

- `specs/10-gameplay/layout.md`, `specs/10-gameplay/levels/level-1.md`
- `specs/30-art/environment.md`
- `specs/glossary.md`, `specs/open-questions.md`
