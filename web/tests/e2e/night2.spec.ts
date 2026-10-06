// GAME-LEVEL-NIGHT-2 e2e (LAYOUT-N2-012, LAYOUT-N2-013): the lantern gate opens when night_1 is
// complete, the child walks west through it into the terrarium garden, the compass strip shows the
// three new animals; a pair led into its glass terrarium is home and the garden keeps rendering.
import fs from 'node:fs';
import path from 'node:path';
import { expect, test, type Page } from '@playwright/test';
import { goto, nextFrames, shots, waitFrames, START_URL } from './helpers';

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

/** Level 1 and night_1 complete at night (debug): the garden gate is open. */
async function toOpenGate(page: Page) {
  for (const a of ['zebra', 'hippo', 'panda']) expect(await app<boolean>(page, 'debug_send_home', a)).toBe(true);
  await nextFrames(page, 2);
  await page.evaluate(() => {
    document.getElementById('celebrate')!.hidden = true;
  });
  await page.evaluate(() => window.__zoo!.app.debug_step(18.5));
  await nextFrames(page, 2);
  expect(await app<string>(page, 'daytime')).toBe('night');
  for (const a of ['hedgehog', 'bat', 'owl']) expect(await app<boolean>(page, 'debug_send_home', a)).toBe(true);
  await nextFrames(page, 2);
}

test('LAYOUT-N2-012: night_1 complete → through the lantern gate → the terrarium garden, 3 icons in the compass', async ({ page }) => {
  const errors = await start(page);
  await toOpenGate(page);
  // the strip lists the three terrarium animals (night_2 is next, night_1 is done)
  const strip = JSON.parse(await app<string>(page, 'compass_json'));
  expect(strip.level).toBe('night_2');
  expect(strip.animals.map((a: { id: string }) => a.id).sort()).toEqual(['chameleon', 'poison_dart_frog', 'snake']);
  // west along the gate path, through the open gate, onto the entry path of night_2
  await page.evaluate(() => window.__zoo!.app.debug_teleport(-62.5, 26));
  await goto(page, -76.5, 25.5);
  expect(await app<string>(page, 'player_level')).toBe('night_2');
  expect(await app<number>(page, 'player_x')).toBeLessThan(-73);
  await nextFrames(page, 3);
  expect(errors).toEqual([]);
});

test('LAYOUT-N2-013: the terrarium hall renders and a led pair is home behind the glass', async ({ page }) => {
  const errors = await start(page);
  await toOpenGate(page);
  await page.evaluate(() => window.__zoo!.app.debug_teleport(-76.5, 25.5));
  await goto(page, -85.5, 41.5); // in the visitor hall, in front of the glass fronts
  await nextFrames(page, 3);
  expect(await app<string>(page, 'player_level')).toBe('night_2');
  expect(await app<boolean>(page, 'debug_send_home', 'snake')).toBe(true);
  await nextFrames(page, 3);
  expect(await app<boolean>(page, 'mission_complete', 'snake')).toBe(true);
  fs.mkdirSync(shots, { recursive: true });
  await page.screenshot({ path: path.join(shots, 'night2_terrarium_hall.png') });
  const strip = JSON.parse(await app<string>(page, 'compass_json'));
  expect(strip.animals.find((a: { id: string }) => a.id === 'snake').home).toBe(true);
  expect(errors).toEqual([]);
});

test('LAYOUT-N2-023: the round terrarium house shows its pictograms (outside and in the hall)', async ({ page }) => {
  const errors = await start(page);
  await toOpenGate(page);
  await page.evaluate(() => window.__zoo!.app.debug_teleport(-76.5, 25.5));
  fs.mkdirSync(shots, { recursive: true });
  await page.waitForTimeout(6000); // the celebration banners fade
  await goto(page, -85.5, 35.5); // on the street in front of the door
  await nextFrames(page, 6);
  await page.screenshot({ path: path.join(shots, 'night2_round_house_outside.png') });
  await page.keyboard.press('Escape');
  await goto(page, -85.5, 38.0); // at the boards next to the door
  await nextFrames(page, 6);
  await page.screenshot({ path: path.join(shots, 'night2_round_house_boards.png') });
  await goto(page, -85.5, 43.5); // in the round hall
  await nextFrames(page, 8);
  await page.screenshot({ path: path.join(shots, 'night2_round_house_hall.png') });
  expect(await app<string>(page, 'player_level')).toBe('night_2');
  expect(errors).toEqual([]);
});
