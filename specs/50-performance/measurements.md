---
id: PERF-MEASUREMENTS
title: Performance measurement log
aspect: performance
module: measurements
status: draft
depends_on: [PERF-BUDGETS]
test_prefix: PERFLOG
updated: 2026-09-28
---

# Performance measurement log

Append-only: one section per run, newest at the bottom. Never edit an old run; corrections
go into the next run's notes. The log has no test cases of its own: the tool that produces
it is PERF-015 (PERF-BUDGETS), the budgets it checks are PERF-001…PERF-014.

## How to measure (one command)

```bash
tools/perf/run.sh                       # snapshot + builds + native probe + browser scenarios
tools/perf/run.sh --skip-build          # re-run probe + browser on the last snapshot
tools/perf/run.sh --only-sizes          # sizes only (after the builds)
PERF_ANGLE=gl tools/perf/run.sh         # real GPU instead of SwiftShader (if the machine has one)
PERF_VIEWPORTS=phone PERF_SCENARIOS=S01,S10 tools/perf/run.sh --skip-build   # a subset
node tools/perf/look.mjs capture <web/dist> <out>   # look scenarios L01–L14 (deterministic)
node tools/perf/look.mjs compare <out A> <out B> 0   # pixel diff ("no visible change")
LOOK_ANGLE=gl node tools/perf/look.mjs ab <dist A> <dist B>   # interleaved frame-time A/B
```

- **"No visible change" optimisations** are checked with `tools/perf/look.mjs`: capture the
  look scenarios of the build before and after and `compare` them (0 differing pixels
  expected); time them with `ab`, which alternates fenced frames of both builds in one
  browser (robust against load drift; on the real GPU compare the phone viewport or both
  page orders — the desktop viewport has a page-order bias there).

- `tools/perf/run.sh` copies the working tree (committed **and** uncommitted) to
  `$PERF_WORK/tree` (default `~/.cache/buchstabenzoo-perf`), builds `wasm-pack --dev` and
  `--release` and `vite build` there (never touching `crates/zoo-web/pkg`, `web/dist` or a
  running dev server), runs the native probe `tools/perf/probe` and the Playwright scenarios
  `web/tests/e2e/perf/scenarios.perf.ts` against its own `vite preview` on port 4190 (stopped
  afterwards), and prints the Markdown tables below. Results: `$PERF_WORK/out/results.json`;
  values > 10 % worse than the newest `tools/perf/baselines/*.json` are printed in **bold**.
  After logging a run, copy `results.json` to `tools/perf/baselines/<date>.json`.
- **Viewports:** `desktop` 1920 × 1080 (DPR 1); `phone` 1080 × 2340 device px portrait
  (CSS 360 × 780 at DPR 3 — the renderer caps the pixel ratio at 2, so it draws
  720 × 1560); `desktop_half` 960 × 540 (pixel-scaling check, S01/S02/S10 only).
