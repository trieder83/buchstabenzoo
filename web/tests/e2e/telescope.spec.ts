// GAME-TELESCOPE e2e (TELE-007…010): at night the toy telescope opens the planet view; tapping
// Saturn shows name, type and the sentence of the reading level; the view fits 780x360 and
// 360x780; closing resumes the game; by day the telescope offers nothing.
import { expect, test, type Page } from '@playwright/test';
import { ftl, nextFrames, START_URL, waitFrames } from './helpers';

async function start(page: Page, w = 1280, h = 720, level = 'klasse2') {
  await page.setViewportSize({ width: w, height: h });
  await page.addInitScript((lv) => {
    if (sessionStorage.getItem('zoo.e2e.init')) return;
    sessionStorage.setItem('zoo.e2e.init', '1');
    localStorage.clear();
    localStorage.setItem('zoo.readingLevel', lv);
  }, level);
  const errors: string[] = [];
  page.on('console', (m) => {
    if (m.type() === 'error') errors.push(m.text());
  });
  page.on('pageerror', (e) => errors.push(String(e)));
  await page.goto(START_URL);
  await waitFrames(page, 3);
  return errors;
}

/** Level 1 home, then night (debug), the player standing in front of the telescope. */
async function atTelescope(page: Page, night = true) {
  await page.evaluate((night) => {
    const a = window.__zoo!.app;
    for (const id of ['zebra', 'hippo', 'panda']) a.debug_send_home(id);
    document.getElementById('celebrate')!.hidden = true;
    if (night) a.debug_set_daytime('night');
    // the telescope stands at (-52, 38); stand south of it facing it
    a.debug_teleport(-51.5, 36.6);
    a.debug_face_point(-51.5, 38.5);
  }, night);
  await nextFrames(page, 3);
}

async function inside(page: Page, sel: string) {
  const vp = page.viewportSize()!;
  const b = (await page.locator(sel).boundingBox())!;
  expect(b.x, sel).toBeGreaterThanOrEqual(-0.5);
  expect(b.y, sel).toBeGreaterThanOrEqual(-0.5);
  expect(b.x + b.width, sel).toBeLessThanOrEqual(vp.width + 0.5);
  expect(b.y + b.height, sel).toBeLessThanOrEqual(vp.height + 0.5);
  return b;
}

test('TELE-007/008 night: open at the telescope, tap Saturn, info box, close', async ({ page }) => {
  const errors = await start(page);
  const t = ftl('de');
  await atTelescope(page);
  await expect(page.locator('#act')).toHaveText('🔭');
  await page.keyboard.press('KeyE');
  await expect(page.locator('#telescope-view')).toBeVisible();
  await expect(page.locator('#telescope-view .tele-planet')).toHaveCount(8);
  await expect(page.locator('#telescope-info')).toContainText(t['telescope-hint']);
  for (const b of await page.locator('.tele-planet').all()) {
    const bb = (await b.boundingBox())!;
    expect(Math.min(bb.width, bb.height)).toBeGreaterThanOrEqual(63.5);
  }
  await page.locator('.tele-planet[data-planet="saturn"]').click();
  const info = page.locator('#telescope-info');
  await expect(info).toContainText(t['telescope-saturn-name']);
  await expect(info).toContainText(t['telescope-kind-gas']);
  await expect(info).toContainText(t['telescope-saturn-klasse2']);
  await expect(page.locator('.tele-planet.selected')).toHaveAttribute('data-planet', 'saturn');
  await page.locator('.tele-planet[data-planet="mars"]').click();
  await expect(info).toContainText(t['telescope-kind-rocky']);
  await expect(info).toContainText(t['telescope-mars-klasse2']);
  // paused: a held key does not move her (TELE-007)
  const p0 = await page.evaluate(() => [window.__zoo!.app.player_x(), window.__zoo!.app.player_z()]);
  await page.keyboard.down('KeyW');
  await page.waitForTimeout(400);
  await page.keyboard.up('KeyW');
  expect(await page.evaluate(() => [window.__zoo!.app.player_x(), window.__zoo!.app.player_z()])).toEqual(p0);
  await page.keyboard.press('Escape');
  await expect(page.locator('#telescope-view')).toBeHidden();
  // the ✖ button closes as well
  await page.keyboard.press('KeyE');
  await expect(page.locator('#telescope-view')).toBeVisible();
  await page.locator('#telescope-close').click();
  await expect(page.locator('#telescope-view')).toBeHidden();
  expect(errors).toEqual([]);
});

