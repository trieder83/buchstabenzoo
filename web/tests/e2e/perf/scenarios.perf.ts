// Fixed performance scenarios (PERF-BUDGETS, specs/50-performance/measurements.md).
// Run with tools/perf/run.sh (or `npx playwright test -c tests/e2e/perf/perf.config.ts`
// against a built web/dist). Scenario ids are stable: add new ones, never change old ones.
//
// Per viewport one page load (load time, downloaded bytes, WASM heap), then per scenario:
//  - timing: PERF_FRAMES frames of the real rAF loop — CPU time of `app.frame` (simulation +
//    GL command submission) and the frame interval (throughput incl. rasterisation);
//  - counters of the last frame: RenderStats draw calls / instances / triangles, culled
//    batches, decals, crowd draw calls, point lights / light pools;
//  - GL census (3 frames, WebGL2 prototype wrapped only meanwhile): calls per frame by kind
//    (draws, program switches, texture binds, uniform calls, uploads + bytes);
//  - pass split (3 frames, 1-pixel `readPixels` fences — `gl.finish()` does not block in Chrome): scene pass vs. the full-screen outline /
//    fog / sky pass (the only `drawArrays(TRIANGLES, 0, 3)`);
//  - simulation only: `debug_step` (game + animals + ambient, no rendering) per 1/60 s step.
// Under SwiftShader (software GL) absolute ms are not representative — compare runs.
import fs from 'node:fs';
import os from 'node:os';
import { test, type Page } from '@playwright/test';

const FRAMES = Number(process.env.PERF_FRAMES ?? 16);
const OUT = process.env.PERF_OUT ?? 'test-results/perf/browser.json';

interface Viewport {
  id: string;
  width: number;
  height: number;
  dpr: number;
  scenarios: string[] | 'all';
}

// phone = 1080 × 2340 device px portrait (CSS 360 × 780 at DPR 3; the renderer caps the
// pixel ratio at 2 → 720 × 1560 drawing buffer); desktop_half = pixel-scaling diagnostic.
const VIEWPORTS: Viewport[] = [
  { id: 'desktop', width: 1920, height: 1080, dpr: 1, scenarios: 'all' },
  { id: 'phone', width: 360, height: 780, dpr: 3, scenarios: 'all' },
  { id: 'desktop_half', width: 960, height: 540, dpr: 1, scenarios: ['S01', 'S02', 'S10'] },
];

type Setup = (page: Page) => Promise<void>;

/* eslint-disable @typescript-eslint/no-explicit-any */
const ev = <T>(page: Page, fn: (arg: any) => T, arg?: unknown) => page.evaluate(fn as any, arg) as Promise<T>;

/** Runs `n` frames synchronously with dt (camera glides, doors, animals settle). */
async function advance(page: Page, n = 12, dt = 0.1) {
  await ev(page, ([k, d]: [number, number]) => {
    const a = (window as any).__zoo.app;
    for (let i = 0; i < k; i++) a.frame(d);
  }, [n, dt]);
}

async function reset(page: Page) {
  await ev(page, () => {
    const a = (window as any).__zoo.app;
    a.look_hold(false);
    a.set_view_mode('zoo');
    a.set_water_animation(true);
    a.set_ambient(true);
    a.zoom(0.001); // 10 m
    a.zoom(1.4); // 14 m (default)
  });
}

async function teleport(page: Page, x: number, z: number) {
  await ev(page, ([px, pz]: [number, number]) => (window as any).__zoo.app.debug_teleport(px, pz), [x, z]);
}

async function standNear(page: Page, x: number, z: number) {
  const p = await ev<number[]>(page, ([px, pz]: [number, number]) => (window as any).__zoo.app.debug_stand_near_point(px, pz, 1.5), [x, z]);
  if (p.length === 2) await teleport(page, p[0], p[1]);
}

