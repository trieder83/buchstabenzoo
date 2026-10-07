// ART-SOUND "Golf cart sounds": ASND-040 (board / drive / get out, locked cart, refused park,
// lock panel, key box, horn). No audio device is needed: the host logs every cue decision
// (`__zoo.audio.log`) and exposes the engine channel (`__zoo.audio.engine`).
import { expect, test, type Page } from '@playwright/test';
import { nextFrames, waitFrames, START_URL } from './helpers';

interface Entry {
  cue: string;
  gain: number;
  rate: number;
  muted: boolean;
  delay: number;
  result: string;
}
interface Engine {
  playing: boolean;
  gain: number;
  rate: number;
  grass: number;
  fetched: boolean;
  layers: [number, number];
}

const NOTE = { stand: [-10.5, 2.5], at: [-10.5, 3.7] };
const BOX = { stand: [-7.5, 1.5], at: [-7.95, 1.5] };

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
  // a user gesture unlocks the audio (autoplay rule)
  await page.keyboard.press('Shift');
  await page.waitForFunction(() => window.__zoo!.audio.isUnlocked);
  return errors;
}

const clear = (page: Page) => page.evaluate(() => void (window.__zoo!.audio.log.length = 0));
const engine = (page: Page) => page.evaluate(() => ({ ...window.__zoo!.audio.engine }) as Engine);

async function log(page: Page): Promise<Entry[]> {
  await nextFrames(page, 4);
  return page.evaluate(() => window.__zoo!.audio.log.map((e) => ({ ...e })) as Entry[]);
}
const cues = (l: Entry[], cue: string) => l.filter((e) => e.cue === cue);

async function stand(page: Page, p: { stand: number[]; at: number[] }) {
  await page.evaluate((p) => {
    const a = window.__zoo!.app;
    a.debug_teleport(p.stand[0], p.stand[1]);
    a.debug_face_point(p.at[0], p.at[1]);
  }, p);
  await nextFrames(page, 4);
}

/** Stands at the boarding cell of cart l1 (2.5, 3.5), facing the cart. */
const atCart = (page: Page) => stand(page, { stand: [2.5, 3.5], at: [4.0, 3.5] });

async function board(page: Page) {
  await atCart(page);
  const r = JSON.parse(await page.evaluate(() => window.__zoo!.app.interact())) as { kind: string };
  expect(r.kind).toBe('cart_boarded');
  await nextFrames(page, 4);
}

function answerOf(expr: string): number {
  const m = /^(\d+) ([+−×÷]) (\d+) = \?$/.exec(expr.trim());
  expect(m, `expression ${expr}`).not.toBeNull();
  const [a, op, b] = [Number(m![1]), m![2], Number(m![3])];
  return op === '+' ? a + b : op === '−' ? a - b : op === '×' ? a * b : a / b;
}

