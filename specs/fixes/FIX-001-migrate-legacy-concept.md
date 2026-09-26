---
id: FIX-001
date: 2026-09-26
type: cleanup
specs: [PROD-VISION, GAME-ANIMALS, GAME-FEED, GAME-QUESTS, CONT-READING, TECH-ARCH, TECH-PLATFORMS, ART-DIRECTION]
questions: []
---

## Problem

The first notes (`specs/game-concept.md`, `specs/game-architecture.md`) were unstructured,
had no frontmatter or tests, and proposed Unity — which contradicts the decision for
Rust + WebAssembly + raw WebGL2.

## Resolution

- Every point was moved into a structured spec (vision, animals, feeding, quests, reading
  levels, architecture, platforms, art direction).
- Unity replaced by Rust/WASM/WebGL2 (TECH-ARCH). The Unity scripts in `Assets/Scripts/`
  stay as gameplay reference only.
- Unclear points became open questions: "voxel" vs. `z.webp` style (Q-010), "3x hippo pool
  + stone" (Q-004), "bücke" (Q-005).
- "Cage" renamed to **enclosure** / *Gehege* everywhere (glossary).
- Both legacy files deleted.

## Changed files

- deleted: `specs/game-concept.md`, `specs/game-architecture.md`
- added: all specs under `specs/00-product` … `specs/40-tech`, `glossary.md`, `open-questions.md`
