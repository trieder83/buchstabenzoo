---
id: TECH-ARCH
title: Technical architecture
aspect: tech
module: architecture
status: draft
depends_on: []
test_prefix: ARCH
updated: 2026-09-26
---

# Technical architecture

## Decisions

1. **Rust is the main language.** All game logic, rendering and asset loading are Rust,
   compiled to WebAssembly (`wasm32-unknown-unknown`, `wasm-bindgen`, `web-sys`).
2. **Rendering:** raw WebGL2 from Rust — no three.js or other engine.
3. **Host:** thin TypeScript + Vite page: canvas, input forwarding, audio, asset fetch.
4. **Assets:** glTF 2.0 `.glb` from Blender (ART-PIPELINE), loaded with the `gltf` crate.
5. **i18n:** Fluent (`fluent-bundle`).
6. **Mobile:** Capacitor wraps the web build for Android and iOS (TECH-PLATFORMS).

## Crates

| Crate | Responsibility | May depend on web APIs |
|---|---|---|
| `zoo-core` | World, animals, rescue missions, feeding, quests, reading and math levels, math task generator, animation clip state and events (ART-RIG §4.8), save state | no |
| `zoo-render` | WebGL2 renderer, shaders, mesh/animation playback | yes |
| `zoo-assets` | glTF loading, manifest, asset tests | no |
| `zoo-web` | wasm-bindgen entry, main loop, glue to host | yes |

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| ARCH-001 | Given `zoo-core`'s dependency tree, then it contains neither `web-sys` nor `wasm-bindgen`. | unit |
| ARCH-002 | Given the web build, then `node_modules` contains no three.js/babylon/playcanvas package. | unit |
| ARCH-003 | Given a release build, when loaded in headless Chromium, then a WebGL2 context is created and the first frame renders without console errors. | e2e |