const spawn = async (page: Page) => {
  const s = await ev<number[]>(page, () => (window as any).__zoo.app.level_spawn('level_1'));
  await teleport(page, s[0], s[1]);
};

/** Fixed scenario list (id → description, setup). Keep ids and setups stable. */
const SCENARIOS: [string, string, Setup][] = [
  ['S01', 'level-1 spawn, zoo view, default zoom (14 m)', async (p) => {
    await reset(p);
    await spawn(p);
  }],
  ['S02', 'level-1 spawn, zoo view, max zoom-out (20 m)', async (p) => {
    await reset(p);
    await spawn(p);
    await ev(p, () => (window as any).__zoo.app.zoom(100));
  }],
  ['S03', 'walking (autopilot) from the level-1 spawn north along the ring path', async (p) => {
    await reset(p);
    await spawn(p);
    await ev(p, () => (window as any).__zoo.app.debug_goto(-2.5, 30.5));
  }],
  ['S04', 'look-around on the level-1 ring path (-2.5, 20.5) looking north', async (p) => {
    await reset(p);
    await teleport(p, -2.5, 20.5);
    await ev(p, () => (window as any).__zoo.app.debug_face_point(-2.5, 30));
    await ev(p, () => (window as any).__zoo.app.look_hold(true));
  }],
  ['S05', 'first person on the level-1 ring path (-2.5, 20.5) looking north', async (p) => {
    await reset(p);
    await teleport(p, -2.5, 20.5);
    await ev(p, () => (window as any).__zoo.app.debug_face_point(-2.5, 30));
    await ev(p, () => (window as any).__zoo.app.set_view_mode('first_person'));
  }],
  ['S06', 'inside zookeeper_house_1 (roof hidden), zoo view', async (p) => {
    await reset(p);
    await teleport(p, -11.5, 2.5);
  }],
  ['S07', 'vegetable garden garden_veg (level 1)', async (p) => {
    await reset(p);
    await standNear(p, 7.5, 38.5);
  }],
  ['S08', 'pond with ducks (level 1), water animation + ambient on', async (p) => {
    await reset(p);
    await standNear(p, -15.0, 19.5);
  }],
  ['S09', 'level-3 spawn, max zoom-out (joined zoo worst case, levels locked)', async (p) => {
    await reset(p);
    const s = await ev<number[]>(p, () => (window as any).__zoo.app.level_spawn('level_3'));
    await teleport(p, s[0], s[1]);
    await ev(p, () => (window as any).__zoo.app.zoom(100));
  }],
  ['S10', 'night at the level-1 spawn (lamps, glow, lantern)', async (p) => {
    await reset(p);
    await ev(p, () => (window as any).__zoo.app.debug_set_daytime('night'));
    await spawn(p);
  }],
  ['S11', 'night zoo night_1 behind the moon door (-34.5, 29.5)', async (p) => {
    await reset(p);
    await ev(p, () => (window as any).__zoo.app.debug_set_daytime('night'));
    await teleport(p, -34.5, 29.5);
  }],
];

/** A/B variants measured right after a scenario (same place): name → toggle. */
const VARIANTS: Record<string, [string, (page: Page) => Promise<void>][]> = {
  S08: [
    ['water_off', (p) => ev(p, () => (window as any).__zoo.app.set_water_animation(false))],
    ['ambient_off', (p) => ev(p, () => (window as any).__zoo.app.set_ambient(false))],
  ],
};

/** Installs the frame sampler and the WASM memory hook (before the page loads). */
async function instrument(page: Page) {
  await page.addInitScript(() => {
    localStorage.clear();
    localStorage.setItem('zoo.readingLevel', 'klasse1');
    const w = window as any;
    const inst = WebAssembly.instantiate;
    (WebAssembly as any).instantiate = async (...args: any[]) => {
      const r: any = await (inst as any)(...args);
      const exp = (r.instance ?? r).exports;
      if (exp?.memory) w.__wasmMemory = exp.memory;
      return r;
    };
    const stream = WebAssembly.instantiateStreaming;
    if (stream) {
      (WebAssembly as any).instantiateStreaming = async (...args: any[]) => {
        const r: any = await (stream as any)(...args);
        if (r.instance?.exports?.memory) w.__wasmMemory = r.instance.exports.memory;
        return r;
      };
    }
  });
}

