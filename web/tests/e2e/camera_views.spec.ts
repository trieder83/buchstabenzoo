// GAME-CAMERA-VIEWS: first person (toggle, `V`) and look-around (hold, `F` / right mouse)
// in the real browser — CAMV-012 (first person: walk, turn, read a board), CAMV-013
// (look-around hold and release), CAMV-014 (draw calls per view). Review shots go to
// art/environment/poc/screenshot_camera_{firstperson,lookaround}.png.
import path from 'node:path';
import { expect, test, type CDPSession, type Page } from '@playwright/test';
import { goto, nextFrames, shots, waitFrames, START_URL } from './helpers';

test.describe.configure({ timeout: 180_000 });
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

const cam = (page: Page) =>
  page.evaluate(() => {
    const a = window.__zoo!.app;
    return {
      mode: a.view_mode(),
      blend: a.camera_blend(),
      eye: a.camera_eye(),
      // feet on the ground surface (GAME-PLAYER 8): eye heights are measured from them
      feet: a.player_foot_y(),
      pitch: a.camera_pitch_deg(),
      yaw: a.camera_view_yaw_deg(),
      zooYaw: a.camera_target_yaw_deg(),
      distance: a.camera_distance(),
      drawn: a.player_drawn(),
      fog: a.camera_fog(),
      x: a.player_x(),
      z: a.player_z(),
      time: a.time(),
    };
  });

/** Waits until the glide has reached `blend` (0 or 1); returns the game time it took. */
async function glide(page: Page, blend: number): Promise<number> {
  const t0 = await page.evaluate(() => window.__zoo!.app.time());
  await page.waitForFunction(
    (b) => Math.abs(window.__zoo!.app.camera_blend() - b) < 1e-3,
    blend,
    { polling: 'raf', timeout: 10_000 },
  );
  return (await page.evaluate(() => window.__zoo!.app.time())) - t0;
}

/** Fraction of "sky" pixels (blue-ish, bright) in the top `rows` fraction of the frame. */
async function skyFraction(page: Page, rows = 0.12): Promise<number> {
  return page.evaluate((rows) => {
    const a = window.__zoo!.app;
    const canvas = document.getElementById('game') as HTMLCanvasElement;
    const gl = canvas.getContext('webgl2')!;
    const W = canvas.width;
    const H = canvas.height;
    a.frame(0);
    const n = Math.max(1, Math.round(H * rows));
    const px = new Uint8Array(W * n * 4);
    gl.readPixels(0, H - n, W, n, gl.RGBA, gl.UNSIGNED_BYTE, px);
    let sky = 0;
    for (let i = 0; i < px.length; i += 4) {
      const [r, g, b] = [px[i], px[i + 1], px[i + 2]];
      if (b > r + 8 && b + 12 >= g && b > 150) sky += 1;
    }
    return sky / (W * n);
  }, rows);
}

