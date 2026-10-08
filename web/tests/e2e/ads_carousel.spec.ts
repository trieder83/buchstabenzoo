// ADS-032..034 / HINT-031: the all-done carousel of the signed campaigns behind the 🧭 target.
// Test build only (trusts the fixture key): `E2E_DIST=dist-adtest scripts/e2e.sh web/tests/e2e/ads_carousel.spec.ts`.
// "All levels solved" is simulated by making `compass_json` report `next = all_done` (the Rust side
// is covered by HINT-028/030); the carousel itself is the host code under test.
import fs from 'node:fs';
import path from 'node:path';
import { expect, test, type Page } from '@playwright/test';
import { repo, waitFrames } from './helpers';

const fixtures = path.join(repo, 'web/tests/fixtures/ads');
const adsDir = path.join(fixtures, 'boards');
const PUB = fs.readFileSync(path.join(fixtures, 'TEST-ONLY-public.key'), 'utf8').trim();
test.skip(process.env.E2E_DIST !== 'dist-adtest', 'needs the ad test build (dist-adtest)');

async function serveAds(page: Page): Promise<void> {
  await page.route('**/boards/**', async (route) => {
    const rel = new URL(route.request().url()).pathname.replace(/^.*\/boards\//, '');
    const f = path.join(adsDir, rel);
    if (!rel || !fs.existsSync(f)) return route.fulfill({ status: 404 });
    return route.fulfill({ status: 200, body: fs.readFileSync(f), contentType: rel.endsWith('.png') ? 'image/png' : 'text/plain' });
  });
}

async function start(page: Page, opts: { key?: boolean; next?: string } = {}): Promise<void> {
  await page.addInitScript(() => {
    if (sessionStorage.getItem('zoo.e2e.init')) return;
    sessionStorage.setItem('zoo.e2e.init', '1');
    localStorage.clear();
    localStorage.setItem('zoo.readingLevel', 'klasse2');
    localStorage.setItem('zoo.language', 'de');
    (window as unknown as { __opens: unknown[] }).__opens = [];
    window.open = (...a: unknown[]) => {
      (window as unknown as { __opens: unknown[] }).__opens.push(a);
      return null;
    };
  });
  await serveAds(page);
  await page.goto(`/?seed=17${opts.key === false ? '' : `&adkey=${encodeURIComponent(PUB)}`}`);
  await waitFrames(page, 3);
  await page.waitForFunction(() => window.__zoo!.ads.state().settled, undefined, { timeout: 30_000 });
  await forceNext(page, opts.next ?? 'all_done');
}

/** Makes the compass report `next` (all levels solved = `all_done`). */
async function forceNext(page: Page, next: string): Promise<void> {
  await page.evaluate((n) => {
    const app = window.__zoo!.app as unknown as { compass_json: () => string; __orig?: () => string };
    app.__orig ??= app.compass_json.bind(app);
    const orig = app.__orig;
    app.compass_json = () => JSON.stringify({ ...JSON.parse(orig()), next: n });
  }, next);
}

const opens = (page: Page) => page.evaluate(() => (window as unknown as { __opens: unknown[][] }).__opens);
const slide = (page: Page) => page.locator('#zb-carousel .zb-body').getAttribute('data-card');

test('ADS-032 HINT-031 compass tap at all_done opens the carousel; arrows, dots, timer, swipe, close', async ({ page }) => {
  await start(page);
  await page.locator('#compass-btn').click();
  const car = page.locator('#zb-carousel');
  await expect(car).toBeVisible();
  await expect(page.locator('#zb-car-title')).toHaveText('Du magst Bildungsabenteuer? Schau mal hier!');
  expect(await slide(page)).toBe('mathfighter');
  const dots = page.locator('.zb-car-dot');
  expect(await dots.count()).toBe(3);
  for (const sel of ['#zb-car-prev', '#zb-car-next']) {
    const r = (await page.locator(sel).boundingBox())!;
    expect(r.width).toBeGreaterThanOrEqual(64);
    expect(r.height).toBeGreaterThanOrEqual(64);
  }
  await page.locator('#zb-car-next').click();
  expect(await slide(page)).toBe('abcsmash');
  await page.locator('#zb-car-next').click();
  await page.locator('#zb-car-next').click();
  expect(await slide(page), 'wraps around').toBe('mathfighter');
  await page.locator('#zb-car-prev').click();
  expect(await slide(page)).toBe('edugamegalaxy');
  await dots.nth(1).click();
  expect(await slide(page)).toBe('abcsmash');
  expect(await page.locator('.zb-car-dot.on').count()).toBe(1);
  // auto-advance after ~4 s
  await expect.poll(() => slide(page), { timeout: 7000 }).toBe('edugamegalaxy');
  // swipe on the picture turns the page without opening the gate
  const box = (await page.locator('#zb-car-image').boundingBox())!;
  const y = box.y + box.height / 2;
  await page.mouse.move(box.x + box.width * 0.8, y);
  await page.mouse.down();
  await page.mouse.move(box.x + box.width * 0.2, y, { steps: 5 });
  await page.mouse.up();
  expect(await slide(page)).toBe('mathfighter');
  await expect(page.locator('#zb-gate')).toBeHidden();
  await page.waitForTimeout(500);
  await page.locator('.zb-x', { hasText: '✖' }).first().click();
  await expect(car).toBeHidden();
  // Esc closes too
  await page.locator('#compass-btn').click();
  await expect(car).toBeVisible();
  await page.keyboard.press('Escape');
  await expect(car).toBeHidden();
  // English
  await page.evaluate(() => window.__zoo!.app.set_language('en'));
  await page.locator('#compass-btn').click();
  await expect(page.locator('#zb-car-title')).toHaveText('You like educational adventures? Check this out!');
});

test('ADS-033 tapping the picture runs the parental gate, release opens the shown link once', async ({ page }) => {
  await start(page);
  await page.locator('#compass-btn').click();
  await expect(page.locator('#zb-carousel')).toBeVisible();
  await page.locator('#zb-car-next').click(); // abcsmash: language gate
  await page.locator('#zb-car-image').click();
  await expect(page.locator('#zb-gate')).toBeVisible();
  expect(await opens(page)).toEqual([]);
  // the right answer: the gate shows the article / plural question; take the first choice that
  // leads to the hold stage, else retry with a fresh gate
  const label = (await page.locator('#zb-gate-question').textContent())!;
  expect(label.length).toBeGreaterThan(0);
  const idx = await page.evaluate(() => window.__zoo!.ads.state().gate);
  expect(idx).toBe('sum');
  // timer frozen under the gate
  const before = await slide(page);
  await page.waitForTimeout(4600);
  expect(await slide(page)).toBe(before);
  await page.locator('#zb-gate .zb-x').click();
  await expect(page.locator('#zb-gate')).toBeHidden();
  await expect(page.locator('#zb-carousel')).toBeVisible();
  // maths campaign: sum gate, full flow
  await page.locator('#zb-car-prev').click();
  await page.locator('#zb-car-image').click();
  const q = (await page.locator('#zb-gate-question').textContent())!;
  const [, a, op, b] = /(\d+) ([+−]) (\d+)/.exec(q)!;
  const answer = op === '+' ? Number(a) + Number(b) : Number(a) - Number(b);
  await page.locator('.zb-choice', { hasText: new RegExp(`^${answer}$`) }).click();
  const hold = (await page.locator('#zb-hold').boundingBox())!;
  await page.mouse.move(hold.x + hold.width / 2, hold.y + hold.height / 2);
  await page.mouse.down();
  await expect(page.locator('#zb-hold.ready')).toBeVisible({ timeout: 8000 });
  expect(await opens(page)).toEqual([]);
  await page.mouse.up();
  await page.waitForTimeout(500);
  expect(await opens(page)).toEqual([['https://mathfighter.rcms.ch/', '_blank', 'noopener,noreferrer']]);
  await expect(page.locator('#zb-carousel')).toBeHidden();
  await expect(page.locator('#zb-gate')).toBeHidden();
});

test('ADS-034 no carousel without verified campaign or when next is not all_done', async ({ page }) => {
  await start(page, { key: false });
  await page.locator('#compass-btn').click();
  await expect(page.locator('#zb-carousel')).toBeHidden();
  await expect(page.locator('#bubble')).toHaveAttribute('data-key', /all_done/);
});

test('ADS-034 next other than all_done does not open it', async ({ page }) => {
  await start(page, { next: 'explore' });
  await page.locator('#compass-btn').click();
  await expect(page.locator('#zb-carousel')).toBeHidden();
});

for (const vp of [
  { w: 780, h: 360 },
  { w: 360, h: 780 },
]) {
  test(`ADS-034 carousel fits ${vp.w}x${vp.h} and leaves the controls free`, async ({ browser }) => {
    const ctx = await browser.newContext({ viewport: { width: vp.w, height: vp.h }, hasTouch: true, isMobile: true });
    const page = await ctx.newPage();
    await start(page);
    await page.locator('#compass-btn').tap();
    const body = page.locator('#zb-carousel .zb-body');
    await expect(body).toBeVisible();
    await page.waitForTimeout(500);
    const r = (await body.boundingBox())!;
    expect(r.x).toBeGreaterThanOrEqual(0);
    expect(r.y).toBeGreaterThanOrEqual(0);
    expect(r.x + r.width).toBeLessThanOrEqual(vp.w + 0.5);
    expect(r.y + r.height).toBeLessThanOrEqual(vp.h + 0.5);
    for (const sel of ['#zb-car-prev', '#zb-car-next']) {
      const b = (await page.locator(sel).boundingBox())!;
      expect(b.width).toBeGreaterThanOrEqual(64);
      expect(b.height).toBeGreaterThanOrEqual(64);
    }
    for (const sel of ['#settings-btn', '#compass-btn', '#view-btn', '#act']) {
      const o = await page.locator(sel).boundingBox();
      if (!o || !(await page.locator(sel).isVisible())) continue;
      const hit = r.x < o.x + o.width && o.x < r.x + r.width && r.y < o.y + o.height && o.y < r.y + r.height;
      expect(hit, `carousel overlaps ${sel}`).toBe(false);
    }
    await page.screenshot({ path: `test-results/shots/carousel-${vp.w}x${vp.h}.png` });
    await ctx.close();
  });
}
