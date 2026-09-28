// PERF-BUDGETS rules checked in the browser (specs/50-performance/budgets.md):
//  - PERF-016 (budget 20, PERF-R-003): the shared per-frame values go up once per frame as
//    one uniform block; no uniform location lookups per frame; no uniform call re-sends the
//    value its location already has.
//  - PERF-017 (budget 21, PERF-R-001): per-draw light masks change no pixel — the night frame
//    with masks equals the same frame with every light for every draw.
import { expect, test, type Page } from '@playwright/test';
import { waitFrames, START_URL } from './helpers';

test.use({ viewport: { width: 960, height: 540 } });

async function start(page: Page) {
  await page.addInitScript(() => {
    localStorage.clear();
    localStorage.setItem('zoo.readingLevel', 'klasse1');
  });
  await page.goto(START_URL);
  await waitFrames(page, 3);
}

/** Moves the player (and snaps the camera) and lets a few frames settle. */
async function place(page: Page, daytime: 'day' | 'night', where: 'spawn' | 'night_zoo') {
  await page.evaluate(
    ([d, w]) => {
      const a = window.__zoo!.app;
      a.debug_set_daytime(d);
      if (w === 'spawn') {
        const s = a.level_spawn('level_1');
        a.debug_teleport(s[0], s[1]);
      } else {
        a.debug_teleport(-34.5, 29.5);
      }
      for (let i = 0; i < 20; i++) a.frame(0.1);
    },
    [daytime, where] as const,
  );
}

interface Census {
  frames: number;
  uboUploads: number;
  otherBufferSubData: number;
  locationLookups: number;
  uniformCalls: number;
  redundant: string[];
}

/** WebGL calls of `frames` frames (the prototype is wrapped only meanwhile). */
async function census(page: Page, frames: number): Promise<Census> {
  return page.evaluate((n) => {
    const app = window.__zoo!.app;
    const proto = WebGL2RenderingContext.prototype as unknown as Record<string, unknown>;
    const UNIFORM_BUFFER = 0x8a11;
    const c = { frames: n, uboUploads: 0, otherBufferSubData: 0, locationLookups: 0, uniformCalls: 0, redundant: [] as string[] };
    const last = new Map<unknown, string>();
    const saved: [string, unknown][] = [];
    for (const name of Object.getOwnPropertyNames(proto)) {
      const d = Object.getOwnPropertyDescriptor(proto, name);
      if (!d || typeof d.value !== 'function' || name === 'constructor') continue;
      const orig = d.value as (...a: unknown[]) => unknown;
      saved.push([name, orig]);
      proto[name] = function (this: unknown, ...args: unknown[]) {
        if (name === 'bufferSubData') {
          if (args[0] === UNIFORM_BUFFER) c.uboUploads++;
          else c.otherBufferSubData++;
        } else if (name === 'getUniformLocation') {
          c.locationLookups++;
        } else if (name.startsWith('uniform') && name !== 'uniformBlockBinding') {
          c.uniformCalls++;
          // arrays and matrices are sent every time by design; scalars / vectors only on change
          if (!name.endsWith('v')) {
            const key = JSON.stringify(args.slice(1));
            if (last.get(args[0]) === key) c.redundant.push(`${name}(${key})`);
            last.set(args[0], key);
          }
        }
        return orig.apply(this, args);
      };
    }
    try {
      for (let i = 0; i < n; i++) app.frame(1 / 60);
    } finally {
      for (const [name, orig] of saved) proto[name] = orig;
    }
    return c;
  }, frames);
}

for (const [daytime, where] of [
  ['day', 'spawn'],
  ['night', 'spawn'],
] as const) {
  test(`PERF-016: shared per-frame uniforms once per frame (${daytime}, ${where})`, async ({ page }) => {
    await start(page);
    await place(page, daytime, where);
    const c = await census(page, 3);
    expect(c.uboUploads, 'one frame-block upload per frame').toBe(c.frames);
    expect(c.locationLookups, 'no getUniformLocation per frame').toBe(0);
    expect(c.redundant, 'no uniform call re-sends an unchanged value').toEqual([]);
    expect(c.uniformCalls).toBeGreaterThan(0);
  });
}

for (const where of ['spawn', 'night_zoo'] as const) {
  test(`PERF-017: per-draw light masks change no pixel (night, ${where})`, async ({ page }) => {
    await start(page);
    await place(page, 'night', where);
    const r = await page.evaluate(() => {
      const a = window.__zoo!.app;
      const canvas = document.getElementById('game') as HTMLCanvasElement;
      const gl = canvas.getContext('webgl2')!;
      const read = () => {
        a.frame(0);
        const px = new Uint8Array(canvas.width * canvas.height * 4);
        gl.readPixels(0, 0, canvas.width, canvas.height, gl.RGBA, gl.UNSIGNED_BYTE, px);
        return px;
      };
      const masked = read();
      a.debug_full_light_masks(true);
      const full = read();
      a.debug_full_light_masks(false);
      let diff = 0;
      for (let i = 0; i < masked.length; i++) if (masked[i] !== full[i]) diff++;
      const lights = Array.from(a.light_stats() as Uint32Array);
      return { diff, lights };
    });
    expect(r.lights[0] + r.lights[1], 'lamps are lit').toBeGreaterThan(1);
    expect(r.diff, 'differing channels masked vs. every light').toBe(0);
  });
}
