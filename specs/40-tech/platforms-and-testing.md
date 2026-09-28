---
id: TECH-PLATFORMS
title: Platforms, performance and testing
aspect: tech
module: platforms-and-testing
status: draft
depends_on: [TECH-ARCH]
test_prefix: PLAT
updated: 2026-09-28
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
6. Size: the release build `web/dist` stays ≤ 30 MB (PLAT-001); the deploy script refuses
   to deploy a larger build.
7. Feedback is collected **outside** the game (a form link sent together with the URL); the
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

## Open questions

- Q-011 saving, Q-012 offline, Q-013 min devices, Q-104 draw-call budget for the joined zoo.
