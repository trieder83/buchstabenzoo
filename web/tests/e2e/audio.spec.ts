// ART-SOUND "Playback": ASND-005 (footsteps), 006 (doors, pickups), 007 (lazy loading after the
// first gesture), 009 (sound switch). No real audio device is needed: the host logs every cue
// decision (`window.__zoo.audio.log`) before it plays.
import fs from 'node:fs';
import path from 'node:path';
import { expect, test, type Page } from '@playwright/test';
import { approach, goto, nextFrames, repo, START_URL, waitFrames } from './helpers';

interface Entry {
  cue: string;
  file: string | null;
  gain: number;
  rate: number;
  x: number;
  z: number;
  muted: boolean;
  result: string;
}

async function start(page: Page, url = START_URL) {
  const errors: string[] = [];
  page.on('console', (m) => {
    if (m.type() === 'error') errors.push(m.text());
  });
  await page.goto(url);
  await waitFrames(page, 3);
  return errors;
}

/** A user gesture (autoplay rule): unlocks the audio. */
async function gesture(page: Page) {
  await page.keyboard.press('Shift');
  await page.waitForFunction(() => window.__zoo!.audio.isUnlocked);
}

const clear = (page: Page) => page.evaluate(() => void (window.__zoo!.audio.log.length = 0));

/** Log entries after the pending frames have polled the sound events. */
async function log(page: Page): Promise<Entry[]> {
  await nextFrames(page, 4);
  return page.evaluate(() => window.__zoo!.audio.log.map((e) => ({ ...e })) as Entry[]);
}

const steps = (l: Entry[]) => l.filter((e) => e.cue.startsWith('step_'));

