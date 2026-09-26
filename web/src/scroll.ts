// Drag-to-scroll for the reading panel (GAME-PLAYER §4 "Scrolling long texts", PLAY-032/033).
// The page sets `touch-action: none` everywhere so the game gets every touch; browsers then
// never pan-scroll the panel with a finger. So the panel scrolls itself: a drag anywhere on
// the panel box moves the scrolling part, a short coast follows a swipe, taps stay taps.

/** Movement (px) before a press becomes a drag; below it, it is a tap (PLAY-033). */
export const DRAG_SLOP_PX = 6;
/** Coast: velocity decay per 16 ms frame and the speed (px/ms) below which it stops. */
const FRICTION = 0.92;
const MIN_SPEED = 0.02;

/** State of one drag gesture, kept pure so it can be unit-tested. */
export class DragGesture {
  private startY: number;
  private lastY: number;
  private lastT: number;
  /** Smoothed velocity in px/ms (positive = finger moves down). */
  velocity = 0;
  dragging = false;

  constructor(y: number, t: number) {
    this.startY = y;
    this.lastY = y;
    this.lastT = t;
  }

  /** Feeds a pointer move; returns the scroll delta to apply (0 while still a tap). */
  move(y: number, t: number): number {
    if (!this.dragging && Math.abs(y - this.startY) < DRAG_SLOP_PX) return 0;
    const first = !this.dragging;
    this.dragging = true;
    const from = first ? this.startY : this.lastY;
    const dy = y - from;
    const dt = Math.max(1, t - this.lastT);
    this.velocity = 0.8 * (dy / dt) + 0.2 * this.velocity;
    this.lastY = y;
    this.lastT = t;
    return -dy; // finger up (dy < 0) scrolls the content down
  }
}

/**
 * Makes `area` (the whole panel box) drag-scroll `target` (its scrolling part). Returns a
 * function that removes the listeners.
 */
export function dragScroll(area: HTMLElement, target: HTMLElement): () => void {
  let gesture: DragGesture | null = null;
  let pointer = -1;
  let coast = 0;

  const stopCoast = () => {
    if (coast) cancelAnimationFrame(coast);
    coast = 0;
  };

  const down = (e: PointerEvent) => {
    if (target.scrollHeight <= target.clientHeight) return; // nothing to scroll
    stopCoast();
    pointer = e.pointerId;
    gesture = new DragGesture(e.clientY, e.timeStamp);
  };
  const move = (e: PointerEvent) => {
    if (!gesture || e.pointerId !== pointer) return;
    const d = gesture.move(e.clientY, e.timeStamp);
    if (!gesture.dragging) return;
    if (!area.hasPointerCapture(e.pointerId)) area.setPointerCapture(e.pointerId);
    target.scrollTop += d;
    e.preventDefault();
    e.stopPropagation();
  };
  const up = (e: PointerEvent) => {
    if (!gesture || e.pointerId !== pointer) return;
    const g = gesture;
    gesture = null;
    pointer = -1;
    if (!g.dragging) return; // a tap: let the click through (PLAY-033)
    // swallow the click that follows a drag
    area.addEventListener('click', (c) => c.stopPropagation(), { capture: true, once: true });
    let v = -g.velocity; // px/ms in scroll direction
    let last = performance.now();
    const step = (now: number) => {
      const dt = now - last;
      last = now;
      target.scrollTop += v * dt;
      v *= Math.pow(FRICTION, dt / 16);
      coast = Math.abs(v) > MIN_SPEED ? requestAnimationFrame(step) : 0;
    };
    if (Math.abs(v) > MIN_SPEED) coast = requestAnimationFrame(step);
  };

  area.addEventListener('pointerdown', down);
  area.addEventListener('pointermove', move);
  area.addEventListener('pointerup', up);
  area.addEventListener('pointercancel', up);
  return () => {
    stopCoast();
    area.removeEventListener('pointerdown', down);
    area.removeEventListener('pointermove', move);
    area.removeEventListener('pointerup', up);
    area.removeEventListener('pointercancel', up);
  };
}
