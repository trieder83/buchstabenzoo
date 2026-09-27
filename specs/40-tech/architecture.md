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
   bounds, against the frustum (whose far plane is the fog end + 2 m = 22.8 m in the close
   views, GAME-CAMERA-VIEWS 6). Tested by CAMV-001…005, 014, 018.

## Crates

| Crate | Responsibility | May depend on web APIs |
|---|---|---|
| `zoo-core` | World, animals, rescue missions, feeding, quests, reading and math levels, math task generator, animation clip state and events (ART-RIG §4.8), save state | no |
| `zoo-render` | WebGL2 renderer, shaders (incl. 2-tone cel shading and outline pass, §7), mesh/animation playback, face decal UV offset (ART-RIG §6) | yes |
| `zoo-assets` | glTF loading, manifest, asset tests | no |
| `zoo-web` | wasm-bindgen entry, main loop, glue to host | yes |


## Multi-node assets, material slots (kit_night & later kits)

The loader (`zoo-assets`) keeps the conventions of `tools/blender/props/README_night.md`:

- **Parts:** every direct child node of the root that has a mesh is a *part* (index ≥ 1,
  the root and everything baked into it = part 0). The mesh stays one vertex/index list;
  each vertex keeps its part index and the part's pivot (the node's translation = hinge /
  hub). The renderer moves / hides parts in the vertex shader from per-mesh part
  behaviours (by node name: `leaf_l` +90° / `leaf_r` −90° / `door` −100° about +Y,
  `arms`, `lid` about +X, `sails` spinning about +Z, `roof` / `walls_upper` hide bits,
  `night_sky` only while the glow slots are on) and a per-instance open amount (0…1) and
  hide mask — one draw call per model and region, no extra meshes.
- **Material slots:** `*_glow` → per-vertex sRGB emission, drawn like the palette by day and
  flat/unlit at night; `eye_glow` of skinned animals → `#E6F7A0 × (0.25 + 0.75 ×
  luminance)` while the animal's eyes shine (not while `sleep` plays, Q-146); `glass`
  (`alphaMode BLEND`) → its triangles are sorted to the end of the index list and drawn
  after everything opaque with alpha 0.35 into the colour buffer only (depth, normal and
  edge mask unchanged: no outlines inside panes) — one extra draw call per visible model
  with glass; `*_face` → flat slot colour, and the host draws a Fluent text decal on the
  face's UV quad (entrance board, garden signs).
- **Empties:** `light`, `light_*` become lamp positions (night point lights), `socket_*`
  attach points (the scene places lamps / notes / keys on them).
- The shared palette atlas is uploaded once; skinned models share one texture per atlas
  image (`body` and `eye_glow`).
- **Static batching:** a group of static placements that never move (a garden's beds, signs,
  fence and tools; a room's furniture) is merged into one mesh; every merged vertex lies
  where the instanced shader would draw it. Parts that glass-sort, hide, spin or show only
  at night are never merged (ARCH-008).

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
| ARCH-006 | Given every `.glb`, when loaded, then static models keep one part index per vertex (part 0 = root) and their movable / hideable child nodes with the node's pivot (moon door `leaf_l` (−0.96, 0, −0.04), `leaf_r` (0.96, 0, −0.04)), material slots keep name, emission (sRGB of `lamp_glow` ≈ `#FFD66B`) and blend mode (night house `glass`), empties keep their model-space position (`light_l` (−1.35, 3.62, 0)), `*_face` slots give their text quad in reading order (garden sign facing +Z, note face up), and an animal's `eye_glow` shares the body's atlas image. Test: `crates/zoo-assets/tests/models.rs`. | unit |
| ARCH-007 | Given a static model with material slots and parts, when packed for the renderer, then glow vertices carry mode 1 and their sRGB emission, every glass triangle comes after `glass_first` and none before (night house: 72), each vertex carries its part code and pivot, and the part behaviours follow the node names (roof / walls_upper hide bits, leaves ±90° about +Y, sails spin, night sky only at night). Test: `crates/zoo-render/src/renderer.rs`. | unit |
| ARCH-008 | Given a group of static placements that never move (a garden's beds, signs, fence and tools; a room's furniture), when merged into one mesh (static batching), then every vertex lies where the instanced shader would draw it and glass, hiding, spinning and night-only parts are never merged. Test: `crates/zoo-render/src/renderer.rs` (`arch_008_baked_groups_match_their_instances`). | unit |

## Open questions

- Q-050 outline technique (inverted hull vs. screen-space edge pass). Q-026 character texture approach.
