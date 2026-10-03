// GAME-HINT e2e (HINT-007, HINT-009, HINT-013) and the night progress + the whole night
// cycle of GAME-NIGHT (NIGHT-020, NIGHT-021): a scripted child who only follows the 🧭 hint
// brings every level-1 animal home; the 🌙 progress shows what is missing, then that night
// is coming; dusk → night, the moon door opens, the bed works with E, the morning comes.
// Positions come from the game (hint target / stand point, food boxes, gates) — never
// hard-coded, so moved boxes or boards do not break it.
import { expect, test, type Page } from '@playwright/test';
import { approach, goto, nextFrames, waitFrames, START_URL } from './helpers';

test.describe.configure({ timeout: 300_000 });

/** Review screenshots: `HINT_SHOTS=<dir>` keeps them, else the test output folder. */
function shot(info: { outputPath: (n: string) => string }, name: string): string {
  const dir = process.env.HINT_SHOTS;
  return dir ? `${dir}/${name}` : info.outputPath(name);
}

interface Hint {
  id: string;
  kind: string;
  on: boolean;
  x: number;
  y: number;
  angle: number;
  dots: number;
  lx: number;
  lz: number;
  sx: number;
  sz: number;
  animal: string;
}

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

async function hint(page: Page): Promise<Hint | null> {
  const json = await page.evaluate(() => window.__zoo!.app.hint_json());
  return json ? (JSON.parse(json) as Hint) : null;
}

/** Presses the hint (as `H` would) and returns what is shown. */
async function press(page: Page): Promise<Hint> {
  await page.evaluate(() => window.__zoo!.app.hint_press());
  const h = await hint(page);
  expect(h, 'a hint target').not.toBeNull();
  return h!;
}

async function progress(page: Page) {
  return page.evaluate(() => JSON.parse(window.__zoo!.app.compass_json()) as { state: string; badge: string; animals: { id: string; home: boolean }[] });
}

async function faceAndSettle(page: Page, x: number, z: number, s = 0.6) {
  await page.evaluate(([px, pz]) => window.__zoo!.app.debug_face_point(px, pz), [x, z]);
  await page.evaluate((t) => window.__zoo!.app.debug_step(t), s);
  await nextFrames(page, 2);
}

async function hideCelebration(page: Page) {
  await page.evaluate(() => {
    document.getElementById('celebrate')!.hidden = true;
  });
}

/** One step of a child who follows the hint (and reads: the board names food and place). */
async function followHint(page: Page, h: Hint) {
  const app = <T>(f: string, ...a: unknown[]) =>
    page.evaluate(
      ([fn, args]) => (window.__zoo!.app as unknown as Record<string, (...x: unknown[]) => unknown>)[fn as string](...(args as unknown[])),
      [f, a] as const,
    ) as Promise<T>;
  switch (h.kind) {
    case 'board': {
      const started = await app<boolean>('mission_started', h.animal);
      const food = await app<string>('debug_animal_food', h.animal);
      if (started && (await app<string>('carry_food')) === food) {
        // the riddle was read: go and find the animal, show the food (E)
        await approach(page, h.animal);
        await page.keyboard.press('KeyE');
        await nextFrames(page, 2);
      } else {
        await goto(page, h.sx, h.sz);
        await faceAndSettle(page, h.lx, h.lz);
        await expect(page.locator('#panel')).toBeVisible(); // opens by itself
        expect(await app<boolean>('mission_started', h.animal)).toBe(true);
        await page.keyboard.press('Escape');
      }
      break;
    }
    case 'food': {
      // read the labels: the box of the food the board named
      const food = await app<string>('debug_animal_food', h.animal);
      await goto(page, h.sx, h.sz);
      const b = await app<number[]>('debug_food_box', food);
      await goto(page, b[0] + b[2] * 1.1, b[1] + b[3] * 1.1);
      await faceAndSettle(page, b[0], b[1]);
      await expect(page.locator('#panel')).toBeVisible();
      await page.locator('#take').click();
      expect(await app<string>('carry_food')).toBe(food);
      break;
    }
    case 'animal': {
      await goto(page, h.sx, h.sz);
      await approach(page, h.animal);
      await page.keyboard.press('KeyE');
      await nextFrames(page, 2);
      break;
    }
    case 'gate': {
      await goto(page, h.sx, h.sz);
      await page.evaluate(() => window.__zoo!.app.debug_step(3)); // the animal catches up
      await faceAndSettle(page, h.lx, h.lz, 0.1);
      if ((await app<string>('target_kind')) === 'gate') await page.keyboard.press('KeyE');
      else {
        const g = await app<number[]>('debug_gate_point', h.animal);
        await goto(page, g[0], g[1]);
      }
      await nextFrames(page, 2);
      break;
    }
    default: {
      // pick up, bamboo, water, bed, moon door, garden, treat: stand there, face it, E
      await goto(page, h.sx, h.sz);
      await faceAndSettle(page, h.lx, h.lz, 0.1);
      await page.keyboard.press('KeyE');
      await nextFrames(page, 2);
    }
  }
}