// ASND-007
test('ASND-007: no audio file is fetched before the first gesture; it loads lazily afterwards', async ({ page }) => {
  const audioRequests: string[] = [];
  page.on('request', (r) => {
    if (r.url().includes('/assets/audio/')) audioRequests.push(r.url());
  });
  const errors = await start(page);
  await nextFrames(page, 30);
  expect(audioRequests).toEqual([]);
  expect(await page.evaluate(() => window.__zoo!.audio.fetched)).toEqual([]);
  expect(await page.evaluate(() => window.__zoo!.audio.state)).toBe('none');
  // the asset list carries the audio files (served by the build)
  const idx = (await (await page.request.get('/assets/index.json')).json()) as string[];
  expect(idx.some((f) => f.startsWith('audio/steps/step_path_1.'))).toBe(true);
  // the first gesture creates the AudioContext; the groups are prefetched when idle (~2 s later)
  await gesture(page);
  await page.waitForFunction(() => window.__zoo!.audio.fetched.length > 0, null, { timeout: 20_000 });
  expect(audioRequests.length).toBeGreaterThan(0);
  expect(audioRequests[0]).toMatch(/\/assets\/audio\/steps\//); // common groups first, one by one
  expect(errors).toEqual([]);
});

// ASND-005
test('ASND-005: one step cue per footfall of the walk clip while moving, none while standing', async ({ page }) => {
  const errors = await start(page);
  await gesture(page);
  await goto(page, 2.5, 4.5);
  await clear(page);
  // standing: nothing
  await page.evaluate(() => window.__zoo!.app.debug_step(3));
  expect(steps(await log(page))).toEqual([]);

  // walking 3 s on the plaza path: footfalls every 0.4 clip-seconds (rate 1.38 on a path)
  await page.evaluate(() => window.__zoo!.app.debug_goto(2.5, 4.5 + 40));
  await page.evaluate(() => window.__zoo!.app.debug_step(3));
  const walk = steps(await log(page));
  expect(walk.length).toBeGreaterThanOrEqual(8);
  expect(walk.length).toBeLessThanOrEqual(13);
  // each step is at the player's position on the surface under it, quiet (ASND-010) and in range
  for (const e of walk) {
    const s = await page.evaluate(([x, z]) => window.__zoo!.app.debug_step_surface(x, z), [e.x, e.z]);
    expect(e.cue).toBe(`step_${s}`);
    expect(e.file).toMatch(/audio\/steps\/step_\w+_\d\.(ogg|m4a)$/);
    expect(e.gain).toBeGreaterThan(0);
    expect(e.gain).toBeLessThanOrEqual(0.175 + 1e-6);
    expect(e.rate).toBeGreaterThanOrEqual(0.95);
    expect(e.rate).toBeLessThanOrEqual(1.05);
  }
  expect(new Set(walk.map((e) => e.cue)).has('step_path')).toBe(true);

  // standing again: nothing more
  await page.evaluate(() => {
    const a = window.__zoo!.app;
    a.debug_teleport(2.5, 4.5);
    a.debug_goto(2.5, 4.5); // ends the pending scripted walk
    a.debug_step(1);
  });
  await nextFrames(page, 3);
  await clear(page);
  await page.evaluate(() => window.__zoo!.app.debug_step(2));
  expect(steps(await log(page))).toEqual([]);
  expect(errors).toEqual([]);
});

test('ASND-005: grass gives step_grass, a bridge step_wood', async ({ page }) => {
  const errors = await start(page);
  await gesture(page);
  // grass: the walk to the zebra's meadow ends on grass
  await clear(page);
  await approach(page, 'zebra');
  const g = steps(await log(page));
  expect(g.length).toBeGreaterThanOrEqual(10);
  expect(g.some((e) => e.cue === 'step_grass')).toBe(true);

  // wood: the river bridge (rect [10, 28, 3, 3] in level-1.toml)
  const toml = fs.readFileSync(path.join(repo, 'assets/levels/level-1.toml'), 'utf8');
  const m = /id = "bridge_river"[\s\S]*?rect = \[(-?\d+), (-?\d+), (\d+), (\d+)\]/.exec(toml)!;
  const [bx, bz, bw, bd] = m.slice(1).map(Number);
  const cx = bx + bw / 2;
  const cz = bz + bd / 2;
  await page.evaluate(([x, z]) => window.__zoo!.app.debug_teleport(x, z - 0.6), [cx, cz]);
  await clear(page);
  await page.evaluate(([x, z]) => {
    window.__zoo!.app.debug_goto(x, z + 0.6);
    window.__zoo!.app.debug_step(1);
  }, [cx, cz]);
  const w = steps(await log(page));
  expect(w.length).toBeGreaterThanOrEqual(1);
  expect(w.some((e) => e.cue === 'step_wood')).toBe(true);
  expect(errors).toEqual([]);
});

// ASND-006
test('ASND-006: a gate opens and closes once each at its position; a box is picked up and put down once', async ({ page }) => {
  const errors = await start(page);
  await gesture(page);
  const gate = await page.evaluate(() => {
    const list = JSON.parse(window.__zoo!.app.openings_json()) as { kind: string; x: number; z: number }[];
    return list.find((o) => o.kind === 'garden_gate') ?? null;
  });
  expect(gate, 'garden gate').not.toBeNull();
  await page.evaluate(([x, z]) => window.__zoo!.app.debug_teleport(x + 6, z), [gate!.x, gate!.z]);
  await nextFrames(page, 20); // the level start is silent (no cue for the first state)
  expect((await log(page)).filter((e) => e.cue.startsWith('gate_'))).toEqual([]);
  await clear(page);
  await page.evaluate(([x, z]) => window.__zoo!.app.debug_teleport(x + 1, z), [gate!.x, gate!.z]);
  await nextFrames(page, 10);
  await page.evaluate(([x, z]) => window.__zoo!.app.debug_teleport(x + 6, z), [gate!.x, gate!.z]);
  await nextFrames(page, 30);
  const doors = (await log(page)).filter((e) => e.cue.startsWith('gate_'));
  expect(doors.map((e) => e.cue)).toEqual(['gate_open', 'gate_close']);
  for (const e of doors) {
    expect(Math.hypot(e.x - gate!.x, e.z - gate!.z)).toBeLessThan(0.1);
    expect(e.file).toMatch(/audio\/doors\/gate_(open|close)_\d\./);
  }

  // pickup / drop of a food box (level 1 food storage row, see feeding.spec.ts)
  await goto(page, -2.86, 9.5);
  await clear(page);
  expect(await page.evaluate(() => window.__zoo!.app.take_food('hay'))).toBe(true);
  let l = await log(page);
  expect(l.filter((e) => e.cue === 'pickup_food')).toHaveLength(1);
  const px = await page.evaluate(() => window.__zoo!.app.player_x());
  expect(Math.abs(l.find((e) => e.cue === 'pickup_food')!.x - px)).toBeLessThan(0.5);
  await clear(page);
  expect(await page.evaluate(() => window.__zoo!.app.put_down())).toBe(true);
  l = await log(page);
  expect(l.filter((e) => e.cue === 'drop_food')).toHaveLength(1);
  expect(l.filter((e) => e.cue === 'pickup_food')).toHaveLength(0);
  expect(errors).toEqual([]);
});

// ASND-009
test('ASND-009: the sound switch mutes every cue, is saved and restored', async ({ page }) => {
  const errors = await start(page);
  await page.locator('#settings-btn').click();
  const toggle = page.locator('#sound-toggle');
  await expect(toggle).toBeVisible();
  const box = (await toggle.boundingBox())!;
  expect(box.width).toBeGreaterThanOrEqual(64);
  expect(box.height).toBeGreaterThanOrEqual(64);
  await expect(toggle).toHaveText('🔊');
  await toggle.click();
  await expect(toggle).toHaveText('🔇');
  expect(await page.evaluate(() => window.localStorage.getItem('zoo.sound'))).toBe('0');
  expect(await page.evaluate(() => window.__zoo!.audio.enabled)).toBe(false);
  // a cue decision is logged as muted; nothing more is fetched, even after the idle prefetch time
  // (a group that was already downloading when the switch was pressed may still finish: wait until the count is stable)
  let fetchedBefore = await page.evaluate(() => window.__zoo!.audio.fetched.length);
  for (let i = 0; i < 20; i++) {
    await page.waitForTimeout(1500);
    const n = await page.evaluate(() => window.__zoo!.audio.fetched.length);
    if (n === fetchedBefore) break;
    fetchedBefore = n;
  }
  await clear(page);
  await goto(page, 2.5, 4.5);
  await page.evaluate(() => {
    window.__zoo!.app.debug_goto(2.5, 40);
    window.__zoo!.app.debug_step(2);
  });
  const l = steps(await log(page));
  expect(l.length).toBeGreaterThan(2);
  expect(l.every((e) => e.muted && e.result === 'muted')).toBe(true);
  await page.waitForTimeout(3500);
  await nextFrames(page, 3);
  expect(await page.evaluate(() => window.__zoo!.audio.fetched.length)).toBe(fetchedBefore);
  // restored after a reload
  await page.reload();
  await waitFrames(page, 3);
  expect(await page.evaluate(() => window.__zoo!.audio.enabled)).toBe(false);
  await page.locator('#settings-btn').click();
  await expect(page.locator('#sound-toggle')).toHaveText('🔇');
  await page.locator('#sound-toggle').click();
  await expect(page.locator('#sound-toggle')).toHaveText('🔊');
  expect(await page.evaluate(() => window.localStorage.getItem('zoo.sound'))).toBe('1');
  expect(await page.evaluate(() => window.__zoo!.audio.enabled)).toBe(true);
  expect(errors).toEqual([]);
});

// ASND-027 (NIGHT-024): quiet crickets at night, fetched lazily after the gesture, none by day
test('ASND-027: the cricket loop fades in at night (gain <= 0.12) and out by day', async ({ page }) => {
  const errors = await start(page);
  const amb = () =>
    page.evaluate(() => ({ ...window.__zoo!.audio.ambient, peak: window.__zoo!.audio.ambient.gain }));
  // day: nothing, even after the gesture and the idle prefetch
  await gesture(page);
  await nextFrames(page, 10);
  expect(await amb()).toMatchObject({ playing: false, gain: 0, fetched: false });
  // night: the loop is fetched and plays, never louder than 0.12
  expect(await page.evaluate(() => window.__zoo!.app.debug_set_daytime('night'))).toBe(true);
  await page.waitForFunction(() => window.__zoo!.audio.ambient.playing, null, { timeout: 15_000 });
  let max = 0;
  for (let i = 0; i < 10; i++) {
    await page.waitForTimeout(400);
    max = Math.max(max, (await amb()).gain);
  }
  expect((await amb()).fetched).toBe(true);
  expect(max).toBeGreaterThan(0.05);
  expect(max).toBeLessThanOrEqual(0.12 + 1e-9);
  // sleeping / the next morning: fades out and stops
  await page.evaluate(() => window.__zoo!.app.debug_set_daytime('day'));
  await page.waitForFunction(() => !window.__zoo!.audio.ambient.playing, null, { timeout: 15_000 });
  expect((await amb()).gain).toBe(0);
  expect(errors).toEqual([]);
});
