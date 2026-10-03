// TECH-PLATFORMS "Installable and full screen": PLAT-019 (button), PLAT-020 (real full screen
// in Chromium), PLAT-021 (HUD controls inside the viewport, no overlaps, both orientations).
import fs from 'node:fs';
import path from 'node:path';
import { expect, test, type Page } from '@playwright/test';
import { ftl, nextFrames, repo, START_URL, waitFrames } from './helpers';

test.use({ hasTouch: true });

const SHOTS = path.join(repo, 'qa/reports/img');
const CONTROLS = ['#settings-btn', '#compass-btn', '#act', '#view-btn', '#hud-carry', '#drop-btn', '#hint', '#compass-btn .strip'];

async function open(page: Page, w: number, h: number) {
  await page.setViewportSize({ width: w, height: h });
  await page.addInitScript(() => {
    if (sessionStorage.getItem('zoo.e2e.init')) return;
    sessionStorage.setItem('zoo.e2e.init', '1');
    localStorage.clear();
  });
  await page.goto(START_URL);
  await waitFrames(page, 3);
}

/** Boxes of the visible HUD controls; asserts each is inside the viewport and none overlap. */
async function checkHud(page: Page, label: string) {
  const vp = page.viewportSize()!;
  const boxes: { id: string; x: number; y: number; w: number; h: number }[] = [];
  for (const sel of CONTROLS) {
    const loc = page.locator(sel).first();
    if ((await loc.count()) === 0 || !(await loc.isVisible())) continue;
    const b = await loc.boundingBox();
    if (b && b.width > 0) boxes.push({ id: sel, x: b.x, y: b.y, w: b.width, h: b.height });
  }
  expect(boxes.length, label).toBeGreaterThanOrEqual(3);
  for (const b of boxes) {
    expect(b.x, `${label} ${b.id} left`).toBeGreaterThanOrEqual(0);
    expect(b.y, `${label} ${b.id} top`).toBeGreaterThanOrEqual(0);
    expect(b.x + b.w, `${label} ${b.id} right`).toBeLessThanOrEqual(vp.width + 0.5);
    expect(b.y + b.h, `${label} ${b.id} bottom`).toBeLessThanOrEqual(vp.height + 0.5);
  }
  // the strip is part of the compass button: compare only independent controls
  const indep = boxes.filter((b) => !b.id.includes('.strip'));
  for (let i = 0; i < indep.length; i++) {
    for (let j = i + 1; j < indep.length; j++) {
      const a = indep[i];
      const c = indep[j];
      const overlap = a.x < c.x + c.w && c.x < a.x + a.w && a.y < c.y + c.h && c.y < a.y + a.h;
      expect(overlap, `${label}: ${a.id} overlaps ${c.id}`).toBe(false);
    }
  }
}

for (const [name, w, h] of [
  ['portrait', 412, 892],
  ['landscape', 892, 412],
] as const) {
  test(`PLAT-021 HUD controls stay inside the viewport and do not overlap (${name})`, async ({ page }) => {
    await open(page, w, h);
    await nextFrames(page, 5);
    await checkHud(page, name);
    // the open settings menu fits the screen too (it scrolls when the screen is short)
    await page.locator('#settings-btn').click();
    const menu = (await page.locator('#settings').boundingBox())!;
    expect(menu.y).toBeGreaterThanOrEqual(0);
    expect(menu.y + menu.height).toBeLessThanOrEqual(h + 0.5);
    expect(menu.x + menu.width).toBeLessThanOrEqual(w + 0.5);
  });
}

test('PLAT-019/020 the full-screen button toggles real full screen and the game keeps running', async ({ page }) => {
  await open(page, 892, 412);
  const de = ftl('de');
  await page.locator('#settings-btn').click();
  const btn = page.locator('#fullscreen-toggle');
  await expect(btn).toBeVisible();
  await expect(btn).toHaveAttribute('aria-label', de['ui-fullscreen']);
  const bb = (await btn.boundingBox())!;
  expect(bb.width).toBeGreaterThanOrEqual(72);
  expect(bb.height).toBeGreaterThanOrEqual(72);
  // next to ❓
  const introBb = (await page.locator('#intro-replay').boundingBox())!;
  expect(Math.abs(bb.y - introBb.y)).toBeLessThan(2);
  await expect(btn).toHaveAttribute('aria-pressed', 'false');
  fs.mkdirSync(SHOTS, { recursive: true });
  await page.screenshot({ path: path.join(SHOTS, 'fullscreen-landscape-settings.png') });

  await btn.click();
  await expect.poll(() => page.evaluate(() => document.fullscreenElement !== null)).toBe(true);
  await expect(page.locator('#settings')).toBeHidden(); // the menu closes after the toggle
  await nextFrames(page, 5);
  const size = await page.evaluate(() => {
    const c = document.getElementById('game') as HTMLCanvasElement;
    return { cw: c.clientWidth, ch: c.clientHeight, w: window.innerWidth, h: window.innerHeight };
  });
  expect(size.cw).toBe(size.w);
  expect(size.ch).toBe(size.h);
  await checkHud(page, 'fullscreen');
  fs.mkdirSync(SHOTS, { recursive: true });
  await page.screenshot({ path: path.join(SHOTS, 'fullscreen-landscape.png') });

  await page.locator('#settings-btn').click();
  await expect(btn).toHaveAttribute('aria-pressed', 'true');
  await btn.click();
  await expect.poll(() => page.evaluate(() => document.fullscreenElement === null)).toBe(true);
  await page.locator('#settings-btn').click();
  await expect(btn).toHaveAttribute('aria-pressed', 'false');
  // leaving full screen (also via Esc) does not break the game
  const f = await page.evaluate(() => window.__zoo!.frames);
  await page.waitForFunction((m) => window.__zoo!.frames >= m + 5, f);
  expect(await page.evaluate(() => window.__zooError ?? null)).toBeNull();
});

test('PLAT-019 the button is hidden without the Fullscreen API (iPhone Safari)', async ({ page }) => {
  await page.addInitScript(() => {
    for (const k of ['fullscreenEnabled', 'webkitFullscreenEnabled']) {
      Object.defineProperty(Document.prototype, k, { get: () => false });
    }
  });
  await open(page, 412, 892);
  await page.locator('#settings-btn').click();
  await expect(page.locator('#settings')).toBeVisible();
  await expect(page.locator('#fullscreen-toggle')).toHaveCount(0);
  await page.screenshot({ path: path.join(SHOTS, 'fullscreen-portrait-settings.png') });
});

test.describe('iPhone Safari', () => {
  test.use({
    userAgent:
      'Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1',
  });
  test('PLAT-017/019 the install hint shows in the settings menu only (de text)', async ({ page }) => {
    await page.addInitScript(() => {
      for (const k of ['fullscreenEnabled', 'webkitFullscreenEnabled']) {
        Object.defineProperty(Document.prototype, k, { get: () => false });
      }
    });
    await open(page, 412, 892);
    await expect(page.locator('#install-hint')).toBeHidden();
    await page.locator('#settings-btn').click();
    await expect(page.locator('#install-hint')).toHaveText(ftl('de')['ui-install-hint-ios']);
    await expect(page.locator('#fullscreen-toggle')).toHaveCount(0);
    await page.screenshot({ path: path.join(SHOTS, 'install-hint-ios.png') });
  });
});
