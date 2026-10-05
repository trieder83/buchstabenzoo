// ADS-026..029: the readable ad boards on a PHONE (touch, small screens) — user report 2026-10-03:
// "on mobile phone the advertisement billboards do not show a dialog or link". Runs on the release
// build (real signed campaigns, `scripts/e2e.sh web/tests/e2e/ads_phone.spec.ts`).
import { expect, test, devices, type Browser, type Page } from '@playwright/test';
import { face, goto, nextFrames, waitFrames, START_URL } from './helpers';

interface Board {
  id: string;
  slot: number;
  x: number;
  z: number;
}

const VIEWPORTS = [
  { name: 'landscape 780x360', width: 780, height: 360 },
  { name: 'portrait 360x780', width: 360, height: 780 },
  { name: 'portrait 412x892', width: 412, height: 892 },
];

type Win = { __opens: unknown[][] };

async function phone(browser: Browser, width: number, height: number, route?: (page: Page) => Promise<void>) {
  const ctx = await browser.newContext({
    viewport: { width, height },
    hasTouch: true,
    isMobile: true,
    deviceScaleFactor: 2,
    userAgent: devices['Pixel 7'].userAgent,
  });
  const page = await ctx.newPage();
  const errors: string[] = [];
  page.on('pageerror', (e) => errors.push(String(e)));
  await page.addInitScript(() => {
    if (sessionStorage.getItem('zoo.e2e.init')) return;
    sessionStorage.setItem('zoo.e2e.init', '1');
    localStorage.clear();
    (window as unknown as Win).__opens = [];
    window.open = ((...a: unknown[]) => {
      (window as unknown as Win).__opens.push(a);
      return null;
    }) as typeof window.open;
  });
  if (route) await route(page);
  await page.goto(START_URL);
  await waitFrames(page, 3);
  return { ctx, page, errors };
}

const verified = async (page: Page) => {
  await page.waitForFunction(() => window.__zoo?.ads?.state().settled, undefined, { timeout: 60_000 });
  expect((await page.evaluate(() => window.__zoo!.ads.state())).loaded, 'manifest verified on the phone').toBe(true);
};

const boards = async (page: Page) => JSON.parse(await page.evaluate(() => window.__zoo!.app.ad_boards_json())) as Board[];
const opens = (page: Page) => page.evaluate(() => (window as unknown as Win).__opens);
const near = (page: Page) => page.evaluate(() => window.__zoo!.app.ad_near());

/** Stands `d` m south (in front) of the board, `dx` m aside. */
async function standAt(page: Page, b: Board, d: number, dx = 0): Promise<void> {
  await goto(page, b.x + dx, b.z - d);
  await page.evaluate(() => window.__zoo!.app.debug_step(0.3));
  await nextFrames(page, 3);
  expect(await near(page), `near ${b.id} at ${d} m / ${dx} m aside`).toBe(b.id);
}

/** Walks to a spot where no board is near (any of a few offsets). */
async function walkAway(page: Page, b: Board): Promise<void> {
  for (const [dx, dz] of [[0, -9], [0, 9], [9, 0], [-9, 0], [0, -12], [12, 0]]) {
    await goto(page, b.x + dx, b.z + dz).catch(() => undefined);
    if ((await near(page)) === '') return;
  }
  throw new Error('no free spot away from the boards');
}

const rect = async (page: Page, sel: string) => {
  const r = await page.locator(sel).boundingBox();
  return r;
};

function overlaps(a: { x: number; y: number; width: number; height: number }, b: { x: number; y: number; width: number; height: number }): boolean {
  return a.x < b.x + b.width && b.x < a.x + a.width && a.y < b.y + b.height && b.y < a.y + a.height;
}

for (const vp of VIEWPORTS) {
  test(`ADS-026 phone ${vp.name}: ad panel + link button visible and tappable, any facing`, async ({ browser }) => {
    const { ctx, page, errors } = await phone(browser, vp.width, vp.height);
    await verified(page);
    const b = (await boards(page)).find((x) => x.slot === 1)!;
    // far inside the new range (3.3 m), looking away in every direction: still opens (rule 10)
    for (const key of ['KeyW', 'KeyA', 'KeyS', 'KeyD']) {
      await standAt(page, b, 3.3);
      await face(page, key);
      await standAt(page, b, 3.3);
      await expect(page.locator('#ad-panel')).toBeVisible({ timeout: 15_000 });
      await walkAway(page, b);
      await expect(page.locator('#ad-panel')).toBeHidden();
    }
    await standAt(page, b, 1.6, 2.4); // beside the picture, still on the readable side
    await expect(page.locator('#ad-panel')).toBeVisible({ timeout: 15_000 });
    const link = page.locator('#ad-link');
    await expect(link).toBeVisible();
    const box = (await link.boundingBox())!;
    expect(box.x).toBeGreaterThanOrEqual(0);
    expect(box.y).toBeGreaterThanOrEqual(0);
    expect(box.x + box.width).toBeLessThanOrEqual(vp.width + 0.5);
    expect(box.y + box.height).toBeLessThanOrEqual(vp.height + 0.5);
    expect(box.height).toBeGreaterThanOrEqual(56);
    await page.waitForTimeout(500);
    await page.screenshot({ path: `../qa/reports/img/2026-10-03-ad-phone-${vp.width}x${vp.height}.png` });
    await link.tap();
    await expect(page.locator('#ad-gate')).toBeVisible();
    expect((await opens(page)).length).toBe(0);
    await page.screenshot({ path: `../qa/reports/img/2026-10-03-ad-phone-gate-${vp.width}x${vp.height}.png` });
    expect(errors).toEqual([]);
    await ctx.close();
  });
}

