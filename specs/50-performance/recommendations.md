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
| PERF-R-001 | Night point lights: cull per draw, cheaper loop | done (accepted 2026-09-28) | uncommitted (after `4cc87d9`) | real iGPU, phone size: night scene pass −11…−46 % (night zoo 9.9 → 5.3 ms), day ±0; picture identical |
| PERF-R-002 | Ground tiles: draw only the visible chunks, lighter path tiles | open | — | — |
| PERF-R-003 | Shared per-frame uniforms (UBO), cached locations, no redundant `set_common` | done (accepted 2026-09-28) | uncommitted (after `4cc87d9`) | GL calls/frame −53…−60 % (S01 358 → 153), `uniform*` −80…−95 %, CPU `frame()` 0.6–1.1 → 0.4–0.7 ms (SwiftShader) |
| PERF-R-004 | Zero per-frame heap allocations (zoo-core, zoo-web, zoo-render, host polling) | open | — | — |
| PERF-R-005 | Pixel-ratio quality tier for weak phones (automatic, PERF-BUDGETS rule 5) | accepted (Q-170, 2026-09-28; not implemented yet) | — | — |
| PERF-R-006 | Full-screen pass: one fewer depth fetch, sky only where needed | open | — | — |
| PERF-R-007 | Skinned animals: `eye_glow` in the same draw, same-model animals instanced | open | — | — |
| PERF-R-008 | Animation: skip clips with weight 0, cache clip indices | open | — | — |
| PERF-R-009 | Dynamic batches: upload only when changed | open | — | — |
| PERF-R-010 | Triangle budget test (APIPE-005) and the over-budget moon door | open | — | — |
| PERF-R-011 | Download: drop the palette PNG embedded in every prop `.glb`; serve compressed | open | — | — |
| PERF-R-012 | WASM size: profile before optimising | open | — | — |
| PERF-R-013 | Measurement: real-GPU runs, GPU timer queries, a WASM allocation counter | open | — | — |
| PERF-R-014 | Night light edges: derivatives once before the light loops (Q-180) | open (needs Q-180) | — | — |

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
  test PERF-022). Not implemented in this round.

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

## Acceptance criteria

- Every entry has finding, proposal, expected gain, cost/risk and a status; `done` entries
  have a commit and a measured gain from `measurements.md`.

## Test cases

No test cases of its own: each entry is verified by the PERF-BUDGETS test case it serves
(PERF-001…PERF-014) and by a before/after run in `measurements.md` (PERF-015).

## Open questions

- Q-166…Q-170 (budgets, allocation rule, quality tiers), Q-153 (buildings budget).
