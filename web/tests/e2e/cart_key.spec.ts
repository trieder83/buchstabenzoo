// GAME-CART key part (P2): the note on the desk, the key box with its lock panel, the key chip,
// the hint chain note -> key box, 3 wrong codes pulse the note, the math level setting, and the
// small screens (CART-013/014/022/024/030, HINT-032 e2e, MATH-010).
import { expect, test, type Page } from '@playwright/test';
import { ftl, nextFrames, shots, waitFrames, START_URL } from './helpers';

// the note on the desk (level 1): stand (-11, 2) facing the note at (-10.5, 3.7); the key box
// on the east facade at (-7.95, 1.5), stand (-8, 1)
const NOTE = { stand: [-10.5, 2.5], at: [-10.5, 3.7] };
const BOX = { stand: [-7.5, 1.5], at: [-7.95, 1.5] };

async function start(page: Page, w = 1280, h = 720, level = 'klasse1', math?: string) {
  await page.setViewportSize({ width: w, height: h });
  await page.addInitScript(
    ([lv, m]) => {
      if (sessionStorage.getItem('zoo.e2e.init')) return;
      sessionStorage.setItem('zoo.e2e.init', '1');
      localStorage.clear();
      localStorage.setItem('zoo.readingLevel', lv as string);
      if (m) localStorage.setItem('zoo.mathLevel', m as string);
    },
    [level, math ?? ''],
  );
  const errors: string[] = [];
  page.on('console', (m) => {
    if (m.type() === 'error') errors.push(m.text());
  });
  page.on('pageerror', (e) => errors.push(String(e)));
  await page.goto(START_URL);
  await waitFrames(page, 3);
  // no celebration / intro in the way
  await page.evaluate(() => {
    document.getElementById('celebrate')!.hidden = true;
  });
  return errors;
}

async function stand(page: Page, p: { stand: number[]; at: number[] }) {
  await page.evaluate((p) => {
    const a = window.__zoo!.app;
    a.debug_teleport(p.stand[0], p.stand[1]);
    a.debug_face_point(p.at[0], p.at[1]);
  }, p);
  await nextFrames(page, 4);
}

async function keyState(page: Page) {
  return JSON.parse(await page.evaluate(() => window.__zoo!.app.cart_key_json())) as {
    has_key: boolean;
    box_open: boolean;
    note_read: boolean;
    tries: number;
    help: boolean;
    aid: boolean;
  };
}

/** The answer of the note task from its numerals (`7 + 5 = ?`, mathe1/2 operators). */
function answerOf(expr: string): number {
  const m = /^(\d+) ([+−×÷]) (\d+) = \?$/.exec(expr.trim());
  expect(m, `expression ${expr}`).not.toBeNull();
  const [a, op, b] = [Number(m![1]), m![2], Number(m![3])];
  return op === '+' ? a + b : op === '−' ? a - b : op === '×' ? a * b : a / b;
}

/** Sets the three wheels with the big ▲ buttons (touch-like clicks). */
async function setWheels(page: Page, code: number) {
  const digits = String(code).padStart(3, '0').split('').map(Number);
  for (let i = 0; i < 3; i++) {
    const cur = Number((await page.locator(`#lock-digit-${i}`).textContent()) ?? '0');
    const ups = (digits[i] - cur + 10) % 10;
    for (let k = 0; k < ups; k++) await page.locator(`#lock-up-${i}`).click();
    await expect(page.locator(`#lock-digit-${i}`)).toHaveText(String(digits[i]));
  }
}

