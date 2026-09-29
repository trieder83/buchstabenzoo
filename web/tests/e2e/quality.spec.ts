// PERF-022 (PERF-BUDGETS rule 5, Q-170, PERF-R-005): the automatic quality tier. Slow frames
// (p95 > 33 ms for 3 s) switch the game to the pixel ratio 1.5 first, then to the full low
// tier (lantern + 4 lamps, no clouds) — without a menu and without flipping back. The
// `?quality=` debug override pins a tier; automated browsers default to `high`.
import { expect, test, type Page } from '@playwright/test';
import { waitFrames } from './helpers';

test.describe.configure({ timeout: 240_000 });
// a phone: CSS 360 × 780 at DPR 3 → the drawing buffer is 720 × 1560 (cap 2) or 540 × 1170
test.use({ viewport: { width: 360, height: 780 }, deviceScaleFactor: 3 });

async function open(page: Page, query: string, slowMs = 0) {
  await page.addInitScript((ms) => {
    localStorage.clear();
    localStorage.setItem('zoo.readingLevel', 'klasse1');
    if (ms > 0) {
      // every animation frame takes at least `ms` (a weak phone)
      const raf = window.requestAnimationFrame.bind(window);
      window.requestAnimationFrame = (cb) =>
        raf((t) => {
          const end = performance.now() + ms;
          while (performance.now() < end) {
            /* busy */
          }
          cb(t);
        });
    }
  }, slowMs);
  await page.goto(`/?seed=17${query}`);
  await waitFrames(page, 3);
}

const state = (page: Page) =>
  page.evaluate(() => {
    const a = window.__zoo!.app;
    const c = document.getElementById('game') as HTMLCanvasElement;
    return { tier: a.quality(), mode: a.quality_mode(), ratio: a.pixel_ratio(), w: c.width, h: c.height };
  });

test('PERF-022: slow frames step down to the pixel ratio 1.5, then the low tier, and stay there', async ({ page }) => {
  const errors: string[] = [];
  page.on('pageerror', (e) => errors.push(String(e)));
  await open(page, '&quality=auto', 45);
  const s0 = await state(page);
  expect(s0.mode).toBe('auto');
  // step 1 after the 5 s warm-up + one 3 s window of slow frames
  await page.waitForFunction(() => window.__zoo!.app.quality() !== 'high', null, { timeout: 90_000, polling: 250 });
  const s1 = await state(page);
  expect(s1.tier).toBe('low1');
  expect(s1.ratio).toBe(1.5);
  expect([s1.w, s1.h]).toEqual([540, 1170]);
  // still slow → step 2: lantern + 4 lamps as point lights, the rest light pools
  await page.waitForFunction(() => window.__zoo!.app.quality() === 'low', null, { timeout: 90_000, polling: 250 });
  const lights = await page.evaluate(() => {
    const a = window.__zoo!.app;
    a.debug_set_daytime('night');
    for (let i = 0; i < 5; i++) a.frame(0.1);
    return Array.from(a.light_stats());
  });
  expect(lights[0]).toBeLessThanOrEqual(5);
  expect(lights[0]).toBeGreaterThanOrEqual(2);
  expect(lights[1]).toBeGreaterThan(0);
  // no flicker: fast frames later do not bring the high tier back
  const f = await page.evaluate(() => window.__zoo!.frames);
  await page.waitForFunction((m) => window.__zoo!.frames >= m, f + 20, { timeout: 90_000 });
  expect((await state(page)).tier).toBe('low');
  expect(errors).toEqual([]);
});

test('PERF-022: the debug override pins a tier; automated browsers start high', async ({ page }) => {
  // default under Playwright (navigator.webdriver): fixed high, even with slow frames
  await open(page, '', 45);
  const f = await page.evaluate(() => window.__zoo!.frames);
  await page.waitForTimeout(12_000);
  expect(await page.evaluate(() => window.__zoo!.frames)).toBeGreaterThan(f);
  expect(await state(page)).toMatchObject({ tier: 'high', mode: 'fixed', ratio: 2, w: 720, h: 1560 });
  // ?quality=low: pixel ratio 1.5 at once, lantern + at most 4 lamps at night
  await page.goto('/?seed=17&quality=low');
  await waitFrames(page, 3);
  expect(await state(page)).toMatchObject({ tier: 'low', mode: 'fixed', ratio: 1.5, w: 540, h: 1170 });
  const lights = await page.evaluate(() => {
    const a = window.__zoo!.app;
    a.debug_set_daytime('night');
    for (let i = 0; i < 5; i++) a.frame(0.1);
    return Array.from(a.light_stats());
  });
  expect(lights[0]).toBeLessThanOrEqual(5);
  // the same spot in the high tier uses up to 9
  await page.goto('/?seed=17&quality=high');
  await waitFrames(page, 3);
  const high = await page.evaluate(() => {
    const a = window.__zoo!.app;
    a.debug_set_daytime('night');
    for (let i = 0; i < 5; i++) a.frame(0.1);
    return Array.from(a.light_stats());
  });
  expect(high[0]).toBeGreaterThan(lights[0]);
  expect(high[0] + high[1]).toBeGreaterThanOrEqual(lights[0] + lights[1]);
});
