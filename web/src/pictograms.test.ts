import { describe, expect, it } from 'vitest';
import { drawPictogram, PICTO_GRID, PICTOGRAM_IDS, type PictoCtx } from './pictograms';
import { labelLayout } from './text';

// the 14 foods of `Food::ALL` (zoo-core food.rs)
const FOODS = ['grass', 'melons', 'bamboo', 'eucalyptus', 'hay', 'fish_food', 'bananas', 'leaves', 'meat', 'berries', 'beetles', 'fruit', 'worms', 'nectar'];

/** A recording 2D context: every call as text, plus the path extents on the design grid. */
function recorder() {
  const log: string[] = [];
  let min = Infinity;
  let max = -Infinity;
  const note = (...n: number[]) => n.forEach((v) => ((min = Math.min(min, v)), (max = Math.max(max, v))));
  const target: Record<string, unknown> = {};
  const ctx = new Proxy(target, {
    get(t, k: string) {
      if (k in t) return t[k];
      return (...a: unknown[]) => {
        log.push(`${k}(${a.join(',')})`);
        const n = a.filter((v): v is number => typeof v === 'number');
        if (['moveTo', 'lineTo', 'quadraticCurveTo', 'bezierCurveTo'].includes(k)) note(...n);
        if (k === 'arc' || k === 'ellipse') {
          const rx = n[2];
          const ry = k === 'arc' ? n[2] : n[3];
          note(n[0] - rx, n[0] + rx, n[1] - ry, n[1] + ry);
        }
      };
    },
    set(t, k: string, v) {
      t[k] = v;
      log.push(`${k}=${String(v)}`);
      return true;
    },
  }) as unknown as PictoCtx;
  return { ctx, log, extent: () => [min, max] };
}

// FEED-031
describe('food pictograms (FEED-031)', () => {
  it('has one drawer per food', () => {
    expect([...PICTOGRAM_IDS].sort()).toEqual([...FOODS].sort());
  });

  it('draws every pictogram distinctly, deterministically and inside the 128 grid', () => {
    const seen = new Set<string>();
    for (const id of FOODS) {
      const a = recorder();
      const b = recorder();
      expect(drawPictogram(a.ctx, id, 0, 0, PICTO_GRID), id).toBe(true);
      drawPictogram(b.ctx, id, 0, 0, PICTO_GRID);
      expect(a.log, id).toEqual(b.log);
      expect(a.log.length, id).toBeGreaterThan(8);
      const [min, max] = a.extent();
      expect(min, id).toBeGreaterThanOrEqual(-2);
      expect(max, id).toBeLessThanOrEqual(PICTO_GRID + 2);
      const key = a.log.join('|');
      expect(seen.has(key), `${id} duplicates another pictogram`).toBe(false);
      seen.add(key);
    }
    expect(drawPictogram(recorder().ctx, 'carrot', 0, 0, 10)).toBe(false);
  });
});

// FEED-032
describe('food label layout (FEED-032)', () => {
  it('puts the pictogram above the word, sharing the height by the scale', () => {
    const [w, h, line] = [256, 176, 10];
    const inner = h - 2 * line;
    for (const scale of [0.62, 0.5, 0.34, 0.28]) {
      const l = labelLayout(w, h, line, scale);
      expect(l.pictoSize).toBeCloseTo(inner * scale, 5);
      expect(l.pictoX).toBeCloseTo((w - l.pictoSize) / 2, 5);
      expect(l.pictoY + l.pictoSize).toBeLessThanOrEqual(l.wordCenterY - l.wordMaxH / 2 + 1);
      expect(l.wordCenterY + l.wordMaxH / 2).toBeLessThanOrEqual(h - line);
    }
    // klasse1: similar size; klasse2/3: the word is larger than the pictogram
    const k1 = labelLayout(w, h, line, 0.5);
    expect(Math.abs(k1.pictoSize - k1.wordMaxH)).toBeLessThan(inner * 0.1);
    for (const scale of [0.34, 0.28]) {
      const l = labelLayout(w, h, line, scale);
      expect(l.wordMaxH).toBeGreaterThan(l.pictoSize);
    }
    // kiga: the pictogram is the larger part
    const k = labelLayout(w, h, line, 0.62);
    expect(k.pictoSize).toBeGreaterThan(k.wordMaxH);
  });
});
