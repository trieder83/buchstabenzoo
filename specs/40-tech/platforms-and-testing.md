---
id: TECH-PLATFORMS
title: Platforms, performance and testing
aspect: tech
module: platforms-and-testing
status: draft
depends_on: [TECH-ARCH]
test_prefix: PLAT
updated: 2026-10-04
---

# Platforms, performance and testing

## Platforms

1. Browser: current Chrome, Firefox, Safari (desktop and mobile) with WebGL2.
2. Android and iOS via Capacitor from the same web build (Q-013 minimum versions).
3. Local development on Linux: Vite dev server, reachable from a phone over LAN.

## Performance

- 60 fps target, 30 fps minimum on mid-range phones (Q-013 reference device).
- Initial download ≤ 30 MB (Q-012 offline).

## Testing strategy

| Level | Tool | Scope |
|---|---|---|
| unit | `cargo test` | `zoo-core`, `zoo-assets` logic |
| wasm | `wasm-bindgen-test` | glue code in the browser |
| asset | `cargo test -p zoo-assets` | manifest, concept files, `.glb` checks |
| e2e | Playwright | real browser, WebGL, screenshots |
| manual | checklist in spec | only where automation is impossible |

## Preview deployment for playtests

External testers (families, teachers) play a **preview** build on the web; how-to in
`docs/deploy-preview.md`.

1. Host: **Firebase Hosting preview channel** (`firebase hosting:channel:deploy <channel>
   --expires 30d`), never the live site for playtests. The URL is HTTPS (needed on phones)
   and expires after at most 30 days; redeploying to the same channel keeps the URL.
2. Config: `firebase.json` at the repo root serves the release build `web/dist`
   (`npm --prefix web run build`). No rewrites or redirects: a missing file is a 404, never
   `index.html` (same as `appType: 'mpa'` in Vite).
3. MIME types: `.wasm` → `application/wasm` (needed for streaming compile), `.glb` →
   `model/gltf-binary`.
4. Caching: hashed build files (`bundle/**`, JS + WASM) `max-age=31536000, immutable`;
   `index.html` / `/` `no-cache`; unhashed game files (`assets/**`) short cache
   (`max-age=300, must-revalidate`) so a redeploy reaches testers within minutes.
5. No tracking by default (Q-128 for ads, still true: ads never track): before a parent's opt-in the page
   loads nothing but its own files and makes no request to Google or Firebase. The only exception is the
   opt-in analytics (section "Analytics (opt-in)", user decision 2026-10-04): the CSP allows exactly the
   Google Analytics hosts of PLAT-028 and the script is added only after consent; `default-src 'self'`
   stays.
