// Controls on the target phone sizes and on desktop (GAME-PLAYER §3):
// - PLAY-015/016/017 on a 1080×2340 phone in portrait and landscape (the older touch tests
//   use 540×1170): joystick speed and direction, one swipe = one 45° step, walking while the
//   other thumb swipes and presses the interact button, pinch zoom in the right half.
// - Camera-relative keyboard directions after every 45° step (PLAY-011 wiring).
// - Speeds on path and grass (PLAY-005/006 in the browser, measured in game time).
import { expect, test, type CDPSession, type Page } from '@playwright/test';
import { goto, nextFrames } from '../helpers';
import { hold, startGame, teleport, touch, turn } from './qa';

test.describe.configure({ timeout: 180_000 });

async function app<T>(page: Page, fn: string): Promise<T> {
  return page.evaluate((f) => (window.__zoo!.app as unknown as Record<string, () => T>)[f](), fn);
}

for (const [name, vp] of [
  ['portrait', { width: 1080, height: 2340 }],
  ['landscape', { width: 2340, height: 1080 }],
] as const) {
  test.describe(`1080×2340 phone, ${name}`, () => {
    test.use({ viewport: vp, hasTouch: true, isMobile: true });

    let cdp: CDPSession;
    test.beforeEach(async ({ page }) => {
      await startGame(page, 'de', 'klasse1');
      cdp = await page.context().newCDPSession(page);
      await touch(cdp, 'touchStart', [{ x: vp.width * 0.75, y: vp.height * 0.3, id: 1 }]);
      await touch(cdp, 'touchEnd', []);
      await nextFrames(page, 2);
      await expect(page.locator('#stick')).toBeVisible();
    });

    test('PLAY-015: 60 % stick deflection = 60 % surface speed, away from the camera; release stops', async ({ page }) => {
      await teleport(page, 0.5, 2.5); // plaza (path)
      const s = { x: vp.width * 0.2, y: vp.height * 0.8, id: 2 };
      const z0 = await app<number>(page, 'player_z');
      await touch(cdp, 'touchStart', [s]);
      await touch(cdp, 'touchMove', [{ ...s, y: s.y - 36 }]);
      await nextFrames(page, 6);
      const speed = await app<number>(page, 'player_speed');
      const surface = await app<number>(page, 'surface_speed');
      expect(Math.abs(speed - 0.6 * surface)).toBeLessThanOrEqual(0.05 * 0.6 * surface);
      expect(await app<number>(page, 'player_z')).toBeGreaterThan(z0);
      await touch(cdp, 'touchEnd', []);
      await nextFrames(page, 2);
      expect(await app<number>(page, 'player_speed')).toBe(0);
    });

    test('PLAY-016/017: swipe rotates one step while the left thumb walks; interact button works meanwhile', async ({ page }) => {
      await goto(page, -6.0, 15.5); // east of the zebra board, on the ring path
      await turn(page, 'KeyA');
      const yaw0 = await app<number>(page, 'camera_target_yaw_deg');
      const s = { x: vp.width * 0.2, y: vp.height * 0.8, id: 3 };
      await touch(cdp, 'touchStart', [s]);
      await touch(cdp, 'touchMove', [{ ...s, x: s.x - 60 }]); // walk west, towards the board
      await nextFrames(page, 3);
      const held = { ...s, x: s.x - 60 };
      await touch(cdp, 'touchStart', [held, { x: vp.width * 0.7, y: vp.height * 0.4, id: 4 }]);
      for (const dx of [20, 50, 90, 140]) {
        await touch(cdp, 'touchMove', [held, { x: vp.width * 0.7 + dx, y: vp.height * 0.4, id: 4 }]);
      }
      await touch(cdp, 'touchEnd', [{ x: vp.width * 0.7 + 140, y: vp.height * 0.4, id: 4 }]);
      expect(Math.abs((await app<number>(page, 'camera_target_yaw_deg')) - yaw0)).toBeCloseTo(45, 3);
      expect(await app<number>(page, 'player_speed')).toBeGreaterThan(0.3);
      // lift, rotate back (snapped), then push west into the board until it is in reach
      await touch(cdp, 'touchEnd', []);
      await page.evaluate(() => {
        window.__zoo!.app.rotate(-1);
        window.__zoo!.app.debug_teleport(-6.0, 15.5); // the walk drifted while the camera turned
        window.__zoo!.app.debug_step(0);
      });
      await touch(cdp, 'touchStart', [s]);
      await touch(cdp, 'touchMove', [held]);
      await page.evaluate(() => window.__zoo!.app.debug_step(1.5));
      await nextFrames(page, 3);
      expect(await app<string>(page, 'target_key')).toBe('info_board:zebra');
      const act = (await page.locator('#act').boundingBox())!;
      expect(act.width).toBeGreaterThanOrEqual(64);
      expect(act.x + act.width).toBeLessThanOrEqual(vp.width);
      expect(act.y + act.height).toBeLessThanOrEqual(vp.height);
      const tap = { x: act.x + act.width / 2, y: act.y + act.height / 2, id: 5 };
      await touch(cdp, 'touchStart', [held, tap]);
      await touch(cdp, 'touchEnd', [tap]);
      await nextFrames(page, 3);
      await expect(page.locator('#panel')).toBeVisible();
      await touch(cdp, 'touchEnd', []);
    });

    test('pinch with two fingers in the right half zooms in and out within 10–20 m', async ({ page }) => {
      const pinch = async (from: number, to: number) => {
        const cx = vp.width * 0.75;
        const cy = vp.height * 0.5;
        await touch(cdp, 'touchStart', [
          { x: cx - from / 2, y: cy, id: 10 },
          { x: cx + from / 2, y: cy, id: 11 },
        ]);
        for (let i = 1; i <= 8; i++) {
          const d = from + ((to - from) * i) / 8;
          await touch(cdp, 'touchMove', [
            { x: cx - d / 2, y: cy, id: 10 },
            { x: cx + d / 2, y: cy, id: 11 },
          ]);
        }
        await touch(cdp, 'touchEnd', []);
        await page.evaluate(() => window.__zoo!.app.debug_step(0));
        return app<number>(page, 'camera_distance');
      };
      const start = await app<number>(page, 'camera_distance');
      expect(start).toBeCloseTo(20, 1); // LAYOUT-L1-011: level starts zoomed out
      const zin = await pinch(80, 400);
      expect(zin).toBeLessThan(start);
      expect(zin).toBeGreaterThanOrEqual(10);
      const zmin = await pinch(60, 600);
      expect(zmin).toBeCloseTo(10, 3);
      const zout = await pinch(500, 50);
      expect(zout).toBeCloseTo(20, 3);
    });
  });
}

