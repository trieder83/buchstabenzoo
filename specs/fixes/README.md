# Fixes log

Every problem the `spec-manager` agent finds **and resolves** (contradiction, gap, naming
inconsistency, missing tests, cleanup of outdated specs, answered open question) gets one
file here: `FIX-NNN-short-slug.md`. Numbers are never reused.

```markdown
---
id: FIX-001
date: 2026-09-26
type: cleanup            # contradiction | gap | naming | missing-tests | cleanup | answered-question
specs: [PROD-VISION]     # specs changed
questions: []            # Q-### resolved, if any
---

## Problem
## Resolution
## Changed files
```
