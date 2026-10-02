// RESC-028 QA 2026-10-02: the welcome panel is longer than its box (max 38 % of the height).
// Everything below the first lines (steps, level note, start hint) must be reachable: by wheel
// on desktop and by a finger drag on a phone, and the player must not move while dragging.
import { expect, test } from '@playwright/test';
import { goto, nextFrames, waitFrames, START_URL } from '../helpers';

test.describe.configure({ timeout: 120_000 });

for (const [name, vp] of [
  ['desktop', { width: 1280, height: 720 }],
  ['phone portrait', { width: 390, height: 844 }],
] as const) {
  test(`RESC-028 QA: welcome panel scrolls to the start hint (${name})`, async ({ browser }) => {
    const ctx = await browser.newContext({ viewport: vp, hasTouch: name !== 'desktop' });
    const page = await ctx.newPage();
    await page.addInitScript(() => {
      if (sessionStorage.getItem('i')) return;
      sessionStorage.setItem('i', '1');
      localStorage.clear();
      localStorage.setItem('zoo.language', 'de');
      localStorage.setItem('zoo.readingLevel', 'klasse1');
    });
    await page.goto(START_URL);
    await waitFrames(page, 3);
    await goto(page, -4.9, 4.0);
    await page.evaluate(() => window.__zoo!.app.debug_face_point(-6.5, 4.0));
    await nextFrames(page, 2);
    await page.evaluate(() => window.__zoo!.app.debug_step(0.6));
    await nextFrames(page, 3);
    const body = page.locator('#panel .panel-body');
    await expect(body).toBeVisible();
    const m = () => body.evaluate((e) => ({ top: e.scrollTop, max: e.scrollHeight - e.clientHeight }));
    expect((await m()).max).toBeGreaterThan(50); // it really overflows
    const box = (await body.boundingBox())!;
    if (name === 'desktop') {
      await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
      await page.mouse.wheel(0, 2000);
    } else {
      const cdp = await ctx.newCDPSession(page);
      const x = box.x + box.width / 2;
      const y0 = box.y + box.height - 20;
      await cdp.send('Input.dispatchTouchEvent', { type: 'touchStart', touchPoints: [{ x, y: y0 }] });
      for (let i = 1; i <= 12; i++) {
        await cdp.send('Input.dispatchTouchEvent', { type: 'touchMove', touchPoints: [{ x, y: y0 - i * 20 }] });
      }
      await cdp.send('Input.dispatchTouchEvent', { type: 'touchEnd', touchPoints: [] });
    }
    await nextFrames(page, 4);
    const after = await m();
    expect(after.top).toBeGreaterThan(20);
    // scrolling far enough shows the start hint inside the panel box
    await page.evaluate(() => { const b = document.querySelector('#panel .panel-body')!; b.scrollTop = b.scrollHeight; });
    const s = (await page.locator('#welcome-start').boundingBox())!;
    const b2 = (await body.boundingBox())!;
    expect(s.y + s.height).toBeLessThanOrEqual(b2.y + b2.height + 2);
  });
}
