---
id: FIX-069
date: 2026-09-28
type: naming
specs: [PROD-GLOSSARY]
questions: []
---

## Problem

The specs of 2026-09-27 use the terms *level gate* (`gate_zoo`, GAME-LAYOUT "Gates between
the levels", LAYOUT-036/038) and *bamboo forest* (harvestable `decoration` kind `bamboo`,
GAME-FEED §14–17), which were not in the glossary. `lying_item` said "a food or the fish
bowl" while GAME-FEED §8 also names the basket and the honey pot.

## Resolution

Added `level_gate` (model `gate_zoo`, *Zootor* †) and `bamboo_forest` (*Bambuswald*, the
existing `kiga` place word) to the glossary; `lying_item` now points to Q-172 for basket and
honey pot.

## Changed files

- `specs/glossary.md`
