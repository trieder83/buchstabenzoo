// PLAY-037 (small-screen right-hand control column), CAMV-027 (one view button fits), HINT-023
// (compass strip collapsed on small screens). User report 2026-10-03: Samsung phone landscape.
import fs from 'node:fs';
import path from 'node:path';
import { expect, test, type Page } from '@playwright/test';
import { nextFrames, repo, START_URL, waitFrames } from './helpers';

test.use({ hasTouch: true });

const SHOTS = path.join(repo, 'qa/reports/img');
const CONTROLS = ['#settings-btn', '#compass-btn', '#view-btn', '#act', '#hud-carry', '#drop-btn', '#stick'];
const SIZES = [
  ['780x360', 780, 360],
  ['360x780', 360, 780],
  ['412x892', 412, 892],
  ['892x412', 892, 412],
] as const;

async function open(page: Page, w: number, h: number) {
  await page.setViewportSize({ width: w, height: h });
  await page.addInitScript(() => {
    if (sessionStorage.getItem('zoo.e2e.init')) return;
    sessionStorage.setItem('zoo.e2e.init', '1');
    localStorage.clear();
  });
  await page.goto(START_URL);
  await waitFrames(page, 3);
  await page.locator('#game').tap({ position: { x: w / 2, y: h / 2 } }); // first touch: touch UI
  await nextFrames(page, 3);
  // show the controls that only appear in some states
  await page.evaluate(() => {
    for (const id of ['act', 'drop-btn', 'hud-carry']) {
      const e = document.getElementById(id)!;
      e.hidden = false;
      if (id === 'act') e.textContent = '✋';
      if (id === 'hud-carry') e.textContent = '🧺';
    }
  });
}

type Box = { id: string; x: number; y: number; w: number; h: number };
async function boxes(page: Page, sels: string[]): Promise<Box[]> {
  const out: Box[] = [];
  for (const sel of sels) {
    const loc = page.locator(sel).first();
    if ((await loc.count()) === 0 || !(await loc.isVisible())) continue;
    const b = await loc.boundingBox();
    if (b && b.width > 0) out.push({ id: sel, x: b.x, y: b.y, w: b.width, h: b.height });
  }
  return out;
}

function expectClean(bs: Box[], vp: { width: number; height: number }, label: string) {
  for (const b of bs) {
    expect(b.x, `${label} ${b.id} left`).toBeGreaterThanOrEqual(0);
    expect(b.y, `${label} ${b.id} top`).toBeGreaterThanOrEqual(0);
    expect(b.x + b.w, `${label} ${b.id} right`).toBeLessThanOrEqual(vp.width + 0.5);
    expect(b.y + b.h, `${label} ${b.id} bottom`).toBeLessThanOrEqual(vp.height + 0.5);
  }
  for (let i = 0; i < bs.length; i++) {
    for (let j = i + 1; j < bs.length; j++) {
      const a = bs[i];
      const c = bs[j];
      const overlap = a.x < c.x + c.w && c.x < a.x + a.w && a.y < c.y + c.h && c.y < a.y + a.h;
      expect(overlap, `${label}: ${a.id} overlaps ${c.id}`).toBe(false);
    }
  }
}

for (const [name, w, h] of SIZES) {
  test(`PLAY-037 / CAMV-027 (${name}): one right-hand column, nothing overlaps, nothing leaves the viewport`, async ({ page }) => {
    await open(page, w, h);
    const bs = await boxes(page, CONTROLS);
    expect(bs.length).toBeGreaterThanOrEqual(6);
    expectClean(bs, { width: w, height: h }, name);
    const by = Object.fromEntries(bs.map((b) => [b.id, b]));
    const gear = by['#settings-btn'];
    const compass = by['#compass-btn'];
    const view = by['#view-btn'];
    const act = by['#act'];
    for (const b of [gear, compass, view]) {
      expect(b.w, `${name} ${b.id} width`).toBeGreaterThanOrEqual(64);
      expect(b.h).toBeGreaterThanOrEqual(64);
    }
    // one column along the right border, in the order gear, compass, view button, interact button
    const cx = (b: Box) => b.x + b.w / 2;
    expect(Math.abs(cx(gear) - cx(compass))).toBeLessThan(1);
    expect(Math.abs(cx(view) - cx(compass))).toBeLessThan(1);
    expect(w - (gear.x + gear.w)).toBeLessThanOrEqual(20);
    expect(compass.y - (gear.y + gear.h)).toBeCloseTo(8, 0);
    expect(view.y).toBeGreaterThanOrEqual(compass.y + compass.h);
    expect(view.y + view.h).toBeLessThanOrEqual(act.y);
    expect(Math.abs(cx(act) - cx(view))).toBeLessThan(4);
    // right of the joystick
    expect(view.x).toBeGreaterThan(by['#stick'].x + by['#stick'].w);
    fs.mkdirSync(SHOTS, { recursive: true });
    await page.screenshot({ path: path.join(SHOTS, `2026-10-03-small-screen-${name}.png`) });
  });

  test(`HINT-023 (${name}): the compass strip is collapsed, a compass tap opens it for ~6 s`, async ({ page }) => {
    await open(page, w, h);
    const c = page.locator('#compass-btn');
    const strip = c.locator('.strip .pa');
    await expect(strip).toHaveCount(3);
    await expect(strip.first()).toBeHidden();
    await expect(c.locator('.badge')).toBeVisible();
    await expect(c).toHaveAttribute('data-missing', '3');
    await c.tap();
    await expect(c).toHaveClass(/expanded/);
    await expect(strip.first()).toBeVisible();
    const bs = await boxes(page, [...CONTROLS, '#compass-btn .strip']);
    const st = bs.find((b) => b.id.includes('.strip'))!;
    const cb = bs.find((b) => b.id === '#compass-btn')!;
    expect(st.x + st.w).toBeLessThanOrEqual(cb.x); // to the left of the compass
    expectClean(bs, { width: w, height: h }, `${name} expanded`);
    if (name === '780x360') {
      await page.screenshot({ path: path.join(SHOTS, `2026-10-03-small-screen-${name}-strip.png`) });
    }
    // any other tap collapses it
    await page.locator('#game').tap({ position: { x: w / 2, y: h / 3 } });
    await expect(c).not.toHaveClass(/expanded/);
    await expect(strip.first()).toBeHidden();
    // and it closes by itself after ~6 s
    await c.tap();
    await expect(strip.first()).toBeVisible();
    await page.waitForTimeout(5000);
    await expect(strip.first()).toBeVisible();
    await expect(strip.first()).toBeHidden({ timeout: 3000 });
  });
}

test('HINT-023: on a large screen the strip stays visible without a tap', async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 720 });
  await page.goto(START_URL);
  await waitFrames(page, 3);
  await expect(page.locator('#compass-btn .strip .pa').first()).toBeVisible();
});

test('the page declares a light colour scheme (no forced dark mode)', async ({ page }) => {
  await page.goto(START_URL);
  expect(await page.locator('meta[name="color-scheme"]').getAttribute('content')).toBe('light');
  expect(await page.evaluate(() => getComputedStyle(document.documentElement).colorScheme)).toBe('light');
});
