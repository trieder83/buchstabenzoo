# web/ — TypeScript host shell (Vite)

Thin host (TECH-ARCH): loads the WASM game from `crates/zoo-web/pkg` (built by wasm-pack),
owns the `<canvas>`, fetches asset files and forwards input. All game logic and rendering
are Rust.

```bash
npm --prefix web install
npx --prefix web playwright install chromium   # once, for the e2e test

npm --prefix web run dev        # wasm-pack --dev build + Vite dev server (LAN: --host)
npm --prefix web run build      # wasm-pack --release + tsc + vite build -> web/dist
npm --prefix web run preview    # serve web/dist
npm --prefix web run lint       # tsc --noEmit
npm --prefix web test           # Vitest (input helpers, ARCH-002)
npm --prefix web run test:e2e   # build + Playwright (POC-001/ARCH-003 smoke, review shots)
```

**Assets:** nothing is copied into `web/`. A small Vite plugin (`vite.config.ts`,
`zooAssets`) serves `assets/{levels,models,textures,i18n}` of the repo under `/assets/` in
dev and preview, writes them to `dist/assets/` on build, and publishes
`/assets/index.json`. The host fetches `index.json`, asks WASM (`required_assets`) which
files the level needs, and fetches only those that exist; a missing model becomes a
placeholder box (PROD-POC "Placeholders", logged as a console warning).

**Controls (GAME-PLAYER §3):**
- Desktop: WASD/arrows walk (camera-relative), `Q`/`R` or mouse drag rotate the camera in
  45° steps, wheel / `+` `-` zoom (10–20 m), `E` / Space / Enter interact (a key hint shows
  when something is in reach; it can be clicked), Escape closes the panel.
- Touch (only after the first touch, never for mouse only): left half = floating joystick
  under the thumb (radius 60 px, dead zone 10 %), right half = swipe ≥ 40 px rotates one
  step, two fingers pinch-zoom, round button bottom-right interacts. Both thumbs at once.
- Gear button (top right): language (flags) and reading level (🧸 1 2 3), stored in
  `localStorage` (`zoo.language`, `zoo.readingLevel`).

**Overlays** (`src/ui.ts`): interact button/hint, text panel (riddle / food box label + take
button), carried food HUD, feedback bubble, mission celebration. What is interactable and all
texts come from WASM (`target_kind`, `interact`, `take_food`, `poll_events`, `t`); icons are
emoji placeholders.

**Debug handle:** `window.__zoo = { app, ui, frames, frameMs, intervalMs }`; `app` exposes
`player_x/z`, `player_speed`, `surface_speed`, `camera_distance`, `camera_target_yaw_deg`,
`draw_calls`, `instances`, `triangles`, `placeholders`, `player_is_model`,
`animal_is_model`, `animal_state/x/z`, `mission_started/complete`, `debug_teleport(x, z)`,
`debug_goto(x, z)` + `debug_step(seconds)` (scripted walk with real movement/collision,
simulated without rendering), `quality()` / `quality_mode()` / `pixel_ratio()` and
`debug_no_culling(on)`, `debug_haze_cull(on)` (performance checks).

**Quality tier** (PERF-BUDGETS rule 5, `src/quality.ts`): automatic by default — slow
frames switch the game to the pixel ratio 1.5, then to the lantern + 4 lamps and no clouds,
never back. `?quality=auto|high|low1|low` overrides it; automated browsers (Playwright,
`navigator.webdriver`) start with `high` so e2e and perf numbers stay comparable.

E2E screenshots go to `art/environment/poc/`.
