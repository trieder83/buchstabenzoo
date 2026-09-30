---
id: GAME-ADS-C1
title: Ad campaign 1 — Math Fighter
aspect: gameplay
module: ad-boards
status: draft
depends_on: [GAME-ADS, CONT-MATH, GAME-CART]
test_prefix: ADC1
updated: 2026-09-30
---

# Ad campaign 1 — Math Fighter

Own cross-promotion (GAME-ADS rule 6, Q-128): the maths game **Math Fighter**.

| Field | Value |
|---|---|
| `id` | `mathfighter` |
| Slot | campaign 1 of 3 (GAME-ADS rule 2) |
| Type | own cross-promotion (no third-party legal check needed, rule 6) |
| Tagline (Fluent `ad-mathfighter-tagline`) | de: **Lerne Mathe in einem lustigen Turnier** · en: **Learn math in a fun tournament** |
| Link | https://mathfighter.rcms.ch — clickable when the child has read the ad in the game (see below, Q-217) |
| Images | `resources/feature_graphic_5.png` (2722 × 1536, RGBA, 7.2 MB) and `resources/feature_graphic_2.png` (1360 × 768, RGB, 1.3 MB) — sources from `mathegame/images/`; the game ships compressed copies (proposal: ≤ 512 KB each, 1024 px wide, WebP/PNG, in `assets/ads/mathfighter/`, listed in `assets/manifest.toml` kind `ad`) |

## Behaviour

1. **Board look:** the campaign appears on ≥ 2 boards (GAME-ADS rule 2) as a picture board. The
   wide banner `feature_graphic_5` is the default image, `feature_graphic_2` the alternative
   (shown on boards with another aspect ratio, or alternating per session, seeded).
2. **Reading the ad:** like an info board (GAME-PLAYER §4/§5) an ad board of this campaign is
   *readable*: standing in front of it opens a **reading panel** with the image and the
   tagline (Fluent, per reading level: `kiga` shows only the picture, `klasse1+` the tagline).
   Ads of placeholder campaigns stay passive.
3. **Clickable link (user request 2026-09-30):** the open panel shows a big link button
   (icon + "mathfighter.rcms.ch"). Pressing it opens https://mathfighter.rcms.ch in the
   **external browser / new tab** — never inside the game. **Child-safety guard (proposal,
   Q-217):** a **parental gate** comes first (hold the button 3 s, or solve a small sum a
   young child cannot do) and the link opens only after it; the game sends no network request
   for the ad itself, only the browser opens the page on the parent's confirmation. No tracking
   parameters in the URL.
4. **Math link to the game:** the ad may mention that Math Fighter trains the same maths
   (CONT-MATH); it never blocks play and is never in the UI menus.
5. **Night:** the board is lit like the other signs (GAME-ADS rule 5).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| ADC1-001 | Given the campaign table, then `mathfighter` is active, appears on ≥ 2 boards and both images exist (`assets/ads/mathfighter/`, licence `own`) and load. | unit |
| ADC1-002 | Given the player stands in front of a Math Fighter board, then a reading panel opens with the image; in `klasse1+` also the tagline (de/en from Fluent); on `kiga` only the picture. | e2e |
| ADC1-003 | Given the open panel, then it has a link button ≥ 64 px with the URL https://mathfighter.rcms.ch; pressing it first shows the parental gate and does **not** open anything yet. | e2e |
| ADC1-004 | Given the parental gate was passed, then the URL opens in a new browser tab/external browser exactly once (`noopener`), with no query parameters; given the gate was failed or cancelled, nothing opens. | e2e |
| ADC1-005 | Given the game runs for a session without pressing the link, then no network request to mathfighter.rcms.ch is made (ADS-004 still holds for all other boards). | e2e |
| ADC1-006 | Given placeholder campaign 3 (and any board without a real campaign), then its boards stay non-interactable (no panel, no link). | e2e |

## Open questions

- Q-217 Clickable external link in a children's game (conflicts with GAME-ADS rule 4 / Q-128 "no
  links" and CLAUDE.md "no external links"): needs the user's confirmation, the parental gate
  and, for store release (Google Play Families / Apple Kids), a check of the store rules.
