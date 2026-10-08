---
id: TECH-PLAY
title: Google Play packaging (Android)
aspect: tech
module: play-store
status: draft
depends_on: [TECH-STORE, TECH-PLATFORMS, GAME-ADS]
test_prefix: PLAY
updated: 2026-10-08
---

# Google Play packaging (Android)

## Goal

Ship Letter Zoo / Buchstabenzoo to Google Play as a free, offline game for children (Designed for Families, Q-440), in the
**existing Play Console account** of Math Fighter and ABC Smash. Same Capacitor wrapper of the web build as iOS
(`VITE_NATIVE=1`, `web/capacitor.config.ts`), package id `ch.rcms.letterzoo`, built on GitHub Actions (`.github/workflows/android.yml`)
into a signed **App Bundle (.aab)**. (ABC Smash / Math Fighter are Bubblewrap TWAs that load the live PWA; Letter Zoo bundles the game instead:
offline start, no web links, no third-party analytics.)

## Rules

1. **Package id** `ch.rcms.letterzoo` (permanent), `minSdk` 24, `targetSdk` = the level Google Play demands for new apps (API 35 or higher; check
   before each release, Q-441). Format: AAB, Play App Signing; we sign with our own **upload key** (`letterzoo-upload.jks`, never in git).
2. **Permissions:** `INTERNET` only (anonymous counters, TECH-PLATFORMS). No `AD_ID`, no location, no storage, no microphone, no camera.
   The merged manifest is checked in CI (PLAY-003). The Play "Advertising ID" declaration: **not used**.
3. **Origin:** `androidScheme: 'https'` (origin `https://localhost`, never changed after release, so saves survive updates).
4. **Display:** portrait + landscape, edge-to-edge (enforced from API 35) with `env(safe-area-inset-*)` (PLAT-021); the hardware Back button
   closes dialogs first, otherwise leaves the app; sound resumes after interruptions (WebView autoplay allowed).
5. **Native posters (Android):** links ONLY to the developer's own Google Play pages (`https://play.google.com/store/apps/details?id=<package>`:
   Math Fighter `com.mathfighter.app`, ABC Smash `app.abcshooter.twa`), opened behind the parental gate. **No** website, itch.io, App Store or
   other web link in the Android bundle (Families policy, no steering outside Play). The manifest and its build switch are specified in
   GAME-ADS ("Android native ads"); the iOS bundle stays iOS-only and the Android bundle Android-only.
6. **No third-party analytics, no ad SDK:** like iOS the native build has no Google Analytics, no `gtag`; the anonymous first-party counters run
   (Data safety: *App activity → App interactions*, collected, not shared, optional = no, purpose Analytics; no data linked to the user, no ids).
7. **Play Console forms** (manual, answers in `store/play/README.md`): Data safety, Content rating (IARC), Target audience (ages 6-8 and 9-12,
   *appeals to children* = yes → Designed for Families), Ads (**yes**, own-app promotions only), App access (all functionality available),
   Government / Financial / Health apps: no, News: no, Advertising ID: no.
8. **Store listing** (`store/play/<locale>/`, locales `de-DE`, `en-US`): title ≤ 30, short description ≤ 80, full description ≤ 4000, no
   emoji, no promotional wording, no names of other platforms/stores; privacy policy URL `https://letterzoo.web.app/privacy.html`.
   Graphics: icon 512×512 PNG, feature graphic 1024×500, 2-8 phone screenshots (the 9:16 renders), optional 7"/10" tablet shots.
9. **Release path:** CI builds the AAB (artifact); the **first upload is manual** in the Play Console (an app cannot be created through the API);
   later uploads may go through the Publisher API with a service account (Q-442, optional).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| PLAY-001 | Given `store/play/<locale>/` for `de-DE` and `en-US`, then every text exists and is within the limits (title 30, short 80, full 4000), the title has no emoji. | unit |
| PLAY-002 | Given the Play texts, then they name no other platform or store (App Store, iOS, Apple, itch.io) and no donation wording. | unit |
| PLAY-003 | Given `web/android/app/src/main/AndroidManifest.xml`, then the only `uses-permission` is `INTERNET`, `package` / `applicationId` is `ch.rcms.letterzoo`, and no `AD_ID`. | unit |
| PLAY-004 | Given `web/capacitor.config.ts`, then `androidScheme` is `https` and `appId` is `ch.rcms.letterzoo`. | unit |
| PLAY-005 | Given `.github/workflows/android.yml`, then it is manual (`workflow_dispatch`), builds `bundleRelease`, reads the keystore only from secrets, and checks that the bundle contains no non-Play link. | unit |
| PLAY-006 | Given `store/play/` and `.gitignore`, then no keystore, `.jks` or `.keystore` file is tracked and the guide names the secrets only by name. | unit |
| PLAY-007 | Given an internal-test install on a phone and a tablet, then the game starts offline, WebGL2 renders, sound plays, progress survives a restart, insets are respected, Back behaves (rule 4), and a poster opens the Play Store only after the gate. | manual |

## Open questions

Q-440 (Designed for Families vs. general audience), Q-441 (target API level at release time), Q-442 (Publisher API upload with a service
account), Q-443 (Play listing languages: de-DE + en-US only).
