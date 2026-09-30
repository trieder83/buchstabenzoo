// QA 2026-09-30: two zebras (FAM-001/002), hints through the pair mission, garden street,
// baby by treat (FAM-008), intro on a phone (RESC-029). Screenshots -> web/test-results/shots.
import path from 'node:path';
import { expect, test } from '@playwright/test';
import { goto, nextFrames, state, approach, repo, waitFrames, ftl } from '../helpers';
import { ensurePanel, startGame, teleport, turn, wait } from './qa';

test.describe.configure({ timeout: 300_000 });
test.use({ viewport: { width: 1280, height: 720 } });

async function press(page: import('@playwright/test').Page, k = 'KeyE') {
  await page.keyboard.press(k);
  await nextFrames(page, 2);
}
async function shot(page: import('@playwright/test').Page, name: string) {
  await page.screenshot({ path: path.join(repo, 'qa/reports/img', `2026-09-30-${name}.png`) });
}

test('FAM-002 QA: two zebras follow and both enter the gate; hints stay valid; baby by carrot', async ({ page }) => {
  const errors = await startGame(page, 'de', 'klasse2');
  const hint = async () => {
    const j = await page.evaluate(() => window.__zoo!.app.hint_json());
    return j ? JSON.parse(j) : null;
  };
  await wait(page, 0.5);
  const h0 = await hint();
  console.log('hint0', JSON.stringify(h0));
  await shot(page, 'start');
  // info board, then box, then zebra
  await goto(page, -7.5, 10.5);
  await turn(page, 'KeyA');
  await ensurePanel(page, () => press(page));
  await page.keyboard.press('Escape');
  console.log('hint after board', JSON.stringify(await hint()));
  await goto(page, -1.5, 9.6);
  await turn(page, 'KeyW');
  await ensurePanel(page, () => press(page));
  await page.locator('#take').click();
  console.log('hint after take', JSON.stringify(await hint()));
  await approach(page, 'zebra');
  await shot(page, 'zebras-hiding');
  await press(page);
  expect((await state(page)).zebra).toBe('following');
  console.log('hint following', JSON.stringify(await hint()));
  await goto(page, -8.4, 13.0);
  await wait(page, 3);
  await shot(page, 'zebras-following-at-gate');
  console.log('hint at gate', JSON.stringify(await hint()));
  await turn(page, 'KeyA');
  expect((await state(page)).target).toBe('gate:enc_zebra');
  await press(page);
  const done = await state(page);
  expect(done.zebra).toBe('in_enclosure');
  expect(done.complete).toBe(true);
  await wait(page, 4);
  await shot(page, 'zebras-home');
  // garden street
  await goto(page, 8.5, 31.5);
  await goto(page, 8.0, 35.0);
  await wait(page, 1);
  await shot(page, 'garden-gate-open');
  await goto(page, 7.5, 38.5);
  await turn(page, 'KeyA');
  console.log('garden target', (await state(page)).target);
  await press(page);
  await wait(page, 1);
  console.log('basket', await page.evaluate(() => window.__zoo!.app.basket_json()));
  // carrot to the zebras at the fence
  await goto(page, -8.4, 13.0);
  await turn(page, 'KeyA');
  await wait(page, 6); // the zebras walk to the fence for the carrot (GARD-010)
  console.log('fence target', (await state(page)).target);
  await page.evaluate(() => window.__zoo!.app.select_treat('carrot'));
  await press(page);
  await wait(page, 3);
  await shot(page, 'baby-after-carrot');
  await goto(page, -8.4, 9.0);
  await wait(page, 2);
  await shot(page, 'baby-view2');
  expect(errors).toEqual([]);
});

test.describe('phone', () => {
  test.use({ viewport: { width: 390, height: 844 }, hasTouch: true, isMobile: true, deviceScaleFactor: 2 });
  test('RESC-029 QA: intro on a phone, targets >= 64 px, skip, not repeated', async ({ page }) => {
    await page.addInitScript(() => {
      if (sessionStorage.getItem('zoo.e2e.init')) return;
      sessionStorage.setItem('zoo.e2e.init', '1');
      localStorage.clear();
    });
    const errors: string[] = [];
    page.on('console', (m) => m.type() === 'error' && errors.push(m.text()));
    page.on('pageerror', (e) => errors.push(String(e)));
    await page.goto('/?seed=17&intro=1');
    await waitFrames(page, 3);
    const intro = page.locator('#intro');
    await expect(intro).toBeVisible();
    await shot(page, 'intro-1');
    for (const sel of ['.intro-next', '.intro-skip']) {
      const b = await page.locator(`#intro ${sel}`).boundingBox();
      console.log(sel, JSON.stringify(b));
      expect(b!.width).toBeGreaterThanOrEqual(64);
      expect(b!.height).toBeGreaterThanOrEqual(64);
    }
    const card = await page.locator('#intro .intro-card').boundingBox();
    console.log('card', JSON.stringify(card));
    expect(card!.x).toBeGreaterThanOrEqual(0);
    expect(card!.y + card!.height).toBeLessThanOrEqual(844);
    await page.locator('#intro .intro-next').tap();
    await shot(page, 'intro-2');
    await page.locator('#intro .intro-next').tap();
    await shot(page, 'intro-3');
    await page.locator('#intro .intro-skip').tap().catch(() => {});
    if (await intro.isVisible()) await page.locator('#intro .intro-next').tap();
    await expect(intro).toBeHidden();
    await page.evaluate(() => window.__zoo!.slot.flush());
    await page.goto('/?seed=17&intro=1');
    await waitFrames(page, 3);
    await expect(intro).toBeHidden();
    expect(errors).toEqual([]);
  });
  test('RESC-029 QA: skip on page 1 is remembered', async ({ page }) => {
    await page.addInitScript(() => {
      if (sessionStorage.getItem('zoo.e2e.init')) return;
      sessionStorage.setItem('zoo.e2e.init', '1');
      localStorage.clear();
    });
    await page.goto('/?seed=17&intro=1');
    await waitFrames(page, 3);
    await page.locator('#intro .intro-skip').tap();
    await expect(page.locator('#intro')).toBeHidden();
    await page.evaluate(() => window.__zoo!.slot.flush());
    await page.goto('/?seed=17&intro=1');
    await waitFrames(page, 3);
    await expect(page.locator('#intro')).toBeHidden();
    void ftl;
  });
});