test.describe('desktop', () => {
  test.use({ viewport: { width: 1280, height: 720 } });

  test('HINT-007 / HINT-009 (desktop H): indicator above an on-screen target, edge arrow with dots off-screen', async ({ page }, info) => {
    const errors = await start(page);
    // H = the 🧭 button
    await page.keyboard.press('KeyH');
    await nextFrames(page, 3);
    const h = await hint(page);
    expect(h?.kind).toBe('board');
    await expect(page.locator('#compass-btn')).toHaveClass(/on/);
    // look at the target: the indicator bounces above it (DOM overlay, drawn on top of the
    // trees), inside the screen
    await page.evaluate(([x, z]) => window.__zoo!.app.debug_look_at(x, z), [h!.lx, h!.lz]);
    await nextFrames(page, 3);
    const on = (await hint(page))!;
    expect(on.on).toBe(true);
    await expect(page.locator('#hint-marker')).toBeVisible();
    await expect(page.locator('#hint-edge')).toBeHidden();
    await expect(page.locator('#hint-marker .icon')).toHaveText('📋');
    const box = (await page.locator('#hint-marker').boundingBox())!;
    expect(Math.abs(box.x + box.width / 2 - on.x)).toBeLessThan(40);
    expect(box.y + box.height).toBeLessThanOrEqual(on.y + 20);
    // an HTML overlay above the canvas: trees can never hide it
    expect(await page.evaluate(() => getComputedStyle(document.getElementById('hint-marker')!).position)).toBe('fixed');
    await page.screenshot({ path: shot(info, 'hint_on_screen.png') });
    // look far away: the edge arrow at the border, pointing to it, with distance dots
    await page.evaluate(([x, z]) => window.__zoo!.app.debug_look_at(x + 40, z + 30), [h!.lx, h!.lz]);
    await nextFrames(page, 3);
    const off = (await hint(page))!;
    expect(off.on).toBe(false);
    await expect(page.locator('#hint-edge')).toBeVisible();
    await expect(page.locator('#hint-marker')).toBeHidden();
    const dots = await page.locator('#hint-edge .dots i').count();
    expect(dots).toBeGreaterThanOrEqual(1);
    expect(dots).toBeLessThanOrEqual(5);
    const eb = (await page.locator('#hint-edge').boundingBox())!;
    expect(eb.x).toBeGreaterThanOrEqual(0);
    expect(eb.y).toBeGreaterThanOrEqual(0);
    expect(eb.x + eb.width).toBeLessThanOrEqual(1280 + 1);
    expect(eb.y + eb.height).toBeLessThanOrEqual(720 + 1);
    await page.screenshot({ path: shot(info, 'hint_off_screen.png') });
    // shown for 12 s (game time), then gone
    await page.evaluate(() => window.__zoo!.app.debug_look_at_player());
    await page.evaluate(() => window.__zoo!.app.debug_step(12.5));
    await nextFrames(page, 2);
    expect(await hint(page)).toBeNull();
    await expect(page.locator('#hint-edge')).toBeHidden();
    await expect(page.locator('#hint-marker')).toBeHidden();
    expect(errors).toEqual([]);
  });

  test('NIGHT-021 / HINT-014: following the hints brings level 1 home; dusk → night, moon door, bed (E), morning', async ({ page }) => {
    test.setTimeout(420_000);
    const errors = await start(page);
    let p = await progress(page);
    expect(p.state).toBe('missing');
    expect(p.animals.map((a) => a.id).sort()).toEqual(['hippo', 'panda', 'zebra']);
    await expect(page.locator('#compass-btn .strip .pa')).toHaveCount(3);
    await expect(page.locator('#night-progress')).toHaveCount(0);
    const kinds: string[] = [];
    for (let step = 0; step < 40; step++) {
      p = await progress(page);
      if (p.state !== 'missing') break;
      const h = await press(page);
      kinds.push(`${h.kind}:${h.animal}`);
      await followHint(page, h);
      await hideCelebration(page);
      // the strip loses one icon per animal brought home
      const home = (await progress(page)).animals.filter((a) => a.home).length;
      await nextFrames(page, 2);
      if (home > 0 && home < 3) await expect(page.locator('#compass-btn')).toHaveAttribute('data-home', String(home));
    }
    for (const a of ['zebra', 'hippo', 'panda']) {
      expect(await page.evaluate((id) => window.__zoo!.app.mission_complete(id), a), `${a} (hints: ${kinds.join(' ')})`).toBe(true);
    }
    // all home: night is coming (the 🌙 badge on the compass), dusk after the celebration, then night
    await nextFrames(page, 2);
    await expect(page.locator('#compass-btn')).toHaveAttribute('data-state', 'night_coming');
    await expect(page.locator('#compass-btn .badge')).toHaveAttribute('data-badge', 'night_coming');
    await expect(page.locator('#compass-btn .strip .pa')).toHaveCount(0);
    await page.evaluate(() => window.__zoo!.app.debug_step(7.5));
    expect(await page.evaluate(() => window.__zoo!.app.daytime())).toBe('dusk');
    await page.evaluate(() => window.__zoo!.app.debug_step(11));
    expect(await page.evaluate(() => window.__zoo!.app.daytime())).toBe('night');
    expect(await page.evaluate(() => window.__zoo!.app.barrier_open('moon_door'))).toBe(true);
    await nextFrames(page, 2);
    await expect(page.locator('#compass-btn')).toHaveAttribute('data-state', 'night');
    await expect(page.locator('#compass-btn .badge')).toHaveAttribute('data-badge', /^(moon_door|bed)$/);
    // at night the hint offers the moon door and the bed (top 3)
    const seen: string[] = [];
    let bed: Hint | null = null;
    for (let i = 0; i < 3; i++) {
      const h = await press(page);
      seen.push(h.kind);
      if (h.kind === 'bed') bed = h;
    }
    expect(seen).toContain('moon_door');
    expect(bed, `bed among ${seen.join(',')}`).not.toBeNull();
    // the bed with E (NIGHT-016) → sleeping → the next morning
    await goto(page, bed!.sx, bed!.sz);
    await faceAndSettle(page, bed!.lx, bed!.lz, 0.1);
    expect(await page.evaluate(() => window.__zoo!.app.target_kind())).toBe('bed');
    await page.keyboard.press('KeyE');
    expect(await page.evaluate(() => window.__zoo!.app.daytime())).toBe('sleeping');
    await nextFrames(page, 2);
    await expect(page.locator('#compass-btn .badge')).toBeHidden();
    await expect(page.locator('#compass-btn .strip .pa')).toHaveCount(0);
    await page.evaluate(() => window.__zoo!.app.debug_step(3));
    expect(await page.evaluate(() => window.__zoo!.app.daytime())).toBe('morning');
    await page.evaluate(() => window.__zoo!.app.debug_step(3.5));
    expect(await page.evaluate(() => window.__zoo!.app.daytime())).toBe('day');
    // the night zoo waits (Q-140): the compass shows the bed
    await nextFrames(page, 2);
    await expect(page.locator('#compass-btn')).toHaveAttribute('data-state', 'sleep');
    await expect(page.locator('#compass-btn .badge')).toHaveAttribute('data-badge', 'bed');
    expect(errors).toEqual([]);
  });
});

