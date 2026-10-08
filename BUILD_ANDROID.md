# Building Letter Zoo for Google Play (Android)

Spec: `specs/40-tech/play-store.md`. Workflow: `.github/workflows/android.yml` (manual start). The AAB is the same game as the web build
(`VITE_NATIVE=1`, `VITE_NATIVE_PLATFORM=android`), wrapped by Capacitor, package `ch.rcms.letterzoo`.

## One-time: the upload key (never commit it, never print it)

Done 2026-10-08: `~/secrets/letterzoo-upload.jks` (alias `letterzoo`, password in `~/secrets/letterzoo-upload.pw`, both mode 600) and the three
GitHub secrets `ANDROID_KEYSTORE_B64`, `ANDROID_KEYSTORE_PASSWORD`, `ANDROID_KEY_ALIAS`. **Back both files up** (password manager / encrypted drive).
To recreate: `keytool -genkeypair -keystore ~/secrets/letterzoo-upload.jks -alias letterzoo -keyalg RSA -keysize 2048 -validity 10000`, then
`base64 -w0 ~/secrets/letterzoo-upload.jks | gh secret set ANDROID_KEYSTORE_B64 -R trieder83/buchstabenzoo` and set the other two secrets.
Google keeps the real app-signing key (Play App Signing); if the upload key is ever lost, Google can reset it.
No Firebase Android registration (`google-services.json`) is needed: the counters are plain HTTPS calls to Firestore.

## Build

GitHub → Actions → **Android build** → *Run workflow*. Result: artifact `LetterZoo-<version>-<run>.aab` (14 days).
Version name = `web/package.json` version, version code = run number.

## First upload (manual, once)

1. Play Console → **Create app**: name *Letter Zoo: Read & Rescue*, default language German (de-DE), **Game**, **Free**, accept the declarations.
2. Left menu → **Dashboard → Set up your app**: fill every task using the answers in `store/play/README.md` (privacy policy, ads, app access,
   content rating, target audience, data safety, advertising ID, government/financial/health, store listing + graphics).
3. **Testing → Internal testing → Create new release**: choose *Play App Signing* (default, "Continue"), upload the `.aab`, release name = version,
   add the release notes (de + en), *Save* → *Review release* → *Start rollout to internal testing*. Add your Google account under *Testers* and open
   the opt-in link on an Android phone: install and play (PLAY-007).
4. If the Console says the account must complete a closed test first (only for accounts created after Nov 2023), run **Closed testing** with the
   required number of testers for the required days; otherwise go on.
5. **Production → Create new release** → *Add from library* (the same bundle) → release notes → *Review release* → *Send for review*
   (choose *managed publishing* to control the launch day). Reviews take from a few hours to about 7 days; the first one for a new app is the slowest.
6. Later releases: run the workflow again, upload the new `.aab` in the track, same steps from "Create new release".
