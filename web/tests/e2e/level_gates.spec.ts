// LAYOUT-037 (GAME-LAYOUT "Gates between the levels"): review screenshots of each level
// transition before and after unlocking — a closed `gate_zoo` behind its story barrier, later
// the open gate (the barrier gone). Shots: art/environment/poc/screenshot_level_gate_*.png.
import path from 'node:path';
import { expect, test, type Page } from '@playwright/test';
import { nextFrames, shots, waitFrames } from './helpers';

test.describe.configure({ timeout: 600_000 });
test.use({ viewport: { width: 1280, height: 720 } });

interface OpeningState {
  kind: string;
  x: number;
  z: number;
  open: number;
  drawn: boolean;
}

/** The level gates (id = its barrier) and where they stand (LAYOUT-036 poses). */
const GATES = [
  // `view`: camera yaw steps (45°, + = counter-clockwise) so the camera looks at the gate's
  // front from the old level (west of gate 1, east of gate 2, south of the north gate)
  { name: 'l1_l2', barrier: 'barrier_ne_tree', x: 25.5, z: 29.5, view: -2 },
  { name: 'l2_l3', barrier: 'barrier_l2_construction', x: 22.5, z: 53.5, view: 2 },
  { name: 'l1_l3', barrier: 'barrier_north_gate', x: -7.5, z: 46.5, view: 0 },
  // the lantern gate of night_1 to the terrarium garden (GAME-LEVEL-NIGHT-2): opens as soon as night_1 is complete
  { name: 'n1_n2', barrier: 'barrier_n1_garden', x: -70.5, z: 26, view: 2 },
];

async function levelGates(page: Page): Promise<OpeningState[]> {
  const all = JSON.parse(await page.evaluate(() => window.__zoo!.app.openings_json())) as OpeningState[];
  return all.filter((o) => o.kind === 'level_gate');
}

async function gate(page: Page, x: number, z: number): Promise<OpeningState> {
  const g = (await levelGates(page)).find((o) => Math.hypot(o.x - x, o.z - z) < 0.1);
  expect(g, `level gate at (${x}, ${z})`).toBeTruthy();
  return g!;
}

/** Runs frames with game time (the presentation eases the open amount per frame). */
async function run(page: Page, seconds: number): Promise<void> {
  await page.evaluate((s) => {
    const a = window.__zoo!.app;
    for (let t = 0; t < s; t += 0.1) a.frame(0.1);
  }, seconds);
}

/** Missions of a level done at once; the night passes and the barriers open next morning. */
async function finish(page: Page, animals: string[]) {
  await page.evaluate((list) => {
    const a = window.__zoo!.app;
    for (const n of list) a.debug_send_home(n);
    a.debug_set_daytime('night');
    for (const n of ['hedgehog', 'bat', 'owl']) a.debug_send_home(n);
    a.debug_next_morning();
  }, animals);
  await nextFrames(page, 3);
}

async function shoot(page: Page, g: (typeof GATES)[number], state: string) {
  await page.evaluate(
    ([x, z, v]) => {
      const a = window.__zoo!.app;
      a.rotate(v);
      a.debug_look_at(x, z);
      for (let k = 0; k < 8; k++) a.frame(0.25); // the camera turns
    },
    [g.x, g.z, g.view] as const,
  );
  await nextFrames(page, 3);
  await page.screenshot({ path: path.join(shots, `screenshot_level_gate_${g.name}_${state}.png`) });
  await page.evaluate((v) => window.__zoo!.app.rotate(-v), g.view);
}

