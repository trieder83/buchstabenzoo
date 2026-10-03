// AENV-015: the ice cream kiosk, carousel, playground slide and swings are drawn by their
// kit_landmarks_play models; the carousel turns (its `rotor` node spins with the clock).
// Review shots: qa/reports/img/2026-10-03-play-*.png.
import path from 'node:path';
import { expect, test } from '@playwright/test';
import { nextFrames, repo, START_URL, waitFrames } from './helpers';

test.describe.configure({ timeout: 300_000 });
test.use({ viewport: { width: 1280, height: 720 } });

const out = path.join(repo, 'qa/reports/img');

// AENV-015
test('AENV-015: kiosk, carousel, slide and swings stand in the zoo and the carousel turns', async ({ page }) => {
  const errors: string[] = [];
  page.on('pageerror', (e) => errors.push(String(e)));
  page.on('console', (m) => {
    if (m.type() === 'error') errors.push(m.text());
  });
  await page.goto(START_URL);
  await waitFrames(page, 3);
  await page.evaluate(() => {
    const a = window.__zoo!.app;
    for (const n of ['zebra', 'hippo', 'panda', 'koala', 'elephant', 'giraffe', 'lion']) a.debug_send_home(n);
    a.debug_set_daytime('night');
    for (const n of ['hedgehog', 'bat', 'owl']) a.debug_send_home(n);
    a.debug_next_morning();
  });
  await nextFrames(page, 3);
  await expect(page.locator('#celebrate')).toBeHidden({ timeout: 20_000 });
  const look = async (name: string, px: number, pz: number, tx: number, tz: number) => {
    const stand = await page.evaluate(([x, z]) => window.__zoo!.app.debug_stand_near_point(x, z, 5.0), [px, pz]);
    await page.evaluate(([x, z]) => window.__zoo!.app.debug_teleport(x, z), [stand[0], stand[1]]);
    await page.evaluate(([x, z]) => window.__zoo!.app.debug_look_at(x, z), [tx, tz]);
    await nextFrames(page, 6);
    return page.screenshot({ path: path.join(out, `2026-10-03-play-${name}.png`) });
  };
  await look('kiosk', 17, 90, 17, 84.5);
  const a = await look('carousel', -14, 60, -14, 54);
  await page.evaluate(() => window.__zoo!.app.debug_step(2.0));
  await page.evaluate(([x, z]) => window.__zoo!.app.debug_look_at(x, z), [-14, 54]);
  await nextFrames(page, 3);
  const b = await page.screenshot({ path: path.join(out, '2026-10-03-play-carousel_b.png') });
  expect(a.equals(b), 'the carousel rotor turned between two frames').toBe(false);
  await look('playground', 69, 22, 69, 16.5);
  expect(errors).toEqual([]);
});
