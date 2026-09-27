// GAME-FEED §8–17 in the browser: putting the carried food down with `G` and the put-down
// button, picking it up again (FEED-009), no button / no effect with empty hands (FEED-016),
// cutting bamboo at a cut spot of the bamboo forest with `E` and the touch action button
// (FEED-017) with review shots of the stalk stages, and the level-3 water wheel turning in the
// stream (LAYOUT-L3-018). Review shots: art/environment/poc/screenshot_feed_*.png,
// screenshot_bamboo_*.png, screenshot_water_wheel_*.png.
import path from 'node:path';
import { expect, test, type Page } from '@playwright/test';
import { goto, nextFrames, shots, START_URL, waitFrames } from './helpers';

test.describe.configure({ timeout: 480_000 });
test.use({ viewport: { width: 1280, height: 720 } });

type Lying = { foods: { uid: number; food: string; x: number; z: number; y: number }[]; bowl: boolean };
type Spot = { id: string; x: number; z: number; sx: number; sz: number; stage: string; regrow_s: number };

async function start(page: Page): Promise<string[]> {
  const errors: string[] = [];
  page.on('pageerror', (e) => errors.push(String(e)));
  page.on('console', (m) => {
    if (m.type() === 'error') errors.push(m.text());
  });
  await page.goto(START_URL);
  await waitFrames(page, 3);
  return errors;
}

async function lying(page: Page): Promise<Lying> {
  return JSON.parse(await page.evaluate(() => window.__zoo!.app.lying_json())) as Lying;
}

async function spots(page: Page): Promise<Spot[]> {
  return JSON.parse(await page.evaluate(() => window.__zoo!.app.cut_spots_json())) as Spot[];
}

async function carry(page: Page): Promise<string> {
  return page.evaluate(() => window.__zoo!.app.carry_food());
}

/** Takes a food at its box in the level-1 food storage row (GAME-FEED §6). */
async function takeFood(page: Page, food: string, x: number) {
  await goto(page, x, 9.5);
  expect(await page.evaluate((f) => window.__zoo!.app.take_food(f), food)).toBe(true);
  await nextFrames(page, 2);
}

/** Stands on the open entrance plaza facing north. */
async function onPlaza(page: Page) {
  await goto(page, 2.5, 4.5);
  await page.evaluate(() => window.__zoo!.app.debug_face_point(2.5, 8.0));
  await nextFrames(page, 2);
}

// FEED-009 (desktop G + the touch put-down button) and FEED-016 (empty hands)
test('FEED-009 / FEED-016: hay is put down with G and the put-down button, and picked up again', async ({ page }) => {
  const errors = await start(page);
  const drop = page.locator('#drop-btn');
  // FEED-016: nothing carried → no button, G does nothing
  await expect(drop).toBeHidden();
  await onPlaza(page);
  await page.keyboard.press('KeyG');
  await nextFrames(page, 2);
  expect((await lying(page)).foods).toEqual([]);
  await expect(drop).toBeHidden();

  // FEED-009 desktop: G puts the hay down ≈ 0.8 m in front of her, on the surface
  await takeFood(page, 'hay', -2.8);
  await onPlaza(page);
  await expect(drop).toBeVisible();
  const box = await drop.boundingBox();
  expect(box!.width).toBeGreaterThanOrEqual(64);
  expect(box!.height).toBeGreaterThanOrEqual(64);
  await page.keyboard.press('KeyG');
  await nextFrames(page, 2);
  let l = await lying(page);
  expect(l.foods.map((f) => f.food)).toEqual(['hay']);
  const p = await page.evaluate(() => [window.__zoo!.app.player_x(), window.__zoo!.app.player_z()]);
  const d = Math.hypot(l.foods[0].x - p[0], l.foods[0].z - p[1]);
  expect(d).toBeGreaterThan(0.6);
  expect(d).toBeLessThan(1.0);
  const g = await page.evaluate(([x, z]) => window.__zoo!.app.ground_height(x, z), [l.foods[0].x, l.foods[0].z]);
  expect(l.foods[0].y).toBeCloseTo(g, 3);
  expect(await carry(page)).toBe('');
  await expect(drop).toBeHidden();
  await expect(page.locator('#hud-carry')).toBeHidden();
  // the readable icon above it
  await expect(page.locator('#lying-icons .lying-icon[data-food="hay"]')).toBeVisible();
  await page.screenshot({ path: path.join(shots, 'screenshot_feed_drop.png') });

  // pick it up again with E (the interact target is the lying hay)
  expect(await page.evaluate(() => window.__zoo!.app.target_key())).toMatch(/^lying_food:hay:/);
  await page.keyboard.press('KeyE');
  await nextFrames(page, 2);
  expect(await carry(page)).toBe('hay');
  expect((await lying(page)).foods).toEqual([]);

  // FEED-009 touch: the put-down button
  await drop.click();
  await nextFrames(page, 2);
  l = await lying(page);
  expect(l.foods.map((f) => f.food)).toEqual(['hay']);
  expect(await carry(page)).toBe('');
  await expect(drop).toBeHidden();
  expect(errors).toEqual([]);
});

async function lookAtBamboo(page: Page, name: string) {
  // camera from the north (the cut spots are on the north and east edges of the thicket)
  await page.evaluate(() => {
    const a = window.__zoo!.app;
    if (Math.abs(a.camera_target_yaw_deg()) < 1) {
      a.rotate(4);
      a.zoom(0.6);
    }
    a.debug_look_at(-19.5, 2.5);
  });
  await nextFrames(page, 12);
  await page.evaluate(() => window.__zoo!.app.debug_look_at(-19.5, 2.5));
  await nextFrames(page, 3);
  await page.screenshot({ path: path.join(shots, `screenshot_bamboo_${name}.png`) });
}