test('CART-024 / CART-013 / CART-022 / HINT-032: note -> key box -> wrong codes -> note -> key', async ({ page }) => {
  const errors = await start(page);
  const t = ftl('de');
  // nothing read yet: the first cart hint stage is the note; the key box is a closed box
  expect(await page.evaluate(() => window.__zoo!.app.debug_cart_hint())).toBe('cart-note');
  expect(await page.evaluate(() => window.__zoo!.app.debug_key_box_shown())).toBe('closed');
  expect((await keyState(page)).note_read).toBe(false);

  // read the note: it opens by itself at the desk
  await stand(page, NOTE);
  await expect(page.locator('#panel[data-kind="note"]')).toBeVisible();
  await expect(page.locator('#panel-title')).toContainText(t['cart-note-title']);
  const expr = (await page.locator('#note-expr').textContent())!;
  await expect(page.locator('#note-expr')).toHaveAttribute('data-level', 'mathe1');
  const answer = answerOf(expr);
  expect(answer).toBeGreaterThanOrEqual(1);
  expect(answer).toBeLessThanOrEqual(19);
  await expect(page.locator('#note-digits')).toHaveText('☐☐☐');
  await expect(page.locator('#note-keybox')).toBeVisible();
  await expect(page.locator('#note-line')).toHaveText(t['cart-note-line-klasse1']);
  await expect(page.locator('#note-aid')).toHaveCount(0);
  await page.screenshot({ path: `${shots}/cart-note.png` });
  let s = await keyState(page);
  expect(s.note_read).toBe(true);
  expect(s.tries).toBe(0);
  // stage 2: the key box
  expect(await page.evaluate(() => window.__zoo!.app.debug_cart_hint())).toBe('cart-keybox');

  // the key box: interact (E) opens the lock panel; the interact button shows 🔑
  await page.keyboard.press('Escape');
  await stand(page, BOX);
  expect(await page.evaluate(() => window.__zoo!.app.target_kind())).toBe('key_box');
  await expect(page.locator('#hint .icon')).toHaveText('🔑');
  await page.keyboard.press('KeyE');
  await expect(page.locator('#lock-panel')).toBeVisible();
  await expect(page.locator('#lock-note')).toBeHidden();
  // big ▲ / ▼ buttons, big ✔, ✖
  for (const sel of ['#lock-up-0', '#lock-up-1', '#lock-up-2', '#lock-down-0', '#lock-down-1', '#lock-down-2']) {
    const b = (await page.locator(sel).boundingBox())!;
    expect(Math.min(b.width, b.height), sel).toBeGreaterThanOrEqual(71.5);
  }
  const ok = (await page.locator('#lock-ok').boundingBox())!;
  expect(Math.min(ok.width, ok.height)).toBeGreaterThanOrEqual(71.5);
  // paused: a held key does not move her
  const p0 = await page.evaluate(() => [window.__zoo!.app.player_x(), window.__zoo!.app.player_z()]);
  await page.keyboard.down('KeyW');
  await page.waitForTimeout(300);
  await page.keyboard.up('KeyW');
  expect(await page.evaluate(() => [window.__zoo!.app.player_x(), window.__zoo!.app.player_z()])).toEqual(p0);

  // three wrong codes (000 is never the answer): shake, digits stay, 📝 pulses after the 3rd
  for (let i = 1; i <= 3; i++) {
    await page.locator('#lock-ok').click();
    await expect(page.locator('#lock-panel')).toBeVisible();
    expect((await keyState(page)).tries).toBe(i);
    await expect(page.locator('#lock-digit-2')).toHaveText('0');
    await expect(page.locator('#lock-note')).toBeVisible({ visible: i === 3 });
  }
  expect(await page.evaluate(() => window.__zoo!.app.debug_cart_hint())).toBe('cart-note');
  expect((await keyState(page)).help).toBe(true);
  expect((await keyState(page)).has_key).toBe(false);
  await page.screenshot({ path: `${shots}/cart-lock-wrong.png` });
  // ✖ / Esc closes; the game runs again
  await page.keyboard.press('Escape');
  await expect(page.locator('#lock-panel')).toBeHidden();

  // read the note again: tries restart, the visual aid appears (3 wrong codes), the hint is the box
  await stand(page, NOTE);
  await expect(page.locator('#panel[data-kind="note"]')).toBeVisible();
  s = await keyState(page);
  expect(s.tries).toBe(0);
  expect(s.aid).toBe(true);
  await expect(page.locator('#note-aid')).toBeVisible();
  expect(await page.evaluate(() => window.__zoo!.app.debug_cart_hint())).toBe('cart-keybox');
  await page.screenshot({ path: `${shots}/cart-note-aid.png` });
  await page.keyboard.press('Escape');

  // the right code with the big wheels: the box opens, the key is in the pocket
  await stand(page, BOX);
  await page.keyboard.press('KeyE');
  await expect(page.locator('#lock-panel')).toBeVisible();
  await expect(page.locator('#lock-digit-0')).toHaveText('0'); // start 000
  await setWheels(page, answer);
  await page.locator('#lock-ok').click();
  await expect(page.locator('#lock-panel')).toBeHidden();
  s = await keyState(page);
  expect(s.has_key && s.box_open).toBe(true);
  await expect(page.locator('#hud-key')).toBeVisible();
  await expect(page.locator('#hud-key')).toHaveText('🔑');
  await expect(page.locator('#bubble')).toHaveText(t['cart-key-got']);
  expect(await page.evaluate(() => window.__zoo!.app.debug_key_box_shown())).toBe('open');
  await nextFrames(page, 3);
  await page.screenshot({ path: `${shots}/cart-key-got.png` });
  // the open box is no interaction and no hint any more
  expect(await page.evaluate(() => window.__zoo!.app.target_kind())).toBe('');
  expect(await page.evaluate(() => window.__zoo!.app.debug_cart_hint())).toBe('');
  expect(errors).toEqual([]);
});

