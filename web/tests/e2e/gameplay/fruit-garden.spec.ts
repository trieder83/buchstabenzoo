// GARD-022: the fruit garden of level 3 (user request 2026-10-01): pick an apple and an orange,
// the basket HUD shows 🍎 / 🍊 with counts, the monkeys eat the fruit at home.
import { expect, test } from '@playwright/test';
import { nextFrames } from '../helpers';
import { startGame, teleport, wait } from './qa';

test.describe.configure({ timeout: 300_000 });
test.use({ viewport: { width: 1280, height: 720 } });

const basket = (page: import('@playwright/test').Page) =>
  page.evaluate(() => JSON.parse(window.__zoo!.app.basket_json()) as Record<string, number>);

/** Stands on a tree's stand cell, facing the tree. */
async function atTree(page: import('@playwright/test').Page, stand: [number, number], tree: [number, number]) {
  await teleport(page, stand[0], stand[1]);
  await page.evaluate(([x, z]) => {
    const a = window.__zoo!.app;
    a.debug_face_point(x, z);
    a.debug_step(0.05);
  }, tree);
  await nextFrames(page, 2);
}

test('GARD-022: apples and oranges from the level-3 fruit garden, then to the monkeys', async ({ page }) => {
  const errors = await startGame(page, 'de', 'klasse2');
  // level 3 opens the morning after levels 1 and 2 (the barriers open)
  await page.evaluate(() => {
    const a = window.__zoo!.app;
    for (const n of ['zebra', 'hippo', 'panda', 'koala', 'elephant', 'giraffe', 'lion']) a.debug_send_home(n);
    a.debug_set_daytime('night');
    for (const n of ['hedgehog', 'bat', 'owl']) a.debug_send_home(n);
    a.debug_next_morning();
  });
  await nextFrames(page, 3);
  expect(await page.evaluate(() => window.__zoo!.app.level_unlocked('level_3'))).toBe(true);
  for (const spot of ['apple_1', 'apple_2', 'orange_1', 'orange_2']) {
    expect(await page.evaluate((s) => window.__zoo!.app.plant_stage(s), spot), spot).toBe('ripe');
  }

  // review shot (not compared): the garden from the street
  await teleport(page, 16, 78.5);
  await wait(page, 1);
  await page.screenshot({ path: 'test-results/shots/fruit-garden.png' });

  // an apple: the interaction is available at the stand cell, the HUD shows 🍎1
  await atTree(page, [11.5, 74.5], [11.8, 75.5]);
  expect(await page.evaluate(() => window.__zoo!.app.target_key())).toBe('plant:apple_1');
  await page.keyboard.press('KeyE');
  await nextFrames(page, 3);
  expect((await basket(page)).apple).toBe(1);
  expect(await page.evaluate(() => window.__zoo!.app.plant_stage('apple_1'))).toBe('empty');
  await expect(page.locator('#hud-basket')).toContainText('🍎1');
  await expect(page.locator('#hud-basket')).toHaveAttribute('data-apple', '1');
  // a second one on the neighbouring tree, an orange on the other bed
  await atTree(page, [13.5, 74.5], [13.2, 75.5]);
  await page.keyboard.press('KeyE');
  await atTree(page, [18.5, 74.5], [18.8, 75.5]);
  expect(await page.evaluate(() => window.__zoo!.app.target_key())).toBe('plant:orange_1');
  await page.keyboard.press('KeyE');
  await nextFrames(page, 3);
  const b = await basket(page);
  expect([b.apple, b.orange, b.carrot, b.potato]).toEqual([2, 1, 0, 0]);
  await expect(page.locator('#hud-basket')).toContainText('🍊1');

  // the monkeys at home eat the fruit (both react, the apple leaves the basket)
  await page.evaluate(() => window.__zoo!.app.debug_send_home('monkey'));
  await wait(page, 2);
  const at = await page.evaluate(() => {
    const a = window.__zoo!.app;
    return [a.animal_x('monkey'), a.animal_z('monkey')];
  });
  let key = '';
  for (const [dx, dz] of [[0, -1.2], [1.2, 0], [-1.2, 0], [0, 1.2]]) {
    await teleport(page, at[0] + dx, at[1] + dz);
    await page.evaluate(() => {
      const a = window.__zoo!.app;
      a.debug_face_animal('monkey');
      a.debug_step(0.05);
    });
    await nextFrames(page, 2);
    key = await page.evaluate(() => window.__zoo!.app.target_key());
    if (key === 'treat:monkey') break;
  }
  expect(key).toBe('treat:monkey');
  await page.keyboard.press('KeyE');
  await nextFrames(page, 2);
  const after = await basket(page);
  expect(after.apple + after.orange).toBe(2);
  expect(await page.evaluate(() => window.__zoo!.app.debug_reacting_views('monkey'))).toBeGreaterThanOrEqual(1);
  expect(await page.evaluate(() => window.__zoo!.app.debug_baby_count())).toBe(1);
  expect(errors).toEqual([]);
});
