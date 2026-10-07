// GAME-CART (P3): the golf carts in the browser — locked feedback, boarding, driving, collision,
// camera, get-out, parking check, headlights, small screens (CART-002/003/004/008/010/019/020/
// 021/028, CAMV-024).
import { expect, test, type Page } from '@playwright/test';
import { ftl, nextFrames, shots, waitFrames, START_URL } from './helpers';

type Cart = {
  id: string;
  x: number;
  z: number;
  yaw: number;
  speed: number;
  lock: string;
  seated: boolean;
  lights: boolean;
};

async function start(page: Page, w = 1280, h = 720) {
  await page.setViewportSize({ width: w, height: h });
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
  await page.evaluate(() => {
    document.getElementById('celebrate')!.hidden = true;
  });
  return errors;
}

const carts = async (page: Page) =>
  JSON.parse(await page.evaluate(() => window.__zoo!.app.carts_json())) as Cart[];

/** Stands at the boarding cell of cart l1 (2.5, 3.5), facing the cart. */
async function atCart(page: Page) {
  await page.evaluate(() => {
    const a = window.__zoo!.app;
    a.debug_teleport(2.5, 3.5);
    a.debug_face_point(4.0, 3.5);
  });
  await nextFrames(page, 3);
}

async function drive(page: Page, code: string, seconds: number) {
  await page.evaluate(
    ([c, s]) => {
      const a = window.__zoo!.app;
      a.key(c as string, true);
      a.debug_step(s as number);
      a.key(c as string, false);
    },
    [code, seconds],
  );
}

test('CART-021 / CART-011: a locked cart shows the bubble, the 🔒 badge and points at the key', async ({ page }) => {
  const errors = await start(page);
  const t = ftl('de');
  await atCart(page);
  expect(await page.evaluate(() => window.__zoo!.app.target_key())).toBe('cart:cart_l1');
  expect(await page.evaluate(() => window.__zoo!.app.target_lock())).toBe('no_key');
  const r = JSON.parse(await page.evaluate(() => window.__zoo!.app.interact())) as { kind: string; key: string };
  expect(r.kind).toBe('cart_locked');
  expect(r.key).toBe('cart-locked');
  expect((await carts(page)).every((c) => !c.seated)).toBe(true);
  // the hint marker points at the next key step (the note) for 12 s
  const hint = JSON.parse(await page.evaluate(() => window.__zoo!.app.hint_json())) as { kind: string; left: number };
  expect(hint.kind).toBe('note');
  expect(hint.left).toBeGreaterThan(10);
  // the UI path: the bubble with 🔒🔑 and the badge on the button
  await page.keyboard.press('KeyE');
  await nextFrames(page, 3);
  await expect(page.locator('#bubble')).toContainText('🔒');
  await expect(page.locator('#bubble')).toContainText(t['cart-locked']);
  expect(errors).toEqual([]);
});

test('CART-020: with the key but a closed level the cart says cart-closed-level, no hint', async ({ page }) => {
  await start(page);
  await page.evaluate(() => {
    const a = window.__zoo!.app;
    a.debug_give_cart_key();
    a.debug_teleport(28.5, 30.5);
    a.debug_face_point(28.5, 31.7);
  });
  await nextFrames(page, 3);
  expect(await page.evaluate(() => window.__zoo!.app.target_lock())).toBe('closed_level');
  const r = JSON.parse(await page.evaluate(() => window.__zoo!.app.interact())) as { kind: string; reason: string };
  expect(r.kind).toBe('cart_locked');
  expect(r.reason).toBe('closed_level');
});

test('CART-002/003/004/008 + CAMV-024: board, drive, slide along the bench, zoo view only, get out', async ({ page }) => {
  const errors = await start(page);
  await page.evaluate(() => {
    const a = window.__zoo!.app;
    a.debug_give_cart_key();
    a.toggle_first_person(); // first person stored
  });
  expect(await page.evaluate(() => window.__zoo!.app.view_mode())).toBe('first_person');
  await atCart(page);
  const b = JSON.parse(await page.evaluate(() => window.__zoo!.app.interact())) as { kind: string };
  expect(b.kind).toBe('cart_boarded');
  await nextFrames(page, 30);
  expect(await page.evaluate(() => window.__zoo!.app.driving())).toBe(true);
  // only the zoo view while driving, the view keys do nothing, the camera zooms out +3 m
  expect(await page.evaluate(() => window.__zoo!.app.view_mode())).toBe('zoo');
  await page.keyboard.press('KeyV');
  await page.keyboard.down('KeyF');
  await page.keyboard.up('KeyF');
  expect(await page.evaluate(() => window.__zoo!.app.view_mode())).toBe('zoo');
  expect(await page.evaluate(() => window.__zoo!.app.camera_extra_distance())).toBeCloseTo(3, 1);
  await expect(page.locator('#view-btn')).toBeHidden();
  expect(await page.evaluate(() => window.__zoo!.app.target_kind())).toBe('get_out');
  // the stored view is kept for the settings
  expect(await page.evaluate(() => window.__zoo!.app.saved_view_mode())).toBe('first_person');

  // drive north along the plaza: the speed reaches the path speed (<= 4.5 m/s)
  const from = (await carts(page))[0];
  await drive(page, 'KeyW', 1.4);
  const mid = (await carts(page))[0];
  expect(mid.z).toBeGreaterThan(from.z + 2);
  expect(mid.speed).toBeLessThanOrEqual(4.6);
  // against the bench / the rock hill / the zookeeper house: never overlapping a solid
  for (const code of ['KeyD', 'KeyA', 'KeyS', 'KeyD']) {
    for (let k = 0; k < 6; k++) {
      await drive(page, code, 0.5);
      expect(await page.evaluate(() => window.__zoo!.app.debug_cart_overlaps(0))).toBe(false);
    }
  }
  // no panel while driving, the only action is get out
  expect(await page.evaluate(() => window.__zoo!.app.panel_key())).toBe('');
  await page.evaluate(() => window.__zoo!.app.debug_cart_pose(0, 4.0, 3.5, 0, 1));
  await drive(page, 'KeyW', 0.1);
  await page.screenshot({ path: `${shots}/cart-driving.png` });
  // get out (the parking rect is always valid)
  let left = false;
  for (let k = 0; k < 10 && !left; k++) {
    const r = JSON.parse((await page.evaluate(() => window.__zoo!.app.interact())) || '{}') as { kind?: string };
    left = r.kind === 'cart_left';
    if (!left) await drive(page, 'KeyD', 1);
  }
  expect(left).toBe(true);
  await nextFrames(page, 3);
  expect(await page.evaluate(() => window.__zoo!.app.driving())).toBe(false);
  // first person returns
  expect(await page.evaluate(() => window.__zoo!.app.view_mode())).toBe('first_person');
  expect(await page.evaluate(() => window.__zoo!.app.camera_extra_distance())).toBeLessThan(2);
  expect(errors).toEqual([]);
});