test('CART-014 keyboard: arrows and digits set the wheels, Enter opens', async ({ page }) => {
  await start(page);
  await stand(page, NOTE);
  const answer = answerOf((await page.locator('#note-expr').textContent())!);
  await page.keyboard.press('Escape');
  await stand(page, BOX);
  await page.keyboard.press('KeyE');
  await expect(page.locator('#lock-panel')).toBeVisible();
  const code = String(answer).padStart(3, '0');
  // digits typed, the selection moves right
  for (const ch of code) await page.keyboard.press(`Digit${ch}`);
  for (let i = 0; i < 3; i++) await expect(page.locator(`#lock-digit-${i}`)).toHaveText(code[i]);
  // arrows: down on the last wheel and back up
  await page.keyboard.press('ArrowDown');
  await page.keyboard.press('ArrowUp');
  await page.keyboard.press('Enter');
  await expect(page.locator('#lock-panel')).toBeHidden();
  expect((await keyState(page)).has_key).toBe(true);
});

test('MATH-010 / CART-030: the math row under the reading row, stored as zoo.mathLevel', async ({ page }) => {
  await start(page);
  await page.locator('#settings-btn').click();
  const row = page.locator('#settings-math');
  await expect(row).toBeVisible();
  // directly under the reading level row
  const [lv, mv] = await Promise.all([page.locator('#settings-level').boundingBox(), row.boundingBox()]);
  expect(mv!.y).toBeGreaterThan(lv!.y);
  const buttons = row.locator('button.choice');
  await expect(buttons).toHaveCount(5);
  for (const b of await buttons.all()) {
    const bb = (await b.boundingBox())!;
    expect(Math.min(bb.width, bb.height)).toBeGreaterThanOrEqual(63.5);
  }
  // numeral over dots
  await expect(buttons.nth(2).locator('.num')).toHaveText('3');
  await expect(buttons.nth(2).locator('.dots')).toHaveText('•••');
  // default mathe1, highlighted
  expect(await page.evaluate(() => window.__zoo!.app.math_level())).toBe('mathe1');
  await expect(buttons.nth(0)).toHaveClass(/on/);
  await buttons.nth(2).click();
  expect(await page.evaluate(() => window.__zoo!.app.math_level())).toBe('mathe3');
  expect(await page.evaluate(() => localStorage.getItem('zoo.mathLevel'))).toBe('mathe3');
  await expect(buttons.nth(2)).toHaveClass(/on/);
  // an unknown id is refused
  expect(await page.evaluate(() => window.__zoo!.app.set_math_level('mathe9'))).toBe(false);
  expect(await page.evaluate(() => window.__zoo!.app.math_level())).toBe('mathe3');
  // the note follows the level at once (mathe3: answers up to 999 incl. mul/div/big sums)
  await page.keyboard.press('Escape');
  await stand(page, NOTE);
  await expect(page.locator('#note-expr')).toHaveAttribute('data-level', 'mathe3');
  // a reload keeps it
  await page.reload();
  await waitFrames(page, 3);
  expect(await page.evaluate(() => window.__zoo!.app.math_level())).toBe('mathe3');
});

