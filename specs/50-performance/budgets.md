---
id: PERF-BUDGETS
title: Performance budgets
aspect: performance
module: budgets
status: draft
depends_on: [TECH-PLATFORMS, TECH-ARCH, PROD-POC, GAME-CAMERA-VIEWS, GAME-NIGHT, GAME-AMBIENT, TECH-WATER, ART-PIPELINE, ART-RIG, ART-ANIMALS]
test_prefix: PERF
updated: 2026-09-27
---

# Performance budgets

## Goal

One place for every performance budget of Buchstabenzoo as a testable rule: the game runs
at **60 fps on a mid-range phone (minimum 30 fps)**, downloads in ≤ 30 MB and keeps the
comic look. The budgets below were scattered over TECH-PLATFORMS, PROD-POC,
GAME-CAMERA-VIEWS, GAME-NIGHT, GAME-AMBIENT, TECH-WATER and the ART specs; this spec
collects them, names the source, and adds the rules the `performance` agent measures
(`specs/50-performance/measurements.md`, how to run: its first section). The source spec
stays authoritative for its own rule; a budget changes only with a reason written next to
it. Undecided budgets are open questions — never invented here.

## Behaviour

Numbers in **bold** are the budget; "source" is the spec that owns it.

| # | Budget | Value | Source | Status |
|---|---|---|---|---|
| 1 | Frame rate on the reference mid-range phone | **60 fps target (16.7 ms), 30 fps minimum (33.3 ms)** while walking, in every view and by night | TECH-PLATFORMS "Performance", POC-005; GAME-NIGHT §10 ("performance budget as by day") | decided; reference device open (Q-013) |
| 2 | Total download on first load (release build) | **≤ 30 MB** | TECH-PLATFORMS, PLAT-001, POC-006 | decided |
| 3 | Draw calls anywhere in the joined zoo, zoo view at max zoom-out (20 m), ambient animals included | **≤ 140 draw calls, ≤ 10 000 instances** (proposal; the e2e test `m5b.spec.ts` asserts < 140) | Q-104 (open) | proposed |
| 4 | Close views vs. zoo view at the same spot | first person and look-around need **fewer** draw calls than the zoo view at 20 m | GAME-CAMERA-VIEWS 6, CAMV-014 | decided |
| 5 | Night vs. day at the same spot | the night adds **≤ 6 draw calls** (night props batched); frame time is reported | GAME-NIGHT, NIGHT-014 | decided |
| 6 | Night lights | **1 lantern + the 8 nearest lamps** as point lights (`MAX_POINT_LIGHTS` = 9), the **next 24** as light pools in the same shader (no extra draw call), farther lamps emissive only | GAME-NIGHT "Renderer", Q-114 (answered) | decided |
| 7 | Water animation | **+0 draw calls**, **≤ 1 ms GPU** per frame on the reference phone | TECH-WATER, WATER-008, WATER-012, AENV-010 | decided |
| 8 | Ambient animals of a level | **≤ 10 draw calls** and **≤ 0.5 ms CPU** per frame | GAME-AMBIENT rule 10, AMB-007 | decided |
| 9 | Sky and distance haze | **+0 draw calls** (drawn in the outline pass) | GAME-CAMERA-VIEWS 7, TECH-ARCH §9 | decided |
| 10 | Triangles per model | characters and animals **≤ 3 000** (target ~2 000–2 500); props **≤ 500**; buildings: **none yet** | ART-PIPELINE rule 10 / APIPE-005, ART-RIG §5 / RIG-018, ART-ANIMALS rule 7 / AANI-004; buildings Q-153 | decided (buildings open) |
| 11 | Textures per model | flat-colour atlases **≤ 512 × 512**; characters body **≤ 256²** + face atlas **≤ 256 × 128**; animals **≤ 256²** | ART-PIPELINE rule 10, ART-RIG §5, AANI-004 | decided |
| 12 | `.glb` file size | characters and animals **≤ 400 KB** including textures | ART-RIG §5, ART-ANIMALS rule 7 | decided |
| 13 | Skeleton | **20 joints** per character (target ≤ 24, hard max 32); the renderer accepts ≤ 128 | ART-RIG §2 "Joint budget" | decided |
| 14 | Heap allocations per frame | **0** in the steady state of the render loop (`Renderer::render`, the zoo-web frame) — "avoid per-frame allocations in the render loop" | CLAUDE.md "Performance"; exact scope Q-169 | proposed |
| 15 | Static meshes are batched | every group of never-moving static placements is merged (one draw call per group and region); static batches are culled per 8 m chunk | TECH-ARCH "Static batching" / ARCH-008, GAME-CAMERA-VIEWS 6 | decided |
| 16 | Frame time per scenario on a desktop / CI reference, per-pass GPU time (scene pass, full-screen outline + fog + sky pass) | **undecided** | Q-166 | open |
| 17 | WASM size (release, compressed) and first-load time to the first frame | **undecided** | Q-167 | open |
| 18 | Memory: GPU (G-buffer, textures, vertex buffers) and WASM heap | **undecided** | Q-168 | open |
| 19 | Triangles on screen per view | **undecided** (measured and reported only) | Q-166 | open |