6. **Ad content** (GAME-ADS "External content"): the signed ad manifest, its signature and images
   are served from the same origin under `ads/` (`web/dist/ads/`, from the repo's `ads/`):
   `ads/**` caches `max-age=300, must-revalidate` (a campaign swap reaches players within
   minutes, without an app update); `ads/*.sig` is `text/plain`. The CSP is unchanged (same origin:
   `connect-src 'self'`, `img-src 'self' data: blob:`). The Capacitor app fetches from the https
   origin of the hosted site (not from its own `capacitor://` / `https://localhost` origin; the
   constant is set when packaging, Q-243). No production manifest ships until the owner signed one
   (`tools/ads/README.md`); the test-key hook exists only in the dev server and the e2e test build
   (`VITE_AD_TEST=1` → `web/dist-adtest`), never in the release build (PLAT-012).
7. Size: the release build `web/dist` stays ≤ 30 MB (PLAT-001); the deploy script refuses
   to deploy a larger build.
8. Feedback is collected **outside** the game (a form link sent together with the URL); the
   game itself gets no feedback button, link or data upload.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| PLAT-001 | Given the release build, then total downloaded bytes on first load ≤ 30 MB. | e2e |
| PLAT-002 | Given a Playwright run on a 1080×2340 mobile viewport, then the zoo entrance renders and the player can walk. | e2e |
| PLAT-003 | Given `firebase.json`, then `hosting.public` is `web/dist` and it has no `rewrites` or `redirects` (unknown paths are 404). | unit |
| PLAT-004 | Given `firebase.json`, then `.wasm` is served as `application/wasm` and `.glb` as `model/gltf-binary`. | unit |
| PLAT-005 | Given `firebase.json`, then `bundle/**` is `immutable` with a one-year max-age, `/` and `**/*.html` are `no-cache`, and `assets/**` has a max-age ≤ 1 hour. | unit |
| PLAT-006 | Given `firebase.json`, `web/index.html` and `web/src`, then `default-src` is `'self'` and no source loads an external URL or a Firebase/Google Analytics SDK, except the opt-in analytics (`analytics.ts`, `analytics-config.ts` only, PLAT-028). | unit |
| PLAT-007 | Given a release build in `web/dist`, then it contains `index.html`, a `.wasm` and `assets/index.json`, and its total size is ≤ 30 MB. | unit (skipped without a build) |
| PLAT-008 | Given `scripts/deploy-preview.sh`, then it deploys with `hosting:channel:deploy … --expires` (≤ 30 days by default) and never runs a live `firebase deploy`. | unit |
| PLAT-009 | Given the preview URL on a phone (Android Chrome, iOS Safari), then the game loads over HTTPS and the player can walk; after a redeploy the same URL shows the new build. | manual |
| PLAT-010 | Given `firebase.json`, then `ads/**` is cached ≤ 10 minutes with `must-revalidate` (not immutable) and the CSP still has `connect-src 'self'`, `img-src 'self' data: blob:`, `form-action 'none'` and no external URL. | unit |
| PLAT-011 | Given the repo's `ads/`, then only manifest, signature and images of ≤ 512 KB are served (no template / key), a shipped manifest has its signature and matches its images' size and SHA-256, and no private key file is tracked except the TEST-ONLY fixture key. | unit |
| PLAT-012 | Given the release configuration, then it has no test-key override: `VITE_AD_TEST` is set by no npm script, `ad-keys.ts` does not contain the test key, the `adkey` parameter is read only behind the build-time switch, and a release bundle in `web/dist` does not contain `adkey`. | unit |
| PLAT-013 | Given `web/public/manifest.webmanifest`, then it has name "Buchstabenzoo", short_name "Zoo", lang `de`, `start_url` and `scope` `./`, `display_override` `["fullscreen","standalone"]`, `orientation` `any`, cream background and brown theme colour. | unit |
| PLAT-014 | Given the manifest, then every icon exists with exactly the declared size, 192, 512 and a `maskable` 512 are present, each PNG ≤ 60 KB, and `apple-touch-icon` (180) and favicon (32) exist. | unit |
| PLAT-015 | Given `firebase.json`, then the manifest is served as `application/manifest+json` with `no-cache`, the CSP is unchanged (`default-src 'self'`, no `manifest-src` widening), and a release build contains the manifest and icons within the 30 MB limit. | unit |
| PLAT-016 | Given `web/index.html`, then it links the manifest and the apple-touch-icon and sets the apple/mobile web-app meta tags, `theme-color` and `viewport-fit=cover`, and no service worker is registered in `web/src`. | unit |
| PLAT-017 | Given the fullscreen helper, then support is detected (standard and `webkit` API), unsupported or standalone / fullscreen display mode hides the button, and the iOS install hint shows only on iPhone/iPad Safari that is not standalone. | unit |
| PLAT-018 | Given the helper, when toggled, then `requestFullscreen({navigationUI:'hide'})` (or the webkit one) is called when not in full screen and `exitFullscreen` when in full screen; rejections and exceptions are swallowed; the orientation is never locked. | unit |
| PLAT-019 | Given the settings menu, then `#fullscreen-toggle` (≥ 72 px, `aria-label` from `ui-fullscreen`) sits next to ❓ and `aria-pressed` follows `fullscreenchange`; de and en keys `ui-fullscreen` and `ui-install-hint-ios` exist. | unit, e2e |
| PLAT-020 | Given Chromium, when the button is pressed, then `document.fullscreenElement` is set, `aria-pressed` is `true`, the settings menu closed and the canvas fills the viewport; pressed again, full screen ends and the game still runs. | e2e |
| PLAT-021 | Given a 412×892 and a 892×412 viewport (also in full screen), then every visible fixed HUD control lies inside the viewport and no two controls overlap; each uses `env(safe-area-inset-*)` (small screens: PLAY-037, also 780×360 and 360×780). | e2e, unit |
| PLAT-022 | Given a fresh start (nothing stored) in a build with a measurement id, then no script is added, `window.dataLayer` does not exist and no request goes to `googletagmanager.com`, `google-analytics.com`, `analytics.google.com` or Firebase; the 📊 button shows "off". | e2e, unit |
| PLAT-023 | Given the 📊 button, when pressed (consent off), then the parental gate (sum, wrong answer ends it; 3 s hold) comes first; only after it the consent card with "Erlauben" / "Nein danke" (≥ 64 px) shows; a wrong answer, ✖ or "Nein danke" grants nothing (no request); no consent dialog or banner blocks the start (only the small 3 s notice of PLAT-033). | e2e, unit |
| PLAT-024 | Given "Erlauben" with a non-empty measurement id, then `zoo.analytics` = `granted`, gtag is set up with `consent default` all denied, then `analytics_storage` granted, `allow_google_signals:false`, `allow_ad_personalization_signals:false`, `ads_data_redaction`, `restricted_data_processing`, `cookie_flags:'SameSite=Lax;Secure'`, `page_location` without query, no `user_id`/user property, and the script `https://www.googletagmanager.com/gtag/js?id=<id>` is requested once. With an empty id nothing is loaded even with consent. | unit, e2e |
| PLAT-025 | Given `ALLOWED_EVENTS` (`zoo_session`, `zoo_play_minutes`, `level_started`, `level_complete`, `mission_complete`, `night_started`, `all_animals_home`, `baby_born`), then an unknown event name is dropped, an unknown param key is removed, and a value of the wrong type or shape (free text, long string, non-integer, out of range) drops the event; nothing is sent without consent. | unit |
| PLAT-026 | Given consent, then `zoo_play_minutes` (`minutes` = active minutes so far, 5, 10 … capped at 120) is sent after each 5 active minutes; time while the tab is hidden does not count; no timer runs without consent or after withdrawal; no per-frame code. | unit |
| PLAT-027 | Given consent, when switched off (📊, no gate needed), then `zoo.analytics` = `denied`, gtag `consent update` denies `analytics_storage`, the `ga-disable-<id>` flag is set, every `_ga*` cookie is removed (host and parent domains), the timer stops and later events are not sent; switching on again needs the gate again. | unit, e2e |
| PLAT-028 | Given `firebase.json`, then the CSP differs from the strict base only by `script-src 'self' 'wasm-unsafe-eval' https://www.googletagmanager.com`, `connect-src 'self' https://*.google-analytics.com https://*.analytics.google.com https://www.googletagmanager.com`, `img-src 'self' data: blob: https://*.google-analytics.com https://*.googletagmanager.com`; `default-src`, `form-action 'none'`, `frame-ancestors 'none'`, `object-src 'none'` stay as before. | unit |
| PLAT-029 | Given the release configuration, then `ANALYTICS_MEASUREMENT_ID` in `analytics-config.ts` is a `G-…` id or `''`; the test id comes only from `VITE_ANALYTICS_TEST_ID` (set by no npm script, honoured only by the build define), and a release bundle in `web/dist` contains no test id; with the empty default the 📊 button is absent and no request is ever made. | unit |
| PLAT-030 | Given host polling of the game events, then `level_started` (first time per session the player stands in a level part, `level_id`), `level_complete` (`level_id`), `mission_complete` (`animal_id`), `night_started`, `all_animals_home`, `baby_born` (`species_id`) are tracked from the existing `poll_events` messages and `App.player_level()`; the parameters are ids only. | unit, e2e |
| PLAT-031 | Given 412×892, 892×412, 1280×800 and 780×360 viewports, then `#analytics-toggle` is ≥ 72 px, shows on/off (`aria-pressed`), and the settings menu stays inside the viewport (scrolls when short). | e2e |
| PLAT-032 | Given the de and en `ui.ftl`, then the `analytics-*` and `ui-analytics` keys exist; `web/public/privacy.html` has a German and an English part, mentions what is collected / not, parental consent, switching off in the settings, retention and a controller placeholder. | unit |
| PLAT-033 | Given analytics is available (measurement id set) and no decision was made, when the game starts (after the intro), then ONE small notice ("Anonyme Statistik ist aus. Eltern können sie im Menü ⚙️ erlauben.") shows at the bottom centre for 3 s, never blocks input (the canvas keeps working), is shown once per session, records nothing, and a tap on it starts the parental gate; given consent was granted or denied, or the id is empty, then no notice shows (user request 2026-10-04). | e2e, unit |

## Analytics (opt-in)

User decision 2026-10-04 (supersedes the earlier "no tracking" statements of `CLAUDE.md` / the vision for
the **game itself**; ads stay same-origin without tracking, GAME-ADS Q-128): Firebase Analytics (Google
Analytics 4) of the hosting project `letterzoo` answers: how many visitors/sessions, how long they play, from
which country and language, how many levels were played / completed.

1. **Off by default (`analytics consent`).** Nothing is loaded, no cookie is set and no request goes to a
   Google domain until a **parent** opts in. The choice is stored in `localStorage` key `zoo.analytics`
   (`granted` | `denied` | unset). No consent dialog at start; only the small non-blocking notice of PLAT-033.
2. **Only through the parental gate.** The 📊 button (`#analytics-toggle`, ≥ 72 px, own row of the settings
   menu, `aria-pressed`) opens the gate of GAME-ADS (`ParentalGate`: plus/minus sum, 3 s hold). Then a card
   shows the privacy note (Fluent `analytics-note`: "Anonyme Statistik hilft uns, das Spiel zu verbessern.
   Es werden keine Namen, keine Texte und keine persönlichen Daten gespeichert.") plus a short detail text
   and the buttons **Erlauben** / **Nein danke** (≥ 64 px). The text is shown in the card, never as a link
   (no external navigation from the game). Switching **off** needs no gate.
3. **Measurement id.** One constant `ANALYTICS_MEASUREMENT_ID` in `web/src/analytics-config.ts` (the GA4
   `G-XXXXXXXXXX` of the web data stream). Empty = analytics disabled entirely (no button, no request, even
   with a stored consent). Tests use `VITE_ANALYTICS_TEST_ID` (build define, test build `dist-adtest` only;
   like the ad test key, PLAT-012/029).
4. **When granted** (`web/src/analytics.ts`, injectable env `loadScript`, `gtag`, `storage`, `now`, cookies,
   timers like `audio.ts`): `consent default` all denied → load `gtag/js?id=<id>` lazily → `consent update`
   `analytics_storage: granted` → `config` with `allow_google_signals:false`,
   `allow_ad_personalization_signals:false`, `cookie_flags:'SameSite=Lax;Secure'`, `page_location` without the
   query string, `page_referrer` empty; `set ads_data_redaction true`, `restricted_data_processing true`. No
   `user_id`, no user properties. GA4 stores no IP address; the country is derived from the IP at Google.
5. **Events** (fixed `ALLOWED_EVENTS`; everything else is dropped): `zoo_session` (once per session:
   `app_language` de|en|fr, `reading_level` kiga|klasse1|klasse2|klasse3), `zoo_play_minutes` (`minutes`),
   `level_started` / `level_complete` (`level_id`), `mission_complete` (`animal_id`), `night_started`,
   `all_animals_home`, `baby_born` (`species_id`). Param values are ids matching `^[a-z][a-z0-9_]{0,31}$` or a
   small integer; never free text, names, coordinates or save contents. Engagement time and browser language
   come from GA4 automatically. Events come from the existing `poll_events` messages (`Ui` forwards them) and
   `App.player_level()` (polled once a second, only while consent is on); zoo-web adds the outbox messages
   `baby_born` and `all_home`.
6. **Withdrawal** (📊 off): store `denied`, `consent update` denied, `window['ga-disable-<id>']=true`, remove
   all `_ga*` cookies (host + parent domains), stop the timer, drop later events.
7. **Hosting.** The CSP is always delivered but the script is only added after consent; PLAT-028 fixes the
   exact allowlist. The standalone `web/public/privacy.html` (de + en) is for the store listing / Firebase.
   GA4 console steps (retention 2 months, Google signals off, custom dimensions) are in the hand-over notes.
8. **Not now:** Capacitor/native builds (Q-358), consent for under-13 with verifiable parental consent beyond
   the gate (Q-356), cookie-less measurement (Q-357).

## Installable and full screen

User request 2026-10-02: the game should fill a phone screen (no browser bars). Two ways, both
without a service worker, popups, network or tracking:

1. **Installable web app (iPhone + Android).** `web/public/manifest.webmanifest` (copied by Vite
   into `dist/`, served as `application/manifest+json`, `Cache-Control: no-cache`) with `name`
   "Buchstabenzoo", `short_name` "Zoo", `lang` "de", `start_url` and `scope` `./` (relative: works under
   any path), `display_override` `["fullscreen","standalone"]`, `display` `fullscreen` (Android
   fallback chain), `orientation` `any` (portrait and landscape are both supported, PLAY-030),
   `background_color` cream `#fff3d6`, `theme_color` brown `#3b2314` and icons 192, 512 and a
   maskable 512 (PNG). `index.html` links the manifest and a 180 px `apple-touch-icon` and a 32 px
   favicon, sets `apple-mobile-web-app-capable`, `mobile-web-app-capable`,
   `apple-mobile-web-app-status-bar-style=black-translucent`, `apple-mobile-web-app-title` and
   `theme-color`. The CSP is unchanged (`default-src 'self'` covers `manifest-src`). The icons are drawn
   deterministically by `tools/make_icons.py` (Pillow; comic look: dark brown outline, yellow, green,
   cream; "ABC" on a zebra-stripe paw motif) and reviewed in `art/index.html` ("App icon").
   Install: iPhone Safari Share ⎙ → "Zum Home-Bildschirm"; Android Chrome menu ⋮ → "App installieren" /
   "Zum Startbildschirm hinzufügen". Safari cannot install by itself, so the **settings menu only**
   shows once a short hint line (Fluent `ui-install-hint-ios`) on iPhone/iPad Safari that is not
   yet running standalone (`navigator.standalone` / `display-mode: standalone`); never a popup.
2. **Full-screen button** (`#fullscreen-toggle`, ≥ 72 px, corner-bracket icon (SVG, outward / inward; the glyph ⛶ is missing in many fonts), `aria-label` Fluent
   `ui-fullscreen`, `aria-pressed`) in the settings menu next to ❓. `web/src/fullscreen.ts` (injectable
   environment like `audio.ts`) uses `requestFullscreen({ navigationUI: 'hide' })` /
   `exitFullscreen()` with the `webkit` prefixed fallbacks; promise rejections are silent. The button is hidden when the
   Fullscreen API is missing (iPhone Safari) or the app already runs standalone / fullscreen
   (`display-mode`). The orientation is **never locked**. `fullscreenchange` updates the button; the settings
   menu closes after the toggle; leaving full screen (Esc / back) changes no game state. The canvas follows
   its `ResizeObserver` (`App.resize`).
3. **Safe areas.** Every fixed HUD control sits inside `env(safe-area-inset-*)` (notches, rounded
   corners, home bar) and none overlaps another (PLAT-021).
4. Not now: service worker / offline cache and an install prompt UI (Q-325).

## Open questions

- Q-355 retention / legal review of the opt-in analytics, Q-356 verifiable parental consent, Q-357 cookie-less alternative, Q-358 native builds. Q-325 service worker / offline + install prompt (later). Q-011 saving, Q-012 offline, Q-013 min devices, Q-104 draw-call budget for the joined zoo.
