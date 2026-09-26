// Input forwarding (GAME-PLAYER §3): keyboard, mouse drag/wheel and the two-thumb touch
// controls. Only raw values are forwarded; the game (Rust) decides what they mean.
//
// Touch (only after the first `pointerType === 'touch'` input, PLAY-013/014):
// - left half: floating joystick under the thumb (dead zone 10 %, deflection = speed),
// - right half: one horizontal swipe ≥ 40 px = one 45° camera step, two fingers pinch-zoom,
// - both at the same time (each pointer id is tracked on its own, PLAY-017).
//
// Camera views (GAME-CAMERA-VIEWS): right-half drags and mouse drags are also forwarded as
// continuous `look_drag`s; the game turns a close view with them and ignores them in the zoo
// view (and ignores the 45° swipe steps in the close views). Look-around is held with the
// eye button, `V` (via `key`) or the right mouse button; `F` (via `key`) toggles first person.

/** The subset of the WASM `App` used for input. */
export interface InputSink {
  key(code: string, down: boolean): boolean;
  set_stick(x: number, y: number): void;
  drag(dx: number): void;
  drag_end(): void;
  rotate(steps: number): void;
  zoom(factor: number): void;
  /** Look-around held / released (GAME-CAMERA-VIEWS 2). */
  look_hold(on: boolean): boolean;
  /** Continuous turn of a close view (CSS px, y down). */
  look_drag(dx: number, dy: number): void;
}

/** Joystick radius in CSS px (full deflection). */
export const STICK_RADIUS = 60;
/** Dead zone as a fraction of the radius (GAME-PLAYER §3). */
export const DEAD_ZONE = 0.1;
/** Minimum horizontal swipe for one camera step (GAME-PLAYER §3). */
export const SWIPE_PX = 40;

/**
 * Joystick deflection for a pointer offset from the stick centre (screen px, y down):
 * returns `[x right, y up]` with length ≤ 1; below the dead zone it is `[0, 0]`. The length
 * is the deflection itself (60 % drag = 60 % speed, PLAY-015), not rescaled.
 */
export function stickVector(dx: number, dy: number, radius: number, deadZone = DEAD_ZONE): [number, number] {
  const len = Math.hypot(dx, dy);
  if (radius <= 0 || len === 0 || len / radius < deadZone) return [0, 0];
  const k = Math.min(len, radius) / len / radius;
  return [dx * k, -dy * k];
}

/** Which thumb a touch at `x` belongs to (split at the vertical centre line). */
export function touchZone(x: number, width: number): 'left' | 'right' {
  return x < width / 2 ? 'left' : 'right';
}

