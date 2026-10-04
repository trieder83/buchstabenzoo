// TECH-PLATFORMS "Analytics (opt-in)": PLAT-022 (nothing before consent), PLAT-023 (parental gate),
// PLAT-024 (grant: script + gtag setup), PLAT-027 (switch off), PLAT-030 (events), PLAT-031 (button, small screens).
// Runs on the test build `dist-adtest` (fake measurement id G-TEST000000, PLAT-029); scripts/e2e.sh picks it.
import { expect, test, type Page } from '@playwright/test';
import { nextFrames, START_URL, waitFrames } from './helpers';

const GOOGLE = /googletagmanager\.com|google-analytics\.com|analytics\.google\.com|firebase|gstatic\.com/;
const ID = 'G-TEST000000';

type Call = unknown[];

/** Intercepts every Google request: the gtag script is a stub that marks itself, the rest is aborted. */
async function spy(page: Page): Promise<string[]> {
  const urls: string[] = [];
  await page.route(GOOGLE, async (route) => {
    const url = route.request().url();
    urls.push(url);
    if (url.includes('/gtag/js')) {
      await route.fulfill({ status: 200, contentType: 'application/javascript', body: 'window.__gtagLoaded = true;' });
    } else {
      await route.abort();
    }
  });
  return urls;
}

async function open(page: Page, w = 412, h = 892, keepStorage = false) {
  await page.setViewportSize({ width: w, height: h });
  if (!keepStorage) {
    await page.addInitScript(() => {
      if (sessionStorage.getItem('zoo.e2e.init')) return;
      sessionStorage.setItem('zoo.e2e.init', '1');
      localStorage.clear();
    });
  }
  await page.goto(START_URL);
  await waitFrames(page, 3);
}

const calls = (page: Page): Promise<Call[]> =>
  page.evaluate(() => ((window as unknown as { dataLayer?: ArrayLike<unknown>[] }).dataLayer ?? []).map((a) => Array.from(a)));
const events = async (page: Page) => (await calls(page)).filter((c) => c[0] === 'event');

async function openSettings(page: Page) {
  if (!(await page.locator('#settings').isVisible())) await page.click('#settings-btn');
  await expect(page.locator('#analytics-toggle')).toBeVisible();
}

/** The gate: solve the sum (or pick a wrong answer), then hold the hand 3 s. */
async function passGate(page: Page, wrong = false) {
  await openSettings(page);
  await page.click('#analytics-toggle');
  const q = (await page.locator('.an-question').innerText()).replace('−', '-');
  const m = /(\d+) ([+-]) (\d+)/.exec(q)!;
  const answer = m[2] === '+' ? Number(m[1]) + Number(m[3]) : Number(m[1]) - Number(m[3]);
  const labels = await page.locator('.an-choices .an-btn').allInnerTexts();
  const pick = wrong ? labels.find((l) => Number(l) !== answer)! : String(answer);
  await page.locator('.an-choices .an-btn', { hasText: new RegExp(`^${pick}$`) }).click();
  if (wrong) return;
  const hold = page.locator('#analytics-hold');
  const box = (await hold.boundingBox())!;
  await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
  await page.mouse.down();
  await expect(page.locator('#analytics-allow')).toBeVisible({ timeout: 6000 });
  await page.mouse.up();
}

test('PLAT-022 nothing is loaded and no request is made before consent; the button shows off', async ({ page }) => {
  const urls = await spy(page);
  await open(page);
  await nextFrames(page, 30);
  await page.waitForTimeout(1500);
  expect(urls).toEqual([]);
  expect(await page.evaluate(() => 'dataLayer' in window)).toBe(false);
  expect(await page.locator('script[src*="googletagmanager"]').count()).toBe(0);
  expect(await page.evaluate(() => localStorage.getItem('zoo.analytics'))).toBeNull();
  expect(await page.locator('#analytics-dialog').isVisible()).toBe(false); // no consent dialog at start
  await openSettings(page);
  await expect(page.locator('#analytics-toggle')).toHaveAttribute('aria-pressed', 'false');
  expect(urls).toEqual([]);
});

test('PLAT-023 a wrong gate answer, the close button and "Nein danke" grant nothing', async ({ page }) => {
  const urls = await spy(page);
  await open(page);
  await passGate(page, true);
  await expect(page.locator('#analytics-dialog')).toBeHidden();
  expect(await page.evaluate(() => localStorage.getItem('zoo.analytics'))).toBeNull();
  // ✖ in the sum step
  await page.click('#analytics-toggle');
  await page.click('.an-x');
  await expect(page.locator('#analytics-dialog')).toBeHidden();
  // pass the gate, then "Nein danke"
  await passGate(page);
  const box = (await page.locator('#analytics-deny').boundingBox())!;
  expect(box.height).toBeGreaterThanOrEqual(64);
  expect(box.width).toBeGreaterThanOrEqual(64);
  await page.click('#analytics-deny');
  await expect(page.locator('#analytics-dialog')).toBeHidden();
  expect(await page.evaluate(() => localStorage.getItem('zoo.analytics'))).toBe('denied');
  await page.waitForTimeout(500);
  expect(urls).toEqual([]);
  expect(await page.evaluate(() => 'dataLayer' in window)).toBe(false);
});

