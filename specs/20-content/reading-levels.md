---
id: CONT-READING
title: Reading levels
aspect: content
module: reading-levels
status: draft
depends_on: [PROD-VISION]
test_prefix: READ
updated: 2026-09-26
---

# Reading levels

## Behaviour

| Reading level | Age | Text form | Support |
|---|---|---|---|
| `kiga` | 4–6 | single letters / very short words, always with picture | read-aloud on tap (Q-007/Q-008) |
| `klasse1` | 6–7 | simple words and 3–5 word sentences, syllable-friendly | read-aloud on request |
| `klasse2` | 7–8 | 1–2 sentences | read-aloud on request |
| `klasse3` | 8–9 | short stories / riddles, 3–5 sentences | none by default |

1. The reading level is chosen per profile at start (by a parent) and can be changed in settings.
2. Every piece of readable content is stored per level; gameplay code selects by level.
3. Font: clear sans-serif school font with single-storey "a" (Q-021 which font/licence).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| READ-001 | Given any content key, then it has a variant for each of the 4 reading levels (which keys are level-dependent: Q-038). | unit |
| READ-002 | Given `klasse1` content in any language (incl. all location riddles, CONT-MISSIONS), then no sentence has more than 5 words. | unit |
| READ-003 | Given the reading level is changed in settings, then the next displayed label uses the new level. | unit |

## Open questions

- Q-007, Q-008, Q-021, Q-035 grades 4–5, Q-038 level-dependent keys.
