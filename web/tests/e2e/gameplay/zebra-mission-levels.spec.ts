// POC-002 / POC-003 / RESC-010 at every reading level in de and en, played like a child:
// shows the food before having any, picks the wrong box first, tries the wrong enclosure
// (RESC-007), walks away until the zebra waits and comes back (RESC-006), then leads it home
// (RESC-008). Every panel and bubble must show real text — never a raw Fluent key.
import { expect, test, type Page } from '@playwright/test';
import { ftl, goto, nextFrames, state, approach } from '../helpers';
import { ensurePanel, hold, norm, RAW_KEY, startGame, teleport, textOf, turn, wait, zebra } from './qa';

test.describe.configure({ timeout: 300_000 });
test.use({ viewport: { width: 1280, height: 720 } });

const LEVELS = ['kiga', 'klasse1', 'klasse2', 'klasse3'] as const;
const LANGS = ['de', 'en'] as const;

async function pressE(page: Page) {
  await page.keyboard.press('KeyE');
  await nextFrames(page, 2);
}

/** Last feedback bubble text (it hides itself after 2.5 s, so read the text, not visibility). */
async function bubble(page: Page) {
  return norm((await page.locator('#bubble').textContent()) ?? '');
}

for (const lang of LANGS) {
  for (const level of LEVELS) {
    test(`RESC-010 / POC-00${lang === 'de' ? 2 : 3}: zebra mission ${lang} ${level}, with detours`, async ({ page }) => {
      const t = ftl(lang);
      const errors = await startGame(page, lang, level);
      const spot = await zebra(page);

      // No food yet: the zebra says it is hungry (gentle feedback, no dead end).
      expect(Math.hypot(spot.x - 8.5, spot.z - 32.5)).toBeLessThan(3.1); // loc_river (seed 17)
      await approach(page, 'zebra');
      expect((await state(page)).target).toBe('animal:zebra');
      await pressE(page);
      await expect.poll(() => bubble(page)).toBe(norm(t['ui-no-food']));

      // Info board: riddle for this level, the food word, no raw keys.
      await goto(page, -7.5, 15.5);
      await turn(page, 'KeyA');
      expect((await state(page)).target).toBe('info_board:zebra');
      await ensurePanel(page, () => pressE(page));
      const board = await textOf(page, '#panel');
      expect(board).toContain(norm(t[`mission-zebra-riddle-loc_river-${level}`]));
      expect(board).toContain(norm(t['food-grass']));
      expect(board).not.toMatch(RAW_KEY);
      expect((await state(page)).started).toBe(true);
      await page.keyboard.press('Escape');
      await expect(page.locator('#panel')).toBeHidden();

      // Wrong box first (bamboo), shown to the zebra: not interested.
      for (const [food, x] of [
        ['bamboo', -1.2],
        ['grass', -0.4],
      ] as const) {
        await goto(page, x, 9.6);
        await turn(page, 'KeyW');
        expect((await state(page)).target).toBe(`food_box:${food}`);
        await ensurePanel(page, () => pressE(page));
        expect(await textOf(page, '#panel')).toContain(norm(t[`food-${food}`]));
        await page.locator('#take').click();
        await expect(page.locator('#panel')).toBeHidden();
        expect((await state(page)).carry).toBe(food);
        await approach(page, 'zebra');
        await pressE(page);
        const said = await bubble(page);
        expect(said).not.toMatch(RAW_KEY);
        if (food === 'bamboo') {
          expect(said).toBe(norm(t['ui-not-interested']));
          expect((await zebra(page)).state).toBe('escaped');
        } else {
          expect(said).toBe(norm(t['ui-following']));
          expect((await zebra(page)).state).toBe('following');
        }
      }

      // Wrong enclosure: walk into the hippo gate (9, 15..16) — the zebra refuses and keeps following.
      await goto(page, 7.5, 16.0);
      await wait(page, 3);
      await hold(page, ['KeyD'], 2);
      await nextFrames(page, 2);
      expect(await bubble(page)).toBe(norm(t['ui-refuse']));
      expect((await zebra(page)).state).toBe('following');
      await hold(page, ['KeyA'], 1.5);

      // Walk away (> 15 m): the zebra waits where it is …
      const before = await zebra(page);
      await teleport(page, -7.0, 5.0);
      await wait(page, 2);
      const waiting = await zebra(page);
      expect(Math.hypot(waiting.x - before.x, waiting.z - before.z)).toBeLessThan(0.05);
      // … still waits at 6–8 m …
      await goto(page, 6.5, waiting.z - 7.0);
      await wait(page, 1.5);
      const still = await zebra(page);
      expect(Math.hypot(still.x - waiting.x, still.z - waiting.z)).toBeLessThan(0.05);
      // … and follows again within 5 m.
      await goto(page, 6.5, waiting.z - 3.5);
      await wait(page, 3);
      const again = await zebra(page);
      expect(Math.hypot(again.x - again.px, again.z - again.pz)).toBeLessThan(2.0);

      // Home: through the zebra gate (-10, 12..13).
      await goto(page, -8.4, 13.0);
      await wait(page, 3);
      await turn(page, 'KeyA');
      expect((await state(page)).target).toBe('gate:enc_zebra');
      await pressE(page);
      const done = await state(page);
      expect(done.zebra).toBe('in_enclosure');
      expect(done.complete).toBe(true);
      expect(done.carry).toBe('');
      await expect(page.locator('#celebrate-text')).toHaveText(t['mission-zebra-home']);
      // Completion triggers once: afterwards the gate is no target any more.
      await wait(page, 1);
      await turn(page, 'KeyA');
      expect((await state(page)).target).toBe('');
      expect(errors).toEqual([]);
    });
  }
}
