// LAYOUT-032 (e2e part, QA 2026-09-27 "no door blocked"): the player walks through every
// walkable opening of the joined zoo in both directions with the real keyboard input path
// (desktop) — the doors of the enterable buildings, the garden gate, the moon door at night —
// and once with the touch joystick on a phone; closed enclosure gates stay solid when she is
// not leading an animal (GAME-LAYOUT "Gates and doors"), and she walks right up to every food
// storage door through the gap in the food-box row (Q-150 answered 2026-09-27). Review shots of the tight spots found
// in the QA run go to qa/reports/img/ (report qa/reports/2026-09-27-doors-blocked.md).
import path from 'node:path';
import { expect, test, type Page } from '@playwright/test';
import { nextFrames, repo } from '../helpers';
import { hold, startGame, teleport, touch } from './qa';

test.describe.configure({ timeout: 180_000 });

const img = path.join(repo, 'qa/reports/img');

/** A walkable opening: centre and the unit normal pointing outside (level x, z). */
interface Door {
  name: string;
  x: number;
  z: number;
  nx: number;
  nz: number;
  night?: boolean;
}

const DOORS: Door[] = [
  { name: 'zookeeper_house_1', x: -8.5, z: 2.5, nx: 1, nz: 0 },
  { name: 'zookeeper_house_3', x: -7.5, z: 61.5, nx: 0, nz: -1 },
  { name: 'night_house', x: -36.5, z: 39.5, nx: 0, nz: -1 },
  { name: 'garden_veg', x: 8, z: 36, nx: 0, nz: -1 },
  { name: 'moon_door', x: -23, z: 30, nx: 1, nz: 0, night: true },
];

/** The WASD key that walks along a level direction for the current camera yaw (camera.rs). */
async function keyFor(page: Page, dx: number, dz: number): Promise<string> {
  const yaw = (await page.evaluate(() => window.__zoo!.app.camera_target_yaw_deg())) * (Math.PI / 180);
  const fwd = [-Math.sin(yaw), Math.cos(yaw)];
  const right = [Math.cos(yaw), Math.sin(yaw)];
  const keys: [string, number[]][] = [
    ['KeyW', fwd],
    ['KeyS', [-fwd[0], -fwd[1]]],
    ['KeyD', right],
    ['KeyA', [-right[0], -right[1]]],
  ];
  keys.sort((a, b) => b[1][0] * dx + b[1][1] * dz - (a[1][0] * dx + a[1][1] * dz));
  return keys[0][0];
}

/** Signed distance of the player from the opening along its outside normal. */
async function depth(page: Page, d: Door): Promise<number> {
  const p = await page.evaluate(() => [window.__zoo!.app.player_x(), window.__zoo!.app.player_z()]);
  return (p[0] - d.x) * d.nx + (p[1] - d.z) * d.nz;
}

test.describe('desktop 1920×1080', () => {
  test.use({ viewport: { width: 1920, height: 1080 } });

  test('LAYOUT-032: walk through every walkable door / gate in both directions (keyboard)', async ({ page }) => {
    const errors = await startGame(page, 'de', 'klasse1');
    for (const d of DOORS) {
      if (d.night) expect(await page.evaluate(() => window.__zoo!.app.debug_set_daytime('night'))).toBe(true);
      // outside → inside
      await teleport(page, d.x + d.nx * 2.5, d.z + d.nz * 2.5);
      await hold(page, [await keyFor(page, -d.nx, -d.nz)], 5.0);
      expect(await depth(page, d), `${d.name}: walked in`).toBeLessThan(-1.5);
      // inside → outside
      await teleport(page, d.x - d.nx * 1.8, d.z - d.nz * 1.8);
      await hold(page, [await keyFor(page, d.nx, d.nz)], 5.0);
      expect(await depth(page, d), `${d.name}: walked out`).toBeGreaterThan(1.5);
    }
    expect(errors).toEqual([]);
  });

  test('LAYOUT-032 / Q-150: she walks right up to every food storage door (gap in the box row)', async ({ page }) => {
    await startGame(page, 'de', 'klasse1');
    // storage doors: centre of the door on the facade, outward normal (not enterable: she stops
    // at the closed door, ≤ player radius + 0.1 m from the facade)
    for (const [name, x, z, nx, nz] of [
      ['food_storage', 0.5, 11.0, 0, -1],
      ['food_storage_2', 39.0, 30.5, -1, 0],
      ['food_storage_3', 3.5, 61.0, 0, -1],
      ['food_storage_n1', -39.0, 29.5, 1, 0],
    ] as const) {
      await teleport(page, x + nx * 2.5, z + nz * 2.5);
      const end = await hold(page, [await keyFor(page, -nx, -nz)], 3.0);
      const gap = (end.x - x) * nx + (end.z - z) * nz;
      expect(gap, `${name}: distance to the door`).toBeLessThan(0.27 + 0.1);
    }
  });

  test('LAYOUT-032: closed enclosure gates stay solid when not leading', async ({ page }) => {
    await startGame(page, 'de', 'klasse1');
    // zebra gate (fence x = −9, gate z 12…14), from the path side
    await teleport(page, -7.5, 13.0);
    await hold(page, [await keyFor(page, -1, 0)], 3.0);
    const x = await page.evaluate(() => window.__zoo!.app.player_x());
    expect(x, 'stops in front of the closed gate').toBeGreaterThan(-9.0);
  });
});