const PHONES = [
  { name: 'portrait', width: 412, height: 892 },
  { name: 'landscape', width: 892, height: 412 },
];
const overlaps = (a: { x: number; y: number; width: number; height: number }, b: { x: number; y: number; width: number; height: number }): boolean =>
  a.x < b.x + b.width && b.x < a.x + a.width && a.y < b.y + b.height && b.y < a.y + a.height;

for (const ph of PHONES) {
  test.describe(`phone ${ph.name} (touch)`, () => {
    test.use({ viewport: { width: ph.width, height: ph.height }, hasTouch: true, isMobile: true, deviceScaleFactor: 2 });

    test(`HINT-009 / HINT-013 / NIGHT-020 (${ph.name}): the 🧭 below the gear (≥ 64 px) carries badge and strip, no moon pane, tap = hint + bubble`, async ({ page }, info) => {
      const errors = await start(page);
      await page.locator('#game').tap({ position: { x: ph.width / 2, y: ph.height / 2 } }); // first touch: touch UI
      await nextFrames(page, 2);
      const gear = (await page.locator('#settings-btn').boundingBox())!;
      const compass = (await page.locator('#compass-btn').boundingBox())!;
      expect(compass.width).toBeGreaterThanOrEqual(64);
      expect(compass.height).toBeGreaterThanOrEqual(64);
      // directly below the gear: same column, below it, a small gap
      expect(Math.abs(compass.x + compass.width / 2 - (gear.x + gear.width / 2))).toBeLessThan(4);
      expect(compass.y).toBeGreaterThanOrEqual(gear.y + gear.height);
      expect(compass.y - (gear.y + gear.height)).toBeLessThan(24);
      // no moon pane any more; the compass shows one strip icon per level-1 animal and the task badge
      await expect(page.locator('#night-progress')).toHaveCount(0);
      const c = page.locator('#compass-btn');
      const strip = c.locator('.strip .pa');
      await expect(strip).toHaveCount(3);
      await expect(c.locator('.badge')).toBeVisible();
      await expect(c.locator('.badge')).toHaveAttribute('data-badge', 'board');
      // both phone sizes are "small screens" (HINT-023): the strip is collapsed until the compass is tapped
      await expect(strip.first()).toBeHidden();
      await expect(c).toHaveAttribute('data-missing', '3');
      await c.tap();
      await expect(strip.first()).toBeVisible();
      // the strip must not cover the interact button, the basket HUD or the stick; inside the screen
      for (const icon of await strip.all()) {
        const b = (await icon.boundingBox())!;
        expect(b.x).toBeGreaterThanOrEqual(0);
        expect(b.x + b.width).toBeLessThanOrEqual(ph.width);
        expect(b.y + b.height).toBeLessThanOrEqual(ph.height);
        for (const other of ['#act', '#hud-carry', '#stick', '#settings-btn', '#drop-btn']) {
          const ob = await page.locator(other).boundingBox();
          if (ob) expect(overlaps(b, ob), `${other} vs strip icon`).toBe(false);
        }
      }
      // tapping the strip is the compass tap: the hint shows (board first)
      await strip.first().tap();
      await nextFrames(page, 3);
      expect((await hint(page))?.kind).toBe('board');
      await page.screenshot({ path: shot(info, `phone_${ph.name}_compass.png`) });
      // one animal home: its icon disappears, only the missing ones stay (NIGHT-022)
      await page.evaluate(() => window.__zoo!.app.debug_send_home('hippo'));
      await nextFrames(page, 3);
      await hideCelebration(page);
      await page.waitForTimeout(400); // the strip is polled at <= 4 Hz
      await expect(strip).toHaveCount(2);
      await expect(c.locator('.strip .pa[data-animal="hippo"]')).toHaveCount(0);
      await expect(c).toHaveAttribute('data-home', '1');
      // HINT-013: the compass tap = hint of a still missing animal + the explaining bubble
      await page.evaluate(() => window.__zoo!.app.debug_step(12.5)); // the old hint is gone
      await nextFrames(page, 2);
      expect(await hint(page)).toBeNull();
      await c.tap();
      await nextFrames(page, 3);
      const h = await hint(page);
      expect(h).not.toBeNull();
      expect(['board', 'food']).toContain(h!.kind);
      expect(h!.animal).not.toBe('hippo');
      await expect(page.locator('#hint-marker, #hint-edge').first()).toBeAttached();
      // the strip icon of the hinted animal pulses
      if (h!.animal) await expect(c.locator(`.strip .pa[data-animal="${h!.animal}"]`)).toHaveClass(/hl/);
      // NIGHT-023: the bubble (default reading level klasse2, de) near the compass
      const bubble = page.locator('#bubble');
      await expect(bubble).toBeVisible();
      await expect(bubble).toHaveAttribute('data-key', /^night-progress-info-(klasse|kiga)/);
      expect(((await bubble.textContent()) ?? '').length).toBeGreaterThan(3);
      expect(await bubble.textContent()).not.toContain('night-progress-info');
      // all home: night is coming, the 🌙 badge, no animal icon
      await page.evaluate(() => {
        for (const a of ['zebra', 'panda']) window.__zoo!.app.debug_send_home(a);
      });
      await nextFrames(page, 3);
      await hideCelebration(page);
      await page.waitForTimeout(400);
      await expect(c).toHaveAttribute('data-state', 'night_coming');
      await expect(c.locator('.badge')).toHaveAttribute('data-badge', 'night_coming');
      await expect(strip).toHaveCount(0);
      await c.tap();
      await expect(bubble).toHaveAttribute('data-key', /^night-progress-info-done-/);
      await page.screenshot({ path: shot(info, `phone_${ph.name}_night_coming.png`) });
      expect(errors).toEqual([]);
    });
  });
}
