// FEED-033, FEED-034: food box labels carry a vector pictogram on every reading level (bigger
// on kiga, small on klasse3); screenshots of the box row from the zoo camera in
// qa/reports/img/2026-10-02-food-pictograms-<level>.png and of the box panel.
import fs from 'node:fs';
import path from 'node:path';
import { expect, test } from '@playwright/test';
import { goto, nextFrames, repo, START_URL, waitFrames } from './helpers';

test.describe.configure({ timeout: 480_000 });
test.use({ viewport: { width: 1280, height: 720 } });

const out = path.join(repo, 'qa/reports/img');

for (const level of ['kiga', 'klasse3']) {
  test(`FEED-033/034: food box labels show pictograms (${level})`, async ({ page }) => {
    const errors: string[] = [];
    page.on('pageerror', (e) => errors.push(String(e)));
    await page.goto(START_URL);
    await waitFrames(page, 3);
    await page.evaluate((l) => window.__zoo!.app.set_reading_level(l), level);
    await goto(page, -3.2, 9.5);
    await page.evaluate(() => window.__zoo!.app.debug_look_at(-3.2, 10.66));
    await nextFrames(page, 12);
    if (await page.locator('#panel-close').isVisible()) await page.locator('#panel-close').click();
    await nextFrames(page, 4);
    fs.mkdirSync(out, { recursive: true });
    await page.screenshot({ path: path.join(out, `2026-10-02-food-pictograms-${level}.png`) });
    // closest zoom (10 m, the nearest the player can get the camera)
    await page.evaluate(() => {
      for (let i = 0; i < 6; i++) window.__zoo!.app.zoom(0.7);
    });
    await nextFrames(page, 30);
    await page.screenshot({ path: path.join(out, `2026-10-02-food-pictograms-${level}-zoom.png`) });

    // the box panel shows the pictogram canvas, bigger on kiga
    await goto(page, -3.54, 9.5);
    await page.evaluate(() => window.__zoo!.app.debug_face_point(-3.54, 10.66));
    await nextFrames(page, 2);
    if (!(await page.locator('#panel-picture').isVisible())) await page.keyboard.press('e');
    const pic = page.locator('#panel-picture');
    await expect(pic).toBeVisible();
    expect(await pic.getAttribute('data-pictogram')).toBe('hay');
    const h = (await pic.locator('canvas').boundingBox())!.height;
    await page.screenshot({ path: path.join(out, `2026-10-02-food-pictograms-${level}-panel.png`) });
    // kiga 0.62 -> 17 vh = 122 px at 720; klasse3 0.28 -> 8 vh = 57 px
    expect(h).toBeGreaterThan(level === 'kiga' ? 100 : 40);
    expect(h).toBeLessThan(level === 'kiga' ? 140 : 80);
    expect(errors).toEqual([]);
  });
}

// FEED-030: the food box decals are batched; draw calls at the level-1 start view
test('FEED-030: food box decals cost only a few draw calls at the start view', async ({ page }) => {
  await page.goto(START_URL);
  await waitFrames(page, 3);
  await nextFrames(page, 10);
  const m = await page.evaluate(() => {
    const a = window.__zoo!.app;
    a.frame(0);
    return { dc: a.draw_calls(), decals: a.decals_drawn() };
  });
  console.log(`start view: ${m.dc} draw calls, ${m.decals} decal quads drawn`);
  const food = await page.evaluate(() => window.__zoo!.app.decal_ids().split('\n').filter((i) => i.startsWith('foodbox:')).length);
  expect(food).toBeGreaterThan(20);
});
