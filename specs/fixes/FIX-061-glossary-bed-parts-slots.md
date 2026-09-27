---
id: FIX-061
date: 2026-09-27
type: naming
specs: [PROD-GLOSSARY]
questions: []
---

## Problem

Today's specs use concepts that the glossary does not define:
- `bed` (the `[[item]] kind = "bed"`, now in two levels)
- model `part` (TECH-ARCH, ARCH-006/007)
- `material_slot` (`*_glow`, `eye_glow`, `glass`, `*_face`)
- `static_batch` (ARCH-008; already used by `render_region`)

## Resolution

Added one glossary row per term. Each row uses the definition already given in its spec.

## Changed files

- `specs/glossary.md`
