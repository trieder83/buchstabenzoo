// RESC-028: standing in front of the big map board at the entrance opens the welcome panel
// with the game description and the animals of the level, in the reading level and language.
import { expect, test, type Page } from '@playwright/test';
import { ftl, goto, nextFrames, waitFrames, START_URL } from './helpers';

test.describe.configure({ timeout: 120_000 });
test.use({ viewport: { width: 1280, height: 720 } });

async function start(page: Page, lang: string, level: string) {
  await page.addInitScript(
    ([l, r]) => {
      if (sessionStorage.getItem('zoo.e2e.init')) return;
      sessionStorage.setItem('zoo.e2e.init', '1');
      localStorage.clear();
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
  await page.goto(START_URL);
  await waitFrames(page, 3);
  return errors;
}

// the map board of level 1 stands at (-6.5, 4.0); its readable side faces the spawn (east)
async function standInFront(page: Page) {
  await goto(page, -4.9, 4.0);
  await page.evaluate(() => window.__zoo!.app.debug_face_point(-6.5, 4.0));
  await nextFrames(page, 2);
  await page.evaluate(() => window.__zoo!.app.debug_step(0.6));
  await nextFrames(page, 2);
}

for (const [lang, level] of [
  ['de', 'klasse2'],
  ['en', 'klasse1'],
  ['de', 'kiga'],
] as const) {
  test(`RESC-028: welcome board shows the game description (${lang} ${level})`, async ({ page }) => {
    const t = ftl(lang);
    const errors = await start(page, lang, level);
    await standInFront(page);
    expect(await page.evaluate(() => window.__zoo!.app.target_key())).toBe('welcome_board:level_1');
    await expect(page.locator('#panel')).toBeVisible();
    await expect(page.locator('#panel-text')).toHaveText(t[`welcome-${level}`]);
    // the animals of level 1 (names via Fluent) and the four steps
    const goal = await page.locator('#welcome-goal').innerText();
    for (const a of ['zebra', 'hippo', 'panda']) expect(goal).toContain(t[`animal-${a}`]);
    await expect(page.locator('#welcome-steps li')).toHaveCount(4);
    await expect(page.locator('#welcome-level')).toHaveText(t[`welcome-level-level_1-${level}`]);
    expect(await page.locator('#panel').innerText()).not.toMatch(/\bwelcome-[a-z0-9_-]+/);
    // not a mission board: nothing started
    expect(await page.evaluate(() => window.__zoo!.app.mission_started('zebra'))).toBe(false);
    expect(errors).toEqual([]);
  });
}