test('TELE-008 english follows the language', async ({ page }) => {
  await start(page, 1280, 720, 'klasse3');
  await page.evaluate(() => localStorage.setItem('zoo.language', 'en'));
  await page.reload();
  await waitFrames(page, 3);
  await atTelescope(page);
  await page.keyboard.press('KeyE');
  await page.locator('.tele-planet[data-planet="saturn"]').click();
  const en = ftl('en');
  await expect(page.locator('#telescope-info')).toContainText(en['telescope-saturn-klasse3']);
  await expect(page.locator('#telescope-info')).toContainText(en['telescope-kind-gas']);
});

for (const [w, h] of [
  [780, 360],
  [360, 780],
] as const) {
  test(`TELE-009 fits ${w}x${h}`, async ({ page }) => {
    await start(page, w, h, 'klasse3');
    await atTelescope(page);
    await page.keyboard.press('KeyE');
    await expect(page.locator('#telescope-view')).toBeVisible();
    await page.locator('.tele-planet[data-planet="saturn"]').click();
    const boxes: Record<string, { x: number; y: number; width: number; height: number }> = {};
    for (const id of ['mercury', 'venus', 'earth', 'mars', 'jupiter', 'saturn', 'uranus', 'neptune']) {
      boxes[id] = await inside(page, `.tele-planet[data-planet="${id}"]`);
      expect(Math.min(boxes[id].width, boxes[id].height)).toBeGreaterThanOrEqual(63.5);
    }
    const info = await inside(page, '#telescope-info');
    const close = await inside(page, '#telescope-close');
    expect(Math.min(close.width, close.height)).toBeGreaterThanOrEqual(63.5);
    const overlap = (a: typeof info, b: typeof info) =>
      a.x < b.x + b.width && b.x < a.x + a.width && a.y < b.y + b.height && b.y < a.y + a.height;
    const ids = Object.keys(boxes);
    for (const id of ids) {
      expect(overlap(boxes[id], info), `${id} vs info`).toBe(false);
      expect(overlap(boxes[id], close), `${id} vs close`).toBe(false);
      for (const o of ids) if (o < id) expect(overlap(boxes[id], boxes[o]), `${id} vs ${o}`).toBe(false);
    }
    await expectLineOrder(page, boxes);
    // the long sentence is fully visible inside the info box
    const scroll = await page.locator('#telescope-info').evaluate((e) => e.scrollHeight - e.clientHeight);
    expect(scroll).toBeLessThanOrEqual(1);
    await page.screenshot({ path: `test-results/shots/telescope-${w}x${h}.png` });
  });
}

/** TELE-012: Sun at the lower left, planets Mercury..Neptune from lower left to upper right. */
async function expectLineOrder(page: Page, boxes?: Record<string, { x: number; y: number; width: number; height: number }>) {
  const ids = ['mercury', 'venus', 'earth', 'mars', 'jupiter', 'saturn', 'uranus', 'neptune'];
  const sky = (await page.locator('.tele-sky').boundingBox())!;
  const sun = (await page.locator('.tele-sun').boundingBox())!;
  const scx = sun.x + sun.width / 2;
  const scy = sun.y + sun.height / 2;
  expect(Math.abs(scx - sky.x)).toBeLessThan(2);
  expect(Math.abs(scy - (sky.y + sky.height))).toBeLessThan(2);
  const c: { x: number; y: number }[] = [];
  for (const id of ids) {
    const b = boxes?.[id] ?? (await page.locator(`.tele-planet[data-planet="${id}"]`).boundingBox())!;
    c.push({ x: b.x + b.width / 2, y: b.y + b.height / 2 });
  }
  expect(c[0].x).toBeGreaterThan(scx);
  for (let i = 1; i < c.length; i++) {
    expect(c[i].x, ids[i]).toBeGreaterThan(c[i - 1].x);
    expect(c[i].y, ids[i]).toBeLessThan(c[i - 1].y);
  }
}

test('TELE-012 line order at 1280x720', async ({ page }) => {
  await start(page, 1280, 720, 'klasse2');
  await atTelescope(page);
  await page.keyboard.press('KeyE');
  await expect(page.locator('#telescope-view')).toBeVisible();
  await expectLineOrder(page);
  await page.screenshot({ path: 'test-results/shots/telescope-1280x720.png' });
});

test('TELE-010 by day the telescope offers nothing', async ({ page }) => {
  await start(page);
  await atTelescope(page, false);
  await expect(page.locator('#act')).not.toHaveText('🔭');
  await page.keyboard.press('KeyE');
  await expect(page.locator('#telescope-view')).toBeHidden();
});