test('CART-019: the parking check refuses a spot on a stand cell (🅿️✖), she stays seated', async ({ page }) => {
  await start(page);
  const t = ftl('de');
  await page.evaluate(() => window.__zoo!.app.debug_give_cart_key());
  await atCart(page);
  await page.evaluate(() => window.__zoo!.app.interact());
  // the key box's stand cell (-8, 1)
  await page.evaluate(() => window.__zoo!.app.debug_cart_pose(0, -7.5, 1.5, 0, 1));
  await nextFrames(page, 2);
  const r = JSON.parse(await page.evaluate(() => window.__zoo!.app.interact())) as { kind: string; text: string };
  expect(r.kind).toBe('cart_no_park');
  expect(r.text).toBe(t['cart-no-park']);
  expect(await page.evaluate(() => window.__zoo!.app.driving())).toBe(true);
});

test('CART-028: headlights at night while driving, the parked cart stays dark', async ({ page }) => {
  await start(page);
  await page.evaluate(() => {
    const a = window.__zoo!.app;
    a.debug_give_cart_key();
    a.debug_set_daytime('night');
  });
  await atCart(page);
  await page.evaluate(() => window.__zoo!.app.interact());
  await nextFrames(page, 5);
  const c = await carts(page);
  expect(c[0].lights).toBe(true);
  expect(c[1].lights).toBe(false);
  await page.screenshot({ path: `${shots}/cart-night.png` });
});

for (const [name, w, h] of [
  ['780x360', 780, 360],
  ['360x780', 360, 780],
] as const) {
  test.describe(`small screen ${name}`, () => {
    test.use({ hasTouch: true });
    test(`CART-010 (${name}): the get-out button stays visible, >= 64 px, apart from the other buttons`, async ({ page }) => {
      await start(page, w, h);
      await page.evaluate(() => window.__zoo!.app.debug_give_cart_key());
      await page.locator('#game').tap({ position: { x: w / 2, y: h / 2 } }); // touch UI
      await atCart(page);
      await page.evaluate(() => window.__zoo!.app.interact());
      await nextFrames(page, 6);
      const act = page.locator('#act');
      await expect(act).toBeVisible();
      await expect(act).toHaveAttribute('data-kind', 'get_out');
      const box = (await act.boundingBox())!;
      expect(box.width).toBeGreaterThanOrEqual(64);
      expect(box.height).toBeGreaterThanOrEqual(64);
      expect(box.x + box.width).toBeLessThanOrEqual(w);
      expect(box.y + box.height).toBeLessThanOrEqual(h);
      for (const id of ['#settings-btn', '#compass-btn']) {
        const o = await page.locator(id).boundingBox();
        if (!o) continue;
        const overlap = !(box.x + box.width <= o.x || o.x + o.width <= box.x || box.y + box.height <= o.y || o.y + o.height <= box.y);
        expect(overlap, `${id} overlaps the get-out button`).toBe(false);
      }
      await expect(page.locator('#view-btn')).toBeHidden();
      // the 🔔 horn (Q-378, ASND-040): visible while seated, >= 64 px, on screen, apart from the other buttons
      const horn = page.locator('#horn-btn');
      await expect(horn).toBeVisible();
      const hb = (await horn.boundingBox())!;
      expect(hb.width).toBeGreaterThanOrEqual(64);
      expect(hb.height).toBeGreaterThanOrEqual(64);
      expect(hb.x + hb.width).toBeLessThanOrEqual(w);
      expect(hb.y + hb.height).toBeLessThanOrEqual(h);
      for (const id of ['#act', '#settings-btn', '#compass-btn']) {
        const o = await page.locator(id).boundingBox();
        if (!o) continue;
        const overlap = !(hb.x + hb.width <= o.x || o.x + o.width <= hb.x || hb.y + hb.height <= o.y || o.y + o.height <= hb.y);
        expect(overlap, `${id} overlaps the horn button`).toBe(false);
      }
      // the right thumb gets out
      await act.tap();
      await nextFrames(page, 3);
      expect(await page.evaluate(() => window.__zoo!.app.driving())).toBe(false);
      await expect(horn).toBeHidden();
      await page.screenshot({ path: `${shots}/cart-small-${name}.png` });
    });
  });
}
