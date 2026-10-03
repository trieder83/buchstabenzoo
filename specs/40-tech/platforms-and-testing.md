---
id: TECH-PLATFORMS
title: Platforms, performance and testing
aspect: tech
module: platforms-and-testing
status: draft
depends_on: [TECH-ARCH]
test_prefix: PLAT
updated: 2026-10-02
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
5. No tracking (CLAUDE.md child-safety, Q-128): Firebase Analytics and every other Firebase
   SDK stay off; the page loads nothing but its own files. A `Content-Security-Policy` with
   `default-src 'self'` / `connect-src 'self'` enforces this in the browser.
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
| PLAT-006 | Given `firebase.json`, `web/index.html` and `web/src`, then the CSP limits `default-src`/`connect-src` to `'self'`, and no source loads an external URL or a Firebase/Google Analytics SDK. | unit |
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

- Q-325 service worker / offline + install prompt (later). Q-011 saving, Q-012 offline, Q-013 min devices, Q-104 draw-call budget for the joined zoo.
