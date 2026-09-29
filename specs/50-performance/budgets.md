---
id: PERF-BUDGETS
title: Performance budgets
aspect: performance
module: budgets
status: draft
depends_on: [TECH-PLATFORMS, TECH-ARCH, PROD-POC, GAME-CAMERA-VIEWS, GAME-NIGHT, GAME-AMBIENT, TECH-WATER, ART-PIPELINE, ART-RIG, ART-ANIMALS]
test_prefix: PERF
updated: 2026-09-28
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
| 14 | Heap allocations per frame | **0** per steady-state frame in zoo-core `Game::update`, zoo-render `Renderer::render` and the zoo-web frame (game events and UI changes excepted); checked by a counting-allocator unit test for zoo-core, the native probe and review for the rest | CLAUDE.md "Performance"; Q-169 (answered 2026-09-28) | decided |
| 15 | Static meshes are batched | every group of never-moving static placements is merged (one draw call per group and region); static batches are culled per 8 m chunk | TECH-ARCH "Static batching" / ARCH-008, GAME-CAMERA-VIEWS 6 | decided |
| 16 | Frame time per pass on the reference phone | per frame **≤ 4 ms CPU** (simulation + GL submission), **scene pass ≤ 8 ms**, **full-screen outline + fog + sky pass ≤ 3 ms** at the capped 720 × 1560 buffer, **night ≤ 1.3 × day** at the same spot; in CI (SwiftShader) only the regression rule 3 applies (no scenario > 10 % worse than the previous run, counts exact). The numbers are confirmed by one measurement on the real phone before they are enforced | Q-166 (answered 2026-09-28); reference phone Q-013 | decided (phone run pending) |
| 17 | WASM size and first load | release WASM **≤ 2.0 MB raw / ≤ 600 KB brotli**; first frame **≤ 5 s** on the reference phone over Wi-Fi from a server that sends brotli / gzip, **≤ 3 s** from the Capacitor app package; revisit when level 4+ and audio arrive | Q-167 (answered 2026-09-28); hosting Q-012 | decided |
| 18 | Memory on the reference phone | **WASM heap ≤ 64 MB, JS heap ≤ 32 MB, GPU ≤ 96 MB** (G-buffer, textures, vertex / instance buffers) — a guard for older 3 GB phones | Q-168 (answered 2026-09-28) | decided |
| 19 | Triangles and instances on screen | **≤ 300 k triangles and ≤ 10 000 instances** submitted per frame in every fixed scenario (`RenderStats`) | Q-166 (answered 2026-09-28) | decided |
| 20 | Shared per-frame uniforms | the values every scene program shares (view, view-projection, sun, shadow tint, fade, dither, night / warm, water clock, glow, point lights, light pools) are uploaded **once per frame** as one uniform block (one `bufferSubData`), never per program; uniform locations are looked up only when a program is linked; a per-draw uniform is sent only when its value changed. *Reason:* ≈ 60 % of the WebGL calls per frame were `uniform*` calls re-sending the same values up to 7 × per frame (run 2026-09-27, PERF-R-003); each call is a WASM → JS → GPU-process round trip. | CLAUDE.md "Performance"; PERF-R-003 (accepted by the user 2026-09-28) | decided |
| 21 | Night lights per draw | each draw (static batch, chunk range, character, crowd) gets only the point lights and light pools whose hard-edged sphere / disc can reach its bounds (`u_light_mask`), and a fragment skips a light outside its radius (`dot(to, to) ≥ r²`) before any other work; the picture is **identical** to shading every fragment with every light (budget 6 unchanged). The edge width of a light comes from the position derivatives taken **once per fragment before any branch** (`light_derivs`), never from `fwidth` inside the light loops (undefined in non-uniform control flow). *Reason:* the per-fragment light loop doubled the night scene pass (run 2026-09-27, PERF-R-001); the derivative change removes dark rim specks and is the approved look (Q-180, PERF-R-014, 2026-09-28). | GAME-NIGHT "Renderer", budget 6; PERF-R-001, PERF-R-014 (accepted by the user 2026-09-28) | decided |
| 22 | Static batches drawn per chunk range | a never-moving batch with instances in more than one 8 m chunk is uploaded **sorted by chunk** (row-major) and draws only the runs of chunk ranges in view — at most **3 draws per batch** (`MAX_CHUNK_RUNS`), off-screen chunks between two runs drawn along when they cost ≤ 16 384 triangles (`GAP_MERGE_TRIANGLES`); the picture is identical to drawing every instance; draw calls stay within budget 3 and the close views keep fewer draw calls than the zoo view at 20 m (budget 4). *Reason:* WebGL2 has no base instance, so a drawn batch drew **all** its instances — the whole level's ground tiles (≈ 85–90 % of the submitted triangles, S09 454 k > budget 19; run 2026-09-27, PERF-R-002). | TECH-ARCH §9, ARCH-008; PERF-R-002 (accepted by the user 2026-09-28) | decided |
| 23 | Culling never changes the picture | every culling box (region, chunk, chunk range) holds its instances at any yaw (`±radius` for quarter turns, the mesh's largest horizontal vertex distance otherwise), the space swept by turning parts (door leaves, lids, sails, turnstile arms: each vertex's circle about its part axis), the parts below the origin, and a margin for bobbing, stretched and re-yawed instances; a frame with culling equals the same frame without culling, also while the view turns (no popping at the screen edge). The one approved exception is the **haze culling** of the full close views (PERF-R-018, Q-193 answered yes 2026-09-28): boxes whose nearest point lies beyond the fog end (+ 5 cm) are skipped — it may change 1–4 outline pixels inside the haze; PERF-025 switches it off in both frames (`debug_haze_cull(false)`), because the chunk runs of budget 22 may draw a haze-culled range along as a gap in one frame and not in the other. *Reason:* user report 2026-09-28 "turning is slightly flickery"; the old boxes (`pos ± radius`, 0.1 m below the origin) could drop a yawed or stretched mesh whose corner was already in view, and per-chunk ranges (budget 22) cull more finely (PERF-R-015). | GAME-CAMERA-VIEWS 6; PERF-R-015 (a no-look-change fix) | decided |

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
   needs a `Q-###` answered by the user.
