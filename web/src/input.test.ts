import fs from 'node:fs';
import path from 'node:path';
import { describe, expect, it } from 'vitest';
import {
  DEAD_ZONE,
  isInteractKey,
  pinchFactor,
  STICK_RADIUS,
  stickVector,
  swipeSteps,
  TouchGestures,
  touchZone,
  wheelFactor,
  type InputSink,
  type StickView,
} from './input';

class Sink implements InputSink {
  stick: [number, number] = [0, 0];
  rotations: number[] = [];
  zooms: number[] = [];
  key(): boolean {
    return false;
  }
  set_stick(x: number, y: number): void {
    this.stick = [x, y];
  }
  drag(): void {}
  drag_end(): void {}
  rotate(steps: number): void {
    this.rotations.push(steps);
  }
  zoom(f: number): void {
    this.zooms.push(f);
  }
}

class View implements StickView {
  shown: number[] | null = null;
  show(ox: number, oy: number, kx: number, ky: number): void {
    this.shown = [ox, oy, kx, ky];
  }
  hide(): void {
    this.shown = null;
  }
}

describe('stickVector', () => {
  it('maps screen offsets to x right / y up', () => {
    const [x, y] = stickVector(0, -75, 75);
    expect(x).toBeCloseTo(0);
    expect(y).toBeCloseTo(1);
  });
  it('clamps to length 1 and has a 10 % dead zone (GAME-PLAYER §3)', () => {
    const [x, y] = stickVector(300, 0, 75);
    expect(Math.hypot(x, y)).toBeCloseTo(1);
    expect(DEAD_ZONE).toBe(0.1);
    expect(stickVector(0, -0.09 * 60, 60)).toEqual([0, 0]);
    expect(stickVector(0, -0.11 * 60, 60)[1]).toBeCloseTo(0.11);
  });
  it('PLAY-015: 60 % deflection gives 60 % (not rescaled)', () => {
    const [, y] = stickVector(0, -0.6 * STICK_RADIUS, STICK_RADIUS);
    expect(y).toBeCloseTo(0.6);
  });
});

describe('zones, swipes, keys', () => {
  it('splits the screen at the centre', () => {
    expect(touchZone(10, 400)).toBe('left');
    expect(touchZone(199, 400)).toBe('left');
    expect(touchZone(200, 400)).toBe('right');
  });
  it('PLAY-016: a swipe needs ≥ 40 px and gives one step in its direction', () => {
    expect(swipeSteps(39)).toBe(0);
    expect(swipeSteps(40)).toBe(1);
    expect(swipeSteps(-200)).toBe(-1);
  });
  it('E, Space and Enter interact (FIX-024)', () => {
    expect(isInteractKey('KeyE')).toBe(true);
    expect(isInteractKey('Space')).toBe(true);
    expect(isInteractKey('Enter')).toBe(true);
    expect(isInteractKey('KeyQ')).toBe(false);
  });
});

describe('TouchGestures', () => {
  it('floating stick: appears under the thumb, releases to zero', () => {
    const sink = new Sink();
    const view = new View();
    const g = new TouchGestures(sink, view);
    g.down(1, 100, 500, 800);
    expect(view.shown).toEqual([100, 500, 100, 500]);
    g.move(1, 100, 500 - 36); // 60 % of 60 px up
    expect(sink.stick[0]).toBeCloseTo(0);
    expect(sink.stick[1]).toBeCloseTo(0.6);
    g.move(1, 400, 500); // beyond the radius: clamped, knob stays on the rim
    expect(Math.hypot(...sink.stick)).toBeCloseTo(1);
    expect(view.shown![2]).toBeCloseTo(160);
    g.up(1);
    expect(sink.stick).toEqual([0, 0]);
    expect(view.shown).toBeNull();
  });
  it('PLAY-016: one step per swipe, re-armed after lifting', () => {
    const sink = new Sink();
    const g = new TouchGestures(sink, new View());
    g.down(2, 500, 300, 800);
    g.move(2, 530, 300);
    expect(sink.rotations).toEqual([]);
    g.move(2, 545, 300);
    g.move(2, 700, 300);
    expect(sink.rotations).toEqual([1]);
    g.up(2);
    g.down(3, 700, 300, 800);
    g.move(3, 600, 300);
    expect(sink.rotations).toEqual([1, -1]);
  });
  it('PLAY-017: stick and swipe at the same time', () => {
    const sink = new Sink();
    const g = new TouchGestures(sink, new View());
    g.down(1, 100, 500, 800);
    g.move(1, 100, 440);
    g.down(2, 500, 300, 800);
    g.move(2, 560, 300);
    expect(sink.stick[1]).toBeCloseTo(1);
    expect(sink.rotations).toEqual([1]);
    expect(g.active).toBe(2);
  });
  it('two right fingers pinch-zoom instead of rotating', () => {
    const sink = new Sink();
    const g = new TouchGestures(sink, new View());
    g.down(5, 500, 300, 800);
    g.down(6, 600, 300, 800);
    g.move(6, 700, 300);
    expect(sink.rotations).toEqual([]);
    expect(sink.zooms[0]).toBeCloseTo(0.5);
  });
  it('reset releases everything (pointercancel / orientation change)', () => {
    const sink = new Sink();
    const g = new TouchGestures(sink, new View());
    g.down(1, 100, 500, 800);
    g.move(1, 100, 440);
    g.down(2, 500, 300, 800);
    g.reset();
    expect(sink.stick).toEqual([0, 0]);
    expect(g.active).toBe(0);
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
