// Reading panel on phones (GAME-PLAYER §4, ADIR-003) and while playing with touch.
// - PLAY-030 (proposed, Q-070): on a phone (portrait and landscape) the whole panel (riddle +
//   food word) fits without scrolling at every reading level, with cap height ≥ 3 %.
// - PLAY-025: with a panel open the left thumb still walks and the panel does not cover the
//   player (portrait and landscape).
// - RESC-017 (proposed, Q-069): no info board shows a raw Fluent key.
import { expect, test, type Page } from '@playwright/test';
import { goto, nextFrames } from '../helpers';
import { ensurePanel, RAW_KEY, startGame, textOf, touch, turn } from './qa';

test.describe.configure({ timeout: 180_000 });

const PHONE = { deviceScaleFactor: 2.625, hasTouch: true, isMobile: true };

async function openZebraBoard(page: Page) {
  await goto(page, -7.5, 10.5);
  await turn(page, 'KeyA');
  await ensurePanel(page, async () => {
    await page.locator('#act').dispatchEvent('pointerdown');
    await nextFrames(page, 2);
  });
}

async function firstTouch(page: Page) {
  const cdp = await page.context().newCDPSession(page);
  const vp = page.viewportSize()!;
  await touch(cdp, 'touchStart', [{ x: vp.width * 0.75, y: vp.height * 0.3, id: 1 }]);
  await touch(cdp, 'touchEnd', []);
  await nextFrames(page, 2);
  return cdp;
}

async function panelMetrics(page: Page) {
  return page.evaluate(() => {
    const body = document.querySelector('#panel .panel-body') as HTMLElement;
    const r = body.getBoundingClientRect();
    const px = parseFloat(getComputedStyle(document.getElementById('panel-text')!).fontSize);
    return {
      scroll: body.scrollHeight,
      client: body.clientHeight,
      top: r.top,
      bottom: r.bottom,
      left: r.left,
      right: r.right,
      cap: (0.7 * px) / window.innerHeight,
      w: window.innerWidth,
      h: window.innerHeight,
    };
  });
}

// QA F2: M4 overflowed in portrait at klasse3 (2× the panel); M4b overflows at every level in
// both orientations (portrait klasse3: 2 295 px content in a 317 px panel; landscape: 135 px).
for (const [orientation, vp] of [
  ['portrait', { width: 412, height: 892 }],
  ['landscape', { width: 892, height: 412 }],
] as const) test.describe(`${orientation} phone ${vp.width}×${vp.height} (1080×2340 px)`, () => {
  test.use({ viewport: vp, ...PHONE });
  for (const lang of ['de', 'en']) {
    for (const level of ['kiga', 'klasse1', 'klasse2', 'klasse3']) {
      test(`PLAY-030 / ADIR-003: board panel fits without scrolling, ${lang} ${level}`, async ({ page }) => {
        // M4b fix of F2: riddle + food first, facts in their own scrolling part; the panel now
        // fits except portrait klasse2/klasse3, where the riddle alone needs a smaller font
        // than cap 3 % (shrinks to fit, ADIR-003 relaxed) — open until Q-070 is decided.
        await startGame(page, lang, level);
        await firstTouch(page);
        await openZebraBoard(page);
        const m = await panelMetrics(page);
        expect(m.cap).toBeGreaterThanOrEqual(0.03);
        expect(m.top).toBeGreaterThanOrEqual(0);
        expect(m.bottom).toBeLessThanOrEqual(m.h);
        expect(m.left).toBeGreaterThanOrEqual(0);
        expect(m.right).toBeLessThanOrEqual(m.w);
        expect(m.scroll, 'panel content must not need scrolling').toBeLessThanOrEqual(m.client + 1);
        // the riddle and the food word are inside the visible panel area
        for (const sel of ['#panel-text', '#panel-food']) {
          const r = await page.locator(sel).boundingBox();
          expect(r!.y + r!.height, sel).toBeLessThanOrEqual(m.bottom);
        }
      });
    }
  }
});

for (const [name, vp] of [
  ['portrait', { width: 412, height: 892 }],
  ['landscape', { width: 892, height: 412 }],
] as const) {
  test.describe(`PLAY-025 ${name} phone`, () => {
    test.use({ viewport: vp, ...PHONE });
    test('left thumb walks while the panel is open; the panel does not cover the player', async ({ page }) => {
      // QA F3/F4 fixed in M4b (PLAY-023…027 implemented).
      await startGame(page, 'de', 'klasse1');
      const cdp = await firstTouch(page);
      await openZebraBoard(page);
      // The player is drawn at the screen centre (follow camera); the panel must leave it free.
      const m = await panelMetrics(page);
      const cy = vp.height / 2;
      expect(m.bottom < cy - 40 || m.top > cy + 40, `panel ${m.top}–${m.bottom} covers the centre ${cy}`).toBe(true);
      // Left thumb: drag right (east, away from the board).
      const x0 = await page.evaluate(() => window.__zoo!.app.player_x());
      const s = { x: vp.width * 0.2, y: vp.height * 0.85, id: 7 };
      await touch(cdp, 'touchStart', [s]);
      await touch(cdp, 'touchMove', [{ ...s, x: s.x + 60 }]);
      await nextFrames(page, 20);
      const x1 = await page.evaluate(() => window.__zoo!.app.player_x());
      await touch(cdp, 'touchEnd', []);
      expect(x1 - x0).toBeGreaterThan(0.1);
    });
  });
}

test.describe('hippo and panda boards (in scope since M5a, Q-069)', () => {
  test.use({ viewport: { width: 1280, height: 720 } });
  for (const [animal, stand, key] of [
    ['hippo', [7.0, 21.5], 'KeyD'],
    ['panda', [-3.5, 28.8], 'KeyW'],
  ] as const) {
    test(`RESC-017: the ${animal} info board shows its riddle, food word and facts — no raw text key`, async ({ page }) => {
      await startGame(page, 'de', 'klasse1');
      await goto(page, stand[0], stand[1]);
      await turn(page, key);
      const target = await page.evaluate(() => window.__zoo!.app.target_key());
      expect(target).toBe(`info_board:${animal}`);
      await ensurePanel(page, async () => {
        await page.keyboard.press('KeyE');
        await nextFrames(page, 2);
      });
      const text = await textOf(page, '#panel');
      expect(text).not.toMatch(RAW_KEY);
      expect(text.length).toBeGreaterThan(40);
    });
  }
});
