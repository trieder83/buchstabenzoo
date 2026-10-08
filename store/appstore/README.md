# App Store listing — Letter Zoo / Buchstabenzoo

Spec: `specs/40-tech/app-store.md` (STORE-*, PLAT-034..041). Texts are one file per field and locale
(`store/appstore/<locale>/*.txt`), limits are tested by `web/src/store.test.ts`.
Only three App Store Connect localisations are published: **English (U.S.)** `en-US`, **English (U.K.)** `en-GB`,
**German** `de-DE` (App Store Connect has one German localisation for DE, AT and CH).
Availability (Pricing and Availability): United States, United Kingdom, Germany, Austria, Switzerland
(add Ireland, Canada, Australia, ... later without a new build). Primary language: **German** (de-DE).

## App-level fields

| Field | Value |
|---|---|
| Name (record) | **Letter Zoo** (en) / **Buchstabenzoo** (de-DE localisation). The record name must be unique store-wide: check at app creation, fallbacks: "Letter Zoo: Read & Rescue", "Buchstabenzoo: Lesespiel" |
| Bundle ID / SKU | `ch.rcms.letterzoo` / `letterzoo-ios-001` |
| Category | Primary **Games → Educational**; secondary Games → Family (or Education) |
| Kids Category | **Yes**, age band **6-8** (recommendation, Q-390) |
| Price | Free, no in-app purchases (no donation link anywhere in the app: Guideline 3.1.1) |
| Support URL | https://letterzoo.web.app (needs a visible contact: add an e-mail to the page, see BUILD_IOS.md) |
| Marketing URL | https://letterzoo.rcms.ch |
| Privacy policy URL | https://letterzoo.web.app/privacy.html |
| Copyright | `2026 Thomas Rieder` |
| Sign-in required | No (no test account) |
| Release | Manual release after approval |

## Privacy nutrition label ("App Privacy")

**Data Not Collected.** The native build has no analytics, no ad network, no account, no network request during play;
the save game lives in the app's local storage. (The posters link to our own App Store pages; that is a link, not data
collection.) Tracking: **No**. If analytics is ever enabled in the native build this answer and the privacy page change.

## Age rating questionnaire (new 2025/26 questionnaire; answer all "None"/"No")

Cartoon or fantasy violence: **None**; realistic violence: None; sexual content, nudity, profanity, horror, medical,
alcohol/tobacco/drugs, gambling, simulated gambling, contests, loot boxes: **None / No**; unrestricted web access: **No**
(the only link-outs are the App Store pages behind the parental gate); user-generated content, messaging, ads for
other apps that is age-inappropriate: No. Expected rating **4+**; the Kids Category band 6-8 is chosen separately.
Caution: the Math Fighter poster shows two martial-arts fighters (stylised, the app itself is rated 4+); if App Review
objects, replace the image or the campaign (Q-392).

## Content rights / AI

"Does your app contain, show or access third-party content?" -> **No** (all content is ours). There is no AI-disclosure
field; Guideline 5.2 applies (we own or generated the material). The review notes state the AI-assisted concept art
honestly. Fonts/sounds: sounds are CC-licensed or synthesised (see `assets/audio` licences, CC-BY credits must appear
in-app if used: check ART-SOUND before submission).

## Screenshots

Required: iPhone **6.9"** (1320x2868 portrait or 2868x1320 landscape; Apple scales 6.9" down for the smaller iPhones, so
6.5" is optional) and, because the app is universal, iPad **13"** (2064x2752 / 2752x2064). 3-10 per set. The five real
renderer screenshots `art/marketing/screens/letterzoo_screen_01..05_*` (3840x2160 landscape, 2160x3840 portrait) cover
all sets: `python3 tools/store/make_appstore_screenshots.py` writes exact sizes (30 files: 6.9", 6.5", 13" x 2
orientations x 5), cover-cropping at most 25 % (iPad 13" portrait is at the limit, 25 %: check that no HUD button is cut)
and never upscaling. Missing: nothing to upscale; **no localised (German) text-free variants needed** because the HUD
language follows the app (screens were taken in German; take English ones for en-US/en-GB if wanted, Q-394).
Output directory `store/appstore/screenshots/` is git-ignored.

## App preview video

Existing: `art/marketing/screens/letterzoo_gameplay_1080x1920.mp4` and `..._1920x1080.mp4` (15.0 s, 30 fps, no audio).
App Store previews must be **15 to 30 seconds** (15.0 s is at the minimum: extend to about 18 s to be safe), 30 fps
at most, H.264 or ProRes, and must match one of the accepted resolutions per device size (the exact list is in
App Store Connect -> "App Preview Specifications"; I could not verify the current list offline, and 1080x1920 is
accepted for some iPhone sizes only). The video must show real app footage (Guideline 2.3.4). Recommendation: skip the
preview for 1.0; if wanted, re-capture at the exact resolution Apple lists for 6.9" and add the game sound.

## Review notes

`store/appstore/review_notes.txt` (paste into "Notes for Review"; no demo account needed).
