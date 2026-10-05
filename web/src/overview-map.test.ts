import { describe, expect, it } from 'vitest';
import { fitMap, parseOverview, toPx } from './overview-map';

// MAP-020 (layout maths)
describe('overview map layout', () => {
  it('fits the whole zoo into the box with the aspect kept', () => {
    const f = fitMap([-72, -2, 148, 96], 700, 280);
    expect(f.width).toBeLessThanOrEqual(700);
    expect(f.height).toBeLessThanOrEqual(280);
    expect(Math.max(f.width / 700, f.height / 280)).toBeCloseTo(1, 5);
    const f2 = fitMap([-72, -2, 148, 96], 344, 600);
    expect(f2.width).toBeCloseTo(344, 3);
    expect(f2.height).toBeLessThan(600);
  });

  it('puts north at the top', () => {
    const f = fitMap([0, 0, 10, 10], 100, 100);
    expect(toPx(f, 0, 10)).toEqual([0, 0]);
    expect(toPx(f, 10, 0)).toEqual([100, 100]);
  });

  it('parses the data and ignores garbage', () => {
    expect(parseOverview('')).toBeNull();
    expect(parseOverview('{')).toBeNull();
    expect(parseOverview('{"parts":[]}')?.parts).toEqual([]);
  });
});
