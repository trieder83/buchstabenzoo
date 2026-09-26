---
id: TECH-WATER
title: Animated cartoon water (rendering)
aspect: tech
module: water-rendering
status: implemented
depends_on: [ART-ENVIRONMENT, TECH-ARCH, GAME-LAYOUT]
test_prefix: WATER
updated: 2026-09-26
---

# Animated cartoon water (rendering)

## Goal

Make all water alive in the comic style (ART-ENVIRONMENT rule 5 "Living water",
tests AENV-007…010) inside the existing renderer (TECH-ARCH §7: raw WebGL2, instanced 1 m
tiles sharing one palette texture, 2-tone cel shading into a G-buffer, screen-space
outline pass). The river must read as **flowing** and the pond as **still but alive** from
the 55° game camera (GAME-PLAYER §2: 35° vertical FOV, 10–20 m zoom) — this contrast is a
riddle clue (zebra vs. hippo, CONT-MISSIONS). Budget: ≤ 1 ms GPU per frame on a mid-range
phone, no extra draw calls.

Prototype: `web/prototypes/water.html` (standalone raw WebGL2; `?t=` freezes time,
`?mode=1|2` shows the alternatives, `?dist=10..20` zoom). Renders:
`art/environment/water/` (catalog item `water_anim`).

## 1. Starting point (PoC M3)

- Water tiles (`tools/blender/props/kit_water.py`) are 1 m × 1 m meshes, autotiled from
  the 4-neighbour water mask and rotated in quarter turns (`zoo_core::scene::water_tile`; M4 moved the level assembly from `zoo-render` to `zoo-core`). A
  tile = flat water polygon (palette cell `water_river` 176 / `water_pond` 179) at y = 0,
  plus — on bank tiles — a 0.22 m grass strip (y = 0.05) and a 0.12 m soil slope down to
  the water. River tiles carry baked, static **streak** polygons (`water_river_light` 177,
  4 mm above the water) and foam dots (`water_foam` 178) along the canonical N–S axis.
- All static meshes share one program: instance = origin + yaw (`a_pos_yaw`), scale +
  fade flag, flat colour; `u_edge_mask` per batch (water tiles currently get 1.0 — their
  names do not end in `_tile`). No time uniform exists.
- Outline pass: depth edges from the Laplacian of inverse linear depth (zero on planes)
  and normal edges where the edge mask allows. The shore line we see today is the normal
  crease between the soil slope and the water polygon.
- Problems visible in `art/environment/poc/screenshot_poc_m3_bridge.png`: streaks are
  static; they repeat every 1 m; at the river bend they switch abruptly from N–S to W–E at
  a tile seam; the pond is flat colour.

## 2. Approaches compared

| | (a) Scroll the kit streaks per tile | (b) World-space bands, constant flow per cell | (c) Vertex waves on water tiles | (d) River-coordinate field + procedural streaks (**recommended**) |
|---|---|---|---|---|
| Idea | Move streak geometry / a streak mask in tile-local coordinates along the tile's canonical axis, `fract()` wrap inside the tile | `step(sin(dot(p, flow) − v·t + wobble))` in world XZ; flow = per-cell constant (from element orientation) | Displace water vertices with `sin(p, t)` | CPU bakes a small level texture holding river coordinates (s = distance along the flow, c = cross offset from the centreline), shore distance and flow flag; the fragment shader draws hard-edged dashes in (s, c) space scrolling with `s − v·t` |
| Comic look | OK (the concept streaks) but a visible 1 m grid repeat (prototype mode 2) | Bold wavy crest bands — cartoon, but reads more like "waves" than current | Needs dense meshes to show; smooth sine shapes = not comic | Lens-shaped light dashes on sinuous lanes + white flecks — matches the kit concept sheet (long light flow lines) |
| River vs. pond | Pond: nothing to scroll | Pond: bands without flow look like a river | Both similar | River: directional scroll; pond: rings + twinkles with no net direction |
| Seams / rotations | Seamless only along a column of equal tiles; **hard seam at every direction change** (bend) and across lanes | Seamless on straight runs; **hard seam at the bend** (prototype mode 1) | Seamless if world-space | Seamless everywhere by construction: every input is a world-space function; tile rotation is irrelevant; streaks follow the bend |
| Cost | ~6 streak evaluations per fragment, 0 texture reads | ~15 ALU | Vertex ALU, but tiles have 2–94 vertices → needs subdivision (+ triangles) | 1 bilinear texture read + ~70–100 ALU per water fragment |
| Data needed | Tile flow sign per instance (bank tiles cannot be flipped by rotation) | Flow per instance or per cell | Subdivided tiles | Level assembly: river centreline (from element flow) → field texture; nothing per instance |
| Outline pass | OK (colour only) | OK | **Bad**: moving depth/normals → Laplacian/normal edges flicker on crests | OK: colour only; depth and normal unchanged |

