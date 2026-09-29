// PoC M5a: every level-1 animal is playable (RESC-010 for hippo and panda, RESC-017 no raw
// keys), discovery (RESC-014/015: the seed picks the hiding place, the board shows its
// riddle; Q-082 a new game avoids the last places) and wandering animals that survive a
// reload (RESC-016, ANIM-011, GAME-SAVE).
import path from 'node:path';
import { expect, test, type Page } from '@playwright/test';
import { approach, face, ftl, goto, nextFrames, shots, START_URL, waitFrames } from './helpers';

test.describe.configure({ timeout: 300_000 });
test.use({ viewport: { width: 1280, height: 720 } });

const RAW_KEY = /\b(mission|animal|food|ui)-[a-z_]+-[a-z0-9_-]+/;

async function start(page: Page, url = START_URL, clear = true) {
  await page.addInitScript(
    ([c]) => {
      if (sessionStorage.getItem('zoo.e2e.init')) return;
      sessionStorage.setItem('zoo.e2e.init', '1');
      if (c) localStorage.clear();
      localStorage.setItem('zoo.language', 'de');
      localStorage.setItem('zoo.readingLevel', 'klasse1');
    },
    [clear] as const,
  );
  const errors: string[] = [];
  page.on('console', (m) => {
    if (m.type() === 'error') errors.push(m.text());
  });
  page.on('pageerror', (e) => errors.push(String(e)));
  await page.goto(url);
  await waitFrames(page, 3);
  return errors;
}

async function settle(page: Page) {
  await page.evaluate(() => window.__zoo!.app.debug_step(0.5));
  await nextFrames(page, 2);
}

const app = <T>(page: Page, fn: string, ...args: unknown[]) =>
  page.evaluate(([f, a]) => (window.__zoo!.app as unknown as Record<string, (...x: unknown[]) => T>)[f as string](...(a as unknown[])), [fn, args] as const);

interface Mission {
  animal: string;
  food: string;
  boxX: number;
  board: [number, number, string];
  gate: [number, number, string];
}

const HIPPO: Mission = { animal: 'hippo', food: 'melons', boxX: -4.22, board: [7.0, 21.5, 'KeyD'], gate: [8.4, 16.0, 'KeyD'] };
const PANDA: Mission = { animal: 'panda', food: 'bamboo', boxX: -2.18, board: [-3.5, 28.8, 'KeyW'], gate: [0.0, 31.3, 'KeyW'] };

