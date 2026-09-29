#!/usr/bin/env node
// Turning smoothness (user report 2026-09-28 "looking left or right is slightly flickery";
// specs/50-performance/recommendations.md PERF-R-015…): two measurements of a built web/dist.
//
//   node tools/perf/turn.mjs flicker <dist> <out dir>
//     Deterministic (fake clock, manual requestAnimationFrame, 1/60 s per frame, 960 × 540):
//     each turn scenario is rendered frame by frame while the view turns slowly, and the
//     temporal variation TV = Σ_t |L_t − L_(t−1)| of every pixel's luminance is summed. The
//     same sequence rendered at 2 × 2 supersampling and box-filtered to 960 × 540 is the
//     anti-aliased reference: the excess TV (base − reference) is the flicker that aliasing
//     adds (edges and thin geometry stepping from pixel to pixel). Variants, each with its own
//     reference: `base`, `nolines` (outline pass without lines — separates the outline's
//     share). Writes <out>/turn_flicker.json, a heat map <id>_<variant>_excess.png and the
//     last frame <id>_<variant>_last.png. Every variant is also compared with the `base`
//     reference (`hot_vs_base_ref`: pixels whose TV exceeds the supersampled *current* look
//     by > 255 — the flicker metric of PERF-026; lower = calmer).
//     (TURN_SCENARIOS=T01,T03 subset; TURN_FRAMES (48); LOOK_ANGLE=gl for the real GPU;
//     TURN_VARIANTS=base,nolines subset; TURN_VARIANTS_FILE=<x.mjs> adds variants — its
//     default export maps a name to an init script (e.g. a shaderSource patch);
//     TURN_OWN_REF=0 skips each variant's own 2 × 2 reference (faster exploration);
//     TURN_BASE_REF=<dir> reuses the base references <id>_base_ref.bin of an earlier run —
//     only valid for the same build and assets.)
//   node tools/perf/turn.mjs pacing <dist> [out.json]
//     Real requestAnimationFrame: holds → in first person (4 s) and rotates the zoo view by
//     45° steps; records every frame interval and the view yaw → interval p50 / p95 / max,
//     long frames (> 1.5 × median), and the yaw speed jitter (std of Δyaw / Δt while the turn
//     is steady). (TURN_VIEWPORTS=desktop,phone; LOOK_ANGLE=gl for the real GPU)
import fs from 'node:fs';
import http from 'node:http';
import path from 'node:path';
import zlib from 'node:zlib';
import { createRequire } from 'node:module';
import { fileURLToPath } from 'node:url';

const repo = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const require = createRequire(path.join(repo, 'web/package.json'));
const W = 960;
const H = 540;

/** Turn scenarios (ids stable; add, never change): setup, then `step()` once per frame. */
const SCENARIOS = [
  // first person on the level-1 ring path looking north, turning right:
  // the key is pressed for 2 frames of every 8 (≈ 0.2°/frame average, slow pan)
  ['T01_fp_ring_drag', `a.debug_teleport(-2.5, 20.5); a.debug_face_point(-2.5, 30); a.set_view_mode('first_person');`, `a.look_drag(0.8, 0);`],
  // look-around held at the spawn, mouse drag 0.8 px per frame (0.2°/frame)
  ['T02_look_around_spawn', `spawn('level_1'); a.look_hold(true);`, `a.look_drag(0.8, 0);`],
  // zoo view at the default 14 m: one 45° rotation step (the eased glide)
  ['T03_zoo_rotate_step', `spawn('level_1');`, `if (i === 0) a.rotate(1);`],
  // first person at night in the night zoo (lamp light edges, string lights)
  ['T04_fp_night_zoo', `a.debug_set_daytime('night'); a.debug_teleport(-34.5, 29.5); a.set_view_mode('first_person');`, `a.look_drag(0.8, 0);`],
];