Parts that are not alternatives but are needed in any case:

- **Shore foam** needs a distance-to-shore value. Options: (1) vertex attribute baked by the
  kit (needs `TEXCOORD_1` in `zoo-assets` + a new vertex layout + re-triangulated water
  polygons so it interpolates correctly); (2) a shoreline mask texture per tile (texture
  read, still per tile); (3) **analytic distance per cell in the same field texture**,
  computed from the tile kind + rotation that `scene.rs` already chooses and the kit
  constants (M = 0.22, SLOPE = 0.12, R = 0.78), guarded by an asset test against the
  exported meshes. → **(3)**: one texture read serves flow and foam, no mesh-format change.
- **Pond ripples**: concentric rings from sources on a jittered world grid (one source per
  2.5 m cell, only the fragment's own cell is evaluated → 1 evaluation, seamless).
  Alternative: a uniform list of ring centres (lily pads, frog) — up to 8 evaluations; not
  needed.
- **Bobbing** of ducks / lily pads / frog: per-batch uniform amplitude + per-instance
  phase from `hash(instance position)` in the vertex shader (no new instance data; a frog
  sitting on its pad at the same position shares the pad's phase).

## 3. Decision (recommended combination)

1. **Water field texture** baked once per level on the CPU (level assembly): RGBA16F,
   4 texels per metre, covering the bounding box of all water + 2 m (behaviour 2), LINEAR
   filtered, CLAMP_TO_EDGE.
   `R = s` (m along the flow), `G = c` (m, signed offset from the river centreline, + =
   inside of the next bend), `B = shore` (m from the waterline into the water; negative on
   land), `A = flow` (1 = river, 0 = pond / still water; values between allowed later,
   e.g. rapids > 1).
2. **Separate water program** (VS + FS) for the `water_*` tile batches; all other props
   keep the static program. The water FS shades palette cells `water_river` (176) and
   `water_pond` (179) procedurally, everything else (grass strip, soil slope) with the
   normal cel path. No new draw calls: water tiles are already separate batches.
3. **River**: sinuous lanes across the flow, lens-shaped hard-edged dashes in
   `water_river_light` scrolling along `s`, faster in the middle; sparse `water_foam`
   flecks; an animated foam band + dashed second foam line along the banks; foam ring and
   dashed V-wake at obstacles (rocks).
4. **Pond**: expanding double rings (4 s cycle), twinkling light dashes that grow and
   shrink in place, slowly breathing comic glints, a soft lapping line at the bank — all in
   `water_pond_light`, **no net motion direction**.
5. **Bobbing** of `duck`, `lily_pad`, `frog` in the static VS via per-batch `u_bob`.
6. **Global loop**: `u_time = elapsed mod 16 s`; every animation period divides 16 s.
7. **Outlines**: the water surface stays flat and writes the tile normal (0, 1, 0) and its
   real depth; the animation changes colour only. The waterline outline keeps coming from
   the slope/water crease (edge mask 1 on water batches). The baked streak and foam
   polygons are removed from the kit (Q-067), so no geometry sits on the water surface.

## Behaviour

1. Water tiles are drawn by the water program; each water fragment reads the water field
   once at its world XZ position. Shading uses only world position, the field and
   `u_time` — never tile-local coordinates or instance yaw — so it is seamless across tile
   seams and independent of tile rotation.
2. The field is baked from level data: river cells get `s`/`c` from the river centreline
   (straight pieces along each river element's `flow` (GAME-LAYOUT "Flowing water",
   Q-066), 90° arcs of radius = half the river width around the inner corner of every
   bend, `s` continuous along the whole river; `c` = offset to the **left of the flow** in
   level coordinates, which is the inside of a left bend); pond, pool and fountain texels
   get `s = c = 0`, `flow = 0`; `shore` is the exact distance to the **visible waterline**
   — the foot of the bank slope, 0.34 m (`M + SLOPE`) inside a bank edge, 0.66 m
   (`R − SLOPE`) from the centre of a rounded outer corner, 0.34 m from the dry corner of an
   inner notch — found among the tiles of the 5 × 5 cells around the texel (clamped to
   −1…3 m); the fountain basin is a still-water rectangle (distance to its rim). Land
   texels take `s`/`c`/`flow` of the nearest water cell's body (the centreline functions
   are defined everywhere, no dilation pass needed), so bilinear filtering stays correct at
   the banks. The field covers the bounding box of all water + 2 m (joined zoo: one
   texture).
