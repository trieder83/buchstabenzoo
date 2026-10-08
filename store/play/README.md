# Google Play listing — Letter Zoo / Buchstabenzoo

Spec: `specs/40-tech/play-store.md` (PLAY-*). Texts: `store/play/<de-DE|en-US>/{title,short_description,full_description}.txt`.
Build + signing: `BUILD_ANDROID.md`.

| Field | Value |
|---|---|
| App name | de-DE **Buchstabenzoo: Rette die Tiere**, en-US **Letter Zoo: Read & Rescue** (default language: German) |
| Package | `ch.rcms.letterzoo` |
| App or game / free or paid | **Game**, **Free** (can never be changed to paid later) |
| Category / tags | Game → **Adventure** (second tag Educational); tags: Educational, Adventure, Family |
| Contact e-mail (public) | riedermagic+letterzoo@gmail.com |
| Website | https://letterzoo.rcms.ch |
| Privacy policy | https://letterzoo.web.app/privacy.html |

## Answers for the Play Console forms (App content)

- **Privacy policy:** URL above.
- **Ads:** *Yes, my app contains ads* (own-app posters in the game world; no ad network).
- **App access:** all functionality available without login.
- **Content rating (IARC):** category *Game*; violence, sex, language, controlled substances, gambling, user-generated content, sharing of location, purchases: all **No**; web browsing: No. Expected result: PEGI 3 / Everyone / USK 0.
- **Target audience and content:** age groups **6-8** and **9-12** (not 5 and under: Q-440), "appeals to children": **Yes** → the app joins *Designed for Families*.
- **Data safety:** *Collects or shares user data?* **Yes**, nothing **shared**. Data type **App activity → App interactions**: collected, not shared, purpose **Analytics**, collection **required** (the native app has no switch to turn the counters off). All other types: not collected. Encrypted in transit: **Yes** (HTTPS). Deletion: no accounts, and the counters are anonymous daily totals that cannot be tied to a person (say so in the free text). Families Policy compliance: **Yes**.
- **Advertising ID:** **No**, the app does not use it.
- **Government / Financial features / Health / News / COVID:** none.
- **Data deletion URL:** not needed (no accounts).

## Graphics

| Asset | File | Size |
|---|---|---|
| App icon | `store/icon/AppIcon-1024.png` → export as 512×512 PNG, ≤ 1 MB, full-bleed square | 512×512 |
| Feature graphic | from the cover art (`art/marketing`), no text smaller than readable at 50 % | 1024×500 |
| Phone screenshots | the 9:16 renders (`store/appstore/screenshots/<locale>/APP_IPHONE_67/`) - 2 to 8 | min 320 px, 9:16 |
| 7" / 10" tablet | optional (the iPad renders work) | |