const VARIANTS = {
  base: null,
  // outline pass without lines (colour, fog and sky unchanged)
  nolines: `{
    const src = WebGL2RenderingContext.prototype.shaderSource;
    WebGL2RenderingContext.prototype.shaderSource = function (s, text) {
      return src.call(this, s, text.replace('float edge = max(depth_edge, normal_edge);', 'float edge = 0.0;'));
    };
  }`,
};

const MIME = { '.html': 'text/html', '.js': 'text/javascript', '.wasm': 'application/wasm', '.json': 'application/json', '.css': 'text/css', '.png': 'image/png', '.glb': 'model/gltf-binary', '.ftl': 'text/plain', '.toml': 'text/plain', '.svg': 'image/svg+xml' };

function serve(dir, port) {
  const server = http.createServer((req, res) => {
    const url = decodeURIComponent((req.url ?? '/').split('?')[0]);
    let file = path.join(dir, url.endsWith('/') ? url + 'index.html' : url);
    if (!file.startsWith(dir) || !fs.existsSync(file)) file = path.join(dir, 'index.html');
    res.writeHead(200, { 'Content-Type': MIME[path.extname(file)] ?? 'application/octet-stream' });
    fs.createReadStream(file).pipe(res);
  });
  return new Promise((ok) => server.listen(port, () => ok(server)));
}

const ANGLE = process.env.LOOK_ANGLE ?? 'swiftshader';
const BROWSER_ARGS =
  ANGLE === 'swiftshader'
    ? ['--use-angle=swiftshader', '--enable-unsafe-swiftshader', '--ignore-gpu-blocklist']
    : [`--use-angle=${ANGLE}`, '--enable-gpu', '--ignore-gpu-blocklist'];

async function open(browser, port, { dpr = 1, width = W, height = H, fake = true, init = null }) {
  const page = await browser.newPage({ viewport: { width, height }, deviceScaleFactor: dpr });
  const errors = [];
  page.on('pageerror', (e) => errors.push(String(e)));
  if (init) await page.addInitScript(init);
  await page.addInitScript((fakeClock) => {
    localStorage.clear();
    localStorage.setItem('zoo.readingLevel', 'klasse1');
    window.__realNow = performance.now.bind(performance);
    if (!fakeClock) return;
    let clock = 1000;
    let queue = [];
    performance.now = () => clock;
    window.requestAnimationFrame = (cb) => (queue.push(cb), queue.length);
    window.__tick = (n) => {
      for (let i = 0; i < n; i++) {
        clock += 1000 / 60;
        const q = queue;
        queue = [];
        for (const cb of q) cb(clock);
      }
    };
  }, fake);
  await page.goto(`http://localhost:${port}/?seed=17&quality=high`);
  await page.waitForFunction(() => window.__zoo?.app || window.__zooError, null, { timeout: 120_000, polling: 100 });
  const failed = await page.evaluate(() => window.__zooError ?? null);
  if (failed) throw new Error(failed);
  return { page, errors };
}

const SETUP_HELPERS = `
  const spawn = (l) => { const s = a.level_spawn(l); a.debug_teleport(s[0], s[1]); };
`;

