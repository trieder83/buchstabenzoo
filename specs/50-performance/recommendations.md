---
id: PERF-RECOMMENDATIONS
title: Performance recommendations (tracked)
aspect: performance
module: recommendations
status: draft
depends_on: [PERF-BUDGETS, PERF-MEASUREMENTS]
test_prefix: PERFREC
updated: 2026-09-28
---

# Performance recommendations

## Goal

The tracked list of optimisations found by the `performance` agent. Each entry has the
finding with numbers (from `measurements.md`), the proposed change, the expected gain, the
cost/risk (visual or gameplay impact), a status (`open` | `accepted` | `in-progress` |
`done` | `rejected`), the commit that implemented it and the measured gain after. Entries
are never deleted; rejected ones keep the reason. The agent implements only `accepted`
entries, when asked, and measures before and after.

## Behaviour

1. Numbers come from a logged run (`measurements.md`, run date given); SwiftShader times are
   relative, counts (draw calls, GL calls, allocations, bytes) are exact.
2. A change that alters the look or gameplay needs an answered `Q-###` first (PERF-BUDGETS
   rule 4).
3. Priority = expected gain on the reference phone ÷ cost; the list is ordered by priority
   at the time an entry is added.

## Recommendations

| ID | Title | Status | Commit | Measured gain |
|---|---|---|---|---|
| PERF-R-001 | Night point lights: cull per draw, cheaper loop | done (accepted 2026-09-28) | `0e71960` | real iGPU, phone size: night scene pass −11…−46 % (night zoo 9.9 → 5.3 ms), day ±0; picture identical |
| PERF-R-002 | Ground tiles: draw only the visible chunks, lighter path tiles | (a) done (accepted 2026-09-28); (b) not taken — visible change, Q-192 | uncommitted (after `1a9dcf5`) | submitted triangles and instances: see measurements.md run 2026-09-28 (2); picture identical (L01–L14 0 px); real iGPU day frame ±5 % (within noise) |
| PERF-R-003 | Shared per-frame uniforms (UBO), cached locations, no redundant `set_common` | done (accepted 2026-09-28) | `0e71960` | GL calls/frame −53…−60 % (S01 358 → 153), `uniform*` −80…−95 %, CPU `frame()` 0.6–1.1 → 0.4–0.7 ms (SwiftShader) |
| PERF-R-004 | Zero per-frame heap allocations (zoo-core, zoo-web, zoo-render, host polling) | open | — | — |
| PERF-R-005 | Pixel-ratio quality tier for weak phones (automatic, PERF-BUDGETS rule 5) | done (accepted 2026-09-28, Q-170) | uncommitted (after `1a9dcf5`) | low tier: 720 × 1560 → 540 × 1170 (−44 % pixels) on a 1080 × 2340 phone; desktop default unchanged (0 px) |
| PERF-R-006 | Full-screen pass: one fewer depth fetch, sky only where needed | open | — | — |
| PERF-R-007 | Skinned animals: `eye_glow` in the same draw, same-model animals instanced | open | — | — |
| PERF-R-008 | Animation: skip clips with weight 0, cache clip indices | open | — | — |
| PERF-R-009 | Dynamic batches: upload only when changed | open | — | — |
| PERF-R-010 | Triangle budget test (APIPE-005) and the over-budget moon door | open | — | — |
| PERF-R-011 | Download: drop the palette PNG embedded in every prop `.glb`; serve compressed | open | — | — |
| PERF-R-012 | WASM size: profile before optimising | open | — | — |
| PERF-R-013 | Measurement: real-GPU runs, GPU timer queries, a WASM allocation counter | open | — | — |
| PERF-R-014 | Night light edges: derivatives once before the light loops (Q-180) | done (accepted 2026-09-28, Q-180) | uncommitted (after `1a9dcf5`) | real iGPU, phone size, with PERF-R-002 per-range light masks: night scene pass −10…−12 %; rim pixels only (11–752 px per night scenario) |
| PERF-R-015 | Conservative culling boxes (no popping while turning) | done (no look change; user report 2026-09-28) | uncommitted (after `1a9dcf5`) | culling = no culling in every turning frame (e2e PERF-025); +1…+8 draw calls |
| PERF-R-016 | Smooth turning: outline anti-aliasing (cause of the reported flicker) | accepted (Q-191); paused 2026-09-29 — prototype measured, not in the renderer | — | prototype `box4`: hot px −75…−84 %; cost real iGPU +0.9…+1.8 ms per frame (see entry) |
| PERF-R-017 | Frame pacing while turning | rejected (no problem found) | — | real iGPU: 241 of 241 intervals 16.7 ms, turn speed jitter 0.03 % |
| PERF-R-018 | Haze culling in the close views | done (Q-193 answered yes 2026-09-28) | uncommitted (after `267dfd9`) | first person at the level-1 spawn 44 → 35 draw calls; 1–4 outline px per frame change |

