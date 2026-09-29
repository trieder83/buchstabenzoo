// PoC M5b: levels 2 and 3 joined to level 1 (GAME-LAYOUT "Joining levels", Q-088) — level
// unlocking (LAYOUT-022 / LAYOUT-L2-010), one full mission per new level (giraffe in level 2,
// goldfish with the bowl in level 3: LAYOUT-L3-014, RESC-018…021), the enterable zookeeper
// house (PLAY-028/029), saves across levels (GAME-SAVE v2) and render culling (QA F12).
import path from 'node:path';
import { expect, test, type Page } from '@playwright/test';
import { approach, ftl, goto, nextFrames, shots, waitFrames } from './helpers';

test.describe.configure({ timeout: 300_000 });
test.use({ viewport: { width: 1280, height: 720 } });

const RAW_KEY = /\b(mission|animal|food|ui)-[a-z_]+-[a-z0-9_-]+/;
/** Seed 4: giraffe at the lookout tower, elephant at the fountain, monkey on the pirate ship,
 * goldfish under the willow (discovery per level, GAME-RESCUE §1). */
const SEED_URL = '/?seed=4';

const app = <T>(page: Page, fn: string, ...args: unknown[]) =>
  page.evaluate(
    ([f, a]) => (window.__zoo!.app as unknown as Record<string, (...x: unknown[]) => T>)[f as string](...(a as unknown[])),
    [fn, args] as const,
  );

