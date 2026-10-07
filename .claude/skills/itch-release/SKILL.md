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

## 2b. Browser upload (no butler) — recipe that worked on 2026-10-07
Claude in Chrome cannot use the native file dialog (clicking an upload button can freeze the tab until the
user closes a native picker — never click an upload button without the hook below). Feed files from a local server instead:
1. Serve the zip with CORS on localhost, e.g. `python3 cors.py` (ThreadingTCPServer, `Access-Control-Allow-Origin: *`,
   `Access-Control-Allow-Private-Network: true`) in a temp dir holding `game.zip`; Chrome may ask the user once to
   allow local-network access ("Allow") — ask them to click it.
2. `tabs_context_mcp` (create a tab), `navigate` to `https://itch.io/game/edit/5113050` (*.itch.io subdomains are blocked for the browser tool).
3. `javascript_tool`: install the hook BEFORE clicking, so the file input is captured instead of opening a dialog:
   ```js
   window.__log = []; const oc = HTMLInputElement.prototype.click;
   HTMLInputElement.prototype.click = function () { if (this.type === 'file') { window.__log.push('click'); this.id = 'cap_file_' + window.__log.length; this.style.cssText = 'position:fixed;left:0;top:0;width:10px;height:10px;opacity:.01;z-index:99999'; if (!this.isConnected) document.body.appendChild(this); return; } return oc.call(this); };
   window.addEventListener('click', e => { const t = e.target; if (t && t.tagName === 'INPUT' && t.type === 'file') e.preventDefault(); }, true);
   ```
4. Trigger **Upload files** with a SCRIPTED click, which reliably hits the hook: `document.querySelector('button[data-max_size]').click()` (real coordinate clicks were hit-or-miss; check `window.__log` / `input[type=file]` appeared). Then set the file from the local server and fire `change`:
   ```js
   const inp = document.getElementById('cap_file_1'); const b = await (await fetch('http://127.0.0.1:4191/game.zip')).blob();
   const dt = new DataTransfer(); dt.items.add(new File([b], 'letter-zoo-html5.zip', {type: 'application/zip'}));
   inp.files = dt.files; inp.dispatchEvent(new Event('change', {bubbles: true}));
   ```
   (The tool's `file_upload` did not work here: the captured input is not in the accessibility tree.) Same for **Add screenshots** / **Replace Cover Image**.
5. After "Success" (uploading a file with the same name replaced the old zip row on 2026-10-07; otherwise delete the previous zip row): tick `input[name$="[embed]"]` ("This file will be played in the browser"), click **Save**; reload the edit page to verify.
6. Verify with `curl -s https://edugamegalaxy.itch.io/letter-zoo` (public page). Test play in a normal browser (the browser tool cannot open the subdomain).

### YouTube trailer (same trick)
Studio → Create → Upload videos: the dialog already has a file input (inside shadow DOM); fetch the mp4 from the local server and set it
as above. Replace the channel's default title/description (it is pre-filled with Math Fighter text), answer "made for kids" truthfully,
Visibility → Public → Publish, then like it. Paste the watch URL into the itch "Gameplay video or trailer" field.

## Notes
- The build is static (`base: './'`), runs inside the itch iframe; the signed ads (`ads/`) and the
  analytics (consent-gated, GA id in `web/src/analytics-config.ts`) work there as on Firebase.
- AI disclosure on the page is set to "Yes" (AI-generated art / assistance) — keep it truthful.
- Video: upload to YouTube separately, then paste the URL into "Gameplay video or trailer".