### PERF-R-001 — Night point lights: cull per draw, cheaper loop

- **Finding (run 2026-09-27):** at night the scene pass takes about twice as long as by day
  at the same spot — phone S10 2 054 ms vs. S01 1 040 ms (× 2.0), desktop 4 623 vs.
  2 023 ms (× 2.3), night zoo S11 2 917 / 3 742 ms (SwiftShader, relative); draw calls rise
  only by 2 (phone) / 5 (desktop), so the cost is per fragment. `shade()`
  (`zoo_render::night::night_glsl`) loops over all 9 point lights (`length`, `fwidth`,
  2× `smoothstep` each) and, on ground fragments, over up to 24 light pools, for **every**
  lit fragment of every opaque shader, although most fragments are far from every lamp.
  `fwidth` is also called inside a loop with a dynamic `break` (non-uniform control flow).
- **Proposal:** (a) CPU: per static batch / chunk and per skinned character, pass only the
  lights whose sphere touches its bounds (`u_light_count` per draw, lights sorted so the
  relevant ones come first — or a small per-draw index list); (b) GLSL: `if (dot(to, to) >
  r * r) continue;` before `length` and the band, and the anti-aliasing width from one
  `fwidth(world)` computed before the loop.
- **Expected gain:** night scene pass −30…50 %; by day nothing changes (early return).
- **Cost / risk:** low; look identical (same lights, same hard edge). Needs NIGHT-012 and the
  night screenshots re-checked.
- **Done 2026-09-28** (accepted by the user 2026-09-28; budget 21, tests PERF-017): (a) per
  draw a light mask `u_light_mask` (`night::light_mask`: point-light spheres / pool discs
  against the batch's chunk bounds + a margin for yawed corners, parts below the origin and
  moving parts; characters and crowds by their culling box; dynamic batches get every
  light); the loops walk the mask (uniform control flow) instead of `u_light_count`;
  (b) a fragment outside a light's radius skips the band and colour work. The `fwidth` of
  the distance stays per light before the skip — moving it out of the loop changed rim
  pixels (Q-180, PERF-R-014). **Measured** (measurements.md, run 2026-09-28): look
  scenarios L01–L14 pixel-identical; real GPU (AMD Renoir, phone size) night scene pass
  spawn −24 %, 20 m −27 %, night zoo −46 %, first person −11 %, day ±0; night / day ratio
  2.9–4.0 × → 2.3–2.5 × (budget 16 wants ≤ 1.3 ×: still open). SwiftShader shows the night
  zoo −17…−33 % but day / night first person +6…14 % (predicated branches; not on the real
  GPU). Expected −30…50 % — reached in the lamp-rich views, less where the lights cover most
  of the screen.

### PERF-R-002 — Ground tiles: draw only the visible chunks, lighter path tiles

- **Finding (run 2026-09-27, `debug_draw_list`):** the ground is ≈ 85–90 % of all triangles
  submitted. Desktop S01 (level-1 spawn, 14 m): 228 k triangles, of which `path_edge`
  344 × 158 = 54 k, `grass_tile` 1 632 × 28 = 46 k, `path_tile_t` 270 × 164 = 44 k,
  `path_tile_cross` 132 × 198 = 26 k, `plaza_tile` 15 k; S09 (level-3 spawn, 20 m): 453 k
  triangles (phone 304 k), 7.5 k instances, again mostly grass / path tiles of three
  regions. Chunk culling (8 m) only decides **whether** a batch is drawn; a drawn batch
  always draws **all** its instances (WebGL2 has no base instance), so the whole level's
  1 632 grass tiles are drawn although the 14 m view shows ≈ 15 × 25 m. Path tiles carry
  158–198 triangles per 1 m tile (curb and edge details).
- **Proposal:** (a) bake the never-moving ground (grass, path, plaza, edge, water-bank
  tiles without animation) into one mesh per 8 m chunk and material program (ARCH-008
  static batching, culled per chunk) — or sort each tile batch's instances by chunk and
  issue one instanced draw per visible chunk range with the instance attribute offset
  re-pointed (one VAO per chunk); (b) re-model `path_edge` / `path_tile_*` with fewer
  triangles where the detail is invisible from the 55° camera (ART, tools/blender).
