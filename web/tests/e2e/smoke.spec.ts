// POC-001 / ARCH-003 smoke test (PROD-POC, TECH-ARCH), milestone M3: the release build
// renders level 1 with WebGL2 in headless Chromium at 1080×2340 without console errors, and
// the player walks when a movement key is held.
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { expect, test, type Page } from '@playwright/test';
import { START_URL } from './helpers';

const here = path.dirname(fileURLToPath(import.meta.url));
const shots = path.resolve(here, '../../../art/environment/poc');

async function waitFrames(page: Page, n: number) {
  await page.waitForFunction(
    (min) => Boolean(window.__zooError) || (window.__zoo?.frames ?? 0) >= min,
    n,
    { timeout: 90_000 },
  );
  expect(await page.evaluate(() => window.__zooError ?? null)).toBeNull();
}

/** Holds a key (real keyboard events) until `seconds` of game time have passed. */
async function holdGameTime(page: Page, code: string, seconds: number): Promise<void> {
  const t0 = await page.evaluate(() => window.__zoo!.app.time());
  await page.keyboard.down(code);
  await page.waitForFunction((t) => window.__zoo!.app.time() >= t, t0 + seconds, { timeout: 60_000 });
  await page.keyboard.up(code);
}

test('POC-001 / ARCH-003: level 1 renders with WebGL2, player walks, no console errors', async ({ page }) => {
  const errors: string[] = [];
  page.on('console', (m) => {
    if (m.type() === 'error') errors.push(m.text());
  });
  page.on('pageerror', (e) => errors.push(String(e)));

  await page.goto(START_URL);
  await waitFrames(page, 5);
  const hasGl2 = await page.evaluate(() => {
    const c = document.getElementById('game') as HTMLCanvasElement;
    return c.getContext('webgl2') !== null && c.width === 1080 && c.height === 2340;
  });
  expect(hasGl2).toBe(true);

  console.log('placeholders:\n' + (await page.evaluate(() => window.__zoo!.app.placeholders())));
  const start = await page.evaluate(() => {
    const a = window.__zoo!.app;
    return { x: a.player_x(), z: a.player_z(), dist: a.camera_distance(), draws: a.draw_calls() };
  });
  // Spawn cell (0, 2) facing north, level 1 starts at maximum zoom-out (LAYOUT-L1-011).
  expect(start.x).toBeCloseTo(0.5, 3);
  expect(start.z).toBeCloseTo(2.5, 3);
  expect(start.dist).toBeCloseTo(20, 3);
  expect(start.draws).toBeGreaterThan(3);
  expect(start.draws).toBeLessThan(60); // instanced batches, not one call per tile
  await page.screenshot({ path: path.join(shots, 'screenshot_poc_m3.png') });

  // Hold W (north) — the player must move north. Held for 1.5 s of *game* time: headless
  // Chromium advances the rAF clock 1/60 s per frame, and software WebGL at 1080×2340 draws
  // only ≈ 11 fps, so 1.5 s of wall time is ≈ 0.3 s of game time (QA 2026-09-27: the walk
  // itself is 1.93 m/s on the path, no movement regression).
  await page.locator('#game').focus();
  await holdGameTime(page, 'KeyW', 1.5);
  await waitFrames(page, 10);
  const moved = await page.evaluate(() => ({ x: window.__zoo!.app.player_x(), z: window.__zoo!.app.player_z() }));
  expect(moved.z - start.z).toBeGreaterThan(0.8);
  expect(Math.abs(moved.x - start.x)).toBeLessThan(0.1);

  // Hold D (east): the player moves east (not mirrored).
  await holdGameTime(page, 'KeyD', 1.5);
  await page.waitForTimeout(200);
  const east = await page.evaluate(() => window.__zoo!.app.player_x());
  expect(east - moved.x).toBeGreaterThan(0.4);
  await page.screenshot({ path: path.join(shots, 'screenshot_poc_m3_walk.png') });

  const stats = await page.evaluate(() => {
    const z = window.__zoo!;
    return {
      draws: z.app.draw_calls(),
      instances: z.app.instances(),
      triangles: z.app.triangles(),
      frameMs: z.frameMs,
      intervalMs: z.intervalMs,
      placeholders: z.app.placeholders(),
      playerModel: z.app.player_is_model(),
    };
  });
  console.log('render stats', JSON.stringify(stats));
  expect(errors).toEqual([]);
});
