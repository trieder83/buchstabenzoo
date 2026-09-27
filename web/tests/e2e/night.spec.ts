// GAME-NIGHT e2e (NIGHT-001…005, NIGHT-008): complete level 1 → dusk → night; the bed →
// the next morning with level 2 open; the moon door → night_1 → back; readability at night;
// a save at night; draw calls / frame time night vs. day; review screenshots
// art/environment/poc/screenshot_night_{path,bedroom,moondoor,house}.png (compare with
// art/environment/style_frame_night/style_frame_night.png — NIGHT-009 is a manual review).
import path from 'node:path';
import { expect, test, type Page } from '@playwright/test';
import { ftl, goto, nextFrames, shots, waitFrames, START_URL } from './helpers';

test.use({ viewport: { width: 1280, height: 720 } });

async function start(page: Page) {
  await page.addInitScript(() => {
    if (sessionStorage.getItem('zoo.e2e.init')) return;
    sessionStorage.setItem('zoo.e2e.init', '1');
    localStorage.clear();
    localStorage.setItem('zoo.readingLevel', 'klasse1');
  });
  const errors: string[] = [];
  page.on('console', (m) => {
    if (m.type() === 'error') errors.push(m.text());
  });
  page.on('pageerror', (e) => errors.push(String(e)));
  await page.goto(START_URL);
  await waitFrames(page, 3);
  return errors;
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
async function app<T>(page: Page, fn: string, ...args: any[]): Promise<T> {
  return page.evaluate(
    ([f, a]) => {
      const zoo = window.__zoo!.app as unknown as Record<string, (...x: unknown[]) => unknown>;
      return zoo[f as string](...(a as unknown[])) as T;
    },
    [fn, args] as const,
  );
}

/** Level 1 complete (debug: the animals walk home), celebration hidden. */
async function finishLevel1(page: Page) {
  for (const a of ['zebra', 'hippo', 'panda']) expect(await app<boolean>(page, 'debug_send_home', a)).toBe(true);
  await nextFrames(page, 2);
  await page.evaluate(() => {
    document.getElementById('celebrate')!.hidden = true;
  });
}

/** Full night quickly (game time: celebration + dusk). */
async function toNight(page: Page) {
  await finishLevel1(page);
  await page.evaluate(() => window.__zoo!.app.debug_step(18.5));
  await nextFrames(page, 2);
  expect(await app<string>(page, 'daytime')).toBe('night');
}

/** Mean luma (0…255) of a CSS-px rect of the canvas (drawn now). */
async function meanLuma(page: Page, r: number[]): Promise<number> {
  return page.evaluate((r) => {
    const a = window.__zoo!.app;
    const canvas = document.getElementById('game') as HTMLCanvasElement;
    const gl = canvas.getContext('webgl2')!;
    a.frame(0);
    const k = canvas.width / canvas.clientWidth;
    const x0 = Math.max(0, Math.floor(r[0] * k));
    const x1 = Math.min(canvas.width, Math.ceil(r[2] * k));
    const y0 = Math.max(0, Math.floor(canvas.height - r[3] * k));
    const y1 = Math.min(canvas.height, Math.ceil(canvas.height - r[1] * k));
    const w = Math.max(1, x1 - x0);
    const h = Math.max(1, y1 - y0);
    const px = new Uint8Array(w * h * 4);
    gl.readPixels(x0, y0, w, h, gl.RGBA, gl.UNSIGNED_BYTE, px);
    let s = 0;
    for (let i = 0; i < px.length; i += 4) s += 0.299 * px[i] + 0.587 * px[i + 1] + 0.114 * px[i + 2];
    return s / (w * h);
  }, r);
}

test('NIGHT-001 / NIGHT-002: level 1 complete → dusk after the celebration → night; moon door and bed', async ({ page }) => {
  const t = ftl('de');
  const errors = await start(page);
  expect(await app<string>(page, 'daytime')).toBe('day');
  expect(await app<boolean>(page, 'barrier_open', 'moon_door')).toBe(false);
  await finishLevel1(page);
  await page.evaluate(() => window.__zoo!.app.debug_step(6.5));
  expect(await app<string>(page, 'daytime'), 'not before the celebration ends').toBe('day');
  await page.evaluate(() => window.__zoo!.app.debug_step(1));
  expect(await app<string>(page, 'daytime')).toBe('dusk');
  await nextFrames(page, 2);
  // the gentle cut-in text (klasse1, Fluent)
  await expect(page.locator('#night-banner')).toHaveText(t['night-dusk-klasse1']);
  await page.evaluate(() => window.__zoo!.app.debug_step(10.5));
  expect(await app<string>(page, 'daytime')).toBe('night');
  const [night] = await app<number[]>(page, 'daylight');
  expect(night).toBe(1);
  expect(await app<boolean>(page, 'barrier_open', 'moon_door')).toBe(true);
  expect(await app<boolean>(page, 'level_unlocked', 'night_1')).toBe(true);
  await nextFrames(page, 2);
  // the two night choices as icons (no reading needed)
  await expect(page.locator('#night-choices')).toBeVisible();
  await expect(page.locator('#choice-bed')).toHaveText('🛏️');
  await expect(page.locator('#choice-moon')).toHaveText('🌙');
  // lamps: the player's lantern + nearby lamps are lit, pools for the rest (Q-114)
  const [lights] = await app<number[]>(page, 'light_stats');
  expect(lights).toBeGreaterThanOrEqual(1);
  expect(lights).toBeLessThanOrEqual(9);
  expect(errors).toEqual([]);
});

test('NIGHT-003: the bed → sleep (saved) → the next morning with level 2 open', async ({ page }) => {
  const errors = await start(page);
  await toNight(page);
  // the night zoo is done this night (debug) — level 2 opens the morning after (Q-078)
  for (const a of ['hedgehog', 'bat', 'owl']) expect(await app<boolean>(page, 'debug_send_home', a)).toBe(true);
  expect(await app<boolean>(page, 'barrier_open', 'barrier_ne_tree'), 'only the next morning').toBe(false);
  await nextFrames(page, 2);
  await expect(page.locator('#night-banner')).toHaveText(ftl('de')['night-complete-klasse1']);
  await page.evaluate(() => {
    document.getElementById('celebrate')!.hidden = true;
    document.getElementById('night-banner')!.hidden = true;
  });
  // into the zookeeper house next to the bed
  await goto(page, -11.5, 2.5);
  expect(await app<string>(page, 'player_inside')).toBe('zookeeper_house_1');
  const bed = await app<number[]>(page, 'bed_point');
  await app(page, 'debug_face_point', bed[0], bed[1]);
  await nextFrames(page, 2);
  expect(await app<string>(page, 'target_kind')).toBe('bed');
  await expect(page.locator('#hint .icon')).toHaveText('🛏️');
  await page.evaluate(() => window.__zoo!.app.debug_step(0.1));
  await nextFrames(page, 2);
  await page.screenshot({ path: path.join(shots, 'screenshot_night_bedroom.png') });
  await page.keyboard.press('KeyE');
  expect(await app<string>(page, 'daytime')).toBe('sleeping');
  await nextFrames(page, 2);
  const saved = await page.evaluate(() => localStorage.getItem('zoo.save') ?? '');
  expect(saved, 'saved when going to bed').toContain('"phase":"sleeping"');
  await page.evaluate(() => window.__zoo!.app.debug_step(1));
  await nextFrames(page, 2);
  expect(Number(await page.locator('#dream').evaluate((e) => getComputedStyle(e).opacity))).toBeGreaterThan(0.9);
  await page.evaluate(() => window.__zoo!.app.debug_step(2));
  expect(await app<string>(page, 'daytime')).toBe('morning');
  expect(await app<boolean>(page, 'barrier_open', 'barrier_ne_tree')).toBe(true);
  expect(await app<boolean>(page, 'level_unlocked', 'level_2')).toBe(true);
  expect(await app<boolean>(page, 'barrier_open', 'moon_door'), 'closed by day').toBe(false);
  for (const a of ['zebra', 'hippo', 'panda']) expect(await app<string>(page, 'animal_state', a)).toBe('in_enclosure');
  await page.evaluate(() => window.__zoo!.app.debug_step(3.5));
  expect(await app<string>(page, 'daytime')).toBe('day');
  await page.evaluate(() => window.__zoo!.slot.flush());
  const morning = await page.evaluate(() => localStorage.getItem('zoo.save') ?? '');
  expect(morning).toContain('"phase":"day"');
  expect(morning).toContain('barrier_ne_tree');
  expect(errors).toEqual([]);
});

test('NIGHT-004 / NIGHT-005: through the moon door into night_1 and back; everything readable', async ({ page }) => {
  const t = ftl('de');
  const errors = await start(page);
  await toNight(page);
  // the moon door, open, on the west wall of level 1
  await goto(page, -20.5, 30.0);
  await app(page, 'debug_face_point', -24, 30);
  await page.evaluate(() => window.__zoo!.app.debug_step(0.1));
  await nextFrames(page, 3);
  await page.screenshot({ path: path.join(shots, 'screenshot_night_moondoor.png') });
  // walk through it (no button needed)
  await goto(page, -28.5, 29.5);
  expect(await app<boolean>(page, 'player_in_night_zoo')).toBe(true);
  // night_1: its own missions — the hedgehog board reads its night riddle
  await goto(page, -41.5, 37.3);
  await app(page, 'debug_face_point', -41.5, 38.5);
  await page.evaluate(() => window.__zoo!.app.debug_step(0.5));
  await nextFrames(page, 3);
  expect(await app<string>(page, 'target_key')).toBe('info_board:hedgehog');
  const place = await app<string>(page, 'animal_hiding_place', 'hedgehog');
  await expect(page.locator('#panel-text')).toHaveText(t[`mission-hedgehog-riddle-${place}-klasse1`]);
  // NIGHT-005: the panel looks as by day (white card, dark text)
  const panel = await page.locator('#panel .panel-body').evaluate((e) => getComputedStyle(e).backgroundColor);
  expect(panel, 'the cream card of the day').toBe('rgb(255, 248, 231)');
  await page.keyboard.press('Escape');
  // the night house (roof hidden inside)
  await goto(page, -37.5, 41.5);
  expect(await app<string>(page, 'player_inside')).toBe('night_house');
  await app(page, 'debug_face_point', -37.5, 45);
  await page.evaluate(() => window.__zoo!.app.debug_step(0.1));
  await nextFrames(page, 3);
  await page.screenshot({ path: path.join(shots, 'screenshot_night_house.png') });
  // NIGHT-005: the player is clearly visible (her lantern light) — bright against the night
  const pr = await app<number[]>(page, 'player_screen_rect');
  const player = await meanLuma(page, pr);
  const w = pr[2] - pr[0];
  const around = await meanLuma(page, [pr[0] - 3 * w, pr[1] - 3 * w, pr[0] - 2 * w, pr[3]]);
  console.log(`night luma: player ${player.toFixed(0)}, beside ${around.toFixed(0)}`);
  expect(player).toBeGreaterThan(70);
  // back through the moon door: the day zoo at night
  await goto(page, -28.5, 29.5);
  await goto(page, -20.5, 30.0);
  expect(await app<boolean>(page, 'player_in_night_zoo')).toBe(false);
  expect(await app<string>(page, 'daytime')).toBe('night');
  expect(errors).toEqual([]);
});

test('NIGHT-008: a save in the night zoo restores the night and the position', async ({ page }) => {
  const errors = await start(page);
  await toNight(page);
  await goto(page, -20.5, 30.0);
  await goto(page, -34.5, 29.5);
  expect(await app<boolean>(page, 'player_in_night_zoo')).toBe(true);
  const before = await page.evaluate(() => [window.__zoo!.app.player_x(), window.__zoo!.app.player_z()]);
  await page.evaluate(() => window.__zoo!.slot.flush());
  await page.reload();
  await waitFrames(page, 3);
  expect(await app<string>(page, 'daytime')).toBe('night');
  expect(await app<boolean>(page, 'player_in_night_zoo')).toBe(true);
  expect(await app<number>(page, 'player_x')).toBeCloseTo(before[0], 2);
  expect(await app<number>(page, 'player_z')).toBeCloseTo(before[1], 2);
  expect(await app<boolean>(page, 'barrier_open', 'moon_door')).toBe(true);
  await nextFrames(page, 2);
  await expect(page.locator('#night-choices')).toBeVisible();
  expect(errors).toEqual([]);
});

test('NIGHT-014 performance + review shot: draw calls and frame time night vs. day', async ({ page }) => {
  const errors = await start(page);
  const measure = async () => {
    await goto(page, -6.9, 12.5);
    await app(page, 'debug_face_point', -6.9, 20);
    await page.evaluate(() => window.__zoo!.app.debug_step(0.1));
    await nextFrames(page, 3);
    // warm-up, then 20 frames
    const fw = await page.evaluate(() => window.__zoo!.frames);
    await page.waitForFunction((f) => window.__zoo!.frames >= f + 10, fw, { timeout: 60_000 });
    const f0 = await page.evaluate(() => window.__zoo!.frames);
    const t0 = Date.now();
    await page.waitForFunction((f) => window.__zoo!.frames >= f + 20, f0, { timeout: 90_000 });
    const wall = (Date.now() - t0) / 20;
    return page.evaluate((wall) => {
      const z = window.__zoo!;
      return { draws: z.app.draw_calls(), frameMs: z.frameMs, wallMs: wall, lights: z.app.light_stats() };
    }, wall);
  };
  const day = await measure();
  await toNight(page);
  const night = await measure();
  await page.screenshot({ path: path.join(shots, 'screenshot_night_path.png') });
  console.log(`day   draw calls ${day.draws}, frame CPU ${day.frameMs.toFixed(2)} ms, frame ${day.wallMs.toFixed(0)} ms`);
  console.log(
    `night draw calls ${night.draws}, frame CPU ${night.frameMs.toFixed(2)} ms, frame ${night.wallMs.toFixed(0)} ms, lights/pools/lamps ${night.lights.join('/')}`,
  );
  // night props are batched: at most a few extra draw calls (lamp boxes, glow boxes)
  expect(night.draws).toBeLessThanOrEqual(day.draws + 6);
  expect(errors).toEqual([]);
});

// NIGHT-016: sleeping works with the normal interact action next to the bed — `E` on the
// desktop, the action button showing 🛏 on touch; away from the bed neither is bed-related.
test('NIGHT-016: next to the bed, E starts sleeping (desktop); away from it nothing happens', async ({ page }) => {
  const errors = await start(page);
  await toNight(page);
  const stand = await app<number[]>(page, 'bed_stand');
  expect(stand.length).toBe(2);
  // away from the bed (on the plaza): E does nothing bed-related
  await goto(page, 0.5, 4.5);
  await page.keyboard.press('KeyE');
  expect(await app<string>(page, 'daytime')).toBe('night');
  await goto(page, stand[0], stand[1]);
  expect(await app<string>(page, 'player_inside')).toBe('zookeeper_house_1');
  const bed = await app<number[]>(page, 'bed_point');
  await app(page, 'debug_face_point', bed[0], bed[1]);
  await nextFrames(page, 2);
  expect(await app<string>(page, 'target_kind')).toBe('bed');
  await expect(page.locator('#hint .icon')).toHaveText('🛏️');
  await page.keyboard.press('KeyE');
  expect(await app<string>(page, 'daytime')).toBe('sleeping');
  expect(errors).toEqual([]);
});

test.describe('touch', () => {
  test.use({ viewport: { width: 540, height: 1170 }, hasTouch: true, isMobile: true });

  test('NIGHT-016: next to the bed, the 🛏 action button starts sleeping (touch)', async ({ page }) => {
    const errors = await start(page);
    await toNight(page);
    // first touch: touch controls on (PLAY-014)
    await page.touchscreen.tap(270, 400);
    const stand = await app<number[]>(page, 'bed_stand');
    await goto(page, stand[0], stand[1]);
    const bed = await app<number[]>(page, 'bed_point');
    await app(page, 'debug_face_point', bed[0], bed[1]);
    await nextFrames(page, 3);
    const act = page.locator('#act');
    await expect(act).toBeVisible();
    await expect(act).toHaveText('🛏️');
    await act.tap();
    await nextFrames(page, 2);
    expect(await app<string>(page, 'daytime')).toBe('sleeping');
    expect(errors).toEqual([]);
  });
});