- **Expected gain:** −60…80 % of the submitted triangles and vertex work in every
  scenario (vertex shading, clipping and binning on tile-based phone GPUs); draw calls
  +0…+10 for (a) depending on the variant (stays far below Q-104's 140).
- **Cost / risk:** medium (renderer + scene code; the chunked bake must keep the
  per-region hiding of barriers and the edge mask of ground tiles = 0); (b) needs art
  review. No visible change intended.
- **Done 2026-09-28, part (a)** (accepted by the user 2026-09-28; budget 22, tests PERF-023):
  the second variant — every static batch with instances in more than one 8 m chunk is
  uploaded sorted by chunk (rows south → north, so the default north-looking camera draws
  front to back; instance handles keep their indices through a GPU-order table), each chunk
  one contiguous range with its own VAO (WebGL2 has no base instance); per frame the visible
  ranges form runs (`plan_chunk_runs`: off-screen gaps ≤ 16 384 triangles drawn along, at most
  3 draws per batch — a first setting of 4 096 / 6 split the close views into more draws
  than the zoo view, CAMV-014), each run with its own light mask (union of its chunks). **Measured:**
  look scenarios L01–L14 pixel-identical by day (0 px); draw calls +1…+8; submitted
  triangles / instances: measurements.md run 2026-09-28 (2). Real iGPU (both page orders):
  day scene pass phone −6…+1 %, desktop +4…+5 % (noise ±5 %) — the iGPU is not
  vertex-bound, the saving is for tile-based phone GPUs (binning) and is not measurable
  here. A first row order (north first = back to front) cost up to +19 % in first person
  through lost early depth rejection; fixed by the south-first order.
- **Part (b) not taken:** the bottom faces are already removed by the kit
  (`delete_down_faces`); the only further saving (6-sided cobbles, plain edging stones:
  path tiles −21 %, `path_edge` −49 % triangles) is **visible** in the 55° zoo view
  (square edging bricks, angular cobbles, a few new outline specks; review shots
  `specs/50-performance/review/r002b_*`). The tiles stay as they are; Q-192.

### PERF-R-003 — Shared per-frame uniforms (UBO), cached locations, no redundant `set_common`

- **Finding:** per frame the renderer issues ≈ 280–385 WebGL calls (phone 283–330, desktop
  320–384), of which 175–232 are `uniform*` calls (≈ 60 %) and 8–9 `useProgram` for 6
  programs.
  `Renderer::set_common` re-sends ≈ 17 uniforms including the light arrays (2 × 9 + 24
  `vec4`) up to 7 times per frame; the static program gets `set_common` twice (before the pass
  loop and again for pass 0); every uniform location is looked up in a
  `HashMap<&str, _>` (SipHash of the name) per call.
- **Proposal:** one std140 uniform block (`u_frame`: view, view-proj, sun, shadow tint, fade,
  dither, night, lights, pools, glow) updated once per frame with `bufferSubData` and bound
  to every program; locations of per-draw uniforms cached in struct fields; drop the first,
  redundant `set_common(static_prog)`.
- **Expected gain:** −100…150 GL calls per frame; −0.2…0.5 ms CPU per frame on mid-range
  phones (each WebGL call costs a WASM→JS→GPU-process round trip plus validation).
- **Cost / risk:** low–medium (std140 padding); no visual change; covered by the existing
  screenshot e2e tests.
- **Done 2026-09-28** (accepted by the user 2026-09-28; budget 20, tests PERF-016): std140
  block `Frame` (`shaders::frame_block`, CPU mirror `renderer::FrameBlock`, offsets
  unit-tested and checked against the driver when a program links) on binding point 0 for
  the static, rich, water, skinned, crowd and decal programs, one `bufferSubData` per
  frame; `Program` keeps a location array indexed by `U` and a last-value cache (a per-draw
  uniform call only on change); samplers, outline colour, the crowd model matrix and other
  constants set once after linking; the duplicate `set_common(static)` removed.
  **Measured:** GL calls per frame desktop 320–384 → 143–192, phone 253–332 → 104–147
  (−53…−60 %); `uniform*` 159–232 → 9–43; `useProgram` 8–9 → 7–8; CPU `frame()` p50
  0.6–1.1 → 0.4–0.7 ms (SwiftShader); real GPU day frame unchanged (the saving is CPU /
  driver time, larger on phones with slow WebGL validation). Expected −100…150 calls:
  reached (−176…−205).

### PERF-R-004 — Zero per-frame heap allocations

- **Finding:** native probe — `zoo_core::game::Game::update` makes **27.2 heap allocations
  (7.3 KB) per frame** in the joined zoo; sites: `Game::interactables()` (≈ 23 per frame: it
  rebuilds a `Vec<Interactable>` with `String` clones of cut spots, plants, garden signs and
  moon doors, and is called by `update_panel` → `available_target`, by `target_distance`,
  and once more per frame by the host through `target_key` / `target_kind`),
  `update_wander` (`collect` of a `Vec<bool>`), `night::moon_doors` and
  `night_zoo_waiting` (`String`s). Review of the frame loop: zoo-render
  `Renderer::render` collects `visible: Vec<bool>` per frame; zoo-web
  `set_dynamic_instances` allocates `name.to_owned()` for the batch lookup (≈ 9 per frame),
  `building_inside` clones a `String` per frame; the host calls ≈ 8 string-returning WASM
  functions per frame (`target_key`, `carry_food`, `carry_bowl`, `basket_json`,
  `poll_events` → `format!("[]")`, `take_save`, `saved_view_mode`, `daytime`) and parses
  `poll_events` with `JSON.parse` every frame. `Ambient` makes 0 allocations.
- **Proposal:** keep the interactables in a scratch `Vec` owned by `Game` (rebuilt only when
  its inputs change) and look up the nearest without collecting; ids as `&'static str` or
  indices instead of `String` clones; the `visible` region list as a reused field; batch
  lookup by a precomputed index; the host polls strings only when a `ui_dirty` counter
  changed. Add the counting-allocator unit test (PERF-013) once Q-169 is answered.
- **Expected gain:** a few µs of CPU and no allocator/GC jitter (p95 frame spikes); the rule
  becomes testable.
- **Cost / risk:** low; zoo-core refactor touches the interaction code (PLAY/FEED tests
  cover it).

### PERF-R-005 — Pixel-ratio quality tier for weak phones

- **Finding:** everything that dominates the SwiftShader frame is per pixel (scene pass
  fragments, the full-screen pass); on a 1080 × 2340 phone the renderer draws 720 × 1560
  (pixel ratio capped at 2). The full-screen pass alone is ≈ 20–25 % of the frame (phone S01
  239 of ≈ 1 280 ms, S02 464 ms).
- **Proposal:** an automatic "low" tier (Q-170): pixel ratio 1.5 when p95 > 33 ms for 3 s
  (−44 % pixels), then fewer point lights; never a menu.
- **Expected gain:** up to −40 % of all fragment work on weak phones.
- **Cost / risk:** visible (softer image, outline width in device px); needs the user's
  answer to Q-170.
- **Status 2026-09-28:** accepted — Q-170 answered "yes, as recommended": automatic, pixel
  ratio first, lights and clouds only if still too slow, never a menu (PERF-BUDGETS rule 5,
  test PERF-022).
- **Done 2026-09-28:** `zoo_core::quality::QualityGovernor` (pure, unit-tested): 3 s windows
  of the real frame intervals, a window with > 5 % frames slower than 1/30 s (p95 > 33 ms)
  steps down one tier — `low1` pixel ratio 1.5 (outline offset 3 device px = still 2 CSS
  px), then `low` (+ lantern and 4 lamps as point lights, the rest light pools, no clouds in
  the close-view sky); never back up within a session (no flicker); 5 s warm-up, 2 s settle
  after a switch, intervals > 1 s ignored. Host: `?quality=auto|high|low1|low`; automated
  browsers start `high`. e2e `quality.spec.ts` (slow frames forced with a 45 ms busy wait:
  high → low1 → low, canvas 720 × 1560 → 540 × 1170, night lights 9 → ≤ 5). **Measured:**
  the pixel count of the low tier is −44 %; the frame-time gain on a weak phone needs the
  reference phone (Q-013). By default (desktop, fast phones) nothing changes: 0 px in all
  look scenarios.

### PERF-R-006 — Full-screen pass: one fewer depth fetch, sky only where needed

- **Finding:** the outline pass (`shaders::post_fs`) reads 10 texels per pixel (5 depth
  incl. the centre twice — once in the Laplacian, once for `atmosphere` — 3 normal,
  1 colour) and in the close views computes the sky (`asin`, `atan`, `pow`, 3 cloud SDFs,
  `fwidth`) for every pixel, even where the haze is 0.
- **Proposal:** pass the centre depth into `atmosphere`; compute `sky_color` only for
  background pixels and fog > 0.85 (keep `fwidth` in uniform control flow by computing the
  derivatives first).
- **Expected gain:** −10…15 % of the full-screen pass in the close views, −1 texture fetch
  per pixel everywhere.
- **Cost / risk:** low; the sky must look identical (CAMV sky tests, screenshots).

### PERF-R-007 — Skinned animals: `eye_glow` in the same draw, same-model animals instanced

- **Finding:** every visible skinned animal is posed, uploads its joint texture
  (`texSubImage2D`) and costs **one draw call per material part** (`body` + `eye_glow` =
  2 draws, `Renderer::render` characters loop), with a texture bind and 2–3 uniforms per part.
- **Proposal:** mark `eye_glow` vertices with a per-vertex mode (as the static slots do) and
  draw the animal in one call; animals of the same model share the instanced crowd path
  (one draw per model).
- **Expected gain:** −1 draw call per visible animal (night: every night animal), fewer
  uploads.
- **Cost / risk:** low; eyeshine tests NIGHT-006 / AANI-010 cover the look.

### PERF-R-008 — Animation: skip clips with weight 0, cache clip indices

- **Finding:** native probe — one pose (sample `idle` + `walk`, blend, joint matrices) costs
  3–17 µs per model (elephant 17.2 µs, 26 joints); one pose of each of the 24 skinned models
  = 276 µs native (≈ 0.4–0.6 ms in WASM). `pose_character` samples both clips every frame,
  also when `walk_blend` is 0 or 1, and finds clips by string compare each frame.
- **Proposal:** sample only clips with weight > 0; resolve clip names to indices once per
  animal/state change.
- **Expected gain:** ≈ −45 % animation CPU for standing or steadily walking animals.
- **Cost / risk:** none (identical poses).

### PERF-R-009 — Dynamic batches: upload only when changed

- **Finding:** 7 `bufferSubData` per frame in every scenario (≈ 2.8–5.7 KB) —
  `set_dynamic_instances` marks its batch dirty unconditionally, also when the instances are
  unchanged or empty (carry box, bowl, bowl water, capsule/marker, placeholder boxes,
  butterflies, glow boxes, hand lantern).
- **Proposal:** compare with the previous instances (as `set_instance` does) and skip empty
  uploads.
- **Expected gain:** −5…7 GL calls per frame; tiny.
- **Cost / risk:** none.

### PERF-R-010 — Triangle budget test (APIPE-005) and the over-budget moon door

- **Finding:** no automated test enforces APIPE-005. Over budget today: `moon_door.glb` and
  `moon_door_open.glb` 1 430 triangles each (kind `props`, budget 500); `night_house.glb`
  3 196 triangles (kind `buildings`, no budget yet — Q-153). All animals and the player are
  within 3 000 (max: hedgehog 2 759).
- **Proposal:** add the asset test (PERF-010) with the moon door as a documented exception or
  reclassified as a building once Q-153 sets a `buildings` budget.
- **Expected gain:** keeps triangle counts from creeping up; the moon door is drawn at most
  twice, so no frame-time gain today.
- **Cost / risk:** none for code; art only if the door must be reduced.

### PERF-R-011 — Download: drop the embedded palette PNG; serve compressed

- **Finding:** web/dist 8.36 MB (6.61 MB fetched on first load = 22 % of the 30 MB budget),
  2.46 MB with brotli. Every static `.glb` embeds a 256 × 256 palette PNG (2–4 KB) although
  the renderer uses the shared `textures/palette.png`; the animal files (70–265 KB) are
  mostly clip data.
- **Proposal:** export props without the embedded image (the loader keeps UVs); make the web
  host send brotli/gzip (Q-167); later, load a level's models when it unlocks.
- **Expected gain:** ≈ −0.3 MB raw; ≈ −70 % transfer with compression.
- **Cost / risk:** low; export-script change (tools/blender) and asset tests.

### PERF-R-012 — WASM size: profile before optimising

- **Finding:** release WASM 1 683 KB (640 KB gzip, 485 KB brotli) after wasm-opt `-Os`; an
  extra `-Oz` pass saves only 2.5 % raw / 0.3 % brotli, `-O3` grows it; the dev build is
  11 973 KB (the Vite dev server serves this one). Contents unknown (`twiggy` not installed).
- **Proposal:** run `twiggy top` / `twiggy dominators` on the rustc output with names; likely
  candidates are `fluent-bundle` + `unic-langid`, `gltf-json` + `serde_json`, `toml`, `png`.
  Only act if Q-167 sets a budget below the current size.
- **Expected gain:** unknown; low priority (well within any likely budget).
- **Cost / risk:** none for profiling.

### PERF-R-013 — Measurement: real-GPU runs, GPU timer queries, a WASM allocation counter

- **Finding:** the 2026-09-27 baseline ran on a machine with load average 35–55 on 12 CPUs;
  SwiftShader frame times (1–4 s) are not usable in absolute terms, and `gl.finish()` does not
  block in Chrome (the tool fences with a 1-pixel `readPixels`).
- **Proposal:** run `PERF_ANGLE=gl tools/perf/run.sh` on a machine with a GPU and once on the
  reference phone (Q-013, remote debugging); a debug-only `EXT_disjoint_timer_query_webgl2`
  per-pass timer and a counting global allocator in the WASM dev build exposed via
  `window.__zoo`.
- **Expected gain:** absolute numbers for budget 1 and Q-166.
- **Cost / risk:** debug code only.

### PERF-R-014 — Night light edges: derivatives once before the light loops (Q-180)

- **Finding (while implementing PERF-R-001, 2026-09-28):** `shade()` takes the edge width
  `fwidth(distance)` per light inside the light loops, and for the light pools inside the
  non-uniform branch `n.y > 0.6 && world.y < 0.5`. Derivatives in non-uniform control flow
  are undefined in GLSL ES; under SwiftShader they leave a few unlit dark-blue specks on the
  rims of the lamp pools (0.04–0.14 % of the pixels of a night frame, `tools/perf/look.mjs`
  L08–L14). To stay pixel-identical, PERF-R-001 keeps the per-light `fwidth` and only skips
  the band and colour work after it.
- **Proposal:** take `dFdx(world)` / `dFdy(world)` once before the loops (uniform control
  flow) and compute the edge width from the neighbour distances; skip a light by
  `dot(to, to) ≥ r²` before any other work (variant "B", measured).
- **Expected gain:** measured (measurements.md, run 2026-09-28, variant B vs. the shipped
  PERF-R-001, real GPU at phone size): night scene pass −2…−4 %; SwiftShader ±0. Mainly a
  correctness fix: defined derivatives on real GPUs (mobile drivers may show the specks
  more).
- **Cost / risk:** the rim pixels of the light pools change (the specks disappear; 49 of
  518 400 pixels by more than 60 / 255 in the worst look scenario, the edge position is
  unchanged) — needs the user's answer to Q-180.
- **Done 2026-09-28** (Q-180 answered yes; budget 21, test PERF-024): `light_derivs(v_world)`
  is the first statement of the scene fragment shaders and takes `dFdx` / `dFdy` of the
  world position once, under the uniform condition `u_night.x > 0`; a light is skipped by
  `dot(to, to) ≥ r²` before any work and its edge width is `(|to·dx| + |to·dy|) / d`; at dusk
  (night 0) `shade()` returns before the loops. **Measured:** the pixel diff against the
  shipped build is limited to the light rims (L08 314, L09 752, L10 279, L11 184, L12 11,
  L13 669, L14 271 px; day and dusk 0 px; diff images `specs/50-performance/review/r014_*`).
  Taking the derivatives unconditionally cost ≈ 11 % of the day scene pass on the iGPU
  (A/B with a shader patch) — hence the uniform guard. Night scene pass (real iGPU, phone
  size, geometric mean of both page orders, together with the per-range light masks of
  PERF-R-002): spawn −12 %, 20 m −11 %, night zoo −10 %, first person −11 %.

### PERF-R-015 — Conservative culling boxes (no popping while turning)

- **Finding (user report 2026-09-28, "looking left or right is slightly flickery"):** the
  culling boxes of regions and chunks were `pos ± radius` from 0.1 m below the origin: a
  mesh yawed by a non-quarter turn reaches `radius × √2`, parts below the origin and
  stretched instances (string lights, up to 6 m) reach beyond — such an instance could be
  culled while a corner was already in view and pop in at the screen edge while turning.
  The per-chunk ranges of PERF-R-002 cull more finely, so this had to be exact.
- **Change (no look change):** `instance_extent` — per instance `±radius` for quarter
  turns, the mesh's largest horizontal vertex distance otherwise (a round canopy keeps its
  circle), the mesh's lowest point; `static_reach` adds the space swept by turning parts
  (each vertex's circle about its part axis — a flat 1 m margin, then a sphere bound, each
  added draws for the turnstile and the moon door and made CAMV-014 worse); bobbing props
  get their drift + amplitude; stretched or re-yawed instances grow the batch / region
  margin. A debug
  switch `debug_no_culling` and the e2e PERF-025 compare every frame of four turning
  sequences with and without culling.
