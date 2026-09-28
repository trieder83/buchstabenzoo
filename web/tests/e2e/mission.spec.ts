// POC-002 / POC-003 / RESC-010 / PLAY-010 / ADIR-003 (PROD-POC milestone M4): a scripted
// player completes the zebra mission — info board → grass box → river → show food → lead
// home — with the texts of the panel taken from the Fluent files.
import path from 'node:path';
import { expect, test, type Page } from '@playwright/test';
import { face, ftl, goto, nextFrames, shots, state, waitFrames, START_URL, approach } from './helpers';

test.describe.configure({ timeout: 300_000 });

/** Starts the game; `lang` null = no stored language (the game default `de` applies). */
async function start(page: Page, lang: string | null, level: string) {
  await page.addInitScript(
    ([l, r]) => {
      // only on the first load, so a reload keeps what the settings UI stored
      if (sessionStorage.getItem('zoo.e2e.init')) return;
      sessionStorage.setItem('zoo.e2e.init', '1');
      localStorage.clear(); // clean state: no saved game or settings from other tests
      if (l) localStorage.setItem('zoo.language', l);
      else localStorage.removeItem('zoo.language');
      localStorage.setItem('zoo.readingLevel', r);
    },
    [lang, level] as const,
  );
  const errors: string[] = [];
  page.on('console', (m) => {
    if (m.type() === 'error') errors.push(m.text());
  });
  page.on('pageerror', (e) => errors.push(String(e)));
  await page.goto(START_URL);
  await waitFrames(page, 3);
  return errors;
}

/** Lets 0.5 s of game time pass (panel settle time 0.25 s) and renders two frames. */
async function settle(page: Page) {
  await page.evaluate(() => window.__zoo!.app.debug_step(0.5));
  await nextFrames(page, 2);
}

/** Screenshots kept (≤ 5): landscape board/river/home, portrait touch food box/home. */
const KEEP = new Set([
  'de_landscape_board',
  'de_landscape_river',
  'de_landscape_home',
  'en_portrait_touch_foodbox',
  'en_portrait_touch_home',
]);

async function snap(page: Page, shot: string | undefined, name: string) {
  if (shot && KEEP.has(`${shot}_${name}`)) {
    await page.screenshot({ path: path.join(shots, `screenshot_poc_m4_${shot}_${name}.png`) });
  }
}

/** Plays the mission; `shot` names screenshots (undefined = none). */
async function playZebra(page: Page, lang: string, level: string, shot?: string) {
  const t = ftl(lang);
  const zebraSpot = await page.evaluate(() => ({
    x: window.__zoo!.app.animal_x('zebra'),
    z: window.__zoo!.app.animal_z('zebra'),
  }));

  // 1. Info board of enc_zebra (cell -9,14, readable side east): stand in front, face west.
  await goto(page, -7.5, 10.5);
  await face(page, 'KeyA');
  expect((await state(page)).target).toBe('info_board:zebra');
  await expect(page.locator(shot?.includes('touch') ? '#act' : '#hint')).toBeVisible();
  // the reading panel opens by itself (GAME-PLAYER §4, PLAY-023) — no button press; let
  // 0.5 s of game time pass deterministically (software WebGL frames can be slow)
  await settle(page);
  await expect(page.locator('#panel')).toBeVisible();
  await expect(page.locator('#panel-text')).toHaveText(t[`mission-zebra-riddle-loc_river-${level}`]);
  await expect(page.locator('#panel-food .word')).toHaveText(t['food-grass']);
  // ANIM-006: riddle → food word first, then "more about the zebra" + facts
  await expect(page.locator('#panel-title .word')).toHaveText(t['animal-zebra-more']);
  await expect(page.locator('#panel-facts')).toHaveText(t[`mission-zebra-facts-${level}`]);
  const order = await page.evaluate(() => [...document.querySelectorAll('#panel [id]')].map((e) => e.id));
  expect(order.indexOf('panel-text')).toBeLessThan(order.indexOf('panel-food'));
  expect(order.indexOf('panel-food')).toBeLessThan(order.indexOf('panel-title'));
  expect(order.indexOf('panel-title')).toBeLessThan(order.indexOf('panel-facts'));
  // ADIR-003: cap height ≥ 3 % of the viewport height (cap height ≈ 0.7 × font size).
  const cap = await page.evaluate(() => {
    const px = parseFloat(getComputedStyle(document.getElementById('panel-text')!).fontSize);
    return (0.7 * px) / window.innerHeight;
  });
  expect(cap).toBeGreaterThanOrEqual(0.03);
  expect((await state(page)).started).toBe(true);
  await snap(page, shot, 'board');
  await page.keyboard.press('Escape');
  await expect(page.locator('#panel')).toBeHidden();

  // 2. Food boxes in front of the storage: first the wrong one (bamboo), then grass.
  for (const [food, x] of [
    ['bamboo', -1.2],
    ['grass', -0.4],
  ] as const) {
    await goto(page, x, 9.6);
    await face(page, 'KeyW');
    expect((await state(page)).target).toBe(`food_box:${food}`);
    // opens by itself; the food is only taken with the take button (PLAY-027)
    await settle(page);
    await expect(page.locator('#panel-text')).toHaveText(t[`food-${food}`]);
    expect((await state(page)).carry).not.toBe(food);
    if (food === 'grass') await snap(page, shot, 'foodbox');
    await page.locator('#take').click();
    await expect(page.locator('#panel')).toBeHidden();
    expect((await state(page)).carry).toBe(food);
    await expect(page.locator('#hud-carry .word')).toHaveText(t[`food-${food}`]);
  }

  // 3. The river: walk up to the (wandering) zebra and face it; show the grass.
  expect(Math.hypot(zebraSpot.x - 8.5, zebraSpot.z - 33.5)).toBeLessThan(3.1); // loc_river spot (8, 33), FIX-056
  await approach(page, 'zebra');
  expect((await state(page)).target).toBe('animal:zebra');
  await page.keyboard.press('KeyE');
  await expect(page.locator('#bubble')).toHaveText(t['ui-following']);
  expect((await state(page)).zebra).toBe('following');
  await snap(page, shot, 'river');

  // 4. Lead it home: walk to the zebra gate (east fence, cells -10,12..13), wait for the
  // zebra, face the gate and interact (= lead in).
  await goto(page, -8.4, 13.0);
  await page.evaluate(() => window.__zoo!.app.debug_step(3));
  const z = await page.evaluate(() => ({
    d: Math.hypot(
      window.__zoo!.app.animal_x('zebra') - window.__zoo!.app.player_x(),
      window.__zoo!.app.animal_z('zebra') - window.__zoo!.app.player_z(),
    ),
  }));
  expect(z.d).toBeLessThan(3);
  await face(page, 'KeyA');
  expect((await state(page)).target).toBe('gate:enc_zebra');
  await page.keyboard.press('KeyE');
  const done = await state(page);
  expect(done.zebra).toBe('in_enclosure');
  expect(done.complete).toBe(true);
  expect(done.carry).toBe('');
  await expect(page.locator('#celebrate-text')).toHaveText(t['mission-zebra-home']);
  // let the zebra walk in and eat
  await page.evaluate(() => window.__zoo!.app.debug_step(3));
  await nextFrames(page, 3);
  await snap(page, shot, 'home');
}

