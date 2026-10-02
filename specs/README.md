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
  50-performance/       # aspect: budgets, measurement log, tracked recommendations (`performance` agent)
```

- One file per module. Use a subfolder (submodule) only when a module grows too large or a
  sub-topic is rarely needed (e.g. `10-gameplay/quests/`).
- Files starting with `_legacy-` are raw input kept for traceability; they are not specs.

## Frontmatter (required on every spec)

```yaml
---
id: GAME-FEED              # unique, UPPER-KEBAB, prefix = aspect (PROD, GAME, CONT, ART, TECH, PERF)
title: Feeding animals
aspect: gameplay           # product | gameplay | content | art | tech | performance
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

## Reading guide for `10-gameplay/` (by topic)

`INDEX.md` lists the specs by id; this is the same folder grouped by topic. A spec belongs to one
topic only; cross-topic rules are referenced by id, never copied.

| Topic | Specs (ids) | Read first |
|---|---|---|
| **Core loop** | GAME-RESCUE (loop, goldfish bowl, welcome board, intro), GAME-FEED (food boxes, carrying, bamboo), GAME-QUESTS (optional quests), GAME-HINT (🧭 hint, never stuck), GAME-SAVE | GAME-RESCUE |
| **World and levels** | GAME-WORLD, GAME-LAYOUT (rules, elements, gates, level design rules), GAME-LEVEL-1/2/3, GAME-LEVEL-NIGHT-1 (`levels/`), GAME-MAP | GAME-LAYOUT |
| **Animals and family** | GAME-ANIMALS (states, wandering, info board), GAME-FAMILY (pairs, babies), GAME-GARDEN (treats, feeding spot), GAME-AMBIENT | GAME-ANIMALS |
| **Player, input, camera** | GAME-PLAYER (controls, collision, ground height), GAME-CAMERA-VIEWS (look-around, first person), GAME-CART | GAME-PLAYER |
| **Night and events** | GAME-NIGHT (nightfall, night zoo, compass strip, bed), GAME-EVENTS (burglars, storm, bees) | GAME-NIGHT |
| **Economy and ads** | GAME-ECON (visitors, coins; idea), GAME-ADS + GAME-ADS-C1/C2/C3 (`ads/`) | GAME-ADS |

Status words: `draft` = specified, `implemented` = built and all its tests green. Spec text marked
*proposal* means a decision is still outstanding (Q-###); the answered ones are written as decisions.
Long specs (layout, levels, missions, environment) start with a **Contents:** line of their sections.
