---
id: PROD-VISION
title: Product vision
aspect: product
module: vision
status: draft
depends_on: []
test_prefix: VIS
updated: 2026-09-28
---

# Product vision

## Goal

**Buchstabenzoo** is a **reading education game** in 3D (third person). The zoo animals have
escaped and all enclosures are empty. The child brings them back by **reading and
understanding simple riddles** that tell where each animal is, and by reading food box
labels to pick the food that makes the animals follow. Optional **math tasks** (grades 1–5)
can be part of the way.

**Game goal:** every escaped animal is back in its enclosure.

## Pillars

1. **Reading comprehension is the key mechanic** — finding an animal always requires
   understanding a riddle at the child's reading level; guessing must be slower than reading.
2. **Math as a second subject** — optional math tasks at the child's math level (CONT-MATH).
3. **Friendly and pressure-free** — no violence, no ads in the UI; only passive in-world ad boards (no links, no tracking, no network; first own cross-promotion, legal/child-safety check before any third-party ad — GAME-ADS, Q-128 answered), no in-app purchases, no timers
   unless a spec says so.
4. **Explorable 3D zoo** — third-person, walk around, talk to visitors, discover places
   (e.g. the pirate ship).
5. **Multilingual** — German first, English in parallel, French later.

## Target audience

- Reading: age 4–9, reading levels `kiga`, `klasse1`, `klasse2`, `klasse3` (CONT-READING;
  extension to grades 4–5 open, Q-035).
- Math: grades 1–5 (CONT-MATH).
- Played on tablets and phones (touch), also in the desktop browser.

## Core loop

Per animal (GAME-RESCUE): empty enclosure → read the riddle on the info board → pick the
right food box by reading → find the animal where the riddle says → show the food → the
animal follows → lead it home → it eats and is happy → next enclosure.

## Scope — first playable

Defined by the answers to Q-002, Q-006 and Q-023.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| VIS-001 | Given any animal's rescue mission, then finding the animal requires a location riddle tagged with a reading level. | unit |
| VIS-002 | Given the built game, then it makes no network request to a third-party ad or tracking domain. | e2e |

## Open questions

- Q-001 Player role (goal answered: bring all escaped animals home).
- Q-034 Math integration, Q-035 reading grades 4–5.
