// ADS-038..042: the ad billboards under the strongest Android emulation Playwright offers (user report
// 2026-10-07: "ad billboards do not work on Android, on the web they do"). Real Android Chrome user agents,
// touch + isMobile + DPR 2.5-3, CPU throttling, a slow-4G model for the ad files, real CDP touch holds
// (also a cancelled touch), blocked pop-ups, the Back button, offline -> online, tab freeze/resume, rotation,
// a cross-origin iframe (itch.io), and the field diagnostics (`?adsdebug=1`, `window.__zoo.adsDebug()`).
//
//   scripts/e2e.sh web/tests/e2e/ads_android.spec.ts                      release bundle (production key, real campaigns)
//   E2E_DIST=dist-adtest scripts/e2e.sh web/tests/e2e/ads_android.spec.ts  test build (fixture campaigns, test key)
//   ADS_LIVE_URL=https://letterzoo.web.app/ scripts/e2e.sh web/tests/e2e/ads_android.spec.ts   the live site
//
// Every profile writes a step table to web/test-results/ads-android-<profile>.json.
import fs from 'node:fs';
import path from 'node:path';
import { expect, test, devices, type Browser, type BrowserContext, type CDPSession, type Page } from '@playwright/test';
import { nextFrames, repo, waitFrames } from './helpers';

const LIVE = process.env.ADS_LIVE_URL ?? '';
const TEST_BUILD = process.env.E2E_DIST === 'dist-adtest' && !LIVE;
const fixtures = path.join(repo, 'web/tests/fixtures/ads');
const PUB = fs.existsSync(path.join(fixtures, 'TEST-ONLY-public.key')) ? fs.readFileSync(path.join(fixtures, 'TEST-ONLY-public.key'), 'utf8').trim() : '';
// Android user agents open the Play Store page of Math Fighter (ADS-049); the live site still serves the older manifest without store links
const MF_ANDROID_LINK = LIVE ? 'https://mathfighter.rcms.ch/' : 'https://play.google.com/store/apps/details?id=com.mathfighter.app';
const CAMPAIGN_BY_SLOT: Record<number, string> = { 1: 'mathfighter', 2: 'abcsmash', 3: 'edugamegalaxy' };

interface Board {
  id: string;
  slot: number;
  x: number;
  z: number;
}
type Win = { __opens: unknown[][]; __openMode: 'null' | 'window' };

interface Profile {
  name: string;
  ua: string;
  width: number;
  height: number;
  dpr: number;
  cpu: number;
  /** Model of the ad files over a slow mobile network: round trip (ms) and speed (kbit/s). */
  net: { rtt: number; kbps: number } | null;
  allBoards: boolean;
}

const PROFILES: Profile[] = [
  { name: 'pixel7', ua: devices['Pixel 7'].userAgent, width: 412, height: 915, dpr: 2.625, cpu: 1, net: null, allBoards: true },
  { name: 'galaxy-s9plus-slow4g', ua: devices['Galaxy S9+'].userAgent, width: 320, height: 658, dpr: 4.5, cpu: 4, net: { rtt: 560, kbps: 1500 }, allBoards: false },
  {
    name: 'lowend-360x640-3g',
    ua: 'Mozilla/5.0 (Linux; Android 8.1.0; SM-J410F) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Mobile Safari/537.36',
    width: 640,
    height: 360,
    dpr: 3,
    cpu: 6,
    net: { rtt: 1200, kbps: 400 },
    allBoards: false,
  },
];

const url = (query = '') => {
  const q = `?seed=17${TEST_BUILD ? `&adkey=${encodeURIComponent(PUB)}` : ''}${query}`;
  return LIVE ? new URL(q, LIVE).toString() : `/${q}`;
};

function fixtureFiles(): Map<string, Buffer> {
  const adsDir = path.join(fixtures, 'boards');
  const files = new Map<string, Buffer>();
  files.set('index.json', fs.readFileSync(path.join(adsDir, 'index.json')));
  files.set('index.sig', fs.readFileSync(path.join(adsDir, 'index.sig')));
  for (const f of fs.readdirSync(path.join(adsDir, 'img'))) files.set(`img/${f}`, fs.readFileSync(path.join(adsDir, 'img', f)));
  return files;
}

