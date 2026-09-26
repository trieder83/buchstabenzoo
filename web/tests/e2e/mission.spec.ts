// POC-002 / POC-003 / RESC-010 / PLAY-010 / ADIR-003 (PROD-POC milestone M4): a scripted
// player completes the zebra mission — info board → grass box → river → show food → lead
// home — with the texts of the panel taken from the Fluent files.
import path from 'node:path';
import { expect, test, type Page } from '@playwright/test';
import { face, ftl, goto, nextFrames, shots, state, waitFrames } from './helpers';

test.describe.configure({ timeout: 300_000 });

async function start(page: Page, lang: string, level: string) {
  await page.addInitScript(
    ([l, r]) => {
      localStorage.setItem('zoo.language', l);
      localStorage.setItem('zoo.readingLevel', r);
    },
    [lang, level],
  );
  const errors: string[] = [];
  page.on('console', (m) => {
    if (m.type() === 'error') errors.push(m.text());
  });
  page.on('pageerror', (e) => errors.push(String(e)));
  await page.goto('/');
  await waitFrames(page, 3);
  return errors;
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
  await goto(page, -7.5, 14.5);
  await face(page, 'KeyA');
  expect((await state(page)).target).toBe('info_board:zebra');
  await expect(page.locator(shot?.includes('touch') ? '#act' : '#hint')).toBeVisible();
  if (shot?.includes('touch')) await page.locator('#act').dispatchEvent('pointerdown');
  else await page.keyboard.press('KeyE');
  await expect(page.locator('#panel')).toBeVisible();
  await expect(page.locator('#panel-text')).toHaveText(t[`mission-zebra-riddle-${level}`]);
  await expect(page.locator('#panel-food .word')).toHaveText(t['food-grass']);
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
    await page.keyboard.press('KeyE');
    await expect(page.locator('#panel-text')).toHaveText(t[`food-${food}`]);
    if (food === 'grass') await snap(page, shot, 'foodbox');
    await page.locator('#take').click();
    await expect(page.locator('#panel')).toBeHidden();
    expect((await state(page)).carry).toBe(food);
    await expect(page.locator('#hud-carry .word')).toHaveText(t[`food-${food}`]);
  }

  // 3. The river: stand west of the zebra and face it; show the grass.
  await goto(page, zebraSpot.x - 1.3, zebraSpot.z);
  await face(page, 'KeyD');
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
    await page.keyboard.press('KeyE');
    await page.locator('#take').click();
    const spot = await page.evaluate(() => ({ x: window.__zoo!.app.animal_x('zebra'), z: window.__zoo!.app.animal_z('zebra') }));
    await goto(page, spot.x - 1.3, spot.z);
    await face(page, 'KeyD');
    await page.keyboard.press('Space');
    await expect(page.locator('#bubble')).toHaveText(t['ui-not-interested']);
    expect((await state(page)).zebra).toBe('escaped');
    expect(errors).toEqual([]);
  });
});

test.describe('portrait touch', () => {
  test.use({ viewport: { width: 540, height: 1170 }, deviceScaleFactor: 2, hasTouch: true, isMobile: true });

  test('POC-003: zebra mission in en (touch UI), all panel texts English', async ({ page }) => {
    const errors = await start(page, 'en', 'klasse1');
    // first touch switches the touch controls on (PLAY-014)
    await page.touchscreen.tap(400, 300);
    await expect(page.locator('#stick')).toBeVisible();
    await playZebra(page, 'en', 'klasse1', 'en_portrait_touch');
    expect(errors).toEqual([]);
  });
});