test('CAMV-012: first person — toggle, walk, turn and read the zebra board', async ({ page }) => {
  const errors = await start(page);
  // on the ring path north-east of the zebra board (out of its range; the board stands south
  // of the zebra gate since Q-171), facing north
  await goto(page, -6.9, 14.0);
  await goto(page, -6.9, 15.5);
  const zoo = await cam(page);
  expect(zoo.mode).toBe('zoo');
  expect(zoo.drawn).toBe(true);

  await page.keyboard.press('KeyV');
  const took = await glide(page, 1);
  console.log(`first person glide ${took.toFixed(2)} s`);
  expect(took).toBeLessThanOrEqual(0.55);
  let c = await cam(page);
  expect(c.mode).toBe('first_person');
  expect(c.drawn).toBe(false);
  expect(c.eye[1] - c.feet).toBeCloseTo(1.1, 2);
  expect(Math.abs(c.pitch)).toBeLessThan(0.5);
  expect(c.fog[1]).toBeCloseTo(20.8, 3); // fog end +30 % (2026-09-27)
  await expect(page.locator('#view-btn')).toHaveAttribute('aria-pressed', 'true');
  expect(await skyFraction(page)).toBeGreaterThan(0.3);
  await page.screenshot({ path: path.join(shots, 'screenshot_camera_firstperson.png') });

  // walk forward (north, where she looks): z grows, x stays, no head bob
  await page.keyboard.down('KeyW');
  await page.waitForFunction((z0) => window.__zoo!.app.player_z() > z0 + 0.4, c.z, { polling: 'raf' });
  await page.keyboard.up('KeyW');
  const walked = await cam(page);
  expect(Math.abs(walked.x - c.x)).toBeLessThan(0.1);
  expect(walked.eye[1] - walked.feet).toBeCloseTo(1.1, 2);
  // level with the board (it is 90° to the left): not available while looking north
  await page.evaluate(() => {
    const a = window.__zoo!.app;
    a.debug_teleport(-6.9, 10.5);
  });
  await page.waitForFunction(() => document.getElementById('panel')!.hidden, null, { polling: 'raf', timeout: 10_000 });
  expect(await page.evaluate(() => window.__zoo!.app.target_key())).toBe('');

  // turn left (west) with the arrow key until the board is in front: the riddle opens
  await page.keyboard.down('ArrowLeft');
  await page.waitForFunction(() => window.__zoo!.app.target_key() === 'info_board:zebra', null, {
    polling: 'raf',
    timeout: 10_000,
  });
  // keep turning until she looks (about) straight west at it
  await page.waitForFunction(() => window.__zoo!.app.camera_view_yaw_deg() > 80, null, { polling: 'raf' });
  await page.keyboard.up('ArrowLeft');
  await nextFrames(page, 10);
  c = await cam(page);
  expect(Math.abs(c.x + 6.9)).toBeLessThan(0.05); // turning does not move her
  expect(await page.evaluate(() => window.__zoo!.app.target_key())).toBe('info_board:zebra');
  await page.waitForFunction(() => !document.getElementById('panel')!.hidden, null, { polling: 'raf', timeout: 10_000 });
  await expect(page.locator('#panel-text')).toBeVisible();
  await page.screenshot({ path: path.join(shots, 'screenshot_camera_firstperson_board.png') });

  // mouse drag turns smoothly (no 45° step) — and the panel closes when looking away
  const y0 = (await cam(page)).yaw;
  await page.mouse.move(640, 400);
  await page.mouse.down();
  await page.mouse.move(1000, 400, { steps: 12 });
  await page.mouse.up();
  await page.waitForFunction((y) => Math.abs(window.__zoo!.app.camera_view_yaw_deg() - y) > 20, y0, { polling: 'raf' });
  await nextFrames(page, 40);
  const turned = (await cam(page)).yaw;
  const dYaw = ((y0 - turned + 540) % 360) - 180; // right drag = clockwise = yaw decreases
  expect(dYaw).toBeGreaterThan(80);
  expect(dYaw).toBeLessThan(100); // 360 px × 0.25°/px = 90°
  await page.waitForFunction(() => document.getElementById('panel')!.hidden, null, { polling: 'raf', timeout: 10_000 });

  // back to the zoo view: body drawn, 45° step and zoom unchanged
  await page.keyboard.press('KeyV');
  await glide(page, 0);
  c = await cam(page);
  expect(c.mode).toBe('zoo');
  expect(c.drawn).toBe(true);
  expect(c.zooYaw).toBeCloseTo(zoo.zooYaw, 3);
  expect(c.distance).toBeCloseTo(zoo.distance, 3);
  // the setting is stored (GAME-CAMERA-VIEWS 9)
  expect(await page.evaluate(() => localStorage.getItem('zoo.view'))).toBe('zoo');
  expect(errors).toEqual([]);
});