test('ADS-027 phone: accidental ✖ ignored, interact button reopens, walking away and back reopens', async ({ browser }) => {
  const { ctx, page } = await phone(browser, 780, 360);
  await verified(page);
  const b = (await boards(page)).find((x) => x.slot === 1)!;
  await walkAway(page, b);
  await page.locator('#game').tap({ position: { x: 300, y: 200 } }); // first touch: touch controls on
  await expect(page.locator('#ad-act')).toBeHidden();
  // walk in and tap ✖ straight after the panel opened (the first-contact touch): ignored
  const stillOpen = await page.evaluate(
    ([x, z]) =>
      new Promise<boolean>((resolve) => {
        const app = window.__zoo!.app;
        app.debug_goto(x, z);
        app.debug_step(90);
        app.debug_step(0.3);
        const poll = () => {
          const btn = document.querySelector<HTMLButtonElement>('#ad-panel .ad-x');
          if (!btn) return requestAnimationFrame(poll);
          btn.click();
          requestAnimationFrame(() => resolve(!document.getElementById('ad-panel')!.hidden));
        };
        poll();
      }),
    [b.x, b.z - 2.0],
  );
  expect(stillOpen, 'early ✖ ignored').toBe(true);
  await page.waitForTimeout(600);
  await page.locator('#ad-panel .ad-x').tap();
  await expect(page.locator('#ad-panel')).toBeHidden();
  // the interact button (🔗) appears and reopens the dialog
  const act = page.locator('#ad-act');
  await expect(act).toBeVisible();
  const ab = (await act.boundingBox())!;
  expect(ab.width).toBeGreaterThanOrEqual(64);
  await page.screenshot({ path: '../qa/reports/img/2026-10-03-ad-phone-act-780x360.png' });
  await act.tap();
  await expect(page.locator('#ad-panel')).toBeVisible();
  await expect(act).toBeHidden();
  // close again, walk away and come back: opens by itself
  await page.waitForTimeout(600);
  await page.locator('#ad-panel .ad-x').tap();
  await expect(page.locator('#ad-panel')).toBeHidden();
  await walkAway(page, b);
  await expect(act).toBeHidden();
  await standAt(page, b, 2.0);
  await expect(page.locator('#ad-panel')).toBeVisible();
  await ctx.close();
});

test('ADS-027 phone: the manifest arrives late — the panel opens once it is verified', async ({ browser }) => {
  const { ctx, page } = await phone(browser, 780, 360, async (p) => {
    await p.route('**/ads/campaigns.*', async (route) => {
      await new Promise((r) => setTimeout(r, 3500));
      await route.continue();
    });
  });
  const b = (await boards(page)).find((x) => x.slot === 1)!;
  await standAt(page, b, 2.0);
  await expect(page.locator('#ad-panel')).toBeHidden();
  await verified(page);
  await expect(page.locator('#ad-panel')).toBeVisible({ timeout: 5000 });
  await ctx.close();
});

for (const vp of [
  { width: 780, height: 360 },
  { width: 360, height: 780 },
  { width: 412, height: 892 },
]) {
  test(`ADS-028 phone ${vp.width}x${vp.height}: the panel does not cover the control buttons`, async ({ browser }) => {
    const { ctx, page } = await phone(browser, vp.width, vp.height);
    await verified(page);
    const b = (await boards(page)).find((x) => x.slot === 1)!;
    await page.locator('#game').tap({ position: { x: 100, y: 100 } });
    await standAt(page, b, 2.0);
    await expect(page.locator('#ad-panel')).toBeVisible();
    await page.waitForTimeout(300);
    const body = (await rect(page, '.ad-body'))!;
    expect(body.x).toBeGreaterThanOrEqual(0);
    expect(body.x + body.width).toBeLessThanOrEqual(vp.width + 0.5);
    expect(body.y + body.height).toBeLessThanOrEqual(vp.height + 0.5);
    for (const sel of ['#settings-btn', '#compass-btn', '#view-btn', '#act']) {
      const r = await rect(page, sel);
      if (!r) continue; // hidden (no target)
      expect(overlaps(body, r), `${sel} ${JSON.stringify(r)} vs card ${JSON.stringify(body)}`).toBe(false);
    }
    if (vp.height <= 460) expect(body.height, 'compact card').toBeLessThanOrEqual(190);
    await ctx.close();
  });
}

