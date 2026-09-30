---
id: GAME-ADS-C2
title: Ad campaign 2 — placeholder
aspect: gameplay
module: ad-boards
status: draft
depends_on: [GAME-ADS]
test_prefix: ADC2
updated: 2026-09-30
---

# Ad campaign 2 — placeholder

Slot 2 of 3 (GAME-ADS rule 2). Until a real campaign is added, the boards show the text
placeholder `ad-placeholder-2` ("Deine Werbung 2" / "Your ad 2"), passive, no link. Put a
campaign's resources (images, tagline, URL, rules) in this directory like
`../campaign-1-mathfighter/`.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| ADC2-001 | Given campaign 2 has no resources, then its boards show "Deine Werbung 2" / "Your ad 2" and are not interactable (ADS-003, ADS-004). | e2e |
