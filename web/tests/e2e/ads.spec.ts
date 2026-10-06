// GAME-ADS: ad billboards with externally loaded, signed content (ADS-003, ADS-004, ADS-020…023,
// ADC1-002..006, ADC2-002..004).
//
// The signature tests need the test build (`E2E_DIST=dist-adtest scripts/e2e.sh web/tests/e2e/ads.spec.ts`,
// it trusts the test key given as ?adkey=); the release build has no such code path (PLAT-012).
// The placeholder tests run on either build.
import * as ed from '@noble/ed25519';
import fs from 'node:fs';
import path from 'node:path';
import { expect, test, type Page } from '@playwright/test';
import { ftl, goto, nextFrames, repo, shots, waitFrames } from './helpers';

const fixtures = path.join(repo, 'web/tests/fixtures/ads');
const adsDir = path.join(fixtures, 'ads');
const PUB = fs.readFileSync(path.join(fixtures, 'TEST-ONLY-public.key'), 'utf8').trim();
const TEST_BUILD = process.env.E2E_DIST === 'dist-adtest';

interface Board {
  id: string;
  slot: number;
  n: number;
  x: number;
  z: number;
}

/** Serves `ads/**` from memory (the Vite preview has no manifest); records every request URL. */
async function serveAds(page: Page, files: Map<string, Buffer>, requests: string[]): Promise<void> {
  await page.route('**/ads/**', async (route) => {
    const rel = new URL(route.request().url()).pathname.replace(/^.*\/ads\//, '');
    const body = files.get(rel);
    if (!body) return route.fulfill({ status: 404 });
    return route.fulfill({ status: 200, body, contentType: rel.endsWith('.png') ? 'image/png' : 'text/plain' });
  });
  page.on('request', (r) => requests.push(r.url()));
}

function fixtureFiles(): Map<string, Buffer> {
  const files = new Map<string, Buffer>();
  files.set('campaigns.json', fs.readFileSync(path.join(adsDir, 'campaigns.json')));
  files.set('campaigns.sig', fs.readFileSync(path.join(adsDir, 'campaigns.sig')));
  for (const f of fs.readdirSync(path.join(adsDir, 'img'))) files.set(`img/${f}`, fs.readFileSync(path.join(adsDir, 'img', f)));
  return files;
}

/** The fixture manifest re-signed with another (wrong) key. */
async function wrongKeyFiles(): Promise<Map<string, Buffer>> {
  const files = fixtureFiles();
  const body = files.get('campaigns.json')!;
  const domain = Buffer.from('buchstabenzoo-ads/1\n');
  const sig = await ed.signAsync(Buffer.concat([domain, body]), ed.utils.randomSecretKey());
  files.set('campaigns.sig', Buffer.from(Buffer.from(sig).toString('base64')));
  return files;
}

async function start(page: Page, query = ''): Promise<void> {
  await page.addInitScript(() => {
    if (sessionStorage.getItem('zoo.e2e.init')) return;
    sessionStorage.setItem('zoo.e2e.init', '1');
    localStorage.clear();
    localStorage.setItem('zoo.readingLevel', 'klasse2');
    localStorage.setItem('zoo.language', 'de');
    // records window.open instead of opening a tab (ADC1-004)
    (window as unknown as { __opens: unknown[] }).__opens = [];
    window.open = (...a: unknown[]) => {
      (window as unknown as { __opens: unknown[] }).__opens.push(a);
      return null;
    };
  });
  await page.goto(`/?seed=17${query}`);
  await waitFrames(page, 3);
  await page.evaluate(() => {
    window.__zoo!.app.set_language('de');
    window.__zoo!.app.set_reading_level('klasse2');
  });
}

/** The gate task `a + b` or `a − b` (up to 20) → its result. */
function gateAnswer(q: string): number {
  const [, a, op, b] = /(\d+) ([+−]) (\d+)/.exec(q)!;
  return op === '+' ? Number(a) + Number(b) : Number(a) - Number(b);
}

const settled = (page: Page) => page.waitForFunction(() => window.__zoo!.ads.state().settled, undefined, { timeout: 30_000 });

async function boards(page: Page): Promise<Board[]> {
  return JSON.parse(await page.evaluate(() => window.__zoo!.app.ad_boards_json()));
}

/** Stands in front (south) of a board, inside its reading range. */
async function standAt(page: Page, b: Board): Promise<void> {
  for (const d of [1.6, 2.4, 1.0, 2.9]) {
    await goto(page, b.x, b.z - d);
    await page.evaluate(() => window.__zoo!.app.debug_step(0.3));
    await nextFrames(page, 3);
    if ((await page.evaluate(() => window.__zoo!.app.ad_near())) === b.id) return;
  }
  throw new Error(`cannot stand in front of ${b.id}`);
}

const opens = (page: Page) => page.evaluate(() => (window as unknown as { __opens: unknown[][] }).__opens);

test('ADS-003 ADS-004 placeholders (no manifest available): every board has its picture, passive, no request to the advertiser', async ({ page }) => {
  const requests: string[] = [];
  page.on('request', (r) => requests.push(r.url()));
  // the production key is compiled in, so the real manifest would load: simulate "offline" (no manifest)
  await page.route('**/ads/**', (r) => r.abort());
  await start(page);
  const list = await boards(page);
  expect(list.length).toBe(18); // 4 per day level + 3 per night level (ADS-001)
  const slots = [1, 2, 3].map((s) => list.filter((b) => b.slot === s).length);
  expect(Math.min(...slots)).toBeGreaterThanOrEqual(2); // ADS-002
  await waitFrames(page, 8);
  await page.waitForFunction(
    (ids) => ids.every((id) => window.__zoo!.app.decal_drawn(`ad:${id}`)),
    list.map((b) => b.id),
    { timeout: 30_000 },
  );
  // the texts of the placeholders (Fluent, both languages)
  const de = ftl('de');
  const en = ftl('en');
  expect(await page.evaluate(() => [1, 2, 3].map((n) => window.__zoo!.app.t(`ad-placeholder-${n}`)))).toEqual([1, 2, 3].map((n) => de[`ad-placeholder-${n}`]));
  expect(de['ad-placeholder-2']).toBe('Deine Werbung 2');
  expect(en['ad-placeholder-2']).toBe('Your ad 2');
  // standing in front of a placeholder board: no panel, no interact target (ADS-004)
  await standAt(page, list[0]);
  expect(await page.evaluate(() => window.__zoo!.app.ad_near())).toBe(list[0].id);
  await expect(page.locator('#ad-panel')).toBeHidden();
  expect(await page.evaluate(() => window.__zoo!.app.target_kind())).toBe('');
  expect(requests.filter((u) => /rcms\.ch/.test(u))).toEqual([]); // never a request to the advertiser (ADC1-005)
  await page.screenshot({ path: path.join(shots, 'ads-placeholder.png') });
});

test.describe('signed campaigns (test build)', () => {
  test.skip(!TEST_BUILD, 'needs E2E_DIST=dist-adtest (?adkey= test hook)');

  test('ADS-020 ADC1-002 ADC1-003 ADC1-004 ADC1-005 a signed campaign: picture, panel, link behind the gate, opened once', async ({ page }) => {
    const requests: string[] = [];
    await serveAds(page, fixtureFiles(), requests);
    await start(page, `&adkey=${encodeURIComponent(PUB)}`);
    await settled(page);
    expect(await page.evaluate(() => window.__zoo!.ads.state().campaigns)).toEqual({ 1: 'mathfighter', 2: 'abcsmash', 3: 'edugamegalaxy' });
    const mf = (await boards(page)).find((b) => b.slot === 1)!;
    await standAt(page, mf);
    await expect(page.locator('#ad-panel')).toBeVisible();
    await expect(page.locator('#ad-image')).toBeVisible();
    await expect(page.locator('#ad-tagline')).toHaveText('Lerne Mathe in einem lustigen Turnier');
    expect(await page.locator('#ad-image').getAttribute('src')).toMatch(/^blob:/);
    await page.screenshot({ path: path.join(shots, 'ads-panel.png') });
    // no request to the campaign host before the click (ADC1-005); only own same-origin ads files
    expect(requests.filter((u) => /rcms\.ch/.test(u))).toEqual([]);
    expect(requests.filter((u) => /\/ads\//.test(u)).every((u) => new URL(u).origin === new URL(page.url()).origin)).toBe(true);
    // link button >= 64 px with the host as text
    const link = page.locator('#ad-link');
    const box = (await link.boundingBox())!;
    expect(box.height).toBeGreaterThanOrEqual(64);
    expect(box.width).toBeGreaterThanOrEqual(64);
    await expect(page.locator('#ad-link-text')).toHaveText('mathfighter.rcms.ch');
    // the gate comes first; nothing opens yet
    await link.click();
    await expect(page.locator('#ad-gate')).toBeVisible();
    expect(await opens(page)).toEqual([]);
    // the sum: the right answer, then 2 s of holding
    const q = (await page.locator('#ad-gate-question').textContent())!;
    const answer = gateAnswer(q);
    await page.locator('.ad-choice', { hasText: new RegExp(`^${answer}$`) }).click();
    const hold = page.locator('#ad-hold');
    await expect(hold).toBeVisible();
    const hb = (await hold.boundingBox())!;
    // releasing early opens nothing
    await page.mouse.move(hb.x + hb.width / 2, hb.y + hb.height / 2);
    await page.mouse.down();
    await page.waitForTimeout(1200);
    await page.mouse.up();
    expect(await opens(page)).toEqual([]);
    await expect(page.locator('#ad-gate')).toBeVisible();
    // holding for 2 s opens the link exactly once, in a new tab without opener
    await page.mouse.down();
    // full hold: the ✔ shows, nothing opened yet (a timer is no user gesture); the RELEASE opens it (ADS-030)
    await expect(page.locator('#ad-hold.ready')).toBeVisible({ timeout: 8000 });
    expect(await opens(page)).toEqual([]);
    await page.mouse.up();
    await page.waitForTimeout(500);
    expect(await opens(page)).toEqual([['https://mathfighter.rcms.ch/', '_blank', 'noopener,noreferrer']]);
    await expect(page.locator('#ad-gate')).toBeHidden();
    expect(requests.filter((u) => /rcms\.ch/.test(u))).toEqual([]); // the game itself never calls it
  });

  test('ADC1-004 a wrong answer or cancelling opens nothing', async ({ page }) => {
    await serveAds(page, fixtureFiles(), []);
    await start(page, `&adkey=${encodeURIComponent(PUB)}`);
    await settled(page);
    const mf = (await boards(page)).find((b) => b.slot === 1)!;
    await standAt(page, mf);
    await page.locator('#ad-link').click();
    const q = (await page.locator('#ad-gate-question').textContent())!;
    const answer = gateAnswer(q);
    const wrong = page.locator('.ad-choice').filter({ hasNotText: new RegExp(`^${answer}$`) }).first();
    await wrong.click();
    await expect(page.locator('#ad-gate')).toBeHidden();
    expect(await opens(page)).toEqual([]);
    // the cancel button of a new gate
    await page.locator('#ad-link').click();
    await page.locator('#ad-gate .ad-x').click();
    await expect(page.locator('#ad-gate')).toBeHidden();
    // Esc closes the panel, which stays closed until the player walks away and comes back
    await page.keyboard.press('Escape');
    await expect(page.locator('#ad-panel')).toBeHidden();
    await nextFrames(page, 3);
    await expect(page.locator('#ad-panel')).toBeHidden();
    expect(await opens(page)).toEqual([]);
  });

  test('ADC2-002 ADC2-003 ABC Smash: image of the language, tagline by language, kiga shows the picture only', async ({ page }) => {
    await serveAds(page, fixtureFiles(), []);
    await start(page, `&adkey=${encodeURIComponent(PUB)}`);
    await settled(page);
    const abc = (await boards(page)).find((b) => b.slot === 2)!;
    await standAt(page, abc);
    await expect(page.locator('#ad-tagline')).toHaveText('Lesen lernen – flüssig und schnell');
    await expect(page.locator('#ad-link-text')).toHaveText('abcsmash.rcms.ch');
    expect(await page.locator('#ad-image').getAttribute('data-path')).toContain('abcsmash-de');
    await page.evaluate(() => window.__zoo!.app.set_language('en'));
    await expect(page.locator('#ad-tagline')).toHaveText('Learn to read – fluent and fast');
    expect(await page.locator('#ad-image').getAttribute('data-path')).toContain('abcsmash-en');
    await page.evaluate(() => window.__zoo!.app.set_reading_level('kiga'));
    await expect(page.locator('#ad-tagline')).toHaveCount(0);
    await expect(page.locator('#ad-image')).toBeVisible();
    await page.evaluate(() => window.__zoo!.app.set_reading_level('klasse2'));
    await page.locator('#ad-link').click();
    // ABC Smash asks a language question (ADS-025): the right German article, e.g. "… Gabel" → die
    const q = (await page.locator('#ad-gate-question').textContent())!;
    const plurals: Record<string, string> = { mouse: 'mice', child: 'children', foot: 'feet', man: 'men', tooth: 'teeth', goose: 'geese' };
    const one = /one (\S+),/.exec(q)?.[1];
    const noun = one ?? /… (\S+)/.exec(q)![1];
    const article = one ? plurals[one] : ({ Gabel: 'die', Löffel: 'der', Messer: 'das', Tisch: 'der', Lampe: 'die', Haus: 'das', Hund: 'der', Katze: 'die', Buch: 'das', Ball: 'der', Schule: 'die', Auto: 'das', Apfel: 'der', Blume: 'die', Fenster: 'das', Stuhl: 'der' } as Record<string, string>)[noun];
    expect(article, `noun ${noun}`).toBeTruthy();
    await page.locator('.ad-choice', { hasText: new RegExp(`^${article}$`) }).click();
    const hb = (await page.locator('#ad-hold').boundingBox())!;
    await page.mouse.move(hb.x + hb.width / 2, hb.y + hb.height / 2);
    await page.mouse.down();
    // full hold: the ✔ shows, nothing opened yet (a timer is no user gesture); the RELEASE opens it (ADS-030)
    await expect(page.locator('#ad-hold.ready')).toBeVisible({ timeout: 8000 });
    expect(await opens(page)).toEqual([]);
    await page.mouse.up();
    expect(await opens(page)).toEqual([['https://abcsmash.rcms.ch/', '_blank', 'noopener,noreferrer']]);
  });

  test('ADC3-002 ADC3-003 EduGameGalaxy: image + tagline by language, link behind the plus/minus gate, opened once', async ({ page }) => {
    const requests: string[] = [];
    await serveAds(page, fixtureFiles(), requests);
    await start(page, `&adkey=${encodeURIComponent(PUB)}`);
    await settled(page);
    const edu = (await boards(page)).find((b) => b.slot === 3)!;
    await standAt(page, edu);
    await expect(page.locator('#ad-panel')).toBeVisible();
    await expect(page.locator('#ad-tagline')).toHaveText('Effizient und mit Spaß lernen – für den Erfolg im Leben');
    await expect(page.locator('#ad-link-text')).toHaveText('edugamegalaxy.rcms.ch');
    expect(await page.locator('#ad-image').getAttribute('data-path')).toContain('edugamegalaxy-de');
    await page.evaluate(() => window.__zoo!.app.set_language('en'));
    await expect(page.locator('#ad-tagline')).toHaveText('Learn efficiently and with fun – for success in life');
    expect(await page.locator('#ad-image').getAttribute('data-path')).toContain('edugamegalaxy-en');
    await page.evaluate(() => window.__zoo!.app.set_reading_level('kiga'));
    await expect(page.locator('#ad-tagline')).toHaveCount(0);
    await page.evaluate(() => window.__zoo!.app.set_reading_level('klasse2'));
    expect(requests.filter((u) => /rcms\.ch/.test(u))).toEqual([]); // ADC3-004
    await page.locator('#ad-link').click();
    const q = (await page.locator('#ad-gate-question').textContent())!;
    await page.locator('.ad-choice', { hasText: new RegExp(`^${gateAnswer(q)}$`) }).click();
    const hb = (await page.locator('#ad-hold').boundingBox())!;
    await page.mouse.move(hb.x + hb.width / 2, hb.y + hb.height / 2);
    await page.mouse.down();
    // full hold: the ✔ shows, nothing opened yet (a timer is no user gesture); the RELEASE opens it (ADS-030)
    await expect(page.locator('#ad-hold.ready')).toBeVisible({ timeout: 8000 });
    expect(await opens(page)).toEqual([]);
    await page.mouse.up();
    expect(await opens(page)).toEqual([['https://edugamegalaxy.rcms.ch/', '_blank', 'noopener,noreferrer']]);
    expect(requests.filter((u) => /rcms\.ch/.test(u))).toEqual([]);
  });

  test('ADC1-006 a board whose campaign is not delivered stays a passive placeholder next to real campaigns', async ({ page }) => {
    const files = fixtureFiles();
    for (const l of ['de', 'en']) {
      const img = Buffer.from(files.get(`img/edugamegalaxy-${l}.png`)!);
      img[img.length - 12] ^= 0xff; // hash mismatch: campaign 3 is dropped
      files.set(`img/edugamegalaxy-${l}.png`, img);
    }
    await serveAds(page, files, []);
    await start(page, `&adkey=${encodeURIComponent(PUB)}`);
    await settled(page);
    expect(await page.evaluate(() => window.__zoo!.ads.state().campaigns)).toEqual({ 1: 'mathfighter', 2: 'abcsmash' });
    const third = (await boards(page)).find((b) => b.slot === 3)!;
    await standAt(page, third);
    await expect(page.locator('#ad-panel')).toBeHidden();
    expect(await page.evaluate(() => window.__zoo!.app.target_kind())).toBe('');
    expect(await page.evaluate((id) => window.__zoo!.app.decal_drawn(`ad:${id}`), third.id)).toBe(true);
  });

  test('ADS-021 a manifest signed with the wrong key is rejected: placeholders, no panel', async ({ page }) => {
    const requests: string[] = [];
    await serveAds(page, await wrongKeyFiles(), requests);
    await start(page, `&adkey=${encodeURIComponent(PUB)}`);
    await settled(page);
    const st = await page.evaluate(() => window.__zoo!.ads.state());
    expect(st.loaded).toBe(false);
    expect(st.campaigns).toEqual({});
    expect(requests.filter((u) => /\/ads\/img\//.test(u))).toEqual([]); // no image fetched
    const mf = (await boards(page))[0];
    await standAt(page, mf);
    await expect(page.locator('#ad-panel')).toBeHidden();
  });

  test('ADS-021 a tampered image drops its campaign, the others still show', async ({ page }) => {
    const files = fixtureFiles();
    const img = Buffer.from(files.get('img/mathfighter-wide.png')!);
    img[img.length - 12] ^= 0xff;
    files.set('img/mathfighter-wide.png', img);
    await serveAds(page, files, []);
    await start(page, `&adkey=${encodeURIComponent(PUB)}`);
    await settled(page);
    expect(await page.evaluate(() => window.__zoo!.ads.state().campaigns)).toEqual({ 2: 'abcsmash', 3: 'edugamegalaxy' });
  });

  test('ADS-022 without the test key a manifest signed by another key is not trusted (only the compiled production key counts)', async ({ page }) => {
    const requests: string[] = [];
    await serveAds(page, fixtureFiles(), requests);
    await start(page); // no ?adkey=: only the compiled production key is trusted
    await settled(page);
    expect(await page.evaluate(() => window.__zoo!.ads.state().loaded)).toBe(false);
  });
});