async function start(page: Page, url = SEED_URL) {
  await page.addInitScript(() => {
    if (sessionStorage.getItem('zoo.e2e.init')) return;
    sessionStorage.setItem('zoo.e2e.init', '1');
    localStorage.clear();
    localStorage.setItem('zoo.language', 'de');
    localStorage.setItem('zoo.readingLevel', 'klasse1');
  });
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

/** Walks to a point and turns towards a level point. */
async function standFacing(page: Page, x: number, z: number, fx: number, fz: number) {
  await goto(page, x, z);
  await app(page, 'debug_face_point', fx, fz);
  await nextFrames(page, 2);
}

/**
 * Completes the missions of a level at once (debug: the animals walk home), then the night
 * passes: the barriers open the next morning (GAME-NIGHT, replaces the Q-091 rule). Level 2
 * opens the morning after the night zoo is complete (Q-078, `unlock_after = "night_1"`), so
 * the night animals are sent home that night too.
 */
async function finish(page: Page, animals: string[]) {
  for (const a of animals) expect(await app<boolean>(page, 'debug_send_home', a), a).toBe(true);
  await nextFrames(page, 3);
  await page.evaluate(() => {
    const a = window.__zoo!.app;
    a.debug_set_daytime('night');
    for (const n of ['hedgehog', 'bat', 'owl']) a.debug_send_home(n);
    a.debug_next_morning();
  });
  await nextFrames(page, 3);
}

const L1 = ['zebra', 'hippo', 'panda'];
const L2 = ['koala', 'elephant', 'giraffe', 'lion'];

test('LAYOUT-022 / LAYOUT-L2-010: finishing level 1 opens the fallen tree; walk into level 2 and back', async ({ page }) => {
  const errors = await start(page);
  expect(await app<string>(page, 'level_ids')).toBe('level_1\nlevel_2\nlevel_3\nnight_1');
  expect(await app<boolean>(page, 'level_unlocked', 'level_2')).toBe(false);
  // the level-2 animals wait asleep and hidden while their level is locked
  expect(await app<boolean>(page, 'animal_visible', 'giraffe')).toBe(false);
  await goto(page, 20.5, 29.5);
  const blocked = await page.evaluate(() => {
    const a = window.__zoo!.app;
    a.debug_goto(27.5, 29.5);
    a.debug_step(15);
    return a.player_x();
  });
  expect(blocked, 'the fallen tree seals level 2').toBeLessThan(22);
  await finish(page, L1);
  expect(await app<boolean>(page, 'barrier_open', 'barrier_ne_tree')).toBe(true);
  expect(await app<boolean>(page, 'level_unlocked', 'level_2')).toBe(true);
  expect(await app<boolean>(page, 'level_unlocked', 'level_3')).toBe(false);
  await nextFrames(page, 2);
  expect(await app<boolean>(page, 'region_hidden', 'barrier_ne_tree'), 'the fallen tree is gone').toBe(true);
  expect(await app<boolean>(page, 'animal_visible', 'giraffe')).toBe(true);
  await goto(page, 27.5, 29.5);
  expect(await app<number>(page, 'player_x')).toBeGreaterThan(26);
  // the construction fence still seals level 3
  expect(await app<boolean>(page, 'barrier_open', 'barrier_l2_construction')).toBe(false);
  // back into level 1
  await goto(page, 12.5, 29.5);
  expect(await app<number>(page, 'player_x')).toBeLessThan(13);
  expect(errors).toEqual([]);
});

test('RESC-010 level 2: giraffe mission de klasse1 — lookout tower, leaves from storage 2, home', async ({ page }) => {
  const t = ftl('de');
  const errors = await start(page);
  await finish(page, L1);
  await expect(page.locator('#celebrate')).toBeHidden({ timeout: 15_000 });
  const place = await app<string>(page, 'animal_hiding_place', 'giraffe');
  expect(place).toBe('loc_lookout_tower');
  expect(await app<boolean>(page, 'animal_is_model', 'giraffe')).toBe(true);
  // level-2 overview from the entry: storage 2, the fountain with the elephant
  await goto(page, 30.5, 28.5);
  await page.evaluate(() => window.__zoo!.app.zoom(2));
  await page.evaluate(() => window.__zoo!.app.debug_step(1));
  await nextFrames(page, 20);
  await page.screenshot({ path: path.join(shots, 'screenshot_poc_m5b_level2_entry.png') });
  // 1. the info board: riddle of the chosen place, food word, no raw key
  await standFacing(page, 43.5, 41.2, 43.5, 42.5);
  expect(await app<string>(page, 'target_key')).toBe('info_board:giraffe');
  await settle(page);
  await expect(page.locator('#panel-text')).toHaveText(t[`mission-giraffe-riddle-${place}-klasse1`]);
  await expect(page.locator('#panel-food .word')).toHaveText(t['food-leaves']);
  expect(await page.locator('#panel').innerText()).not.toMatch(RAW_KEY);
  expect(await app<boolean>(page, 'mission_started', 'giraffe')).toBe(true);
  await page.keyboard.press('Escape');
  // 2. leaves from the level's own food storage (Q-089)
  await standFacing(page, 37.4, 32.77, 38.66, 32.77); // the row in front of storage 2 (Q-181)
  expect(await app<string>(page, 'target_key')).toBe('food_box:leaves');
  await settle(page);
  await expect(page.locator('#panel-text')).toHaveText(t['food-leaves']);
  await page.locator('#take').click();
  expect(await app<string>(page, 'carry_food')).toBe('leaves');
  // 3. the giraffe at the tower follows
  await approach(page, 'giraffe');
  expect(await app<string>(page, 'target_key')).toBe('animal:giraffe');
  await page.keyboard.press('KeyE');
  await expect(page.locator('#bubble')).toHaveText(t['ui-following']);
  // 4. home through its gate (south side of enc_giraffe)
  await goto(page, 46.0, 43.2);
  await page.evaluate(() => window.__zoo!.app.debug_step(5));
  await app(page, 'debug_face_point', 46.0, 44.5);
  await nextFrames(page, 2);
  expect(await app<string>(page, 'target_key')).toBe('gate:enc_giraffe');
  await page.keyboard.press('KeyE');
  expect(await app<string>(page, 'animal_state', 'giraffe')).toBe('in_enclosure');
  expect(await app<boolean>(page, 'mission_complete', 'giraffe')).toBe(true);
  await expect(page.locator('#celebrate-text')).toHaveText(t['mission-giraffe-home']);
  expect(errors).toEqual([]);
});

test('LAYOUT-L3-014 / RESC-018…021: goldfish with the bowl de klasse1 — willow, tap, fish food, home', async ({ page }) => {
  const t = ftl('de');
  const errors = await start(page);
  await finish(page, [...L1, ...L2]);
  expect(await app<boolean>(page, 'level_unlocked', 'level_3')).toBe(true);
  expect(await app<boolean>(page, 'barrier_open', 'barrier_north_gate'), 'second entry (Q-090)').toBe(true);
  await expect(page.locator('#celebrate')).toBeHidden({ timeout: 15_000 });
  const place = await app<string>(page, 'animal_hiding_place', 'goldfish');
  expect(place).toBe('loc_willow');
  // walk in from level 2 through the opened construction fence
  await goto(page, 20.5, 53.5);
  // 1. the board: riddle, fish food and the bowl hint
  await standFacing(page, 9.3, 70.5, 10.5, 70.5);
  expect(await app<string>(page, 'target_key')).toBe('info_board:goldfish');
  await settle(page);
  await expect(page.locator('#panel-text')).toHaveText(t[`mission-goldfish-riddle-${place}-klasse1`]);
  await expect(page.locator('#panel-hint')).toHaveText(t['mission-goldfish-bowl-hint-klasse1']);
  expect(await page.locator('#panel').innerText()).not.toMatch(RAW_KEY);
  await page.keyboard.press('Escape');
  // 2. fish food from storage 3 first: shown without a bowl, the fish cannot come (RESC-018)
  await standFacing(page, 5.05, 59.5, 5.05, 60.66); // the row in front of storage 3 (Q-181)
  expect(await app<string>(page, 'target_key')).toBe('food_box:fish_food');
  await settle(page);
  await page.locator('#take').click();
  expect(await app<string>(page, 'carry_food')).toBe('fish_food');
  await approach(page, 'goldfish');
  expect(await app<string>(page, 'target_key')).toBe('animal:goldfish');
  await page.keyboard.press('KeyE');
  await expect(page.locator('#bubble')).toHaveText(t['mission-goldfish-needs-bowl']);
  expect(await app<string>(page, 'animal_state', 'goldfish')).toBe('escaped');
  // 3. the bowl in the zookeeper house (enterable, roof hidden inside: PLAY-028)
  await standFacing(page, -7.5, 64.5, -8.5, 64.5);
  expect(await app<string>(page, 'player_inside')).toBe('zookeeper_house_3');
  expect(await app<string>(page, 'target_key')).toBe('item:fish_bowl');
  await page.keyboard.press('KeyE');
  expect(await app<string>(page, 'carry_bowl')).toBe('empty');
  // RESC-020: the HUD shows the bowl and the food in the pocket
  await nextFrames(page, 3);
  await expect(page.locator('#hud-bowl')).toBeVisible();
  await expect(page.locator('#hud-carry .word')).toHaveText(t['food-fish_food']);
  // (an empty bowl does not work — RESC-019 in zoo-core; at a bank the fill target comes
  // first, so the child fills the bowl there instead)
  // 4. fill it at the tap next to the house door
  await standFacing(page, -5.5, 59.5, -5.5, 60.9);
  expect(await app<string>(page, 'target_key')).toBe('water:tap_l3');
  await page.keyboard.press('KeyE');
  expect(await app<string>(page, 'carry_bowl')).toBe('water');
  await expect(page.locator('#bubble')).toHaveText(t['mission-goldfish-bowl-filled']);
  // 5. feed it from the bank: it jumps into the bowl
  await approach(page, 'goldfish');
  await page.keyboard.press('KeyE');
  await expect(page.locator('#bubble')).toHaveText(t['mission-goldfish-in-bowl']);
  expect(await app<string>(page, 'animal_state', 'goldfish')).toBe('in_bowl');
  expect(await app<string>(page, 'carry_bowl')).toBe('fish');
  expect(await app<string>(page, 'carry_food')).toBe('');
  // out from under the willow onto the lawn: the fish swims in the carried glass bowl
  await goto(page, -15.5, 63.5);
  await page.evaluate(() => {
    const a = window.__zoo!.app;
    a.debug_step(0.5);
    a.zoom(0.5);
  });
  await nextFrames(page, 20);
  await page.screenshot({ path: path.join(shots, 'screenshot_poc_m5b_goldfish_bowl.png') });
  // 6. carefully home (RESC-023: 0.9 × speed), put the bowl on the stone step
  await standFacing(page, 10.5, 68.0, 11.5, 68.0);
  expect(await app<string>(page, 'target_key')).toBe('gate:enc_goldfish');
  await page.keyboard.press('KeyE');
  expect(await app<string>(page, 'animal_state', 'goldfish')).toBe('in_enclosure');
  expect(await app<boolean>(page, 'mission_complete', 'goldfish')).toBe(true);
  expect(await app<string>(page, 'carry_bowl')).toBe('');
  await expect(page.locator('#celebrate-text')).toHaveText(t['mission-goldfish-home']);
  expect(errors).toEqual([]);
});

test('PLAY-028 / PLAY-029: the zookeeper-house roof disappears inside and returns outside', async ({ page }) => {
  const errors = await start(page);
  await finish(page, [...L1, ...L2]);
  await goto(page, -7.5, 59.5);
  await nextFrames(page, 3);
  expect(await app<string>(page, 'player_inside')).toBe('');
  expect(await app<boolean>(page, 'region_hidden', 'zookeeper_house_3')).toBe(false);
  await goto(page, -7.5, 63.5);
  await nextFrames(page, 3); // well within 0.3 s
  expect(await app<string>(page, 'player_inside')).toBe('zookeeper_house_3');
  expect(await app<boolean>(page, 'region_hidden', 'zookeeper_house_3')).toBe(true);
  // other buildings keep their roofs (the storages too, while she is not in them)
  expect(await app<boolean>(page, 'region_hidden', 'food_storage_3')).toBe(false);
  expect(await app<boolean>(page, 'region_hidden', 'barrier_ne_tree')).toBe(true);
  // PLAY-029: the bowl on the table is on screen from every 45° rotation
  for (let k = 0; k < 8; k++) {
    await page.evaluate(() => {
      const a = window.__zoo!.app;
      a.rotate(1);
      a.debug_step(1);
    });
    await nextFrames(page, 2);
    const r = await page.evaluate(() => window.__zoo!.app.player_screen_rect());
    expect(r.length).toBe(4);
    expect(r[0]).toBeGreaterThan(0);
    expect(r[2]).toBeLessThan(1280);
  }
  await goto(page, -7.5, 58.5);
  await nextFrames(page, 3);
  expect(await app<boolean>(page, 'region_hidden', 'zookeeper_house_3')).toBe(false);
  expect(errors).toEqual([]);
});

test('RESC-026 / Q-094: the monkey sits in the crow\'s nest of the pirate ship (4 m)', async ({ page }) => {
  const errors = await start(page);
  await finish(page, [...L1, ...L2]);
  await expect(page.locator('#celebrate')).toBeHidden({ timeout: 15_000 });
  expect(await app<string>(page, 'animal_hiding_place', 'monkey')).toBe('loc_pirate_ship');
  await goto(page, 11.5, 58.0);
  await page.evaluate(() => {
    const a = window.__zoo!.app;
    a.zoom(0.8);
    a.debug_step(2);
  });
  await nextFrames(page, 20);
  expect(await app<number>(page, 'animal_lift', 'monkey')).toBeCloseTo(4, 1);
  await page.screenshot({ path: path.join(shots, 'screenshot_poc_m5b_monkey_ship.png') });
  expect(errors).toEqual([]);
});

test('SAVE-011 / GAME-SAVE v2: a save in level 2 restores the joined zoo (unlocked levels, position)', async ({ page }) => {
  const errors = await start(page);
  await finish(page, L1);
  await goto(page, 30.5, 29.5);
  const before = await page.evaluate(() => [window.__zoo!.app.player_x(), window.__zoo!.app.player_z()]);
  await page.evaluate(() => window.__zoo!.slot.flush());
  const saved = await page.evaluate(() => localStorage.getItem('zoo.save'));
  expect(saved).toContain('"version":2');
  await page.reload();
  await waitFrames(page, 3);
  expect(await app<number>(page, 'player_x')).toBeCloseTo(before[0], 2);
  expect(await app<number>(page, 'player_z')).toBeCloseTo(before[1], 2);
  expect(await app<boolean>(page, 'level_unlocked', 'level_2')).toBe(true);
  expect(await app<boolean>(page, 'mission_complete', 'panda')).toBe(true);
  await nextFrames(page, 2);
  expect(await app<boolean>(page, 'region_hidden', 'barrier_ne_tree')).toBe(true);
  // the walk back into level 1 works after the reload
  await goto(page, 12.5, 29.5);
  expect(errors).toEqual([]);
});

test('QA F12: region culling keeps draw calls near level-1 numbers at the spawn and in level 3', async ({ page }) => {
  await start(page);
  await page.evaluate(() => window.__zoo!.app.debug_step(0.1));
  await nextFrames(page, 3);
  const spawn = await page.evaluate(() => {
    const a = window.__zoo!.app;
    return { dc: a.draw_calls(), inst: a.instances(), culled: a.culled_batches() };
  });
  await finish(page, [...L1, ...L2]);
  await goto(page, -2.5, 70.0); // level-3 ring, west side
  await nextFrames(page, 3);
  const l3 = await page.evaluate(() => {
    const a = window.__zoo!.app;
    return { dc: a.draw_calls(), inst: a.instances(), culled: a.culled_batches() };
  });
  console.log(`draw calls / instances — spawn: ${JSON.stringify(spawn)}, level 3: ${JSON.stringify(l3)}`);
  expect(spawn.culled).toBeGreaterThan(10);
  expect(spawn.dc).toBeLessThan(140);
  expect(l3.dc).toBeLessThan(140);
});
