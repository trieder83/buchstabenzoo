#!/usr/bin/env node
// Look check for "no visible change" optimisations (PERF-BUDGETS rule 4,
// specs/50-performance/recommendations.md): renders fixed scenarios of a built web/dist
// deterministically and compares two runs pixel by pixel.
//
//   node tools/perf/look.mjs capture <dist dir> <out dir>   # raw RGBA + PNG per scenario
//     (LOOK_SCENARIOS=L08,L10 captures a subset; LOOK_PORT, default 4191)
//   node tools/perf/look.mjs compare <out dir A> <out dir B> [max channel diff, default 2]
//     (LOOK_DIFF_DIR=dir: writes <id>_diff.png per differing scenario — B dimmed to grey,
//      differing pixels red, brighter = larger difference)
//   node tools/perf/look.mjs ab <dist A> <dist B> [out.json]   # interleaved frame-time A/B
//     (AB_VIEWPORTS=desktop,phone, AB_ROUNDS=8, AB_FRAMES=3; relative under SwiftShader;
//      AB_INIT_B=file.js: an init script for the B pages, e.g. a shaderSource patch to
//      try a GLSL variant without rebuilding)
//
// Determinism: `?seed=17`, fresh save, a fake clock (`performance.now`) and a manual
// `requestAnimationFrame` queue — every frame advances exactly 1/60 s, so two captures of
// the same build are identical (checked by capturing a build twice). Headless Chromium with
// SwiftShader (software WebGL). The canvas is read with `readPixels` right after a frame.
// Uses Playwright from web/node_modules; serves the dist with a tiny static server on
// LOOK_PORT (default 4191), stopped at the end.
import fs from 'node:fs';
import http from 'node:http';
import zlib from 'node:zlib';
import path from 'node:path';
import { createRequire } from 'node:module';
import { fileURLToPath } from 'node:url';

const repo = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const require = createRequire(path.join(repo, 'web/package.json'));

const W = 960;
const H = 540;

/** Fixed look scenarios (ids stable; add, never change). Setup runs on `window.__zoo.app`. */
const SCENARIOS = [
  ['L01_day_spawn', `a.set_view_mode('zoo'); spawn('level_1');`],
  ['L02_day_spawn_20m', `a.set_view_mode('zoo'); spawn('level_1'); a.zoom(100);`],
  ['L03_day_look_around', `a.debug_teleport(-2.5, 20.5); a.debug_face_point(-2.5, 30); a.look_hold(true);`],
  ['L04_day_first_person', `a.debug_teleport(-2.5, 20.5); a.debug_face_point(-2.5, 30); a.set_view_mode('first_person');`],
  ['L05_day_house', `a.debug_teleport(-11.5, 2.5);`],
  ['L06_day_pond', `near(-15.0, 19.5);`],
  ['L07_dusk_spawn', `a.debug_set_daytime('dusk'); spawn('level_1');`],
  ['L08_night_spawn', `a.debug_set_daytime('night'); spawn('level_1');`],
  ['L09_night_spawn_20m', `a.debug_set_daytime('night'); spawn('level_1'); a.zoom(100);`],
  ['L10_night_zoo', `a.debug_set_daytime('night'); a.debug_teleport(-34.5, 29.5);`],
  ['L11_night_pond', `a.debug_set_daytime('night'); near(-15.0, 19.5);`],
  ['L12_night_first_person', `a.debug_set_daytime('night'); a.debug_teleport(-2.5, 20.5); a.debug_face_point(-2.5, 30); a.set_view_mode('first_person');`],
  ['L13_night_look_around', `a.debug_set_daytime('night'); a.debug_teleport(-34.5, 29.5); a.look_hold(true);`],
  ['L14_night_house', `a.debug_set_daytime('night'); a.debug_teleport(-11.5, 2.5);`],
];

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

// LOOK_ANGLE=gl | vulkan: the real GPU instead of SwiftShader (like PERF_ANGLE of run.sh)
const ANGLE = process.env.LOOK_ANGLE ?? 'swiftshader';
const BROWSER_ARGS =
  ANGLE === 'swiftshader'
    ? ['--use-angle=swiftshader', '--enable-unsafe-swiftshader', '--ignore-gpu-blocklist']
    : [`--use-angle=${ANGLE}`, '--enable-gpu', '--ignore-gpu-blocklist'];

