---
id: FIX-070
date: 2026-09-28
type: gap
specs: [PERF-MEASUREMENTS]
questions: []
---

## Problem

`specs/50-performance/measurements.md` (PERF-MEASUREMENTS) had frontmatter but no
"Test cases" and "Open questions" sections of the body template.

## Resolution

Added both sections: no test cases of its own (the tool and log are checked by PERF-015 in
PERF-BUDGETS, as PERF-RECOMMENDATIONS already states); open questions Q-013, Q-166…Q-169.

## Changed files

- `specs/50-performance/measurements.md`
