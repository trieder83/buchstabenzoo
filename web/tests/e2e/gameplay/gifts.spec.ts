// GARD-012: giving directly at a home animal (user report 2026-09-30): walk up to the zebra
// pair with 4 carrots, give: both zebras react, carrots leave the basket, one baby.
import { expect, test } from '@playwright/test';
import { nextFrames } from '../helpers';
import { startGame, teleport, wait } from './qa';

test.describe.configure({ timeout: 300_000 });
test.use({ viewport: { width: 1280, height: 720 } });

test('GARD-012: carrots given directly to the home zebra pair', async ({ page }) => {
  const errors = await startGame(page, 'de', 'klasse2');
  await page.evaluate(() => {
    const a = window.__zoo!.app;
    a.debug_send_home('zebra');
    a.debug_give_treats('carrot', 4);
  });
  await wait(page, 2);
  const basket = async () => JSON.parse(await page.evaluate(() => window.__zoo!.app.basket_json()));
  expect((await basket()).carrot).toBe(4);
  // stand 1.2 m from the nearer zebra, inside its enclosure, facing it
  const at = await page.evaluate(() => {
    const a = window.__zoo!.app;
    return [a.animal_x('zebra'), a.animal_z('zebra')];
  });
  await teleport(page, at[0], at[1] - 1.2);
  await page.evaluate(() => {
    const a = window.__zoo!.app;
    a.debug_face_animal('zebra');
    a.debug_step(0.05);
  });
  await nextFrames(page, 2);
  expect(await page.evaluate(() => window.__zoo!.app.target_key())).toBe('treat:zebra');
  await page.keyboard.press('KeyE');
  await nextFrames(page, 2);
  expect(await page.evaluate(() => window.__zoo!.app.debug_reacting_views('zebra'))).toBe(2);
  expect((await basket()).carrot).toBe(3);
  expect(await page.evaluate(() => window.__zoo!.app.debug_baby_count())).toBe(1);
  // FAM-011/013: the baby is inside the fence beside the female
  expect(await page.evaluate(() => window.__zoo!.app.debug_baby_inside('zebra'))).toBe(true);
  expect(await page.evaluate(() => window.__zoo!.app.debug_baby_gap('zebra'))).toBeLessThan(6);
  // a second gift: the carrot leaves the basket, no second baby
  await wait(page, 4);
  await page.keyboard.press('KeyE');
  await nextFrames(page, 2);
  expect((await basket()).carrot).toBe(2);
  expect(await page.evaluate(() => window.__zoo!.app.debug_baby_count())).toBe(1);
  expect(errors).toEqual([]);
});
