---
id: GAME-ADS
title: Ad billboards (in-world)
aspect: gameplay
module: ad-boards
status: draft
depends_on: [GAME-LAYOUT, ART-ENVIRONMENT, PROD-VISION, TECH-PLATFORMS]
test_prefix: ADS
updated: 2026-10-08
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

1. **Boards:** every level has boards of **all three campaign slots** (user request 2026-10-06:
   "1 of each of the 3 ad groups in every level"). Day levels `level_1…3`: 4 boards each (12 in the
   three day levels), `variant = "post"`: free-standing on open grass beside the main paths, **facing
   south** (the camera side). Night levels `night_1`, `night_2`: **exactly 3 boards each**, one per slot,
   by default `variant = "poster"`, at most one `flyer` (rule 1a). Level data `[[ad_board]]` (`id`, `pos` = board centre,
   `facing`, `variant`, default `post`). A post board is a wooden frame on two posts, picture
   2.4 × 1.2 m, footprint 2.6 × 0.3 m, solid (GAME-PLAYER 9). Never on enclosures, info boards,
   the entrance welcome board, hiding places (overlay rects), scenery, gardens, where they
   would block a riddle's sight line or within 4 m of an info board / 4 m of a food box / 5 m
   of a gate or door (LAYOUT-032…039 keep holding); never inside the night house or the terrarium
   house hall. Boards on building walls of the day levels are not implemented (Q-244); the night
   levels use posters instead. The positions were chosen under these rules by the implementer and
   need a review by `zoo-level-designer`.
1a. **Variants** (user request 2026-10-06; Q-377):
   - `post`: the free-standing sign above (day levels; one in `night_1`).
   - `poster`: a **framed poster mounted flat on a wall face** (building facade, hedge): picture
     2.0 × 1.0 m (same 2 : 1 texture, same host pipeline, same placeholder texts), frame 0.06 m,
     0.05 m deep, lower edge 0.9 m above the ground. `pos` is the centre **on the wall face**
     (the plane of the wall), `facing` the outward normal; it has **no footprint of its own** (the wall
     is solid) and no posts. A night poster gets a small **board lamp** (the wall-board socket of the
     info boards) that lights it softly (rule 5). Placement: only `facing = "-z"` (the camera side;
     a poster facing east or west is edge-on to the ~55° camera), the cell behind the poster's whole
     width is solid and the stand area in front (2 m wide, 3 m deep) is open walkable ground
     without tree canopy (nothing that blocks the view to the poster); not over a door, window,
     info board or lamp, and the same clearances as a post board (≥ 4 m to an info board / food box,
     ≥ 4 m to a gate or door (no footprint, so no walkway is needed; post boards keep 5 m), not on hiding
     places or scenery).
   - `flyer` (user request 2026-10-06): a **paper flyer lying flat on the ground** (picture
     1.2 × 0.6 m, cream sheet 2.5 cm thick, top edge to the north so the ~55° camera from the south
     reads it upright). `pos` = centre. Not solid (no collider, walk over it); read from **any side**
     within 2.5 m (it has no front). It stands on open **grass** (walkable, not a path cell), keeps
     the post board's clearances (≥ 4 m to info boards and food boxes, 4 m to gates/doors, not on
     hiding places, scenery, gardens, lamps) so a free stand area is next to it. **At most one flyer
     per night level**; the other boards of that level are posters (or posts). Used once in `night_1` and once in `night_2`.
   - Reading range, panel, 🔗 button, parental gate and carousel are **identical** for all variants
     (rule 10, ADS-007).
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
5. At night the boards are lit softly like the other signs (GAME-NIGHT, category b): the picture
   shows in warm lamp light, and every night poster has its own small board lamp.
6. **First campaigns** (Q-128 answered, user 2026-09-27): own cross-promotion (Math Fighter, ABC
   Smash, EduGameGalaxy); a legal/child-safety check is required before any third-party ad.

### External content (user requirements 2026-09-30, Q-240…Q-247)