- **Measured:** 0 differing pixels in all turning frames (e2e PERF-025); look scenarios
  0 px; draw calls at the CAMV-014 spots equal to the committed renderer (43 / 44 / 43 at
  the spawn, 41 / 36 / 37, 48 / 35 / 41). A first try that reused the light-mask margin cost
  +10…+23 draw calls and was replaced.

### PERF-R-018 — Haze culling in the close views

- **Finding (2026-09-28, while checking CAMV-014):** with the level data of 2026-09-28,
  first person at the level-1 spawn draws 44 calls against 43 in the zoo view at 20 m
  (CAMV-014 / PERF-004 red, with the committed renderer too): the view reaches the pond and
  river at the edge of the haze (water tiles, lily pads, reeds, jetty, ducks). Beyond the
  fog end (20.8 m) every pixel is exactly the sky colour.
- **Proposal:** skip boxes whose nearest point is beyond the fog end (+ 5 cm) in the full
  close views (`Cull::haze`, implemented as `Renderer::haze_cull`, off).
- **Measured:** first person 44 → 35, look-around 43 → 34 at the spawn (36 → 29, 35 → 30 at
  the other CAMV-014 spots); CAMV-014 green. PERF-025 (culling vs. no culling while turning):
  1–4 pixels per frame differ in first person — outlines of nearer silhouettes that sample
  the culled geometry behind them.