Rules:

1. The fixed scenarios of `measurements.md` (S01…S11, desktop 1920 × 1080 and phone
   1080 × 2340 portrait) are measured after every big change (new models, renderer or scene
   work, new levels/features) and before a milestone is called done.
2. Headless SwiftShader (software WebGL) numbers are **relative**: they are compared run to
   run; absolute frame times need the reference phone (Q-013) or a real GPU and are logged
   separately.
3. A value more than 10 % worse than the previous run is a **regression** and is flagged in
   the log; a budget that is exceeded gets a `PERF-R-NNN` recommendation.
4. Budgets never change the look or gameplay silently: trading style or content for speed
   needs a `Q-###` answered by the user (quality tiers: Q-170).

## Acceptance criteria

- Every budget above has a test case below; decided budgets are checked automatically where
  possible (unit / asset / e2e), otherwise manually on the reference device.
- `measurements.md` has a run for the current milestone with all scenarios.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| PERF-001 | Given the reference phone (Q-013) and the release build, when walking through levels 1–3 in the zoo view, in first person and at night, then the frame rate stays ≥ 30 fps and reaches 60 fps in the level-1 spawn view (budget 1, POC-005). | manual |
| PERF-002 | Given the release build served locally, when the game loads from an empty cache, then the sum of transferred bytes (navigation + resources) is ≤ 30 MB (budget 2, PLAT-001, POC-006; measured by `tools/perf/run.sh` → "transferred"). | e2e |
| PERF-003 | Given the joined zoo at the level-1 spawn, a level border and the level-3 spawn in the zoo view at 20 m, then draw calls ≤ 140 and instances ≤ 10 000 (budget 3, Q-104; today `m5b.spec.ts`). | e2e |
| PERF-004 | Given the same player position, then the draw calls in first person and in look-around are lower than in the zoo view at 20 m (budget 4 = CAMV-014, `camera_views.spec.ts`). | e2e |
| PERF-005 | Given the same spot by day and at night, then the night adds ≤ 6 draw calls (budget 5 = NIGHT-014, `night.spec.ts`). | e2e |
| PERF-006 | Given night, then the renderer gets at most 9 point lights and 24 light pools per frame, and adding the light pools adds no draw call (budget 6; `light_stats()`, `zoo_render::night` unit tests). | unit |
| PERF-007 | Given level 1 with water animation on and off, then the draw calls are equal (budget 7 = WATER-008); on the reference phone the GPU time difference is ≤ 1 ms (WATER-012). | e2e / manual |
| PERF-008 | Given level 1 with all ambient animals, then they add ≤ 10 draw calls and ≤ 0.5 ms CPU per frame (budget 8 = AMB-007, `water.spec.ts`; native probe "Ambient"). | e2e |
| PERF-009 | Given a close view with sky, then the draw calls equal those of the same view without sky (budget 9; the sky is part of the one full-screen pass). | e2e |
| PERF-010 | Given every `.glb` in `assets/models/`, then its triangle count is within the budget for its kind (budget 10 = APIPE-005; buildings once Q-153 is answered). | asset |
| PERF-011 | Given every `.glb`, then its textures are within budget 11 and character / animal files are ≤ 400 KB (budget 12; RIG-018, AANI-004). | asset |
| PERF-012 | Given every skinned `.glb`, then it has ≤ 32 joints (characters: 20 ± 4) (budget 13). | asset |
| PERF-013 | Given the game running a steady scenario (S01), when a frame is rendered, then `Renderer::render` and the zoo-web frame make no heap allocation; `zoo_core::game::Game::update` makes none outside events (budget 14, Q-169; native probe for zoo-core, allocation counter in the WASM build once it exists). | unit |
| PERF-014 | Given the level scene, then every bake group of never-moving placements is drawn as one merged mesh per region (budget 15 = ARCH-008) and the draw list of S01 (`debug_draw_list`) contains no single-instance batch of a bakeable model. | unit / e2e |
| PERF-015 | Given `tools/perf/run.sh`, then it reports for every fixed scenario and viewport the frame CPU and interval p50/p95, draw calls, instances, triangles, GL calls, program switches, texture binds, uniform calls, uploads, scene / full-screen pass split, simulation step, the WASM and download sizes and the load time, and flags values > 10 % worse than the previous run (rules 1–3). | manual |

## Open questions

- Q-013 reference device (budget 1), Q-104 draw-call budget of the joined zoo (budget 3),
  Q-153 triangle budget for `buildings` (budget 10).
- Q-166 frame-time / per-pass budgets on a measurable reference and triangles on screen.
- Q-167 WASM size and first-load time budgets.
- Q-168 memory budget (GPU and WASM heap).
- Q-169 scope of the "no per-frame allocation" rule.
- Q-170 quality tiers for weak phones.