3. River streaks (layer "streak"): lane width 0.24 m; lane offset
   `c' = c + 0.045·sin(2.1·s + 1.7·c)`; per lane speed `V = mix(0.85, 0.5, cn²)` m/s with
   `cn = |lane centre| / (half river width − 0.34)`; dash period `P = 2 s · V`; dash length
   28–55 % of P; half width 0.02–0.05 m with a `sin` lens profile; 70 % of dashes present;
   hidden where `shore < 0.16`. Colour `water_river_light`.
4. River flecks (layer "fleck"): lane 0.45 m, `V = 1.0` m/s, `P = 2.0` m, length 9 % of
   P, half width 0.03 m, 28 % present, only where `shore ≥ 0.4`. Colour `water_foam`.
5. River shore foam: solid band `shore < 0.035 + 0.05·wob`,
   `wob = 0.5 + 0.35·sin(2π(s/1.4 − t/2)) + 0.3·sin(2π(s/0.7 − t))`, plus a dashed line
   0.028 m wide at `shore = band + 0.07`, dashes `fract((s − 0.7t)/0.7) > 0.55`. Colour
   `water_foam`. The band travels with the flow (0.7 m/s).
6. Obstacle foam (Q-068 answered: bridge posts, stones in the water, the water wheel, the
   jetty posts — plus the fountain jet): for each obstacle (up to 4 per frame, nearest to
   the camera target; uniform `u_obstacles[4] = (x, z, radius, s)` + `u_obstacles_b[4] =
   (c, river flag, –, –)`): ring `|p − o| < r + 0.06 + 0.03·sin(3·angle − 2πt)` (river and
   still water); in rivers only a V-wake: two dashed lines at `|c − c_o| = 0.8r +
   0.22·(s − s_o)` for `0 < s − s_o < 1.1`, tapering from 0.02 m to 0.006 m, dashes
   `fract((s − s_o − 0.7t)/0.35) > 0.5` (shorter and thinner than the prototype: four piles
   side by side read as rain streaks with the long version). Obstacles of level 1–3: the
   four `bridge_wood` piles (r 0.075, standing in the water under the deck edges; added to
   the kit for this), three stones of the `river_n` rapids (`rock` × 0.3, r 0.2), the two
   jetty posts in the pond (r 0.08, ring only), the mill's water wheel in the level-3 stream
   (r 0.3) and the fountain jet (r 0.13, ring only = the small spray ring).
7. Pond: shimmer lanes rotated 30°, lane 0.42 m, dash period 1.1 m, length 40 %, half
   width `0.045·max(0, sin(2π(t/4 + h)))` (twinkle, 45 % present); rings: one source per
   2.5 m world cell (jitter ±0.35 m, 75 % of cells), cycle 4 s, radius `0.95·age`, second
   ring 0.24 m behind, thickness `0.04·(1 − age) + 0.006`, hidden where `shore < 0.18`;
   glints: per 4 m cell (65 %) two parallel capsules (0.52/0.26 m × 0.09/0.08 m) breathing
   with an 8 s period; lapping line `shore < 0.04 + 0.025·sin(2πt/4 + 1.3(x − z))`. All in
   `water_pond_light`.