test('CART-024 kiga: pictograms, no reading needed; CART-026: the level change resets the tries', async ({ page }) => {
  await start(page, 1280, 720, 'kiga');
  const t = ftl('de');
  await stand(page, NOTE);
  await expect(page.locator('#panel[data-kind="note"]')).toBeVisible();
  await expect(page.locator('#note-line')).toHaveText(t['cart-note-line-kiga']);
  await expect(page.locator('#panel-text')).not.toHaveText('');
  // wrong codes, then a level change: the tries restart and the box stays closed
  await page.keyboard.press('Escape');
  await stand(page, BOX);
  await page.keyboard.press('KeyE');
  await page.locator('#lock-ok').click();
  await page.locator('#lock-ok').click();
  expect((await keyState(page)).tries).toBe(2);
  await page.keyboard.press('Escape');
  await page.evaluate(() => window.__zoo!.app.set_math_level('mathe2'));
  const s = await keyState(page);
  expect(s.tries).toBe(0);
  expect(s.box_open).toBe(false);
});

for (const [w, h] of [
  [780, 360],
  [360, 780],
] as const) {
  test(`CART-014 / CART-030 fits ${w}x${h}: lock panel, note panel and settings row`, async ({ page }) => {
    await start(page, w, h, 'klasse3');
    const inside = async (sel: string, min = 0) => {
      const b = (await page.locator(sel).boundingBox())!;
      expect(b, sel).not.toBeNull();
      expect(b.x, sel).toBeGreaterThanOrEqual(-0.5);
      expect(b.y, sel).toBeGreaterThanOrEqual(-0.5);
      expect(b.x + b.width, sel).toBeLessThanOrEqual(w + 0.5);
      expect(b.y + b.height, sel).toBeLessThanOrEqual(h + 0.5);
      if (min) expect(Math.min(b.width, b.height), sel).toBeGreaterThanOrEqual(min - 0.5);
      return b;
    };
    // the note panel (with the aid: the longest content)
    await page.evaluate(() => {
      const a = window.__zoo!.app;
      a.debug_teleport(-7.5, 1.5);
      a.debug_face_point(-7.95, 1.5);
    });
    await nextFrames(page, 3);
    await page.keyboard.press('KeyE');
    await expect(page.locator('#lock-panel')).toBeVisible();
    await page.locator('#lock-ok').click();
    await page.locator('#lock-ok').click();
    await page.locator('#lock-ok').click();
    for (const sel of ['#lock-up-0', '#lock-up-1', '#lock-up-2', '#lock-down-0', '#lock-down-1', '#lock-down-2']) await inside(sel, 72);
    await inside('#lock-ok', 72);
    await inside('#lock-close', 64);
    await inside('#lock-note');
    await inside('.lock-card');
    await page.screenshot({ path: `${shots}/cart-lock-${w}x${h}.png` });
    await page.keyboard.press('Escape');
    await stand(page, NOTE);
    await expect(page.locator('#panel[data-kind="note"]')).toBeVisible();
    await page.screenshot({ path: `${shots}/cart-note-${w}x${h}.png` });
    const body = await inside('.panel-body[data-kind="note"]');
    // the task, the three digit boxes and the key box picture are visible without scrolling
    for (const sel of ['#note-expr', '#note-digits', '#note-keybox']) {
      const b = (await page.locator(sel).boundingBox())!;
      expect(b.y, sel).toBeGreaterThanOrEqual(body.y - 0.5);
      expect(b.y + b.height, sel).toBeLessThanOrEqual(body.y + body.height + 0.5);
    }
    await expect(page.locator('#note-aid')).toBeVisible();
    await page.screenshot({ path: `${shots}/cart-note-${w}x${h}.png` });
    await page.keyboard.press('Escape');
    // the settings menu with the math row: fits, scrolls where short, the buttons stay >= 64 px
    await page.locator('#settings-btn').click();
    const menu = (await page.locator('#settings').boundingBox())!;
    expect(menu.y + menu.height).toBeLessThanOrEqual(h + 0.5);
    expect(menu.x + menu.width).toBeLessThanOrEqual(w + 0.5);
    expect(menu.x).toBeGreaterThanOrEqual(-0.5);
    for (const b of await page.locator('#settings-math button.choice').all()) {
      await b.scrollIntoViewIfNeeded();
      const bb = (await b.boundingBox())!;
      expect(Math.min(bb.width, bb.height)).toBeGreaterThanOrEqual(63.5);
      expect(bb.x).toBeGreaterThanOrEqual(menu.x - 0.5);
      expect(bb.x + bb.width).toBeLessThanOrEqual(menu.x + menu.width + 0.5);
    }
    await page.screenshot({ path: `${shots}/cart-settings-${w}x${h}.png` });
  });
}