test('LAYOUT-037: level gates are closed behind their barriers, then swing open on unlock', async ({ page }) => {
  const errors: string[] = [];
  page.on('console', (m) => {
    if (m.type() === 'error') errors.push(m.text());
  });
  page.on('pageerror', (e) => errors.push(String(e)));
  await page.addInitScript(() => {
    localStorage.clear();
    localStorage.setItem('zoo.language', 'de');
    localStorage.setItem('zoo.readingLevel', 'klasse1');
  });
  await page.goto('/?seed=4');
  await waitFrames(page, 3);

  const all = await levelGates(page);
  expect(all.length).toBe(GATES.length);
  expect(all.every((o) => o.drawn), 'every level gate has its model').toBe(true);
  for (const g of GATES) {
    expect((await gate(page, g.x, g.z)).open, `${g.name} closed while locked`).toBe(0);
    await shoot(page, g, 'closed');
  }

  // level 1 done → the fallen tree is cleared, the gate to level 2 swings open (measured in
  // the same script run: no browser frames in between)
  const mid = await page.evaluate(() => {
    const a = window.__zoo!.app;
    for (const n of ['zebra', 'hippo', 'panda']) a.debug_send_home(n);
    a.debug_set_daytime('night');
    for (const n of ['hedgehog', 'bat', 'owl']) a.debug_send_home(n);
    a.debug_next_morning();
    a.frame(0.1);
    a.frame(0.1);
    const all = JSON.parse(a.openings_json()) as { kind: string; x: number; open: number }[];
    return all.find((o) => o.kind === 'level_gate' && Math.abs(o.x - 25.5) < 0.1)!.open;
  });
  expect(mid, 'the gate swings (not snaps) open').toBeGreaterThan(0);
  expect(mid).toBeLessThan(1);
  await run(page, 3);
  expect((await gate(page, 25.5, 29.5)).open).toBeGreaterThan(0.99);
  expect(await page.evaluate(() => window.__zoo!.app.region_hidden('barrier_ne_tree'))).toBe(true);
  expect((await gate(page, 22.5, 53.5)).open, 'level 3 still locked').toBe(0);
  await shoot(page, GATES[0], 'open');

  // level 2 done → both level-3 gates open and stay open
  await finish(page, ['koala', 'elephant', 'giraffe', 'lion']);
  await run(page, 3);
  for (const g of GATES) {
    expect((await gate(page, g.x, g.z)).open, `${g.name} open`).toBeGreaterThan(0.99);
  }
  await shoot(page, GATES[1], 'open');
  await shoot(page, GATES[2], 'open');
  // walk through the level-1 → level-3 gate: open and walkable
  const z = await page.evaluate(() => {
    const a = window.__zoo!.app;
    a.debug_look_at_player();
    a.debug_teleport(-7.5, 44.5);
    a.debug_goto(-7.5, 50.5);
    a.debug_step(8);
    return a.player_z();
  });
  expect(z, 'she walks through the open level gate').toBeGreaterThan(49.5);
  expect(errors).toEqual([]);
});

// LAYOUT-042 (GAME-LAYOUT "Gates between the levels", user request 2026-09-28): the street
// continues under the level gate. With the level-1 → level-2 gate open she walks from
// `path_ne` east through the gate into level 2 and never leaves the path (her surface speed
// stays at the path speed, GAME-PLAYER §6 — a grass gap under the gate would slow her down);
// review shot of the open gate with the street through it.
test('LAYOUT-042: the street runs on under the open level-1 → level-2 gate', async ({ page }) => {
  const errors: string[] = [];
  page.on('console', (m) => {
    if (m.type() === 'error') errors.push(m.text());
  });
  page.on('pageerror', (e) => errors.push(String(e)));
  await page.addInitScript(() => {
    localStorage.clear();
    localStorage.setItem('zoo.language', 'de');
    localStorage.setItem('zoo.readingLevel', 'klasse1');
  });
  await page.goto('/?seed=4');
  await waitFrames(page, 3);
  await finish(page, ['zebra', 'hippo', 'panda']);
  await run(page, 3);
  expect((await gate(page, 25.5, 29.5)).open).toBeGreaterThan(0.99);
  // on path_ne west of the fallen tree's place, the camera behind her looking east
  const walk = await page.evaluate(() => {
    const a = window.__zoo!.app;
    a.debug_teleport(18.5, 29.5);
    a.rotate(-2);
    for (let k = 0; k < 8; k++) a.frame(0.25);
    a.debug_goto(30.5, 29.5);
    const out: { x: number; v: number }[] = [];
    for (let k = 0; k < 80; k++) {
      a.debug_step(0.1);
      out.push({ x: a.player_x(), v: a.surface_speed() });
    }
    return out;
  });
  expect(walk[walk.length - 1].x, 'she walks through the gate').toBeGreaterThan(29.5);
  const slow = walk.filter((w) => w.v < 1.9);
  expect(slow, 'on the path all the way (no grass under the gate)').toEqual([]);
  // review shot: back on path_ne, the open gate and the street through it
  await page.evaluate(() => {
    const a = window.__zoo!.app;
    // on the former barrier cells (the fallen tree lay here), 3 m before the gate; the
    // celebration bubbles and confetti are gone after a few seconds
    a.debug_teleport(22.5, 29.5);
    a.debug_face_point(25.5, 29.5);
    for (let k = 0; k < 60; k++) a.frame(0.25);
    // the review shot is about the ground: the morning banner and the last celebration
    // bubble (host overlays with their own real-time timers) are hidden for it
    for (const id of ['celebrate', 'night-banner', 'bubble']) {
      const el = document.getElementById(id);
      if (el) el.style.visibility = 'hidden';
    }
  });
  await nextFrames(page, 3);
  await page.screenshot({ path: path.join(shots, 'screenshot_level_gate_street.png') });
  expect(errors).toEqual([]);
});