8. Bobbing (static VS, per batch `u_bob = (amp_y m, tilt rad, drift radius m, 0)`, phase
   `bob_hash(floor(origin.xz·10))·2π` — an **integer** hash, identical in GLSL and Rust
   (`zoo_core::water::bob_hash`), so CPU-placed animals move exactly with their pad):
   vertical `amp_y·sin(2πt/2 + φ)`, roll about the model's local X
   `tilt·sin(2πt/4 + φ + 1.3)`, drift on a circle with period 16 s. Values: `duck` and
   `duckling` (0.03, 0.07, 0.12), `lily_pad` (0.012, 0.04, 0.03), `frog` (0.012, 0.04, 0.03); every
   other batch (0, 0, 0) — unchanged. Ducks, ducklings and frogs are animated ambient
   animals (GAME-AMBIENT), not static props: their bob is computed on the CPU with the same
   function (swimmers without drift — they swim themselves; a frog on a pad gets the pad's
   full transform: `bob_transform`).
9. Time: `u_time = (elapsed_s mod 16.0)` computed in Rust from `f64`; all periods (2, 4,
   8, 16 s; spatial patterns hash `cell mod 8` along `s`) divide 16 s, so the animation
   loops seamlessly and depends on time only, not on frame rate.
10. Anti-aliasing of the hard edges uses the pixel footprint of the world position
    (`length(fwidth(p.xz))`), not `fwidth` of the pattern value (that produces seams at
    lane/cell borders — seen in the prototype).
11. The water program writes `o_normal = (0, 1, 0)` packed and the batch edge mask, never
    discards, never displaces vertices → the outline pass sees a static flat surface; no
    outline appears inside the water except at props and the waterline.
12. Colours come from the palette table only (`water_river`, `water_river_light`,
    `water_foam`, `water_pond`, `water_pond_light`), read from the palette texture by cell
    index (`textureLod`, no derivatives in divergent code); no new colours.
13. Ripples of the ambient animals (GAME-AMBIENT 5): up to 8 per frame (nearest to the
    player; uniforms `u_ripples[8] = (x, z, heading x, heading z)`, `u_ripples_b[8] =
    (wake 0…1 = speed / 0.6 m/s, dip ring age 0…1 or −1)`, `u_ripple_count`): a dashed V
    behind a swimming duck or frog (arms `0.1 + 0.4·back` m for `back` 0.08…1.3 m, dashes
    travelling backwards) and a double ring growing to ≈ 1 m while a duck dips (2.6 s). In
    rivers the wake is `water_foam`, in still water `water_pond_light`.
14. Still basins without ground tiles (the level-2 fountain) are drawn with `water_pond`
    tiles scaled to the basin (2 × 2, y = 0.61 m) and get a still-water rectangle in the
    field, so they look like the pond (shimmer, rings, lapping line at the rim) with the
    jet's spray ring. Pools (hippo, elephant, goldfish) use the pond tiles as before.
15. Water hides what is below its surface: the water surface is opaque and depth-tested
    (y = 0), so duck feet, a dipping head and the submerged body of a swimming frog are
    hidden; the intersection line gets the comic outline from the normal/depth edge.

## 4. Shader specification

### Uniforms (water program)

| Uniform | Type | Source |
|---|---|---|
| `u_view`, `u_view_proj`, `u_sun_dir`, `u_shadow_tint`, `u_edge_mask`, `u_palette`, `u_fade`, `u_dither` | as static program | unchanged |
| `u_time` | float | `elapsed mod 16` |
| `u_field` | sampler2D (RGBA16F) | water field of the level |
| `u_field_xf` | vec4 | `xy` = world XZ of the field origin, `zw` = 1 / field size (m) |
| `u_water_cols` | vec3[5] | river, river_light, foam, pond, pond_light (palette) |
| `u_obstacles` / `u_obstacles_b` | vec4[4] / vec4[4] | obstacle foam (behaviour 6); radius 0 = unused; `b = (c, river flag)` |
| `u_ripples` / `u_ripples_b` / `u_ripple_count` | vec4[8] / vec4[8] / int | duck and frog wakes, dip rings (behaviour 13) |

Vertex shader = `static_vs` plus `out vec3 v_world`. (Bobbing lives in `static_vs`:
`uniform float u_time; uniform vec4 u_bob;`.)

### Pseudo-GLSL (fragment)

