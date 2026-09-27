// LAYOUT-031 (e2e part): gate and door models open visibly by the rules of GAME-LAYOUT
// "Gates and doors" — the garden gate within 2 m, the door of an enterable building while the
// player passes, the moon door at night — and close again. Review shot of the open garden
// gate: art/environment/poc/screenshot_garden_gate.png.
import path from 'node:path';
import { expect, test, type Page } from '@playwright/test';
import { goto, shots, waitFrames, START_URL } from './helpers';

test.use({ viewport: { width: 1280, height: 720 } });

interface OpeningState {
  kind: string;
  x: number;
  z: number;
  open: number;
  drawn: boolean;
}

async function openings(page: Page): Promise<OpeningState[]> {
  return JSON.parse(await page.evaluate(() => window.__zoo!.app.openings_json())) as OpeningState[];
}

/** The opening of a kind nearest to a level point. */
async function nearest(page: Page, kind: string, x: number, z: number): Promise<OpeningState> {
  const all = (await openings(page)).filter((o) => o.kind === kind);
  all.sort((a, b) => Math.hypot(a.x - x, a.z - z) - Math.hypot(b.x - x, b.z - z));
  return all[0];
}

/** Runs frames with game time (the presentation eases the open amount per frame). */
async function run(page: Page, seconds: number): Promise<void> {
  await page.evaluate((s) => {
    const a = window.__zoo!.app;
    for (let t = 0; t < s; t += 0.1) a.frame(0.1);
  }, seconds);
}

test('LAYOUT-031: garden gate, house door and moon door open visibly and close again', async ({ page }) => {
  const errors: string[] = [];
  page.on('console', (m) => {
    if (m.type() === 'error') errors.push(m.text());
  });
  page.on('pageerror', (e) => errors.push(String(e)));
  await page.addInitScript(() => {
    localStorage.clear();
    localStorage.setItem('zoo.readingLevel', 'klasse1');
  });
  await page.goto(START_URL);
  await waitFrames(page, 3);
  const all = await openings(page);
  expect(all.length).toBeGreaterThan(5);
  expect(all.every((o) => o.drawn), 'every gate / door has its model').toBe(true);

  // garden gate (x 7–9 on z = 36): opens within 2 m, closes when she leaves
  await goto(page, 8.5, 33.5);
  await run(page, 1.5);
  expect((await nearest(page, 'garden_gate', 8, 36)).open).toBe(0);
  await goto(page, 8.5, 35.5);
  await run(page, 2.0);
  expect((await nearest(page, 'garden_gate', 8, 36)).open).toBeGreaterThan(0.99);
  await page.screenshot({ path: path.join(shots, 'screenshot_garden_gate.png') });
  await goto(page, 8.5, 31.5);
  await run(page, 2.0);
  expect((await nearest(page, 'garden_gate', 8, 36)).open).toBe(0);

  // zookeeper house door (east facade, cell (−9, 2)): open while she passes
  await goto(page, -6.5, 2.5);
  await run(page, 1.5);
  expect((await nearest(page, 'door', -8.5, 2.5)).open).toBe(0);
  await goto(page, -9.5, 2.5);
  await run(page, 2.0);
  expect((await nearest(page, 'door', -8.5, 2.5)).open).toBeGreaterThan(0.99);
  await goto(page, -11.5, 2.5);
  await run(page, 2.0);
  expect((await nearest(page, 'door', -8.5, 2.5)).open).toBe(0);

  // moon door: shut by day, open at night
  expect((await nearest(page, 'moon_door', -23, 30)).open).toBe(0);
  expect(await page.evaluate(() => window.__zoo!.app.debug_set_daytime('night'))).toBe(true);
  await run(page, 2.0);
  expect((await nearest(page, 'moon_door', -23, 30)).open).toBeGreaterThan(0.99);
  expect(errors).toEqual([]);
});
