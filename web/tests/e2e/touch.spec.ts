// GAME-PLAYER §3 two-thumb touch controls (PLAY-013…018) and §5/§7 interaction + collision
// in the browser. Touches are real CDP touch events (distinct touch ids → distinct pointer
// ids), so the browser's own pointer/touch pipeline is exercised.
import { expect, test, type CDPSession, type Page } from '@playwright/test';
import { face, goto, nextFrames, state, waitFrames, START_URL } from './helpers';

type Pt = { x: number; y: number; id: number };
// CDP: touchStart/touchMove list the active points; touchEnd lists the points that end
// (an empty list ends all).

async function touch(cdp: CDPSession, type: 'touchStart' | 'touchMove' | 'touchEnd', points: Pt[]) {
  await cdp.send('Input.dispatchTouchEvent', { type, touchPoints: points });
}

async function open(page: Page) {
  const errors: string[] = [];
  page.on('pageerror', (e) => errors.push(String(e)));
  page.on('console', (m) => {
    if (m.type() === 'error') errors.push(m.text());
  });
  await page.goto(START_URL);
  await waitFrames(page, 3);
  return errors;
}

async function camYaw(page: Page) {
  return page.evaluate(() => window.__zoo!.app.camera_target_yaw_deg());
}

test.describe('no touch device', () => {
  test.use({ viewport: { width: 1280, height: 720 } });

  test('PLAY-013: no touch controls without touch; key hint instead', async ({ page }) => {
    const errors = await open(page);
    await page.mouse.move(300, 300);
    await page.mouse.down();
    await page.mouse.move(420, 300);
    await page.mouse.up();
    await goto(page, -7.5, 15.5);
    await face(page, 'KeyA');
    expect((await state(page)).target).toBe('info_board:zebra');
    await expect(page.locator('#stick')).toBeHidden();
    await expect(page.locator('#act')).toBeHidden();
    await expect(page.locator('#hint')).toBeVisible();
    expect(await page.evaluate(() => document.body.classList.contains('touch'))).toBe(false);
    expect(errors).toEqual([]);
  });

  test('PLAY-019/020 in the browser: the zebra board blocks, hint only in front of it', async ({ page }) => {
    await open(page);
    // In front (east) of the board facing west: hint shown.
    await goto(page, -7.0, 15.5);
    await page.keyboard.down('KeyA'); // walk straight into the board for 2 s
    await page.waitForTimeout(2000);
    await page.keyboard.up('KeyA');
    await nextFrames(page, 2);
    const s = await state(page);
    expect(s.x).toBeGreaterThan(-8.0 + 0.3 - 0.02); // stopped in front of the board cell
    expect(s.target).toBe('info_board:zebra');
    await expect(page.locator('#hint')).toBeVisible();
    // Beside the board (north of it, looking south at it): no hint.
    await page.evaluate(() => window.__zoo!.app.debug_teleport(-8.5, 17.2));
    await face(page, 'KeyS');
    expect((await state(page)).target).toBe('');
    await expect(page.locator('#hint')).toBeHidden();
    // In front but facing away: no hint.
    await page.evaluate(() => window.__zoo!.app.debug_teleport(-7.3, 15.5));
    await face(page, 'KeyD');
    expect((await state(page)).target).toBe('');
    await expect(page.locator('#hint')).toBeHidden();
  });
});