test('CAMV-013: look-around while F is held, back to the zoo view on release', async ({ page }) => {
  const errors = await start(page);
  await goto(page, -2.5, 20.5); // the ring path, looking north over the zoo
  const zoo = await cam(page);
  await page.keyboard.down('KeyF');
  const took = await glide(page, 1);
  console.log(`look-around glide ${took.toFixed(2)} s`);
  expect(took).toBeLessThanOrEqual(0.55);
  let c = await cam(page);
  expect(c.mode).toBe('look_around');
  expect(c.drawn).toBe(true);
  expect(c.eye[1] - c.feet).toBeCloseTo(1.6, 1);
  const back = Math.hypot(c.eye[0] - c.x, -c.eye[2] - c.z);
  expect(back).toBeCloseTo(3.5, 1);
  expect(c.pitch).toBeCloseTo(-12, 0);
  expect(await skyFraction(page)).toBeGreaterThan(0.3);
  // a mouse drag turns the look-around view smoothly
  await page.mouse.move(640, 400);
  await page.mouse.down();
  await page.mouse.move(520, 400, { steps: 6 });
  await page.mouse.up();
  await nextFrames(page, 40);
  c = await cam(page);
  const dYaw = ((c.yaw - zoo.yaw + 540) % 360) - 180; // left drag = counter-clockwise
  expect(dYaw).toBeGreaterThan(20);
  expect(dYaw).toBeLessThan(40);
  await page.screenshot({ path: path.join(shots, 'screenshot_camera_lookaround.png') });
  // she can walk while looking around
  await page.keyboard.down('KeyW');
  await page.waitForFunction(
    ([x0, z0]) => Math.hypot(window.__zoo!.app.player_x() - x0, window.__zoo!.app.player_z() - z0) > 0.3,
    [c.x, c.z],
    { polling: 'raf' },
  );
  await page.keyboard.up('KeyW');
  await page.keyboard.up('KeyF');
  const back0 = await glide(page, 0);
  expect(back0).toBeLessThanOrEqual(0.55);
  c = await cam(page);
  expect(c.mode).toBe('zoo');
  expect(c.zooYaw).toBeCloseTo(zoo.zooYaw, 3);
  expect(c.distance).toBeCloseTo(zoo.distance, 3);
  expect(c.yaw).toBeCloseTo(zoo.yaw, 1);
  // right mouse button holds look-around too
  await page.mouse.move(900, 300);
  await page.mouse.down({ button: 'right' });
  await nextFrames(page, 3);
  expect((await cam(page)).mode).toBe('look_around');
  await page.mouse.up({ button: 'right' });
  await nextFrames(page, 3);
  expect((await cam(page)).mode).toBe('zoo');
  // never stored
  expect(await page.evaluate(() => localStorage.getItem('zoo.view'))).not.toBe('look_around');
  expect(errors).toEqual([]);
});

test('CAMV-014: the close views need fewer draw calls than the zoo view at 20 m', async ({ page }) => {
  const errors = await start(page);
  const measure = async () =>
    page.evaluate(() => {
      const a = window.__zoo!.app;
      a.frame(0);
      return { dc: a.draw_calls(), culled: a.culled_batches() };
    });
  const rows: { spot: { x: number; z: number }; zoo: { dc: number; culled: number }; fp: { dc: number; culled: number }; la: { dc: number; culled: number } }[] = [];
  const l2 = await page.evaluate(() => window.__zoo!.app.level_spawn('level_2'));
  for (const spot of [
    { x: 0.5, z: 4.5 }, // level-1 spawn
    { x: -2.5, z: 20.5 }, // level-1 ring, north
    { x: l2[0], z: l2[1] }, // level-2 spawn
  ]) {
    await page.evaluate(({ x, z }) => {
      const a = window.__zoo!.app;
      a.debug_teleport(x, z);
      a.zoom(100); // 20 m
      a.debug_step(0.05);
    }, spot);
    await nextFrames(page, 3);
    const zoo = await measure();
    await page.keyboard.press('KeyV');
    await glide(page, 1);
    const fp = await measure();
    await page.keyboard.press('KeyV');
    await glide(page, 0);
    await page.keyboard.down('KeyF');
    await glide(page, 1);
    const la = await measure();
    await page.keyboard.up('KeyF');
    await glide(page, 0);
    rows.push({ spot, zoo, fp, la });
  }
  console.log(
    `draw calls:\n${rows
      .map((r) => `(${r.spot.x}, ${r.spot.z}): zoo ${r.zoo.dc} (culled ${r.zoo.culled}), first person ${r.fp.dc} (culled ${r.fp.culled}), look-around ${r.la.dc} (culled ${r.la.culled})`)
      .join('\n')}`,
  );
  for (const r of rows) {
    expect(r.fp.dc).toBeLessThan(r.zoo.dc);
    expect(r.la.dc).toBeLessThan(r.zoo.dc);
  }
  expect(errors).toEqual([]);
});