/** Plays one mission end to end like a child (board → box → animal → gate). */
async function play(page: Page, m: Mission, shot?: (name: string) => Promise<void>) {
  const t = ftl('de');
  const place = await app<string>(page, 'animal_hiding_place', m.animal);
  // 1. Info board: the riddle of the chosen place, the food word and the facts (RESC-015,
  //    ANIM-006), no raw key (RESC-017).
  await goto(page, m.board[0], m.board[1]);
  await face(page, m.board[2]);
  expect(await app<string>(page, 'target_key')).toBe(`info_board:${m.animal}`);
  await settle(page);
  await expect(page.locator('#panel')).toBeVisible();
  await expect(page.locator('#panel-text')).toHaveText(t[`mission-${m.animal}-riddle-${place}-klasse1`]);
  await expect(page.locator('#panel-food .word')).toHaveText(t[`food-${m.food}`]);
  await expect(page.locator('#panel-title .word')).toHaveText(t[`animal-${m.animal}-more`]);
  await expect(page.locator('#panel-facts')).toHaveText(t[`mission-${m.animal}-facts-klasse1`]);
  expect(await page.locator('#panel').innerText()).not.toMatch(RAW_KEY);
  expect(await app<boolean>(page, 'mission_started', m.animal)).toBe(true);
  await page.keyboard.press('Escape');
  await expect(page.locator('#panel')).toBeHidden();
  // 2. The food box.
  await goto(page, m.boxX, 9.6); // the row in front of the storage (Q-181)
  await face(page, 'KeyW');
  expect(await app<string>(page, 'target_key')).toBe(`food_box:${m.food}`);
  await settle(page);
  await expect(page.locator('#panel-text')).toHaveText(t[`food-${m.food}`]);
  await page.locator('#take').click();
  expect(await app<string>(page, 'carry_food')).toBe(m.food);
  // 3. Find the animal where the riddle points, show the food.
  await approach(page, m.animal);
  expect(await app<string>(page, 'target_key')).toBe(`animal:${m.animal}`);
  if (shot) await shot('found');
  await page.keyboard.press('KeyE');
  await expect(page.locator('#bubble')).toHaveText(t['ui-following']);
  expect(await app<string>(page, 'animal_state', m.animal)).toBe('following');
  // 4. Lead it home through its gate.
  await goto(page, m.gate[0], m.gate[1]);
  await page.evaluate(() => window.__zoo!.app.debug_step(4));
  const d = await page.evaluate((id) => {
    const a = window.__zoo!.app;
    return Math.hypot(a.animal_x(id) - a.player_x(), a.animal_z(id) - a.player_z());
  }, m.animal);
  expect(d).toBeLessThan(3);
  await face(page, m.gate[2]);
  expect(await app<string>(page, 'target_key')).toBe(`gate:enc_${m.animal}`);
  await page.keyboard.press('KeyE');
  expect(await app<string>(page, 'animal_state', m.animal)).toBe('in_enclosure');
  expect(await app<boolean>(page, 'mission_complete', m.animal)).toBe(true);
  expect(await app<string>(page, 'carry_food')).toBe('');
  await expect(page.locator('#celebrate-text')).toHaveText(t[`mission-${m.animal}-home`]);
}

test('RESC-010 / RESC-017: hippo mission de klasse1 — pond, melons, home into the pool', async ({ page }) => {
  const errors = await start(page);
  expect(await app<string>(page, 'animal_hiding_place', 'hippo')).toBe('loc_pond');
  expect(await app<boolean>(page, 'animal_is_model', 'hippo')).toBe(true);
  await play(page, HIPPO, async (name) => {
    if (name !== 'found') return;
    // in the pond the hippo swims, sunk to its water line (only back, eyes and ears show)
    const clip = (await app<string>(page, 'animal_clip', 'hippo')).split('|');
    expect(clip[0]).toBe('swim');
    expect(Number(clip[2])).toBeGreaterThan(0.5);
    await page.screenshot({ path: path.join(shots, 'screenshot_poc_m5a_hippo_pond.png') });
  });
  // at home it wanders into the pool over the ramp (ANIM-012): let 60 s pass
  await page.evaluate(() => window.__zoo!.app.debug_step(60));
  const x = await app<number>(page, 'animal_x', 'hippo');
  const z = await app<number>(page, 'animal_z', 'hippo');
  expect(x).toBeGreaterThan(9);
  expect(x).toBeLessThan(20);
  expect(z).toBeGreaterThan(11);
  expect(z).toBeLessThan(23);
  // the pool with its ramp, seen from the ring path north-west of the gate
  await goto(page, 6.5, 19.5);
  await page.evaluate(() => window.__zoo!.app.zoom(0.8));
  await expect(page.locator('#celebrate')).toBeHidden({ timeout: 15_000 });
  await nextFrames(page, 20);
  await page.screenshot({ path: path.join(shots, 'screenshot_poc_m5a_hippo_home.png') });
  expect(errors).toEqual([]);
});

test('RESC-010 / RESC-017: panda mission de klasse1 — cave, bamboo, home', async ({ page }) => {
  const errors = await start(page);
  expect(await app<string>(page, 'animal_hiding_place', 'panda')).toBe('loc_cave');
  await play(page, PANDA);
  await page.evaluate(() => window.__zoo!.app.debug_step(3));
  expect(errors).toEqual([]);
});