/** Real touch hold through CDP (Playwright's touchscreen can only tap). */
async function touchHold(page: Page, cdp: import('@playwright/test').CDPSession, ms: number, wiggle: boolean): Promise<void> {
  const hb = (await page.locator('#ad-hold').boundingBox())!;
  const x = hb.x + hb.width / 2;
  const y = hb.y + hb.height / 2;
  await cdp.send('Input.dispatchTouchEvent', { type: 'touchStart', touchPoints: [{ x, y }] });
  await page.waitForTimeout(ms / 2);
  if (wiggle) {
    await cdp.send('Input.dispatchTouchEvent', { type: 'touchMove', touchPoints: [{ x: x + 10, y: y + 6 }] });
    await page.waitForTimeout(ms / 2);
    await cdp.send('Input.dispatchTouchEvent', { type: 'touchMove', touchPoints: [{ x: x - 8, y: y - 4 }] });
  } else {
    await page.waitForTimeout(ms / 2);
  }
}

test('ADS-029 ADC1-004 phone: gate by touch, full hold, release opens the link directly once (ADS-030)', async ({ browser }) => {
  const { ctx, page, errors } = await phone(browser, 780, 360);
  await verified(page);
  const b = (await boards(page)).find((x) => x.slot === 1)!;
  await standAt(page, b, 2.0);
  await page.waitForTimeout(600);
  await page.locator('#ad-link').tap();
  const q = (await page.locator('#ad-gate-question').textContent())!;
  const [, a, op, c] = /(\d+) ([+−]) (\d+)/.exec(q)!;
  const answer = op === '+' ? Number(a) + Number(c) : Number(a) - Number(c);
  await page.locator('.ad-choice', { hasText: new RegExp(`^${answer}$`) }).tap();
  await expect(page.locator('#ad-hold')).toBeVisible();
  // no context menu / selection on the hold button
  expect(await page.evaluate(() => getComputedStyle(document.getElementById('ad-hold')!).touchAction)).toBe('none');
  // a short touch resets
  const cdp = await ctx.newCDPSession(page);
  await touchHold(page, cdp, 800, false);
  await cdp.send('Input.dispatchTouchEvent', { type: 'touchEnd', touchPoints: [] });
  await page.waitForTimeout(200);
  expect(await page.locator('#ad-hold.ready').count()).toBe(0);
  // 2 s with a moving finger: the ✔ shows, nothing has been opened yet
  await touchHold(page, cdp, 2400, true);
  await expect(page.locator('#ad-hold.ready')).toBeVisible({ timeout: 4000 });
  expect(await opens(page), 'no window.open from the hold timer').toEqual([]);
  await page.screenshot({ path: '../qa/reports/img/2026-10-03-ad-phone-open-780x360.png' });
  // releasing the finger opens the link directly, once (ADS-030)
  await cdp.send('Input.dispatchTouchEvent', { type: 'touchEnd', touchPoints: [] });
  await page.waitForTimeout(400);
  expect(await opens(page)).toEqual([['https://mathfighter.rcms.ch/', '_blank', 'noopener,noreferrer']]);
  await expect(page.locator('#ad-gate')).toBeHidden();
  await expect(page.locator('#ad-panel')).toBeHidden();
  expect(errors).toEqual([]);
  await ctx.close();
});

test('ADS-029 phone: images that need 6 s (slow mobile data) still show the campaign', async ({ browser }) => {
  const { ctx, page } = await phone(browser, 412, 892, async (p) => {
    await p.route('**/ads/img/**', async (route) => {
      await new Promise((r) => setTimeout(r, 6000));
      await route.continue();
    });
  });
  await verified(page);
  expect((await page.evaluate(() => window.__zoo!.ads.state().campaigns))[1]).toBe('mathfighter');
  await ctx.close();
});

test('ADS-029 phone: a failed first load is retried once', async ({ browser }) => {
  test.setTimeout(120_000);
  let n = 0;
  const { ctx, page } = await phone(browser, 412, 892, async (p) => {
    await p.route('**/ads/campaigns.json', async (route) => {
      n += 1;
      if (n === 1) return route.abort('connectionreset');
      return route.continue();
    });
  });
  await page.waitForFunction(() => window.__zoo?.ads?.state().settled, undefined, { timeout: 30_000 });
  expect((await page.evaluate(() => window.__zoo!.ads.state())).loaded).toBe(false);
  await page.waitForFunction(() => window.__zoo!.ads.state().loaded, undefined, { timeout: 40_000 });
  expect(n).toBe(2);
  await ctx.close();
});