test.describe('landscape desktop', () => {
  test.use({ viewport: { width: 1280, height: 720 } });

  test('POC-002 / RESC-010 / PLAY-010 / ADIR-003: zebra mission in de, klasse1', async ({ page }) => {
    const errors = await start(page, 'de', 'klasse1');
    await playZebra(page, 'de', 'klasse1', 'de_landscape');
    expect(errors).toEqual([]);
  });

  test('wrong food: the zebra is not interested (RESC-005 in the browser)', async ({ page }) => {
    const errors = await start(page, 'de', 'klasse2');
    const t = ftl('de');
    await goto(page, -1.2, 9.6);
    await face(page, 'KeyW');
    await expect(page.locator('#take')).toBeVisible(); // panel opens by itself
    await page.locator('#take').click();
    await approach(page, 'zebra');
    await page.keyboard.press('Space');
    await expect(page.locator('#bubble')).toHaveText(t['ui-not-interested']);
    expect((await state(page)).zebra).toBe('escaped');
    expect(errors).toEqual([]);
  });
});

test.describe('portrait touch', () => {
  test.use({ viewport: { width: 540, height: 1170 }, deviceScaleFactor: 2, hasTouch: true, isMobile: true });

  // English browser: the game still starts in German (CONT-L10N §5, L10N-005).
  test.use({ locale: 'en-US' });

  test('POC-003 / L10N-005: English chosen in the settings (touch UI), all panel texts English', async ({ page }) => {
    const errors = await start(page, null, 'klasse1');
    expect(await page.evaluate(() => window.__zoo!.app.language())).toBe('de');
    await expect(page.locator('html')).toHaveAttribute('lang', 'de');
    // switch to English in the settings menu (icon buttons)
    await page.locator('#settings-btn').tap();
    await expect(page.locator('#settings')).toBeVisible();
    await page.locator('#settings [data-lang="en"]').tap();
    await page.locator('#settings-btn').tap();
    await expect(page.locator('#settings')).toBeHidden();
    expect(await page.evaluate(() => window.__zoo!.app.language())).toBe('en');
    // the choice is kept after a reload
    await page.reload();
    await waitFrames(page, 3);
    expect(await page.evaluate(() => window.__zoo!.app.language())).toBe('en');
    await expect(page.locator('html')).toHaveAttribute('lang', 'en');
    // first touch switches the touch controls on (PLAY-014)
    await page.touchscreen.tap(400, 300);
    await expect(page.locator('#stick')).toBeVisible();
    await playZebra(page, 'en', 'klasse1', 'en_portrait_touch');
    expect(errors).toEqual([]);
  });
});