test.describe('review shots 1280×720', () => {
  test.use({ viewport: { width: 1280, height: 720 } });

  test('QA review shots: tight spots beside doors and gates', async ({ page }) => {
    await startGame(page, 'de', 'klasse1');
    const shot = async (name: string, x: number, z: number) => {
      await teleport(page, x, z);
      await nextFrames(page, 4);
      await page.screenshot({ path: path.join(img, `2026-09-27-doors-${name}.jpg`), type: 'jpeg', quality: 50, scale: 'css' });
    };
    // zebra: the info board stands 1 m north of the gate post (Q-157 answered); walking
    // south-west from the path north of it (towards the gate)
    await teleport(page, -7.5, 17.0);
    await hold(page, [await keyFor(page, 0, -1), await keyFor(page, -1, 0)], 3.0);
    await shot('zebra-board', await page.evaluate(() => window.__zoo!.app.player_x()), await page.evaluate(() => window.__zoo!.app.player_z()));
    // zookeeper house 3: the tap flush on the facade, 1.5 m beside the door (Q-157)
    await shot('zh3-tap', -5.5, 60.0);
    // night house: the boards hang flat on the facade (Q-157, `mount = "wall"`)
    await shot('nighthouse-board', -39.3, 38.4);
    // food storage (Q-150 answered): the box row leaves a gap in front of the door
    await shot('storage-gap', 0.5, 10.6);
    // night: she walks into a lantern post (no collider yet, LAYOUT-035 / F4)
    expect(await page.evaluate(() => window.__zoo!.app.debug_set_daytime('night'))).toBe(true);
    await teleport(page, 52.2, 24.25);
    const end = await hold(page, [await keyFor(page, 1, 0)], 0.45);
    await shot('lamp-post-night', end.x, end.z);
  });
});

test.describe('phone 1080×2340 portrait (touch)', () => {
  test.use({ viewport: { width: 1080, height: 2340 }, hasTouch: true, isMobile: true });

  test('LAYOUT-032: the joystick walks through the zookeeper house door and back', async ({ page }) => {
    await startGame(page, 'de', 'klasse1');
    const cdp = await page.context().newCDPSession(page);
    await touch(cdp, 'touchStart', [{ x: 810, y: 700, id: 1 }]);
    await touch(cdp, 'touchEnd', []);
    await nextFrames(page, 2);
    await expect(page.locator('#stick')).toBeVisible();
    const d = DOORS[0];
    const yaw = (await page.evaluate(() => window.__zoo!.app.camera_target_yaw_deg())) * (Math.PI / 180);
    // screen offset (right, up) of a level direction for this yaw (camera.rs: W = fwd, D = right)
    const stick = (dx: number, dz: number) => {
      const fwd = [-Math.sin(yaw), Math.cos(yaw)];
      const right = [Math.cos(yaw), Math.sin(yaw)];
      return { r: dx * right[0] + dz * right[1], u: dx * fwd[0] + dz * fwd[1] };
    };
    for (const [from, dir] of [
      [2.5, -1],
      [-1.8, 1],
    ] as const) {
      await teleport(page, d.x + d.nx * from, d.z + d.nz * from);
      const s = stick(d.nx * dir, d.nz * dir);
      const p0 = { x: 216, y: 1870, id: 2 };
      await touch(cdp, 'touchStart', [p0]);
      await touch(cdp, 'touchMove', [{ ...p0, x: p0.x + s.r * 60, y: p0.y - s.u * 60 }]);
      await page.evaluate(() => window.__zoo!.app.debug_step(5.0));
      await touch(cdp, 'touchEnd', []);
      await nextFrames(page, 2);
      const dep = await depth(page, d);
      if (dir < 0) expect(dep, 'walked in by touch').toBeLessThan(-1.5);
      else expect(dep, 'walked out by touch').toBeGreaterThan(1.5);
    }
  });
});
