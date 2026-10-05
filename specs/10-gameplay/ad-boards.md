---
id: GAME-ADS
title: Ad billboards (in-world)
aspect: gameplay
module: ad-boards
status: draft
depends_on: [GAME-LAYOUT, ART-ENVIRONMENT, PROD-VISION, TECH-PLATFORMS]
test_prefix: ADS
updated: 2026-10-04
---

# Ad billboards (in-world)

## Goal

A **few ad billboards** in the zoo — on some building walls or beside the paths (user
request 2026-09-27). Up to **3 campaigns** run at the same time; each campaign is shown on
**several** boards. For now the boards show placeholders: **"Your ad 1"**, **"Your ad 2"**,
**"Your ad 3"** (de: *Deine Werbung 1/2/3*). The real campaign content is **loaded externally
and accepted only if signed by us** (rules 7–9, threat model below).

**Campaign directories (user request 2026-09-30):** each of the 3 campaigns has its own
directory with its resources and spec: [`ads/campaign-1-mathfighter/`](ads/campaign-1-mathfighter/campaign.md)
(Math Fighter, images + tagline + link), [`ads/campaign-2-abcsmash/`](ads/campaign-2-abcsmash/campaign.md) (ABC Smash, reading game,
image + tagline + link) and [`ads/campaign-3-edugamegalaxy/`](ads/campaign-3-edugamegalaxy/campaign.md) (EduGameGalaxy, image + tagline + link; its slot stays a placeholder only while no valid campaign is delivered).

## Behaviour

1. **Boards:** 4–6 per level (implemented: 4 per level, 12 in the joined zoo), free-standing
   on open grass beside the main paths, **facing south** (the camera side). Level data
   `[[ad_board]]` (`id`, `pos` = board centre, `facing`): a wooden frame on two posts, picture
   2.4 × 1.2 m, footprint 2.6 × 0.3 m, solid (GAME-PLAYER 9). Never on enclosures, info boards,
   the entrance welcome board, hiding places (overlay rects), scenery, gardens, where they
   would block a riddle's sight line or within 4 m of an info board / 4 m of a food box / 5 m
   of a gate or door (LAYOUT-032…039 keep holding); never inside the night house. Boards on
   building walls are not implemented (Q-244). The positions were chosen under these rules by
   the implementer and need a review by `zoo-level-designer`.
2. **Campaigns and slots:** at most **3 campaign slots**; a session seed assigns every board
   one slot (`zoo_core::ads::assign_slots`: shuffle by seed + board ids, dealt round-robin over
   the joined zoo) so that every slot is on ≥ 2 boards (ADS-002) and another seed gives another
   assignment (ADS-005). What a slot shows is decided by the host (rule 3/7).