type Pt = { x: number; y: number; id: number };
async function touch(cdp: CDPSession, type: 'touchStart' | 'touchMove' | 'touchEnd', points: Pt[]) {
  await cdp.send('Input.dispatchTouchEvent', { type, touchPoints: points });
}

test.describe('touch phone portrait', () => {
  test.use({ viewport: { width: 412, height: 892 }, deviceScaleFactor: 2.625, hasTouch: true, isMobile: true });

  test('CAMV-019: the 👓 button sits in the right-thumb zone and toggles first person while the left thumb walks', async ({ page }) => {
    const errors = await start(page);
    await goto(page, 0.5, 4.5);
    const cdp = await page.context().newCDPSession(page);
    // first touch: touch controls on
    await touch(cdp, 'touchStart', [{ x: 300, y: 300, id: 1 }]);
    await touch(cdp, 'touchEnd', []);
    await nextFrames(page, 2);
    const btn = page.locator('#view-btn');
    await expect(btn).toBeVisible();
    const b = (await btn.boundingBox())!;
    const act = await page.evaluate(() => {
      const e = document.getElementById('act')!;
      const cs = getComputedStyle(e);
      return { right: parseFloat(cs.right), bottom: parseFloat(cs.bottom), size: parseFloat(cs.width) };
    });
    // bottom right, above the interact button, ≥ 64 px, on screen, in the right half
    expect(b.width).toBeGreaterThanOrEqual(64);
    expect(b.x).toBeGreaterThan(412 / 2);
    expect(b.x + b.width).toBeLessThanOrEqual(412);
    expect(b.y + b.height).toBeLessThanOrEqual(892 - act.bottom - act.size + 1); // above #act
    expect(b.y).toBeGreaterThan(892 * 0.6); // lower part: thumb zone

    // left thumb walks (hold the stick up), right thumb taps 👓: first person, still walking
    const stick = { x: 90, y: 760, id: 2 };
    await touch(cdp, 'touchStart', [stick]);
    await touch(cdp, 'touchMove', [{ ...stick, y: stick.y - 50 }]);
    await nextFrames(page, 5);
    const tap = { x: b.x + b.width / 2, y: b.y + b.height / 2, id: 3 };
    await touch(cdp, 'touchStart', [{ ...stick, y: stick.y - 50 }, tap]);
    await touch(cdp, 'touchEnd', [tap]);
    await page.waitForFunction(() => window.__zoo!.app.view_mode() === 'first_person', null, { polling: 'raf', timeout: 5_000 });
    await expect(btn).toHaveAttribute('aria-pressed', 'true');
    const z0 = await page.evaluate(() => window.__zoo!.app.player_z());
    await nextFrames(page, 20);
    const z1 = await page.evaluate(() => window.__zoo!.app.player_z());
    expect(Math.abs(z1 - z0), 'the left thumb keeps walking').toBeGreaterThan(0.1);
    await glide(page, 1);
    await page.screenshot({ path: path.join(shots, 'screenshot_camera_firstperson_touch.png') });
    // tap again: back to the zoo view, like V
    await touch(cdp, 'touchStart', [{ ...stick, y: stick.y - 50 }, tap]);
    await touch(cdp, 'touchEnd', [tap]);
    await page.waitForFunction(() => window.__zoo!.app.view_mode() === 'zoo', null, { polling: 'raf', timeout: 5_000 });
    await touch(cdp, 'touchEnd', []);
    await expect(btn).toHaveAttribute('aria-pressed', 'false');
    expect(errors).toEqual([]);
  });
});