/** Serves the ad files: the test fixtures (test build) or the real ones, delayed like a slow mobile network. */
async function routeAds(page: Page, net: Profile['net'], timeline: string[], offline: () => boolean = () => false): Promise<void> {
  const files = TEST_BUILD ? fixtureFiles() : null;
  let busyUntil = 0;
  await page.route('**/boards/**', async (route) => {
    if (offline()) return route.abort('internetdisconnected');
    const rel = new URL(route.request().url()).pathname.replace(/^.*\/boards\//, '');
    let status = 200;
    let body: Buffer;
    let contentType: string;
    if (files) {
      const f = files.get(rel);
      if (!f) return route.fulfill({ status: 404 });
      body = f;
      contentType = rel.endsWith('.png') ? 'image/png' : rel.endsWith('.webp') ? 'image/webp' : 'text/plain';
    } else {
      const res = await route.fetch();
      status = res.status();
      body = await res.body();
      contentType = res.headers()['content-type'] ?? 'application/octet-stream';
    }
    if (net) {
      // one shared line: transfers queue behind each other like on a single slow link
      const now = Date.now();
      const start = Math.max(now, busyUntil);
      const done = start + (body.length * 8) / net.kbps; // ms (kbit/s = bit/ms)
      busyUntil = done;
      await new Promise((r) => setTimeout(r, done - now + net.rtt));
    }
    timeline.push(`${rel} ${body.length}B`);
    return route.fulfill({ status, body, contentType });
  });
}

interface Phone {
  ctx: BrowserContext;
  page: Page;
  cdp: CDPSession;
  errors: string[];
}

async function phone(browser: Browser, p: Profile, opts: { query?: string; init?: boolean; timeline?: string[]; skipAds?: boolean } = {}): Promise<Phone> {
  const ctx = await browser.newContext({
    viewport: { width: p.width, height: p.height },
    hasTouch: true,
    isMobile: true,
    deviceScaleFactor: p.dpr,
    userAgent: p.ua,
    locale: 'de-CH',
  });
  const page = await ctx.newPage();
  const errors: string[] = [];
  page.on('pageerror', (e) => errors.push(String(e)));
  const cdp = await ctx.newCDPSession(page);
  if (p.cpu > 1) await cdp.send('Emulation.setCPUThrottlingRate', { rate: p.cpu });
  await page.addInitScript(() => {
    if (sessionStorage.getItem('zoo.e2e.init')) return;
    sessionStorage.setItem('zoo.e2e.init', '1');
    localStorage.clear();
    const w = window as unknown as Win;
    w.__opens = [];
    w.__openMode = 'null';
    window.open = ((...a: unknown[]) => {
      w.__opens.push(a);
      return w.__openMode === 'window' ? ({} as Window) : null;
    }) as typeof window.open;
  });
  if (!opts.skipAds) await routeAds(page, p.net, opts.timeline ?? []);
  await page.goto(url(opts.query ?? ''), { timeout: 90_000 });
  await waitFrames(page, 3);
  return { ctx, page, cdp, errors };
}

// (a build from before ADS-038 has no adsDebug(): empty data then, so the live baseline still runs)
const dbg = (page: Page) =>
  page.evaluate(
    () =>
      ({ manifest: {}, signature: {}, images: {}, uploads: null, pointer: {}, windowOpen: {}, near: {}, env: { gl: {}, ua: '' }, errors: [], ...(window.__zoo!.adsDebug?.() ?? {}) }) as Record<string, any>, // eslint-disable-line @typescript-eslint/no-explicit-any
  );
const boards = async (page: Page) => JSON.parse(await page.evaluate(() => window.__zoo!.app.ad_boards_json())) as Board[];
const opens = (page: Page) => page.evaluate(() => (window as unknown as Win).__opens);
const near = (page: Page) => page.evaluate(() => window.__zoo!.app.ad_near());

async function verified(page: Page, timeout = 90_000): Promise<void> {
  await page.waitForFunction(() => window.__zoo?.ads?.state().settled, undefined, { timeout });
  expect((await page.evaluate(() => window.__zoo!.ads.state())).loaded, 'manifest verified').toBe(true);
}

async function standAt(page: Page, b: Board): Promise<void> {
  // teleport (the later levels are behind locked gates); the real walk is covered by ads_phone.spec.ts
  for (const d of [1.6, 2.4, 1.0, 2.9]) {
    await page.evaluate(([x, z]) => window.__zoo!.app.debug_teleport(x, z), [b.x, b.z - d]);
    await page.evaluate(() => window.__zoo!.app.debug_step(0.3));
    await nextFrames(page, 3);
    if ((await near(page)) === b.id) return;
  }
  throw new Error(`cannot stand in front of ${b.id}`);
}

async function walkAway(page: Page, b: Board): Promise<void> {
  for (const [dx, dz] of [[0, -9], [0, 9], [9, 0], [-9, 0], [0, -12], [12, 0]]) {
    await page.evaluate(([x, z]) => window.__zoo!.app.debug_teleport(x, z), [b.x + dx, b.z + dz]);
    await page.evaluate(() => window.__zoo!.app.debug_step(0.3));
    await nextFrames(page, 2);
    if ((await near(page)) === '') return;
  }
  throw new Error('no free spot away from the boards');
}

function gateAnswer(q: string): number | string {
  const m = /(\d+) ([+−]) (\d+)/.exec(q);
  if (m) return m[2] === '+' ? Number(m[1]) + Number(m[3]) : Number(m[1]) - Number(m[3]);
  return q; // language gate: handled by the caller
}

/** Passes the gate question of whichever campaign is open (sum, article or plural). */
async function answerGate(page: Page): Promise<void> {
  const q = (await page.locator('#zb-gate-question').textContent())!;
  const a = gateAnswer(q);
  if (typeof a === 'number') {
    await page.locator('.zb-choice', { hasText: new RegExp(`^${a}$`) }).tap();
    return;
  }
  // abcsmash gate: the right article / plural — read the answer from the internal gate
  const right = await page.evaluate(() => (window.__zoo!.ads as unknown as { gate: { question: { labels?: string[]; answer: number } } | null }).gate?.question);
  await page.locator('.zb-choice', { hasText: new RegExp(`^${right!.labels![right!.answer]}$`) }).tap();
}

async function touchHold(page: Page, cdp: CDPSession, ms: number, wiggle: boolean): Promise<void> {
  const hb = (await page.locator('#zb-hold').boundingBox())!;
  const x = hb.x + hb.width / 2;
  const y = hb.y + hb.height / 2;
  await cdp.send('Input.dispatchTouchEvent', { type: 'touchStart', touchPoints: [{ x, y }] });
  await page.waitForTimeout(ms / 2);
  if (wiggle) await cdp.send('Input.dispatchTouchEvent', { type: 'touchMove', touchPoints: [{ x: x + 9, y: y + 5 }] });
  await page.waitForTimeout(ms / 2);
}

interface Step {
  step: string;
  ok: boolean;
  ms: number;
  note?: string;
}

async function run(steps: Step[], name: string, fn: () => Promise<string | void>): Promise<void> {
  const t = Date.now();
  try {
    const note = await fn();
    steps.push({ step: name, ok: true, ms: Date.now() - t, note: note || undefined });
  } catch (e) {
    steps.push({ step: name, ok: false, ms: Date.now() - t, note: String((e as Error).message).split('\n')[0].slice(0, 300) });
  }
}

for (const p of PROFILES) {
  test(`ADS-042 android emulation ${p.name}: board near -> panel -> gate -> hold -> release opens the link`, async ({ browser }) => {
    test.setTimeout(900_000);
    const steps: Step[] = [];
    const timeline: string[] = [];
    const { ctx, page, cdp, errors } = await phone(browser, p, { timeline });
    await page.locator('#game').tap({ position: { x: 100, y: 100 } }); // first touch: touch controls
    let list: Board[] = [];
    await run(steps, 'manifest verified (signature, hashes)', async () => {
      await verified(page);
      const d = await dbg(page);
      return `manifest ${d.manifest.ms} ms, verify ${d.signature.verifier} ${d.signature.ms} ms, images ${JSON.stringify(Object.fromEntries(Object.entries(d.images as Record<string, { ms: number }>).map(([k, v]) => [k.slice(4), v.ms])))}`;
    });
    await run(steps, 'all 3 slots loaded', async () => {
      const st = await page.evaluate(() => window.__zoo!.ads.state());
      expect(st.campaigns).toEqual(CAMPAIGN_BY_SLOT);
    });
    list = await boards(page);
    const pick = p.allBoards ? list : [1, 2, 3].map((s) => list.find((b) => b.slot === s)!);
    for (const b of pick) {
      await run(steps, `board ${b.id} (slot ${b.slot}): near -> panel with the campaign picture + texture uploaded`, async () => {
        await standAt(page, b);
        await expect(page.locator('#zb-panel')).toBeVisible({ timeout: 20_000 });
        await expect(page.locator('.zb-body')).toHaveAttribute('data-card', CAMPAIGN_BY_SLOT[b.slot]);
        await expect.poll(() => page.evaluate(() => (document.querySelector('#zb-image') as HTMLImageElement | null)?.naturalWidth ?? 0), { timeout: 10_000 }).toBeGreaterThan(64);
        const d = await dbg(page);
        if (d.uploads) expect(d.uploads[b.id], 'texture uploaded').toBe(true);
        const box = (await page.locator('#zb-link').boundingBox())!;
        expect(box.x + box.width).toBeLessThanOrEqual(p.width + 0.5);
        expect(box.y + box.height).toBeLessThanOrEqual(p.height + 0.5);
      });
    }
    const b1 = list.find((b) => b.slot === 1)!;
    await run(steps, 'close, 🔗 button reopens (touch)', async () => {
      await standAt(page, b1);
      await expect(page.locator('#zb-panel')).toBeVisible();
      await page.waitForTimeout(600);
      await page.locator('#zb-panel .zb-x').tap();
      await expect(page.locator('#zb-panel')).toBeHidden();
      await expect(page.locator('#zb-act')).toBeVisible();
      await page.locator('#zb-act').tap();
      await expect(page.locator('#zb-panel')).toBeVisible();
    });
    await run(steps, 'gate: sum, 2 s CDP touch hold (moving finger), ✔, release opens the link once', async () => {
      await page.locator('#zb-link').tap();
      await expect(page.locator('#zb-gate')).toBeVisible();
      await answerGate(page);
      await expect(page.locator('#zb-hold')).toBeVisible();
      await touchHold(page, cdp, 800, false);
      await cdp.send('Input.dispatchTouchEvent', { type: 'touchEnd', touchPoints: [] });
      await page.waitForTimeout(200);
      expect(await page.locator('#zb-hold.ready').count(), 'early release resets').toBe(0);
      await touchHold(page, cdp, 2600 * Math.min(2, p.cpu), true);
      await expect(page.locator('#zb-hold.ready')).toBeVisible({ timeout: 6000 });
      expect(await opens(page)).toEqual([]);
      await cdp.send('Input.dispatchTouchEvent', { type: 'touchEnd', touchPoints: [] });
      await expect.poll(() => opens(page), { timeout: 3000 }).toEqual([[MF_ANDROID_LINK, '_blank', 'noopener,noreferrer']]);
      const d = await dbg(page);
      return `pointer ${JSON.stringify(d.pointer)} open ${JSON.stringify(d.windowOpen)}`;
    });
    await run(steps, 'touch cancelled after the full hold (Android gesture): no blocked open, a tap-able link appears', async () => {
      await page.evaluate(() => ((window as unknown as Win).__opens.length = 0));
      await walkAway(page, b1);
      await standAt(page, b1);
      await expect(page.locator('#zb-panel')).toBeVisible();
      await page.waitForTimeout(600);
      await page.locator('#zb-link').tap();
      await answerGate(page);
      await touchHold(page, cdp, 2600 * Math.min(2, p.cpu), false);
      await expect(page.locator('#zb-hold.ready')).toBeVisible({ timeout: 6000 });
      await cdp.send('Input.dispatchTouchEvent', { type: 'touchCancel', touchPoints: [] });
      await expect(page.locator('#zb-gate a#zb-open')).toBeVisible({ timeout: 3000 });
      expect(await opens(page)).toEqual([]);
      const href = await page.locator('#zb-gate a#zb-open').getAttribute('href');
      expect(href).toBe(MF_ANDROID_LINK);
      expect(await page.locator('#zb-gate a#zb-open').getAttribute('rel')).toBe('noopener noreferrer');
      expect(await page.locator('#zb-gate a#zb-open').getAttribute('target')).toBe('_blank');
      const d = await dbg(page);
      return `pointer ${JSON.stringify(d.pointer)}`;
    });
    await run(steps, 'blocked pop-up (window.open without effect): fallback link shows after 1.5 s', async () => {
      await page.locator('#zb-gate .zb-x').tap();
      await expect(page.locator('#zb-gate')).toBeHidden();
      await page.evaluate(() => ((window as unknown as Win).__opens.length = 0));
      await page.locator('#zb-link').tap();
      await answerGate(page);
      await touchHold(page, cdp, 2600 * Math.min(2, p.cpu), false);
      await expect(page.locator('#zb-hold.ready')).toBeVisible({ timeout: 6000 });
      await cdp.send('Input.dispatchTouchEvent', { type: 'touchEnd', touchPoints: [] });
      await expect(page.locator('#zb-fallback a#zb-open')).toBeVisible({ timeout: 4000 });
      expect((await opens(page)).length).toBe(1);
      await page.locator('#zb-fallback .zb-x').tap();
      await expect(page.locator('#zb-fallback')).toBeHidden();
    });
    await run(steps, 'Back button with the gate open closes the gate, the game stays', async () => {
      await walkAway(page, b1);
      await standAt(page, b1);
      await page.waitForTimeout(600);
      await expect(page.locator('#zb-panel')).toBeVisible();
      await page.locator('#zb-link').tap();
      await expect(page.locator('#zb-gate')).toBeVisible();
      const before = page.url();
      await page.goBack();
      await expect(page.locator('#zb-gate')).toBeHidden({ timeout: 3000 });
      expect(page.url()).toBe(before);
      expect(await page.evaluate(() => !!window.__zoo)).toBe(true);
    });
    await run(steps, 'rotation while the panel is open keeps it inside the viewport', async () => {
      await walkAway(page, b1);
      await standAt(page, b1);
      await expect(page.locator('#zb-panel')).toBeVisible();
      for (const [w, h] of [[p.height, p.width], [p.width, p.height]]) {
        await page.setViewportSize({ width: w, height: h });
        await page.waitForTimeout(400);
        const r = (await page.locator('.zb-body').boundingBox())!;
        expect(r.x).toBeGreaterThanOrEqual(0);
        expect(r.x + r.width).toBeLessThanOrEqual(w + 0.5);
        expect(r.y + r.height).toBeLessThanOrEqual(h + 0.5);
      }
    });
    await run(steps, 'tab frozen and resumed (Android background): ads still work', async () => {
      await cdp.send('Page.setWebLifecycleState', { state: 'frozen' });
      await page.waitForTimeout(300);
      await cdp.send('Page.setWebLifecycleState', { state: 'active' });
      await walkAway(page, b1);
      await standAt(page, b1);
      await expect(page.locator('#zb-panel')).toBeVisible({ timeout: 10_000 });
    });
    await run(steps, 'diagnostics JSON is complete', async () => {
      const d = await dbg(page);
      for (const k of ['env', 'manifest', 'signature', 'images', 'near', 'pointer', 'windowOpen', 'errors']) expect(d, k).toHaveProperty(k);
      expect(d.env.gl.maxTextureSize).toBeGreaterThan(0);
      return `${d.env.ua.slice(0, 60)} secure=${d.env.secureContext} subtle=${d.env.subtle} maxTex=${d.env.gl.maxTextureSize} errors=${JSON.stringify(d.errors)}`;
    });
    steps.push({ step: 'page errors', ok: errors.length === 0, ms: 0, note: errors.join(' | ') || undefined });
    fs.mkdirSync(path.join(repo, 'web/test-results'), { recursive: true });
    fs.writeFileSync(path.join(repo, `web/test-results/ads-android-${p.name}${LIVE ? '-live' : TEST_BUILD ? '-adtest' : '-release'}.json`), JSON.stringify({ profile: p.name, steps, timeline }, null, 1));
    await ctx.close();
    const failed = steps.filter((s) => !s.ok);
    expect(failed, JSON.stringify(failed, null, 1)).toEqual([]);
  });
}

test('ADS-039 android: first load fails (offline), the ads come when the phone is online again', async ({ browser }) => {
  test.setTimeout(240_000);
  const p = PROFILES[0];
  let blocked = true;
  const ctx = await browser.newContext({ viewport: { width: p.width, height: p.height }, hasTouch: true, isMobile: true, deviceScaleFactor: p.dpr, userAgent: p.ua });
  const page = await ctx.newPage();
  await routeAds(page, null, [], () => blocked);
  await page.goto(url());
  await waitFrames(page, 3);
  await page.waitForFunction(() => window.__zoo!.ads.state().settled, undefined, { timeout: 30_000 });
  expect((await page.evaluate(() => window.__zoo!.ads.state())).loaded).toBe(false);
  const d = await dbg(page);
  expect(d.manifest.state).toBe('failed');
  expect(JSON.stringify(d.errors)).toContain('network');
  blocked = false;
  await page.evaluate(() => window.dispatchEvent(new Event('online'))); // what Android does when the data connection returns
  await page.waitForFunction(() => window.__zoo!.ads.state().loaded, undefined, { timeout: 30_000 });
  await ctx.close();
});

test('ADS-039 android: images that do not all arrive in time are retried (incomplete load)', async ({ browser }) => {
  test.setTimeout(240_000);
  const p = PROFILES[0];
  let slow = true;
  const ctx = await browser.newContext({ viewport: { width: p.width, height: p.height }, hasTouch: true, isMobile: true, deviceScaleFactor: p.dpr, userAgent: p.ua });
  const page = await ctx.newPage();
  const files = TEST_BUILD ? fixtureFiles() : null;
  await page.route('**/boards/img/**', async (route) => {
    if (slow && /edugalaxy|edugamegalaxy/.test(route.request().url())) return new Promise(() => undefined); // never answers
    if (files) {
      const rel = new URL(route.request().url()).pathname.replace(/^.*\/boards\//, '');
      return route.fulfill({ body: files.get(rel)!, contentType: 'image/png' });
    }
    return route.continue();
  });
  if (files) await page.route('**/boards/index.*', (route) => {
    const rel = new URL(route.request().url()).pathname.replace(/^.*\/boards\//, '');
    return route.fulfill({ body: files.get(rel)!, contentType: 'text/plain' });
  });
  await page.goto(url());
  await waitFrames(page, 3);
  await page.waitForFunction(() => window.__zoo!.ads.state().settled, undefined, { timeout: 60_000 });
  const first = await page.evaluate(() => window.__zoo!.ads.state().campaigns);
  expect(Object.keys(first).length).toBeLessThan(3);
  slow = false;
  await page.evaluate(() => window.dispatchEvent(new Event('online')));
  await page.waitForFunction(() => Object.keys(window.__zoo!.ads.state().campaigns).length === 3, undefined, { timeout: 60_000 });
  await ctx.close();
});

test('ADS-038 android: ?adsdebug=1 shows the readable, copyable diagnostics overlay; adsDebug() returns the same data', async ({ browser }) => {
  const p = PROFILES[0];
  const { ctx, page } = await phone(browser, p, { query: '&adsdebug=1' });
  await verified(page);
  const ov = page.locator('#zb-dbg');
  await expect(ov).toBeVisible();
  const text = (await ov.locator('pre').textContent())!;
  for (const k of ['ua', 'secureContext', 'subtle', 'maxTextureSize', 'manifest', 'signature', 'verifier', 'images', 'near', 'pointer', 'windowOpen', 'errors']) expect(text, k).toContain(k);
  const fs0 = await ov.locator('pre').evaluate((e) => parseFloat(getComputedStyle(e).fontSize));
  expect(fs0).toBeGreaterThanOrEqual(14);
  await expect(page.locator('#zb-dbg-copy')).toBeVisible();
  expect(((await page.locator('#zb-dbg-copy').boundingBox())!.height)).toBeGreaterThanOrEqual(48);
  await page.screenshot({ path: path.join(repo, 'web/test-results/zb-dbg-overlay.png') });
  await page.locator('#zb-dbg-close').tap();
  await expect(ov).toBeHidden();
  await page.locator('#zb-dbg-chip').tap();
  await expect(ov).toBeVisible();
  const json = await dbg(page);
  expect(json.signature.verifier).toMatch(/subtle|js/);
  expect(json.signature.ok).toBe(true);
  await ctx.close();
});

test('ADS-038 android: without ?adsdebug=1 there is no overlay and no chip', async ({ browser }) => {
  const { ctx, page } = await phone(browser, PROFILES[0]);
  await expect(page.locator('#zb-dbg')).toHaveCount(0);
  await expect(page.locator('#zb-dbg-chip')).toHaveCount(0);
  await ctx.close();
});

test('ADS-042 android: the game inside a cross-origin iframe (itch.io style) loads the ads and opens the gate', async ({ browser, baseURL }) => {
  test.skip(!!LIVE, 'the live site forbids framing (frame-ancestors none)');
  test.setTimeout(240_000);
  const p = PROFILES[0];
  const ctx = await browser.newContext({ viewport: { width: p.width, height: p.height }, hasTouch: true, isMobile: true, deviceScaleFactor: p.dpr, userAgent: p.ua });
  const page = await ctx.newPage();
  // like html.itch.zone: another (public) origin, the game one folder deeper (/game/), everything proxied to the local server
  const origin = 'https://html.itch.test';
  const local = new URL(baseURL!).origin;
  await page.route(`${origin}/**`, async (route) => {
    const u = new URL(route.request().url());
    if (u.pathname === '/index.html') {
      return route.fulfill({
        contentType: 'text/html',
        body: `<!doctype html><html><body style="margin:0"><iframe id="g" src="/game/${url().replace(/^\//, '')}" style="border:0;width:100vw;height:100vh" allow="autoplay; fullscreen" sandbox="allow-scripts allow-same-origin allow-popups allow-popups-to-escape-sandbox allow-forms allow-pointer-lock allow-downloads"></iframe></body></html>`,
      });
    }
    const res = await route.fetch({ url: local + u.pathname.replace(/^\/game/, '') + u.search });
    return route.fulfill({ response: res });
  });
  if (TEST_BUILD) await routeAds(page, null, []); // registered last, so it wins for **/boards/** (fixture campaigns, test key)
  await page.goto(`${origin}/index.html`);
  const frame = page.frameLocator('#g');
  const fr = await (await page.waitForSelector('#g')).contentFrame();
  expect(fr).not.toBeNull();
  await fr!.waitForFunction(() => (window as Window).__zoo?.ads?.state().settled, undefined, { timeout: 120_000 });
  expect(await fr!.evaluate(() => window.__zoo!.ads.state().loaded)).toBe(true);
  await expect(frame.locator('#zb-panel')).toBeHidden();
  const d = await fr!.evaluate(() => window.__zoo!.adsDebug() as { env: { framed: boolean; origin: string } });
  expect(d.env.framed).toBe(true);
  await page.unrouteAll({ behavior: 'ignoreErrors' });
  await ctx.close();
});