- **Fixed scenarios** (stable ids; add new ones, never change old ones; seed 17, fresh save,
  reading level `klasse1`):

  | ID | Scenario |
  |---|---|
  | S01 | level-1 spawn, zoo view, default zoom 14 m |
  | S02 | level-1 spawn, zoo view, max zoom-out 20 m |
  | S03 | walking (autopilot) from the level-1 spawn north along the ring path |
  | S04 | look-around on the level-1 ring path (−2.5, 20.5) looking north |
  | S05 | first person at the same spot |
  | S06 | inside `zookeeper_house_1` (roof hidden), zoo view |
  | S07 | vegetable garden `garden_veg` |
  | S08 | pond with ducks (water animation + ambient on; A/B: water off, ambient off) |
  | S09 | level-3 spawn at 20 m (the joined zoo's worst case; levels still locked) |
  | S10 | night at the level-1 spawn (lamps, glow, lantern) |
  | S11 | night zoo `night_1` behind the moon door (−34.5, 29.5) |

- **Columns:** draw calls / instances / triangles = `RenderStats` of the last frame
  (triangles count every instance of a drawn batch); *CPU `frame()`* = JS time of
  `app.frame` (simulation + GL command submission); *CPU+GPU fenced* = the same frame
  followed by a 1-pixel `readPixels` (blocks until the frame is drawn; `gl.finish()` does
  not block in Chrome); *scene / full-screen pass* = time to the full-screen
  `drawArrays(TRIANGLES, 0, 3)` and of that pass, both fenced; *sim step* = one 1/60 s
  `debug_step` (game + animals + ambient, no rendering); GL calls, `useProgram`,
  `bindTexture`, `uniform*` calls and uploaded bytes per frame from a WebGL2 call census.
- **SwiftShader** (headless Chromium, software WebGL): times are **relative** only and depend
  on the machine's load (logged per viewport); counts are exact. Record real-GPU / phone
  runs separately.

## Run 2026-09-27 — baseline

- **Commit** `966c8db` **plus uncommitted work in progress** of another agent (dirty tree:
  bamboo / dropping items / water wheel / level gates in `crates/zoo-core`, `zoo-web`,
  `assets/levels`; `git status` listed 25 modified and 17 untracked paths). Snapshot taken
  ≈ 21:31 (runs 2, 4, 5 of the tool on the same snapshot: builds + probe, phone +
  desktop_half, desktop; the census counts of that run were corrected by ½ — the census
  counted 6 frames per 3, fixed in the spec since). Raw data:
  `tools/perf/baselines/2026-09-27.json` (with the static draw lists of S01/S09).
- **What changed since the last run:** first run (no previous run).
- **Machine:** AMD Ryzen 5 5500U (12 threads), Linux 6.8; rustc 1.98.1, wasm-pack 0.15.0;
  Chrome headless shell 153, ANGLE Vulkan → **SwiftShader**. **The machine was heavily
  loaded (load average 23–58 on 12 CPUs: other agents' test runs and Blender)** — the frame
  times below are far slower than normal (the last integration round saw ≈ 340 ms at the
  spawn) and noisy (p95 outliers of 20–85 s are load spikes): use only the p50 ratios
  between scenarios of the same viewport; counts and sizes are exact.

### Sizes and load

| Artefact | raw | gzip | brotli |
|---|---|---|---|
| WASM dev (`--dev`, what the Vite dev server serves) | 11 973 KB | 1 883 KB | 1 178 KB |
| WASM release, rustc output (opt-level "s", LTO) | 3 682 KB | 856 KB | 621 KB |
| WASM release after wasm-bindgen + wasm-opt `-Os` (shipped) | **1 683 KB** | 640 KB | 485 KB |
| … + extra wasm-opt `-Oz` (estimate) | 1 641 KB | 640 KB | 483 KB |
| … + extra wasm-opt `-O3` (estimate) | 1 705 KB | 641 KB | 483 KB |
| JS glue `zoo_web.js` | 78 KB | 14 KB | 12 KB |
| **web/dist total** (183 files) | **8.36 MB** | 3.14 MB | 2.46 MB |
| · assets/models (131) | 6.22 MB | 2.23 MB | 1.71 MB |
| · bundle (JS + WASM) | 1.69 MB | 0.64 MB | 0.49 MB |
| · assets/textures (36) | 0.23 MB | 0.21 MB | 0.21 MB |
| · assets/levels (4) + i18n (8) | 0.20 MB | 0.06 MB | 0.05 MB |

| Viewport | canvas px | first frame (wall, local, busy machine) | resources | transferred (no HTTP compression) | WASM heap | JS heap |
|---|---|---|---|---|---|---|
| desktop | 1920 × 1080 | 3.9 s | 136 | **6.61 MB** (22 % of 30 MB) | 17.2 MB | 9.5 MB |
| phone | 720 × 1560 | 4.6 s | 136 | 6.61 MB | 17.1 MB | 10.1 MB |
| desktop_half | 960 × 540 | 5.5 s | 136 | 6.61 MB | 17.2 MB | 10.1 MB |

### Scenarios — desktop 1920 × 1080 (load average 46 → 33)

| Scenario | draw calls | inst. | tris | CPU `frame()` p50 ms | CPU+GPU fenced p50 ms | scene / full-screen pass ms | sim step ms | GL calls | programs | tex binds | uniforms | upload KB | lights / pools |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| S01 spawn 14 m | 30 | 2 709 | 228 k | 5.3 | 3 589 | 2 023 / 515 | 0.48 | 358 | 9 | 13 | 220 | 4.2 | 0 / 0 |
| S02 spawn 20 m | 37 | 2 800 | 251 k | 1.4 | 2 640 | 1 775 / 517 | 0.24 | 384 | 9 | 14 | 232 | 4.2 | 0 / 0 |
| S03 walking | 34 | 2 768 | 243 k | 1.1 | 2 597 | 1 678 / 460 | 0.36 | 351 | 9 | 10 | 216 | 2.8 | 0 / 0 |
| S04 look-around | 37 | 2 839 | 234 k | 3.5 | 3 621 | 2 758 / 709 | 0.20 | 379 | 9 | 15 | 227 | 4.2 | 0 / 0 |
| S05 first person | 36 | 2 864 | 242 k | 1.0 | 2 985 | 2 244 / 654 | 0.21 | 367 | 9 | 14 | 218 | 2.9 | 0 / 0 |
| S06 in the house | 29 | 2 693 | 238 k | 3.4 | 2 537 | 2 104 / 531 | 0.79 | 344 | 9 | 9 | 213 | 2.8 | 0 / 0 |
| S07 garden | 35 | 2 735 | 201 k | 6.2 | 2 957 | 1 651 / 440 | 0.22 | 370 | 9 | 17 | 222 | 4.2 | 0 / 0 |
| S08 pond | 27 | 4 668 | 291 k | 4.8 | 2 114 | 1 903 / 462 | 0.35 | 320 | 8 | 13 | 191 | 5.6 | 0 / 0 |
| S08 · water off | 27 | 4 668 | 291 k | 1.9 | 1 646 | | | | | | | | |
| S08 · ambient off | 25 | 4 664 | 289 k | 1.3 | 1 425 | | | | | | | | |
| S09 level-3 spawn 20 m | 35 | 7 459 | 453 k | 1.6 | 2 043 | 1 354 / 351 | 0.11 | 337 | 8 | 8 | 202 | 2.8 | 0 / 0 |
| S10 night spawn | 35 | 2 732 | 234 k | 1.0 | 3 934 | 4 623 / 447 | 0.20 | 378 | 9 | 13 | 230 | 4.2 | 9 / 5 |
| S11 night zoo | 27 | 4 417 | 269 k | 0.9 | 3 616 | 3 742 / 443 | 0.11 | 335 | 9 | 10 | 208 | 2.8 | 9 / 13 |

### Scenarios — phone 720 × 1560 drawn (load average 46 → 54)

| Scenario | draw calls | inst. | tris | CPU `frame()` p50 ms | CPU+GPU fenced p50 ms | scene / full-screen pass ms | sim step ms | GL calls | programs | tex binds | uniforms | upload KB | lights / pools |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| S01 spawn 14 m | 17 | 2 499 | 192 k | 1.1 | 1 153 | 1 040 / 239 | 0.20 | 294 | 9 | 10 | 187 | 2.8 | 0 / 0 |
| S02 spawn 20 m | 20 | 2 653 | 219 k | 1.4 | 1 705 | 1 713 / 464 | 0.22 | 306 | 9 | 10 | 193 | 2.8 | 0 / 0 |
| S03 walking | 20 | 2 615 | 207 k | 2.1 | 2 027 | 1 397 / 350 | 0.15 | 303 | 9 | 9 | 192 | 2.8 | 0 / 0 |
| S04 look-around | 18 | 2 586 | 204 k | 1.6 | 1 966 | 905 / 278 | 0.23 | 273 | 8 | 8 | 172 | 2.8 | 0 / 0 |
| S05 first person | 15 | 2 546 | 200 k | 3.2 | 1 358 | 919 / 263 | 0.30 | 253 | 8 | 5 | 159 | 1.5 | 0 / 0 |
| S06 in the house | 22 | 2 579 | 215 k | 0.9 | 996 | 901 / 231 | 0.40 | 315 | 9 | 9 | 198 | 2.8 | 0 / 0 |
| S07 garden | 25 | 2 468 | 169 k | 1.1 | 1 066 | 879 / 310 | 0.30 | 332 | 9 | 16 | 204 | 4.2 | 0 / 0 |
| S08 pond | 16 | 1 864 | 72 k | 1.5 | 1 331 | 1 017 / 319 | 0.10 | 276 | 8 | 13 | 169 | 5.6 | 0 / 0 |
| S08 · water off | 16 | 1 864 | 72 k | 3.4 | 848 | | | | | | | | |
| S08 · ambient off | 14 | 1 860 | 70 k | 2.1 | 758 | | | | | | | | |
| S09 level-3 spawn 20 m | 22 | 6 347 | 304 k | 1.5 | 1 500 | 1 253 / 289 | 0.27 | 286 | 8 | 8 | 177 | 2.8 | 0 / 0 |
| S10 night spawn | 19 | 2 505 | 195 k | 3.1 | 2 515 | 2 054 / 230 | 0.92 | 302 | 9 | 10 | 191 | 2.8 | 9 / 5 |
| S11 night zoo | 14 | 2 543 | 177 k | 2.2 | 3 742 | 2 917 / 279 | 0.45 | 283 | 9 | 9 | 182 | 2.8 | 9 / 13 |

### Scenarios — desktop_half 960 × 540 (load average 55)

| Scenario | draw calls | inst. | tris | CPU `frame()` p50 ms | CPU+GPU fenced p50 ms | scene / full-screen pass ms | GL calls | uniforms |
|---|---|---|---|---|---|---|---|---|
| S01 spawn 14 m | 30 | 2 709 | 228 k | 4.7 | 1 282 | 724 / 161 | 358 | 220 |
| S02 spawn 20 m | 37 | 2 800 | 251 k | 3.4 | 1 073 | 957 / 211 | 384 | 232 |
| S10 night spawn | 35 | 2 732 | 234 k | 1.2 | 2 214 | 1 982 / 165 | 378 | 230 |

### Native probe (x86-64 release; WASM is slower, allocation counts exact)

| System | p50 µs | p95 µs | heap allocations / frame | bytes / frame |
|---|---|---|---|---|
| zoo-core `Game::update`, joined zoo (level 1–3 + night 1), 1 140 frames | 14.7 | 19.4 | **27.2** | 7 255 |
| `Ambient` update + poses (10 crowd poses, 4 butterflies) | 9.4 | 10.3 | 0 | 0 |
| `LevelScene::build` (once at load) | 14.5 ms | | | |

- Allocation sites of `Game::update` (10 traced frames, `PROBE_TRACE=1`): 230 ×
  `Game::interactables` (`game.rs` list rebuild + `String` clones of cut spots / plants /
  garden signs / moon doors), 10 × `update_wander` (`collect::<Vec<bool>>`), 20 ×
  `night::moon_doors`, 10 × `night_zoo_waiting`.
- Animation pose (sample idle + walk, blend, joint matrices, joint-texture copy): 3.4 µs
  (frog) … 28.6 µs (elephant, 26 joints); one pose of each of the 24 skinned models = 276 µs
  (192 µs in a quieter re-run).
- Models: 129 `.glb`, 73 533 triangles in total. Over the APIPE-005 budget: `moon_door.glb`
  and `moon_door_open.glb` 1 430 each (props ≤ 500). Buildings (no budget, Q-153):
  `entrance_arch` 692, `food_hut` 1 218, `food_storage` 1 198, `zookeeper_house` 1 388,
  `night_house` 3 196. Animals 925–2 759 (hedgehog), player 2 156 — within 3 000. Textures:
  all ≤ 256² (player body 64²).

### Static draw list (`debug_draw_list`, triangles = per-model triangles × instances)

- Desktop S01 (228 k triangles): `path_edge` 344 × 158 = 54 k, `grass_tile` 1 632 × 28 = 46 k,
  `path_tile_t` 270 × 164 = 44 k, `path_tile_cross` 132 × 198 = 26 k, `plaza_tile` 15 k,
  `zoo_wall` 13 k, `fence_wood` 8 k — **the ground tiles are ≈ 85 % of all triangles**, and
  every drawn batch draws all its instances (chunk culling decides only whether the batch is
  drawn), so the whole level's grass is submitted from a 14 m view. Buildings and baked
  groups are 1–2 k each.
- Desktop S09 (453 k): `grass_tile` of three regions (58 k + 46 k + 46 k), `path_edge` 48 k +
  44 k, `path_tile_t` 45 k + 40 k, `path_tile_cross` 34 k + 28 k, `hedge` 3 × 11–13 k.

### Findings vs. budgets

| Budget (PERF-BUDGETS) | Measured | Verdict |
|---|---|---|
| 1 — 60 / 30 fps on the reference phone | not measurable here (no phone, SwiftShader on a busy machine) | open (Q-013, Q-166) |
| 2 — download ≤ 30 MB | 6.61 MB transferred on first load (8.36 MB dist, 2.46 MB with brotli) | ✅ 22 % |
| 3 — ≤ 140 draw calls / ≤ 10 k instances (proposal Q-104) | max 37 draw calls (desktop S02/S04), 7 459 instances (S09) | ✅ |
| 4 — close views < zoo view at 20 m | S04 37 / S05 36 at the ring path; the same-spot 20 m comparison is CAMV-014 (not re-measured here) | — |
| 5 — night ≤ +6 draw calls | desktop +5 (S10 35 vs. S01 30), phone +2 | ✅ (close to the limit on desktop) |
| 6 — 9 point lights + ≤ 24 pools | S10 9 / 5, S11 9 / 13 | ✅ |
| 7 — water +0 draw calls | S08 27 = 27 (desktop), 16 = 16 (phone); GPU ≤ 1 ms not measurable here | ✅ counts |
| 8 — ambient ≤ 10 draw calls, ≤ 0.5 ms CPU | +2 draw calls (S08 A/B); `Ambient` update 9 µs native | ✅ |
| 9 — sky +0 draw calls | S04/S05 (sky visible) have no extra draw | ✅ |
| 10 — triangles per model | moon door 1 430 > 500 (props); night house 3 196 (no budget) | ❌ moon door (PERF-R-010) |
| 11/12 — textures, `.glb` size | all textures ≤ 256²; largest animal `.glb` 265 KB ≤ 400 KB | ✅ |
| 14 — 0 allocations per frame (proposal Q-169) | `Game::update` 27 allocations / 7.3 KB per frame; zoo-web and zoo-render allocate per frame too (review) | ❌ (PERF-R-004) |
| 15 — static batching | ground tiles are instanced but not culled below batch level: all of a region's tiles drawn | ⚠️ (PERF-R-002) |

### Where the time goes (relative, SwiftShader)

- **Scene pass ≈ 75–80 % of a frame, full-screen outline + fog + sky pass ≈ 20–25 %** (day,
  both viewports). Pixel scaling: desktop_half (0.52 MP) vs. desktop (2.07 MP) → full-screen
  pass 161 vs. 515 ms (× 3.2 for × 4 pixels) — fragment bound as expected.
- **Night doubles the scene pass** at the same spot (desktop S10 4 623 vs. S01 2 023 ms,
  phone 2 054 vs. 1 040 ms) with only +2…5 draw calls: the per-fragment point-light / light-
  pool loop (PERF-R-001).
- **CPU side is small:** `app.frame()` 1–6 ms p50 in WASM under load (simulation step
  0.1–0.9 ms), ≈ 280–385 WebGL calls per frame of which ≈ 60 % are `uniform*` calls
  (PERF-R-003), 8–9 `useProgram`, 7 `bufferSubData` (dynamic batches re-uploaded even when
  unchanged, PERF-R-009), 1–3 `texSubImage2D` (joint textures), 1.5–5.6 KB uploaded.
- **Vertex side:** 170–450 k triangles submitted per frame, ≈ 85 % ground tiles, most of
  them off screen (PERF-R-002).
- Water animation and ambient animals are cheap: 0 and 2 draw calls; their time A/B is
  below the noise of this run.

Regressions: none (first run). Top recommendations: PERF-R-001 (night lights), PERF-R-002
(ground tiles), PERF-R-003 (uniform traffic) — see `recommendations.md`.

## Run 2026-09-28 — PERF-R-001 + PERF-R-003 (before / after)

- **What changed since the last run:** PERF-R-003 — the values every scene program shares
  (view, view-projection, sun, shadow tint, fade, dither, night / warm, water clock, glow,
  point lights, light pools) are one std140 uniform block uploaded once per frame; uniform
  locations are an array per program looked up at link time; per-draw uniforms are sent
  only when their value changed; constant uniforms (samplers, outline colour, crowd model
  matrix) are set once; the redundant first `set_common(static)` is gone. PERF-R-001 — each
  draw gets a light mask (`u_light_mask`: the lights whose sphere / pool disc can reach its
  chunk bounds); the light loops walk the mask and skip a light outside its radius right
  after its `fwidth` (pixel-identical; the derivative-free variant is Q-180 / PERF-R-014).
- **Before:** commit `d0134d2`, clean tree (`tools/perf/run.sh`, 06:50). **After:** the same
  snapshot tree with only `crates/zoo-render/src/{night,renderer,shaders}.rs` and
  `crates/zoo-web/src/lib.rs` replaced by the working copy (08:09; the other agent's level /
  zoo-core changes of that time were kept out, so the A/B isolates the renderer change;
  `run.sh --skip-build` after a manual rebuild of the snapshot). Raw data:
  `tools/perf/baselines/2026-09-28-before.json`, `tools/perf/baselines/2026-09-28.json`.
- **Machine:** as 2026-09-27 (Ryzen 5 5500U, Linux 6.8, rustc 1.98.1, wasm-pack 0.15.0,
  Chrome headless shell 153). Load average 9–25 during the sequential SwiftShader runs
  (other agents' tests): **their frame times are not comparable between the two runs**; the
  timing verdict comes from the interleaved A/B below. Counts are exact.
- **New tool:** `tools/perf/look.mjs` — `capture` renders 14 fixed look scenarios (L01–L14:
  day, dusk, night, pond, house, close views; 960 × 540, seed 17, fake clock and manual
  `requestAnimationFrame`, so deterministic) and `compare` diffs two captures pixel by pixel;
  `ab` opens both builds side by side and alternates fenced frames A, B, B, A, … (load drift
  hits both; `LOOK_ANGLE=gl` uses the real GPU; `AB_INIT_B` patches the B pages, e.g. a GLSL
  variant via `shaderSource`).

### Look (identical?)

- Baseline captured twice: 0 differing pixels in all 14 scenarios (deterministic).
- **Baseline vs. after: 0 differing pixels in all 14 scenarios** (L01–L07 day / dusk,
  L08–L14 night: spawn at 14 m and 20 m, night zoo, night pond, first person, look-around,
  house) — lights, pools, eye glow, fog, sky and outlines unchanged.
- e2e PERF-017: the night frame with per-draw masks equals the frame with every light for
  every draw (spawn, night zoo; `perf_rules.spec.ts`).
- A first attempt (derivatives once before the loops, variant "B") changed 14–699 rim pixels
  per night scenario (up to 120 / 255): it removes dark specks that the old per-light
  `fwidth` in non-uniform control flow leaves on the pool rims. Kept out: Q-180 / PERF-R-014.

### GL traffic per frame (census, exact; before → after)

| desktop | GL calls | `uniform*` | `useProgram` | `bufferSubData` | CPU `frame()` p50 ms | scene / full-screen ms (SwiftShader, sequential) |
|---|---|---|---|---|---|---|
| S01 | 358 → 153 | 220 → 14 | 9 → 8 | 7 → 8 | 1.1 → 0.7 | 700 / 180 → 964 / 272 |
| S02 | 384 → 169 | 232 → 16 | 9 → 8 | 7 → 8 | 0.9 → 0.5 | 1665 / 476 → 658 / 161 |
| S03 | 342 → 146 | 211 → 14 | 9 → 8 | 7 → 8 | 0.9 → 0.6 | 762 / 216 → 590 / 159 |
| S04 | 379 → 169 | 227 → 16 | 9 → 8 | 7 → 8 | 0.7 → 0.6 | 924 / 215 → 890 / 195 |
| S05 | 367 → 168 | 218 → 18 | 9 → 8 | 9 → 10 | 0.6 → 0.6 | 658 / 204 → 670 / 192 |
| S06 | 344 → 145 | 213 → 13 | 9 → 8 | 7 → 8 | 0.8 → 0.5 | 701 / 214 → 590 / 223 |
| S07 | 370 → 168 | 222 → 19 | 9 → 8 | 7 → 8 | 0.8 → 0.5 | 463 / 175 → 451 / 160 |
| S08 | 320 → 143 | 191 → 13 | 8 → 7 | 7 → 8 | 0.8 → 0.6 | 626 / 217 → 818 / 276 |
| S09 | 342 → 154 | 205 → 16 | 8 → 7 | 7 → 8 | 0.7 → 0.6 | 829 / 223 → 533 / 154 |
| S10 | 378 → 192 | 230 → 43 | 9 → 8 | 7 → 8 | 0.6 → 0.6 | 1600 / 189 → 1238 / 142 |
| S11 | 335 → 163 | 208 → 35 | 9 → 8 | 7 → 8 | 0.9 → 0.5 | 2291 / 240 → 1165 / 141 |

| phone | GL calls | `uniform*` | `useProgram` | `bufferSubData` | CPU `frame()` p50 ms | scene / full-screen ms (SwiftShader, sequential) |
|---|---|---|---|---|---|---|
| S01 | 294 → 118 | 187 → 10 | 9 → 8 | 7 → 8 | 0.8 → 0.5 | 405 / 122 → 270 / 76 |
| S02 | 306 → 124 | 193 → 10 | 9 → 8 | 7 → 8 | 0.8 → 0.5 | 325 / 92 → 277 / 77 |
| S03 | 294 → 121 | 187 → 11 | 9 → 8 | 7 → 8 | 0.6 → 0.5 | 280 / 83 → 262 / 84 |
| S04 | 273 → 111 | 172 → 9 | 8 → 7 | 7 → 8 | 0.6 → 0.4 | 415 / 133 → 302 / 86 |
| S05 | 253 → 104 | 159 → 9 | 8 → 7 | 9 → 10 | 0.7 → 0.4 | 311 / 123 → 215 / 78 |
| S06 | 315 → 130 | 198 → 12 | 9 → 8 | 7 → 8 | 0.8 → 0.5 | 396 / 108 → 269 / 74 |
| S07 | 332 → 147 | 204 → 18 | 9 → 8 | 7 → 8 | 0.8 → 0.5 | 269 / 109 → 182 / 84 |
| S08 | 276 → 119 | 169 → 11 | 8 → 7 | 7 → 8 | 0.9 → 0.5 | 301 / 98 → 230 / 61 |
| S09 | 291 → 123 | 180 → 11 | 8 → 7 | 7 → 8 | 0.6 → 0.4 | 277 / 86 → 264 / 78 |
| S10 | 302 → 135 | 191 → 23 | 9 → 8 | 7 → 8 | 0.7 → 0.5 | 821 / 110 → 625 / 71 |
| S11 | 283 → 124 | 182 → 22 | 9 → 8 | 7 → 8 | 0.8 → 0.5 | 968 / 107 → 533 / 76 |

- **PERF-R-003:** GL calls −53…−60 % (desktop S01 358 → 153, phone S01 294 → 118);
  `uniform*` calls −80…−95 % (day 187–232 → 9–19; night 191–230 → 22–43, incl. the per-draw
  light masks); `useProgram` −1; `bufferSubData` +1 (the 880-byte frame block; uploads
  +0.8 KB). CPU `frame()` p50 (JS time incl. GL submission; SwiftShader, busy machine)
  0.6–1.1 → 0.4–0.7 ms.
- Draw calls, instances and triangles are unchanged in every scenario. The scene /
  full-screen ms column is load-dominated (sequential runs) — see the A/B.

### Frame time — interleaved A/B (fenced frames, medians; after / before < 1 = faster)

Real GPU (**AMD Renoir iGPU, ANGLE → OpenGL 4.6**, `LOOK_ANGLE=gl`, 20 rounds × 10 frames;
noise check baseline vs. baseline at phone size: 0.99–1.01):

| phone 720 × 1560 | frame ms before → after | scene pass ms before → after | full-screen pass ms |
|---|---|---|---|
| L01 day spawn | 2.9 → 2.9 (1.00) | 1.9 → 1.9 | 1.0 → 1.0 |
| L02 day 20 m | 2.9 → 3.0 (1.03) | 1.9 → 1.9 | 1.0 → 1.0 |
| L04 day first person | 4.1 → 4.0 (0.98) | 1.9 → 1.9 | 2.1 → 2.2 |
| L06 day pond | 3.1 → 3.0 (0.97) | 2.1 → 2.0 | 1.0 → 1.0 |
| L08 night spawn | 8.6 → 6.8 (**0.79**) | 7.1 → 5.4 (**−24 %**) | 1.5 → 1.4 |
| L09 night 20 m | 8.1 → 6.3 (**0.78**) | 6.7 → 4.9 (**−27 %**) | 1.4 → 1.4 |
| L10 night zoo | 11.5 → 6.8 (**0.59**) | 9.9 → 5.3 (**−46 %**) | 1.5 → 1.4 |
| L12 night first person | 8.0 → 7.2 (**0.90**) | 5.4 → 4.8 (**−11 %**) | 2.5 → 2.4 |

- Desktop 1920 × 1080 on the real GPU has a page-order bias (the second page's full-screen
  pass is 2× slower even for identical builds); both orders averaged (geometric mean): scene
  pass day −6…−10 % (L01, L04), night spawn −25 %, night zoo −40 %, night first person −13 %.
- Night vs. day at the same spot (real GPU, phone): before 2.9–4.0 ×, after 2.3–2.5 ×.
- Attribution (real GPU, phone): the old loop with the new per-draw masks (patched GLSL via
  `AB_INIT_B`) is 15–28 % slower in the night scene pass than the shipped loop, so most of
  the gain is the per-fragment skip, the rest the masks; variant B (Q-180) adds only 2–4 %.

SwiftShader (software, `look.mjs ab`, relative only): night zoo −17 % (desktop) / −33 %
(phone), night spawn −1…−8 %, but **day +6…+12 % and night first person +11…+14 %** (in both
page orders; the unchanged full-screen pass also +5…12 %). Diagnostics ruled out the vertex
shader's block reads (D1: plain uniforms), the loop shape (D2: old loop) and the fragment
shader's block reads (D3: plain uniforms). SwiftShader predicates branches (a per-fragment
`continue` saves nothing), so it cannot show the skip's gain; the real GPU shows no day
slowdown. Logged as a SwiftShader-only effect, to watch in the next runs (the CI regression
rule uses SwiftShader).

### Findings vs. budgets (budgets of Q-166…Q-170, answered 2026-09-28)

| Budget (PERF-BUDGETS) | Measured | Verdict |
|---|---|---|
| 16 — night ≤ 1.3 × day (reference phone) | real iGPU at phone size: night 2.3–2.5 × day (was 2.9–4.0 ×) | ❌ still above: next PERF-R-014 (small), PERF-R-005 low tier, PERF-R-002 |
| 16 — per-pass ms on the reference phone | not measurable here (iGPU ≠ phone; Q-013) | open |
| 17 — release WASM ≤ 2.0 MB / ≤ 600 KB brotli | 1 698 KB / 490 KB (+3 KB raw) | ✅ |
| 18 — memory | WASM heap 17.2 MB, JS heap 10.1 MB | ✅ (GPU not measured) |
| 19 — ≤ 300 k triangles, ≤ 10 k instances | desktop S09 **454 k** triangles (phone S09 304 k), 7 460 instances | ❌ triangles in S09 (PERF-R-002) |
| 20 — shared uniforms once per frame | 1 frame-block upload, 0 location lookups, 0 redundant scalar uniform calls (e2e PERF-016) | ✅ |
| 21 — night lights per draw, identical picture | 0 differing pixels (look L08–L14, e2e PERF-017) | ✅ |

Regressions (> 10 % worse than the previous run): none in counts; the sequential SwiftShader
times are load-dominated; the interleaved SwiftShader day +6…12 % is explained above (not
seen on the real GPU).

## Test cases

No test cases of its own: the measuring tool and the log format are checked by PERF-015
(PERF-BUDGETS); each run is the evidence for the PERF-BUDGETS test cases it reports.

## Open questions

- Q-013 reference phone, Q-166 frame-time budgets, Q-167 WASM / first-load budgets,
  Q-168 memory budget, Q-169 allocation rule scope (see PERF-BUDGETS).