7. **Loaded from outside, accepted only if signed by us.** The campaign content is **not part of
   the game build**: `boards/index.json` (manifest), `boards/index.sig` (detached signature)
   and `boards/img/*` are served from our own origin (`<origin>/boards/`, same origin: CSP
   `connect-src 'self'` / `img-src 'self'` stay unchanged) and can be swapped without an app
   update (`boards/**` cache: `max-age=300, must-revalidate`, PLAT-010). The manifest is
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
   - tagline per language (de, en): plain text, 1…90 characters, no `< > & " \` or control
     characters; shown only with `textContent`, never as HTML (ADS-016);
   - per image: path `img/<name>` (no traversal, no scheme), type by **magic bytes** PNG / WebP /
     JPEG equal to the declared type, exact declared byte size ≤ 512 KB, dimensions from the
     header = declared (64…2048 px), **SHA-256 equal to the signed value**, browser decode
     gives the same dimensions (ADS-012/013/014).
   Loading starts after the first frame (never blocks play), same origin only, `credentials:
   omit`, no referrer, timeout **10 s for the manifest and 15 s for the images** (a phone on mobile
   data needs more than 4 s for the TLS handshake plus up to 3 × 512 KB; user reports 2026-10-03
   and 2026-10-07, ADS-029/039); a failed or **incomplete** load (not every active campaign arrived)
   is retried after 20 s, at once when the phone is `online` again or the tab becomes visible
   again; at most **4 loads** in total (no loop); offline → placeholders. The
   verified images live in memory only (Q-243).
9. **Test key hook.** `?adkey=<base64 public key>` replaces the compiled keys **only** in the dev
   server and in the e2e test build (`VITE_AD_TEST=1`, `dist-adtest`, Q-245); the release build
   contains no such code path (PLAT-012, ADS-019). Test fixtures and the TEST-ONLY key live in
   `web/tests/fixtures/ads/` (served as `boards/` by the tests).
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
    answer or ✖ ends the gate; (2) hold the ✋ button **2 s** (user request 2026-10-04, was 3 s) (progress ring; releasing resets;
    a small finger movement does not release; no context menu / scrolling on the button). When
    the hold is complete a big button (≥ 64 px, icon + "mathfighter.rcms.ch") appears; **the
    child / parent TAPS it** and only that tap opens the link: an `<a href=<canonical URL>
    target=_blank rel="noopener noreferrer">` built by the game, whose click handler calls
    `window.open(url, '_blank', 'noopener,noreferrer')` **once** inside the user gesture (phone
    browsers — iOS Safari, Samsung Internet — block pop-ups started from a timer, which is what
    the end of a 2 s hold is). No analytics, no query parameters, no request to the campaign
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
    2 s hold, release opens the link directly, ADS-030) — the link is never opened without it. With
    **no verified campaign** the panel is not offered and the normal all-done bubble shows. It
    uses the images already in memory: no extra request, no tracking. While it is open the board
    reading panels stay closed. Small screens: a compact card left of the control column
    (landscape) / below the compass (portrait), never covering `#settings-btn`, `#compass-btn`,
    `#view-btn`, `#act`.

