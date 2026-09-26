// Living water and ambient animals (TECH-WATER, GAME-AMBIENT): the river flows along its
// direction and the pond is still but alive (AENV-007 / WATER-006), no outline flicker on the
// water (WATER-007), no extra draw calls (WATER-008), animated ducks (AMB-005) within their
// budget (AMB-007). Frames are rendered at fixed times with the debug clock and read back
// with readPixels in the same task (no compositing in between). Review shots go to
// art/environment/poc/screenshot_poc_water_*.png.
import path from 'node:path';
import { expect, test, type Page } from '@playwright/test';
import { shots, waitFrames } from './helpers';

test.describe.configure({ timeout: 240_000 });
test.use({ viewport: { width: 1280, height: 720 } });

async function start(page: Page, quietPond = true) {
  const errors: string[] = [];
  page.on('console', (m) => {
    if (m.type() === 'error') errors.push(m.text());
  });
  page.on('pageerror', (e) => errors.push(String(e)));
  await page.goto('/?seed=17');
  await waitFrames(page, 3);
  await page.evaluate((quiet) => {
    const a = window.__zoo!.app;
    a.debug_pause(true);
    // the hippo of seed 17 hides in the pond: send it home so the pond crop is plain water
    if (quiet) {
      a.debug_send_home('hippo');
      a.debug_step(25);
    }
    a.debug_teleport(0.5, 4.5);
    a.debug_step(0.1);
  }, quietPond);
  return errors;
}

/** Level rectangle → screen rect (CSS px, y down) with the current camera. */
async function screenRect(page: Page, x0: number, z0: number, x1: number, z1: number) {
  return page.evaluate(
    ([x0, z0, x1, z1]) => {
      const a = window.__zoo!.app;
      const pts = [a.screen_point(x0, z0, 0), a.screen_point(x1, z0, 0), a.screen_point(x0, z1, 0), a.screen_point(x1, z1, 0)];
      const xs = pts.map((p) => p[0]);
      const ys = pts.map((p) => p[1]);
      return [Math.min(...xs), Math.min(...ys), Math.max(...xs), Math.max(...ys)].map(Math.round);
    },
    [x0, z0, x1, z1],
  );
}

/**
 * Renders the frame at water times t0 and t1 (clock frozen otherwise) and compares a screen
 * crop: mean absolute difference, dominant motion vector by block matching (px, y down) and
 * the number of outline-coloured pixels in both frames.
 */
async function analyse(page: Page, rect: number[], t0: number, t1: number, avoid: number[][] = []) {
  return page.evaluate(
    ([rect, t0, t1, avoid]) => {
      const a = window.__zoo!.app;
      const canvas = document.getElementById('game') as HTMLCanvasElement;
      const gl = canvas.getContext('webgl2')!;
      const W = canvas.width;
      const H = canvas.height;
      const ratio = W / canvas.clientWidth;
      const grab = (t: number) => {
        a.debug_set_time(t);
        a.frame(0);
        const px = new Uint8Array(W * H * 4);
        gl.readPixels(0, 0, W, H, gl.RGBA, gl.UNSIGNED_BYTE, px);
        return px;
      };
      const A = grab(t0 as number);
      const B = grab(t1 as number);
      const [x0, y0, x1, y1] = (rect as number[]).map((v) => Math.round(v * ratio));
      const idx = (x: number, y: number) => ((H - 1 - y) * W + x) * 4; // y down → GL rows
      const lum = (P: Uint8Array, x: number, y: number) => {
        const i = idx(x, y);
        return 0.3 * P[i] + 0.59 * P[i + 1] + 0.11 * P[i + 2];
      };
      const skip = (x: number, y: number) =>
        (avoid as number[][]).some((p) => Math.hypot(x - p[0] * ratio, y - p[1] * ratio) < 34 * ratio);
      let diff = 0;
      let n = 0;
      let outline = 0;
      const dark = (P: Uint8Array, x: number, y: number) => {
        const i = idx(x, y);
        return P[i] < 110 && P[i + 1] < 85 && P[i + 2] < 70;
      };
      for (let y = y0 + 3; y < y1 - 3; y++) {
        for (let x = x0 + 3; x < x1 - 3; x++) {
          if (skip(x, y)) continue;
          diff += Math.abs(lum(A, x, y) - lum(B, x, y));
          n++;
          if (dark(A, x, y)) outline++;
          if (dark(B, x, y)) outline++;
        }
      }
      // block matching per 32 px block: displacement d minimising mean |A(p) − B(p + d)|;
      // the dominant motion is the mean vector of all textured blocks (rings of a still
      // pond point every way and cancel, a flowing river moves every block the same way)
      const R = Math.round(26 * ratio);
      const S = Math.round(32 * ratio);
      let sx = 0;
      let sy = 0;
      let blocks = 0;
      for (let by = y0; by + S <= y1; by += S) {
        for (let bx = x0; bx + S <= x1; bx += S) {
          let best = [0, 0, Infinity];
          let zero = 0;
          for (let dy = -R; dy <= R; dy++) {
            for (let dx = -R; dx <= R; dx++) {
              let s = 0;
              let m = 0;
              for (let y = by; y < by + S; y += 2) {
                for (let x = bx; x < bx + S; x += 2) {
                  if (skip(x, y) || x + dx < 0 || y + dy < 0 || x + dx >= W || y + dy >= H) continue;
                  s += Math.abs(lum(A, x, y) - lum(B, x + dx, y + dy));
                  m++;
                }
              }
              s /= Math.max(m, 1);
              if (dx === 0 && dy === 0) zero = s;
              if (s < best[2] - 1e-9 || (s <= best[2] + 1e-9 && Math.hypot(dx, dy) < Math.hypot(best[0], best[1]))) {
                best = [dx, dy, s];
              }
            }
          }
          if (zero < 1.0) continue; // untextured block: no motion information
          sx += best[0];
          sy += best[1];
          blocks++;
        }
      }
      const best = [sx / Math.max(blocks, 1), sy / Math.max(blocks, 1)];
      return { diff: diff / Math.max(n, 1), vx: best[0] / ratio, vy: best[1] / ratio, blocks, outline, n };
    },
    [rect, t0, t1, avoid] as const,
  );
}