test('CAMV-022: inside the zookeeper house in first person the roof and its ceiling stay; the zoo view hides the roof', async ({ page }) => {
  const errors = await start(page);
  // into the house (door in the east facade), in the middle of the room
  await goto(page, -7.5, 2.5);
  await goto(page, -11.5, 2.5);
  expect(await page.evaluate(() => window.__zoo!.app.player_inside())).toBe('zookeeper_house_1');
  await page.evaluate(() => window.__zoo!.app.frame(0));
  expect(await page.evaluate(() => window.__zoo!.app.region_hidden('zookeeper_house_1')), 'zoo view: roof hidden').toBe(true);
  // face the door wall (east; the other walls have windows whose panes show a painted sky)
  await page.evaluate(() => window.__zoo!.app.debug_face_point(-7, 2.5));
  await page.keyboard.press('KeyV');
  await glide(page, 1);
  // look up as far as allowed: the ceiling fills the top of the view, no sky
  await page.evaluate(() => {
    const a = window.__zoo!.app;
    a.look_drag(0, -2000);
    a.frame(0);
  });
  expect(await page.evaluate(() => window.__zoo!.app.region_hidden('zookeeper_house_1')), 'first person: roof drawn').toBe(false);
  expect(await skyFraction(page, 0.1)).toBeLessThan(0.02);
  await page.screenshot({ path: path.join(shots, 'screenshot_camera_ceiling.png') });
  await page.keyboard.press('KeyV');
  await glide(page, 0);
  await page.evaluate(() => window.__zoo!.app.frame(0));
  expect(await page.evaluate(() => window.__zoo!.app.region_hidden('zookeeper_house_1')), 'back in the zoo view: hidden again').toBe(true);
  expect(errors).toEqual([]);
});