```glsl
g_px = length(fwidth(v_world.xz)) * 0.7071;            // pixel footprint in m
float aastep(float e, float v) { return smoothstep(e - 0.6*g_px, e + 0.6*g_px, v); }

int cell = int(floor(v_uv.y * 16.0)) * 16 + int(floor(v_uv.x * 16.0));
if (cell != 176 && cell != 179) { /* normal cel path */ }
vec4 F = texture(u_field, (v_world.xz - u_field_xf.xy) * u_field_xf.zw);
float s = F.r, c = F.g, shore = F.b;
vec3 col;
if (cell == 176) {                       // river
  col = RIVER;
  col = streaks(s, c, shore, u_time, col);   // behaviour 3 (+ flecks, 4)
  col = shoreFoam(s, shore, u_time, col);    // behaviour 5
  for (i < 4) col = obstacleFoam(i, v_world.xz, s, c, u_time, col); // 6
} else {                                 // pond
  col = pond(v_world.xz, shore, u_time);     // behaviour 7
}
// water is always lit (normal up), no shadow tone: o_color = col
o_normal = vec4(0.5, 1.0, 0.5, u_edge_mask);

vec3 streaks(float s, float c, float shore, float t, vec3 col) {
  c += 0.045 * sin(2.1 * s + 1.7 * c);
  float li = floor(c / 0.24), lc = (fract(c / 0.24) - 0.5) * 0.24;
  float cn = clamp(abs((li + 0.5) * 0.24) / HALF_W, 0.0, 1.0);   // HALF_W = 1.16 (3 m river)
  float V = mix(0.85, 0.5, cn * cn), P = 2.0 * V;
  float x = (s - V * t) / P + hash1(li + 3.1) * 8.0;
  float h = hash2(vec2(li, mod(floor(x), 8.0))), u = fract(x);
  float len = mix(0.28, 0.55, fract(h * 13.7));
  float hw = mix(0.02, 0.05, fract(h * 7.3)) * sin(PI * clamp(u / len, 0.0, 1.0))
           * step(u, len) * step(0.3, h);
  col = mix(col, RIVER_LIGHT, (1.0 - aastep(hw, abs(lc))) * step(0.16, shore));
  /* flecks: same scheme, lane 0.45, V 1.0, P 2.0, len 0.09, hw 0.03, h > 0.72, FOAM */
  return col;
}
```

The full, tested GLSL of every function is in `web/prototypes/water.html`
(`riverStreaks`, `riverShore`, `rockFoam`, `pondWater`, `PROP_VS` for bobbing).
`HALF_W` should come from the field (e.g. store `c / half_width` in G instead of `c`) once
rivers of other widths exist (Q-066).

### Parameters (summary)

| Parameter | Value | Why |
|---|---|---|
| River streak speed | 0.5 (banks) – 0.85 m/s (middle) | AENV rule 5: 0.5–1 m/s visual speed |
| Fleck speed | 1.0 m/s | rapids feel |
| Shore foam travel | 0.7 m/s | follows the flow |
| Loop | 16 s; river cells loop every 2 s | AENV-008 |
| Pond ring cycle | 4 s, max radius 0.95 m | calm |
| Twinkle / glint | 4 s / 8 s | calm, no direction |
| Field resolution | 4 texels/m, RGBA16F | 0.25 m is enough for smooth bends and exact straight banks; measured (M6): level 1 188 × 148 texels ≈ 220 KB, joined zoo 368 × 308 texels ≈ 0.9 MB |
| `s` precision | half float: ≤ 3 cm up to 64 m, 6 cm up to 128 m | static error, invisible |

## 5. Implementation plan (after M4)

### zoo-core (level assembly — `scene.rs` lives here since M4)

1. `src/water.rs` (new, pure, `cargo test -p zoo-core`):
   - `pub struct WaterField { origin: Vec2, texels_per_m: u32, w: u32, h: u32, data: Vec<[f32; 4]> }`
   - `pub fn bake_water_field(level: &LevelData, tiles: &[WaterTilePlacement]) -> WaterField`:
     river centreline from river elements (+ bridge cells) in flow order, pieces =
     straight / 90° arc; per texel: nearest piece → `(s, c)`; `shore` from the cell's tile
     kind + quarter turns (kit constants `M = 0.22`, `SLOPE = 0.12`, `R = 0.78`, waterline
     inset `0.34`); land texels take the nearest water body's values (behaviour 2 — no
     separate dilation pass).
   - `pub const WATER_LOOP_S: f64 = 16.0; pub fn water_time(elapsed_s: f64) -> f32`.
   - Rust mirror of the streak / ring masks (`river_streak_mask(s, c, t)`,
     `pond_ring_mask(p, t)`) and of the bobbing offset, used only by unit tests (loop and
     frame-rate independence).
