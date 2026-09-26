// POC-001 / ARCH-003 smoke test (PROD-POC, TECH-ARCH), milestone M3: the release build
// renders level 1 with WebGL2 in headless Chromium at 1080×2340 without console errors, and
// the player walks when a movement key is held.
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { expect, test, type Page } from '@playwright/test';

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

test('POC-001 / ARCH-003: level 1 renders with WebGL2, player walks, no console errors', async ({ page }) => {
  const errors: string[] = [];
  page.on('console', (m) => {
    if (m.type() === 'error') errors.push(m.text());
  });
  page.on('pageerror', (e) => errors.push(String(e)));

  await page.goto('/');
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

  // Hold W (north) — the player must move north.
  await page.locator('#game').focus();
  await page.keyboard.down('KeyW');
  await page.waitForTimeout(1500);
  await page.keyboard.up('KeyW');
  await waitFrames(page, 10);
  const moved = await page.evaluate(() => ({ x: window.__zoo!.app.player_x(), z: window.__zoo!.app.player_z() }));
  expect(moved.z - start.z).toBeGreaterThan(0.8);
  expect(Math.abs(moved.x - start.x)).toBeLessThan(0.1);

  // Hold D (east): the player moves east (not mirrored).
  await page.keyboard.down('KeyD');
  await page.waitForTimeout(1500);
  await page.keyboard.up('KeyD');
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
