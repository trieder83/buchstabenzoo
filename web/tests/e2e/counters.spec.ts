// TECH-PLATFORMS "Anonymous counters": PLAT-049 (game events produce the expected commit bodies), PLAT-050 (nothing is sent
// with Do-Not-Track / in automation / on dev hosts), PLAT-051 (ad flow: panel, gate right / wrong, hold, release).
// All Firestore requests are intercepted by a stub (no production number is ever touched); `?count=1` switches the
// counters on for localhost + automation. The ad flow needs the test build (`?adkey=`), scripts/e2e.sh picks it.
import fs from 'node:fs';
import path from 'node:path';
import { expect, test, type Page } from '@playwright/test';
import { ftl, goto, nextFrames, repo, waitFrames } from './helpers';

const TEST_BUILD = process.env.E2E_DIST === 'dist-adtest';
const fixtures = path.join(repo, 'web/tests/fixtures/ads');
const PUB = fs.readFileSync(path.join(fixtures, 'TEST-ONLY-public.key'), 'utf8').trim();
const FIRESTORE = /firestore\.googleapis\.com/;

/** Stub for the Firestore endpoint: records the written document ids and their fields. */
async function stub(page: Page) {
  const docs: string[] = [];
  const bodies: { writes: { update: { name: string; fields: Record<string, { stringValue: string }> }; updateTransforms: unknown[] }[] }[] = [];
  let requests = 0;
  await page.route(FIRESTORE, async (route) => {
    requests++;
    const b = JSON.parse(route.request().postData() ?? '{}');
    bodies.push(b);
    for (const w of b.writes ?? []) docs.push(String(w.update.name).replace(/^.*\/c\//, ''));
    await route.fulfill({ status: 200, contentType: 'application/json', body: '{}' });
  });
  return { docs, bodies, requests: () => requests };
}

/** Flushes the queue now (the page-hide path) as often as needed for queued repeats. */
async function flush(page: Page, times = 3) {
  for (let i = 0; i < times; i++) {
    await page.evaluate(() => window.dispatchEvent(new Event('pagehide')));
    await page.waitForTimeout(1100);
  }
}

const has = (docs: string[], event: string, param = '') => docs.some((d) => new RegExp(`^\\d{8}_${event}_${param}_web_de_[a-z0-9.-]+$`).test(d));

async function open(page: Page, query: string, level = 'klasse1') {
  await page.setViewportSize({ width: 1280, height: 720 });
  await page.addInitScript((lv) => {
    if (sessionStorage.getItem('zoo.e2e.init')) return;
    sessionStorage.setItem('zoo.e2e.init', '1');
    localStorage.clear();
    localStorage.setItem('zoo.readingLevel', lv);
    localStorage.setItem('zoo.language', 'de');
    (window as unknown as { __opens: unknown[] }).__opens = [];
    window.open = (...a: unknown[]) => {
      (window as unknown as { __opens: unknown[] }).__opens.push(a);
      return null;
    };
  }, level);
  await page.goto(`/?seed=17${query}`);
  await waitFrames(page, 3);
  await page.evaluate(() => {
    document.getElementById('celebrate')!.hidden = true;
  });
}

const NOTE = { stand: [-10.5, 2.5], at: [-10.5, 3.7] };
const BOX = { stand: [-7.5, 1.5], at: [-7.95, 1.5] };
async function stand(page: Page, p: { stand: number[]; at: number[] }) {
  await page.evaluate((p) => {
    const a = window.__zoo!.app;
    a.debug_teleport(p.stand[0], p.stand[1]);
    a.debug_face_point(p.at[0], p.at[1]);
  }, p);
  await nextFrames(page, 4);
}

test('PLAT-049 note, wrong code, right code, hint and level start produce the expected commit bodies', async ({ page }) => {
  const s = await stub(page);
  await open(page, '&count=1');
  await stand(page, NOTE);
  await expect(page.locator('#panel[data-kind="note"]')).toBeVisible();
  const expr = (await page.locator('#note-expr').textContent())!;
  const m = /^(\d+) ([+−×÷]) (\d+) = \?$/.exec(expr.trim())!;
  const [a, b] = [Number(m[1]), Number(m[3])];
  const answer = m[2] === '+' ? a + b : m[2] === '−' ? a - b : m[2] === '×' ? a * b : a / b;
  await page.keyboard.press('Escape');
  await stand(page, BOX);
  await page.keyboard.press('KeyE');
  await expect(page.locator('#lock-panel')).toBeVisible();
  await page.locator('#lock-ok').click(); // 000: wrong
  await expect(page.locator('#lock-panel')).toBeVisible();
  for (let i = 0; i < 3; i++) {
    const d = String(answer).padStart(3, '0')[i];
    await page.keyboard.press('Digit' + d);
  }
  await page.locator('#lock-ok').click();
  await expect(page.locator('#lock-panel')).toBeHidden();
  await page.locator('#compass-btn').dispatchEvent('pointerdown');
  await goto(page, -9, 4);
  await flush(page);
  const t = ftl('de');
  expect(t['cart-key-got']).toBeTruthy();
  for (const [ev, p] of [['session_start', ''], ['key_note_read', ''], ['lock_wrong', ''], ['lock_ok', ''], ['key_box_opened', ''], ['hint_used', 'compass']]) {
    expect(has(s.docs, ev, p), `${ev}\n${s.docs.join('\n')}`).toBe(true);
  }
  expect(s.docs.some((d) => /^\d{8}_level_started_[a-z0-9_-]+_web_de_/.test(d))).toBe(true);
  // exactly the allowed fields, an increment of 1, no extra data
  const w = s.bodies[0].writes[0];
  expect(Object.keys(w.update.fields).sort()).toEqual(['date', 'event', 'lang', 'param', 'platform', 'v']);
  expect(w.updateTransforms).toEqual([{ fieldPath: 'n', increment: { integerValue: '1' } }]);
  expect(JSON.stringify(s.bodies)).not.toMatch(/seed|localhost|client|user/i);
  // nothing is stored by the counters: no cookies at all, no counter keys in the storage
  expect(await page.evaluate(() => document.cookie)).toBe('');
  expect(await page.evaluate(() => Object.keys(localStorage).filter((k) => /count|firestore|zoo\.c/i.test(k)))).toEqual([]);
});

async function pokeAndFlush(page: Page) {
  await stand(page, BOX);
  await page.keyboard.press('KeyE');
  await page.locator('#lock-ok').click();
  await flush(page, 1);
}

test('PLAT-050 nothing is sent with Do-Not-Track, even with ?count=1', async ({ page }) => {
  const s = await stub(page);
  await page.addInitScript(() => Object.defineProperty(navigator, 'doNotTrack', { get: () => '1' }));
  await open(page, '&count=1');
  await pokeAndFlush(page);
  expect(s.requests()).toBe(0);
});

test('PLAT-050 nothing is sent in automation without ?count=1', async ({ page }) => {
  const s = await stub(page);
  await open(page, '');
  await pokeAndFlush(page);
  expect(s.requests()).toBe(0);
});

test('PLAT-050 nothing is sent with ?nocount=1', async ({ page }) => {
  const s = await stub(page);
  await open(page, '&count=1&nocount=1');
  await pokeAndFlush(page);
  expect(s.requests()).toBe(0);
});

test('PLAT-050 Global Privacy Control switches the counters off', async ({ page }) => {
  const s = await stub(page);
  await page.addInitScript(() => Object.defineProperty(navigator, 'globalPrivacyControl', { get: () => true }));
  await open(page, '&count=1');
  await flush(page, 1);
  expect(s.requests()).toBe(0);
});

test.describe('ad billboard flow (test build)', () => {
  test.skip(!TEST_BUILD, 'needs E2E_DIST=dist-adtest (?adkey= test hook)');

  async function serveAds(page: Page) {
    const dir = path.join(fixtures, 'boards');
    const files = new Map<string, Buffer>();
    files.set('index.json', fs.readFileSync(path.join(dir, 'index.json')));
    files.set('index.sig', fs.readFileSync(path.join(dir, 'index.sig')));
    for (const f of fs.readdirSync(path.join(dir, 'img'))) files.set(`img/${f}`, fs.readFileSync(path.join(dir, 'img', f)));
    await page.route('**/boards/**', async (route) => {
      const rel = new URL(route.request().url()).pathname.replace(/^.*\/boards\//, '');
      const body = files.get(rel);
      if (!body) return route.fulfill({ status: 404 });
      return route.fulfill({ status: 200, body, contentType: rel.endsWith('.png') ? 'image/png' : 'text/plain' });
    });
  }
  const gateAnswer = (q: string) => {
    const [, a, op, b] = /(\d+) ([+−]) (\d+)/.exec(q)!;
    return op === '+' ? Number(a) + Number(b) : Number(a) - Number(b);
  };

  test('PLAT-051 panel, link tap, wrong answer, right answer, early release, full hold and release', async ({ page }) => {
    const s = await stub(page);
    await serveAds(page);
    await open(page, `&count=1&adkey=${encodeURIComponent(PUB)}`, 'klasse2');
    await page.waitForFunction(() => window.__zoo!.ads.state().settled, undefined, { timeout: 30_000 });
    const list = JSON.parse(await page.evaluate(() => window.__zoo!.app.ad_boards_json())) as { id: string; slot: number; x: number; z: number }[];
    const b = list.find((x) => x.slot === 1)!;
    for (const d of [1.6, 2.4, 1.0, 2.9]) {
      await goto(page, b.x, b.z - d);
      await page.evaluate(() => window.__zoo!.app.debug_step(0.3));
      await nextFrames(page, 3);
      if ((await page.evaluate(() => window.__zoo!.app.ad_near())) === b.id) break;
    }
    await expect(page.locator('#zb-panel')).toBeVisible();
    // a wrong answer closes the gate
    await page.locator('#zb-link').click();
    let q = (await page.locator('#zb-gate-question').textContent())!;
    let answer = gateAnswer(q);
    await page.locator('.zb-choice').filter({ hasNotText: new RegExp(`^${answer}$`) }).first().click();
    await expect(page.locator('#zb-gate')).toBeHidden();
    // right answer, early release, then the full hold + release
    await page.locator('#zb-link').click();
    q = (await page.locator('#zb-gate-question').textContent())!;
    answer = gateAnswer(q);
    await page.locator('.zb-choice', { hasText: new RegExp(`^${answer}$`) }).click();
    const hb = (await page.locator('#zb-hold').boundingBox())!;
    await page.mouse.move(hb.x + hb.width / 2, hb.y + hb.height / 2);
    await page.mouse.down();
    await page.waitForTimeout(600);
    await page.mouse.up();
    await page.mouse.down();
    await expect(page.locator('#zb-hold.ready')).toBeVisible({ timeout: 8000 });
    await page.mouse.up();
    await page.waitForTimeout(500);
    await flush(page);
    for (const ev of ['board_panel_opened', 'board_link_tapped', 'gate_answer_wrong', 'gate_answer_right', 'gate_hold_started', 'gate_hold_cancelled', 'gate_hold_complete', 'board_link_opened']) {
      expect(has(s.docs, ev, 's1_web'), `${ev}\n${s.docs.join('\n')}`).toBe(true);
    }
    // tapped twice -> the second count of the same document went into a later commit (never two writes of one document in one commit)
    for (const body of s.bodies) {
      const names = body.writes.map((w) => w.update.name);
      expect(new Set(names).size).toBe(names.length);
    }
  });
});
