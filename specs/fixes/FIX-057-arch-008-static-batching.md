---
id: FIX-057
date: 2026-09-27
type: missing-tests
specs: [TECH-ARCH]
questions: []
---

## Problem

Static batching is tested in `crates/zoo-render/src/renderer.rs`
(`arch_008_baked_groups_match_their_instances`), but TECH-ARCH had neither a rule nor an
ARCH-008 test case. The ARCH test rows were also out of order (ARCH-005 after ARCH-007).

## Resolution

- Added a "Static batching" bullet to "Multi-node assets, material slots": static placements
  that never move are merged per group, and glass, hiding, spinning and night-only parts are
  never merged.
- Added the ARCH-008 row with the wording the caller supplied, and linked the test.
- Sorted the ARCH rows by number.

## Changed files

- `specs/40-tech/architecture.md`
