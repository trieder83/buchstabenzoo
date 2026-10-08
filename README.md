# Letter Zoo (Buchstabenzoo)

A comic-style 3D **reading adventure for children** (kindergarten to grade 3, optional math).
The zoo animals have escaped and every enclosure is empty. Read the riddle on each enclosure's
info board, pick the right food box, find the animal where the riddle points, show it the food
and lead it safely back home. German and English.

## Play

- **itch.io:** https://edugamegalaxy.itch.io/letter-zoo (free or donation, runs in the browser)
- **Web:** https://letterzoo.web.app
- **Trailer (15 s):** https://youtu.be/lkmNmxssVUw

## Tech

Raw WebGL2, game core in Rust compiled to WebAssembly, thin TypeScript/Vite host shell,
3D assets modelled in Blender by Python scripts. Specs drive everything (`specs/`).
See `CLAUDE.md` for the project rules and layout.

## Develop

```bash
rustup target add wasm32-unknown-unknown && cargo install wasm-pack
npm --prefix web install
wasm-pack build crates/zoo-web --target web --dev   # or: npm --prefix web run wasm:dev
npm --prefix web run dev                            # Vite dev server

cargo fmt --all && cargo clippy --all-targets -- -D warnings && cargo test --workspace
npm --prefix web run lint && npm --prefix web test
```

Releases: `scripts/itch-build.sh` builds the itch.io HTML5 zip; the Firebase deploy and the itch.io
upload steps are described in `.claude/skills/deploy` and `.claude/skills/itch-release`.

Concept art, store covers and screenshots: open `art/index.html`.

## How to read the numbers (anonymous counters)

The game counts events without any identifier (spec: `specs/40-tech/platforms-and-testing.md` "Anonymous counters"). Nobody can read
the numbers from a client; you read them with your own Google login:

```bash
gcloud auth login                                   # once (account with access to project letterzoo)
node tools/analytics/report.mjs                     # last 7 days, per event + param
node tools/analytics/report.mjs --from 20261001 --by day,event --csv
node tools/analytics/report.mjs --event lock_wrong --by platform,lang
```

Without gcloud: Firebase console -> Firestore Database -> collection `c` (document id `<day>_<event>_<param>_<platform>_<lang>_<version>`,
field `n` = count). Check the rules against the live project with `node --experimental-strip-types tools/analytics/rules_check.mjs`.
Deploy rule changes with `firebase deploy --only firestore:rules --project letterzoo`. Test days/documents are never written by the tests
(they stub the endpoint); `rules_check.mjs` writes only `c/20000101_session_start__web_de_rulecheck`.