/** Renders `frames` frames of a scenario; returns per-pixel TV at 960 × 540 (box-filtered). */
async function sequence(page, setup, step, frames, dpr) {
  return page.evaluate(
    ([setup, step, frames, dpr, W, H]) => {
      const a = window.__zoo.app;
      new Function('a', `${'const spawn = (l) => { const s = a.level_spawn(l); a.debug_teleport(s[0], s[1]); };'} ${setup}`)(a);
      for (let i = 0; i < 20; i++) window.__tick(1); // settle (glides, doors)
      const canvas = document.getElementById('game');
      const gl = canvas.getContext('webgl2');
      const cw = canvas.width;
      const ch = canvas.height;
      const px = new Uint8Array(cw * ch * 4);
      const lum = () => {
        gl.readPixels(0, 0, cw, ch, gl.RGBA, gl.UNSIGNED_BYTE, px);
        const out = new Float32Array(W * H);
        const k = cw / W;
        for (let y = 0; y < H; y++) {
          for (let x = 0; x < W; x++) {
            let s = 0;
            for (let dy = 0; dy < k; dy++) {
              for (let dx = 0; dx < k; dx++) {
                const i = ((y * k + dy) * cw + x * k + dx) * 4;
                s += 0.299 * px[i] + 0.587 * px[i + 1] + 0.114 * px[i + 2];
              }
            }
            out[y * W + x] = s / (k * k);
          }
        }
        return out;
      };
      const stepFn = new Function('a', 'i', step);
      const tv = new Float32Array(W * H);
      let prev = null;
      let yaw0 = a.camera_view_yaw_deg();
      for (let i = 0; i < frames; i++) {
        stepFn(a, i);
        window.__tick(1);
        const l = lum();
        if (prev) for (let p = 0; p < l.length; p++) tv[p] += Math.abs(l[p] - prev[p]);
        prev = l;
      }
      return { tv: Array.from(tv), turned_deg: a.camera_view_yaw_deg() - yaw0, canvas: [cw, ch] };
    },
    [setup, step, frames, dpr, W, H],
  );
}

function png(w, h, rgba) {
  const crcTable = Array.from({ length: 256 }, (_, n) => {
    let c = n;
    for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
    return c >>> 0;
  });
  const crc = (buf) => {
    let c = 0xffffffff;
    for (const x of buf) c = crcTable[(c ^ x) & 0xff] ^ (c >>> 8);
    return (c ^ 0xffffffff) >>> 0;
  };
  const chunk = (type, data) => {
    const len = Buffer.alloc(4);
    len.writeUInt32BE(data.length);
    const td = Buffer.concat([Buffer.from(type), data]);
    const c = Buffer.alloc(4);
    c.writeUInt32BE(crc(td));
    return Buffer.concat([len, td, c]);
  };
  const ihdr = Buffer.alloc(13);
  ihdr.writeUInt32BE(w, 0);
  ihdr.writeUInt32BE(h, 4);
  ihdr[8] = 8;
  ihdr[9] = 6;
  const raw = Buffer.alloc((w * 4 + 1) * h);
  for (let y = 0; y < h; y++) rgba.copy(raw, y * (w * 4 + 1) + 1, y * w * 4, (y + 1) * w * 4);
  return Buffer.concat([
    Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]),
    chunk('IHDR', ihdr),
    chunk('IDAT', zlib.deflateSync(raw)),
    chunk('IEND', Buffer.alloc(0)),
  ]);
}

/** Heat map of the excess TV (readPixels rows are bottom first). */
function heat(excess, scale) {
  const out = Buffer.alloc(W * H * 4);
  for (let y = 0; y < H; y++) {
    for (let x = 0; x < W; x++) {
      const v = Math.min(1, excess[(H - 1 - y) * W + x] / scale);
      const o = (y * W + x) * 4;
      out[o] = Math.round(255 * Math.min(1, v * 2));
      out[o + 1] = Math.round(255 * Math.max(0, v * 2 - 1));
      out[o + 2] = 0;
      out[o + 3] = 255;
    }
  }
  return png(W, H, out);
}

function selected() {
  const only = process.env.TURN_SCENARIOS?.split(',');
  return SCENARIOS.filter(([id]) => !only || only.some((o) => id.startsWith(o)));
}

async function variants() {
  const all = { ...VARIANTS };
  if (process.env.TURN_VARIANTS_FILE) {
    const extra = await import(path.resolve(process.env.TURN_VARIANTS_FILE));
    Object.assign(all, extra.default);
  }
  const only = process.env.TURN_VARIANTS?.split(',');
  return Object.entries(all).filter(([name]) => !only || only.includes(name));
}

