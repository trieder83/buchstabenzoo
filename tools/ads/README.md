# Ad content: signing and deploying (GAME-ADS "External content")

The game shows the content of the ad boards **only if it is signed by you**. Until you did the
steps below the boards show the local placeholders ("Deine Werbung 1/2/3") and the game makes no
request for ads.

Needs Python 3 with `cryptography` and `Pillow` (`pip install cryptography pillow`).

## One time: create the key pair

```bash
python3 tools/ads/keygen.py --private ~/secrets/zoo-ads.key
```

* The **private key** goes to the file you name (mode 600, never overwritten). Keep it OFFLINE
  (password manager, encrypted stick) and **never** copy it into the repository, a CI secret or
  the hosting. Whoever has it can show content in the game. `*.key` / `*.pem` are git-ignored.
* The **public key** is printed as a TypeScript line. Paste it into `web/src/ad-keys.ts`
  (`AD_PUBLIC_KEYS`), then **release a new game version** (build + deploy). Only games built with
  that key accept your content. For a key rotation keep the old key next to the new one in the list
  until all installed apps are updated, then remove it.

## Every campaign change

1. Put/refresh the source images in `specs/10-gameplay/ads/campaign-*/resources/`, then
   `python3 tools/ads/prepare_images.py` (1024 px wide WebP, ≤ 512 KB, into `ads/img/`).
2. Edit `tools/ads/campaigns.template.json` (taglines de/en ≤ 80 characters, plain text, `active`,
   image list per language; `lang` `*` = every language). Campaign ids, slots and link hosts are
   fixed in the game (`web/src/ads.ts` `KNOWN_CAMPAIGNS`: `mathfighter` slot 1
   `mathfighter.rcms.ch`, `abcsmash` slot 2 `abcsmash.rcms.ch`); a new advertiser needs a game
   release (Q-241) and a legal / child-safety check (GAME-ADS rule 6).
3. Sign (fills sizes, dimensions and SHA-256 of the images, checks the game's limits):

   ```bash
   python3 tools/ads/sign.py --key ~/secrets/zoo-ads.key --version 2 --valid-days 90
   ```

   `--version` must be **higher than the last deployed one** (the game never accepts a lower one;
   a first run uses 1). `--valid-days` is the lifetime: after `valid_until` the game falls back to
   the placeholders, so re-sign before it runs out. This writes `ads/campaigns.json` and
   `ads/campaigns.sig`.
4. Deploy: commit `ads/` (manifest, signature, images) and deploy the site (the `deploy` skill /
   `firebase deploy`). `ads/**` is cached for 5 minutes, so players see a change within minutes —
   no app update is needed (only for a new key, a new campaign id or link host).
5. Check: open the game with your key compiled in, stand in front of a board of each campaign.
   If a board shows the placeholder, something is wrong and the game refused it silently — run
   `npm --prefix web test` and compare the file sizes/hashes (`PLAT-011`).

## What is checked in the game

Signature, format, version (no rollback), validity window, campaign id + slot + link host,
https-only link without query / port / user info, plain-text tagline ≤ 80 characters, every image
by size, magic bytes (PNG/WebP/JPEG), dimensions and SHA-256. Any failure keeps the placeholder.
The threat model is in `specs/10-gameplay/ad-boards.md`.

## Tests and fixtures

`web/tests/fixtures/ads/` holds a **TEST-ONLY** key pair and a manifest signed with it. The
release build never trusts it (`?adkey=` exists only in the dev server and in the e2e test build,
`E2E_DIST=dist-adtest scripts/e2e.sh tests/e2e/ads.spec.ts`).
