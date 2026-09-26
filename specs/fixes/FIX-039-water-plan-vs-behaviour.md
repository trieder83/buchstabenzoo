---
id: FIX-039
date: 2026-09-26
type: contradiction
specs: [TECH-WATER]
questions: []
---

## Problem

Inside TECH-WATER the decision/plan sections contradicted the revised, implemented
behaviour: §3.1 said the field covers the level bounds (behaviour 2: bounding box of all
water + 2 m, one field for the joined zoo); §5 planned a "dilation into land texels"
(behaviour 2: no dilation pass); the parameter table and cost estimate still gave the
estimated 192 × 200 texels ≈ 300 KB per level and a bake < 5 ms, while the Implementation
section measured 188 × 148 texels (level 1), 368 × 308 texels ≈ 0.9 MB (joined zoo) and
17 / 34 ms.

## Resolution

Aligned §3.1, the §5 plan step, the "Field resolution" row and the cost estimate with
behaviour 2 and the measured values (estimate kept, measurement added).

## Changed files

- `specs/40-tech/water-rendering.md`