// LAYOUT-043 (GAME-LAYOUT "Enterable buildings", user request 2026-09-28): she walks (keyboard)
// from the ring through the door into the level-1 food storage; the door opens, she stands on
// the floor inside between the unlabelled stock boxes (Q-194, never a target), the roof, the
// upper walls and the "Futter" board hide in the zoo view and come back in first person
// (CAMV-022); she walks back out through the door and takes the grass from its box in the row
// outside (Q-181 answered: the labelled boxes stay outside).
test('LAYOUT-043: into the food storage and out — door opens, roof hidden in the zoo view, kept in first person', async ({ page }) => {
  const errors = await start(page);
  const door = async () => {
    const all = JSON.parse(await page.evaluate(() => window.__zoo!.app.openings_json())) as {
      kind: string;
      x: number;
      z: number;
      open: number;
    }[];
    return all.find((o) => o.kind === 'door' && Math.hypot(o.x - 0.5, o.z - 11.5) < 0.1)!;
  };
  /** Holds the WASD key that walks along a level direction (camera yaw, camera.rs) for `s` seconds. */
  const walk = async (dx: number, dz: number, s: number) =>
    page.evaluate(
      ([ddx, ddz, secs]) => {
        const a = window.__zoo!.app;
        const yaw = a.camera_target_yaw_deg() * (Math.PI / 180);
        const fwd = [-Math.sin(yaw), Math.cos(yaw)];
        const right = [Math.cos(yaw), Math.sin(yaw)];
        const keys: [string, number[]][] = [
          ['KeyW', fwd],
          ['KeyS', [-fwd[0], -fwd[1]]],
          ['KeyD', right],
          ['KeyA', [-right[0], -right[1]]],
        ];
        keys.sort((p, q) => q[1][0] * ddx + q[1][1] * ddz - (p[1][0] * ddx + p[1][1] * ddz));
        a.key(keys[0][0], true);
        for (let k = 0; k < Math.round(secs * 10); k++) a.frame(0.1);
        a.key(keys[0][0], false);
        a.frame(0.1);
        return { x: a.player_x(), z: a.player_z() };
      },
      [dx, dz, s] as const,
    );
  await goto(page, 0.5, 9.0);
  expect((await door()).open, 'shut while she is away').toBe(0);
  // in through the door (walking north)
  let p = await walk(0, 1, 1.2);
  expect(p.z, 'at the door').toBeGreaterThan(10.4);
  expect((await door()).open, 'the door opens as she comes').toBeGreaterThan(0.5);
  p = await walk(0, 1, 2.5);
  expect(p.z, 'inside, stopped in front of the back wall band').toBeGreaterThan(14.5);
  expect(p.z).toBeLessThan(16.0);
  expect(await page.evaluate(() => window.__zoo!.app.player_inside())).toBe('food_storage');
  expect(await page.evaluate(() => window.__zoo!.app.player_foot_y())).toBeGreaterThan(0.06);
  // the stock boxes along the walls are decoration: never a target (Q-194)
  await page.evaluate(() => window.__zoo!.app.debug_face_point(-1.88, 16.44));
  await page.evaluate(() => window.__zoo!.app.frame(0));
  expect(await page.evaluate(() => window.__zoo!.app.target_key())).not.toMatch(/^food_box:/);
  expect(await page.evaluate(() => window.__zoo!.app.region_hidden('food_storage')), 'zoo view: roof hidden').toBe(true);
  await nextFrames(page, 3);
  await page.screenshot({ path: path.join(shots, 'screenshot_storage_inside_zoo.png') });
  // first person, looking back at the door and a little up: roof, ceiling and the open door
  await page.evaluate(() => window.__zoo!.app.debug_face_point(0.5, 11.0));
  await nextFrames(page, 3);
  await page.keyboard.press('KeyV');
  await glide(page, 1);
  await page.evaluate(() => {
    const a = window.__zoo!.app;
    a.look_drag(0, -250);
    a.frame(0);
  });
  expect(await page.evaluate(() => window.__zoo!.app.region_hidden('food_storage')), 'first person: roof drawn').toBe(false);
  await nextFrames(page, 3);
  await page.screenshot({ path: path.join(shots, 'screenshot_storage_inside_fp.png') });
  // looking up as far as allowed: the ceiling, no sky
  await page.evaluate(() => {
    const a = window.__zoo!.app;
    a.look_drag(0, -2000);
    a.frame(0);
  });
  expect(await skyFraction(page, 0.1)).toBeLessThan(0.02);
  await page.keyboard.press('KeyV');
  await glide(page, 0);
  // out again through the door (walking south): the roof is back
  await goto(page, p.x, p.z);
  p = await walk(0, -1, 3.5);
  expect(p.z, 'walked out through the door').toBeLessThan(10.0);
  expect(await page.evaluate(() => window.__zoo!.app.player_inside())).toBe('');
  expect(await page.evaluate(() => window.__zoo!.app.region_hidden('food_storage'))).toBe(false);
  // the grass from its box in the row outside
  await goto(page, -1.5, 9.6);
  await page.evaluate(() => window.__zoo!.app.debug_face_point(-1.5, 10.66));
  await page.evaluate(() => window.__zoo!.app.frame(0));
  expect(await page.evaluate(() => window.__zoo!.app.target_key())).toBe('food_box:grass');
  expect(await page.evaluate(() => window.__zoo!.app.take_food('grass'))).toBe(true);
  expect(await page.evaluate(() => window.__zoo!.app.carry_food())).toBe('grass');
  expect(errors).toEqual([]);
});