2. `scene.rs`: keep `water_tile()`; additionally record `WaterTilePlacement { cell, body,
   kind, quarter_turns }` for every water cell and expose `LevelScene::water_field` (world space via `level_to_world`). River
   elements' flow direction comes from level data (Q-066); `flow_x` is then only used to
   pick the tile rotation.

### zoo-render

3. `shaders.rs`: `water_vs()` (= `static_vs` + `v_world`), `water_fs()` (CEL_FRAGMENT
   variant with the water branch, §4); add `uniform float u_time; uniform vec4 u_bob;` and
   the bobbing transform to `static_vs()` (§ behaviour 8).
4. `renderer.rs`:
   - `Batch` gets `water: bool` (set when the model name starts with `water_`) and
     `bob: [f32; 4]` (set via `set_bob(name, [..])`, default 0).
   - `Renderer::set_water_field(&WaterField)` uploads RGBA16F (`tex_image_2d` with
     `RGBA16F`/`RGBA`/`FLOAT`, LINEAR, CLAMP) to texture unit 2.
   - `render(..., time_s: f64)`: `u_time = water_time(time_s)`; draw non-water batches with
     the static program, then water batches with the water program (one program switch),
     same instanced draw calls. `RenderStats.draw_calls` must not change.
   - Obstacles: `set_water_obstacles(&[(Vec2, f32)])` (level assembly: rocks on river
     cells, Q-068); per frame pick the 4 nearest to the camera target, compute their
     `(s, c)` from the field on the CPU.
   - Water batches keep `edge_mask = 1.0` (waterline crease line).
5. Quality knob (optional): `WaterQuality::{Full, Lite}` — Lite skips flecks, obstacle
   wakes and pond twinkles (≈ −40 % ALU) for weak GPUs (TECH-PLATFORMS).

### zoo-web

- Pass elapsed time (already available in the main loop) to `render`; after building the
  scene call `set_water_field(&scene.water_field)` and set `u_bob` for `duck`,
  `lily_pad`, `frog`. For e2e tests expose a debug time override (`?water_time=` /
  `window.__zoo.setTime(t)`), so screenshots at fixed times are reproducible (WATER-006/007).

### Blender water kit (`tools/blender/props/kit_water.py`)

- Remove the baked `streak`/`arc_streak` polygons and the foam `disc`s from river tiles
  (Q-067): the renderer draws them. River tiles become water polygon + bank, like pond
  tiles; their water faces use only cell `water_river` (176). Update the vertex counts in
  `README_kits_3_6.md` and `check_props_3_6.py`.
- Keep the canonical tile shapes and export the shape constants (M, SLOPE, R) in one place
  (e.g. a comment block the Rust constants cite); the asset test WATER-003 checks the
  analytic shore against the exported water polygons, so a changed shape fails the test.
- `duck`, `frog`, `lily_pad`: origin at the waterline already (y = 0) — required for
  bobbing; nothing to change.
- Preview render (`sample()`): unchanged; it will show plain river tiles.

### Level data / assembly

- River elements get an explicit flow direction (proposal in Q-066:
  `flow = "S"` on `river_n`, `"E"` on `river_e`; `river_mid` inherits), so the centreline
  order and direction are data, not inferred from `notes`.
- Obstacles: rocks placed on river cells (e.g. "small rapids with stones" of `river_n`)
  become foam obstacles (Q-068).

### Cost estimate

- Draw calls: +0 (water tiles are existing batches; one extra program switch).
- Memory: one RGBA16F texture for the joined zoo (≈ 0.9 MB measured); bake estimated < 5 ms,
  measured 17 ms (level 1) / 34 ms (joined zoo, native release) — once at load (see
  Implementation).
- Per water fragment: 1 bilinear RGBA16F fetch (+ the palette fetch already there),
  ≈ 70 ALU (river: streaks ≈ 30, flecks ≈ 20, shore ≈ 15, obstacles ≈ 4 × 8) or ≈ 60 ALU
  (pond). With water covering up to ~25 % of a 1080 × 2340 frame (≈ 0.6 M fragments):
  ≈ 50–60 M ALU ops ≈ 0.3–0.6 ms on an Adreno 6xx/Mali-G5x class GPU → within AENV-010
  (≤ 1 ms); Lite quality halves it. Bobbing: ~20 vertex ALU on ~6 instances — negligible.

