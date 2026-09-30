---
id: GAME-ADS-C3
title: Ad campaign 3 — placeholder
aspect: gameplay
module: ad-boards
status: draft
depends_on: [GAME-ADS]
test_prefix: ADC3
updated: 2026-09-30
---

# Ad campaign 3 — placeholder

Slot 3 of 3 (GAME-ADS rule 2). Until a real campaign is added, the boards show the text
placeholder `ad-placeholder-3` ("Deine Werbung 3" / "Your ad 3"), passive, no link. Put a
campaign's resources (images, tagline, URL, rules) in this directory like
`../campaign-1-mathfighter/`.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| ADC3-001 | Given campaign 3 has no resources, then its boards show "Deine Werbung 3" / "Your ad 3" and are not interactable (ADS-003, ADS-004). | e2e |
