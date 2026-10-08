// TECH-PLATFORMS "Analytics (opt-in)": PLAT-022 (only the anonymous cookie-less ping before consent), PLAT-023 (parental gate),
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

/** One tap on the 📊 button in the settings grants consent (no question, no hold). */
async function grantByButton(page: Page) {
  await openSettings(page);
  await page.click('#analytics-toggle');
}

test('PLAT-022 before any decision only the anonymous cookie-less page ping runs; the button shows off', async ({ page }) => {
  const urls = await spy(page);
  await open(page);
  await nextFrames(page, 30);
  await page.waitForTimeout(1500);
  // gtag loads once, with every storage denied
  expect(urls.filter((u) => u.includes('/gtag/js'))).toEqual([`https://www.googletagmanager.com/gtag/js?id=${ID}`]);
  const c = await calls(page);
  expect(c[0]).toEqual(['consent', 'default', { analytics_storage: 'denied', ad_storage: 'denied', ad_user_data: 'denied', ad_personalization: 'denied' }]);
  const cfg = c.find((x) => x[0] === 'config')!;
  expect(cfg[2]).toMatchObject({ client_storage: 'none', allow_google_signals: false, allow_ad_personalization_signals: false });
  expect(c.some((x) => x[0] === 'consent' && x[1] === 'update')).toBe(false);
  expect((await events(page)).length).toBe(0); // no game events before consent
  expect(await page.evaluate(() => document.cookie)).not.toContain('_ga');
  expect(await page.evaluate(() => localStorage.getItem('zoo.analytics'))).toBeNull();
  expect(await page.locator('#analytics-dialog').isVisible()).toBe(false); // no consent dialog at start
  await openSettings(page);
  await expect(page.locator('#analytics-toggle')).toHaveAttribute('aria-pressed', 'false');
});

test('PLAT-023 one tap on the settings 📊 grants consent at once (no question, no hold) and the button then disappears', async ({ page }) => {
  const urls = await spy(page);
  await open(page);
  await openSettings(page);
  expect(await page.locator('.an-question, #analytics-hold, #analytics-dialog').count()).toBe(0);
  await page.click('#analytics-toggle');
  expect(await page.evaluate(() => localStorage.getItem('zoo.analytics'))).toBe('granted');
  await expect(page.locator('#settings-analytics')).toBeHidden(); // consented once: nothing more to show
  await expect.poll(() => urls.filter((u) => u.includes('/gtag/js')).length).toBe(1);
  // after a reload the button stays hidden
  await page.reload();
  await waitFrames(page, 3);
  await page.click('#settings-btn');
  await expect(page.locator('#settings-analytics')).toBeHidden();
});

test('PLAT-024 PLAT-030 PLAT-027 grant loads the script and sends allowlisted events; switching off stops and clears cookies', async ({ page }) => {
  const urls = await spy(page);
  await open(page);
  await grantByButton(page);
  expect(await page.evaluate(() => window.__zoo!.analytics.on)).toBe(true);
  expect(await page.evaluate(() => localStorage.getItem('zoo.analytics'))).toBe('granted');
  await expect.poll(() => urls.filter((u) => u.includes('/gtag/js')).length).toBe(1);
  expect(urls.find((u) => u.includes('/gtag/js'))).toBe(`https://www.googletagmanager.com/gtag/js?id=${ID}`);
  await page.waitForFunction(() => (window as unknown as { __gtagLoaded?: boolean }).__gtagLoaded === true);

  // gtag setup: consent default denied first, then granted; private config
  const c = await calls(page);
  expect(c[0]).toEqual(['consent', 'default', { analytics_storage: 'denied', ad_storage: 'denied', ad_user_data: 'denied', ad_personalization: 'denied' }]);
  expect(c).toContainEqual(['consent', 'update', { analytics_storage: 'granted' }]);
  const cfg = c.filter((x) => x[0] === 'config').at(-1)!; // the 2nd config (after consent); the 1st is the anonymous ping
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

  // switching off (analytics.deny): no more events, cookies cleared
  await page.evaluate(() => {
    document.cookie = '_ga=GA1.1.123.456; path=/';
    document.cookie = `_ga_${'G-TEST000000'.slice(2)}=GS1.1.1; path=/`;
  });
  expect(await page.evaluate(() => document.cookie)).toContain('_ga=');
  const before = (await events(page)).length;
  await page.evaluate(() => window.__zoo!.analytics.deny()); // (the button is hidden once consent is given)
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
  await grantByButton(page);
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
    expect(urls.filter((u) => !u.includes('/gtag/js'))).toEqual([]);
  });
}