test('ASND-040: board, engine channel, drive faster, grass blend, get out', async ({ page }) => {
  const errors = await start(page);
  await page.evaluate(() => window.__zoo!.app.debug_give_cart_key());
  expect((await engine(page)).fetched).toBe(false); // lazy: not before the first boarding
  await clear(page);
  await board(page);
  const l = await log(page);
  expect(cues(l, 'cart_board').length).toBe(1);
  expect(cues(l, 'cart_board')[0].gain).toBeCloseTo(0.28, 2);
  // the engine channel appears and rises to the standing level 0.07 (0.28 * 0.25)
  await page.waitForFunction(() => window.__zoo!.audio.engine.playing, null, { timeout: 15_000 });
  await page.waitForFunction(() => window.__zoo!.audio.engine.gain > 0.06, null, { timeout: 5000 });
  let e = await engine(page);
  expect(e.fetched).toBe(true);
  expect(e.gain).toBeLessThanOrEqual(0.0705);
  expect(e.rate).toBeCloseTo(0.75, 1);

  // drive north on the plaza path: the level climbs towards 0.28 and never above
  await page.keyboard.down('KeyW');
  await page.waitForFunction(() => window.__zoo!.audio.engine.gain > 0.2, null, { timeout: 8000 });
  let max = 0;
  for (let i = 0; i < 20; i++) {
    max = Math.max(max, (await engine(page)).gain);
    await page.waitForTimeout(50);
  }
  await page.keyboard.up('KeyW');
  e = await engine(page);
  expect(max).toBeLessThanOrEqual(0.2801);
  expect(max).toBeGreaterThan(0.2);
  expect(e.rate).toBeGreaterThan(1.0);
  expect(e.layers[0]).toBeGreaterThan(0); // the path layer carries it
  expect(e.layers[1]).toBeLessThan(e.layers[0]);

  // parked on grass (the cart stands still): the grass layer takes over, softer
  const grass = await page.evaluate(() => {
    const a = window.__zoo!.app;
    for (let x = -24; x <= 24; x += 2) {
      for (let z = -24; z <= 24; z += 2) {
        if (a.debug_step_surface(x, z) === 'grass') return [x, z];
      }
    }
    return null;
  });
  expect(grass).not.toBeNull();
  await page.evaluate((g) => window.__zoo!.app.debug_cart_pose(0, g![0], g![1], 1, 0), grass);
  await page.waitForFunction(() => window.__zoo!.audio.engine.grass > 0.95, null, { timeout: 5000 });
  await page.waitForFunction(() => window.__zoo!.audio.engine.layers[0] < 0.001, null, { timeout: 5000 });
  e = await engine(page);
  expect(e.layers[1]).toBeGreaterThan(0);
  expect(e.layers[1]).toBeLessThanOrEqual(0.07 * 0.7 + 0.001);

  // get out: cart_get_out once, the engine fades out and releases its nodes
  await page.evaluate(() => window.__zoo!.app.debug_cart_pose(0, 4.0, 3.5, 0, 1));
  await clear(page);
  await page.evaluate(() => window.__zoo!.app.interact());
  expect(await page.evaluate(() => window.__zoo!.app.driving())).toBe(false);
  expect(cues(await log(page), 'cart_get_out').length).toBe(1);
  await page.waitForFunction(() => !window.__zoo!.audio.engine.playing, null, { timeout: 5000 });
  expect((await engine(page)).gain).toBe(0);
  expect(errors).toEqual([]);
});

test('ASND-040: a locked cart knocks, a refused park blips', async ({ page }) => {
  const errors = await start(page);
  await atCart(page);
  await clear(page);
  await page.evaluate(() => window.__zoo!.app.interact());
  expect(cues(await log(page), 'cart_locked').length).toBe(1);
  expect((await engine(page)).playing).toBe(false);
  // refused park: the key box's stand cell
  await page.evaluate(() => window.__zoo!.app.debug_give_cart_key());
  await board(page);
  await page.evaluate(() => window.__zoo!.app.debug_cart_pose(0, -7.5, 1.5, 0, 1));
  await nextFrames(page, 2);
  await clear(page);
  const r = JSON.parse(await page.evaluate(() => window.__zoo!.app.interact())) as { kind: string };
  expect(r.kind).toBe('cart_no_park');
  const l = await log(page);
  expect(cues(l, 'cart_park_refuse').length).toBe(1);
  expect(cues(l, 'cart_get_out').length).toBe(0);
  expect(errors).toEqual([]);
});

test('ASND-040: the horn button (only while seated) and key H, one per 0.6 s, sound only', async ({ page }) => {
  const errors = await start(page);
  await page.evaluate(() => window.__zoo!.app.debug_give_cart_key());
  await expect(page.locator('#horn-btn')).toBeHidden();
  await clear(page);
  await page.keyboard.press('KeyH'); // on foot: the 🧭 hint key, no horn
  expect(cues(await log(page), 'cart_horn').length).toBe(0);
  await board(page);
  await expect(page.locator('#horn-btn')).toBeVisible();
  const b = (await page.locator('#horn-btn').boundingBox())!;
  expect(Math.min(b.width, b.height)).toBeGreaterThanOrEqual(64);
  await clear(page);
  await page.locator('#horn-btn').dispatchEvent('pointerdown');
  await page.locator('#horn-btn').dispatchEvent('pointerdown'); // within 0.6 s: dropped
  let l = await log(page);
  expect(cues(l, 'cart_horn').length).toBe(1);
  expect(cues(l, 'cart_horn')[0].gain).toBeCloseTo(0.28, 2);
  await page.evaluate(() => window.__zoo!.app.debug_step(0.7)); // game time, not wall time (slow machines)
  await page.keyboard.press('KeyH');
  l = await log(page);
  expect(cues(l, 'cart_horn').length).toBe(2);
  // getting out hides the button again
  await page.evaluate(() => window.__zoo!.app.interact());
  await expect(page.locator('#horn-btn')).toBeHidden();
  expect(errors).toEqual([]);
});

