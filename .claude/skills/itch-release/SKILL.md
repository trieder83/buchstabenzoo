---
name: itch-release
description: Build the HTML5 zip of Letter Zoo from a committed version and publish / update it on itch.io (project edugamegalaxy/letter-zoo, edit id 5113050) — via butler if installed, otherwise via the itch.io edit page in the browser. Use when the user asks to update, upload or publish the game on itch.io.
---

# Update Letter Zoo on itch.io

Project: https://edugamegalaxy.itch.io/letter-zoo (edit page `https://itch.io/game/edit/5113050`,
account `edugamegalaxy`). Pricing: $0 or donation (suggested $2). Embed 960x600, mobile friendly,
fullscreen button. Status: **Draft** until the user says to make it Public (never switch to Public
without asking).

## 1. Deploy first, then build the zip (same commit)
Deploy with the `deploy` skill (commit, push, Firebase) so itch.io and the web version are the same
commit, then:

```bash
scripts/itch-build.sh            # HEAD -> out/itch/letter-zoo-html5-<sha>.zip (index.html at the zip root)
```
The script uses a clean worktree + a release build (needs ≥ 3 GB free disk; Rust target in the temp dir),
prints the zip path and size. `out/` is git-ignored.

## 2a. butler (preferred, once the user installed it and ran `butler login` themselves)
```bash
ITCH_TARGET=edugamegalaxy/letter-zoo:html5 scripts/itch-build.sh --push
```
Never ask for / store the itch API key; `butler login` is done by the user (`! butler login`).

## 2b. Browser upload (no butler)
Claude in Chrome cannot use the native file dialog, so capture the file input:
1. `tabs_context_mcp`, `navigate` to `https://itch.io/game/edit/5113050` (the `*.itch.io` subdomains are blocked for the browser tool; use itch.io/…).
2. `javascript_tool`: hook the click so the input lands in the DOM:
   ```js
   const o = HTMLInputElement.prototype.click;
   HTMLInputElement.prototype.click = function () {
     if (this.type === 'file') { this.id = 'captured_file'; this.style.cssText='position:fixed;left:0;top:0;width:10px;height:10px;opacity:.01;z-index:99999'; document.body.appendChild(this); return; }
     return o.call(this);
   };
   ```
3. `find` the **Upload files** button, click it (ref click), then `find`/`read_page` the file input (a `type=file` button ref) and `file_upload` the zip. **The upload tool limit is 10 MB** — if the zip is larger, use butler or ask the user to drag it in.
4. Wait for the upload bar, tick **"This file will be played in the browser"** on the new file, delete the previous zip entry, click **Save**.
5. Check the page (View page works only on itch.io links; ask the user to test play in a normal browser: sound, touch, fullscreen).

Cover, screenshots, tags and description are edited the same way on the edit page
(`art/marketing/covers/…`, `art/marketing/screens/…`; cover currently `cover_3_reading_square.png`).

## Notes
- The build is static (`base: './'`), runs inside the itch iframe; the signed ads (`ads/`) and the
  analytics (consent-gated, GA id in `web/src/analytics-config.ts`) work there as on Firebase.
- AI disclosure on the page is set to "Yes" (AI-generated art / assistance) — keep it truthful.
- Video: upload to YouTube separately, then paste the URL into "Gameplay video or trailer".
