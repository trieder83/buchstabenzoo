# CLAUDE.md — Buchstabenzoo

A **reading education game** in 3D (third person) for children from Kindergarten to
grade 3 (reading) and grades 1–5 (optional math). The zoo animals have escaped and all
enclosures are empty. Per animal (a *rescue mission*, `specs/10-gameplay/rescue-mission.md`):
read the **location riddle** on the enclosure's info board → pick the right **food box** by
reading its label → find the animal where the riddle points → show the food → it follows →
lead it home. Goal: every animal back in its enclosure. Understanding the riddle must be
faster than guessing.

- Visual style: **comic** (cel-shaded, bold outlines) — defined only in `art/style/style.md`;
  high-angle zoo-park camera (~55°). Review page `art/index.html`.
- Primary language: **German (de)**. Also **English (en)**. French (fr) planned later.
- Targets: web browsers (desktop + mobile), later packaged for Android/iOS.

## Tech stack (non-negotiable)

- **Rendering:** raw **WebGL2** — no three.js, Babylon.js, or any other 3D engine/library.
- **Main language: Rust.** Everything that can be Rust is Rust.
- **Game core:** **Rust → WebAssembly** (`wasm32-unknown-unknown`) via `wasm-bindgen` /
  `web-sys`. All game logic, simulation, and rendering calls live in Rust.
- **Host shell:** minimal **TypeScript** + **Vite** — loads the WASM module, owns the
  `<canvas>`, forwards input (touch, mouse, keyboard), handles audio and asset loading.
  Keep JS thin; if logic can live in Rust, it goes in Rust.
- **3D assets:** modelled in **Blender** by **Python scripts** (`tools/blender/`, run headless —
  the source of truth). The **Blender MCP** server (ahujasid `blender-mcp`, see below) is for
  live inspection and tuning. Export as **glTF 2.0 binary (`.glb`)**, loaded in Rust with the
  `gltf` crate.
- **Math:** `glam`. **i18n:** Project Fluent (`fluent-bundle`, `.ftl` files).
- **Mobile packaging (later):** Capacitor wrapping the same web build — one codebase for
  web, Android, iOS.

> `Assets/Scripts/**` (Unity C#) is an **early prototype, kept only as reference** for
> gameplay ideas. Do not extend it; do not port Unity patterns blindly.

## Target layout

```
specs/                 # Source of truth — specs drive everything (see below)
art/                   # Concept art BEFORE modelling: reference/, characters/, animals/, environment/
                       #   index.html = review page (open directly), catalog.js = its data
crates/
  zoo-core/            # Pure game logic: world, animals, quests, riddles, levels. No web deps.
  zoo-render/          # WebGL2 renderer, skinned mesh + animation playback, shaders (GLSL ES 3.00)
  zoo-assets/          # glTF loading, manifest, asset tests (no web deps)
  zoo-web/             # wasm-bindgen entry point, glue between core/render and browser
web/                   # TypeScript host shell (Vite), index.html, input, audio
assets/
  manifest.toml        # every asset + concept_approved gate (ART-PIPELINE)
  levels/              # level layout data (GAME-LAYOUT)
  i18n/{de,en,fr}/     # Fluent .ftl files — every user-visible string lives here
  blender/             # .blend files written by tools/blender/*.py (never hand-edited)
  models/              # Exported .glb files used by the game
  audio/
tests/e2e/             # Playwright end-to-end tests
```

Keep `zoo-core` free of `web-sys`/`wasm-bindgen` so it is unit-testable with plain
`cargo test` on Linux.

## Spec-driven workflow

Every feature starts in `specs/` — **no code without a spec, no spec without tests.**
Conventions (structure, frontmatter, test IDs) are in `specs/README.md`;
`specs/INDEX.md` is generated; names follow `specs/glossary.md`.

1. Write/update the spec before implementing. Undecided points go to
   `specs/open-questions.md` (`Q-###`) — never silently assume.
2. Write the tests from the spec's test cases first (test ID in name or comment, e.g.
   `// FEED-001`).
3. Implement until the tests pass.
4. If implementation reveals a gap or contradiction, fix the spec first, then the code.
5. A feature is done only when all its spec test cases are covered and green.
6. Run the `spec-manager` agent after spec changes.

## Project agents (`.claude/agents/`)

| Agent | Responsibility |
|---|---|
| `spec-manager` | Owns `specs/`: regenerates `INDEX.md`, checks structure, glossary naming, contradictions, gaps, missing tests; records open questions; cleans up outdated specs; logs every resolved problem in `specs/fixes/FIX-NNN-*.md`. |
| `zoo-level-designer` | Zoo map: enclosures, buildings, paths, landmarks, and barriers (road blocks, stones, gates) that limit each level. Owns GAME-LAYOUT, `specs/10-gameplay/levels/`, `assets/levels/`, environment mockup briefs. |
| `character-artist` | Human characters only: briefs/turnarounds, Blender model, rig, skinning, animation, `.glb` export. Owns ART-RIG and ART-CHARACTERS. |
| `gameplay-qa` | Plays the game automatically (desktop + touch) and verifies gameplay quality: movement, collision, interaction prompts, camera, touch controls, mission flow. Reports findings in `qa/reports/`, adds regression tests. Run after gameplay/renderer changes and before a milestone is done. |

