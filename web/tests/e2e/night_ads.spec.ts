// GAME-ADS ADS-002 / ADS-036: every level shows all three campaign slots; the night levels
// carry their boards as wall posters (and one flyer on the grass), lit and readable at night.
import path from 'node:path';
import { expect, test, type Page } from '@playwright/test';
import { nextFrames, shots, waitFrames, START_URL } from './helpers';

test.use({ viewport: { width: 1280, height: 720 } });

interface Board {
  id: string;
  slot: number;
  variant: string;
  x: number;
  z: number;
}

async function start(page: Page) {
  await page.addInitScript(() => {
    if (sessionStorage.getItem('zoo.e2e.init')) return;
    sessionStorage.setItem('zoo.e2e.init', '1');
    localStorage.clear();
    localStorage.setItem('zoo.readingLevel', 'klasse2');
  });
  await page.route('**/boards/**', (r) => r.abort()); // placeholders only
  await page.goto(START_URL);
  await waitFrames(page, 3);
  await page.evaluate(() => window.__zoo!.app.debug_set_daytime('night'));
  await nextFrames(page, 3);
}

const boards = async (page: Page): Promise<Board[]> => JSON.parse(await page.evaluate(() => window.__zoo!.app.ad_boards_json()));

const LEVELS: Record<string, (b: Board) => boolean> = {
  level_1: (b) => b.id.startsWith('ad_l1_'),
  level_2: (b) => b.id.startsWith('ad_l2_'),
  level_3: (b) => b.id.startsWith('ad_l3_'),
  night_1: (b) => b.id.startsWith('ad_n1_'),
  night_2: (b) => b.id.startsWith('ad_n2_'),
};

test('ADS-002 every level has all three slots; night levels exactly one board per slot', async ({ page }) => {
  await start(page);
  const list = await boards(page);
  expect(list.length).toBe(18);
  for (const [level, own] of Object.entries(LEVELS)) {
    const slots = list.filter(own).map((b) => b.slot);
    expect(new Set(slots), level).toEqual(new Set([1, 2, 3]));
    if (level.startsWith('night')) expect(slots.length, level).toBe(3);
  }
  expect(list.filter(LEVELS.night_1).map((b) => b.variant).sort()).toEqual(['flyer', 'poster', 'poster']);
  expect(list.filter(LEVELS.night_2).map((b) => b.variant)).toEqual(['flyer', 'poster', 'poster']);
});

for (const level of ['night_1', 'night_2']) {
  test(`ADS-036 ${level}: three boards at night, drawn with their picture, near from the front, passive placeholders`, async ({ page }) => {
    test.setTimeout(300_000);
    await start(page);
    const list = (await boards(page)).filter(LEVELS[level]);
    await page.waitForFunction((ids) => ids.every((id) => window.__zoo!.app.decal_drawn(`ad:${id}`)), list.map((b) => b.id), { timeout: 30_000 });
    for (const b of list) {
      const d = b.variant === 'flyer' ? 1.2 : 1.8;
      await page.evaluate(([x, z]) => window.__zoo!.app.debug_teleport(x, z), [b.x, b.z - d]);
      await page.evaluate(() => window.__zoo!.app.debug_step(0.3));
      await nextFrames(page, 4);
      expect(await page.evaluate(() => window.__zoo!.app.ad_near()), b.id).toBe(b.id);
      await expect(page.locator('#zb-panel')).toBeHidden(); // placeholder: passive
      expect(await page.evaluate(() => window.__zoo!.app.target_kind())).toBe('');
      await page.screenshot({ path: path.join(shots, `night_ads_${b.id}.png`) });
    }
  });
}
