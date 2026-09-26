# Specs — conventions

The specs are the source of truth for Buchstabenzoo. Code, assets and tests follow the specs,
never the other way round. The `spec-manager` agent (`.claude/agents/spec-manager.md`)
maintains structure, index, consistency and open questions.

## Structure: Aspect → Module → Submodule

```
specs/
  README.md             # this file — conventions
  INDEX.md              # generated from frontmatter — do not edit by hand
  glossary.md           # one name per concept (de ↔ en ↔ code identifier)
  open-questions.md     # everything undefined, with owner and status
  00-product/           # aspect: why and for whom
  10-gameplay/          # aspect: what the player does
  20-content/           # aspect: texts, reading levels, localization
  30-art/               # aspect: look, concept art, 3D asset pipeline
  40-tech/              # aspect: architecture, rendering, platforms, testing
```

- One file per module. Use a subfolder (submodule) only when a module grows too large or a
  sub-topic is rarely needed (e.g. `10-gameplay/quests/`).
- Files starting with `_legacy-` are raw input kept for traceability; they are not specs.

## Frontmatter (required on every spec)

```yaml
---
id: GAME-FEED              # unique, UPPER-KEBAB, prefix = aspect (PROD, GAME, CONT, ART, TECH)
title: Feeding animals
aspect: gameplay           # product | gameplay | content | art | tech
module: feeding
status: draft              # draft | review | approved | implemented | deprecated
depends_on: [GAME-ANIMALS] # ids of specs this one relies on
test_prefix: FEED          # prefix of the test case IDs in this spec
updated: 2026-09-26
---
```

## Body template

```markdown
# <Title>

## Goal
## Behaviour            # numbered rules, testable statements
## Acceptance criteria
## Test cases           # table: ID | Given / When / Then | Level (unit, wasm, e2e, asset, manual)
## Open questions       # link to entries in open-questions.md (Q-###)
```

## Rules

- Every behaviour rule has at least one test case. Test IDs are `<test_prefix>-NNN`
  (e.g. `FEED-001`) and appear in the test code as a comment or in the test name.
- Use the terms from `glossary.md` exactly. New concept → add it to the glossary first.
- Anything undecided goes to `open-questions.md` as `Q-###` — never silently assumed.
- A spec may only move to `approved` when it has no open blocking questions.
