// GAME-MAP overview map (MAP-006, MAP-017…021): button in the settings, level states, player
// marker, close, input paused while open, fit on small screens.
import { expect, test, type Page } from '@playwright/test';
import { ftl, nextFrames, shots, START_URL, waitFrames } from './helpers';

async function start(page: Page, w = 1280, h = 720) {
  await page.setViewportSize({ width: w, height: h });
  await page.addInitScript(() => {
    if (sessionStorage.getItem('zoo.e2e.init')) return;
    sessionStorage.setItem('zoo.e2e.init', '1');
    localStorage.clear();
  });
  await page.goto(START_URL);
  await waitFrames(page, 3);
}

async function openMap(page: Page) {
  await page.locator('#settings-btn').click();
  await page.locator('#map-btn').click();
  await expect(page.locator('#overview-map')).toBeVisible();
}

// MAP-017, MAP-018
test('MAP-017/018 map button in the settings opens the overview with player and level states', async ({ page }) => {
  await start(page);
  await page.locator('#settings-btn').click();
  const btn = page.locator('#map-btn');
  await expect(btn).toBeVisible();
  const b = (await btn.boundingBox())!;
  expect(Math.min(b.width, b.height)).toBeGreaterThanOrEqual(64);
  expect(await page.evaluate(() => document.querySelector('#settings')!.firstElementChild!.id)).toBe('settings-map');
  await btn.click();
  await expect(page.locator('#overview-map')).toBeVisible();
  await expect(page.locator('#settings')).toBeHidden();
  await expect(page.locator('#overview-map .map-player')).toHaveCount(1);
  const lv = (id: string) => page.locator(`#overview-map .map-level[data-level="${id}"]`);
  await expect(lv('level_1')).toHaveAttribute('data-state', 'open');
  await expect(lv('level_1')).toContainText('0/3');
  await expect(lv('level_2')).toHaveAttribute('data-state', 'locked');
  await expect(lv('level_2')).toContainText('🔒');
  await expect(lv('level_3')).toHaveAttribute('data-state', 'locked');
  await expect(lv('night_1')).toHaveAttribute('data-night', 'true');
  await expect(lv('night_1')).toContainText('🌙');
  // enclosures show their animals at home only as ✔; escaped animals are never listed
  await expect(page.locator('#overview-map .map-enclosure[data-animal="zebra"]')).toHaveAttribute('data-home', 'false');
  await expect(page.locator('#overview-map .map-enclosure .check')).toHaveCount(0);
  // all of level 1 home: solved + ✔
  await page.locator('#map-close').click();
  await page.evaluate(() => {
    const a = window.__zoo!.app;
    for (const id of ['zebra', 'hippo', 'panda']) a.debug_send_home(id);
  });
  await nextFrames(page, 2);
  await page.keyboard.press('KeyM');
  await expect(lv('level_1')).toHaveAttribute('data-state', 'solved');
  await expect(lv('level_1')).toContainText('3/3 ✔');
  await expect(page.locator('#overview-map .map-enclosure[data-home="true"] .check')).toHaveCount(3);
});

// MAP-019, MAP-006
test('MAP-019/006 close with the button, Esc and M; input is paused while open', async ({ page }) => {
  await start(page);
  await openMap(page);
  await page.locator('#map-close').click();
  await expect(page.locator('#overview-map')).toBeHidden();
  await page.keyboard.press('KeyM');
  await expect(page.locator('#overview-map')).toBeVisible();
  await page.keyboard.press('Escape');
  await expect(page.locator('#overview-map')).toBeHidden();
  await page.keyboard.press('KeyM');
  await expect(page.locator('#overview-map')).toBeVisible();
  // paused: a held movement key does not move her
  const z0 = await page.evaluate(() => window.__zoo!.app.player_z());
  await page.keyboard.down('KeyW');
  await nextFrames(page, 20);
  await page.keyboard.up('KeyW');
  expect(await page.evaluate(() => window.__zoo!.app.player_z())).toBeCloseTo(z0, 3);
  await page.keyboard.press('KeyM');
  await expect(page.locator('#overview-map')).toBeHidden();
  await page.keyboard.down('KeyW');
  await nextFrames(page, 20);
  await page.keyboard.up('KeyW');
  expect(await page.evaluate(() => window.__zoo!.app.player_z())).toBeGreaterThan(z0 + 0.2);
});

// MAP-020
for (const [w, h] of [
  [780, 360],
  [360, 780],
] as const) {
  test(`MAP-020 map and close button fit ${w}x${h}`, async ({ page }) => {
    await start(page, w, h);
    await openMap(page);
    await nextFrames(page, 2);
    await page.screenshot({ path: `${shots}/map-${w}x${h}.png` });
    const box = async (sel: string) => (await page.locator(sel).boundingBox())!;
    const stage = await box('#overview-map .map-stage');
    expect(stage.x).toBeGreaterThanOrEqual(0);
    expect(stage.y).toBeGreaterThanOrEqual(0);
    expect(stage.x + stage.width).toBeLessThanOrEqual(w + 0.5);
    expect(stage.y + stage.height).toBeLessThanOrEqual(h + 0.5);
    const close = await box('#map-close');
    expect(close.width).toBeGreaterThanOrEqual(64);
    expect(close.x + close.width).toBeLessThanOrEqual(w);
    expect(close.y + close.height).toBeLessThanOrEqual(h);
    // the player marker is inside the map
    const p = await box('#overview-map .map-player .dot');
    expect(p.x).toBeGreaterThanOrEqual(stage.x);
    expect(p.x + p.width).toBeLessThanOrEqual(stage.x + stage.width + 1);
    // the close button does not cover the map drawing
    const overlaps = !(close.x + close.width <= stage.x || close.x >= stage.x + stage.width || close.y + close.height <= stage.y || close.y >= stage.y + stage.height);
    expect(overlaps).toBe(false);
  });
}

// MAP-021
test('MAP-021 map title follows the language', async ({ page }) => {
  await start(page);
  await openMap(page);
  const lang = await page.evaluate(() => window.__zoo!.app.language());
  const de = ftl('de');
  const en = ftl('en');
  expect(await page.locator('#overview-map .map-title').textContent()).toBe(lang === 'de' ? de['map-title'] : en['map-title']);
  await page.locator('#map-close').click();
  await page.evaluate(() => window.__zoo!.app.set_language('en'));
  await page.keyboard.press('KeyM');
  await expect(page.locator('#overview-map .map-title')).toHaveText(en['map-title']);
  await expect(page.locator('#overview-map .map-level[data-level="level_1"]')).toContainText(en['map-level-level_1']);
});