5. **Automatic quality tier** (Q-170, answered 2026-09-28): when the p95 frame time stays
   above 33 ms for 3 s, the game switches itself to a "low" tier — first the pixel ratio
   1.5 (lines stay 2 px); only if still too slow, the lantern + 4 lamps as point lights (the
   rest light pools) and no clouds in the close-view sky. Never a menu the child must read.
   Implementation (PERF-R-005, `zoo_core::quality`): 3 s windows of real frame intervals;
   p95 > 33 ms ⇔ more than 5 % of the window's frames slower than 1/30 s; one step down per
   slow window (`high` → `low1` = pixel ratio 1.5 → `low` = + lantern & 4 lamps, no clouds);
   **never back up within a session** (no flicker); the first 5 s after the start, 2 s after
   a switch and single intervals > 1 s (pauses, background tab) are ignored. The outline
   sample offset at the pixel ratio 1.5 is 3 device px (2 CSS px, as at the ratios 1 and 2).
   Debug override `?quality=auto|high|low1|low`; automated browsers (`navigator.webdriver`,
   the e2e tests and `tools/perf`) start with `high` unless the URL says otherwise, so their
   numbers stay comparable. Desktop and fast phones stay `high`.

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
| PERF-018 | Given the reference phone (Q-013) and the release build, when the fixed scenarios S01–S11 run, then per frame CPU ≤ 4 ms, scene pass ≤ 8 ms, full-screen pass ≤ 3 ms, and S10 / S11 ≤ 1.3 × the day scenario at the same spot (budget 16; `PERF_ANGLE=gl tools/perf/run.sh` against the phone, GPU timer queries PERF-R-013). | manual |
| PERF-019 | Given `tools/perf/run.sh`, then the release WASM is ≤ 2.0 MB raw and ≤ 600 KB brotli (budget 17, `sizes.json`); on the reference phone the first frame comes ≤ 5 s after navigation over Wi-Fi with compression and ≤ 3 s from the app package. | e2e / manual |
| PERF-020 | Given the reference phone after loading and after S01–S11, then the WASM heap ≤ 64 MB, the JS heap ≤ 32 MB and the GPU memory estimate (G-buffer + textures + buffers) ≤ 96 MB (budget 18; `run.sh` reports the heaps, GPU memory estimated by the renderer once a counter exists). | manual |
| PERF-021 | Given every fixed scenario and viewport of `run.sh`, then triangles ≤ 300 k and instances ≤ 10 000 (budget 19, `RenderStats`). | e2e |
| PERF-022 | Given a p95 frame time above 33 ms for 3 s, then the game switches itself to the low tier (pixel ratio 1.5 first; then fewer point lights and no clouds), without any menu and never back; given fast frames (a few hitches, pauses), it stays in the high tier; `?quality=` pins a tier (rule 5, Q-170; PERF-R-005; unit: `zoo_core::quality` `perf_022_*`, host `quality.test.ts`; e2e: `quality.spec.ts`). | unit / e2e |
| PERF-023 | Given a static batch with instances in several chunks, then its instances are on the GPU sorted by chunk (every chunk one contiguous range), a frame draws only the runs of visible chunk ranges (≤ 3 per batch, cheap gaps merged), and the look scenarios L01–L14 are pixel-identical to drawing whole batches; every fixed scenario stays within budgets 3 and 19 (budget 22; unit: `perf_023_*` in zoo-render; before/after: `tools/perf/look.mjs`; counts: `tools/perf/run.sh`, `m5b.spec.ts`, `camera_views.spec.ts`). | unit / e2e |
| PERF-024 | Given the scene fragment shaders, then no derivative (`fwidth`, `dFdx`, `dFdy`) is taken inside `shade()`; `light_derivs(v_world)` is the first statement of every `main()` that calls `shade()`; night frames differ from the per-light-`fwidth` shader only on the rims of the light pools (budget 21, Q-180; unit: `perf_024_*` in zoo-render; before/after: `tools/perf/look.mjs` diff images). | unit / manual |
| PERF-016 | Given any frame (S01 by day, S10 at night), then the frame block is uploaded with exactly one `bufferSubData` to the uniform buffer, no `uniform*` call sets a member of the frame block, `getUniformLocation` is not called, and the std140 layout of `FrameBlock` matches the GLSL block and the driver (budget 20; unit: `perf_016_*` in zoo-render, the driver offsets are checked when a program links; e2e: `perf_rules.spec.ts` GL census). | unit / e2e |
| PERF-017 | Given night at the level-1 spawn and in the night zoo, when the same frame is drawn with per-draw light masks and with every light for every draw (`debug_full_light_masks`), then both pictures are pixel-identical; the light mask of a box never misses a light that reaches a point of the box; a light outside its radius is skipped before any other work (budget 21; unit: `perf_017_*` in zoo-render; e2e: `perf_rules.spec.ts`; before/after: `tools/perf/look.mjs`). | unit / e2e |
| PERF-025 | Given the view turning (first person, look-around, the zoo view's 45° glide, the night zoo), when every frame is drawn with culling and again with `debug_no_culling`, then both are pixel-identical; the culling box of an instance holds its mesh at any yaw and scale (budget 23; unit: `perf_025_*` in zoo-render; e2e: `perf_rules.spec.ts`). | unit / e2e |
| PERF-015 | Given `tools/perf/run.sh`, then it reports for every fixed scenario and viewport the frame CPU and interval p50/p95, draw calls, instances, triangles, GL calls, program switches, texture binds, uniform calls, uploads, scene / full-screen pass split, simulation step, the WASM and download sizes and the load time, and flags values > 10 % worse than the previous run (rules 1–3). | manual |

## Open questions

- Q-013 reference device (budget 1), Q-104 draw-call budget of the joined zoo (budget 3),
  Q-153 triangle budget for `buildings` (budget 10).
- Answered 2026-09-28 (user, "yes, as recommended"): Q-166 (budgets 16, 19), Q-167
  (budget 17), Q-168 (budget 18), Q-169 (budget 14), Q-170 (rule 5, PERF-R-005).
- Answered 2026-09-28 (user): Q-180 night light edges — derivatives out of the light loops
  (budget 21, PERF-R-014).
