// Helpers for the gameplay QA tests (qa/checklist.md). Deterministic: movement runs through
// `debug_step` (fixed 1/60 s steps with the real movement and collision), touches are real
// CDP touch events.
import { expect, type CDPSession, type Page } from '@playwright/test';
import { nextFrames, waitFrames, START_URL } from '../helpers';

/** Starts the game with stored settings and collects console errors. */
export async function startGame(page: Page, lang: string, level: string): Promise<string[]> {
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
  await page.goto(START_URL);
  await waitFrames(page, 3);
  return errors;
}

/** Holds keys for `seconds` of game time (fixed steps), then releases them. */
export async function hold(page: Page, keys: string[], seconds: number): Promise<{ x: number; z: number }> {
  return page.evaluate(
    ([ks, t]) => {
      const a = window.__zoo!.app;
      for (const k of ks) a.key(k, true);
      a.debug_step(t);
      for (const k of ks) a.key(k, false);
      a.debug_step(1 / 60);
      return { x: a.player_x(), z: a.player_z() };
    },
    [keys, seconds] as const,
  );
}

/** Teleports the player (debug) and snaps the camera. */
export async function teleport(page: Page, x: number, z: number): Promise<void> {
  await page.evaluate(
    ([px, pz]) => {
      window.__zoo!.app.debug_teleport(px, pz);
      window.__zoo!.app.debug_step(0);
    },
    [x, z],
  );
}

/** Turns the player towards a key direction without moving her (tap, then teleport back). */
export async function turn(page: Page, code: string): Promise<void> {
  await page.evaluate((k) => {
    const a = window.__zoo!.app;
    const [x, z] = [a.player_x(), a.player_z()];
    a.key(k, true);
    a.debug_step(1 / 60);
    a.key(k, false);
    a.debug_step(1 / 60);
    a.debug_teleport(x, z);
  }, code);
  await nextFrames(page, 2);
}

/** Lets game time pass (animals follow, panels open by themselves), then renders. */
export async function wait(page: Page, seconds: number): Promise<void> {
  await page.evaluate((t) => window.__zoo!.app.debug_step(t), seconds);
  await nextFrames(page, 2);
}

/**
 * Makes sure the reading panel of the target in front of the player is open: it may open
 * by itself (GAME-PLAYER §4, PLAY-023) — otherwise interact once.
 */
export async function ensurePanel(page: Page, interact: () => Promise<void>): Promise<void> {
  const panel = page.locator('#panel');
  await wait(page, 0.6);
  if (await panel.isHidden()) await interact();
  await expect(panel).toBeVisible();
}

/** Visible text of an element with collapsed white space. */
export async function textOf(page: Page, selector: string): Promise<string> {
  return page.evaluate((s) => (document.querySelector(s) as HTMLElement | null)?.innerText.replace(/\s+/g, ' ').trim() ?? '', selector);
}

/** A raw Fluent key shown to the child (e.g. `mission-panda-riddle-klasse2`). */
export const RAW_KEY = /\b(mission|food|ui|animal|sign)-[a-z0-9_]+(-[a-z0-9_]+)*\b/;

export function norm(s: string): string {
  return s.replace(/\s+/g, ' ').trim();
}

export async function zebra(page: Page) {
  return page.evaluate(() => {
    const a = window.__zoo!.app;
    return { state: a.animal_state('zebra'), x: a.animal_x('zebra'), z: a.animal_z('zebra'), px: a.player_x(), pz: a.player_z() };
  });
}

export type TouchPoint = { x: number; y: number; id: number };

export async function touch(cdp: CDPSession, type: 'touchStart' | 'touchMove' | 'touchEnd', points: TouchPoint[]) {
  await cdp.send('Input.dispatchTouchEvent', { type, touchPoints: points });
}