test('PLAT-023 a short hold does not pass the gate', async ({ page }) => {
  await spy(page);
  await open(page);
  await openSettings(page);
  await page.click('#analytics-toggle');
  const q = (await page.locator('.an-question').innerText()).replace('−', '-');
  const m = /(\d+) ([+-]) (\d+)/.exec(q)!;
  const answer = m[2] === '+' ? Number(m[1]) + Number(m[3]) : Number(m[1]) - Number(m[3]);
  await page.locator('.an-choices .an-btn', { hasText: new RegExp(`^${answer}$`) }).click();
  const box = (await page.locator('#analytics-hold').boundingBox())!;
  await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
  await page.mouse.down();
  await page.waitForTimeout(1200);
  await page.mouse.up();
  await page.waitForTimeout(2500);
  await expect(page.locator('#analytics-allow')).toHaveCount(0);
  expect(await page.evaluate(() => localStorage.getItem('zoo.analytics'))).toBeNull();
});

test('PLAT-024 PLAT-030 PLAT-027 grant loads the script and sends allowlisted events; switching off stops and clears cookies', async ({ page }) => {
  const urls = await spy(page);
  await open(page);
  await passGate(page);
  await page.click('#analytics-allow');
  await expect(page.locator('#analytics-toggle')).toHaveAttribute('aria-pressed', 'true');
  expect(await page.evaluate(() => localStorage.getItem('zoo.analytics'))).toBe('granted');
  await expect.poll(() => urls.filter((u) => u.includes('/gtag/js')).length).toBe(1);
  expect(urls.find((u) => u.includes('/gtag/js'))).toBe(`https://www.googletagmanager.com/gtag/js?id=${ID}`);
  await page.waitForFunction(() => (window as unknown as { __gtagLoaded?: boolean }).__gtagLoaded === true);

  // gtag setup: consent default denied first, then granted; private config
  const c = await calls(page);
  expect(c[0]).toEqual(['consent', 'default', { analytics_storage: 'denied', ad_storage: 'denied', ad_user_data: 'denied', ad_personalization: 'denied' }]);
  expect(c).toContainEqual(['consent', 'update', { analytics_storage: 'granted' }]);
  const cfg = c.find((x) => x[0] === 'config')!;
  expect(cfg[1]).toBe(ID);
  expect(cfg[2]).toMatchObject({ allow_google_signals: false, allow_ad_personalization_signals: false, cookie_flags: 'SameSite=Lax;Secure' });
  expect(String((cfg[2] as { page_location: string }).page_location)).not.toContain('?');
  expect(JSON.stringify(c)).not.toMatch(/user_id|user_properties/);

  // the real game: level_started for the level the player stands in, session event with language + reading level
  await expect.poll(async () => (await events(page)).some((e) => e[1] === 'level_started'), { timeout: 5000 }).toBe(true);
  expect((await events(page)).find((e) => e[1] === 'zoo_session')![2]).toEqual({ app_language: expect.stringMatching(/^(de|en)$/), reading_level: expect.stringMatching(/^(kiga|klasse[123])$/) });
  expect((await events(page)).find((e) => e[1] === 'level_started')![2]).toEqual({ level_id: expect.stringMatching(/^level_1$/) });

  // events from the game messages: ids only; unknown / free text never reaches gtag
  await page.evaluate(() => {
    const a = window.__zoo!.analytics;
    a.onGameEvent({ type: 'mission_complete', animal: 'zebra' });
    a.onGameEvent({ type: 'level_complete', level: 'level_1' });
    a.onGameEvent({ type: 'night' });
    a.onGameEvent({ type: 'baby_born', animal: 'panda' });
    a.onGameEvent({ type: 'all_home' });
    a.track('mission_complete', { animal_id: 'Anna Müller', x: 1 });
    a.track('custom_thing', { y: 2 });
  });
  const ev = await events(page);
  const names = ev.map((e) => e[1]);
  for (const n of ['mission_complete', 'level_complete', 'night_started', 'baby_born', 'all_animals_home']) expect(names, n).toContain(n);
  expect(names).not.toContain('custom_thing');
  expect(JSON.stringify(ev)).not.toMatch(/Anna|Müller/);
  const allowedKeys = new Set(['app_language', 'reading_level', 'level_id', 'animal_id', 'species_id', 'minutes']);
  for (const e of ev) for (const k of Object.keys(e[2] as object)) expect(allowedKeys.has(k), `${e[1]}.${k}`).toBe(true);

  // switching off: no gate, no more events, cookies cleared
  await page.evaluate(() => {
    document.cookie = '_ga=GA1.1.123.456; path=/';
    document.cookie = `_ga_${'G-TEST000000'.slice(2)}=GS1.1.1; path=/`;
  });
  expect(await page.evaluate(() => document.cookie)).toContain('_ga=');
  const before = (await events(page)).length;
  await openSettings(page);
  await page.click('#analytics-toggle');
  await expect(page.locator('#analytics-toggle')).toHaveAttribute('aria-pressed', 'false');
  await expect(page.locator('#analytics-dialog')).toBeHidden();
  expect(await page.evaluate(() => localStorage.getItem('zoo.analytics'))).toBe('denied');
  expect(await page.evaluate(() => document.cookie)).not.toContain('_ga');
  expect((await calls(page)).at(-1)).toEqual(['consent', 'update', { analytics_storage: 'denied' }]);
  await page.evaluate(() => window.__zoo!.analytics.onGameEvent({ type: 'mission_complete', animal: 'zebra' }));
  await page.waitForTimeout(1500);
  expect((await events(page)).length).toBe(before);
});

