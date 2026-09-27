---
id: FIX-071
date: 2026-09-28
type: cleanup
specs: []
questions: []
---

## Problem

`INDEX.md` was out of date: the new aspect `performance` (`specs/50-performance/`:
PERF-BUDGETS, PERF-MEASUREMENTS, PERF-RECOMMENDATIONS) was missing; new tests FEED-009…026,
LAYOUT-033…038, LAYOUT-L3-017/018, CAMV-023/024, AMB-014, AANI-013, HINT-010, ADS-006;
Q-155…Q-158, Q-166…Q-174 added, Q-128, Q-133…Q-140, Q-148, Q-150, Q-157 now answered.

## Resolution

Regenerated `INDEX.md` from frontmatter with a "Performance" table after "Tech":
- 40 specs: draft 38, implemented 2.
- 509 active test cases (the `PERF-R-###` rows of PERF-RECOMMENDATIONS are recommendation
  ids, not test cases, and are not counted).
- Questions: answered 67, open 91, partly answered 7, proposed 2. Numbers Q-159…Q-165 were
  never used (skipped by a parallel agent); the gap stays, new questions continue after the
  highest number (Q-174).
- `Q-AAA…Q-BBB` ranges in a spec now count every question in the range.

No frontmatter errors, duplicate spec ids, duplicate test ids, test-prefix mismatches,
unknown dependencies, unknown Q-### references or dependency cycles (after FIX-064).

## Changed files

- `specs/INDEX.md`
