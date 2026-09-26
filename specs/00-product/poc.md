---
id: PROD-POC
title: Proof of concept — zebra mission on level 1
aspect: product
module: poc
status: draft
depends_on: [PROD-VISION, GAME-RESCUE, GAME-PLAYER, GAME-LEVEL-1, CONT-MISSIONS, TECH-ARCH, ART-PIPELINE]
test_prefix: POC
updated: 2026-09-26
---

# Proof of concept — zebra mission on level 1

## Goal

Prove the whole stack end to end with **one playable rescue mission** (zebra) on level 1,
in a desktop and a mobile browser: Rust/WASM game logic, raw WebGL2 comic renderer with
the high camera, glTF assets from scripted Blender, data-driven level, reading panel in
German and English. Scope is deliberately small; everything else is out.

## In scope

| Area | What | Specs |
|---|---|---|
| Level | Level 1 built at runtime from `assets/levels/level-1.toml` + kit models; walkable grid, path vs. grass speed, sealed by barriers/boundary | GAME-LEVEL-1, GAME-LAYOUT |
| Player | `player_girl` only (boy later), walk with joystick / WASD, idle + walk animation | GAME-PLAYER, ART-RIG |
| Camera | High-angle follow camera: 55°, 35° vertical FOV, zoom 10–20 m, 45° rotation steps | GAME-PLAYER §2 |
| Rendering | WebGL2, 2-tone cel shading, outline pass, flat-colour textures | ART-DIRECTION, TECH-ARCH §7 |
| Mission | Zebra: info board riddle (panel) → food storage → pick `Gras` box → zebra at `loc_river` → show food → follows → enclosure → `happy` | GAME-RESCUE, GAME-FEED, CONT-MISSIONS §1 |
| Reading | Text panel with the zebra riddle and food labels, reading level selectable (`kiga`–`klasse3`), `de` + `en` via Fluent | CONT-READING, CONT-L10N |
| Assets | Kits 1–6 (only props used by level 1), zebra, girl; buildings as simple block-outs | ART-PIPELINE |

## Out of scope (PoC)

Hippo and panda missions (their enclosures and hiding places exist as scenery only), boy,
map (GAME-MAP), math tasks, visitors/quests, audio/read-aloud, saving, mobile packaging
(Capacitor), final building models, level transitions.

## Placeholders

Code never waits for art: every missing `.glb` is replaced by a coloured box of the asset's
size (from the layout data), so logic and camera can be tested before models exist.
Placeholders are logged as warnings and must be gone for POC-004.

## Milestones

| # | Milestone | Done when |
|---|---|---|
| M1 | Workspace + core logic | `cargo test --workspace` green: grid/surfaces, walk speeds, animal states, rescue flow, riddle/food data (unit tests from the specs) |
| M2 | Assets v1 | Ground/fence/sign kits, trees, water, bridge, zebra, girl exported as `.glb`; asset tests green (APIPE-001…010) |
| M3 | Renderer | Level 1 renders in the browser with the high camera, cel shading and outlines; placeholder boxes allowed |
| M4 | Playable | The zebra mission can be played end to end with keyboard and touch |
| M5 | PoC done | All POC tests green on desktop Chrome/Firefox and a mid-range phone |

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| POC-001 | Given the release build served locally, when opened in headless Chromium at 1080×2340, then level 1 renders with a WebGL2 context and no console errors (ARCH-003). | e2e |
| POC-002 | Given reading level `klasse1` and language `de`, when a scripted player runs the zebra mission (board → Gras box → river → show food → enclosure), then mission `zebra` completes (RESC-010). | e2e |
| POC-003 | Given the same run with language `en`, then all panel texts are English and the mission completes. | e2e |
| POC-004 | Given the PoC build, then no placeholder box is rendered (all level-1 assets used by the zebra mission are real `.glb` models). | e2e |
| POC-005 | Given a mid-range phone (Q-013 reference device) in Chrome, then the level renders at ≥ 30 fps while walking. | manual |
| POC-006 | Given the PoC build, then the total download size is ≤ 30 MB (PLAT-001). | e2e |

## Open questions

- Q-013 Reference device for POC-005.