async function flicker(dist, out) {
  const { chromium } = require('playwright');
  const port = Number(process.env.LOOK_PORT ?? 4192);
  const frames = Number(process.env.TURN_FRAMES ?? 48);
  const ownRef = process.env.TURN_OWN_REF !== '0';
  const refDir = process.env.TURN_BASE_REF;
  const server = await serve(path.resolve(dist), port);
  fs.mkdirSync(out, { recursive: true });
  const browser = await chromium.launch({ args: BROWSER_ARGS });
  const result = {};
  const run = async (id, setup, step, init, dpr, shot) => {
    const { page, errors } = await open(browser, port, { dpr, init });
    const r = await sequence(page, setup, step, frames, dpr);
    if (errors.length) console.log(`${id} dpr ${dpr}: ${errors.join(' | ')}`);
    if (shot) fs.writeFileSync(shot, await page.screenshot());
    await page.close();
    return r;
  };
  try {
    const list = await variants();
    for (const [id, setup, step] of selected()) {
      // reference of the current look: `base` at 2 × 2 supersampling
      const refFile = path.join(out, `${id}_base_ref.bin`);
      let baseRef;
      const cached = refDir && path.join(refDir, `${id}_base_ref.bin`);
      if (cached && fs.existsSync(cached)) {
        baseRef = new Float32Array(new Uint8Array(fs.readFileSync(cached)).buffer);
      } else {
        baseRef = Float32Array.from((await run(id, setup, step, VARIANTS.base, 2, null)).tv);
      }
      fs.writeFileSync(refFile, Buffer.from(baseRef.buffer));
      for (const [variant, init] of list) {
        const b = (await run(id, setup, step, init, 1, path.join(out, `${id}_${variant}_last.png`))).tv;
        const own = variant === 'base' ? baseRef : ownRef ? (await run(id, setup, step, init, 2, null)).tv : null;
        let tb = 0;
        let tr = 0;
        let ex = 0;
        let hot = 0;
        let hotBase = 0;
        const excess = new Float32Array(W * H);
        for (let p = 0; p < b.length; p++) {
          tb += b[p];
          if (b[p] - baseRef[p] > 255) hotBase++;
          const e = Math.max(0, b[p] - (own ? own[p] : baseRef[p]));
          excess[p] = e;
          ex += e;
          if (own) {
            tr += own[p];
            if (e > 255) hot++;
          }
        }
        const row = {
          frames,
          tv_per_frame: tb / (frames - 1) / (W * H),
          hot_vs_base_ref: hotBase,
        };
        if (own) {
          Object.assign(row, {
            tv_ref_per_frame: tr / (frames - 1) / (W * H),
            tv_ratio: tb / tr,
            excess_share: ex / tb,
            hot_pixels: hot,
          });
        }
        result[`${id} ${variant}`] = row;
        fs.writeFileSync(path.join(out, `${id}_${variant}_excess.png`), heat(excess, 1024));
        console.log(
          `${id} ${variant}: TV/px/frame ${row.tv_per_frame.toFixed(3)}, hot vs base SSAA ${hotBase}` +
            (own
              ? `; own SSAA ${row.tv_ref_per_frame.toFixed(3)} (× ${row.tv_ratio.toFixed(2)}), aliasing excess ${(100 * row.excess_share).toFixed(1)} %, ${hot} hot px`
              : ''),
        );
      }
    }
  } finally {
    await browser.close();
    server.close();
  }
  fs.writeFileSync(path.join(out, 'turn_flicker.json'), JSON.stringify(result, null, 1));
}