test.describe('desktop 1920×1080', () => {
  test.use({ viewport: { width: 1920, height: 1080 } });

  test('camera-relative keys after every 45° step; path vs grass speed (game time)', async ({ page }) => {
    await startGame(page, 'de', 'klasse1');
    for (let k = 0; k < 8; k++) {
      const yaw = (await app<number>(page, 'camera_target_yaw_deg')) * (Math.PI / 180);
      // expected level direction (x east, z north) of W and D for this yaw (camera.rs)
      const fwd = [-Math.sin(yaw), Math.cos(yaw)];
      const right = [Math.cos(yaw), Math.sin(yaw)];
      for (const [code, want] of [
        ['KeyW', fwd],
        ['KeyD', right],
        ['KeyS', [-fwd[0], -fwd[1]]],
        ['KeyA', [-right[0], -right[1]]],
      ] as const) {
        // open grass (moved from -18, 3.5: the new bamboo_sw thicket blocks walking south there)
        await teleport(page, -16.0, 4.5);
        const end = await hold(page, [code], 0.5);
        const d = [end.x + 16.0, end.z - 4.5];
        const len = Math.hypot(d[0], d[1]);
        expect(len).toBeGreaterThan(0.3);
        expect((d[0] * want[0] + d[1] * want[1]) / len, `${code} at step ${k}`).toBeGreaterThan(0.999);
      }
      await page.keyboard.press('KeyR');
    }
    // PLAY-005/006: 1 s on the ring path vs 1 s on grass (speed = surface speed ± 5 %)
    await teleport(page, -7.5, 9.5);
    await hold(page, [], 0.5);
    await page.keyboard.press('KeyQ'); // back to 0°
    for (let i = 0; i < 7; i++) await page.keyboard.press('KeyQ');
    await page.evaluate(() => window.__zoo!.app.debug_step(0));
    const p0 = await hold(page, ['KeyD'], 1.0);
    const pathSpeed = p0.x + 7.5;
    await teleport(page, -21.0, 3.5);
    await hold(page, [], 0.5);
    const g0 = await hold(page, ['KeyD'], 1.0);
    const grassSpeed = g0.x + 21.0;
    expect(pathSpeed).toBeGreaterThan(grassSpeed * 1.2);
    expect(Math.abs(grassSpeed - 0.98)).toBeLessThanOrEqual(0.98 * 0.05 + 1 / 60); // both speed decisions: 0.98 m/s
  });
});