test('PLAT-024 a stored consent is resumed after a reload', async ({ page }) => {
  const urls = await spy(page);
  await open(page);
  await passGate(page);
  await page.click('#analytics-allow');
  await expect.poll(() => urls.length).toBeGreaterThan(0);
  urls.length = 0;
  await page.reload();
  await waitFrames(page, 3);
  await expect.poll(() => urls.filter((u) => u.includes('/gtag/js')).length).toBe(1);
  await expect(page.locator('script[src*="googletagmanager"]')).toHaveCount(1);
});

for (const [name, w, h] of [
  ['portrait phone', 412, 892],
  ['landscape phone', 892, 412],
  ['desktop', 1280, 800],
  ['small landscape', 780, 360],
] as const) {
  test(`PLAT-031 the 📊 button is at least 72 px and the settings menu fits (${name})`, async ({ page }) => {
    const urls = await spy(page);
    await open(page, w, h);
    await openSettings(page);
    const b = (await page.locator('#analytics-toggle').boundingBox())!;
    expect(b.width).toBeGreaterThanOrEqual(72);
    expect(b.height).toBeGreaterThanOrEqual(72);
    const s = (await page.locator('#settings').boundingBox())!;
    expect(s.x).toBeGreaterThanOrEqual(0);
    expect(s.y).toBeGreaterThanOrEqual(0);
    expect(s.x + s.width).toBeLessThanOrEqual(w + 0.5);
    expect(s.y + s.height).toBeLessThanOrEqual(h + 0.5);
    // the consent card fits as well (gate → card)
    await passGate(page);
    const card = (await page.locator('#analytics-dialog .an-card').boundingBox())!;
    expect(card.y).toBeGreaterThanOrEqual(0);
    expect(card.y + card.height).toBeLessThanOrEqual(h + 0.5);
    expect(card.x + card.width).toBeLessThanOrEqual(w + 0.5);
    for (const id of ['#analytics-allow', '#analytics-deny']) {
      const bb = (await page.locator(id).boundingBox())!;
      expect(bb.height).toBeGreaterThanOrEqual(64);
      expect(bb.y + bb.height).toBeLessThanOrEqual(h + 0.5);
    }
    await page.click('#analytics-deny');
    expect(urls).toEqual([]);
  });
}

test('PLAT-033 a small 3 s notice at the bottom, non-blocking, once per session; a tap opens the parental gate', async ({ page }) => {
  const urls = await spy(page);
  await open(page);
  const notice = page.locator('#analytics-notice');
  await expect(notice).toBeVisible({ timeout: 10_000 });
  const box = (await notice.boundingBox())!;
  const vp = page.viewportSize()!;
  expect(box.y + box.height).toBeLessThanOrEqual(vp.height);
  expect(box.y).toBeGreaterThan(vp.height * 0.6); // bottom area
  // non-blocking: the full-screen consent dialog is not shown and the game keeps running
  expect(await page.locator('#analytics-dialog').isVisible()).toBe(false);
  await expect(notice).toBeHidden({ timeout: 6_000 }); // gone by itself after ~3 s
  expect(urls).toEqual([]); // the notice records nothing
  // once per session: a reload in the same tab shows it no more
  await page.reload();
  await waitFrames(page, 3);
  await page.waitForTimeout(3500);
  await expect(page.locator('#analytics-notice')).toHaveCount(0);
});

test('PLAT-033 tapping the notice starts the parental gate, never consent by itself', async ({ page }) => {
  const urls = await spy(page);
  await open(page);
  const notice = page.locator('#analytics-notice');
  await expect(notice).toBeVisible({ timeout: 10_000 });
  await notice.dispatchEvent('pointerdown');
  await expect(page.locator('.an-question')).toBeVisible(); // the gate's sum
  expect(urls).toEqual([]);
  expect(await page.evaluate(() => window.localStorage.getItem('zoo.analytics'))).toBeNull();
});
