---
id: TECH-PLATFORMS
title: Platforms, performance and testing
aspect: tech
module: platforms-and-testing
status: draft
depends_on: [TECH-ARCH]
test_prefix: PLAT
updated: 2026-10-08
---

# Platforms, performance and testing

## Platforms

1. Browser: current Chrome, Firefox, Safari (desktop and mobile) with WebGL2.
2. Android and iOS via Capacitor from the same web build (Q-013 minimum versions); iOS: TECH-STORE (`specs/40-tech/app-store.md`, PLAT-034..043).
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
   are served from the same origin under `boards/` (`web/dist/boards/`, from the repo's `boards/`; a neutral name, ADS-043):
   `boards/**` caches `max-age=300, must-revalidate` (a campaign swap reaches players within
   minutes, without an app update); `boards/*.sig` is `text/plain`. The CSP is unchanged (same origin:
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
| PLAT-006 | Given `firebase.json`, `web/index.html` and `web/src`, then `default-src` is `'self'` and no source loads an external URL or a Firebase/Google Analytics SDK, except the opt-in analytics (`analytics.ts`, `analytics-config.ts` only, PLAT-028) and the anonymous counters (`counter.ts`, `counter-config.ts`: the Firestore REST endpoint only, no SDK, PLAT-048). | unit |
| PLAT-007 | Given a release build in `web/dist`, then it contains `index.html`, a `.wasm` and `assets/index.json`, and its total size is ≤ 30 MB. | unit (skipped without a build) |
| PLAT-008 | Given `scripts/deploy-preview.sh`, then it deploys with `hosting:channel:deploy … --expires` (≤ 30 days by default) and never runs a live `firebase deploy`. | unit |
| PLAT-009 | Given the preview URL on a phone (Android Chrome, iOS Safari), then the game loads over HTTPS and the player can walk; after a redeploy the same URL shows the new build. | manual |
| PLAT-010 | Given `firebase.json`, then `boards/**` is cached ≤ 10 minutes with `must-revalidate` (not immutable) and the CSP still has `connect-src 'self'`, `img-src 'self' data: blob:`, `form-action 'none'` and no external URL. | unit |
| PLAT-011 | Given the repo's `boards/`, then only manifest, signature and images of ≤ 512 KB are served (no template / key), a shipped manifest has its signature and matches its images' size and SHA-256, and no private key file is tracked except the TEST-ONLY fixture key. | unit |
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
| PLAT-022 | Given a fresh start (nothing stored) in a build with a measurement id (user request 2026-10-04: "before consent it should send the anonymous tracking"), then ONLY the anonymous, cookie-less page ping runs: gtag loads once with `consent default` = all storage denied, `ads_data_redaction`, `config` with `client_storage: 'none'`, no Google signals, no ad personalisation, `send_page_view` — no cookies (`_ga*`), nothing in storage, no custom game events, no play-minute timer, no user ids; the 📊 button shows "off"; a stored `denied` sends nothing at all; granting later updates the consent (cookies on, game events on) without a second page view. | e2e, unit |
| PLAT-023 | Given no decision was made, when the 📊 button in the settings is tapped once, then consent is granted at once — no parental question, no 3 s hold, no extra dialog (user request 2026-10-04) — analytics starts, and the 📊 row is hidden from then on (also after a reload); consent can still be withdrawn programmatically / by clearing site data; the only consent prompt at the start is the welcome dialog of PLAT-033. | e2e |
| PLAT-024 | Given "Erlauben" with a non-empty measurement id, then `zoo.analytics` = `granted`, gtag is set up with `consent default` all denied, then `analytics_storage` granted, `allow_google_signals:false`, `allow_ad_personalization_signals:false`, `ads_data_redaction`, `restricted_data_processing`, `cookie_flags:'SameSite=Lax;Secure'`, `page_location` without query, no `user_id`/user property, and the script `https://www.googletagmanager.com/gtag/js?id=<id>` is requested once. With an empty id nothing is loaded even with consent. | unit, e2e |
| PLAT-025 | Given `ALLOWED_EVENTS` (`zoo_session`, `zoo_play_minutes`, `level_started`, `level_complete`, `mission_complete`, `night_started`, `all_animals_home`, `baby_born`), then an unknown event name is dropped, an unknown param key is removed, and a value of the wrong type or shape (free text, long string, non-integer, out of range) drops the event; nothing is sent without consent. | unit |
| PLAT-026 | Given consent, then `zoo_play_minutes` (`minutes` = active minutes so far, 5, 10 … capped at 120) is sent after each 5 active minutes; time while the tab is hidden does not count; no timer runs without consent or after withdrawal; no per-frame code. | unit |
| PLAT-027 | Given consent, when switched off (📊, no gate needed), then `zoo.analytics` = `denied`, gtag `consent update` denies `analytics_storage`, the `ga-disable-<id>` flag is set, every `_ga*` cookie is removed (host and parent domains), the timer stops and later events are not sent; switching on again needs the gate again. | unit, e2e |
| PLAT-028 | Given `firebase.json`, then the CSP differs from the strict base only by `script-src 'self' 'wasm-unsafe-eval' https://www.googletagmanager.com`, `connect-src 'self' https://*.google-analytics.com https://*.analytics.google.com https://www.googletagmanager.com`, `img-src 'self' data: blob: https://*.google-analytics.com https://*.googletagmanager.com`; `default-src`, `form-action 'none'`, `frame-ancestors 'none'`, `object-src 'none'` stay as before. | unit |
| PLAT-029 | Given the release configuration, then `ANALYTICS_MEASUREMENT_ID` in `analytics-config.ts` is a `G-…` id or `''`; the test id comes only from `VITE_ANALYTICS_TEST_ID` (set by no npm script, honoured only by the build define), and a release bundle in `web/dist` contains no test id; with the empty default the 📊 button is absent and no request is ever made. | unit |
| PLAT-030 | Given host polling of the game events, then `level_started` (first time per session the player stands in a level part, `level_id`), `level_complete` (`level_id`), `mission_complete` (`animal_id`), `night_started`, `all_animals_home`, `baby_born` (`species_id`) are tracked from the existing `poll_events` messages and `App.player_level()`; the parameters are ids only. | unit, e2e |
| PLAT-031 | Given 412×892, 892×412, 1280×800 and 780×360 viewports, then `#analytics-toggle` is ≥ 72 px, shows on/off (`aria-pressed`), and the settings menu stays inside the viewport (scrolls when short). | e2e |
| PLAT-032 | Given the de and en `ui.ftl`, then the `analytics-*` and `ui-analytics` keys exist; `web/public/privacy.html` has a German and an English part, mentions what is collected / not, parental consent, switching off in the settings, retention and a controller placeholder. | unit |
| PLAT-033 | Given analytics is available (measurement id set), no decision was made and the intro is enabled (a new player), when the game starts, then BEFORE the intro a welcome dialog shows: the story ("Die Tiere sind aus dem Zoo ausgebrochen – bring sie nach Hause!"), a smaller data note ("Das Spiel sammelt anonyme Daten, nur um das Spiel zu analysieren und zu bewerben."), and two buttons ≥ 64 px: light "Nein, ich will nicht spielen" and bold green "Ja, einverstanden, los geht's!"; Yes grants consent (analytics starts) and the intro follows; No stores nothing, sends nothing and shows a goodbye card whose ↩ button asks again; given consent was granted/denied earlier or the id is empty, no welcome shows; the button texts always stay inside their boxes: on small screens the text wraps and the font scales down (≥ 11 px), the two buttons stack below 340 px width (user request 2026-10-08); the ⚙️ 📊 button (parental gate) still changes the decision later (user request 2026-10-04, replaces the 3 s notice). | e2e |
| PLAT-034 | Given `web/capacitor.config.ts`, then appId is `ch.rcms.letterzoo`, appName `Letter Zoo`, webDir `dist`, the default origin is kept and no `server.url` loads the game from the web (TECH-STORE). | unit |
| PLAT-035 | Given `web/ios` Info.plist and project, then `ITSAppUsesNonExemptEncryption` is false, no `NS*UsageDescription` / background modes, `CFBundleLocalizations` de + en, portrait + landscape, deployment target 15.0, bundle id `ch.rcms.letterzoo`. | unit |
| PLAT-036 | Given `VITE_NATIVE=1` (`npm run build:native`), then the Google Analytics id is `''` (no button, no gtag script, no GA request; the anonymous counters of PLAT-044 still run) and ads are off unless `NATIVE_ADS`; the web build keeps both. | unit |
| PLAT-037 | Given the iOS `AppDelegate`, then the audio session is `.playback` (sound with the ringer on silent) and re-activated when the app becomes active. | unit, manual |
| PLAT-038 | Given `tools/ads/campaigns-native.template.json` (and `boards-native/index.json` once signed), then all three slots are covered and every link is `https://apps.apple.com/app/id<ID>`; the native build emits `boards-native/` instead of `boards/`. | unit |
| PLAT-039 | Given `.github/workflows/ios.yml`, then it starts manually only, runs on macOS, builds with `npm run build:native`, uses the six secret names and contains no key material. | unit |
| PLAT-040 | Given `tools/store/make_appstore_screenshots.py`, then it knows the exact App Store sizes (6.9", 6.5", iPad 13") and never upscales. | unit |
| PLAT-041 | Given the App Store icon, then it is 1024x1024 RGB without alpha and the launch image exists. | unit |
| PLAT-042 | Given `web/public/privacy.html`, then it covers the iOS app (no third-party analytics, only the anonymous counters), the section "Anonyme Zähler / Anonymous counters" in both languages and the alias `https://letterzoo.rcms.ch`. | unit |
| PLAT-044 | Given the counter module, when `count(event, param)` is called, then it queues one write per count and sends (every ~10 s, on page hide) a Firestore REST `documents:commit` to `c/<yyyymmdd UTC>_<event>_<param>_<platform>_<lang>_<version>` with exactly the fields `date, event, param, platform, lang, v` (update mask) and an `updateTransforms` increment of `n` by 1; unknown events and wrong param shapes are dropped; a document appears at most once per commit (repeats go into a later commit); per event at most 30 counts per minute and 200 queued; errors are swallowed (no retry), nothing blocks. | unit |
| PLAT-045 | Given `COUNTER_EVENTS` in `web/src/counter.ts` and `firestore.rules`, then the event lists are identical, and the rules require exactly the keys `date,event,param,platform,lang,v,n`, `param` ~ `[a-z0-9_-]{0,32}`, platform in web/ios/android, lang in de/en, id = the composed name, `n == 1` on create and `n == old + 1` on update, and deny get/list/delete and every other path. | unit |
| PLAT-046 | Given the counters run, then no cookie, `localStorage`, `sessionStorage` or IndexedDB access happens, the source contains no id/random/user-agent code and the request body holds no identifier or finer timestamp than the UTC day. | unit, e2e |
| PLAT-047 | Given Do-Not-Track (`navigator.doNotTrack` = 1), Global Privacy Control, `?nocount=1`, automation (`navigator.webdriver`) or a dev host (localhost, 127.*, 10.*, 172.16-31.*, 192.168.*, *.local), then nothing is counted; `?count=1` re-enables automation and dev hosts (tests, `test_ping`) but never overrides DNT / GPC / `?nocount=1`. | unit |
| PLAT-048 | Given `firebase.json`, then `connect-src` additionally allows `https://firestore.googleapis.com` and no other directive does; `main.ts` starts the counters in every build (also `VITE_NATIVE=1`) before the Google Analytics block and the native Google Analytics id stays `''`. | unit |
| PLAT-049 | Given the game with `?count=1` and a stubbed Firestore endpoint, when the child reads the note, enters a wrong and then the right code, taps the compass and walks, then commit bodies contain `session_start`, `key_note_read`, `lock_wrong`, `lock_ok`, `key_box_opened`, `hint_used` (`compass`) and `level_started`, each with the allowed fields only. | e2e |
| PLAT-050 | Given a stubbed endpoint, then with Do-Not-Track, Global Privacy Control, `?nocount=1`, or automation without `?count=1` no request is made. | e2e |
| PLAT-051 | Given a signed campaign (test build), when the panel opens and the gate is answered wrong, then right, the hold is released early and then after the full hold, then `board_panel_opened`, `board_link_tapped`, `gate_answer_wrong`, `gate_answer_right`, `gate_hold_started`, `gate_hold_cancelled`, `gate_hold_complete`, `board_link_opened` are counted with the param `s1_web`. | e2e |
| PLAT-052 | Given the live project, then `node --experimental-strip-types tools/analytics/rules_check.mjs` shows: a valid increment returns 200, and an extra field, unknown event, `n = 5`, bad platform/lang/param, wrong id, other collection, read, list and delete return 403. | manual (live) |
| PLAT-053 | Given `web/public/privacy.html`, `store/appstore/README.md`, `BUILD_IOS.md` and `store/appstore/review_notes.txt`, then they describe the counters (web: always, also without consent; iOS: Product Interaction / Analytics / not linked / no tracking) and none states "Data Not Collected" as the current iOS label. | unit |

## Anonymous counters

User decision 2026-10-08 (approved proposal): the game counts how often events happen, **with no identifier**, so that
the iOS app can stay in Apple's Kids Category (Guidelines 1.3 / 5.1.4: no third-party analytics; first-party anonymous
counting is fine) and the web build gets the same numbers independent of the Google Analytics consent (PLAT-022..033,
which stay unchanged). Free Firebase Spark plan, no server code: the client writes to Firestore through the REST API.

1. **Data.** One document `c/<yyyymmdd>_<event>_<param>_<platform>_<lang>_<v>` per UTC day, event, param, platform
   (`web` | `ios` | `android`), language (`de` | `en`) and short build id `v` (git hash). Fields: `date, event, param,
   platform, lang, v` and the number `n` (increment by 1 per count). No cookie, no `localStorage`, no user/device/session id,
   no free text, nothing finer than the day; the IP address is not stored by us. (`lang` and `v` are part of the id so that
   the fields of a document never change.)
2. **Client.** `web/src/counter.ts` (`count(event, param)`), project and public web API key in `counter-config.ts`
   (the key only names the project; the rules decide). Queue in memory, flush every ~10 s and on `visibilitychange`
   hidden / `pagehide` with `fetch(..., {keepalive: true})`; the same document at most once per commit; failures are dropped.
   Wired at the existing hooks only: `Ui.onGameEvent` (the `poll_events` stream; same hook as the Google Analytics events),
   `countLevel` (once per level and session, next to `observeLevel`), the lock panel result, the compass tap and the
   interact results of `ui.ts`, and the ad flow of `ads-ui.ts` / `ads-rescue.ts`.
3. **Off switches.** Do-Not-Track, Global Privacy Control, `?nocount=1`; automation (`navigator.webdriver`) and dev hosts unless
   `?count=1` (tests intercept `firestore.googleapis.com`; nothing in the tests reaches production).
4. **Firestore.** Database `(default)`, location `eur3` (EU). `firestore.rules`: create / update of `/c/{id}` only with exactly
   the keys above, an allowlisted event, `n == 1` (create) or `n == old + 1` (update); no read, list or delete by anyone
   (clients cannot read the numbers). Deploy: `firebase deploy --only firestore:rules --project letterzoo`. The numbers are
   read by the operator with `tools/analytics/report.mjs` (README "How to read the numbers") or in the Firebase console.
5. **Allowlist** (the same list is in `firestore.rules`, parity test PLAT-045). `param` is `''` unless stated; `slot param`
   = `s<1|2|3>_<ios|android|web>` (poster slot + the link type actually used on this device).

| Event | Param | Counted when |
|---|---|---|
| `session_start` | – | page / app start |
| `level_started` | level id | first time per session the player stands in a level part |
| `level_completed` | level id | `level_complete` game event |
| `mission_complete` | animal id | an animal is home (rescue mission done) |
| `all_animals_home` | – | every animal of the level is home |
| `baby_born` | animal id | a baby is born |
| `night_started` | – | the night event |
| `sleep_started` | – | the sleep event |
| `hint_used` | `compass` | the compass / hint button was tapped |
| `key_note_read` | – | the golf-cart note panel opened |
| `key_box_opened` | – | the cart key was unlocked |
| `lock_wrong` | – | every wrong code (the count is also the number of tries) |
| `lock_ok` | – | the right code |
| `cart_boarded` | – | she got into a golf cart |
| `cart_locked_tap` | `no_key` \| `closed_level` | a locked cart was tapped |
| `cart_park_refused` | – | parking was refused |
| `board_panel_opened` | slot param | an ad poster panel opened |
| `board_link_tapped` | slot param | the poster link was tapped (parental gate shown) |
| `gate_answer_right` / `gate_answer_wrong` | slot param | the gate question was answered |
| `gate_hold_started` | slot param | the 2 s hold started |
| `gate_hold_complete` | slot param | the hold reached the ✔ |
| `gate_hold_cancelled` | slot param | the hold was released early (or the touch was cancelled) |
| `board_link_opened` | slot param | the link was opened (released after the full hold, or the link button tapped) |
| `board_link_blocked_fallback` | slot param | the browser did not open the link: the fallback link card is shown |
| `board_rescue_dialog` | slot param | a blocker hid the panel / gate: the native fallback dialog is used |
| `board_blocked_detected` | view name | an element of the poster flow was hidden by the browser / an ad blocker |
| `carousel_opened` | – | the all-done poster carousel opened |
| `carousel_link_opened` | slot param | a link was opened from the carousel |

## Analytics (opt-in)

User decision 2026-10-04 (supersedes the earlier "no tracking" statements of `CLAUDE.md` / the vision for
the **game itself**; ads stay same-origin without tracking, GAME-ADS Q-128): Firebase Analytics (Google
Analytics 4) of the hosting project `letterzoo` answers: how many visitors/sessions, how long they play, from
which country and language, how many levels were played / completed.

1. **Off by default (`analytics consent`).** Nothing is loaded, no cookie is set and no request goes to a
   Google domain until a **parent** opts in. The choice is stored in `localStorage` key `zoo.analytics`
   (`granted` | `denied` | unset). Only the welcome dialog of PLAT-033 asks at the first start.
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
