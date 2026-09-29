// GAME-SAVE in the browser (SAVE-002/003/004/008): a page reload continues where the child
// stopped — progress and the last positions of the player and the animals.
import { expect, test, type Page } from '@playwright/test';
import { face, goto, nextFrames, state, waitFrames, START_URL, approach } from './helpers';

test.describe.configure({ timeout: 240_000 });
test.use({ viewport: { width: 1280, height: 720 } });

/** Clean start: storage cleared on the first load only (reloads keep the save). */
async function start(page: Page) {
  await page.addInitScript(() => {
    if (sessionStorage.getItem('zoo.e2e.init')) return;
    sessionStorage.setItem('zoo.e2e.init', '1');
    localStorage.clear();
  });
  const errors: string[] = [];
  page.on('console', (m) => {
    if (m.type() === 'error') errors.push(m.text());
  });
  page.on('pageerror', (e) => errors.push(String(e)));
  await page.goto(START_URL);
  await waitFrames(page, 3);
  return errors;
}

async function reload(page: Page) {
  await page.reload();
  await waitFrames(page, 3);
}

async function takeGrass(page: Page) {
  await goto(page, -1.5, 9.6); // the grass box in the row outside (Q-181)
  await face(page, 'KeyW');
  await expect(page.locator('#take')).toBeVisible();
  await page.locator('#take').click();
  expect((await state(page)).carry).toBe('grass');
}

async function zebraFollows(page: Page) {
  await approach(page, 'zebra');
  await page.keyboard.press('KeyE');
  await expect.poll(async () => (await state(page)).zebra).toBe('following');
}

const zebraDistance = (page: Page) =>
  page.evaluate(() => {
    const a = window.__zoo!.app;
    return Math.hypot(a.animal_x('zebra') - a.player_x(), a.animal_z('zebra') - a.player_z());
  });

test('SAVE-004: position, facing and carried food survive a reload', async ({ page }) => {
  const errors = await start(page);
  await takeGrass(page);
  await goto(page, 3.2, 6.4);
  await face(page, 'KeyD'); // face east
  const before = await page.evaluate(() => {
    const a = window.__zoo!.app;
    return { x: a.player_x(), z: a.player_z(), fx: a.player_facing_x(), fz: a.player_facing_z() };
  });
  await reload(page);
  const after = await page.evaluate(() => {
    const a = window.__zoo!.app;
    return { x: a.player_x(), z: a.player_z(), fx: a.player_facing_x(), fz: a.player_facing_z(), carry: a.carry_food() };
  });
  expect(Math.abs(after.x - before.x)).toBeLessThanOrEqual(0.05);
  expect(Math.abs(after.z - before.z)).toBeLessThanOrEqual(0.05);
  expect(Math.abs(after.fx - before.fx)).toBeLessThanOrEqual(0.01);
  expect(Math.abs(after.fz - before.fz)).toBeLessThanOrEqual(0.01);
  expect(after.carry).toBe('grass');
  await expect(page.locator('#hud-carry')).toBeVisible();
  expect(errors).toEqual([]);
});

test('SAVE-002: a following zebra follows again after a reload, standing behind the player', async ({ page }) => {
  const errors = await start(page);
  await takeGrass(page);
  await zebraFollows(page);
  await goto(page, 6.5, 8.5); // lead it a bit along the ring path
  await page.evaluate(() => window.__zoo!.app.debug_step(3));
  expect(await zebraDistance(page)).toBeLessThan(3);
  await reload(page);
  expect((await state(page)).zebra).toBe('following');
  expect(await zebraDistance(page)).toBeLessThan(3);
  // it keeps following
  await goto(page, 2.5, 8.5);
  await page.evaluate(() => window.__zoo!.app.debug_step(3));
  expect(await zebraDistance(page)).toBeLessThan(3);
  expect(errors).toEqual([]);
});

test('SAVE-003: a completed mission stays completed after a reload, no second celebration', async ({ page }) => {
  const errors = await start(page);
  await takeGrass(page);
  await zebraFollows(page);
  await goto(page, -8.4, 13.0);
  await page.evaluate(() => window.__zoo!.app.debug_step(3));
  await face(page, 'KeyA');
  expect((await state(page)).target).toBe('gate:enc_zebra');
  await page.keyboard.press('KeyE');
  expect((await state(page)).complete).toBe(true);
  await expect(page.locator('#celebrate-text')).toBeVisible();
  await page.evaluate(() => window.__zoo!.app.debug_step(3));
  await reload(page);
  const s = await state(page);
  expect(s.zebra).toBe('in_enclosure');
  expect(s.complete).toBe(true);
  await nextFrames(page, 10);
  await expect(page.locator('#celebrate')).toBeHidden();
  expect(errors).toEqual([]);
});

test('SAVE-008: "new game" in the settings deletes the save and restarts the level', async ({ page }) => {
  const errors = await start(page);
  await takeGrass(page);
  await page.evaluate(() => window.__zoo!.slot.flush());
  expect(await page.evaluate(() => localStorage.getItem('zoo.save'))).toContain('"carry":"grass"');
  await page.locator('#settings-btn').click();
  await page.locator('#new-game').click();
  await expect(page.locator('#new-game-confirm')).toBeVisible();
  // "no" keeps the game
  await page.locator('#new-game-no').click();
  await expect(page.locator('#new-game-confirm')).toBeHidden();
  expect((await state(page)).carry).toBe('grass');
  await page.locator('#new-game').click();
  await Promise.all([page.waitForEvent('load'), page.locator('#new-game-yes').click()]);
  await waitFrames(page, 3);
  const s = await state(page);
  expect(s.carry).toBe('');
  expect(s.x).toBeCloseTo(0.5, 3); // spawn
  expect(s.z).toBeCloseTo(2.5, 3);
  expect(s.started).toBe(false);
  expect(errors).toEqual([]);
});