async function ambientScreen(page: Page) {
  return page.evaluate(() => {
    const a = window.__zoo!.app;
    return (JSON.parse(a.ambient_json()) as { x: number; z: number }[])
      .map((d) => Array.from(a.screen_point(d.x, d.z, 0.1)))
      .filter((p) => p.length === 2);
  });
}

// AENV-007 / WATER-006: the river moves along its flow, the pond has no dominant motion.
// WATER-007: no outline-coloured pixels inside the water in two frames 0.25 s apart.
test('AENV-007 / WATER-006 / WATER-007: river flows south, pond is still but alive, no outline flicker', async ({ page }) => {
  const errors = await start(page);
  // river_n north of the bridge (flows south = down on screen)
  await page.evaluate(() => window.__zoo!.app.debug_look_at(11.5, 36));
  const river = await screenRect(page, 10.75, 33.5, 12.25, 38.5);
  const avoidR = await ambientScreen(page);
  const r = await analyse(page, river, 4.0, 4.5, avoidR);
  const flicker = await analyse(page, river, 7.0, 7.25, avoidR);
  // pond interior west of the lily pads
  await page.evaluate(() => window.__zoo!.app.debug_look_at(-16, 24.5));
  // motion: the whole pond interior (lily pads only bob in place); outlines: a strip of open
  // water west of the pads (pads and reeds are props with outlines)
  const pondAll = await screenRect(page, -18.3, 20.6, -11.7, 27.4);
  const pond = await screenRect(page, -18.35, 22.4, -17.45, 26.6);
  const avoidP = await ambientScreen(page);
  const p = await analyse(page, pondAll, 4.0, 4.5, avoidP);
  const pFlicker = await analyse(page, pond, 4.0, 4.25, avoidP);
  const pFlicker2 = await analyse(page, pond, 7.0, 7.25, avoidP);
  console.log('river', JSON.stringify(r), 'pond', JSON.stringify(p));

  expect(r.diff, 'river pixels change within 0.5 s').toBeGreaterThan(2);
  expect(p.diff, 'pond pixels change within 0.5 s').toBeGreaterThan(0.5);
  const len = Math.hypot(r.vx, r.vy);
  expect(len, 'river has a dominant motion').toBeGreaterThan(4);
  const angle = (Math.acos(r.vy / len) * 180) / Math.PI; // screen down = level south
  expect(angle, 'river motion within 30° of its flow').toBeLessThan(30);
  expect(Math.hypot(p.vx, p.vy), 'pond has no dominant direction').toBeLessThanOrEqual(0.2 * len);

  expect(r.outline + flicker.outline, 'no outline pixels on the river').toBe(0);
  expect(pFlicker.outline + pFlicker2.outline, 'no outline pixels on the pond').toBe(0);
  expect(errors).toEqual([]);
});

// WATER-008 (AENV-010): water animation on/off does not change the draw calls.
test('WATER-008: water animation adds no draw calls', async ({ page }) => {
  const errors = await start(page);
  const dc = await page.evaluate(() => {
    const a = window.__zoo!.app;
    a.debug_teleport(14.5, 29.5);
    a.frame(0);
    const on = a.draw_calls();
    a.set_water_animation(false);
    a.frame(0);
    const off = a.draw_calls();
    a.set_water_animation(true);
    return { on, off };
  });
  expect(dc.on).toBe(dc.off);
  expect(errors).toEqual([]);
});

