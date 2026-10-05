import { describe, expect, it } from 'vitest';
import { lineLayout, discBase, parsePlanets, planetSvg, type PlanetData } from './telescope';

// TELE-011 (line layout maths)
const SKIES: [number, number][] = [
  [554, 332], // 780x360 landscape
  [332, 522], // 360x780 portrait
  [940, 690],
  [1200, 700],
];
describe('telescope line layout', () => {
  it('orders the planets from lower left to upper right, evenly spaced, inside the sky', () => {
    for (const [w, h] of SKIES) {
      const c = lineLayout(w, h, 8).centres;
      expect(c.length).toBe(8);
      for (let i = 1; i < 8; i++) {
        expect(c[i].x).toBeGreaterThan(c[i - 1].x);
        expect(c[i].y).toBeLessThan(c[i - 1].y);
        const dx = c[i].x - c[i - 1].x;
        const dy = c[i - 1].y - c[i].y;
        expect(dx).toBeCloseTo(c[1].x - c[0].x, 5);
        expect(dy).toBeCloseTo(c[0].y - c[1].y, 5);
      }
      for (const p of c) {
        expect(p.x).toBeGreaterThanOrEqual(32);
        expect(p.x).toBeLessThanOrEqual(w - 32);
        expect(p.y).toBeGreaterThanOrEqual(32);
        expect(p.y).toBeLessThanOrEqual(h - 32);
      }
    }
  });

  it('keeps 64 px boxes apart (x or y) at the phone sizes', () => {
    for (const [w, h] of SKIES) {
      const c = lineLayout(w, h, 8).centres;
      for (let i = 1; i < 8; i++) {
        const sep = Math.max(c[i].x - c[i - 1].x, c[i - 1].y - c[i].y);
        expect(sep, `${w}x${h} step ${i}`).toBeGreaterThanOrEqual(64);
      }
    }
  });

  it('puts the sun just outside the lower-left corner and Mercury clear of its rim', () => {
    for (const [w, h] of SKIES) {
      const { sun, centres, step } = lineLayout(w, h, 8);
      expect(sun.x).toBeLessThan(0);
      expect(sun.y).toBeGreaterThan(h);
      expect(centres[0].x).toBeGreaterThan(0);
      // gap between the Sun's rim and Mercury's disc (radius ~ 0.2 x disc base) is at least 20 px
      const dist = Math.hypot(centres[0].x - sun.x, centres[0].y - sun.y);
      expect(dist - sun.r - 0.2 * discBase(step), `${w}x${h}`).toBeGreaterThanOrEqual(20);
    }
  });

  it('draws a planet svg for each id with unique gradient ids', () => {
    const ids = ['mercury', 'venus', 'earth', 'mars', 'jupiter', 'saturn', 'uranus', 'neptune'];
    for (const id of ids) {
      const svg = planetSvg({ id, colors: ['#111111', '#222222'], bands: 3, ring: id === 'saturn' } as PlanetData);
      expect(svg).toContain('<svg');
      expect(svg).toContain(`tp-${id}-`);
    }
  });

  it('parses the planet table and rejects garbage', () => {
    expect(parsePlanets(undefined)).toBeNull();
    expect(parsePlanets('nope')).toBeNull();
    expect(parsePlanets('{"planets":[]}')).toBeNull();
    expect(parsePlanets('{"planets":[{"id":"mars"}]}')?.length).toBe(1);
  });
});