## Acceptance criteria

- River and pond in level 1 look like the prototype renders (`art/environment/water/`):
  river streaks follow the river through the bend without seams, pond has no dominant
  motion; reviewers approve (AENV-009).
- No visible seams at tile borders or tile rotations; no outline flicker on water.
- Draw calls unchanged; water costs ≤ 1 ms GPU on the reference phone.

## Test cases

| ID | Given / When / Then | Level |
|---|---|---|
| WATER-001 | Given the baked water field of level 1, then in every river texel the gradient of `s` points along the river element's flow (south in `river_n`, east in `river_e`, rotating monotonically through the bend) and `s` is continuous: neighbouring river texels differ by ≤ 1.6 × texel size × max(1, r/ρ) (incl. the bend and the bridge cells; inside a bend `s` is the arc length at the centreline, so at radius ρ from the arc centre one metre across the flow is r/ρ metres of `s` — the inner side flows slower). The centreline round-trips (`coords(point(s, 0)) = (s, 0)`). (AENV-007 direction) | unit |
| WATER-002 | Given the field, then `c` is continuous across the whole river (same bound as WATER-001), `|c|` ≤ half the river width, and pond texels have `flow = 0`, river texels `flow = 1`. | unit |
| WATER-003 | Given the exported `water_*` tile `.glb` files, then the analytic shore distance used by the bake is 0 (± 1 cm) at every visible-waterline vertex (foot of the soil bank slope), for all four quarter turns; points the bake calls open water lie over water faces and not under the bank, points it calls land lie under bank faces. | asset |
| WATER-004 | Given the Rust mirror of the river streak and pond ring masks, then `mask(p, water_time(t)) == mask(p, water_time(t + 16))` for sampled `p`, `t`, and the image at time `t` is identical whether reached in 60 Hz or 23 Hz steps (time-only function). (AENV-008) | unit |
| WATER-005 | Given the water parameter table (speeds, periods, cycles), then every period divides 16 s and every river lane dash period satisfies `P = 2 s · V`. (AENV-008 loop) | unit |
| WATER-006 | Given level 1 with the time override at t and t + 0.5 s, when screenshots of the river and the pond are compared, then both water areas change; block matching (32 px blocks, the dominant vector = mean of the blocks' best displacements) on the river crop gives a dominant motion vector within 30° of the river direction; the pond crop has no dominant vector (mean vector < 20 % of the river's). (AENV-007) | e2e |
| WATER-007 | Given two screenshots 0.25 s apart, then the number of outline-coloured pixels inside the water areas (excluding a 3 px band at the waterline and prop silhouettes) is 0 in both (no outline flicker on animated water). | e2e |
| WATER-008 | Given level 1 rendered with water animation on and off, then `RenderStats.draw_calls` is equal. (AENV-010 draw calls) | e2e |
| WATER-009 | Given the bobbing function, then the offset of `duck`/`lily_pad`/`frog` stays within `u_bob` amplitudes, two instances at different positions have different phases, a frog and a pad at the same origin have the same phase, other batches get zero offset, and the motion is periodic in 16 s. | unit |
| WATER-010 | Given the kit water tiles, then no face of a `water_river_*` tile uses the cells `water_river_light` (177) or `water_foam` (178) (streaks and foam come from the renderer). | asset |
| WATER-011 | Given in-game screenshots and the GIF of river and pond at zoom 10, 14 and 20 m, then reviewers confirm the comic look (flat bands, hard edges) and "flowing vs. still". (AENV-009) | manual |
| WATER-012 | Given level 1 on the reference mid-range phone, then the GPU time difference between water animation on and off is ≤ 1 ms per frame (timer query where available, otherwise frame-time A/B). (AENV-010) | manual |
| WATER-013 | Given ducks swimming and one duck dipping near the player, then the ripple list for the water shader holds at most 8 entries (the nearest to the player), a swimmer's entry has its position, heading and `wake = speed / 0.6 m/s` clamped to 0…1, a dipping duck's entry has a dip ring age in 0…1 (−1 otherwise), and a duck that stands still with no dip has no wake. (Behaviour 13, GAME-AMBIENT 5) | unit |
| WATER-014 | Given the joined zoo, then the level-2 fountain is drawn with `water_pond` tiles scaled to its 2 × 2 basin at y = 0.61 m, and its field texels are still water (`flow = 0`, `s = c = 0`) with `shore` = distance to the basin rim. (Behaviour 14) | unit |
| WATER-015 | Given close-up review shots of a dipping duck and a swimming frog, then nothing below y = 0 (feet, head, submerged body) is visible through the water and the intersection line carries the comic outline. (Behaviour 15) | manual |
| WATER-016 | Given the level-1 scene, then the water obstacles are exactly the four `bridge_wood` piles (r 0.075, river), the three `river_n` rapids stones (r 0.2, river) and the two jetty posts (r 0.08, still water, ring only); in the joined zoo additionally the water wheel (r 0.3) and the fountain jet (r 0.13, ring only); per frame at most 4 obstacles — the nearest to the camera target — are passed to the shader. (Behaviour 6, Q-068) | unit |

