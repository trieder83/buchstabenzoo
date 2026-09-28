---
name: deploy
description: Deploy Buchstabenzoo (Letter Zoo) to Firebase Hosting (project `letterzoo`, https://letterzoo.web.app) — clean release build of a committed version in a separate worktree, pre-deploy checks, then the Firebase deploy (live or preview channel) and a post-deploy check. Use when the user asks to deploy, publish, release or update the web version, or to make a preview link for testers.
---

# Deploy Buchstabenzoo to Firebase Hosting

Target: Firebase project **`letterzoo`** (display name "Letter Zoo"), live URL
**https://letterzoo.web.app** (also `letterzoo.firebaseapp.com`). Hosting only — no
Analytics, no tracking (CLAUDE.md child safety, Q-128). Config: `firebase.json` at the repo
root (public `web/dist`, MIME types, caching, strict CSP); rules and tests PLAT-003…009 in
`specs/40-tech/platforms-and-testing.md`, checks in `web/src/deploy.test.ts`.

## 1. Decide what to deploy

- Deploy **only committed code** — never a working tree with other agents' unfinished
  changes. Default: `HEAD` of `main`. Check `git status`; if there are uncommitted changes,
  tell the user they are not included (or commit them first if the user wants them).
- **Live** (default when the user says "deploy"/"final") or **preview** (a temporary link for
  testers: `firebase hosting:channel:deploy <channel> --expires 30d`). Ask only if unclear.

## 2. Pre-deploy checks (on the chosen commit)

```bash
~/.cargo/bin/cargo test --workspace
npm --prefix web run lint && npm --prefix web test
```

Stop and report if anything fails. (The full e2e suite is optional here — it takes long.)

## 3. Clean release build in a separate worktree

Use the scratchpad dir so the main tree (and running agents) are untouched:

```bash
D=<scratchpad>/release
git worktree remove --force "$D" 2>/dev/null; rm -rf "$D"
git worktree add --detach "$D" <commit>
cd "$D/web" && npm ci --silent
PATH=$HOME/.cargo/bin:$PATH CARGO_TARGET_DIR="$D/target" npm run build
```

Then verify `"$D/web/dist"`: `index.html`, `bundle/zoo_web_bg-*.wasm`, `assets/index.json`
exist and `du -sh dist` is **< 30 MB** (PLAT-001). Keep ≥ 10 GB disk free (`df -h`).

## 4. Deploy

Firebase deploys are a production action. **Claude runs them only if the user's
permission settings allow `firebase` commands**; otherwise give the user the exact command
to run with the `!` prefix so the output lands in the session:

- Live: `! cd <D> && firebase deploy --only hosting --project letterzoo`
- Preview: `! cd <D> && firebase hosting:channel:deploy playtest --expires 30d --project letterzoo`
  (list: `firebase hosting:channel:list --project letterzoo`, delete:
  `firebase hosting:channel:delete playtest --project letterzoo`)

Never work around a permission denial. Login problems: the user runs
`! firebase login --reauth`, opens the printed URL (replace `&amp;` with `&`), checks the
session ID, then `! firebase login <authorization code>` — the long code from the web page,
not the session ID; every new login command invalidates the previous URL.

## 5. Post-deploy check (read-only)

```bash
for p in / /assets/index.json; do curl -sI "https://letterzoo.web.app$p" | grep -iE "^HTTP|content-type|cache-control"; done
curl -sI "https://letterzoo.web.app/bundle/$(ls <D>/web/dist/bundle | grep wasm)" | grep -iE "^HTTP|content-type"
```

Expect 200, `text/html` + `no-cache` for `/`, `application/wasm` + `immutable` for the
bundle, `max-age=300` for assets. Report the URL, the deployed commit hash and what is not
included.

## 6. Clean up

`git worktree remove --force <D>` (the build folder with its `target/` is several GB).
Optionally record the deployed commit in `git tag deploy-YYYY-MM-DD` if the user wants tags.
