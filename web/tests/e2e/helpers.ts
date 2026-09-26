// Shared e2e helpers: wait for the game, read Fluent files, scripted player via the debug
// API (`debug_goto` + `debug_step` run the real movement/collision in zoo-core).
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { expect, type Page } from '@playwright/test';

const here = path.dirname(fileURLToPath(import.meta.url));
export const repo = path.resolve(here, '../../..');
export const shots = path.join(repo, 'art/environment/poc');

/** Messages of all .ftl files of a language (simple `key = value` lines). */
export function ftl(lang: string): Record<string, string> {
  const dir = path.join(repo, 'assets/i18n', lang);
  const out: Record<string, string> = {};
  for (const f of fs.readdirSync(dir).filter((n) => n.endsWith('.ftl'))) {
    for (const line of fs.readFileSync(path.join(dir, f), 'utf8').split('\n')) {
      const m = /^([a-z][a-z0-9_-]*)\s*=\s*(.*)$/.exec(line);
      if (m) out[m[1]] = m[2].trim();
    }
  }
  return out;
}

export async function waitFrames(page: Page, n: number): Promise<void> {
  await page.waitForFunction(
    (min) => Boolean(window.__zooError) || (window.__zoo?.frames ?? 0) >= min,
    n,
    { timeout: 90_000 },
  );
  expect(await page.evaluate(() => window.__zooError ?? null)).toBeNull();
}

/** Lets `n` more animation frames run (UI update). */
export async function nextFrames(page: Page, n = 2): Promise<void> {
  const f = await page.evaluate(() => window.__zoo!.frames);
  await page.waitForFunction((m) => window.__zoo!.frames >= m, f + n);
}

/** Scripted walk to a level point with the real movement rules. */
export async function goto(page: Page, x: number, z: number): Promise<void> {
  const arrived = await page.evaluate(
    ([px, pz]) => {
      const a = window.__zoo!.app;
      a.debug_goto(px, pz);
      return a.debug_step(90);
    },
    [x, z],
  );
  expect(arrived, `walk to ${x}, ${z}`).toBe(true);
  await nextFrames(page);
}

/** Short key tap: turns the player towards a direction (and moves a few cm). */
export async function face(page: Page, code: string): Promise<void> {
  await page.keyboard.down(code);
  await nextFrames(page, 2);
  await page.keyboard.up(code);
  await nextFrames(page, 2);
}

export async function state(page: Page) {
  return page.evaluate(() => {
    const a = window.__zoo!.app;
    return {
      x: a.player_x(),
      z: a.player_z(),
      target: a.target_key(),
      carry: a.carry_food(),
      zebra: a.animal_state('zebra'),
      started: a.mission_started('zebra'),
      complete: a.mission_complete('zebra'),
    };
  });
}
