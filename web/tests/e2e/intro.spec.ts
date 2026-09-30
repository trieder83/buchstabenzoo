// RESC-029: the intro at the entrance gate — 3 pages, skippable, never repeated, ends with
// the first hint (GAME-RESCUE "Intro at the entrance gate").
import { expect, test } from '@playwright/test';
import { ftl, waitFrames } from './helpers';

test('RESC-029 intro at the gate: 3 pages, then the first hint, not repeated after a reload', async ({ page }) => {
  await page.addInitScript(() => {
    if (sessionStorage.getItem('zoo.e2e.init')) return;
    sessionStorage.setItem('zoo.e2e.init', '1');
    localStorage.clear();
    localStorage.setItem('zoo.readingLevel', 'klasse2');
  });
  await page.goto('/?seed=17&intro=1');
  await waitFrames(page, 3);
  const de = ftl('de');
  const intro = page.locator('#intro');
  await expect(intro).toBeVisible();
  await expect(intro.locator('.intro-text')).toHaveText(de['intro-1-klasse2']);
  await page.locator('#intro .intro-next').click();
  await expect(intro.locator('.intro-text')).toHaveText(de['intro-2-klasse2']);
  await page.locator('#intro .intro-next').click();
  await expect(intro.locator('.intro-text')).toHaveText(de['intro-3-klasse2']);
  await page.locator('#intro .intro-next').click();
  await expect(intro).toBeHidden();
  // the first step is shown with the hint (RESC-029: "ends with the first hint target")
  const json = await page.evaluate(() => window.__zoo!.app.hint_json());
  expect(json).not.toBe('');
  // saved: a reload continues the game without the intro
  await page.evaluate(() => window.__zoo!.slot.flush());
  await page.goto('/?seed=17&intro=1');
  await waitFrames(page, 3);
  await expect(page.locator('#intro')).toBeHidden();
});
