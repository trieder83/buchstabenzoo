---
id: FIX-015
date: 2026-09-26
type: answered-question
specs: [TECH-ARCH, ART-DIRECTION, ART-PIPELINE]
questions: [Q-010]
---

## Problem

Q-010 (art style) was still `open` in `open-questions.md` and still described the old
voxel vs. low-poly comparison, although the user decided on the comic style and the art
specs were already rewritten for it. TECH-ARCH did not mention that `zoo-render` must
produce the cel shading and outlines (ART-DIRECTION §2).

## Resolution

- Q-010 marked `answered` (user, 2026-09-26: comic style, single source
  `art/style/style.md`, prompts copy its blocks verbatim — APIPE-010) with the places it
  is specified.
- TECH-ARCH: new decision §7 (renderer draws 2-tone cel shading + outline; technique stays
  Q-050), `zoo-render` responsibility extended (cel shading, outline pass, face decal UV
  offset), new test ARCH-004 and an open-questions section (Q-050, Q-026).
- ART-DIRECTION §2 now points to TECH-ARCH §7; new manual test ADIR-005 (in-game cel
  shading/outline matches the style frame) so rules 1–2 have a test.
- Q-026 note: ART-PIPELINE §10 / ART-RIG §7 already assume option (a); the stale quote
  "atlas or vertex colours" was replaced.

## Changed files

- `specs/open-questions.md`, `specs/40-tech/architecture.md`, `specs/30-art/art-direction.md`,
  `specs/30-art/asset-pipeline.md`
