---
id: FIX-022
date: 2026-09-26
type: naming
specs: [PROD-GLOSSARY]
questions: []
---

## Problem

The glossary rows `map`, `explored`, `fog` stood after the "† German term is a proposal"
footnote, so they were cut off from the table and did not render as table rows.

## Resolution

Moved the three rows into the table above the footnote.

## Changed files

- `specs/glossary.md`