// FEED-017 (desktop E and the touch action button), stages stump → young → full (FEED-019)
test('FEED-017: bamboo is cut at a cut spot of the bamboo forest (E and touch)', async ({ page }) => {
  const errors = await start(page);
  const all = await spots(page);
  expect(all.length).toBe(4);
  expect(all.every((s) => s.stage === 'full')).toBe(true);
  // desktop: E at the first spot
  const s0 = all[0];
  await goto(page, s0.sx, s0.sz);
  await page.evaluate(([x, z]) => window.__zoo!.app.debug_face_point(x, z), [s0.x, s0.z]);
  await nextFrames(page, 2);
  expect(await page.evaluate(() => window.__zoo!.app.target_key())).toBe(`bamboo:${s0.id}`);
  await lookAtBamboo(page, 'full');
  await page.keyboard.press('KeyE');
  await nextFrames(page, 2);
  expect(await carry(page)).toBe('bamboo');
  expect((await spots(page))[0].stage).toBe('stump');
  await lookAtBamboo(page, 'stump');

  // touch: the action button at the second spot; the bamboo in the hands goes down at her feet
  const s1 = all[1];
  await goto(page, s1.sx, s1.sz);
  await page.evaluate(([x, z]) => window.__zoo!.app.debug_face_point(x, z), [s1.x, s1.z]);
  await page.evaluate(() => window.__zoo!.ui.setTouch());
  await nextFrames(page, 3);
  const act = page.locator('#act');
  await expect(act).toBeVisible();
  await expect(act).toHaveText('🎋');
  await act.dispatchEvent('pointerdown');
  await nextFrames(page, 2);
  expect(await carry(page)).toBe('bamboo');
  expect((await spots(page))[1].stage).toBe('stump');
  expect((await lying(page)).foods.map((f) => f.food)).toEqual(['bamboo']);

  // regrowth in play time: young after half of the 3 minutes, full again after 3 minutes
  await page.evaluate(() => window.__zoo!.app.debug_step(100));
  expect((await spots(page))[0].stage).toBe('young');
  await lookAtBamboo(page, 'young');
  await page.evaluate(() => window.__zoo!.app.debug_step(85));
  expect((await spots(page))[0].stage).toBe('full');
  expect(errors).toEqual([]);
});

/** Completes levels 1 and 2 and the night between (the level-3 barriers open next morning). */
async function unlockLevel3(page: Page) {
  await page.evaluate(() => {
    const a = window.__zoo!.app;
    for (const n of ['zebra', 'hippo', 'panda', 'koala', 'elephant', 'giraffe', 'lion']) a.debug_send_home(n);
    a.debug_set_daytime('night');
    for (const n of ['hedgehog', 'bat', 'owl']) a.debug_send_home(n);
    a.debug_next_morning();
  });
  await nextFrames(page, 3);
  expect(await page.evaluate(() => window.__zoo!.app.level_unlocked('level_3'))).toBe(true);
}

type Wheel = { x: number; z: number; axle_y: number; radius: number; angle_deg: number };

// LAYOUT-L3-018
test('LAYOUT-L3-018: the water wheel dips into the stream and turns', async ({ page }) => {
  const errors = await start(page);
  await unlockLevel3(page);
  await expect(page.locator('#celebrate')).toBeHidden({ timeout: 20_000 });
  const wheels = JSON.parse(await page.evaluate(() => window.__zoo!.app.water_wheels_json())) as Wheel[];
  expect(wheels.length).toBe(1);
  const w = wheels[0];
  expect(w.axle_y - w.radius, 'lowest paddle under the water surface').toBeLessThan(-0.25 * w.radius + 1e-3);
  const stand = await page.evaluate(([x, z]) => window.__zoo!.app.debug_stand_near_point(x, z, 3.0), [w.x + 3, w.z - 3]);
  await page.evaluate(([x, z]) => window.__zoo!.app.debug_teleport(x, z), [stand[0], stand[1]]);
  await page.evaluate(([x, z]) => window.__zoo!.app.debug_look_at(x, z), [w.x, w.z]);
  await nextFrames(page, 4);
  const a0 = (JSON.parse(await page.evaluate(() => window.__zoo!.app.water_wheels_json())) as Wheel[])[0].angle_deg;
  const shot0 = await page.screenshot({ path: path.join(shots, 'screenshot_water_wheel_a.png') });
  // 0.5 s of game time later
  await page.evaluate(() => window.__zoo!.app.debug_step(0.5));
  await page.evaluate(([x, z]) => window.__zoo!.app.debug_look_at(x, z), [w.x, w.z]);
  await nextFrames(page, 2);
  const a1 = (JSON.parse(await page.evaluate(() => window.__zoo!.app.water_wheels_json())) as Wheel[])[0].angle_deg;
  const shot1 = await page.screenshot({ path: path.join(shots, 'screenshot_water_wheel_b.png') });
  let turned = Math.abs(a1 - a0) % 360;
  turned = Math.min(turned, 360 - turned);
  expect(turned, 'the wheel angle changed').toBeGreaterThan(15);
  expect(shot0.equals(shot1), 'the two frames differ').toBe(false);
  expect(errors).toEqual([]);
});