async function hookFrames(page: Page) {
  await ev(page, () => {
    const w = window as any;
    const app = w.__zoo.app;
    const orig = Object.getPrototypeOf(app).frame;
    const gl = (document.getElementById('game') as HTMLCanvasElement).getContext('webgl2')!;
    const px = new Uint8Array(4);
    const P = (w.__perf = { cpu: [] as number[], sync: [] as number[], iv: [] as number[], last: 0, on: false });
    app.frame = function (dt: number) {
      const t0 = performance.now();
      orig.call(this, dt);
      const t1 = performance.now();
      if (P.on) {
        // CPU + GPU of the frame, serialised by a 1-pixel readback (gl.finish() does not block in Chrome) (under SwiftShader the GPU is the CPU;
        // without it the rAF interval only shows how far the GPU queue has backed up)
        gl.readPixels(0, 0, 1, 1, gl.RGBA, gl.UNSIGNED_BYTE, px); // blocks until drawn
        P.cpu.push(t1 - t0);
        P.sync.push(performance.now() - t0);
        if (P.last) P.iv.push(t0 - P.last);
      }
      P.last = t0;
    };
  });
}

function stat(v: number[]) {
  const s = [...v].sort((a, b) => a - b);
  const q = (p: number) => s[Math.min(s.length - 1, Math.round((s.length - 1) * p))] ?? 0;
  return { p50: q(0.5), p95: q(0.95), max: q(1), n: s.length };
}

async function sample(page: Page) {
  await ev(page, () => {
    const P = (window as any).__perf;
    P.cpu = [];
    P.sync = [];
    P.iv = [];
    P.last = 0;
    P.on = true;
  });
  await page.waitForFunction((n) => (window as any).__perf.cpu.length >= n, FRAMES, { timeout: 600_000, polling: 100 });
  const raw = await ev<{ cpu: number[]; sync: number[]; iv: number[] }>(page, () => {
    const P = (window as any).__perf;
    P.on = false;
    return { cpu: P.cpu, sync: P.sync, iv: P.iv };
  });
  const counters = await ev<Record<string, unknown>>(page, () => {
    const a = (window as any).__zoo.app;
    const canvas = document.getElementById('game') as HTMLCanvasElement;
    return {
      draw_calls: a.draw_calls(),
      instances: a.instances(),
      triangles: a.triangles(),
      culled_batches: a.culled_batches(),
      decals: a.decals_drawn(),
      crowd_draw_calls: a.ambient_draw_calls(),
      lights: Array.from(a.light_stats() as Uint32Array),
      view: a.view_mode(),
      camera_distance: a.camera_distance(),
      player: [a.player_x(), a.player_z()],
      canvas: [canvas.width, canvas.height],
      daytime: a.daytime(),
    };
  });
  return { cpu_ms: stat(raw.cpu), frame_ms: stat(raw.sync), interval_ms: stat(raw.iv), ...counters };
}