/** Camera steps for a horizontal swipe distance: ±1 once |dx| ≥ threshold, else 0. */
export function swipeSteps(dx: number, threshold = SWIPE_PX): number {
  if (Math.abs(dx) < threshold) return 0;
  return dx > 0 ? 1 : -1;
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

/** Keys that interact (GAME-PLAYER §3, FIX-024). */
export function isInteractKey(code: string): boolean {
  return code === 'KeyE' || code === 'Space' || code === 'Enter' || code === 'NumpadEnter';
}

/** Where the floating stick is drawn (null = released). */
export interface StickView {
  show(originX: number, originY: number, knobX: number, knobY: number): void;
  hide(): void;
}

interface Swipe {
  startX: number;
  x: number;
  y: number;
  fired: boolean;
}

/**
 * Two-thumb touch gestures as a small state machine over pointer ids (no DOM access, so it
 * is unit-tested in Vitest).
 */
export class TouchGestures {
  private stickId: number | null = null;
  private origin = { x: 0, y: 0 };
  private right = new Map<number, Swipe>();
  private pinchDist = 0;

  constructor(
    private readonly sink: InputSink,
    private readonly view: StickView,
    private readonly radius = STICK_RADIUS,
  ) {}

  /** Number of pointers currently tracked. */
  get active(): number {
    return (this.stickId === null ? 0 : 1) + this.right.size;
  }

  down(id: number, x: number, y: number, width: number): void {
    if (touchZone(x, width) === 'left') {
      if (this.stickId !== null) return; // one stick only
      this.stickId = id;
      this.origin = { x, y };
      this.sink.set_stick(0, 0);
      this.view.show(x, y, x, y);
      return;
    }
    this.right.set(id, { startX: x, x, y, fired: false });
    if (this.right.size === 2) {
      this.pinchDist = this.rightDistance();
      for (const s of this.right.values()) s.fired = true; // a pinch is not a swipe
    }
  }

  move(id: number, x: number, y: number): void {
    if (id === this.stickId) {
      const dx = x - this.origin.x;
      const dy = y - this.origin.y;
      const [sx, sy] = stickVector(dx, dy, this.radius);
      this.sink.set_stick(sx, sy);
      const len = Math.hypot(dx, dy);
      const k = len > this.radius ? this.radius / len : 1;
      this.view.show(this.origin.x, this.origin.y, this.origin.x + dx * k, this.origin.y + dy * k);
      return;
    }
    const s = this.right.get(id);
    if (!s) return;
    if (this.right.size === 1) this.sink.look_drag(x - s.x, y - s.y); // close views
    s.x = x;
    s.y = y;
    if (this.right.size >= 2) {
      const d = this.rightDistance();
      this.sink.zoom(pinchFactor(this.pinchDist, d));
      this.pinchDist = d;
      return;
    }
    if (!s.fired) {
      const steps = swipeSteps(x - s.startX);
      if (steps !== 0) {
        this.sink.rotate(steps);
        s.fired = true; // one step per swipe; re-armed by lifting the finger
      }
    }
  }

  up(id: number): void {
    if (id === this.stickId) {
      this.stickId = null;
      this.sink.set_stick(0, 0);
      this.view.hide();
      return;
    }
    this.right.delete(id);
    if (this.right.size < 2) this.pinchDist = 0;
  }

  /** Releases everything (pointercancel of all, orientation change, blur). */
  reset(): void {
    if (this.stickId !== null) this.up(this.stickId);
    this.right.clear();
    this.pinchDist = 0;
  }

  private rightDistance(): number {
    const [a, b] = [...this.right.values()];
    return Math.hypot(a.x - b.x, a.y - b.y);
  }
}

/**
 * Mouse on the canvas: any button drags (zoo view: 45° steps via `drag`; close views:
 * continuous `look_drag`), the right button also holds look-around (CAMV-010).
 */
export class MouseGestures {
  private pointers = new Map<number, { x: number; y: number; right: boolean }>();

  constructor(private readonly sink: InputSink) {}

  down(id: number, button: number, x: number, y: number): void {
    const right = button === 2;
    this.pointers.set(id, { x, y, right });
    if (right) this.sink.look_hold(true);
  }

  move(id: number, x: number, y: number): void {
    const p = this.pointers.get(id);
    if (!p) return;
    this.sink.drag(x - p.x);
    this.sink.look_drag(x - p.x, y - p.y);
    p.x = x;
    p.y = y;
  }

  up(id: number): void {
    const p = this.pointers.get(id);
    if (!p) return;
    this.pointers.delete(id);
    if (p.right) this.sink.look_hold(false);
    if (this.pointers.size === 0) this.sink.drag_end();
  }

  reset(): void {
    for (const id of [...this.pointers.keys()]) this.up(id);
  }
}

/**
 * The touch eye button (GAME-CAMERA-VIEWS 2): look-around while pressed; sliding the same
 * thumb from the button turns the view.
 */
export class LookButton {
  private id: number | null = null;
  private last = { x: 0, y: 0 };

  constructor(private readonly sink: InputSink) {}

  get held(): boolean {
    return this.id !== null;
  }

  down(id: number, x: number, y: number): void {
    if (this.id !== null) return;
    this.id = id;
    this.last = { x, y };
    this.sink.look_hold(true);
  }

  move(id: number, x: number, y: number): void {
    if (id !== this.id) return;
    this.sink.look_drag(x - this.last.x, y - this.last.y);
    this.last = { x, y };
  }

  up(id: number): void {
    if (id !== this.id) return;
    this.id = null;
    this.sink.look_hold(false);
  }
}

/** Wires the eye button's pointer events to a {@link LookButton}. */
export function attachLookButton(button: HTMLElement, sink: InputSink): LookButton {
  const look = new LookButton(sink);
  button.addEventListener('pointerdown', (e) => {
    e.preventDefault();
    e.stopPropagation();
    try {
      button.setPointerCapture(e.pointerId);
    } catch {
      // synthetic pointers cannot be captured
    }
    look.down(e.pointerId, e.clientX, e.clientY);
  });
  button.addEventListener('pointermove', (e) => look.move(e.pointerId, e.clientX, e.clientY));
  for (const ev of ['pointerup', 'pointercancel', 'lostpointercapture'] as const) {
    button.addEventListener(ev, (e) => look.up(e.pointerId));
  }
  button.addEventListener('contextmenu', (e) => e.preventDefault());
  return look;
}

export interface InputOptions {
  canvas: HTMLElement;
  stickView: StickView;
  /** Called once, on the first touch input (shows the touch controls, PLAY-014). */
  onFirstTouch: () => void;
  /** Interact key pressed (E / Space / Enter). */
  onInteract: () => void;
  /** Escape pressed. */
  onEscape: () => void;
}

export function attachInput(sink: InputSink, opts: InputOptions): TouchGestures {
  const { canvas } = opts;
  const held = new Set<string>();
  window.addEventListener('keydown', (e) => {
    if (isInteractKey(e.code)) {
      e.preventDefault();
      if (!e.repeat) opts.onInteract();
      return;
    }
    if (e.code === 'Escape') {
      opts.onEscape();
      return;
    }
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

  const gestures = new TouchGestures(sink, opts.stickView);
  const mice = new MouseGestures(sink);
  let touchSeen = false;
  const releaseAll = () => {
    for (const code of held) sink.key(code, false);
    held.clear();
    gestures.reset();
    mice.reset();
    sink.set_stick(0, 0);
  };
  window.addEventListener('blur', releaseAll);
  window.addEventListener('orientationchange', () => gestures.reset());

  // First touch anywhere switches the touch controls on (they never show for mouse only).
  window.addEventListener(
    'pointerdown',
    (e) => {
      if (e.pointerType === 'touch' && !touchSeen) {
        touchSeen = true;
        opts.onFirstTouch();
      }
    },
    { capture: true },
  );

  // Mouse: drag rotates (right button: look-around), wheel zooms. Touch: gestures.
  canvas.addEventListener('pointerdown', (e) => {
    e.preventDefault();
    if (e.pointerType === 'touch') {
      gestures.down(e.pointerId, e.clientX, e.clientY, canvas.clientWidth || window.innerWidth);
      return;
    }
    try {
      canvas.setPointerCapture(e.pointerId);
    } catch {
      // synthetic pointers cannot be captured
    }
    mice.down(e.pointerId, e.button, e.clientX, e.clientY);
    canvas.focus();
  });
  canvas.addEventListener('pointermove', (e) => {
    if (e.pointerType === 'touch') {
      gestures.move(e.pointerId, e.clientX, e.clientY);
      return;
    }
    mice.move(e.pointerId, e.clientX, e.clientY);
  });
  const end = (e: PointerEvent) => {
    if (e.pointerType === 'touch') {
      gestures.up(e.pointerId);
      return;
    }
    mice.up(e.pointerId);
  };
  canvas.addEventListener('pointerup', end);
  canvas.addEventListener('pointercancel', end);
  canvas.addEventListener('lostpointercapture', (e) => {
    if (e.pointerType !== 'touch') end(e);
  });
  canvas.addEventListener(
    'wheel',
    (e) => {
      sink.zoom(wheelFactor(e.deltaY));
      e.preventDefault();
    },
    { passive: false },
  );
  canvas.addEventListener('contextmenu', (e) => e.preventDefault());
  // No browser gestures while playing (PLAY-018): iOS pinch and double-tap zoom.
  for (const ev of ['gesturestart', 'gesturechange', 'dblclick']) {
    document.addEventListener(ev, (e) => e.preventDefault(), { passive: false });
  }
  document.addEventListener(
    'touchmove',
    (e) => {
      if (e.cancelable) e.preventDefault();
    },
    { passive: false },
  );
  document.addEventListener('selectstart', (e) => e.preventDefault());
  return gestures;
}
