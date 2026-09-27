---
id: FIX-045
date: 2026-09-27
type: naming
specs: [TECH-ARCH]
questions: []
---

## Problem

TECH-ARCH (`test_prefix: ARCH`) contained the z-fighting test `RENDER-001`, a prefix that
belongs to no spec. The code and the QA files used the same wrong id.

## Resolution

Renamed to the next free id **ARCH-005** in the spec (rule text and test table) and in all
references: `crates/zoo-core/tests/zfight.rs` (doc comment; test functions
`arch_005_no_z_fighting_in_the_joined_zoo`, `arch_005_detector_finds_the_entrance_case`),
comments in `crates/zoo-core/src/scene.rs`, `.claude/agents/gameplay-qa.md`,
`qa/checklist.md`. No behaviour change.

## Changed files

- `specs/40-tech/architecture.md`
- `crates/zoo-core/tests/zfight.rs`
- `crates/zoo-core/src/scene.rs` (comments only)
- `.claude/agents/gameplay-qa.md`
- `qa/checklist.md`
