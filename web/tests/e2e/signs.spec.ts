// AENV-011 / AENV-012 (ART-ENVIRONMENT behaviour 6, 7): the zebra enclosure sign shows the
// zebra silhouette decal and the food storage shows the "Futter" sign (Fluent
// `sign-food-storage`), both readable from the default camera (14 m, 55°).
import path from 'node:path';
import { expect, test, type Page } from '@playwright/test';
import { ftl, goto, shots, waitFrames, START_URL } from './helpers';

test.describe.configure({ timeout: 180_000 });
test.use({ viewport: { width: 1280, height: 720 } });

async function start(page: Page) {
  await page.addInitScript(() => {
    if (sessionStorage.getItem('zoo.e2e.init')) return;
    sessionStorage.setItem('zoo.e2e.init', '1');
    localStorage.clear();
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

/** Default camera distance (GAME-PLAYER §2: 14 m; level 1 starts at 20 m). */
async function defaultCamera(page: Page) {
  await page.evaluate(() => {
    const a = window.__zoo!.app;
    a.zoom(14 / a.camera_distance());
    a.debug_step(0.02); // snaps the camera (distance and target)
  });
  // let a few frames render with the new camera
  const f = await page.evaluate(() => window.__zoo!.frames);
  await page.waitForFunction((m) => window.__zoo!.frames >= m, f + 3);
  expect(await page.evaluate(() => window.__zoo!.app.camera_distance())).toBeCloseTo(14, 1);
}

type Rect = [number, number, number, number];

async function decalRect(page: Page, id: string): Promise<Rect> {
  const r = await page.evaluate((i) => window.__zoo!.app.decal_screen_rect(i), id);
  expect(r.length, `decal ${id} on screen`).toBe(4);
  return [r[0], r[1], r[2], r[3]];
}

/** Pixels of a screen rectangle (CSS px) as RGBA rows, decoded in the page. */
async function pixels(page: Page, r: Rect) {
  const clip = { x: Math.floor(r[0]), y: Math.floor(r[1]), width: Math.ceil(r[2] - r[0]), height: Math.ceil(r[3] - r[1]) };
  const png = await page.screenshot({ clip });
  return page.evaluate(async (b64) => {
    const img = new Image();
    img.src = `data:image/png;base64,${b64}`;
    await img.decode();
    const c = document.createElement('canvas');
    c.width = img.width;
    c.height = img.height;
    const ctx = c.getContext('2d')!;
    ctx.drawImage(img, 0, 0);
    const d = ctx.getImageData(0, 0, c.width, c.height).data;
    return { w: c.width, h: c.height, data: Array.from(d) };
  }, png.toString('base64'));
}

/** Dark ink (#2B2320 / #3B2314, possibly in the shadow tone) vs. light cream. */
const isInk = (r: number, g: number, b: number) => r < 90 && g < 80 && b < 80;
const isCream = (r: number, g: number, b: number) => r > 150 && g > 130 && b > 110;

test('AENV-011: the zebra enclosure sign shows the zebra silhouette, facing the path', async ({ page }) => {
  const errors = await start(page);
  expect(await page.evaluate(() => window.__zoo!.app.decal_ids())).toContain('sign:enc_zebra');
  expect(await page.evaluate(() => window.__zoo!.app.decal_drawn('sign:enc_zebra'))).toBe(true);
  // every enclosure sign whose silhouette file exists shows it (hippo, panda too)
  for (const id of ['sign:enc_hippo', 'sign:enc_panda']) {
    expect(await page.evaluate((i) => window.__zoo!.app.decal_drawn(i), id)).toBe(true);
  }

  // on the ring path in front of the zebra gate, default camera
  await goto(page, -7.4, 12.5);
  await defaultCamera(page);
  const rect = await decalRect(page, 'sign:enc_zebra');
  const vp = page.viewportSize()!;
  expect(rect[0]).toBeGreaterThanOrEqual(0);
  expect(rect[2]).toBeLessThanOrEqual(vp.width);
  expect(rect[3] - rect[1]).toBeGreaterThan(20); // big enough to read
  const px = await pixels(page, rect);
  let ink = 0;
  let cream = 0;
  for (let i = 0; i < px.data.length; i += 4) {
    const [r, g, b] = [px.data[i], px.data[i + 1], px.data[i + 2]];
    if (isInk(r, g, b)) ink += 1;
    else if (isCream(r, g, b)) cream += 1;
  }
  const n = px.w * px.h;
  console.log(`zebra sign ${px.w}×${px.h} px: ink ${(ink / n).toFixed(2)}, cream ${(cream / n).toFixed(2)}`);
  expect(ink / n).toBeGreaterThan(0.12); // the silhouette
  expect(cream / n).toBeGreaterThan(0.15); // on the cream panel
  expect(errors).toEqual([]);
});

test('AENV-012: "Futter" sign above the food boxes, letters ≥ 3 % of the viewport, follows the language', async ({ page }) => {
  const errors = await start(page);
  const de = ftl('de');
  const en = ftl('en');
  expect(de['sign-food-storage']).toBe('Futter');
  const spec = await page.evaluate(() => JSON.parse(window.__zoo!.app.text_textures()));
  // the food storage sign and the night house sign of night_1 (GAME-LEVEL-NIGHT-1), plus the
  // text faces of the entrance arch and the garden signs (ARCH-006)
  expect(spec).toEqual(
    expect.arrayContaining([
      expect.objectContaining({ id: 'text:sign-food-storage', text: 'Futter' }),
      expect.objectContaining({ id: 'text:sign-night-house', text: de['sign-night-house'] }),
      expect.objectContaining({ id: 'text:sign-zoo-entrance', text: de['sign-zoo-entrance'] }),
      expect.objectContaining({ id: 'text:garden-carrot', text: de['garden-carrot'] }),
    ]),
  );
  expect(await page.evaluate(() => window.__zoo!.app.decal_drawn('sign:food_storage'))).toBe(true);

  await goto(page, 0.5, 8.5); // ring path in front of the storage
  await defaultCamera(page);
  const rect = await decalRect(page, 'sign:food_storage');

  // letter height: rows with ink inside the board (inset past the texture's border line)
  const letterRows = async () => {
    const w = rect[2] - rect[0];
    const h = rect[3] - rect[1];
    const inner: Rect = [rect[0] + w * 0.06, rect[1] + h * 0.14, rect[2] - w * 0.06, rect[3] - h * 0.14];
    const px = await pixels(page, inner);
    let top = -1;
    let bottom = -1;
    let inkCount = 0;
    for (let y = 0; y < px.h; y++) {
      let row = 0;
      for (let x = 0; x < px.w; x++) {
        const i = (y * px.w + x) * 4;
        if (isInk(px.data[i], px.data[i + 1], px.data[i + 2])) row += 1;
      }
      inkCount += row;
      if (row >= 2) {
        if (top < 0) top = y;
        bottom = y;
      }
    }
    return { height: bottom - top + 1, ink: inkCount, hash: px.data.reduce((a, v, i) => (a + v * ((i % 97) + 1)) % 1_000_003, 0) };
  };
  const vp = page.viewportSize()!;
  const deRows = await letterRows();
  console.log(`Futter sign: board ${Math.round(rect[2] - rect[0])}×${Math.round(rect[3] - rect[1])} px, letters ${deRows.height} px = ${((100 * deRows.height) / vp.height).toFixed(1)} % of the viewport`);
  expect(deRows.height / vp.height).toBeGreaterThanOrEqual(0.03);

  // switch to English in the settings: the sign follows (re-rendered text texture)
  await page.locator('#settings-btn').click();
  await page.locator('#settings [data-lang="en"]').click();
  await page.locator('#settings-btn').click();
  const f = await page.evaluate(() => window.__zoo!.frames);
  await page.waitForFunction((m) => window.__zoo!.frames >= m, f + 3);
  expect(await page.evaluate(() => window.__zoo!.app.text_textures_dirty())).toBe(false);
  const enSpec = await page.evaluate(() => JSON.parse(window.__zoo!.app.text_textures()));
  expect(enSpec[0].text).toBe(en['sign-food-storage']);
  const enRows = await letterRows();
  expect(enRows.hash).not.toBe(deRows.hash);
  expect(enRows.height / vp.height).toBeGreaterThanOrEqual(0.03);
  expect(errors).toEqual([]);
});

test('screenshots PoC M4b: zebra sign + Futter sign; info board panel with facts', async ({ page }) => {
  const errors = await start(page);
  // both signs from the default camera, player on the path south of them (no panel open)
  await goto(page, -4.8, 9.3);
  await page.keyboard.down('KeyS');
  await page.waitForTimeout(60);
  await page.keyboard.up('KeyS');
  await expect(page.locator('#panel')).toBeHidden();
  await defaultCamera(page);
  await page.screenshot({ path: path.join(shots, 'screenshot_poc_m4b_signs.png') });
  // info board with facts (klasse2 shows several sentences)
  await page.evaluate(() => window.__zoo!.app.set_reading_level('klasse2'));
  await goto(page, -7.5, 14.5);
  await page.keyboard.down('KeyA');
  await page.waitForTimeout(80);
  await page.keyboard.up('KeyA');
  await expect(page.locator('#panel-facts')).toBeVisible({ timeout: 10_000 });
  await page.screenshot({ path: path.join(shots, 'screenshot_poc_m4b_board_facts.png') });
  expect(errors).toEqual([]);
});
