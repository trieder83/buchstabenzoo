---
id: FIX-054
date: 2026-09-27
type: naming
specs: [PROD-GLOSSARY]
questions: []
---

## Problem

The night specs use terms that were not in the glossary: `daytime` and its phases
(`day`/`dusk`/`night`/`sleeping`/`morning`), night level (`night_<N>`, `time = "night"`),
night house / indoor enclosure (`indoor = true`). The `fog_end` entry still called Q-109 a
proposal.

## Resolution

Added the three entries (German terms marked † as proposals, Q-045); `fog_end` now says
"Q-109 answered".

## Changed files

- `specs/glossary.md`
