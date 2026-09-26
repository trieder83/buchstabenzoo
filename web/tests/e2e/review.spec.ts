// Visual review shots (not a pass/fail check beyond "renders without errors"): landscape
// views at a few level-1 places, saved to art/environment/poc/ for ART-DIRECTION review.
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { expect, test } from '@playwright/test';
import { START_URL } from './helpers';

const here = path.dirname(fileURLToPath(import.meta.url));
const shots = path.resolve(here, '../../../art/environment/poc');

test.use({ viewport: { width: 2340, height: 1080 } });

test('review shots: zebra gate, bridge, plaza (landscape)', async ({ page }) => {
  const errors: string[] = [];
  page.on('console', (m) => {
    if (m.type() === 'error') errors.push(m.text());
  });
  await page.goto(START_URL);
  await page.waitForFunction(() => (window.__zoo?.frames ?? 0) > 3 || Boolean(window.__zooError), null, {
    timeout: 90_000,
  });
  const places: [string, number, number][] = [
    ['plaza', 0.5, 4.5],
    ['zebra_gate', -7.5, 13.5],
    ['bridge', 8.5, 29.5],
    ['pond', -9.5, 23.5],
    ['cave_repair', 18.5, 9.5],
  ];
  for (const [name, x, z] of places) {
    await page.evaluate(([px, pz]) => window.__zoo!.app.debug_teleport(px, pz), [x, z]);
    const f = await page.evaluate(() => window.__zoo!.frames);
    await page.waitForFunction((n) => window.__zoo!.frames > n + 2, f);
    await page.screenshot({ path: path.join(shots, `screenshot_poc_m3_${name}.png`) });
  }
  expect(errors).toEqual([]);
});