- **Cost / risk:** a few outline pixels inside the haze change; needs the user's answer to
  Q-193.
- **Done 2026-09-29** (Q-193 answered yes): `Renderer::haze_cull` defaults to on. The debug
  switch `debug_no_culling` drops only the frustum / chunk culling (`Cull::all`); PERF-025
  switches the haze culling off in both frames (`debug_haze_cull(false)`, new) — with it on,
  first person differed by 3 px in one frame (a haze-culled chunk range drawn along as a gap
  of a merged run in one frame only). Budget 23 names the exception. CAMV-014 green.

### PERF-R-016 — Smooth turning: outline anti-aliasing (the cause of the flicker)

- **Finding (`tools/perf/turn.mjs flicker`, run 2026-09-28):** four slow turns (first person,
  look-around, the zoo view's 45° glide, first person in the night zoo) rendered frame by
  frame at 960 × 540 and at 2 × 2 supersampling. The per-pixel temporal variation of the
  normal render exceeds the supersampled reference by 6.9–11.5 % of all change; the heat
  maps put it on the outlines of dense small detail — cobble and plaza stones, wall stones,
  fence rails, string-light cords — at a distance (moiré and crawling lines). With the
  outline pass drawing no lines (shader patch) the strongly flickering pixels fall from
  1 393 / 13 924 / 12 843 / 3 261 to 41 / 1 735 / 321 / 389 (−87…−97 %). The renderer has no
  anti-aliasing (`antialias: false`, hard 2 px lines, no MSAA on the G-buffer).
- **Proposal (changes the look slightly, Q-191):** (a) anti-aliased lines in the outline
  pass (soft 1 px fringe from sub-pixel depth / normal samples, no extra pass); (b) an FXAA
  pass; (c) line level of detail for small depth steps at a distance.
- **Expected gain:** the aliasing excess towards the supersampled reference (TV × 1.05–1.09
  → ≈ × 1.0), most of the hot pixels; cost (a) ≈ +8 texture reads per pixel in the
  full-screen pass (budget 16: ≤ 3 ms), (b) +1 full-screen pass.
- **Cost / risk:** softer line edges; needs the user's answer to Q-191 and a review.
- **Status 2026-09-29: accepted (Q-191: yes, anti-aliased outlines), paused by the user**
  before it reached the renderer — the committed renderer draws the outlines as before.
  Prototypes as shader patches in `tools/perf/outline_aa_variants.mjs` (turn.mjs
  `TURN_VARIANTS_FILE`). Flicker (`turn.mjs flicker`, 48 frames, SwiftShader, hot px vs the
  2 × 2 supersampled current look; base T01 / T02 / T03 / T04 = 1 393 / 13 402 / 12 678 /
  3 254):
  | Variant | Edge evaluations | T01 | T02 | T03 | T04 |
  |---|---|---|---|---|---|
  | `box4` (mean of the 2 × 2 block) | 4 | 355 | 3 346 | 1 992 | 651 |
  | `tent5` (½ centre + ⅛ × 4 neighbours) | 5 | 348 | 2 911 | 1 167 | 354 |
  | `tri3` | 3 | 404 | 4 034 | 2 342 | 623 |
  | `diag2` | 2 | 506 | 5 071 | 5 083 | 1 076 |
  | `box4d` / `box4n` (depth / normal edges only) | 4 | 1 263 / 1 181 | 8 898 / 6 336 | 11 607 / 2 351 | 1 807 / 1 597 |
  `box4` brings the TV per pixel and frame below the supersampled reference (× 0.95–0.98,
  was × 1.05–1.09); both edge kinds must be filtered. **Cost** (real iGPU AMD Renoir, `look.mjs
  ab`, 16 rounds): `box4` in the one outline pass +0.9…+1.8 ms per frame (phone size
  720 × 1560: full-screen pass 1.5 → 2.8 ms at L01 — inside budget 16's 3 ms on this iGPU,
  a real phone is slower), `tent5` ≈ +0.4 ms more. A two-pass form (R8 edge mask, one
  bilinear fetch = the same `box4` picture, 352 / 3 356 / 1 996 / 653 hot px) cost more:
  +1.4…+2.1 ms per frame (extra render-target pass) — dropped.
- **Next (resume):** (1) review shots of `box4` vs today in L01 / L02 / L04 / L09 into
  `specs/50-performance/review/` (lines 0.5 / 1 / 0.5 across 3 px, shifted ½ px — bold lines
  stay bold, check they do not look blurry); (2) make `box4` cheaper before shipping — share
  the normal taps, `texelFetch` with integer offsets, skip the 3 extra evaluations where the
  centre's 5 depth taps and 3 normal taps are all flat; target ≤ +0.5 ms on the iGPU phone
  size; (3) implement in `post_fs` with a PERF test (turn flicker metric, PERF-026 draft) and
  look.mjs L01–L14 review.

### PERF-R-017 — Frame pacing while turning

- **Finding (`tools/perf/turn.mjs pacing`, 2026-09-28):** real iGPU (AMD Renoir, ANGLE GL),
  desktop and phone viewport, → held in first person for 4 s: 241 intervals, p50 = p95 =
  16.7 ms, max 16.8 ms, 0 long frames, turn speed 90.0°/s with 0.03 % jitter; the zoo
  view's 45° steps ease smoothly (exponential, dt-based). The camera turn is already
  eased with the real `dt` (rAF timestamps). SwiftShader: 0.6–1.6 s per frame (not
  representative); there the `dt` clamp of 0.1 s slows turning, which only matters below
  10 fps.
- **Status:** rejected — no pacing problem found; per-frame allocations and host string
  polling (possible GC hitches on phones) stay in PERF-R-004.

### PERF-R-019 — Headroom watch: WASM size, night draw calls, per-frame bytes in `Game::update`

- **Finding (run 2026-09-30):** all budgets pass, but the margins are small: release WASM
  1 779 KiB raw of 2 048 (−89 KiB left) and 514 KiB brotli of 600 (gzip 680 KiB — the
  budget is met only on servers that send brotli); night adds +5 of +6 draw calls (S10 35
  vs. S01 30); `Game::update` 28 allocations / 14.4 KB per frame (was 7.3 KB — the
  families / babies state; the count is the known budget-14 exception); S09 triangles
  147 k (+41 % against the prototype-state run, limit 300 k).
- **Proposal:** before level 4 / audio: (a) `twiggy` on the WASM for the biggest items
  (`-Oz` saves only 42 KiB), (b) find the per-frame allocation sites of `Game::update`
  (`alloc_sites_10_frames` of the probe) and reuse buffers, (c) bake the night props
  into fewer batches to regain draw-call margin.
- **Expected gain:** a few 10 KiB of WASM; fewer GC hitches on phones; 1–3 draw calls at night.
- **Cost/risk:** none for the look.
- **Update (run 2026-09-30 (2)):** WASM 1 832 KiB raw (216 KiB left) / 526 KiB brotli (74 left); night +5 of +6; `Game::update` 29.1 allocs / 14.4 KB. Still open.
- **Status:** open (watch item, not a budget break).

### PERF-R-020 — JS heap 61 - 64 MB in the run 2026-09-30 (2) (budget 18: <= 32 MB)

- **Finding:** `performance.memory.usedJSHeapSize` after the scenarios was 61 - 64 MB at
  the desktop / phone viewports (10.7 MB in the run before; desktop_half 33 MB in the same
  run). Reading is coarse and taken at machine load 25, so possibly GC timing; the new
  suspects are `audio.ts` (decoded buffers only after a gesture: none in the perf run),
  the signed-ad loader (`@noble/ed25519`, small) and the DOM (look stick, intro).
- **Proposal:** re-measure on a quiet machine with a forced GC (`--js-flags=--expose-gc`,
  `gc()` before reading); if still > 32 MB take a heap snapshot.
- **Update 2026-10-02:** 29.8 MB desktop, 40.2 MB phone / desktop_half (was 61 - 64), still
  at load 25 - 37 and without forced GC; the two equal 40.15 MB values look like a GC plateau.
- **Status:** open (unverified; needs a quiet machine + `gc()`).

### PERF-R-021 — Draw-call creep from pairs / characters (watch item, run 2026-10-02)

- **Finding:** +2 draw calls (+5 %) in animal-heavy scenes after all animals became pairs
  (S01 31 -> 33, S02 38 -> 40, S10 36 -> 38; level 3 unchanged). Max is 40 of 140, night
  margin to the +6 rule is 1. Each skinned character is its own draw (no batching).
- **Proposal:** if scenes with 10+ visible animals appear, instance animals per model
  (GPU skinning with per-instance bone offsets / texture) or cull members beyond the
  camera range; otherwise do nothing. Re-check the night +6 rule when new night props land.
- **Expected gain:** up to ~1 draw per extra visible animal. **Cost/risk:** skinning rewrite.
- **Status:** open (watch only).

## Acceptance criteria

- Every entry has finding, proposal, expected gain, cost/risk and a status; `done` entries
  have a commit and a measured gain from `measurements.md`.

## Test cases

No test cases of its own: each entry is verified by the PERF-BUDGETS test case it serves
(PERF-001…PERF-014) and by a before/after run in `measurements.md` (PERF-015).

## Open questions

- Q-166…Q-170 (budgets, allocation rule, quality tiers), Q-153 (buildings budget).