Agents never decide game design silently — they add open questions and report them.

## Testing

| Layer | Tool | Command |
|---|---|---|
| Game logic (`zoo-core`) | `cargo test` | `cargo test -p zoo-core` |
| WASM/browser glue | `wasm-bindgen-test` (headless) | `wasm-pack test --headless --firefox crates/zoo-web` |
| TS host shell | Vitest | `npm --prefix web test` |
| End-to-end (real browser, WebGL) | Playwright | `npm --prefix web run test:e2e` |
| i18n completeness | test in `zoo-core` | every key in `de` must exist in `en` (and `fr` once added) |

Prefer testing logic in `zoo-core` (fast, deterministic). Rendering is tested via
Playwright smoke tests + screenshot comparisons, not by unit-testing GL calls.

## Local development (Linux)

```bash
# one-time setup
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
npm --prefix web install
npx --prefix web playwright install

# dev loop
wasm-pack build crates/zoo-web --target web --dev   # build WASM
npm --prefix web run dev                            # Vite dev server, open on phone via LAN IP

# checks before committing
cargo fmt --all && cargo clippy --all-targets -- -D warnings
cargo test --workspace
npm --prefix web run lint && npm --prefix web test
```

## 3D asset pipeline (Blender + MCP)

- **Concept first:** no modelling before the concept is approved. Concept images live in
  `art/` and are reviewed via `art/index.html`; add every new image to `art/catalog.js`.
- Order for environments: approved **style frame** → level layout → **greybox** in Blender
  (throwaway, `art/greybox/`, never exported) → mockups generated from greybox renders +
  style frame → approval → real modelling.
- Characters and animals need a **turnaround sheet** (front, side, back, ¾); environment areas need **mockups**
  (overview + player view + layout). A human sets `concept_approved = true` in
  `assets/manifest.toml`. Details: `specs/30-art/asset-pipeline.md`.
- Models are built by scripts: `blender -b --python tools/blender/<kind>/<asset>.py` writes
  `assets/blender/…/.blend`, exports `assets/models/…/.glb` and a preview render — commit all.
  Change a model by changing its script, never by hand-editing the `.blend`.
- **Blender MCP (live):** ahujasid `blender-mcp` — in Blender the add-on (N-panel → BlenderMCP →
  Connect, port 9876); in Claude Code registered per project with
  `claude mcp add blender -- uvx blender-mcp`. Use it to look at models, test lighting/camera
  and try changes; port anything worth keeping back into the script.
- **Generating concept art:** `tools/gen_image.py <brief.md> --prompt N --out a.png b.png
  --ref art/environment/style_frame/style_frame.png` (Gemini, `GEMINI_API_KEY`); `--prompt 0
  --extra "..." --ref img` edits an image. Characters/animals/props: generate one **sheet**
  with all views (consistent angles), then `tools/split_sheet.py`. Every generated image is
  logged in its brief; record choices there and in `art/catalog.js`.
- **One style for all prompts:** every image prompt copies the blocks from
  `art/style/style.md` verbatim (APIPE-010). Comic look: bold outlines, flat colours, one
  hard shadow tone, rounded chunky shapes. Outlines + cel shading come from the renderer,
  not from textures. Low poly counts (mobile).
- Conventions: 1 unit = 1 metre, Y-up on export, origin at the model's feet, apply
  transforms before export, name animations (`idle`, `walk`, `eat`, …) consistently.
- Each asset is listed in a spec (e.g. `specs/30-art/animals.md`) with its required
  animations; an automated test checks every referenced `.glb` exists and loads.

## Conventions

- **Localization:** never hard-code user-visible text. All strings, riddles, food labels,
  and visitor dialogue go through Fluent keys. German is written first and is the
  reference locale; English must be kept in sync in the same change.
- **Reading and math levels:** readable content is tagged per `reading_level`
  (`kiga`, `klasse1`, `klasse2`, `klasse3`); math tasks per `math_level` (`mathe1`–`mathe5`,
  CONT-MATH). Gameplay code must not assume one level. "Level" alone means a map level
  (GAME-LAYOUT).
- **Child-friendly UX:** large touch targets, no reading required to navigate menus
  (icons + audio), no time pressure unless a spec says so, no ads/external links.
- **Performance:** target 60 fps on mid-range phones — batch static meshes, minimise
  draw calls, avoid per-frame allocations in the render loop.
- **Code/identifiers/comments in English;** game content in German/English via i18n.
- Deterministic game logic: seedable RNG so tests are reproducible.
