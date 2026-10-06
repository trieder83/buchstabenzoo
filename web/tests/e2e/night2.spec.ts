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

// LAYOUT-N2-028 (AENV-019): the real terrarium models: the house from the game camera, then each case from
// the hall (zoo view with the roof cut away, and first person through the glass): snake sand + stone, frog sand +
// pool + waterfall, chameleon branches / tree. Review shots go to the shots folder; draw calls are logged.
test('LAYOUT-N2-028: the terrarium models - house outside, the three cases from the hall', async ({ page }) => {
  test.setTimeout(540_000);
  const errors = await start(page);
  await toOpenGate(page);
  await page.evaluate(() => window.__zoo!.app.debug_teleport(-76.5, 25.5));
  fs.mkdirSync(shots, { recursive: true });
  await page.waitForTimeout(6000); // the celebration banners fade
  const stats = async (label: string) => {
    await nextFrames(page, 4);
    const s = await page.evaluate(() => {
      const a = window.__zoo!.app;
      return { draws: a.draw_calls(), instances: a.instances(), triangles: a.triangles() };
    });
    fs.appendFileSync(path.join(shots, 'night2_models_stats.txt'), `${label} ${JSON.stringify(s)}\n`);
    return s;
  };
  const shot = async (name: string) => {
    await nextFrames(page, 6);
    await page.screenshot({ path: path.join(shots, `night2_${name}.png`) });
  };
  await goto(page, -85.5, 33.5);
  await page.keyboard.press('Escape');
  await page.evaluate(() => window.__zoo!.app.debug_look_at(-84.5, 41));
  await stats('outside');
  await shot('models_outside');
  await page.evaluate(() => window.__zoo!.app.debug_look_at_player());
  await goto(page, -85.5, 38.0);
  await shot('models_boards');
  await goto(page, -85.5, 42.5); // in the hall
  await stats('hall');
  const cases: [string, number, number][] = [
    ['snake', -91.35, 43.0],
    ['frog', -77.65, 43.0],
    ['chameleon', -84.5, 47.85],
  ];
  for (const [name, x, z] of cases) {
    await page.evaluate(([px, pz]) => window.__zoo!.app.debug_look_at(px, pz), [x, z]);
    await shot(`case_${name}_zoo`);
  }
  await page.evaluate(() => window.__zoo!.app.debug_look_at_player());
  await page.evaluate(() => window.__zoo!.app.set_view_mode('first_person'));
  // first person at the glass of each case: stand 1.2-2 m in front of it and turn the view with the look drag
  // (view yaw: 0 = north, +90 = west, -90 = east; 0.25 deg per px)
  const stands: [string, number, number, number][] = [
    ['snake', -87.2, 43.0, 90],
    ['frog', -81.8, 43.0, -90],
    ['chameleon', -82.5, 43.8, 0],
  ];
  for (const [name, sx, sz, yaw] of stands) {
    await goto(page, sx, sz);
    const cur = await app<number>(page, 'camera_view_yaw_deg');
    let delta = (yaw - cur) % 360;
    if (delta > 180) delta -= 360;
    if (delta < -180) delta += 360;
    await page.evaluate(([dx, dy]) => window.__zoo!.app.look_drag(dx, dy), [-delta / 0.25, 10]);
    await nextFrames(page, 40); // the first-person view turns smoothly
    await shot(`case_${name}_fp`);
  }
  expect(await app<string>(page, 'player_level')).toBe('night_2');
  expect(errors).toEqual([]);
});