async function pacing(dist, out) {
  const { chromium } = require('playwright');
  const port = Number(process.env.LOOK_PORT ?? 4192);
  const server = await serve(path.resolve(dist), port);
  const browser = await chromium.launch({ args: BROWSER_ARGS });
  const viewports = {
    desktop: { width: 1920, height: 1080, dpr: 1 },
    phone: { width: 360, height: 780, dpr: 3 },
  };
  const result = {};
  const q = (v, f) => [...v].sort((x, y) => x - y)[Math.min(v.length - 1, Math.floor(f * v.length))];
  try {
    for (const vp of (process.env.TURN_VIEWPORTS ?? 'desktop,phone').split(',')) {
      const { page } = await open(browser, port, { ...viewports[vp], fake: false });
      result.renderer ??= await page.evaluate(() => {
        const gl = document.getElementById('game').getContext('webgl2');
        const d = gl.getExtension('WEBGL_debug_renderer_info');
        return d ? gl.getParameter(d.UNMASKED_RENDERER_WEBGL) : gl.getParameter(gl.RENDERER);
      });
      for (const [name, setup, action, seconds] of [
        ['first person, → held', `a.debug_teleport(-2.5, 20.5); a.debug_face_point(-2.5, 30); a.set_view_mode('first_person');`, `a.key('ArrowRight', true)`, 4],
        ['zoo view, 45° steps', `spawn('level_1'); a.set_view_mode('zoo');`, `a.rotate(1)`, 4],
      ]) {
        const r = await page.evaluate(
          ([setup, action, seconds, helpers]) =>
            new Promise((done) => {
              const a = window.__zoo.app;
              new Function('a', `${helpers} ${setup}`)(a);
              const samples = [];
              let t0 = null;
              let lastRotate = 0;
              const tick = (t) => {
                if (t0 === null) {
                  t0 = t;
                  new Function('a', action)(a);
                }
                if (action.includes('rotate') && t - lastRotate > 1000) {
                  new Function('a', action)(a);
                  lastRotate = t;
                }
                samples.push([t, a.camera_view_yaw_deg()]);
                if (t - t0 < seconds * 1000) requestAnimationFrame(tick);
                else {
                  a.key('ArrowRight', false);
                  done(samples);
                }
              };
              // wait a few frames first (text textures, first uploads)
              let n = 0;
              const warm = () => (++n < 30 ? requestAnimationFrame(warm) : requestAnimationFrame(tick));
              requestAnimationFrame(warm);
            }),
          [setup, action, seconds, SETUP_HELPERS],
        );
        const dts = [];
        const speeds = [];
        for (let i = 1; i < r.length; i++) {
          const dt = r[i][0] - r[i - 1][0];
          dts.push(dt);
          let dy = r[i][1] - r[i - 1][1];
          dy -= 360 * Math.round(dy / 360);
          if (dt > 0) speeds.push(dy / (dt / 1000));
        }
        const med = q(dts, 0.5);
        // steady part of the key turn: the middle half
        const mid = speeds.slice(Math.floor(speeds.length / 4), Math.floor((3 * speeds.length) / 4));
        const mean = mid.reduce((s, v) => s + v, 0) / Math.max(1, mid.length);
        const std = Math.sqrt(mid.reduce((s, v) => s + (v - mean) ** 2, 0) / Math.max(1, mid.length));
        const row = {
          frames: dts.length,
          interval_p50_ms: med,
          interval_p95_ms: q(dts, 0.95),
          interval_max_ms: Math.max(...dts),
          long_frames: dts.filter((d) => d > 1.5 * med).length,
          yaw_speed_mean_deg_s: mean,
          yaw_speed_jitter: mean !== 0 ? std / Math.abs(mean) : null,
        };
        result[`${vp} ${name}`] = row;
        console.log(`${vp} ${name}: ${JSON.stringify(row)}`);
      }
      await page.close();
    }
  } finally {
    await browser.close();
    server.close();
  }
  console.log(`renderer: ${result.renderer}`);
  if (out) fs.writeFileSync(out, JSON.stringify(result, null, 1));
}

const [cmd, a, b] = process.argv.slice(2);
if (cmd === 'flicker' && a && b) {
  await flicker(a, b);
} else if (cmd === 'pacing' && a) {
  await pacing(a, b);
} else {
  console.error('usage: turn.mjs flicker <dist> <out dir> | pacing <dist> [out.json]');
  process.exit(2);
}