Mapping: AENV-007 → WATER-001, 002, 006; AENV-008 → WATER-004, 005, 009;
AENV-009 → WATER-011; AENV-010 → WATER-008, 012.
Behaviours 6, 13, 14, 15 → WATER-016, 013, 014, 015 (added by the spec manager, FIX-040;
the code tests should carry these IDs — part of 14 is already asserted in
`water_002_zoo_stream_pools_and_fountain`).

## Implementation (M6, 2026-09-26)

- `zoo_core::water`: kit shape constants, `TileShape` (analytic waterline), `river_paths`
  (centreline chain from `flow`, LAYOUT-026), `WaterScene` (tiles, still basins, rivers,
  obstacles — built by `LevelScene::build`), `WaterField::bake` (level 1: 188 × 148
  texels, 17 ms; joined zoo 368 × 308 texels, 34 ms native release — above the 5 ms
  estimate but only once at load; 1.8 MB f32 upload, 0.9 MB RGBA16F on the GPU),
  `water_time`, pattern / bob mirrors for the tests.
- `zoo-render`: `water_vs` / `water_fs` (the prototype's functions + ripples),
  bobbing in `static_vs`, `Renderer::{set_time, set_water_field, set_water_obstacles,
  set_water_ripples, water_animation}`; water tile batches are drawn after the other
  static batches with one program switch. Instanced skinning for the ambient animals
  (`crowd_vs`: one joint-texture row per instance).
- Kit (Q-067): river tiles lost their streak / foam polygons (straight 32 → 2, bank
  43 → 21, curve 94 → 51, inner 42 → 22 triangles); `bridge_wood` got 4 piles (340 → 404).
- Debug API (e2e, GIF): `debug_pause`, `debug_set_time`, `debug_look_at`,
  `set_water_animation`, `water_clock`, `screen_point`.
- Measured (1280 × 720, headless swiftshader, level 1, standing at the bridge, 20 m):
  water animation on vs. off in the same build: **draw calls equal** (WATER-008); ambient
  animals add **4 draw calls** (duck, duckling, frog crowds + butterflies; the removed
  static `duck` / `frog` batches were 2) and **≈ 0 ms CPU** in the A/B of `frame()`
  (0.2 vs. 0.3 ms median, below the noise; AMB-007). Before/after builds (HEAD 50a5d41 vs.
  M6): CPU of `frame()` 0.56–0.74 ms → 1.4–3 ms and CPU + software GPU 160–213 ms →
  460–860 ms per frame, **but** measured on a machine with load average ≈ 33 (other jobs
  running) and with further renderer/camera changes of the same round in the working tree —
  not a clean attribution; swiftshader rasterises on the CPU, so the per-fragment water ALU
  dominates there. The reference-phone GPU time (WATER-012, ≤ 1 ms) is still to be
  measured manually.

## Open questions

- Q-066 answered: `flow` key on river elements (GAME-LAYOUT "Flowing water").
- Q-067 answered: baked streak / foam geometry removed from the river tiles.
- Q-068 answered: foam at bridge posts (piles), stones in the water, the water wheel and the
  jetty posts (behaviour 6).
