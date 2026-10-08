---
id: GAME-ADS-C3
title: Ad campaign 3 — EduGameGalaxy
aspect: gameplay
module: ad-boards
status: draft
depends_on: [GAME-ADS, GAME-ADS-C1, GAME-ADS-C2]
test_prefix: ADC3
updated: 2026-10-01
---

# Ad campaign 3 — EduGameGalaxy

Own cross-promotion (GAME-ADS rule 6, Q-128): the learning-game portal **EduGameGalaxy**, the home
of the sister games ABC Smash (star) and Math Fighter (plus). Same mechanics as campaign 1
([`../campaign-1-mathfighter/campaign.md`](../campaign-1-mathfighter/campaign.md)): readable board,
link button behind the parental gate, no tracking. The game knows only the id, slot 3 and the link
host `edugamegalaxy.rcms.ch` (Q-241).

| Field | Value |
|---|---|
| `id` | `edugamegalaxy` |
| Slot | campaign 3 of 3 (GAME-ADS rule 2) |
| Type | own cross-promotion |
| Tagline (signed manifest, Q-246, ≤ 80 characters) | de: **Effizient und mit Spaß lernen – für den Erfolg im Leben** · en: **Learn efficiently and with fun – for success in life** (full slogan: "EduGameGalaxy – effizient und mit Spaß lernen – für den Erfolg im Leben" / "EduGameGalaxy – learn efficiently and with fun – for success in life"; the brand name is in the picture) |
| Image | an anime-style, smiling invented child (no real person) holds a star (symbol of ABC Smash) and a plus sign (symbol of Math Fighter) in front of a friendly galaxy, same colour mood as campaigns 1/2. The picture is generated **without text** ([`art/ads/campaign-3-edugamegalaxy/brief.md`](../../../../art/ads/campaign-3-edugamegalaxy/brief.md), prompt + negative prompt + log); brand name "EduGameGalaxy" and a short line are set with Pillow (`make_banner.py`). |
| Link | https://edugamegalaxy.rcms.ch — clickable when the child has read the ad (Q-217), behind the parental gate |
| Images | sources `resources/edugamegalaxy-de.png` / `-en.png` (1024 × 500, German / English line) and the text-free `resources/v4.jpg`. Served copies (`tools/ads/prepare_images.py`): `boards/img/c3-de.webp` (language `de`) and `boards/img/c3-en.webp` (`en`), 1024 × 500, WebP, ≈ 60 KB, licence `own`, in the **signed manifest** (GAME-ADS rules 7–8). |
| Parental gate | the plus/minus task (ADS-018), like campaign 1 (ABC Smash alone asks the language question) |

## Behaviour

1. The campaign appears on ≥ 2 boards (GAME-ADS rule 2); the wide banner fits the billboard.
2. Reading the ad, the link button, the parental gate and the night lighting work exactly as in
   campaign 1 (ADC1-002…005); the panel shows the picture of the current language and, in
   `klasse1+`, the tagline.
3. Until its campaign is delivered and verified (or if it fails verification) the slot shows the
   passive placeholder `ad-placeholder-3` ("Deine Werbung 3" / "Your ad 3", ADS-003, ADC1-006).
4. The ad never blocks play and never appears in menus or reading panels of the missions.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| ADC3-001 | Given the repo, then both served images of `edugamegalaxy` exist in `boards/img/` (≤ 512 KB, 1024 px wide), the template manifest names them (languages `de`, `en`, taglines ≤ 80 characters), the test-signed fixture and the signed production manifest list campaign `edugamegalaxy` (slot 3, link host `edugamegalaxy.rcms.ch`, link exactly `https://edugamegalaxy.rcms.ch`), and the release build loads it (ADS-023). | unit + e2e |
| ADC3-002 | Given the player stands in front of an EduGameGalaxy board, then a reading panel opens with the image of the current language (`de` / `en`) and, in `klasse1+`, the tagline in the current language; on `kiga` only the picture. | e2e |
| ADC3-003 | Given the open panel, then its link button targets https://edugamegalaxy.rcms.ch and opens it only after the plus/minus parental gate (ADS-018), in a new tab, without query parameters. | e2e |
| ADC3-004 | Given a session without pressing the link, then no network request to edugamegalaxy.rcms.ch is made; any other link or host for this id is refused (ADS-015). | e2e + unit |

## Open questions

- None new (Q-217 answered, gate shared with campaign 1).

Store links (ADS rule 16): none; web only (`https://edugamegalaxy.rcms.ch`) on every platform.
