// Input forwarding (GAME-PLAYER §3): keyboard, mouse drag/wheel, touch joystick, one-finger
// drag and pinch. Only raw values are forwarded; the game decides what they mean.

/** The subset of the WASM `App` used for input. */
export interface InputSink {
  key(code: string, down: boolean): boolean;
  set_stick(x: number, y: number): void;
  drag(dx: number): void;
  drag_end(): void;
  zoom(factor: number): void;
}

/**
 * Joystick deflection for a pointer offset from the stick centre (screen px, y down):
 * returns `[x right, y up]` with length ≤ 1 and a small dead zone.
 */
export function stickVector(dx: number, dy: number, radius: number, deadZone = 0.12): [number, number] {
  const len = Math.hypot(dx, dy);
  if (radius <= 0 || len / radius < deadZone) return [0, 0];
  const k = Math.min(len, radius) / len / radius;
  return [dx * k, -dy * k];
}

/** Zoom factor for a pinch from `before` to `after` finger distance (> 1 = zoom out). */
export function pinchFactor(before: number, after: number): number {
  if (before <= 0 || after <= 0) return 1;
  return before / after;
}

/** Zoom factor for a wheel delta in pixels. */
export function wheelFactor(deltaY: number): number {
  return Math.exp(Math.max(-200, Math.min(200, deltaY)) * 0.0015);
}

export function attachInput(sink: InputSink, canvas: HTMLElement, stick: HTMLElement, knob: HTMLElement): void {
  const held = new Set<string>();
  window.addEventListener('keydown', (e) => {
    if (e.repeat && held.has(e.code)) {
      e.preventDefault();
      return;
    }
    if (sink.key(e.code, true)) {
      held.add(e.code);
      e.preventDefault();
    }
  });
  window.addEventListener('keyup', (e) => {
    held.delete(e.code);
    if (sink.key(e.code, false)) e.preventDefault();
  });
  window.addEventListener('blur', () => {
    for (const code of held) sink.key(code, false);
    held.clear();
    sink.set_stick(0, 0);
  });

  // Joystick.
  let stickPointer: number | null = null;
  const moveStick = (e: PointerEvent) => {
    const r = stick.getBoundingClientRect();
    const radius = r.width / 2;
    const dx = e.clientX - (r.left + radius);
    const dy = e.clientY - (r.top + radius);
    const [x, y] = stickVector(dx, dy, radius);
    sink.set_stick(x, y);
    knob.style.transform = `translate(${x * radius * 0.6}px, ${-y * radius * 0.6}px)`;
  };
  stick.addEventListener('pointerdown', (e) => {
    stickPointer = e.pointerId;
    stick.setPointerCapture(e.pointerId);
    moveStick(e);
    e.preventDefault();
  });
  stick.addEventListener('pointermove', (e) => {
    if (e.pointerId === stickPointer) moveStick(e);
  });
  const endStick = (e: PointerEvent) => {
    if (e.pointerId !== stickPointer) return;
    stickPointer = null;
    sink.set_stick(0, 0);
    knob.style.transform = '';
  };
  stick.addEventListener('pointerup', endStick);
  stick.addEventListener('pointercancel', endStick);

  // Canvas: drag rotates (mouse or one finger), two fingers pinch-zoom, wheel zooms.
  const pointers = new Map<number, { x: number; y: number }>();
  let pinchDist = 0;
  const dist = () => {
    const [a, b] = [...pointers.values()];
    return Math.hypot(a.x - b.x, a.y - b.y);
  };
  canvas.addEventListener('pointerdown', (e) => {
    canvas.setPointerCapture(e.pointerId);
    pointers.set(e.pointerId, { x: e.clientX, y: e.clientY });
    if (pointers.size === 2) pinchDist = dist();
    canvas.focus();
  });
  canvas.addEventListener('pointermove', (e) => {
    const p = pointers.get(e.pointerId);
    if (!p) return;
    const dx = e.clientX - p.x;
    p.x = e.clientX;
    p.y = e.clientY;
    if (pointers.size === 1) {
      sink.drag(dx);
    } else if (pointers.size === 2) {
      const d = dist();
      sink.zoom(pinchFactor(pinchDist, d));
      pinchDist = d;
    }
  });
  const endPointer = (e: PointerEvent) => {
    pointers.delete(e.pointerId);
    if (pointers.size < 2) pinchDist = 0;
    if (pointers.size === 0) sink.drag_end();
  };
  canvas.addEventListener('pointerup', endPointer);
  canvas.addEventListener('pointercancel', endPointer);
  canvas.addEventListener(
    'wheel',
    (e) => {
      sink.zoom(wheelFactor(e.deltaY));
      e.preventDefault();
    },
    { passive: false },
  );
  canvas.addEventListener('contextmenu', (e) => e.preventDefault());
}