/** Opens a fresh page of the game on `port`, runs a scenario setup and lets it settle. */
async function openScenario(browser, port, id, setup, viewport = { width: W, height: H, dpr: 1 }, initFile = null) {
  const page = await browser.newPage({ viewport: { width: viewport.width, height: viewport.height }, deviceScaleFactor: viewport.dpr });
  if (initFile) await page.addInitScript({ path: path.resolve(initFile) });
  const errors = [];
  page.on('pageerror', (e) => errors.push(String(e)));
  page.on('console', (m) => m.type() === 'error' && errors.push(m.text()));
  await page.addInitScript(() => {
    localStorage.clear();
    localStorage.setItem('zoo.readingLevel', 'klasse1');
    window.__realNow = performance.now.bind(performance);
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
  });
  await page.goto(`http://localhost:${port}/?seed=17`);
  // (polling by interval: requestAnimationFrame is replaced)
  await page.waitForFunction(() => window.__zoo?.app || window.__zooError, null, { timeout: 120_000, polling: 100 });
  const failed = await page.evaluate(() => window.__zooError ?? null);
  if (failed) throw new Error(`${id}: ${failed}`);
  await page.evaluate(([code]) => {
    const a = window.__zoo.app;
    const spawn = (l) => {
      const s = a.level_spawn(l);
      a.debug_teleport(s[0], s[1]);
    };
    const near = (x, z) => {
      const p = a.debug_stand_near_point(x, z, 1.5);
      if (p.length === 2) a.debug_teleport(p[0], p[1]);
    };
    window.__tick(3);
    a.zoom(0.001);
    a.zoom(1.4); // 14 m default
    new Function('a', 'spawn', 'near', code)(a, spawn, near);
    for (let i = 0; i < 10; i++) a.frame(0.1); // camera glide, doors, day / night blend settle
    window.__tick(1); // one frame of the page loop (sign text textures)
  }, [setup]);
  return { page, errors };
}

function selected() {
  const only = process.env.LOOK_SCENARIOS?.split(',');
  return SCENARIOS.filter(([id]) => !only || only.some((o) => id.startsWith(o)));
}

async function capture(dist, out) {
  const { chromium } = require('playwright');
  const port = Number(process.env.LOOK_PORT ?? 4191);
  const server = await serve(path.resolve(dist), port);
  fs.mkdirSync(out, { recursive: true });
  const browser = await chromium.launch({ args: BROWSER_ARGS });
  const summary = {};
  try {
    for (const [id, setup] of selected()) {
      const { page, errors } = await openScenario(browser, port, id, setup);
      const shot = await page.evaluate(() => {
        const a = window.__zoo.app;
        const canvas = document.getElementById('game');
        const gl = canvas.getContext('webgl2');
        a.frame(0);
        const px = new Uint8Array(canvas.width * canvas.height * 4);
        gl.readPixels(0, 0, canvas.width, canvas.height, gl.RGBA, gl.UNSIGNED_BYTE, px);
        a.frame(0);
        const png = canvas.toDataURL('image/png');
        return { w: canvas.width, h: canvas.height, px: Array.from(px), png, lights: Array.from(a.light_stats()), dc: a.draw_calls(), daytime: a.daytime() };
      });
      fs.writeFileSync(path.join(out, `${id}.rgba`), Buffer.from(shot.px));
      fs.writeFileSync(path.join(out, `${id}.png`), Buffer.from(shot.png.split(',')[1], 'base64'));
      summary[id] = { w: shot.w, h: shot.h, lights: shot.lights, draw_calls: shot.dc, daytime: shot.daytime, errors };
      console.log(`${id} ${shot.w}x${shot.h} dc=${shot.dc} lights=${shot.lights.join('/')} ${shot.daytime}${errors.length ? ' ERR ' + errors.join(' | ') : ''}`);
      await page.close();
    }
  } finally {
    await browser.close();
    server.close();
  }
  fs.writeFileSync(path.join(out, 'summary.json'), JSON.stringify(summary, null, 1));
}

