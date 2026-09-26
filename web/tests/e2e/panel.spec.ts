// PLAY-023 / PLAY-025 (GAME-PLAYER §4): reading panels open by themselves when the player
// stands in front of an info board and faces it, close by themselves when she walks away,
// never block walking and never cover the player on screen.
import { expect, test, type Page } from '@playwright/test';
import { face, goto, nextFrames, waitFrames, START_URL } from './helpers';

test.describe.configure({ timeout: 180_000 });

async function start(page: Page, level = 'klasse1') {
  await page.addInitScript((r) => {
    if (sessionStorage.getItem('zoo.e2e.init')) return;
    sessionStorage.setItem('zoo.e2e.init', '1');
    localStorage.clear();
    localStorage.setItem('zoo.readingLevel', r);
  }, level);
  const errors: string[] = [];
  page.on('console', (m) => {
    if (m.type() === 'error') errors.push(m.text());
  });
  page.on('pageerror', (e) => errors.push(String(e)));
  await page.goto(START_URL);
  await waitFrames(page, 3);
  return errors;
}

const BOARD = { x: -8.5, z: 14.5 }; // zebra info board, readable side east

const panelVisible = (page: Page) => page.evaluate(() => !document.getElementById('panel')!.hidden);

test.describe('landscape', () => {
  test.use({ viewport: { width: 1280, height: 720 } });

  test('PLAY-023: the board panel opens by itself after 0.25–0.5 s and closes when walking away', async ({ page }) => {
    const errors = await start(page);
    // 1.6 m east of the board, facing north (board to the side: not available)
    await goto(page, -6.9, 11.0);
    await goto(page, -6.9, 14.5);
    await page.evaluate(() => {
      const a = window.__zoo!.app;
      a.key('KeyW', true); // face north (one step), board to the side
      a.debug_step(1 / 60);
      a.key('KeyW', false);
      a.debug_teleport(-6.9, 14.5);
      a.debug_step(0.6); // an open panel closes (turned away)
    });
    await nextFrames(page, 2);
    await expect(page.locator('#panel')).toBeHidden();
    expect(await panelVisible(page)).toBe(false);
    // turn towards the board: available from now on
    await page.keyboard.down('KeyA');
    await page.waitForFunction(() => window.__zoo!.app.target_key() === 'info_board:zebra', null, { polling: 'raf' });
    await page.keyboard.up('KeyA');
    const t0 = await page.evaluate(() => window.__zoo!.app.time());
    await page.waitForFunction(() => !document.getElementById('panel')!.hidden, null, { polling: 'raf', timeout: 10_000 });
    const t1 = await page.evaluate(() => window.__zoo!.app.time());
    console.log(`panel opened after ${(t1 - t0).toFixed(2)} s of game time`);
    expect(t1 - t0).toBeGreaterThanOrEqual(0.2);
    expect(t1 - t0).toBeLessThanOrEqual(0.6);
    await expect(page.locator('#panel-text')).toBeVisible();

    // stepping away (still facing it): 2.3 m keeps it open (hysteresis), > 2.5 m closes it
    // within 0.5 s of game time
    await page.evaluate(([bx, bz]) => window.__zoo!.app.debug_teleport(bx + 2.3, bz), [BOARD.x, BOARD.z]);
    await page.evaluate(() => window.__zoo!.app.debug_step(1.0));
    await nextFrames(page, 2);
    await expect(page.locator('#panel')).toBeVisible();
    await page.evaluate(([bx, bz]) => window.__zoo!.app.debug_teleport(bx + 3.0, bz), [BOARD.x, BOARD.z]);
    await page.evaluate(() => window.__zoo!.app.debug_step(0.5));
    await nextFrames(page, 2);
    await expect(page.locator('#panel')).toBeHidden();

    // turning away closes it as well
    await goto(page, -7.0, 14.5);
    await face(page, 'KeyA');
    await expect(page.locator('#panel')).toBeVisible();
    await face(page, 'KeyD');
    await expect(page.locator('#panel')).toBeHidden({ timeout: 3000 });
    expect(errors).toEqual([]);
  });
});