test.describe('touch device', () => {
  test.use({ viewport: { width: 540, height: 1170 }, hasTouch: true, isMobile: true });

  test('PLAY-014 / PLAY-018: first touch shows the controls; no scroll, zoom or selection', async ({ page }) => {
    const errors = await open(page);
    await expect(page.locator('#stick')).toBeHidden();
    const cdp = await page.context().newCDPSession(page);
    await touch(cdp, 'touchStart', [{ x: 400, y: 500, id: 1 }]);
    await touch(cdp, 'touchEnd', []);
    await nextFrames(page);
    await expect(page.locator('#stick')).toBeVisible();
    // browser gestures: a two-finger spread and a long vertical swipe
    await touch(cdp, 'touchStart', [
      { x: 350, y: 500, id: 2 },
      { x: 450, y: 500, id: 3 },
    ]);
    for (let i = 1; i <= 6; i++) {
      await touch(cdp, 'touchMove', [
        { x: 350 - i * 20, y: 500 - i * 30, id: 2 },
        { x: 450 + i * 15, y: 500 + i * 30, id: 3 },
      ]);
    }
    await touch(cdp, 'touchEnd', []);
    await touch(cdp, 'touchStart', [{ x: 100, y: 300, id: 4 }]);
    await touch(cdp, 'touchMove', [{ x: 100, y: 900, id: 4 }]);
    await touch(cdp, 'touchEnd', []);
    await page.waitForTimeout(500);
    const env = await page.evaluate(() => ({
      scrollY: window.scrollY,
      scrollX: window.scrollX,
      scale: window.visualViewport?.scale ?? 1,
      selection: String(window.getSelection() ?? ''),
      touchAction: [document.documentElement, document.body, document.getElementById('game')!].map(
        (e) => getComputedStyle(e).touchAction,
      ),
      viewport: document.querySelector('meta[name=viewport]')!.getAttribute('content'),
    }));
    expect(env.scrollX).toBe(0);
    expect(env.scrollY).toBe(0);
    expect(env.scale).toBe(1);
    expect(env.selection).toBe('');
    expect(env.touchAction).toEqual(['none', 'none', 'none']);
    expect(env.viewport).toContain('user-scalable=no');
    await expect(page.locator('#stick')).toBeVisible(); // stays visible (PLAY-014)
    // interact button: ≥ 64 px, bottom-right inside the viewport
    await goto(page, -7.5, 15.5);
    await face(page, 'KeyA');
    const box = await page.locator('#act').boundingBox();
    expect(box).not.toBeNull();
    expect(box!.width).toBeGreaterThanOrEqual(64);
    expect(box!.x + box!.width).toBeLessThanOrEqual(540);
    expect(box!.x).toBeGreaterThan(270);
    expect(box!.y + box!.height).toBeLessThanOrEqual(1170);
    expect(errors).toEqual([]);
  });

  test('PLAY-015: left thumb 60 % up walks away from the camera at 60 % speed; release stops', async ({ page }) => {
    await open(page);
    const cdp = await page.context().newCDPSession(page);
    const radius = 60;
    await touch(cdp, 'touchStart', [{ x: 130, y: 900, id: 1 }]);
    await touch(cdp, 'touchMove', [{ x: 130, y: 900 - 0.6 * radius, id: 1 }]);
    const z0 = (await state(page)).z;
    await nextFrames(page, 6);
    const m = await page.evaluate(() => ({
      speed: window.__zoo!.app.player_speed(),
      surface: window.__zoo!.app.surface_speed(),
    }));
    expect(Math.abs(m.speed - 0.6 * m.surface)).toBeLessThanOrEqual(0.05 * 0.6 * m.surface);
    const s = await state(page);
    expect(s.z).toBeGreaterThan(z0); // north = away from the default camera
    await touch(cdp, 'touchEnd', []);
    await nextFrames(page, 3);
    expect(await page.evaluate(() => window.__zoo!.app.player_speed())).toBe(0);
  });

  test('PLAY-016: one swipe ≥ 40 px in the right half rotates exactly one 45° step', async ({ page }) => {
    await open(page);
    const cdp = await page.context().newCDPSession(page);
    const y0 = await camYaw(page);
    // 30 px: nothing
    await touch(cdp, 'touchStart', [{ x: 400, y: 600, id: 1 }]);
    await touch(cdp, 'touchMove', [{ x: 430, y: 600, id: 1 }]);
    await touch(cdp, 'touchEnd', []);
    expect(await camYaw(page)).toBeCloseTo(y0, 3);
    // long swipe right (in several moves): exactly one step
    await touch(cdp, 'touchStart', [{ x: 300, y: 600, id: 2 }]);
    for (const x of [320, 345, 380, 450, 520]) await touch(cdp, 'touchMove', [{ x, y: 600, id: 2 }]);
    await touch(cdp, 'touchEnd', []);
    const y1 = await camYaw(page);
    expect(Math.abs(y1 - y0)).toBeCloseTo(45, 3);
    // swipe left: back
    await touch(cdp, 'touchStart', [{ x: 500, y: 600, id: 3 }]);
    await touch(cdp, 'touchMove', [{ x: 440, y: 600, id: 3 }]);
    await touch(cdp, 'touchEnd', []);
    expect(await camYaw(page)).toBeCloseTo(y0, 3);
  });

  test('PLAY-017: walking with the left thumb while the right thumb swipes', async ({ page }) => {
    await open(page);
    const cdp = await page.context().newCDPSession(page);
    const y0 = await camYaw(page);
    const left = { x: 130, y: 900, id: 1 };
    await touch(cdp, 'touchStart', [left]);
    await touch(cdp, 'touchMove', [{ ...left, y: 840 }]);
    await nextFrames(page, 3);
    await touch(cdp, 'touchStart', [{ ...left, y: 840 }, { x: 350, y: 500, id: 2 }]);
    await touch(cdp, 'touchMove', [{ ...left, y: 840 }, { x: 420, y: 500, id: 2 }]);
    await nextFrames(page, 3);
    expect(await page.evaluate(() => window.__zoo!.app.player_speed())).toBeGreaterThan(0.5);
    expect(Math.abs((await camYaw(page)) - y0)).toBeCloseTo(45, 3);
    await touch(cdp, 'touchEnd', [{ x: 420, y: 500, id: 2 }]); // right thumb lifts (CDP lists the ended point), left keeps walking
    await nextFrames(page, 3);
    expect(await page.evaluate(() => window.__zoo!.app.player_speed())).toBeGreaterThan(0.5);
    await touch(cdp, 'touchEnd', []);
    await nextFrames(page, 3);
    expect(await page.evaluate(() => window.__zoo!.app.player_speed())).toBe(0);
  });
});
