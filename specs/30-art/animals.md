---
id: ART-ANIMALS
title: Animals — concept and models
aspect: art
module: animals
status: draft
depends_on: [ART-PIPELINE, ART-DIRECTION, GAME-ANIMALS]
test_prefix: AANI
updated: 2026-09-26
---

# Animals — concept and models

## Goal

Every animal in GAME-ANIMALS has an approved turnaround sheet before it is modelled.

## Asset list

Candidate list — same 10 animals as GAME-ANIMALS / CONT-MISSIONS; final scope depends on
Q-002. Animations per animal are provisional until Q-043.

| Asset id | Animal | Variants | Animations | Status |
|---|---|---|---|---|
| `hippo` | Flusspferd | adult | `idle`, `walk`, `eat`, `swim`, `happy` | concept |
| `panda` | Panda | adult | `idle`, `walk`, `eat`, `happy` | concept |
| `zebra` | Zebra | adult | `idle`, `walk`, `eat`, `happy` | concept |
| `koala` | Koala | adult | `idle`, `climb`, `eat`, `happy` | concept |
| `elephant` | Elefant | adult | `idle`, `walk`, `eat`, `drink`, `happy` | concept |
| `goldfish` | Goldfisch | adult | `swim`, `eat` | concept |
| `monkey` | Affe | adult, **baby** (baby only if `quest_monkey_baby` stays — Q-040) | `idle`, `walk`, `climb`, `eat`, `happy`; baby: `hide`, `wave` | concept |
| `giraffe` | Giraffe | adult | `idle`, `walk`, `eat`, `happy` | concept |
| `lion` | Löwe | adult | `idle`, `walk`, `eat`, `happy` | concept |
| `snow_fox` | Schneefuchs | adult | `idle`, `walk`, `eat`, `happy` | concept |

## Behaviour

1. Every animal follows the common animation set `idle`, `walk` (or `swim`/`climb`),
   `eat`, `happy` — the game reacts identically to all animals.
2. Animals are stylised and friendly (big eyes, no teeth shown), same comic style (`art/style/style.md`) as the
   player character.
3. Turnaround for quadrupeds: front, left side, back, ¾ — standing pose, all four feet on
   the ground.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| AANI-001 | Given every animal in GAME-ANIMALS, then an asset with the same id exists in the manifest. | asset |
| AANI-002 | Given each animal `.glb`, then it contains `eat` and `happy` plus at least one locomotion animation. | asset |

## Open questions

- Q-002 Final animal list.
- Q-043 Animation set (hiding-place idles such as `drink`/`sleep`, reactions `not_interested`/`refuse`, locomotion for koala and goldfish while following).
- Q-040 Monkey baby needed?
