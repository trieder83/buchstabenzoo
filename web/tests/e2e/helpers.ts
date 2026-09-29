// Shared e2e helpers: wait for the game, read Fluent files, scripted player via the debug
// API (`debug_goto` + `debug_step` run the real movement/collision in zoo-core).
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { expect, type Page } from '@playwright/test';

const here = path.dirname(fileURLToPath(import.meta.url));
export const repo = path.resolve(here, '../../..');
// Review screenshots: written to the tracked art/environment/poc/ only with UPDATE_SHOTS=1,
// otherwise to the ignored web/test-results/shots/ (keeps git diffs clean, saves agent tokens).
export const shots = process.env.UPDATE_SHOTS
  ? path.join(repo, 'art/environment/poc')
  : path.join(repo, 'web/test-results/shots');

/**
 * Start URL with a fixed seed (GAME-RESCUE §1 discovery): seed 17 puts the zebra at
 * `loc_river`, the panda at `loc_cave` and the hippo at `loc_pond` — the classic places the
 * scripted tests walk to. Other seeds pick other candidates (discovery.spec.ts).
 */
export const START_URL = '/?seed=17';

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

/**
 * Walks up to a (wandering) escaped animal and faces it (GAME-ANIMALS: it stops and looks at
 * the player within 3 m; out of reach in the pond it comes closer). Repeats the walk if the
 * animal moved meanwhile. Ends within the 2 m interaction range.
 */
export async function approach(page: Page, animal: string): Promise<void> {
  for (let i = 0; i < 4; i++) {
    const p = await page.evaluate((id) => window.__zoo!.app.debug_stand_near(id), animal);
    expect(p.length, `stand point near ${animal}`).toBe(2);
    await goto(page, p[0], p[1]);
    await page.evaluate(() => window.__zoo!.app.debug_step(1.0));
    const d = await page.evaluate((id) => {
      const a = window.__zoo!.app;
      return Math.hypot(a.animal_x(id) - a.player_x(), a.animal_z(id) - a.player_z());
    }, animal);
    if (d <= 1.95) break;
  }
  await page.evaluate((id) => window.__zoo!.app.debug_face_animal(id), animal);
  await nextFrames(page, 2);
}
