// Standing on surfaces (GAME-PLAYER rules 8–9, PLAY-036; user report 2026-09-27: the feet
// sank into the garden path, the bridge deck and the jetty). The scripted player walks over
// the arched bridge, the jetty, the garden path and into the zookeeper house frame by frame
// (1/60 s); every frame her drawn foot height must stay on `ground_height` under her (± 2 cm
// once settled) and change by at most 0.25 m per frame (no pops). Review shots:
// art/environment/poc/screenshot_ground_{bridge,garden}.png.
import path from 'node:path';
import { expect, test, type Page } from '@playwright/test';
import { nextFrames, shots, START_URL, waitFrames } from './helpers';

test.describe.configure({ timeout: 240_000 });
test.use({ viewport: { width: 1280, height: 720 } });

type Walk = { frames: number; maxJump: number; maxLag: number; settled: number; maxY: number; arrived: boolean };

/** Teleports to `from`, walks to `to` with the scripted player and samples every frame. */
async function walk(page: Page, from: number[], to: number[]): Promise<Walk> {
  return page.evaluate(
    ([from, to]) => {
      const a = window.__zoo!.app;
      a.debug_teleport(from[0], from[1]);
      a.debug_step(0.2);
      a.debug_goto(to[0], to[1]);
      let last = a.player_foot_y();
      const out = { frames: 0, maxJump: 0, maxLag: 0, settled: 0, maxY: last, arrived: false };
      for (let i = 0; i < 60 * 30 && !out.arrived; i++) {
        out.arrived = a.debug_step(1 / 60);
        const y = a.player_foot_y();
        const g = a.ground_height(a.player_x(), a.player_z());
        out.frames++;
        out.maxJump = Math.max(out.maxJump, Math.abs(y - last));
        out.maxLag = Math.max(out.maxLag, Math.abs(y - g));
        out.maxY = Math.max(out.maxY, y);
        last = y;
      }
      a.debug_step(0.5);
      out.settled = Math.abs(a.player_foot_y() - a.ground_height(a.player_x(), a.player_z()));
      return out;
    },
    [from, to],
  );
}

test.beforeEach(async ({ page }) => {
  await page.goto(START_URL);
  await waitFrames(page, 3);
  await page.evaluate(() => window.__zoo!.app.debug_pause(true));
});

// PLAY-036: bridge (arched deck up to 0.5 m), jetty (0.2 m), garden path, zookeeper house
test('PLAY-036 feet follow the surface on bridge, jetty, garden path and into the house', async ({ page }) => {
  const routes: [string, number[], number[], number, number][] = [
    // name, from, to, highest ground on the way (m), largest lag while walking (m): the
    // bridge slope is followed at once, the jetty edge (0.13 m) is eased over a few frames
    ['bridge', [8.5, 29.5], [14.5, 29.5], 0.5, 0.03],
    ['jetty', [-6.5, 23.0], [-10.5, 23.0], 0.2, 0.16],
    ['garden', [8.0, 35.5], [8.0, 44.5], 0.075, 0.03],
    ['house', [-6.5, 2.5], [-11.5, 2.5], 0.075, 0.03],
  ];
  for (const [name, from, to, top, lag] of routes) {
    const w = await walk(page, from, to);
    expect(w.arrived, `${name}: walked`).toBe(true);
    expect(w.frames, `${name}: walked frame by frame`).toBeGreaterThan(90);
    expect(w.maxJump, `${name}: height change per frame`).toBeLessThanOrEqual(0.25);
    expect(w.maxLag, `${name}: feet follow the surface while walking`).toBeLessThanOrEqual(lag);
    expect(w.settled, `${name}: feet on the surface`).toBeLessThanOrEqual(0.02);
    expect(w.maxY, `${name}: up on the surface`).toBeGreaterThan(top - 0.03);
    expect(w.maxY, `${name}: never above it`).toBeLessThan(top + 0.03);
  }
  // the bridge top: feet at 0.5 m on the deck centre
  const top = await page.evaluate(() => {
    const a = window.__zoo!.app;
    a.debug_teleport(11.5, 29.5);
    a.debug_step(0.2);
    return [a.player_foot_y(), a.ground_height(11.5, 29.5)];
  });
  expect(top[0]).toBeCloseTo(0.5, 2);
  expect(top[1]).toBeCloseTo(0.5, 2);
});

test('PLAY-036 review shots: feet on the bridge deck and the garden path', async ({ page }) => {
  for (const [name, x, z] of [
    ['bridge', 11.2, 29.5],
    ['garden', 8.0, 39.5],
  ] as [string, number, number][]) {
    await page.evaluate(
      ([x, z]) => {
        const a = window.__zoo!.app;
        a.debug_teleport(x, z);
        a.debug_step(0.3);
        a.zoom(0.45);
      },
      [x, z],
    );
    await nextFrames(page, 3);
    await page.screenshot({ path: path.join(shots, `screenshot_ground_${name}.png`) });
    await page.evaluate(() => window.__zoo!.app.zoom(1 / 0.45));
  }
});