13. **Phone robustness and field diagnostics** (user report 2026-10-07: "does not work on Android, on
    the web it does"; no real phone available, so the flow is emulated and the phone can report itself).
    - **Release of the hold.** The link opens at the finger release (`pointerup`, ADS-030). If the
      touch is **cancelled** after the full hold (`pointercancel`, an Android gesture), nothing is
      opened (a cancel is no gesture); the hold button turns into a big real link (`<a id="zb-open"
      href=canonical target=_blank rel="noopener noreferrer">`, ≥ 64 px) that the child taps (a tap on a
      link is never blocked). Losing the pointer capture while the finger is down never resets the
      hold. A mouse leaving the button during the ✔ counts as a cancel, not as a release.
    - **Blocked pop-up.** After `window.open` the page must have been hidden by the new tab within
      1.5 s; if not (blocked, or the browser did nothing), a card `#zb-fallback` ("Der Link hat sich
      nicht geöffnet. Tippe hier:" / "The link did not open. Tap here:") with the same real link
      appears for 15 s (✖ closes it).
    - **Back button.** While the gate or the carousel is open one history entry is pushed; Android
      Back / swipe-back closes the top layer instead of leaving the game, and closing by ✖ removes
      the entry again. (The auto-opening board panel does not push an entry.)
    - **Hold text.** `ad-gate-hold` no longer names a number of seconds ("Halte den Knopf gedrückt,
      bis ✔ erscheint. Dann lass los.").
    - **Field diagnostics (ADS-038).** `?adsdebug=1` or the neutral alias `?boarddebug=1` (release builds too; no network, no storage, no
      tracking) opens a full-screen dismissible text panel (≥ 15 px monospace, selectable, buttons
      *copy* and *close* ≥ 48 px high; a round `ZB` chip reopens it) with the live state: user agent,
      secure context, `crypto.subtle`, iframe, online, DPR, viewport, touch points, WebGL version,
      `MAX_TEXTURE_SIZE`, renderer; manifest fetch state/ms/bytes/version; signature result, the
      verifier (`subtle` or pure `js`) and its ms; per image load state / bytes / ms / decode result;
      the board texture upload result per board; the nearest board, its distance and whether one is
      in reach; panel / gate / carousel state; the pointer counters of the hold (down, up, cancel,
      lost capture, context menu, leave, the last event); the `window.open` call (result, whether the
      page was hidden afterwards); WebGL context losses; the last 20 errors and events.
      `window.__zoo.adsDebug()` returns the same data as an object. The recording itself is always on
      (a few counters, nothing per frame); only the overlay needs the parameter.

14. **Blocker-neutral naming and block detection** (user report 2026-10-08: "on the Android browser we still
    do not see the billboards open the dialog with the link"; emulation passes, so the cause is on the
    device). Browser **ad blockers filter by name**: network rules (EasyList/EasyPrivacy) match paths such
    as `/ads/`, `ad-`, `banner`, `sponsor`, `promo`; cosmetic rules **hide elements by id/class**
    (EasyList hides `#ad-panel`, `#ad-carousel`, `.ad-body`, `.ad-choices` … with `display:none !important`
    on every site). Proven with the Ghostery/EasyList engine against the live build 2026-10-08: the
    board panel and the link inside had size 0x0 (see the fix log). Therefore:
    - **Neutral names everywhere the browser sees them.** URL path `boards/` (`boards/index.json`,
      `boards/index.sig`, `boards/img/c1-a.webp`, `c1-b`, `c2-de`, `c2-en`, `c3-de`, `c3-en`); DOM ids,
      classes and data attributes use the prefix `zb-` (`#zb-panel`, `#zb-link`, `#zb-gate`, `#zb-hold`,
      `#zb-act`, `#zb-fallback`, `#zb-carousel`, `.zb-body`, `.zb-x`, `.zb-choice`, `.zb-car-*`,
      `#zb-dbg`, `data-card`); no id, class, data attribute, CSS selector, request path or file name of
      the ad UI contains `ad`/`ads` as a word, `advert`, `banner`, `sponsor` or `promo` (ADS-043).
      Internal code names (`AdsHost`, `ad_near`, Fluent keys `ad-*`) are not visible to the browser and stay.
      The signature scheme (`buchstabenzoo-ads/1`) is unchanged.
    - **Block detection (ADS-044).** A `fetch` of the manifest or an image that is rejected with a
      `TypeError` (`Failed to fetch`, `net::ERR_BLOCKED_BY_CLIENT`; not our own timeout, not an HTTP
      status) is recorded as *blocked by browser/ad blocker (suspected)* with the URL; after a view
      (panel, gate, carousel, link card, 🔗 button) is shown it is checked once (two frames later): computed
      `display:none`, `visibility:hidden` or a size under 8 px counts as hidden by the browser and is
      recorded with the element id. `?adsdebug=1` / `?boarddebug=1` and `adsDebug()` show `verdict`,
      `blocked[]`, `views` and `rescue`.
    - **Fallback (ADS-045).** A hidden view is first forced back (`display:flex !important` inline beats a
      stylesheet rule). If it is still hidden, a native `<dialog>` with a random id, no class names and
      inline styles only (`ads-rescue.ts`) takes over: the panel case shows picture, tagline and the link
      button; the gate case shows only the gate. **The parental gate stays**: question, then the
      press-and-hold, the link opens at the release; a cancelled touch gives a real `<a>`. ✖ / Esc closes it.
      A request that is blocked cannot be rescued (no content): the boards then keep their placeholders and
      the debug panel names the URL.
15. **Samsung Internet and older Chromium** (user report 2026-10-08: works in Android Chrome, not in Samsung
    Internet). Samsung Internet lags several Chromium versions behind, has its own ad-blocker add-ons,
    *smart anti-tracking* and a strict pop-up blocker. Beside rule 14: (a) the signature check tries Web
    Crypto first and, if it throws or answers *false*, the pure-JS Ed25519/SHA-512 once more (same for the
    image SHA-256: `subtle.digest` failing falls back to pure JS); every attempt and exception message is in
    `signature.tries` of the diagnostics (ADS-046); (b) the bundle is built for `chrome87` / `es2020`
    (Samsung Internet 15+); (c) the diagnostics list a `features` table (BigInt, `subtle.digest`,
    `createImageBitmap`, `<dialog>`, CSS `inset`, `dvh`, `:has`, conic-gradient …) so a missing feature
    shows; (d) the ad flow makes **no request to a third-party host** (only `boards/…` of the own origin; the
    link host is contacted only by the browser after the gate).
16. **Platform-dependent links** (user request 2026-10-08). A campaign can carry a link per platform:
    `links: { ios, android }` next to the old `link` (= web / default; a manifest without `links` stays valid).
    - **Detection** (`web/src/platform.ts`, `detectPlatform(ua, platform, maxTouchPoints, uaDataPlatform)`):
      `navigator.userAgentData.platform` first when it names a known platform (Android -> `android`; iOS,
      macOS -> `ios`; Windows, Linux, Chrome OS -> `web`), else the user agent: `Android` -> `android`
      (Chrome, Samsung Internet, Firefox); `iPhone|iPad|iPod`, `Macintosh|Mac OS X` (iPadOS 13+ in desktop
      mode, macOS Safari / Chrome) or `navigator.platform = MacIntel` -> `ios`; an ARM `Linux` platform
      with a touch screen (Android "desktop site") -> `android`; anything else, empty or unknown -> `web`.
      Overrides, strongest first: `setPlatformOverride()` (native wrappers), `?platform=ios|android|web`
      (harmless: only chooses which public store page a link opens), `window.__ZOO_PLATFORM__` or
      `VITE_NATIVE` = `ios|android` (a native build flag), then the detection.
    - **Choice.** At the moment the panel / carousel is built the platform link is chosen (`linkFor`): the
      store link of the platform, else the web link. The button shows the chosen link's **host**
      (`apps.apple.com`, `play.google.com`, the web host); the parental gate and the release-to-open flow are
      unchanged and open exactly that URL.
    - **Validation** when parsing the signed manifest (`checkStoreLink`, mirrored in `tools/ads/sign.py`):
      https, no user info / port / fragment; `ios`: host `apps.apple.com` or `itunes.apple.com`, path
      `/app/id<digits>` or `/<cc>/app/<slug>/id<digits>`, no query; `android`: host `play.google.com`, path
      `/store/apps/details`, query exactly `id=<package name>`. The canonical URL is rebuilt. An invalid
      store link is dropped (that platform then uses the web link); the web `link` keeps its strict rule.
      The signature scheme and `format` are unchanged (additive field).
    - **Content** (verified in the sibling repos 2026-10-08): Math Fighter iOS
      `https://apps.apple.com/app/id6760628828` (from `mathfighter-ios/videos/.../youtube.md`, App Store
      `us/app/math-fighter/id6760628828`), Android `https://play.google.com/store/apps/details?id=com.mathfighter.app`
      (`mathfighter-main/public/about.html`, `android-twa/app/build.gradle`); ABC Smash iOS
      `https://apps.apple.com/app/id6790508038` (`abcshooter/marketing/tiktok-de.md`), Android
      `https://play.google.com/store/apps/details?id=app.abcshooter.twa` (`abcshooter/android-twa/README.md`);
      EduGameGalaxy: web only. The Play pages of both packages were fetched (HTTP 200, "Math Fighter - Learn Math" / "ABC Smash: Learn to Read"), as were the App Store pages. **Native app build:** its own manifest `boards-native/` (campaign ids `mathfighter-ios`, `abcsmash-ios`, `mathfighter-ios-b`, link = the fixed App Store page, nothing else, PLAT-038). (The id `6760028628` named in the request appears in no file; the repo has
      `6760628828`, which is used: see the open question.)
17. **No ad flow while seated** (camera agent report 2026-10-08; GAME-CART CART-008 "no panels while driving"):
    while she sits in a golf cart (`App.driving()`), the board panel does not open, the 🔗 chip is hidden,
    the carousel does not open and `interact()` is not taken; boarding closes an open panel, gate, rescue
    dialog and carousel. Getting out in front of a board opens the panel again.

### Threat model (what the signature does and does not protect)

| Attacker | Can | Cannot |
|---|---|---|
| Network (MITM, bad Wi-Fi, CDN / proxy cache poisoning) | delay or block the ads (placeholders) | show any content: no valid signature, images must match the signed hashes; replaying an old signed manifest works only until `valid_until` and never below the last seen `version` |
| Someone who can write only `ads/**` on the hosting (leaked upload token, bad content deploy) | delete the content | publish content without the private key; swap images (hash), change links/taglines (signed), add a campaign, point a link elsewhere (compiled allowlist) |
| Someone who controls the **whole hosting origin** (incl. the game bundle) | replace the game, its compiled key and its code | — the signature cannot help here (same as for any web game); the store apps (Capacitor) carry the key inside the signed binary and run the packaged bundle |
| Holder of the private key | show any content of a **known campaign id** (images, taglines, `active`) | add a new campaign id or another link host without a game release (Q-241) |
| A child | tap / hold the gate by accident (needs the right plus/minus task **and** 2 s holding) | — (a grade-1/2 child can solve the task, so the gate only stops accidental taps, not a determined child; Q-242) |

### Android native ads (PLAT-054..057, Q-420)

The Android Capacitor app (Google Play, Families policy) bundles its own signed manifest
`boards-native-android/` (template `tools/ads/campaigns-native-android.template.json`, version 1, valid 365 days, images
of slots 1-2 shared with the iOS manifest, which carry no store wording). Campaign ids `mathfighter-android` (slot 1),
`abcsmash-android` (slot 2), `credit-android` (slot 3, user decision 2026-10-08: a CREDIT poster, de/en images `credit-de/en.webp` rendered with PIL, taglines "Buchstabenzoo basiert auf den Ideen von Elena Rieder, entwickelt von Thomas Rieder" / "Letter Zoo is based on the ideas of Elena Rieder, engineered by Thomas Rieder"; the tagline limit is now 90 characters) with NO link: `KNOWN_CAMPAIGNS` `noLink`, the manifest entry has no `link` / `links` key (so the CI check over `"link"` lines sees only Play links), the board shows no link chip, no parental gate, no tap action, no rescue dialog. For the other two the link is the fixed Google Play page
(`play.google.com/store/apps/details?id=com.mathfighter.app` resp. `app.abcshooter.twa`): `KNOWN_CAMPAIGNS` fixes host, path and
the exact query per id (`query` field), everything else is dropped. No itch.io, website, App Store or other-store link or wording; the iOS bundle stays
iOS-only. The build picks the dir with `VITE_NATIVE_PLATFORM=ios|android` (default ios), scripts `build:native` / `build:native:android`.
Links open behind the parental gate like on iOS. Tests: PLAT-054..057 in `web/src/native.test.ts` (listed in `platforms-and-testing.md`).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| ADS-001 | Given a day level, then it has 4 `post` boards (night level: exactly 3 boards, rule 1) with unique ids, none on enclosures, info boards (≥ 4 m), hiding places, scenery, gardens, within 4 m of a food box, 5 m of a gate/door, or on a path/solid cell (LAYOUT-L*-006, -032…039 still hold). The whole game (5 levels) has 18 boards. | unit |
| ADS-002 | Given the whole game and any seed, then **in every level** (`level_1`, `level_2`, `level_3`, `night_1`, `night_2`) each of the 3 slots is on ≥ 1 board (night levels: exactly one each), all boards have a slot, and every slot is on ≥ 2 boards over the zoo. | unit |
| ADS-035 | Given a `poster` board (night levels), then it faces `-z`, its whole width has a solid cell behind it and open walkable ground (grass or path, no tree canopy) in the 2 × 3 m area in front, it is ≥ 4 m from info boards and food boxes, ≥ 4 m from gates/doors, not inside a hiding place / scenery / garden rect, not inside the night house or the terrarium house hall; its geometry adds no collider; the scene has a board lamp for it at night; `near_board` finds it from the front only. | unit |
| ADS-037 | Given a `flyer` board, then the scene adds no collider, its picture decal faces up (normal +y), the cells under it are open grass, and `near_board` finds it from every side within 2.5 m (not from 3 m); a night level has at most one flyer. | unit |
| ADS-036 | Given `night_1` or `night_2` in the browser at night, then the level has 3 boards with the slots 1, 2, 3, each shows its picture (placeholder text or verified campaign), standing in front of it opens the panel / gate behaviour exactly like a day board (✔ guard, 🔗 button), and the screenshot from the game camera shows the lit, readable poster. | e2e |
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
| ADS-016 | Given a tagline with markup characters, control characters or over 90 characters, then the campaign is dropped; shown texts are set as text only. | unit |
| ADS-017 | Given no key / a missing file / a hanging server / offline, then placeholders remain within the timeout (10 s manifest, 15 s images); only same-origin `ads/` URLs are requested. | unit |
| ADS-024 | Given a browser without Web Crypto (plain http on a LAN IP, e.g. the phone on the dev server http://192.168.x.x:5173), then the signature check and the SHA-256 image hashes use the pure-JS fallback (@noble/hashes) and give exactly the same result (a tampered manifest is still rejected), so the signed ads show there too. | unit |
| ADS-025 | Given the gate of the reading campaign (ABC Smash), then it asks a language question instead of a sum: German a noun and its right article (e.g. "… Gabel" → der / die / das, answer die), English the right plural (one mouse → mice) with 4 answers; the maths campaign keeps the plus/minus task up to 20. | unit, e2e |
| ADS-018 | Given the parental gate, then (after the right answer and 2 s holding the open button appears; `window.open` is not called before it is tapped) holding without the right answer never opens, a wrong answer ends it, the right answer + 3 s holding opens it, releasing early resets; the task is a plus or minus task with numbers and result in 0…20 and 4 distinct answers; the gate shows no "adults only" claim (title "Zur Webseite" / "To the website"). | unit |
| ADS-019 | Given `?adkey=` in a build without the test hook, then it is ignored. | unit |
| ADS-020 | Given a correctly signed test manifest (test build), then the boards show the campaign pictures, the panel shows picture + tagline + link button ≥ 64 px, and the link opens (`noopener`, canonical URL) exactly once, only after the gate and the tap on the open button. | e2e |
| ADS-021 | Given a manifest signed with a wrong key (test build) or a tampered image, then the placeholders remain / only that campaign is missing, and no panel opens. | e2e |
| ADS-022 | Given the test build without `?adkey=`, then the test manifest is not trusted and not even requested. | e2e |
| ADS-023 | Given the RELEASE build (production key compiled in, no `?adkey=`) and the real signed `boards/index.json`, then campaigns 1, 2 and 3 are accepted and loaded (needs the owner's public key in `ad-keys.ts` and a signed manifest; `web/tests/e2e/ads_prod.spec.ts`). | e2e |
| ADS-026 | Given an emulated phone (touch, 780×360, 360×780, 412×892), when the player stands in front of a verified board (any facing, 1…3.4 m), then the panel and the link button (≥ 56 px) are inside the viewport and tapping the button starts the gate. | e2e |
| ADS-027 | Given a phone, a verified board is near and the panel was closed by ✖ (or the ✖ was tapped < 400 ms after opening: ignored), then the interact button (🔗) is visible and tapping it reopens the panel; leaving and coming back also reopens it; the manifest arriving late opens the panel for a child already standing there. | e2e |
| ADS-028 | Given a small screen (780×360), then the open panel does not overlap `#settings-btn`, `#compass-btn`, `#view-btn` and `#act` (bounding boxes) and is a compact card; at 360×780 it does not overlap them either. | e2e |
| ADS-029 | Given the gate: after the right answer + 2 s touch hold (finger moving 10 px does not release; no context menu) an open button ≥ 64 px with an anchor (`href` canonical, `target=_blank`, `rel=noopener noreferrer`) appears and `window.open` has NOT been called; tapping it calls `window.open` exactly once and closes panel and gate. A slow image download (> 4 s, < 15 s) still shows the campaign; a failed load is retried once. | e2e + unit |
| ADS-030 | Given the gate (user request 2026-10-04: "the link should open directly, not show the link again"): after the right answer + 2 s touch hold the hold button shows ✔ (`#ad-hold.ready`) and `window.open` has NOT been called (a timer is no user gesture); releasing the finger then opens the link directly, exactly once (`_blank`, `noopener,noreferrer`) and closes panel and gate; an early release opens nothing; there is no second "open" button. Replaces the tap-button part of ADS-029. | e2e |
| ADS-031 | Carousel logic (`carouselItems`, `Carousel`): items = verified campaigns in slot order, one image each for the language, empty without content; `next`/`prev` wrap, `go` wraps, auto-advance after exactly 4 s, a manual move restarts the timer, a paused timer never advances, a single slide never advances. | unit |
| ADS-032 | Given all levels solved (`next = all_done`) and a verified campaign set, when the compass is tapped, then the carousel opens with the headline (de / en) and the first campaign, ◀ ▶ are ≥ 64 px, there is one dot per campaign; ▶ / ◀ / a dot / the 4 s timer change the slide and wrap; ✖ and Esc close it. | e2e |
| ADS-033 | Given the carousel, when the picture is tapped, then the parental gate starts and `window.open` is not called; after the right answer + 2 s hold + release the link of the shown campaign opens exactly once and the carousel closes; the timer does not advance while the gate is up. | e2e |
| ADS-034 | Given `next = all_done` but no verified campaign (no key / failed load) or `next` is not `all_done`, when the compass is tapped, then no carousel opens (the normal bubble shows). On 780x360 and 360x780 the carousel is inside the viewport, its buttons are ≥ 64 px and it overlaps none of `#settings-btn`, `#compass-btn`, `#view-btn`, `#act`. | e2e |

| ADS-038 | Given `?adsdebug=1`, then the overlay `#zb-dbg` is visible with the listed fields (UA, secure context, subtle, MAX_TEXTURE_SIZE, manifest, signature + verifier, images, near, pointer, windowOpen, errors), its text is ≥ 14 px, *copy* is ≥ 48 px high, *close* hides it and the `ZB` chip shows it again; `window.__zoo.adsDebug()` has the same fields; without the parameter there is no overlay and no chip; the recording makes no request. Unit: the recorder keeps the last 20 errors and the last 30 events. | e2e + unit |
| ADS-039 | Given a failed first load (offline, 4 s manifest timeout was too short for slow networks), then the manifest timeout is 10 s; a failed or incomplete load (an active campaign missing) is retried after 20 s and at once on the `online` event / when the tab becomes visible; at most 4 loads; a complete load is never repeated. | unit + e2e |
| ADS-040 | Given the full hold, when the touch is cancelled (`touchCancel`), then `window.open` has NOT been called and `#zb-gate a#zb-open` (canonical href, `_blank`, `noopener noreferrer`) is visible; when `window.open` was called but the page did not become hidden within 1.5 s, `#zb-fallback a#zb-open` shows and ✖ closes it; a lost pointer capture while the finger is down does not reset the hold. | e2e |
| ADS-041 | Given the gate (or the carousel) is open, when the browser Back button is pressed, then the gate (carousel) closes and the page URL and the game stay; closing by ✖ does not leave an extra history entry behind. | e2e |
| ADS-042 | Given Android Chrome emulation (Pixel 7, Galaxy S9+ and a low-end 640x360 DPR 3 profile; touch, mobile UA, CPU 4-6x, slow-network model for `boards/**`, CDP touch hold), then on every profile all three campaigns load and verify, near a board of each slot the panel shows the campaign picture and the board texture is uploaded, the gate flow with a real 2 s touch hold opens the link once on release, rotation keeps the panel in the viewport, a frozen + resumed tab still works, and the game inside a cross-origin iframe (itch.io style) loads and verifies the ads. Report table per step: `web/test-results/ads-android-<profile>.json`. | e2e |
| ADS-043 | Given the sources and the built `web/dist`, then no served path (`boards/…`), id, class, data attribute, CSS selector or request URL of the ad UI, the debug overlay and the rescue dialog contains `ad`/`ads` as a word, `advert`, `banner`, `sponsor` or `promo`; `dist` has no `ads/`, `#ad-`, `.ad-`, `ads-debug`, `campaigns.json` string (Fluent keys and `privacy.html` text excepted); `?boarddebug=1` shows the overlay like `?adsdebug=1`; `firebase.json` caches `boards/**`. | unit (vitest on sources + dist) + e2e (DOM names of the open panel / gate / overlay and all requests) |
| ADS-044 | Given a manifest or image `fetch` that rejects with a `TypeError` (blocked by the client), then `blocked[]` names the URL and the verdict is *blocked by browser/ad blocker (suspected)*; a timeout abort or an HTTP 404 is not reported as a block; given a stylesheet that hides `#zb-panel` with `!important`, then the panel is forced visible, the block is recorded with the element id and shown in the overlay. | unit + e2e |
| ADS-045 | Given a "blocker" that keeps `#zb-panel` hidden, then a `<dialog open>` with a random id shows the picture and the link button; the link starts the parental gate, a wrong answer closes it, the right answer + full hold + release opens the link once (not before the release); given the same for `#zb-gate`, the gate runs inside the dialog and the link opens only after it. | e2e |
| ADS-046 | Given `crypto.subtle` whose `digest` rejects (old / odd Chromium), then the manifest signature still verifies via the pure-JS retry, the image hash check still works, and `signature.tries` lists `subtle: threw …` then `js: ok`; a tampered body gives `subtle: false`, `js: false`; given a Samsung Internet user agent on an old Chromium (≈ 100), the release bundle loads, verifies the three campaigns and opens the panel / gate / link (manual / emulated, see the fix log). | unit + e2e (emulated) |
| ADS-047 | Given real user agent strings, then `detectPlatform` gives `ios` for iPhone Safari/Chrome, iPad desktop mode, Mac Safari/Chrome; `android` for Android Chrome, Samsung Internet, Firefox Android; `web` for Windows, Linux, ChromeOS, unknown and empty; `userAgentData.platform` wins when present; `?platform=` and `setPlatformOverride` beat the detection, invalid values are ignored. | unit |
| ADS-048 | Given store links, then the App Store / Play Store product URLs are accepted and rebuilt canonically; http, wrong or look-alike hosts, `javascript:`, extra query, fragment, user info, port and wrong paths are rejected; an invalid store link is dropped (the platform falls back to the web link), a manifest without `links` is valid, a campaign without store links opens the web link on every platform. | unit |
| ADS-049 | Given emulated iPhone, iPad desktop mode, Mac Chrome, Pixel, Samsung Internet, Windows, Linux and unknown user agents, then the button shows `apps.apple.com` / `play.google.com` / the web host and the gate opens exactly the matching URL once (stubbed `window.open`); EduGameGalaxy opens its web link on iOS and Android; `?platform=android` overrides; the debug panel lists `platform` and `linkHosts`. | e2e |
| ADS-050 | Given the panel and the gate are open, when `driving()` becomes true, then both close, the 🔗 chip stays hidden, `interact()` and `openCarousel()` return false and nothing is opened; when she gets out in front of the board the panel opens again (cross-reference CART-008). | e2e |

## Open questions

- Q-217 answered 2026-09-30: readable own boards with a link behind the parental gate. CLAUDE.md
  ("no ads / no external links") to be updated by the user.
- Q-240 signature scheme and key custody, Q-241 compiled campaign ids / link hosts, Q-242 gate
  difficulty, Q-243 caching / offline / Capacitor origin, Q-244 board placement and review,
  Q-245 test-key hook, Q-246 taglines from the manifest (not Fluent), Q-247 panel behaviour.
- Q-377 night-level boards (posters, 3 per night level, per-level slot deal): implemented as recommended; open for the user.
- Q-364 all-done carousel (rule 12): implemented as recommended; open for the user.
- Q-128 answered 2026-09-27: passive boards only (no links, no tracking, no network), own
  cross-promotion first, legal/child-safety check before any third-party ad (rules 4, 6).
