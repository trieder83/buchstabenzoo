// ADS-023: the RELEASE build (production key compiled in) accepts the real signed manifest
// `boards/index.json` (+ .sig) of this repo and shows the three own campaigns on the boards.
// This is the check that the connection works end to end (key, signature, hashes, serving).
import { expect, test } from '@playwright/test';
import { START_URL } from './helpers';

test('ADS-023 the real signed manifest is accepted by the release build: campaigns 1, 2 and 3 are loaded', async ({ page }) => {
  const errors: string[] = [];
  page.on('pageerror', (e) => errors.push(String(e)));
  await page.goto(START_URL);
  await page.waitForFunction(() => window.__zoo?.ads?.state().settled, undefined, { timeout: 60_000 });
  const st = await page.evaluate(() => window.__zoo!.ads.state());
  expect(st.loaded, 'manifest verified').toBe(true);
  expect(st.campaigns).toEqual({ 1: 'mathfighter', 2: 'abcsmash', 3: 'edugamegalaxy' });
  expect(errors).toEqual([]);
});