test('RESC-014 / RESC-015: two seeds put the animals at different places; the board riddle matches', async ({ page }) => {
  const t = ftl('de');
  const picks: Record<string, string>[] = [];
  for (const seed of [17, 2]) {
    await start(page, `/?seed=${seed}`);
    const p = JSON.parse(await app<string>(page, 'picks_json')) as Record<string, string>;
    picks.push(p);
    // the zebra board shows the riddle of the zebra's place
    await goto(page, -7.5, 10.5);
    await face(page, 'KeyA');
    await settle(page);
    await expect(page.locator('#panel-text')).toHaveText(t[`mission-zebra-riddle-${p.zebra}-klasse1`]);
    // the zebra is really there: its info-board riddle names the place it wanders at
    const place = await app<string>(page, 'animal_hiding_place', 'zebra');
    expect(place).toBe(p.zebra);
    await page.keyboard.press('Escape');
    if (place === 'loc_sand') {
      // seed 2: the zebra rolls in the yellow sand patch north of the panda enclosure
      await approach(page, 'zebra');
      await nextFrames(page, 10);
      await page.screenshot({ path: path.join(shots, 'screenshot_poc_m5a_discovery_sand.png') });
    }
    await page.evaluate(() => sessionStorage.clear());
  }
  expect(picks[0]).not.toEqual(picks[1]);
  expect(picks[0].zebra).toBe('loc_river');
  expect(picks[1].zebra).toBe('loc_sand');
});

test('RESC-024 / Q-082: a new game avoids the hiding places of the previous game', async ({ page }) => {
  await start(page);
  const first = JSON.parse(await app<string>(page, 'picks_json')) as Record<string, string>;
  // "new game" (settings, GAME-SAVE §6): the save is deleted and the level restarts
  await page.evaluate(() => {
    window.__zoo!.slot.reset();
  });
  await page.reload();
  await waitFrames(page, 3);
  const second = JSON.parse(await app<string>(page, 'picks_json')) as Record<string, string>;
  for (const a of ['zebra', 'hippo', 'panda']) {
    expect(second[a], a).not.toBe(first[a]);
  }
});

test('RESC-016 / ANIM-011: wandering animals are restored at their places after a reload', async ({ page }) => {
  await start(page);
  await page.evaluate(() => window.__zoo!.app.debug_step(40));
  const before = await page.evaluate(() => {
    const a = window.__zoo!.app;
    return ['zebra', 'hippo', 'panda'].map((id) => ({ id, x: a.animal_x(id), z: a.animal_z(id), place: a.animal_hiding_place(id) }));
  });
  await page.evaluate(() => window.__zoo!.slot.flush());
  await page.reload();
  await waitFrames(page, 3);
  const after = await page.evaluate(() => {
    const a = window.__zoo!.app;
    return ['zebra', 'hippo', 'panda'].map((id) => ({ id, x: a.animal_x(id), z: a.animal_z(id), place: a.animal_hiding_place(id) }));
  });
  for (let i = 0; i < 3; i++) {
    expect(after[i].place).toBe(before[i].place);
    // real frames run between the read-out and the reload (pagehide saves): ≤ 2 s of wandering
    expect(Math.hypot(after[i].x - before[i].x, after[i].z - before[i].z), after[i].id).toBeLessThan(1.0);
  }
  // they keep wandering afterwards and stay at their place (≤ 3 m around the start area)
  let moved = false;
  for (let k = 0; k < 6; k++) {
    await page.evaluate(() => window.__zoo!.app.debug_step(10));
    const now = await page.evaluate(() => {
      const a = window.__zoo!.app;
      return ['zebra', 'hippo', 'panda'].map((id) => ({ x: a.animal_x(id), z: a.animal_z(id) }));
    });
    for (let i = 0; i < 3; i++) {
      const d = Math.hypot(now[i].x - after[i].x, now[i].z - after[i].z);
      expect(d).toBeLessThan(6.1);
      if (d > 0.5) moved = true;
    }
  }
  expect(moved).toBe(true);
});