/**
 * Interleaved A/B timing: both builds open side by side at the same scenario; rounds of
 * `k` fenced frames (`app.frame(0)` + 1-pixel `readPixels`) alternate A, B, B, A, … so
 * machine-load drift hits both equally. Prints medians and B/A per scenario and viewport,
 * split into the scene pass and the full-screen outline / fog / sky pass (both fenced).
 */
async function ab(distA, distB, out) {
  const { chromium } = require('playwright');
  const port = Number(process.env.LOOK_PORT ?? 4191);
  const rounds = Number(process.env.AB_ROUNDS ?? 8);
  const k = Number(process.env.AB_FRAMES ?? 3);
  const viewports = {
    desktop: { width: 1920, height: 1080, dpr: 1 },
    phone: { width: 360, height: 780, dpr: 3 },
  };
  const vps = (process.env.AB_VIEWPORTS ?? 'desktop,phone').split(',');
  const servers = [await serve(path.resolve(distA), port), await serve(path.resolve(distB), port + 1)];
  const browser = await chromium.launch({ args: BROWSER_ARGS });
  const result = {};
  const med = (v) => [...v].sort((x, y) => x - y)[Math.floor(v.length / 2)];
  try {
    for (const vp of vps) {
      for (const [id, setup] of selected()) {
        const pages = [
          await openScenario(browser, port, id, setup, viewports[vp]),
          await openScenario(browser, port + 1, id, setup, viewports[vp], process.env.AB_INIT_B ?? null),
        ];
        const times = [[], []];
        if (!result.renderer) {
          result.renderer = await pages[0].page.evaluate(() => {
            const gl = document.getElementById('game').getContext('webgl2');
            const d = gl.getExtension('WEBGL_debug_renderer_info');
            return d ? gl.getParameter(d.UNMASKED_RENDERER_WEBGL) : gl.getParameter(gl.RENDERER);
          });
          console.log(`renderer: ${result.renderer}`);
        }
        for (let r = 0; r < rounds; r++) {
          for (const s of r % 2 ? [1, 0] : [0, 1]) {
            const t = await pages[s].page.evaluate((n) => {
              const a = window.__zoo.app;
              const gl = document.getElementById('game').getContext('webgl2');
              const px = new Uint8Array(4);
              const sync = () => gl.readPixels(0, 0, 1, 1, gl.RGBA, gl.UNSIGNED_BYTE, px);
              // scene pass = everything before the full-screen drawArrays(TRIANGLES, 0, 3), fenced
              const proto = WebGL2RenderingContext.prototype;
              const draw = proto.drawArrays;
              let tPre = 0;
              proto.drawArrays = function (...args) {
                if (args[0] === 4 && args[1] === 0 && args[2] === 3) {
                  sync();
                  tPre = window.__realNow();
                }
                return draw.apply(this, args);
              };
              sync();
              const out = [];
              try {
                for (let i = 0; i < n; i++) {
                  const t0 = window.__realNow();
                  a.frame(0);
                  sync();
                  const t1 = window.__realNow();
                  out.push([t1 - t0, tPre - t0, t1 - tPre]);
                }
              } finally {
                proto.drawArrays = draw;
              }
              return out;
            }, k);
            times[s].push(...t);
          }
        }
        for (const p of pages) await p.page.close();
        const col = (s, c) => med(times[s].map((x) => x[c]));
        const [ma, mb] = [col(0, 0), col(1, 0)];
        const sp = [col(0, 1), col(1, 1), col(0, 2), col(1, 2)];
        result[`${vp} ${id}`] = { a_ms: ma, b_ms: mb, ratio: mb / ma, a_scene_ms: sp[0], b_scene_ms: sp[1], a_fullscreen_ms: sp[2], b_fullscreen_ms: sp[3], n: times[0].length };
        console.log(
          `${vp} ${id}: A ${ma.toFixed(1)} ms, B ${mb.toFixed(1)} ms, B/A ${(mb / ma).toFixed(3)}; ` +
            `scene ${sp[0].toFixed(1)} → ${sp[1].toFixed(1)} (${(sp[1] / sp[0]).toFixed(3)}), ` +
            `full-screen ${sp[2].toFixed(1)} → ${sp[3].toFixed(1)} (${(sp[3] / sp[2]).toFixed(3)}) (n=${times[0].length})`,
        );
      }
    }
  } finally {
    await browser.close();
    for (const s of servers) s.close();
  }
  if (out) fs.writeFileSync(out, JSON.stringify(result, null, 1));
}