// AMB-005 / AMB-007: ducks on screen move and are animated; all ambient animals together
// cost ≤ 10 draw calls and ≤ 0.5 ms CPU per frame.
test('AMB-005 / AMB-007: animated ducks within the draw-call and CPU budget', async ({ page }) => {
  const errors = await start(page);
  const res = await page.evaluate(() => {
    const a = window.__zoo!.app;
    a.debug_teleport(16.5, 29.5); // on the path east of the bridge, ducks in view
    a.debug_step(0.1);
    a.debug_look_at(12, 27.5);
    const canvas = document.getElementById('game') as HTMLCanvasElement;
    const gl = canvas.getContext('webgl2')!;
    const W = canvas.width;
    const H = canvas.height;
    const ducks = () =>
      (JSON.parse(a.ambient_json()) as { kind: string; x: number; z: number }[]).filter((d) => d.kind === 'duck');
    const before = ducks();
    const pts = before.map((d) => Array.from(a.screen_point(d.x, d.z, 0.12)));
    a.frame(0);
    const A = new Uint8Array(W * H * 4);
    gl.readPixels(0, 0, W, H, gl.RGBA, gl.UNSIGNED_BYTE, A);
    a.debug_step(1.0);
    a.frame(0);
    const B = new Uint8Array(W * H * 4);
    gl.readPixels(0, 0, W, H, gl.RGBA, gl.UNSIGNED_BYTE, B);
    const after = ducks();
    const regionDiff = pts.map((p) => {
      let d = 0;
      let n = 0;
      for (let y = Math.round(p[1]) - 16; y < p[1] + 16; y++) {
        for (let x = Math.round(p[0]) - 16; x < p[0] + 16; x++) {
          if (x < 0 || y < 0 || x >= W || y >= H) continue;
          const i = ((H - 1 - y) * W + x) * 4;
          d += Math.abs(A[i] - B[i]) + Math.abs(A[i + 1] - B[i + 1]) + Math.abs(A[i + 2] - B[i + 2]);
          n++;
        }
      }
      return d / Math.max(n, 1);
    });
    const moved = before.map((d, k) => Math.hypot(after[k].x - d.x, after[k].z - d.z));
    const ambientCalls = a.ambient_draw_calls();
    // CPU A/B: frame() with and without the ambient animals (median of 90 frames)
    const cpu = (on: boolean) => {
      a.set_ambient(on);
      const t: number[] = [];
      for (let i = 0; i < 90; i++) {
        const t0 = performance.now();
        a.frame(1 / 60);
        t.push(performance.now() - t0);
      }
      t.sort((x, y) => x - y);
      return t[45];
    };
    a.debug_pause(false);
    const off = cpu(false);
    const on = cpu(true);
    const off2 = cpu(false);
    a.debug_pause(true);
    return { regionDiff, moved, ambientCalls, on, off: Math.min(off, off2), total: a.draw_calls() };
  });
  console.log(JSON.stringify(res));
  expect(res.moved.some((m) => m > 0.1), 'ducks swim').toBe(true);
  expect(res.regionDiff.every((d) => d > 3), 'every duck area changes within 1 s').toBe(true);
  expect(res.ambientCalls).toBeGreaterThan(0);
  expect(res.ambientCalls, 'ambient animals ≤ 10 draw calls').toBeLessThanOrEqual(10);
  expect(res.on - res.off, 'ambient animals ≤ 0.5 ms CPU per frame').toBeLessThanOrEqual(0.5);
  expect(errors).toEqual([]);
});

// Review shots (AENV-009 / WATER-011 manual review): river with ducks at the bridge, pond with
// frogs and lily pads, level-3 stream with the water wheel.
test('review shots: living water', async ({ page }) => {
  const errors = await start(page, false);
  const views: [string, number, number, number, number, number][] = [
    // name, player x, z, look x, z, zoom factor
    ['river', 16.5, 29.5, 12.2, 28.0, 0.62],
    ['pond', -9.0, 22.8, -14.5, 23.8, 0.62],
    ['stream', -17.5, 66.5, -20.5, 70.0, 0.62],
  ];
  for (const [name, px, pz, lx, lz, zoom] of views) {
    await page.evaluate(
      ([px, pz, lx, lz, zoom]) => {
        const a = window.__zoo!.app;
        a.debug_teleport(px, pz);
        a.zoom(zoom);
        a.debug_step(2.0);
        a.debug_look_at(lx, lz);
        a.debug_set_time(5.3);
        document.querySelectorAll<HTMLElement>('button').forEach((b) => (b.style.visibility = 'hidden'));
      },
      [px, pz, lx, lz, zoom],
    );
    const f = await page.evaluate(() => window.__zoo!.frames);
    await page.waitForFunction((n) => window.__zoo!.frames > n + 2, f);
    await page.screenshot({ path: path.join(shots, `screenshot_poc_water_${name}.png`) });
    await page.evaluate(() => {
      const a = window.__zoo!.app;
      a.zoom(1 / 0.62);
    });
  }
  expect(errors).toEqual([]);
});