/** GL call census and the scene / full-screen pass split over 3 direct frames. */
async function census(page: Page) {
  return ev(page, () => {
    const w = window as any;
    const app = w.__zoo.app;
    const canvas = document.getElementById('game') as HTMLCanvasElement;
    const gl = canvas.getContext('webgl2') as WebGL2RenderingContext;
    const proto = WebGL2RenderingContext.prototype as any;
    const px = new Uint8Array(4);
    const sync = () => gl.readPixels(0, 0, 1, 1, gl.RGBA, gl.UNSIGNED_BYTE, px); // blocks until drawn
    const C: any = { counts: {} as Record<string, number>, bytes: {} as Record<string, number>, split: false, tPre: 0, tPost: 0 };
    const saved: [string, any][] = [];
    for (const name of Object.getOwnPropertyNames(proto)) {
      const d = Object.getOwnPropertyDescriptor(proto, name);
      if (!d || typeof d.value !== 'function' || name === 'constructor' || name === 'finish' || name === 'readPixels' || name.startsWith('get')) continue;
      const orig = d.value;
      saved.push([name, orig]);
      proto[name] = function (...args: any[]) {
        C.counts[name] = (C.counts[name] ?? 0) + 1;
        if (name === 'bufferSubData' || name === 'bufferData' || name === 'texSubImage2D' || name === 'texImage2D') {
          const v = args.find((x) => ArrayBuffer.isView(x));
          if (v) C.bytes[name] = (C.bytes[name] ?? 0) + (v as ArrayBufferView).byteLength;
        }
        if (C.split && name === 'drawArrays' && args[0] === 4 && args[1] === 0 && args[2] === 3) {
          sync();
          C.tPre = performance.now();
          const r = orig.apply(this, args);
          sync();
          C.tPost = performance.now();
          return r;
        }
        return orig.apply(this, args);
      };
    }
    const frames = 3;
    for (let i = 0; i < frames; i++) app.frame(1 / 60);
    const counts = { ...C.counts }; // copies: the split frames below must not count
    const bytes = { ...C.bytes };
    // pass split (fences change the timing a little; relative numbers)
    C.split = true;
    const scene: number[] = [];
    const post: number[] = [];
    for (let i = 0; i < 3; i++) {
      sync();
      const t0 = performance.now();
      app.frame(1 / 60);
      scene.push(C.tPre - t0);
      post.push(C.tPost - C.tPre);
    }
    C.split = false;
    for (const [name, orig] of saved) proto[name] = orig;
    // simulation only (no rendering): 30 steps of 1/60 s
    const t0 = performance.now();
    app.debug_step(0.5);
    const simMs = (performance.now() - t0) / 30;
    const per = (k: string) => (counts[k] ?? 0) / frames;
    const sum = (f: (k: string) => boolean) => Object.keys(counts).filter(f).reduce((s, k) => s + counts[k], 0) / frames;
    const med = (v: number[]) => [...v].sort((a, b) => a - b)[1];
    return {
      gl_calls: sum(() => true),
      draws: per('drawElements') + per('drawElementsInstanced') + per('drawArrays'),
      use_program: per('useProgram'),
      bind_texture: per('bindTexture'),
      bind_vao: per('bindVertexArray'),
      uniform_calls: sum((k) => k.startsWith('uniform')),
      tex_sub_image: per('texSubImage2D'),
      tex_image: per('texImage2D'),
      buffer_sub_data: per('bufferSubData'),
      buffer_data: per('bufferData'),
      upload_bytes: Object.values(bytes as Record<string, number>).reduce((s, v) => s + v, 0) / frames,
      scene_pass_ms: med(scene),
      fullscreen_pass_ms: med(post),
      sim_step_ms: simMs,
      // static batches drawn (model@region xN instances, triangles) — batching coverage
      draw_list: String(app.debug_draw_list()).split('\n').filter(Boolean),
    };
  });
}

