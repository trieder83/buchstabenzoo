import fs from 'node:fs';
import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { pinchFactor, stickVector, wheelFactor } from './input';

describe('stickVector', () => {
  it('maps screen offsets to x right / y up', () => {
    const [x, y] = stickVector(0, -75, 75);
    expect(x).toBeCloseTo(0);
    expect(y).toBeCloseTo(1);
  });
  it('clamps to length 1 and has a dead zone', () => {
    const [x, y] = stickVector(300, 0, 75);
    expect(Math.hypot(x, y)).toBeCloseTo(1);
    expect(stickVector(3, 3, 75)).toEqual([0, 0]);
  });
});

describe('zoom factors', () => {
  it('pinching out zooms in (factor < 1)', () => {
    expect(pinchFactor(100, 200)).toBeLessThan(1);
    expect(wheelFactor(100)).toBeGreaterThan(1);
    expect(wheelFactor(-100)).toBeLessThan(1);
  });
});

// ARCH-002: no 3D engine in the web build.
describe('ARCH-002', () => {
  it('node_modules contains no three.js/babylon/playcanvas package', () => {
    const nm = path.resolve(__dirname, '../node_modules');
    const names = fs.readdirSync(nm).flatMap((n) =>
      n.startsWith('@') ? fs.readdirSync(path.join(nm, n)).map((s) => `${n}/${s}`) : [n],
    );
    const banned = names.filter((n) => /^(three|@babylonjs\/.*|babylonjs.*|playcanvas)$/.test(n));
    expect(banned).toEqual([]);
  });
});
