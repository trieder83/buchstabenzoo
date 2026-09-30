---
id: GAME-ADS-C2
title: Ad campaign 2 — ABC Smash
aspect: gameplay
module: ad-boards
status: draft
depends_on: [GAME-ADS, GAME-ADS-C1, CONT-READING]
test_prefix: ADC2
updated: 2026-09-30
---

# Ad campaign 2 — ABC Smash

Own cross-promotion (GAME-ADS rule 6, Q-128): the reading game **ABC Smash**. Same mechanics
as campaign 1 ([`../campaign-1-mathfighter/campaign.md`](../campaign-1-mathfighter/campaign.md)):
readable board, link button behind the parental gate (Q-217), no tracking.

| Field | Value |
|---|---|
| `id` | `abcsmash` |
| Slot | campaign 2 of 3 (GAME-ADS rule 2) |
| Type | own cross-promotion |
| Tagline (Fluent `ad-abcsmash-tagline`) | de: **Lesen lernen – flüssig und schnell** · en: **Learn to read – fluent and fast** |
| Image text (in the picture, German) | "Schneller lesen — flüssig lesen! auf dem Weg zum ABC-Stern" · logo "ABC Smash" (rocket flying along the planets to the ABC star) |
| Link | https://abcsmash.rcms.ch — clickable when the child has read the ad in the game (Q-217, behind the parental gate) |
| Images | `resources/feature-graphic-de-de.png` (1024 × 500, RGB, 573 KB, German; shown for `de`) and `resources/feature-graphic-en-us.png` (English; shown for `en`) — sources `abcshooter/marketing/feature-graphic-{de-de,en-us}.png`. The game ships a compressed copy (proposal: ≤ 512 KB, 1024 px wide, `assets/ads/abcsmash/`, manifest kind `ad`, licence `own`). |

## Behaviour

1. The campaign appears on ≥ 2 boards (GAME-ADS rule 2), the wide banner fits the billboard.
2. Reading the ad, the link button, the parental gate and the night lighting work exactly as in
   campaign 1 (ADC1-002…005); the reading panel shows the picture, and in `klasse1+` the tagline.
3. The ad is only offered as a reading-related tip; it never blocks play and never appears in
   menus or reading panels of the missions.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| ADC2-001 | Given the campaign table, then `abcsmash` is active, appears on ≥ 2 boards and its image exists (`assets/ads/abcsmash/`, licence `own`) and loads. | unit |
| ADC2-002 | Given the player stands in front of an ABC Smash board, then a reading panel opens with the image of the current language (`de-de` / `en-us`) (and in `klasse1+` the tagline in the current language); on `kiga` only the picture. | e2e |
| ADC2-003 | Given the open panel, then its link button (≥ 64 px) targets https://abcsmash.rcms.ch and opens it only after the parental gate, in a new tab, without query parameters (as ADC1-003/004). | e2e |
| ADC2-004 | Given a session without pressing the link, then no network request to abcsmash.rcms.ch is made. | e2e |

## Open questions

- Q-217 (link behind a parental gate, shared with campaign 1).