async function loadMetrics(page: Page) {
  return ev(page, () => {
    const w = window as any;
    const nav = performance.getEntriesByType('navigation')[0] as PerformanceNavigationTiming | undefined;
    const res = performance.getEntriesByType('resource') as PerformanceResourceTiming[];
    const gl = (document.getElementById('game') as HTMLCanvasElement).getContext('webgl2')!;
    const dbg = gl.getExtension('WEBGL_debug_renderer_info');
    const mem = (performance as any).memory;
    return {
      first_frame_ms: w.__firstFrameAt ?? null,
      dom_content_loaded_ms: nav ? nav.domContentLoadedEventEnd : null,
      resources: res.length,
      transfer_bytes: res.reduce((s, r) => s + (r.transferSize || 0), 0) + (nav?.transferSize ?? 0),
      decoded_bytes: res.reduce((s, r) => s + (r.decodedBodySize || 0), 0) + (nav?.decodedBodySize ?? 0),
      wasm_memory_bytes: w.__wasmMemory ? w.__wasmMemory.buffer.byteLength : null,
      js_heap_used_bytes: mem ? mem.usedJSHeapSize : null,
      renderer: dbg ? gl.getParameter(dbg.UNMASKED_RENDERER_WEBGL) : gl.getParameter(gl.RENDERER),
      user_agent: navigator.userAgent,
    };
  });
}

/**
 * Merges one viewport's results into PERF_OUT right away (a failed test restarts the worker
 * and would lose module state; a partial viewport is still written).
 */
function save(id: string, data: unknown) {
  fs.mkdirSync(OUT.replace(/\/[^/]*$/, ''), { recursive: true });
  const all = fs.existsSync(OUT) ? JSON.parse(fs.readFileSync(OUT, 'utf8')) : { viewports: {} };
  all.frames_per_sample = FRAMES;
  all.viewports[id] = data;
  fs.writeFileSync(OUT, JSON.stringify(all, null, 2));
}

// PERF_VIEWPORTS=desktop,phone runs a subset (the others stay in PERF_OUT).
const only = process.env.PERF_VIEWPORTS?.split(',');
for (const vp of VIEWPORTS.filter((v) => !only || only.includes(v.id))) {
  test(`perf ${vp.id} ${vp.width}x${vp.height}@${vp.dpr}`, async ({ browser }) => {
    const context = await browser.newContext({
      viewport: { width: vp.width, height: vp.height },
      deviceScaleFactor: vp.dpr,
      isMobile: false,
    });
    const page = await context.newPage();
    const errors: string[] = [];
    page.on('pageerror', (e) => errors.push(String(e)));
    page.on('console', (m) => {
      if (m.type() === 'error') errors.push(m.text());
    });
    await instrument(page);
    const t0 = Date.now();
    await page.goto('/?seed=17');
    await page.waitForFunction(() => Boolean((window as any).__zooError) || ((window as any).__zoo?.frames ?? 0) >= 1, null, {
      timeout: 300_000,
      polling: 50,
    });
    const firstFrameMs = Date.now() - t0;
    await hookFrames(page);
    const load = { ...(await loadMetrics(page)), first_frame_wall_ms: firstFrameMs };
    const out: Record<string, unknown> = { viewport: vp, load, scenarios: {}, errors, loadavg_start: os.loadavg(), cpus: os.cpus().length };
    const onlyS = process.env.PERF_SCENARIOS?.split(','); // e.g. S01,S09
    for (const [id, desc, setup] of SCENARIOS) {
      if (vp.scenarios !== 'all' && !vp.scenarios.includes(id)) continue;
      if (onlyS && !onlyS.includes(id)) continue;
      await setup(page);
      await advance(page, id === 'S03' ? 2 : 8);
      const s: Record<string, unknown> = { desc, ...(await sample(page)) };
      s.census = await census(page);
      for (const [name, toggle] of VARIANTS[id] ?? []) {
        await toggle(page);
        await advance(page, 4);
        s[name] = await sample(page);
      }
      (out.scenarios as Record<string, unknown>)[id] = s;
      save(vp.id, out);
      const p50 = (k: string) => (s[k] as { p50: number }).p50.toFixed(1);
      console.log(`${vp.id} ${id} dc=${s.draw_calls} cpu=${p50('cpu_ms')} frame=${p50('frame_ms')} iv=${p50('interval_ms')} load=${os.loadavg()[0].toFixed(1)}`);
    }
    out.load_after = await loadMetrics(page);
    out.loadavg_end = os.loadavg();
    save(vp.id, out);
    await context.close();
  });
}