test('ASND-040: lock panel ticks, wrong code, right code and the delayed key box cues', async ({ page }) => {
  const errors = await start(page);
  await stand(page, NOTE);
  await expect(page.locator('#panel[data-kind="note"]')).toBeVisible();
  const answer = answerOf((await page.locator('#note-expr').textContent())!);
  await page.keyboard.press('Escape');
  await stand(page, BOX);
  await page.keyboard.press('KeyE');
  await expect(page.locator('#lock-panel')).toBeVisible();
  await clear(page);
  // one tick per digit step (touch buttons and the keyboard)
  await page.locator('#lock-up-0').click();
  await page.locator('#lock-down-1').click();
  await page.keyboard.press('ArrowUp');
  let l = await log(page);
  expect(cues(l, 'lock_wheel_tick').length).toBe(3);
  expect(cues(l, 'lock_wheel_tick').every((e) => e.gain > 0.2 && e.rate >= 0.95 && e.rate <= 1.05)).toBe(true);
  // a wrong code: lock_wrong once, no ui_refuse on top
  await page.locator('#lock-down-0').click();
  await page.keyboard.press('ArrowRight'); // selection moves, no step
  await clear(page);
  await page.locator('#lock-ok').click(); // the wheels are back at 000: never the answer
  l = await log(page);
  expect(cues(l, 'lock_wrong').length).toBe(1);
  expect(cues(l, 'ui_refuse').length).toBe(0);
  expect(cues(l, 'lock_ok').length).toBe(0);
  await expect(page.locator('#lock-panel')).toBeVisible();
  // the right code: lock_ok at once, key_box_open after 0.6 s, key_pickup after 1.2 s
  const digits = String(answer).padStart(3, '0').split('').map(Number);
  for (let i = 0; i < 3; i++) {
    const cur = Number((await page.locator(`#lock-digit-${i}`).textContent()) ?? '0');
    for (let k = 0; k < (digits[i] - cur + 10) % 10; k++) await page.locator(`#lock-up-${i}`).click();
  }
  await clear(page);
  await page.locator('#lock-ok').click();
  await expect(page.locator('#lock-panel')).toBeHidden();
  l = await log(page);
  expect(cues(l, 'lock_ok').length).toBe(1);
  expect(cues(l, 'lock_ok')[0].delay).toBe(0);
  expect(cues(l, 'key_box_open').map((e) => e.delay)).toEqual([0.6]);
  expect(cues(l, 'key_pickup').map((e) => e.delay)).toEqual([1.2]);
  expect(cues(l, 'lock_wrong').length).toBe(0);
  expect(errors).toEqual([]);
});

test('ASND-040: muted: no cart cue plays and no engine starts', async ({ page }) => {
  const errors = await start(page);
  await page.evaluate(() => window.dispatchEvent(new CustomEvent('zoo-sound', { detail: { on: false } })));
  await page.evaluate(() => window.__zoo!.app.debug_give_cart_key());
  await clear(page);
  await board(page);
  await page.evaluate(() => window.__zoo!.app.honk());
  const l = await log(page);
  expect(cues(l, 'cart_board').length).toBe(1);
  expect(l.every((e) => e.muted && e.result === 'muted')).toBe(true);
  await page.waitForTimeout(1500);
  const e = await engine(page);
  expect(e.playing).toBe(false);
  expect(e.fetched).toBe(false);
  expect(errors).toEqual([]);
});