3. **Placeholders:** a slot without a verified campaign shows `ad-placeholder-1/2/3` =
   "Deine Werbung 1/2/3" / "Your ad 1/2/3" (Fluent, big lettering on a cream board, drawn by the
   host's text-texture path like the "Futter" sign). They are passive (rule 4) and are what the
   boards show whenever an external campaign is missing, late, invalid or not signed by us.
4. **Passive unless an own campaign:** placeholders and every board without a verified own
   campaign are **not interactable** (no tap, no link, no popup, no video, no sound), collect
   **no data** and never interrupt play (see Q-128). **Exception (user decision 2026-09-30,
   Q-217 answered: option (a)):** a board whose slot shows a verified own campaign
   (`mathfighter`, `abcsmash`, `edugamegalaxy`) is *readable*: a reading panel with the picture, the tagline
   (`klasse1+`) and a link button; the link opens only after the parental gate (rule 8). Content
   rules: age-appropriate, no food/sweets marketing to children, no gambling, no in-app purchase
   hints (to be confirmed legally).
5. At night the boards are lit softly like the other signs (GAME-NIGHT, category b).
6. **First campaigns** (Q-128 answered, user 2026-09-27): own cross-promotion (Math Fighter, ABC
   Smash, EduGameGalaxy); a legal/child-safety check is required before any third-party ad.

### External content (user requirements 2026-09-30, Q-240…Q-247)

7. **Loaded from outside, accepted only if signed by us.** The campaign content is **not part of
   the game build**: `ads/campaigns.json` (manifest), `ads/campaigns.sig` (detached signature)
   and `ads/img/*` are served from our own origin (`<origin>/ads/`, same origin: CSP
   `connect-src 'self'` / `img-src 'self'` stay unchanged) and can be swapped without an app
   update (`ads/**` cache: `max-age=300, must-revalidate`, PLAT-010). The manifest is
   authenticated by an **Ed25519** signature (over `"buchstabenzoo-ads/1\n"` + the exact manifest
   bytes); the **public key is compiled into the game** (`web/src/ad-keys.ts`, ≤ 2 keys for a
   rotation), the private key never enters the repo (`tools/ads/keygen.py`, `sign.py`,
   `tools/ads/README.md`). Until the owner has put a public key into `ad-keys.ts` the game makes
   **no request at all** and shows placeholders.
8. **Verification (host, `web/src/ads.ts`) before anything is shown**; any failure → that
   campaign (or the whole manifest) keeps the placeholder, silently, never unsigned content:
   - signature valid for a compiled key (ADS-008/009); `format` = `buchstabenzoo-ads/1`;
   - `version` ≥ the last accepted one (stored in `localStorage` `zoo.ads.version`; no rollback,
     ADS-010); `issued` not more than 1 day in the future, now ≤ `valid_until` (ADS-011);
   - ≤ 3 campaigns; each campaign id, **slot and link host are compiled into the game**
     (`mathfighter` → slot 1 → `mathfighter.rcms.ch`, `abcsmash` → slot 2 → `abcsmash.rcms.ch`,
     `edugamegalaxy` → slot 3 → `edugamegalaxy.rcms.ch`,
     Q-241); unknown ids, a wrong slot, duplicates, `active = false` are not shown (ADS-011);
   - link: https only, exactly the allowlisted host of *that* campaign, no user info, port,
     path, query or fragment; the game opens the canonical URL it builds itself (ADS-015);
   - tagline per language (de, en): plain text, 1…80 characters, no `< > & " \` or control
     characters; shown only with `textContent`, never as HTML (ADS-016);
   - per image: path `img/<name>` (no traversal, no scheme), type by **magic bytes** PNG / WebP /
     JPEG equal to the declared type, exact declared byte size ≤ 512 KB, dimensions from the
     header = declared (64…2048 px), **SHA-256 equal to the signed value**, browser decode
     gives the same dimensions (ADS-012/013/014).
   Loading starts after the first frame (never blocks play), same origin only, `credentials:
   omit`, no referrer, timeout **4 s for the manifest and 15 s for the images** (a phone on mobile
   data needs more than 4 s for up to 3 × 512 KB; user report 2026-10-03, ADS-029); a failed load
   is retried **once** after 20 s (no loop); offline → placeholders. The
   verified images live in memory only (Q-243).
9. **Test key hook.** `?adkey=<base64 public key>` replaces the compiled keys **only** in the dev
   server and in the e2e test build (`VITE_AD_TEST=1`, `dist-adtest`, Q-245); the release build
   contains no such code path (PLAT-012, ADS-019). Test fixtures and the TEST-ONLY key live in
   `web/tests/fixtures/ads/`.
10. **Reading panel** (GAME-PLAYER §4 pattern; Q-247; touch rules user report 2026-10-03): a
    readable board is "near" when the player is within **3.5 m** of its centre and on its
    readable side (in front of the picture, up to 3 m beside its centre) — **no facing
    requirement** (the touch stick often leaves the child looking elsewhere). Then the panel
    opens at once (image, tagline in `klasse1+` — `kiga` only the picture — and a link button
    ≥ 64 px with the host name as text); it appears as soon as the campaign is verified, also if
    the child arrived first. It closes by ✖ / Esc / walking away; a ✖ tap within 400 ms after
    the panel opened is ignored (accidental touch). After closing it stays closed until the
    player leaves and comes back **or presses the interact button**: on touch an extra button
    (🔗, same place as the interact button) shows while a verified board is near and its panel
    is closed (also the interact key on desktop); it (re)opens the panel. The panel never opens
    while another reading panel is open; the hint and interact target ignore ad boards.
    **Small screens** (landscape height ≤ 460 px, portrait width ≤ 480 px; GAME-PLAYER §3):
    the panel is a **compact card** — picture left, tagline + link button right (landscape), or
    below the gear/compass in portrait — and never covers the right-hand control column
    (gear, compass, view button, interact button).
11. **Parental gate** (Q-242): pressing the link button shows a modal gate: (1) for the maths campaign a **simple plus or
    minus task up to 20** (user request 2026-10-01: e.g. 9 + 7 or 15 − 6; numbers and result 0…20) with 4 number buttons — a wrong
    answer or ✖ ends the gate; (2) hold the ✋ button **3 s** (progress ring; releasing resets;
    a small finger movement does not release; no context menu / scrolling on the button). When
    the hold is complete a big button (≥ 64 px, icon + "mathfighter.rcms.ch") appears; **the
    child / parent TAPS it** and only that tap opens the link: an `<a href=<canonical URL>
    target=_blank rel="noopener noreferrer">` built by the game, whose click handler calls
    `window.open(url, '_blank', 'noopener,noreferrer')` **once** inside the user gesture (phone
    browsers — iOS Safari, Samsung Internet — block pop-ups started from a timer, which is what
    the end of a 3 s hold is). No analytics, no query parameters, no request to the campaign
    host by the game (ADC1-005).

12. **All-done carousel** (user request 2026-10-04, Q-364): when the 🧭 compass reports
    `next = all_done` (every level that exists is solved, HINT-028/030 — not merely "nothing to
    do right now") and the child taps it, a panel opens instead of the plain bubble: headline
    `ad-carousel-title` ("Du magst Bildungsabenteuer? Schau mal hier!" / "You like educational
    adventures? Check this out!", Fluent de + en) and a **carousel** of the verified campaigns in
    slot order (all of them, picture for the current language; never placeholders). It advances by
    itself every **4 s** (`CAROUSEL_MS`, frozen while the gate is up, restarted by any manual
    move), has big **◀ ▶** buttons (64 px) and one **dot** per slide (current one marked), a swipe
    (≥ 40 px horizontal) turns the page, wraps around, shows the tagline (`klasse1+`, hidden on
    very small landscape screens) and a ✖ (same 400 ms accidental-touch guard) / Esc closes it.
    Tapping the picture starts the **same parental gate** as rule 11 (campaign-specific question,
    3 s hold, release opens the link directly, ADS-030) — the link is never opened without it. With
    **no verified campaign** the panel is not offered and the normal all-done bubble shows. It
    uses the images already in memory: no extra request, no tracking. While it is open the board
    reading panels stay closed. Small screens: a compact card left of the control column
    (landscape) / below the compass (portrait), never covering `#settings-btn`, `#compass-btn`,
    `#view-btn`, `#act`.

### Threat model (what the signature does and does not protect)

| Attacker | Can | Cannot |
|---|---|---|
| Network (MITM, bad Wi-Fi, CDN / proxy cache poisoning) | delay or block the ads (placeholders) | show any content: no valid signature, images must match the signed hashes; replaying an old signed manifest works only until `valid_until` and never below the last seen `version` |
| Someone who can write only `ads/**` on the hosting (leaked upload token, bad content deploy) | delete the content | publish content without the private key; swap images (hash), change links/taglines (signed), add a campaign, point a link elsewhere (compiled allowlist) |
| Someone who controls the **whole hosting origin** (incl. the game bundle) | replace the game, its compiled key and its code | — the signature cannot help here (same as for any web game); the store apps (Capacitor) carry the key inside the signed binary and run the packaged bundle |
| Holder of the private key | show any content of a **known campaign id** (images, taglines, `active`) | add a new campaign id or another link host without a game release (Q-241) |
| A child | tap / hold the gate by accident (needs the right plus/minus task **and** 3 s holding) | — (a grade-1/2 child can solve the task, so the gate only stops accidental taps, not a determined child; Q-242) |

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| ADS-001 | Given a level, then it has 4–6 ad boards with unique ids, none on enclosures, info boards (≥ 4 m), hiding places, scenery, gardens, within 4 m of a food box, 5 m of a gate/door, or on a path/solid cell (LAYOUT-L*-006, -032…039 still hold). | unit |
| ADS-002 | Given the joined zoo and any seed, then every one of the 3 slots is on ≥ 2 boards and all boards have a slot. | unit |
| ADS-003 | Given no campaign is loaded (or a slot's campaign is not delivered), then every board has its picture texture and the placeholders read "Deine Werbung 1/2/3" (de) / "Your ad 1/2/3" (en) from Fluent. | e2e |
| ADS-004 | Given an ad board whose slot has no verified campaign (or any board without a verified own campaign), then it is not interactable (no panel, no interact target) and the game makes no network request for ads — in the release build (no key compiled) no request at all. | e2e |
| ADS-005 | Given two play sessions with different seeds, then the slot-to-board assignment differs; with the same seed (and any order of the data) it is identical. | unit |
| ADS-006 | Given the active campaigns, then every one is own cross-promotion unless a legal/child-safety check is recorded for it (rule 6, Q-128). | manual |
| ADS-007 | Given the player position, then a board is "near" only within 3.5 m, in front of its picture (not behind, not far beside, any facing, nearest wins). | unit |
| ADS-008 | Given a manifest signed by `tools/ads/sign.py` with the matching key, then the host accepts it and its images (cross-check Python signer ↔ TypeScript verifier). | unit |
| ADS-009 | Given a wrong key, a changed manifest byte, a changed signature, a missing domain prefix or no key, then nothing is shown and no image is fetched. | unit + e2e |
| ADS-010 | Given a manifest `version` lower than the last accepted one, then it is refused; equal or higher is accepted and remembered; a refused manifest does not move the mark. | unit |
| ADS-011 | Given an expired / not-yet-valid / wrong-format manifest, an unknown campaign id, a wrong slot, a duplicate slot or `active = false`, then it (or the campaign) is not shown. | unit |
| ADS-012 | Given an image whose bytes do not match the signed SHA-256 (tampered, swapped), then its campaign keeps the placeholder and the other campaigns still show. | unit + e2e |
| ADS-013 | Given an image over 512 KB (declared or served), dimensions outside 64…2048 px or different from the file header, then its campaign is dropped. | unit |
| ADS-014 | Given an image whose magic bytes differ from the declared type (or are not PNG/WebP/JPEG), or a path with traversal / scheme / outside `img/`, then the campaign is dropped. | unit |
| ADS-015 | Given a link with http, another host, another campaign's host, user info, port, path, query, fragment or a look-alike host, then the campaign is dropped; a valid one is opened in its canonical form. | unit |
| ADS-016 | Given a tagline with markup characters, control characters or over 80 characters, then the campaign is dropped; shown texts are set as text only. | unit |
| ADS-017 | Given no key / a missing file / a hanging server / offline, then placeholders remain within the timeout (4 s manifest, 15 s images); only same-origin `ads/` URLs are requested. | unit |
| ADS-024 | Given a browser without Web Crypto (plain http on a LAN IP, e.g. the phone on the dev server http://192.168.x.x:5173), then the signature check and the SHA-256 image hashes use the pure-JS fallback (@noble/hashes) and give exactly the same result (a tampered manifest is still rejected), so the signed ads show there too. | unit |
| ADS-025 | Given the gate of the reading campaign (ABC Smash), then it asks a language question instead of a sum: German a noun and its right article (e.g. "… Gabel" → der / die / das, answer die), English the right plural (one mouse → mice) with 4 answers; the maths campaign keeps the plus/minus task up to 20. | unit, e2e |
| ADS-018 | Given the parental gate, then (after the right answer and 3 s holding the open button appears; `window.open` is not called before it is tapped) holding without the right answer never opens, a wrong answer ends it, the right answer + 3 s holding opens it, releasing early resets; the task is a plus or minus task with numbers and result in 0…20 and 4 distinct answers; the gate shows no "adults only" claim (title "Zur Webseite" / "To the website"). | unit |
| ADS-019 | Given `?adkey=` in a build without the test hook, then it is ignored. | unit |
| ADS-020 | Given a correctly signed test manifest (test build), then the boards show the campaign pictures, the panel shows picture + tagline + link button ≥ 64 px, and the link opens (`noopener`, canonical URL) exactly once, only after the gate and the tap on the open button. | e2e |
| ADS-021 | Given a manifest signed with a wrong key (test build) or a tampered image, then the placeholders remain / only that campaign is missing, and no panel opens. | e2e |
| ADS-022 | Given the test build without `?adkey=`, then the test manifest is not trusted and not even requested. | e2e |
| ADS-023 | Given the RELEASE build (production key compiled in, no `?adkey=`) and the real signed `ads/campaigns.json`, then campaigns 1, 2 and 3 are accepted and loaded (needs the owner's public key in `ad-keys.ts` and a signed manifest; `web/tests/e2e/ads_prod.spec.ts`). | e2e |
| ADS-026 | Given an emulated phone (touch, 780×360, 360×780, 412×892), when the player stands in front of a verified board (any facing, 1…3.4 m), then the panel and the link button (≥ 56 px) are inside the viewport and tapping the button starts the gate. | e2e |
| ADS-027 | Given a phone, a verified board is near and the panel was closed by ✖ (or the ✖ was tapped < 400 ms after opening: ignored), then the interact button (🔗) is visible and tapping it reopens the panel; leaving and coming back also reopens it; the manifest arriving late opens the panel for a child already standing there. | e2e |
| ADS-028 | Given a small screen (780×360), then the open panel does not overlap `#settings-btn`, `#compass-btn`, `#view-btn` and `#act` (bounding boxes) and is a compact card; at 360×780 it does not overlap them either. | e2e |
| ADS-029 | Given the gate: after the right answer + 3 s touch hold (finger moving 10 px does not release; no context menu) an open button ≥ 64 px with an anchor (`href` canonical, `target=_blank`, `rel=noopener noreferrer`) appears and `window.open` has NOT been called; tapping it calls `window.open` exactly once and closes panel and gate. A slow image download (> 4 s, < 15 s) still shows the campaign; a failed load is retried once. | e2e + unit |
| ADS-030 | Given the gate (user request 2026-10-04: "the link should open directly, not show the link again"): after the right answer + 3 s touch hold the hold button shows ✔ (`#ad-hold.ready`) and `window.open` has NOT been called (a timer is no user gesture); releasing the finger then opens the link directly, exactly once (`_blank`, `noopener,noreferrer`) and closes panel and gate; an early release opens nothing; there is no second "open" button. Replaces the tap-button part of ADS-029. | e2e |
| ADS-031 | Carousel logic (`carouselItems`, `Carousel`): items = verified campaigns in slot order, one image each for the language, empty without content; `next`/`prev` wrap, `go` wraps, auto-advance after exactly 4 s, a manual move restarts the timer, a paused timer never advances, a single slide never advances. | unit |
| ADS-032 | Given all levels solved (`next = all_done`) and a verified campaign set, when the compass is tapped, then the carousel opens with the headline (de / en) and the first campaign, ◀ ▶ are ≥ 64 px, there is one dot per campaign; ▶ / ◀ / a dot / the 4 s timer change the slide and wrap; ✖ and Esc close it. | e2e |
| ADS-033 | Given the carousel, when the picture is tapped, then the parental gate starts and `window.open` is not called; after the right answer + 3 s hold + release the link of the shown campaign opens exactly once and the carousel closes; the timer does not advance while the gate is up. | e2e |
| ADS-034 | Given `next = all_done` but no verified campaign (no key / failed load) or `next` is not `all_done`, when the compass is tapped, then no carousel opens (the normal bubble shows). On 780x360 and 360x780 the carousel is inside the viewport, its buttons are ≥ 64 px and it overlaps none of `#settings-btn`, `#compass-btn`, `#view-btn`, `#act`. | e2e |

## Open questions

- Q-217 answered 2026-09-30: readable own boards with a link behind the parental gate. CLAUDE.md
  ("no ads / no external links") to be updated by the user.
- Q-240 signature scheme and key custody, Q-241 compiled campaign ids / link hosts, Q-242 gate
  difficulty, Q-243 caching / offline / Capacitor origin, Q-244 board placement and review,
  Q-245 test-key hook, Q-246 taglines from the manifest (not Fluent), Q-247 panel behaviour.
- Q-364 all-done carousel (rule 12): implemented as recommended; open for the user.
- Q-128 answered 2026-09-27: passive boards only (no links, no tracking, no network), own
  cross-promotion first, legal/child-safety check before any third-party ad (rules 4, 6).