/** Minimal RGBA PNG encoder (filter 0, zlib), rows top first. */
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

/** Diff image of two captures (readPixels order: bottom row first) → PNG buffer. */
function diffImage(x, y, w, h) {
  const out = Buffer.alloc(w * h * 4);
  for (let row = 0; row < h; row++) {
    for (let col = 0; col < w; col++) {
      const i = ((h - 1 - row) * w + col) * 4;
      const o = (row * w + col) * 4;
      let m = 0;
      for (let c = 0; c < 3; c++) m = Math.max(m, Math.abs(x[i + c] - y[i + c]));
      if (m > 0) {
        out[o] = 255;
        out[o + 1] = Math.max(0, 160 - m * 2);
        out[o + 2] = 0;
      } else {
        const g = Math.round((y[i] + y[i + 1] + y[i + 2]) / 3 * 0.45);
        out[o] = out[o + 1] = out[o + 2] = g;
      }
      out[o + 3] = 255;
    }
  }
  return png(w, h, out);
}

function compare(a, b, tol) {
  const diffDir = process.env.LOOK_DIFF_DIR;
  if (diffDir) fs.mkdirSync(diffDir, { recursive: true });
  const sizes = fs.existsSync(path.join(b, 'summary.json')) ? JSON.parse(fs.readFileSync(path.join(b, 'summary.json'), 'utf8')) : {};
  let worst = 0;
  const rows = [];
  for (const [id] of SCENARIOS) {
    const fa = path.join(a, `${id}.rgba`);
    const fb = path.join(b, `${id}.rgba`);
    if (!fs.existsSync(fa) || !fs.existsSync(fb)) {
      rows.push(`${id}: missing`);
      worst = Infinity;
      continue;
    }
    const x = fs.readFileSync(fa);
    const y = fs.readFileSync(fb);
    if (x.length !== y.length) {
      rows.push(`${id}: size differs`);
      worst = Infinity;
      continue;
    }
    let diff = 0;
    let over = 0;
    let max = 0;
    let sum = 0;
    for (let i = 0; i < x.length; i += 4) {
      let m = 0;
      for (let c = 0; c < 3; c++) {
        const d = Math.abs(x[i + c] - y[i + c]);
        sum += d;
        if (d > m) m = d;
      }
      if (m > 0) diff++;
      if (m > tol) over++;
      if (m > max) max = m;
    }
    const n = x.length / 4;
    worst = Math.max(worst, over);
    if (diffDir && diff > 0) {
      const w = sizes[id]?.w ?? W;
      const h = sizes[id]?.h ?? H;
      fs.writeFileSync(path.join(diffDir, `${id}_diff.png`), diffImage(x, y, w, h));
    }
    rows.push(`${id}: ${diff} px differ (${((100 * diff) / n).toFixed(3)} %), ${over} px > ${tol}, max channel diff ${max}, mean ${(sum / (n * 3)).toFixed(4)}`);
  }
  console.log(rows.join('\n'));
  return worst;
}

const [cmd, a, b, t] = process.argv.slice(2);
if (cmd === 'capture' && a && b) {
  await capture(a, b);
} else if (cmd === 'ab' && a && b) {
  await ab(a, b, t);
} else if (cmd === 'compare' && a && b) {
  process.exit(compare(a, b, Number(t ?? 2)) > 0 ? 1 : 0);
} else {
  console.error('usage: look.mjs capture <dist> <out> | compare <a> <b> [tol] | ab <distA> <distB> [out.json]');
  process.exit(2);
}
