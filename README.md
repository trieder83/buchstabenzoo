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
