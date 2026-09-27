---
id: TECH-ARCH
title: Technical architecture
aspect: tech
module: architecture
status: draft
depends_on: []
test_prefix: ARCH
updated: 2026-09-27
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
7. **Comic shading in the renderer** (Q-010 answered, ART-DIRECTION §2): `zoo-render` draws
   cel shading (one hard shadow tone, 2-tone shader) and the dark outline for characters,
   animals and props; textures and meshes contain neither. Outline technique: Q-050.
8. **Coordinate spaces** (Q-056 answered, GAME-LAYOUT): `zoo-core` works in level
   coordinates (x east, z north); `zoo-render` works in right-handed Y-up world space. The
   only conversion is `zoo_core::coords::level_to_world` / `world_to_level`
   (`world = (x, 0, −z)`); models are never mirrored (tests LAYOUT-007…011).
9. **Camera views, sky and haze** (GAME-CAMERA-VIEWS): the camera poses and glides are pure
   math in `zoo_render::camera` (constants in `zoo_core::view`); the sky and the distance
   haze are drawn in the outline pass from the depth buffer (no extra draw call). Static
   batches are culled per 8 m chunk of their instances, decals and skinned characters per
   bounds, against the frustum (whose far plane is the fog end + 2 m = 18 m in the close
   views, GAME-CAMERA-VIEWS 6). Tested by CAMV-001…005, 014, 018.

## Crates

| Crate | Responsibility | May depend on web APIs |
|---|---|---|
| `zoo-core` | World, animals, rescue missions, feeding, quests, reading and math levels, math task generator, animation clip state and events (ART-RIG §4.8), save state | no |
| `zoo-render` | WebGL2 renderer, shaders (incl. 2-tone cel shading and outline pass, §7), mesh/animation playback, face decal UV offset (ART-RIG §6) | yes |
| `zoo-assets` | glTF loading, manifest, asset tests | no |
| `zoo-web` | wasm-bindgen entry, main loop, glue to host | yes |


## Rendering hygiene (z-fighting)

Geometry built by code (placeholders, rims, perches, arches) must never place two faces in
the same plane facing the same way: stacked parts end at each other's boundary (a pillar
stops under the beam instead of reaching its top), rims/segments differ by ≥ 1 cm in
height, platforms sit ≥ 1 cm above what they stand on. ARCH-005 checks this for the whole
joined zoo; models from Blender follow the same rule (no overlapping coplanar faces between
parts of one model or between models that are meant to touch).

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| ARCH-001 | Given `zoo-core`'s dependency tree, then it contains neither `web-sys` nor `wasm-bindgen`. | unit |
| ARCH-002 | Given the web build, then `node_modules` contains no three.js/babylon/playcanvas package. | unit |
| ARCH-003 | Given a release build, when loaded in headless Chromium, then a WebGL2 context is created and the first frame renders without console errors. | e2e |
| ARCH-004 | Given a reference scene (lit sphere + a character) rendered in headless Chromium, then every lit surface shows at most two shading tones per material and an outline in the outline colour surrounds the silhouette (screenshot comparison against an approved reference). | e2e |
| ARCH-005 | Given the assembled scene of the joined zoo (all placeholder and fallback boxes), then no two boxes have coplanar, same-facing, overlapping faces (no z-fighting / flicker while the camera moves). User report 2026-09-26: the entrance arch top flickered red/blue. Test: `crates/zoo-core/tests/zfight.rs`. | unit |

## Open questions

- Q-050 outline technique (inverted hull vs. screen-space edge pass). Q-026 character texture approach.
