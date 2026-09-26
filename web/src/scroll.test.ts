import { describe, expect, it } from 'vitest';
import { DRAG_SLOP_PX, DragGesture } from './scroll';

describe('panel drag scroll (GAME-PLAYER §4)', () => {
  it('PLAY-033: movement below the slop is a tap and scrolls nothing', () => {
    const g = new DragGesture(100, 0);
    expect(g.move(100 + DRAG_SLOP_PX - 1, 16)).toBe(0);
    expect(g.dragging).toBe(false);
  });

  it('PLAY-032: dragging the finger up scrolls the content down by the dragged distance', () => {
    const g = new DragGesture(300, 0);
    let scrolled = 0;
    for (let i = 1; i <= 10; i++) scrolled += g.move(300 - i * 12, i * 16);
    expect(g.dragging).toBe(true);
    expect(scrolled).toBe(120);
    expect(g.velocity).toBeLessThan(0); // finger moving up → coast continues downward
  });

  it('dragging down scrolls back up', () => {
    const g = new DragGesture(100, 0);
    expect(g.move(130, 16)).toBe(-30);
  });
});