async function checkNotCovered(page: Page) {
  const player = await page.evaluate(() => window.__zoo!.app.player_screen_rect());
  const box = await page.locator('#panel .panel-body').boundingBox();
  expect(box).not.toBeNull();
  expect(player.length).toBe(4);
  // the panel ends above the player's head
  expect(box!.y + box!.height, `panel bottom ${box!.y + box!.height} vs player top ${player[1]}`).toBeLessThanOrEqual(player[1]);
}

for (const [name, viewport] of [
  ['landscape', { width: 1280, height: 720 }],
  ['portrait', { width: 540, height: 1170 }],
] as const) {
  test.describe(name, () => {
    test.use({ viewport });

    test(`PLAY-025 (${name}): walking works with the panel open, the panel never covers the player`, async ({ page }) => {
      const errors = await start(page, 'klasse3'); // longest texts
      for (const zoom of [1, 14 / 20, 10 / 14]) {
        await page.evaluate((z) => {
          window.__zoo!.app.zoom(z);
          window.__zoo!.app.debug_step(0.02);
        }, zoom);
        await goto(page, -7.0, 14.5);
        await face(page, 'KeyA');
        await expect(page.locator('#panel-facts')).toBeVisible();
        await checkNotCovered(page);
      }
      // keyboard movement still moves the player while the panel is open
      const before = await page.evaluate(() => window.__zoo!.app.player_x());
      await page.keyboard.down('KeyD');
      await page.waitForFunction((x0) => window.__zoo!.app.player_x() > x0 + 0.1, before, { timeout: 5000 });
      await page.keyboard.up('KeyD');
      // the panel does not take pointer input outside its box: the canvas gets it
      const hit = await page.evaluate(() => {
        const el = document.elementFromPoint(window.innerWidth * 0.25, window.innerHeight * 0.75);
        return el?.id ?? '';
      });
      expect(hit).toBe('game');
      expect(errors).toEqual([]);
    });
  });
}

// QA F2 (GAME-ANIMALS "Info board" 4, until Q-070): on phones the riddle and the food word are
// always fully visible without scrolling; only the facts part may scroll.
for (const [name, viewport] of [
  ['portrait', { width: 412, height: 892 }],
  ['landscape', { width: 892, height: 412 }],
] as const) {
  test.describe(`phone ${name}`, () => {
    test.use({ viewport });
    test(`riddle + food word visible without scrolling at every level (${name})`, async ({ page }) => {
      const errors = await start(page, 'klasse1');
      for (const lang of ['de', 'en']) {
        for (const level of ['kiga', 'klasse1', 'klasse2', 'klasse3']) {
          await page.evaluate(
            ([l, r]) => {
              window.__zoo!.app.set_language(l);
              window.__zoo!.app.set_reading_level(r);
            },
            [lang, level],
          );
          await goto(page, -6.0, 14.5); // away from the board: panel closes
          await expect(page.locator('#panel')).toBeHidden();
          await goto(page, -7.0, 14.5);
          await face(page, 'KeyA');
          await expect(page.locator('#panel-facts')).toBeAttached();
          const m = await page.evaluate(() => {
            const body = document.querySelector('#panel .panel-body') as HTMLElement;
            const b = body.getBoundingClientRect();
            const main = document.getElementById('panel-main')!;
            const r = (id: string) => document.getElementById(id)!.getBoundingClientRect();
            return {
              bodyBottom: b.bottom - 6,
              bodyScroll: body.scrollHeight - body.clientHeight,
              mainOverflow: main.scrollHeight - main.clientHeight,
              text: r('panel-text').bottom,
              food: r('panel-food').bottom,
              more: document.getElementById('panel-more')!.scrollHeight > 0,
            };
          });
          const tag = `${lang} ${level}`;
          expect(m.bodyScroll, tag).toBeLessThanOrEqual(1);
          expect(m.mainOverflow, tag).toBeLessThanOrEqual(1);
          expect(m.text, `riddle ${tag}`).toBeLessThanOrEqual(m.bodyBottom + 1);
          expect(m.food, `food ${tag}`).toBeLessThanOrEqual(m.bodyBottom + 1);
          expect(m.more, tag).toBe(true);
        }
      }
      expect(errors).toEqual([]);
    });
  });
}
