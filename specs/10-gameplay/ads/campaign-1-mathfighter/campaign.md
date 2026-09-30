---
id: GAME-ADS-C1
title: Ad campaign 1 — Math Fighter
aspect: gameplay
module: ad-boards
status: draft
depends_on: [GAME-ADS, CONT-MATH, GAME-CART]
test_prefix: ADC1
updated: 2026-10-01
---

# Ad campaign 1 — Math Fighter

Own cross-promotion (GAME-ADS rule 6, Q-128): the maths game **Math Fighter**. Content (picture, tagline, link) is delivered by the signed external manifest; the game knows only this campaign's id, slot 1 and its link host `mathfighter.rcms.ch` (Q-241).

| Field | Value |
|---|---|
| `id` | `mathfighter` |
| Slot | campaign 1 of 3 (GAME-ADS rule 2) |
| Type | own cross-promotion (no third-party legal check needed, rule 6) |
| Tagline (signed manifest, `tagline.de` / `tagline.en`, Q-246) | de: **Lerne Mathe in einem lustigen Turnier** · en: **Learn math in a fun tournament** |
| Link | https://mathfighter.rcms.ch — clickable when the child has read the ad in the game (see below, Q-217) |
| Images | sources `resources/feature_graphic_5.png` (2722 × 1536, RGBA, 7.2 MB) and `resources/feature_graphic_2.png` (1360 × 768, RGB, 1.3 MB) from `mathegame/images/`. Served copies (`tools/ads/prepare_images.py`): `ads/img/mathfighter-wide.webp` and `ads/img/mathfighter-alt.webp` (1024 × 578, WebP, ≈ 80 KB each, licence `own`), listed with size and SHA-256 in the **signed manifest** (GAME-ADS rules 7–8); not part of the game build. |

## Behaviour

1. **Board look:** the campaign appears on ≥ 2 boards (GAME-ADS rule 2) as a picture board. The
   wide banner `feature_graphic_5` is the default image, `feature_graphic_2` the alternative
   (shown on boards with another aspect ratio, or alternating per session, seeded).
2. **Reading the ad:** like an info board (GAME-PLAYER §4/§5) an ad board of this campaign is
   *readable*: standing in front of it opens a **reading panel** with the image and the
   tagline (Fluent, per reading level: `kiga` shows only the picture, `klasse1+` the tagline).
   Ads of placeholder campaigns stay passive.
3. **Clickable link (user request 2026-09-30, Q-217 answered: option (a)):** the open panel shows
   a big link button (icon + "mathfighter.rcms.ch"). Pressing it shows the **parental gate**
   (GAME-ADS rule 11: a two-digit sum, then hold ✋ 3 s); only after it the URL opens in the
   **external browser / new tab** (`noopener,noreferrer`) — never inside the game. The game sends
   no request to the campaign host, only the browser opens the page on the parent's
   confirmation. No tracking parameters in the URL.
4. **Math link to the game:** the ad may mention that Math Fighter trains the same maths
   (CONT-MATH); it never blocks play and is never in the UI menus.
5. **Night:** the board is lit like the other signs (GAME-ADS rule 5).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| ADC1-001 | Given the repo, then both served images of `mathfighter` exist in `ads/img/` (≤ 512 KB, 1024 px wide), the template manifest names them (`tools/ads/campaigns.template.json`) and the test-signed fixture manifest lists campaign `mathfighter` (slot 1, link host `mathfighter.rcms.ch`); with a signed manifest the slot appears on ≥ 2 boards (ADS-002). | unit |
| ADC1-002 | Given the player stands in front of a Math Fighter board, then a reading panel opens with the image; in `klasse1+` also the tagline (de/en from Fluent); on `kiga` only the picture. | e2e |
| ADC1-003 | Given the open panel, then it has a link button ≥ 64 px with the host `mathfighter.rcms.ch`; pressing it first shows the parental gate and does **not** open anything yet (test build, ADS-020). | e2e |
| ADC1-004 | Given the parental gate was passed, then the URL opens in a new browser tab/external browser exactly once (`noopener`), with no query parameters; given the gate was failed or cancelled, nothing opens. | e2e |
| ADC1-005 | Given the game runs for a session without pressing the link, then no network request to mathfighter.rcms.ch is made (ADS-004 still holds for all other boards). | e2e |
| ADC1-006 | Given placeholder campaign 3 (and any board without a verified campaign), then its boards stay non-interactable (no panel, no link). | e2e |

## Open questions

- Q-217 answered 2026-09-30 (option (a)). Still to do before a store release: check the gate
  against Google Play Families / Apple Kids rules (Q-242); the user updates CLAUDE.md.