test('PLAT-033 first start: welcome dialog with story, data note, light "No" and green "Yes"; Yes grants consent and starts the intro', async ({ page }) => {
  const urls = await spy(page);
  await page.setViewportSize({ width: 412, height: 892 });
  await page.addInitScript(() => localStorage.clear());
  await page.goto(`${START_URL}${START_URL.includes('?') ? '&' : '?'}intro=1`);
  const w = page.locator('#analytics-welcome');
  await expect(w).toBeVisible({ timeout: 10_000 });
  await expect(w).toContainText(/ausgebrochen|escaped/);
  const yes = page.locator('#welcome-yes');
  const no = page.locator('#welcome-no');
  for (const b of [yes, no]) expect((await b.boundingBox())!.height).toBeGreaterThanOrEqual(64);
  expect(await no.evaluate((e) => getComputedStyle(e).fontWeight)).not.toBe('700'); // light
  expect(await yes.evaluate((e) => getComputedStyle(e).backgroundColor)).toBe('rgb(124, 196, 106)'); // green
  expect(urls.filter((u) => !u.includes('/gtag/js'))).toEqual([]); // nothing before the answer
  await yes.click();
  await expect(w).toHaveCount(0);
  expect(await page.evaluate(() => localStorage.getItem('zoo.analytics'))).toBe('granted');
  await expect(page.locator('#intro')).toBeVisible(); // the intro follows
});

test('PLAT-033 "No, I don\'t want to play" stores nothing, sends nothing, shows goodbye; back asks again; not shown once decided', async ({ page }) => {
  const urls = await spy(page);
  await page.setViewportSize({ width: 412, height: 892 });
  await page.addInitScript(() => {
    if (!sessionStorage.getItem('zoo.e2e.init')) {
      sessionStorage.setItem('zoo.e2e.init', '1');
      localStorage.clear();
    }
  });
  const url = `${START_URL}${START_URL.includes('?') ? '&' : '?'}intro=1`;
  await page.goto(url);
  await page.click('#welcome-no');
  await expect(page.locator('#welcome-back')).toBeVisible();
  expect(await page.evaluate(() => localStorage.getItem('zoo.analytics'))).toBeNull();
  await page.click('#welcome-back');
  await expect(page.locator('#welcome-yes')).toBeVisible();
  expect(urls.filter((u) => !u.includes('/gtag/js'))).toEqual([]);
  await page.click('#welcome-yes');
  await page.reload(); // decided: no welcome any more
  await waitFrames(page, 3);
  await expect(page.locator('#analytics-welcome')).toHaveCount(0);
});

for (const [w, h] of [
  [280, 560],
  [320, 568],
  [360, 640],
  [360, 780],
  [412, 892],
  [568, 320],
  [780, 360],
] as const) {
  test(`PLAT-033 welcome buttons keep their text inside the box (${w}x${h})`, async ({ page }) => {
    await spy(page);
    await page.setViewportSize({ width: w, height: h });
    await page.addInitScript(() => localStorage.clear());
    await page.goto(`${START_URL}${START_URL.includes('?') ? '&' : '?'}intro=1`);
    await expect(page.locator('#analytics-welcome')).toBeVisible({ timeout: 15_000 });
    for (const sel of ['#welcome-yes', '#welcome-no']) {
      const m = await page.locator(sel).evaluate((e) => ({
        sw: e.scrollWidth,
        cw: e.clientWidth,
        sh: e.scrollHeight,
        ch: e.clientHeight,
        fs: parseFloat(getComputedStyle(e).fontSize),
      }));
      expect(m.sw, `${sel} text wider than the box`).toBeLessThanOrEqual(m.cw + 1);
      expect(m.sh, `${sel} text taller than the box`).toBeLessThanOrEqual(m.ch + 1);
      expect(m.fs, `${sel} font size`).toBeGreaterThanOrEqual(11);
    }
    const card = (await page.locator('.an-card').boundingBox())!;
    expect(card.x).toBeGreaterThanOrEqual(0);
    expect(card.x + card.width).toBeLessThanOrEqual(w + 0.5);
  });
}
