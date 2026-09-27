---
id: GAME-ADS
title: Ad billboards (in-world)
aspect: gameplay
module: ad-boards
status: draft
depends_on: [GAME-LAYOUT, ART-ENVIRONMENT, PROD-VISION]
test_prefix: ADS
updated: 2026-09-27
---

# Ad billboards (in-world)

## Goal

A **few ad billboards** in the zoo — on some building walls or beside the paths (user
request 2026-09-27). Up to **3 campaigns** run at the same time; each campaign is shown on
**several** boards. For now the boards show placeholders: **"Your ad 1"**, **"Your ad 2"**,
**"Your ad 3"** (de: *Deine Werbung 1/2/3*).

## Behaviour

1. **Boards:** 4–6 per level (proposal), placed by the level designer: on building walls
   (food storage, zookeeper house, kiosk) and a few free-standing boards beside main paths.
   Never on enclosures, info boards, the entrance welcome board, hiding places or where they
   would block a riddle's sight line; never inside the night house.
2. **Campaigns:** a small campaign table (data, not code): `id`, `image` (texture) or
   `text` (Fluent key), `boards` (which board ids, or "any"), active yes/no. At most **3
   active** campaigns; each campaign appears on ≥ 2 boards; campaigns rotate between boards
   per play session (seeded), so the same board may show another campaign next time.
3. **Placeholders now:** three campaigns with text only: `ad-placeholder-1/2/3` =
   "Deine Werbung 1/2/3" / "Your ad 1/2/3", big friendly lettering on a plain coloured board
   (rendered by the game's text-texture path, like the "Futter" sign).
4. **Passive only — child safety** (see Q-128): ads are **static pictures in the world**.
   They are **not interactable** (no tap, no link, no store, no popup, no video, no sound),
   collect **no data**, need **no network** (images ship with the game), never interrupt play
   and are never placed in the reading panels or the UI. Content rules: age-appropriate, no
   food/sweets marketing to children, no gambling, no in-app purchase hints (to be confirmed
   legally).
5. At night, boards are lit softly like other signs (GAME-NIGHT, category b).

> **Conflict to resolve:** PROD-VISION pillar 3 and CLAUDE.md say "no ads". This spec
> introduces passive in-world ad boards by user decision (2026-09-27); PROD-VISION and
> CLAUDE.md must be updated once Q-128 (legal/child-safety check) is answered.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| ADS-001 | Given a level, then it has 4–6 ad boards, none on enclosures, info boards, hiding places or blocking a riddle sight line (LAYOUT-L*-006 still holds). | unit |
| ADS-002 | Given the campaign table, then at most 3 are active and each active campaign is shown on ≥ 2 boards. | unit |
| ADS-003 | Given the placeholder campaigns, then boards show "Deine Werbung 1/2/3" (de) / "Your ad 1/2/3" (en) from Fluent. | e2e |
| ADS-004 | Given an ad board, then it is not interactable and the game makes no network request for ads. | e2e |
| ADS-005 | Given two play sessions with different seeds, then the campaign-to-board assignment differs, with the same seed it is identical. | unit |

## Open questions

- Q-128 Ads in a game for 4–9 year olds: legal/child-safety check (e.g. EU AVMSD/DSA, German
  JMStV, COPPA, app-store kids-category rules) — which ad content is allowed at all?
